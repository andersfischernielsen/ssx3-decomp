//! Staging module oc396 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc396`.

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
    (0x0041_ab78, strncasecmp, "strncasecmp"),
    (
        0x003a_c048,
        cps2_chunk_resolve_patch_dma_chain,
        "cPS2ChunkResolve_PatchDmaChain",
    ),
    (
        0x0012_5448,
        rider_update_script_sounds,
        "Rider_UpdateScriptSounds",
    ),
    (
        0x0015_1c90,
        be_set_char_profile_bolt_equipped,
        "BE_SetCharProfileBoltEquipped",
    ),
    (0x0012_42b0, rider_probe_ground, "Rider_ProbeGround"),
    (
        0x0032_1880,
        input_parser_lookup_builtin_name,
        "InputParser_LookupBuiltinName",
    ),
    (
        0x002b_7318,
        c_world_sound_insert_emitter,
        "cWorldSound_InsertEmitter",
    ),
    (0x0014_efa8, be_get_rider_size_scale, "BE_GetRiderSizeScale"),
    (
        0x0032_36d8,
        input_parser_parse_comparison,
        "InputParser_ParseComparison",
    ),
    (
        0x0032_3a68,
        input_parser_parse_multiplicative,
        "InputParser_ParseMultiplicative",
    ),
    (
        0x0032_3900,
        input_parser_parse_additive,
        "InputParser_ParseAdditive",
    ),
    (0x001f_2aa0, hud_draw_wrapped_text, "HUD_DrawWrappedText"),
    (
        0x0019_f2d0,
        fe_module_rider_preview_apply_transform,
        "FEModule_RiderPreviewApplyTransform",
    ),
    (
        0x0012_c230,
        rider_gate_sub_update_charge,
        "RiderGateSub_UpdateCharge",
    ),
    (
        0x0013_bd80,
        rider_railslide_state_compute_rail_frame,
        "RiderRailslideState_ComputeRailFrame",
    ),
    (
        0x0029_a7d8,
        ssx_audio_system_play_trick_points_tick,
        "SSXAudioSystem_PlayTrickPointsTick",
    ),
    (
        0x0027_3698,
        in_seq_ramp_track_update,
        "InSeq_RampTrackUpdate",
    ),
    (
        0x0029_3948,
        ssx_audio_system_calc_surface_loop_group2,
        "SSXAudioSystem_CalcSurfaceLoopGroup2",
    ),
    (
        0x0019_ee88,
        fe_module_rider_preview_render,
        "FEModule_RiderPreviewRender",
    ),
];

// ---------------------------------------------------------------------------
// strncasecmp (0x41ab78)

/// `tolower` (0x41ea18), the libc character-class lookup.
const STRNCASECMP_TOLOWER: Callee = Callee {
    addr: 0x0041_ea18,
    f: ee::s15::tolower,
};

/// `int strncasecmp(const char* s1, const char* s2, int n)` (0x41ab78):
/// compare at most `n` characters case-insensitively; returns the
/// difference of the lowered characters where they part (0 for `n == 0`).
pub fn strncasecmp(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        if c.g[S1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[S4][0] = 0xffff_ffff_ffff_ffff;
            'scan: loop {
                // n-- != 0
                c.cycles += 3;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                if c.g[S1][0] == c.g[S4][0] {
                    break 'scan;
                }
                c.cycles += 7;
                c.g[A0][0] = c.lb(c.g[S2][0] as u32);
                STRNCASECMP_TOLOWER.call(c, 0x0041_abd0)?;
                c.g[A0][0] = c.lb(c.g[S3][0] as u32);
                c.g[S0][0] = c.g[V0][0];
                STRNCASECMP_TOLOWER.call(c, 0x0041_abdc)?;
                if c.g[S0][0] != c.g[V0][0] {
                    break 'scan;
                }
                c.cycles += 2;
                if c.g[S1][0] == 0 {
                    break 'scan;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                if c.g[V0][0] == 0 {
                    break 'scan;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[S3][0] as u32);
                if c.g[V0][0] == 0 {
                    break 'scan;
                }
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                if event_due(c) {
                    deliver_event_here(c, 0x0041_abb8);
                }
                c.cycles += 1;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            }
            // The difference of the lowered (unsigned) characters.
            c.cycles += 6;
            c.g[A0][0] = c.lbu(c.g[S2][0] as u32);
            STRNCASECMP_TOLOWER.call(c, 0x0041_ac0c)?;
            c.g[A0][0] = c.lbu(c.g[S3][0] as u32);
            c.g[S0][0] = c.g[V0][0];
            STRNCASECMP_TOLOWER.call(c, 0x0041_ac18)?;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
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
// cPS2ChunkResolve_PatchDmaChain (0x3ac048)

/// DMA tag id (`(tag << 4) >> 32 & 7`, bits 28..30) of the chain entry in `a1`.
#[inline]
fn patch_dma_chain_tag_id(tag: u64) -> u64 {
    (((tag << 4) as i64) >> 32) as u64 & 0x7
}

/// The relocation shared by both "ref + data" steps, with `t0` at the data
/// tag: relocate its address (`+4 += ctx->base`), read its qwc and leave in
/// `a1` the address of the last quadword it points at (`addr + (qwc-1)*16`);
/// `t0` moves past it.
fn patch_dma_chain_relocate_data(c: &mut Cpu) {
    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x24));
    c.g[A1][0] = c.ld(c.g[T0][0] as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[V1][0] = (((c.g[A1][0] << 32) as i64) >> 32) as u64;
    c.g[V0][0] = c.g[V1][0] & 0xffff;
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = c.lw(c.g[A3][0] as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
}

/// `int cPS2ChunkResolve_PatchDmaChain(ctx, obj, list, index)` (0x3ac048):
/// walk a loaded model's DMA chain (`ctx->base (+0x24) + obj->chainOffset
/// (+4)`), relocating each ref/data tag pair by `ctx->base`, chaining the
/// last data quadword to the first or the shared microprogram call, and, once
/// the chain is "split" (object flags `0x60000` or a ref address with bit 31),
/// inserting the extra call/end tags. Stops at an end tag (id 6); returns 1.
pub fn cps2_chunk_resolve_patch_dma_chain(c: &mut Cpu) {
    c.cycles += 46;
    let frame = Frame::open(c, 0x50);
    c.g[V0][0] = 0x510000;
    frame.save(c, S0, 0x40);
    c.g[V1][0] = 0x510000;
    frame.save(c, S1, 0x30);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6bf0));
    frame.save(c, S2, 0x20);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6be0));
    frame.save(c, S3, 0x10);
    c.g[T2][0] = c.g[A2][0];
    frame.save(c, S4, 0);
    // t7/t6/s2: the call tags' addresses (0x509410, 0x509420, 0x509400) in
    // the upper word.
    c.g[T7][0] = c.g[V0][0] << 32;
    c.g[T6][0] = c.g[V1][0] << 32;
    c.g[A2][0] = 0x0;
    c.g[A3][0] = c.lw(c.g[A1][0] as u32);
    c.g[V1][0] = 0x60000;
    c.g[V0][0] = 0x510000;
    c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = c.g[A3][0] & c.g[V1][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c00));
    c.g[A0][0] = 0x0;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2270));
    c.g[T3][0] = 0x1;
    c.g[A2][0] = sx((c.g[A2][0] as u32) >> 3);
    c.g[V1][0] = 0x1400_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x22c8));
    c.g[A1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x24));
    c.g[S2][0] = c.g[V0][0] << 32;
    c.g[V0][0] = 0x3000_0001;
    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 3);
    // s1/a0: the MSCAL words for the first / the later batches.
    c.g[S1][0] = c.g[A2][0] | c.g[V1][0];
    c.g[T1][0] = c.g[T3][0];
    c.g[T4][0] = 0x7fff_0000;
    c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
    c.g[T5][0] = c.g[T6][0] | c.g[V0][0];
    c.g[S0][0] = c.g[T7][0] | c.g[V0][0];
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    // t1: split, t3: first batch, t9: done.
    if c.g[A3][0] == 0 {
        c.g[T1][0] = 0;
    }
    c.g[T9][0] = 0;
    c.g[A2][0] = 0x6;
    c.g[T8][0] = 0xffff_ffff_8000_0000;
    c.g[T4][0] = c.g[T4][0] | 0xffff;
    c.g[S4][0] = 0x3;
    c.g[S3][0] = 0x6000_0000;
    c.g[A1][0] = c.ld(c.g[T0][0] as u32);
    loop {
        c.cycles += 5;
        c.g[V0][0] = patch_dma_chain_tag_id(c.g[A1][0]);
        if c.g[V0][0] == c.g[S4][0] {
            // A ref tag: its address may carry the split marker (bit 31).
            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
            c.cycles += 3;
            let marked = c.g[V1][0] & c.g[T8][0] != 0;
            c.g[V0][0] = c.g[V1][0] & c.g[T4][0];
            if marked {
                c.cycles += 2;
                c.g[T1][0] = 0x1;
                c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            }
            c.cycles += 20;
            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x24));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
            patch_dma_chain_relocate_data(c);
            if c.g[T3][0] == 0 {
                c.cycles += 1;
                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            } else {
                c.cycles += 2;
                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S1][0]);
            }
            c.cycles += 2;
            if c.g[T1][0] == 0 {
                c.g[T3][0] = 0;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = patch_dma_chain_tag_id(c.g[V0][0]);
                c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
                if c.g[V0][0] == c.g[A2][0] {
                    // The chain ends after this batch: call, call, end.
                    c.cycles += 6;
                    c.g[V1][0] = 0x3000_0001;
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), 0);
                    c.g[V0][0] = c.g[S2][0] | c.g[V1][0];
                    c.sd(c.g[T0][0] as u32, c.g[V0][0]);
                    if c.g[T3][0] == 0 {
                        c.cycles += 1;
                        c.g[V0][0] = c.g[T6][0] | c.g[V1][0];
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[T7][0] | c.g[V1][0];
                    }
                    c.cycles += 5;
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), 0);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x20), c.g[S3][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x28), 0);
                } else {
                    // A second ref/data pair, then the two call tags.
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                    let marked = c.g[V1][0] & c.g[T8][0] != 0;
                    c.g[V0][0] = c.g[V1][0] & c.g[T4][0];
                    if marked {
                        c.cycles += 3;
                        c.g[T1][0] = 0x1;
                        c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                    }
                    c.cycles += 20;
                    c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[T0][0] = c.g[A1][0];
                    patch_dma_chain_relocate_data(c);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    if c.g[T3][0] == 0 {
                        c.cycles += 1;
                        c.sd(c.g[T0][0] as u32, c.g[T5][0]);
                    } else {
                        c.cycles += 2;
                        c.sd(c.g[T0][0] as u32, c.g[S0][0]);
                    }
                    c.cycles += 3;
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), 0);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[T5][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), 0);
                }
                c.cycles += 3;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                c.g[T3][0] = 0;
            }
        } else {
            c.cycles += 2;
            if c.g[V0][0] == c.g[A2][0] {
                // The end tag.
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
                c.cycles += 1;
                c.g[T9][0] = 0x1;
            } else {
                c.cycles += 2;
            }
        }
        c.cycles += 2;
        if c.g[T9][0] != 0 {
            break;
        }
        c.g[A1][0] = c.ld(c.g[T0][0] as u32);
        if event_due(c) {
            deliver_event_here(c, 0x003a_c100);
        }
    }
    c.cycles += 8;
    frame.restore(c, S0, 0x40);
    c.g[V0][0] = 0x1;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore(c, S4, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Rider_UpdateScriptSounds (0x125448)

/// Rider field: the four scripted-sound handles (`-1` = none).
pub(crate) const RIDER_SCRIPT_SOUND_IDS: u32 = 0xb08;

/// Push one array element of the current sound slot: `*(S[field] + cur*4) = v`
/// with `S = T + 0x1d8`, `cur = T[0x1f4]` (`s0` = T, `s1` = S).
fn rider_update_script_sounds_put(c: &mut Cpu, field: u32, value: u64, cur_first: bool) {
    if cur_first {
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(field));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, value);
    } else {
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(field));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, value);
    }
}

/// The +0x70 array (16-byte elements): `*(S[0x70] + cur*16 + off) = v`.
fn rider_update_script_sounds_put70(c: &mut Cpu, off: u32, value: u64, cur_first: bool) {
    if cur_first {
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(off), value);
    } else {
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(off), value);
    }
}

/// `void Rider_UpdateScriptSounds(cRider* self, ScriptEvent* ev, float when,
/// int flags)` (0x125448): drop the rider's scripted-sound handles whose bank
/// instance is gone; with `flags & 1` start each of the event's sounds timed
/// at `when` (push a fresh audio slot set up like the defaults, its volume
/// clamped to 0..127, positioned on the rider) and keep the new handle; with
/// `flags & 2` stop every handle the event no longer lists.
pub fn rider_update_script_sounds(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S2, 0x90);
        frame.save(c, S5, 0x60);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S6, 0x50);
        c.g[S5][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0xc0);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S0, 0xb0);
        c.mov_s(20, 12);
        frame.save(c, S1, 0xa0);
        frame.save(c, S3, 0x80);
        c.g[S1][0] = 0x3;
        frame.save(c, S4, 0x70);
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        c.g[S7][0] = sx((c.g[S6][0] as u32).wrapping_add(RIDER_SCRIPT_SOUND_IDS));
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[S7][0];
        call_ptr(c, 0x0028_b180, 0x0012_549c)?; // SSXAudioSystem_GetInstance
        c.g[S4][0] = c.g[V0][0];

        // Forget handles whose bank instance is gone.
        loop {
            c.cycles += 3;
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A2][0] = 0x1;
            if (c.g[A1][0] as i64) >= 0 {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                call_ptr(c, 0x002a_d550, 0x0012_54bc)?; // BankMonitor_FindInstance
                if c.g[V0][0] == 0 {
                    c.sw(c.g[S0][0] as u32, c.g[S3][0]);
                }
            }
            c.cycles += 3;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0012_54a0);
            }
        }

        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 0x1;
        let start = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S2][0] & 0x2;
        'out: {
            if start {
                c.cycles += 3;
                c.g[S5][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x110));
                if c.g[S5][0] == 0 {
                    break 'out;
                }
                c.cycles += 2;
                c.g[FP][0] = 0x18;
                let a = frame.at(c, 0x18);
                c.sw(a, c.g[V0][0]); // the rider's position
                loop {
                    c.cycles += 6;
                    c.lwc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    c.c_eq_s(0, 20);
                    let due = c.fcc();
                    c.g[A3][0] = sx(frame.at(c, 0x14));
                    if !due {
                        break 'out;
                    }
                    c.cycles += 161;
                    c.g[A1][0] = c.lw(c.g[S5][0] as u32);
                    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6d0));
                    c.g[A2][0] = sx(frame.at(c, 0x10));
                    call_ptr(c, 0x0028_0548, 0x0012_5518)?; // the sound's bank args/type
                                                            // Push a fresh slot on the current track (T = *snd, S = T + 0x1d8).
                    c.g[S0][0] = c.lw(c.g[S4][0] as u32);
                    c.g[A1][0] = 0;
                    c.g[S2][0] = c.lw(frame.at(c, 0x14));
                    c.g[A2][0] = 0x8;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                    c.g[S3][0] = c.lw(frame.at(c, 0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.mult(0, V1, V1, FP);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    // ents[cur] = the 24-byte default config (unaligned copy).
                    let s0 = c.g[S0][0] as u32;
                    c.ldl(A0, s0.wrapping_add(0x1e3));
                    c.ldr(A0, s0.wrapping_add(0x1dc));
                    c.ldl(A3, s0.wrapping_add(0x1eb));
                    c.ldr(A3, s0.wrapping_add(0x1e4));
                    c.ldl(T0, s0.wrapping_add(0x1f3));
                    c.ldr(T0, s0.wrapping_add(0x1ec));
                    let e = c.g[V1][0] as u32;
                    c.sdl(A0, e.wrapping_add(0x7));
                    c.sdr(A0, e);
                    c.sdl(A3, e.wrapping_add(0xf));
                    c.sdr(A3, e.wrapping_add(0x8));
                    c.sdl(T0, e.wrapping_add(0x17));
                    c.sdr(T0, e.wrapping_add(0x10));
                    // The inline +0x24 array (T + 0x1fc + cur*4).
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0);
                    rider_update_script_sounds_put(c, 0x34, 0, false);
                    // p38[cur] = ents[cur].b0
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                    c.mult(0, A0, A0, FP);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                    c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                    rider_update_script_sounds_put(c, 0x3c, 0, true);
                    // memset(&S[0x40 + cur*8], 0, 8)
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                    call_ptr(c, 0x0041_6210, 0x0012_55f0)?; // memset
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A0][0] = 0x64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                    c.g[A1][0] = 0x5a;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A2][0] = 0x32;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = 0x7f;
                    c.sw(c.g[V1][0] as u32, 0);
                    c.g[T0][0] = 0x1;
                    rider_update_script_sounds_put(c, 0x64, 0, true);
                    rider_update_script_sounds_put(c, 0x68, 0, false);
                    rider_update_script_sounds_put(c, 0x6c, 0, true);
                    rider_update_script_sounds_put70(c, 0x4, 0, false);
                    let (a0, a1, a2) = (c.g[A0][0], c.g[A1][0], c.g[A2][0]);
                    rider_update_script_sounds_put70(c, 0x0, a0, true);
                    rider_update_script_sounds_put70(c, 0x8, a1, false);
                    rider_update_script_sounds_put70(c, 0xc, a2, true);
                    rider_update_script_sounds_put(c, 0x74, 0, false);
                    rider_update_script_sounds_put(c, 0x80, 0, true);
                    rider_update_script_sounds_put(c, 0x84, 0, false);
                    let a3 = c.g[A3][0];
                    rider_update_script_sounds_put(c, 0x78, a3, true);
                    let t0 = c.g[T0][0];
                    rider_update_script_sounds_put(c, 0x7c, t0, false);
                    // snd->types[cur] = type, snd->args[cur] = arg.
                    c.g[A0][0] = c.lw(c.g[S4][0] as u32);
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[S2][0]);
                    c.g[V1][0] = c.lw(c.g[S4][0] as u32);
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[S3][0]);
                    // The volume: p38[cur] = *ev->vol, ents[cur].b0 = clamp(0, 127).
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x1c));
                    c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A0][0] = c.lb(c.g[V0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                    c.mult(0, A0, A0, FP);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
                    let below_max = c.g[V0][0] != 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    let vol_at = |c: &mut Cpu| -> u32 {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                        c.g[V0][0] as u32
                    };
                    let mut clamped = false;
                    if below_max {
                        c.cycles += 6;
                        let a = vol_at(c);
                        c.g[V1][0] = c.lw(a);
                        c.g[V0][0] = 0;
                        clamped = (c.g[V1][0] as i64) < 0;
                    }
                    if !clamped {
                        c.cycles += 7;
                        let a = vol_at(c);
                        c.g[V1][0] = c.lw(a);
                        c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
                        if c.g[V1][0] == 0 {
                            c.g[V0][0] = 0x7f;
                        } else {
                            c.cycles += 4;
                            let a = vol_at(c);
                            c.g[V0][0] = c.lbu(a);
                        }
                    }
                    c.cycles += 38;
                    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[A2][0] = 0;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = 0x4;
                    c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    call_ptr(c, 0x0028_7968, 0x0012_57e8)?; // SSXAudioSystem_GetChannelGainPtr
                    c.g[A3][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1f4));
                    c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x214));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A3][0] = c.lw(frame.at(c, 0x18));
                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[A3][0]);
                    c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    call_ptr(c, 0x0028_8a20, 0x0012_582c)?; // SSXAudioSystem_GetRiderListenerIndex
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x23c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    call_ptr(c, 0x0029_06b8, 0x0012_584c)?; // Audio_PlayPushedSound
                                                            // ids[ev->slot] = handle; next sound.
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    c.g[S5][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x20));
                    let more = c.g[S5][0] != 0;
                    frame.restore(c, S0, 0xb0);
                    if !more {
                        c.cycles += 2;
                        frame.restore(c, S1, 0xa0);
                        c.cycles += 11;
                        rider_update_script_sounds_epilogue(c, frame);
                        return Ok(());
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0012_54f0);
                    }
                }
            }
            // flags & 2: stop the handles the event does not list.
            c.cycles += 2;
            let stop = c.g[V0][0] != 0;
            c.g[V1][0] = 0x1;
            if !stop {
                break 'out;
            }
            c.cycles += 2;
            c.g[S1][0] = 0x3;
            c.g[V0][0] = sx(frame.at(c, 0xc));
            loop {
                c.cycles += 7;
                c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                let more = (c.g[S1][0] as i64) >= 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_5880);
                }
            }
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = c.g[S7][0];
            if c.g[A1][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx(sp(c).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V1][0] as u32, 0);
                    c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x20));
                    if c.g[A1][0] == 0 {
                        c.cycles += 1;
                        c.g[S0][0] = c.g[S7][0];
                        break;
                    }
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    if event_due(c) {
                        deliver_event_here(c, 0x0012_58b0);
                    }
                }
            }
            c.cycles += 4;
            c.g[S3][0] = 0xffff_ffff_ffff_ffff;
            c.g[S2][0] = c.g[SP][0];
            c.g[S1][0] = 0x3;
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A2][0] = 0x1;
                    if (c.g[A1][0] as i64) >= 0 {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                        c.mtc1(0, 12);
                        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                        call_ptr(c, 0x002a_d5f0, 0x0012_590c)?; // BankMonitor_StopHandle
                        c.sw(c.g[S0][0] as u32, c.g[S3][0]);
                    }
                    c.cycles += 1;
                }
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                c.cycles += 3;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                let more = (c.g[S1][0] as i64) >= 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_58e0);
                }
            }
        }
        c.cycles += 2;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        c.cycles += 11;
        rider_update_script_sounds_epilogue(c, frame);
        Ok(())
    })
}

/// The shared epilogue of [`rider_update_script_sounds`] (s2-s7, fp, ra, f20).
fn rider_update_script_sounds_epilogue(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S2, 0x90);
    frame.restore(c, S3, 0x80);
    frame.restore(c, S4, 0x70);
    frame.restore(c, S5, 0x60);
    frame.restore(c, S6, 0x50);
    frame.restore(c, S7, 0x40);
    frame.restore(c, FP, 0x30);
    frame.restore_ra(c, 0x20);
    frame.restore_fpr(c, 20, 0xc0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// BE_SetCharProfileBoltEquipped (0x151c90)

const BOLT_EQUIP_GET_DB: Callee = Callee {
    addr: 0x0014_bdb8,
    f: ee::s01::Bolts_GetDB,
};
const BOLT_EQUIP_FIND_INDEX: Callee = Callee {
    addr: 0x0014_d448,
    f: ee::s01::Bolts_FindBoltIndex,
};
const BOLT_EQUIP_FIRST_CHILD: Callee = Callee {
    addr: 0x0014_d558,
    f: ee::s01::Bolts_FindFirstChild,
};
const BOLT_EQUIP_COUNT_CHILDREN: Callee = Callee {
    addr: 0x0014_d4c8,
    f: ee::s01::Bolts_CountChildren,
};
const BOLT_EQUIP_APPLY_DEPS: Callee = Callee {
    addr: 0x0015_1ef0,
    f: ee::s01::BE_ApplyBoltDependencies,
};
/// The recursion goes through the generated alias (the override slot once
/// installed).
const BOLT_EQUIP_SELF: Callee = Callee {
    addr: 0x0015_1c90,
    f: ee::s01::BE_SetCharProfileBoltEquipped,
};

/// Profile fields: the id -> item index table (`short*`), the items (4 bytes
/// each, flags at +2: 0x10 = equipped), the bolt kind (byte).
pub(crate) const BOLT_EQUIP_LOOKUP: u32 = 0x288;
pub(crate) const BOLT_EQUIP_ITEMS: u32 = 0x290;
pub(crate) const BOLT_EQUIP_KIND: u32 = 0xbc0;

/// `int BE_SetCharProfileBoltEquipped(CharProfile* prof, int id, int on)`
/// (0x151c90): set or clear bolt `id`'s equipped flag. Equipping an exclusive
/// bolt (def flag 1) first unequips the other members of its group (owned
/// ones, or forced by their flag 8); unequipping a bolt (or one with flag 8)
/// unequips its children. A real change then applies the bolt's dependency
/// links. Returns 0 when a nested change failed, else 1.
pub fn be_set_char_profile_bolt_equipped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S5, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S7, 0x10);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S4, 0x40);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0);
        c.g[S7][0] = c.g[A2][0];
        BOLT_EQUIP_GET_DB.call(c, 0x0015_1cc8)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_KIND));
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S5][0];
        BOLT_EQUIP_FIND_INDEX.call(c, 0x0015_1cdc)?;
        // t0 = the bolt's definition (0x38 bytes each), or null.
        c.g[A0][0] = c.g[V0][0];
        if (c.g[V0][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x38;
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
            c.mult(0, V0, A0, V0);
            c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        } else {
            c.cycles += 2;
            c.g[T0][0] = 0;
        }
        // a1 = the profile's item for the bolt, or null.
        c.cycles += 6;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_LOOKUP));
        c.g[V0][0] = sx((c.g[S5][0] as u32) << 1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = c.lh(c.g[V0][0] as u32);
        let missing = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !missing {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(BOLT_EQUIP_ITEMS));
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        } else {
            c.cycles += 1;
            c.g[A1][0] = 0;
        }
        // a3 = exclusive (def flag 1), a2 = forced (flag 8), s6 = was equipped.
        c.cycles += 8;
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2));
        c.g[A3][0] = c.g[V0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] & 0x8;
        c.g[V1][0] = c.g[A0][0] & 0x10;
        c.g[A2][0] = (c.g[V0][0] != 0) as u64;
        c.g[S6][0] = (c.g[V1][0] != 0) as u64;
        if c.g[S7][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0] | 0x10;
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.g[A0][0] & 0xffef;
        }
        c.cycles += 2;
        c.sh((c.g[A1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
        'exit: {
            'e84: {
                if c.g[S7][0] != 0 {
                    // Equip.
                    c.cycles += 2;
                    if c.g[S6][0] != 0 {
                        c.g[V0][0] = 0x1;
                        break 'exit;
                    }
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S4][0];
                    if c.g[A3][0] == 0 {
                        // e8c: (s6 is 0 here) apply the dependencies.
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S2][0];
                        bolt_equip_apply_deps(c)?;
                        break 'exit;
                    }
                    // Unequip the rest of the exclusive group.
                    c.cycles += 14;
                    c.g[S0][0] = c.lh((c.g[T0][0] as u32).wrapping_add(0x6));
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_KIND));
                    c.g[S3][0] = 0;
                    c.g[A2][0] = c.g[S0][0];
                    BOLT_EQUIP_FIRST_CHILD.call(c, 0x0015_1d7c)?;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_KIND));
                    c.g[A2][0] = c.g[S0][0];
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S4][0];
                    BOLT_EQUIP_COUNT_CHILDREN.call(c, 0x0015_1d90)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break 'e84;
                    }
                    loop {
                        c.cycles += 3;
                        c.g[A1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                        let is_self = c.g[A1][0] == c.g[S5][0];
                        c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
                        'counted: {
                            'skip: {
                                if is_self {
                                    break 'skip;
                                }
                                c.cycles += 5;
                                c.g[V1][0] =
                                    c.lw((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_LOOKUP));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                                let missing = (c.g[V0][0] as i64) < 0;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                if missing {
                                    break 'skip;
                                }
                                c.cycles += 5;
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                c.g[V1][0] =
                                    c.lhu((c.g[V0][0] as u32).wrapping_add(BOLT_EQUIP_ITEMS + 2));
                                c.g[V1][0] = c.g[V1][0] & 0x10;
                                let equipped = c.g[V1][0] != 0;
                                c.g[A0][0] = c.g[S2][0];
                                if !equipped {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                                    c.g[V0][0] = c.g[V0][0] & 0x8;
                                    if c.g[V0][0] == 0 {
                                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                        break 'counted;
                                    }
                                }
                                c.cycles += 4;
                                c.g[A2][0] = 0;
                                BOLT_EQUIP_SELF.call(c, 0x0015_1dec)?;
                                let failed = c.g[V0][0] == 0;
                                c.g[V0][0] = 0;
                                if failed {
                                    break 'exit;
                                }
                            }
                            c.cycles += 1;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S0][0] as i64)) as u64;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                        if c.g[V0][0] == 0 {
                            c.cycles += 2;
                            break 'e84;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_1da0);
                        }
                    }
                } else {
                    // Unequip: also unequip the children (if it was
                    // equipped, or is forced).
                    c.cycles += 2;
                    if c.g[S6][0] != 0 {
                        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_KIND));
                    } else {
                        c.cycles += 2;
                        if c.g[A2][0] == 0 {
                            break 'e84;
                        }
                        c.cycles += 1;
                        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_KIND));
                    }
                    c.cycles += 13;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S5][0];
                    c.g[S3][0] = 0;
                    BOLT_EQUIP_FIRST_CHILD.call(c, 0x0015_1e38)?;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(BOLT_EQUIP_KIND));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S5][0];
                    BOLT_EQUIP_COUNT_CHILDREN.call(c, 0x0015_1e4c)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break 'e84;
                    }
                    c.cycles += 1;
                    loop {
                        c.cycles += 6;
                        c.g[A1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = 0;
                        BOLT_EQUIP_SELF.call(c, 0x0015_1e70)?;
                        let failed = c.g[V0][0] == 0;
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        if failed {
                            if event_due(c) {
                                deliver_event_here(c, 0x0015_1e0c);
                            }
                            c.cycles += 2;
                            c.g[V0][0] = 0;
                            break 'exit;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S1][0] as i64)) as u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                        if c.g[V0][0] == 0 {
                            break 'e84;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0015_1e60);
                        }
                    }
                }
            }
            // e84: apply the dependency links if the state really changed.
            c.cycles += 2;
            if c.g[S7][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = c.g[S2][0];
                if c.g[S6][0] == 0 {
                    bolt_equip_apply_deps(c)?;
                    break 'exit;
                }
                // (s7 is still set: e94 returns 1.)
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                break 'exit;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S2][0];
            if c.g[S6][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'exit;
            }
            bolt_equip_apply_deps(c)?;
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

/// 0x151ea4: `v0 = BE_ApplyBoltDependencies(prof, id, on) ? 1 : 0` (a0 is set).
fn bolt_equip_apply_deps(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 6;
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = c.g[S7][0];
    BOLT_EQUIP_APPLY_DEPS.call(c, 0x0015_1eb0)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = 0;
    if c.g[V1][0] != 0 {
        c.cycles += 1;
        c.g[V0][0] = 0x1;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Rider_ProbeGround (0x1242b0)

/// VU0 `vf4.w = vf3 . vf<b>` (the dot-product macro: `vf6.w = 1`, products,
/// then the accumulate chain).
fn probe_ground_dot(c: &mut Cpu, b: u8) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// Store `vf<v>` at `sp + off` and move it through the GPR `lq/sq` copy the
/// compiled code makes (`v1`.. = scratch quadword in register 2).
fn probe_ground_spill(c: &mut Cpu, v: usize, tmp: u32, dst: u32) {
    let sp_ = sp(c);
    c.sqc2(v, sp_.wrapping_add(tmp));
    c.lq(2, sp_.wrapping_add(tmp));
    c.sq(2, sp_.wrapping_add(dst));
}

/// The ground frame, from `vf4 = normal` and `vf3.x = row1 . normal`:
/// forward = normalize(row1 - normal * dot) to rider+0x3a0, then
/// side = normal x forward left in `v0`-quad register 2 (and at sp+0x30).
fn probe_ground_frame(c: &mut Cpu, s2: u32) {
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    probe_ground_spill(c, 5, 0x1b0, 0x190);
    let sp_ = sp(c);
    c.lqc2(3, sp_.wrapping_add(0x180));
    c.lqc2(4, sp_.wrapping_add(0x190));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    probe_ground_spill(c, 5, 0x1b0, 0x40);
    c.lqc2(3, sp_.wrapping_add(0x40));
    probe_ground_dot(c, 3);
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, sp_.wrapping_add(0x1b0));
    c.lq(2, sp_.wrapping_add(0x1b0));
    c.sq(2, s2.wrapping_add(0x3a0));
    c.sq(2, sp_.wrapping_add(0x30));
    c.lqc2(4, s2.wrapping_add(0x370));
    c.lqc2(5, s2.wrapping_add(0x3a0));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    probe_ground_spill(c, 6, 0x40, 0x30);
}

/// `void Rider_ProbeGround(cRider* self, const Vec4* pos)` (0x1242b0): cast a
/// ray along the ground normal (from 200 above to 100 below `pos`, shifted
/// sideways by the edge slew) through the rider's overlap list. A hit sets the
/// ground poly, normal, contact point, surface type, forward/side frame, the
/// height above ground and the audio braking/speed levels (and fires the
/// landing spray once); a miss resets the ground to the default up frame and
/// silences the board audio. Keeps the speed for the next frame (+0xb04).
pub fn rider_probe_ground(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 96;
        let frame = Frame::open(c, 0x210);
        frame.save(c, S0, 0x1f0);
        frame.save(c, S1, 0x1e0);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x1d0);
        frame.save_fpr(c, 20, 0x200);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x1c0);
        // speed = |GetVelocity()| (vtable slot 0x10 of the rider's +0x6c0 base).
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6c0));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6c0));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0012_42e8)?;
        let s2 = c.g[S2][0] as u32;
        c.lqc2(3, c.g[V0][0] as u32);
        probe_ground_dot(c, 3);
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 20);
        // s = model->unk_140.x * (slew * 45); the ray ends.
        c.lwc1(1, s2.wrapping_add(0x1f0));
        c.g[AT][0] = 0x4234_0000;
        c.mtc1(1, 0);
        c.g[S1][0] = sx(frame.at(c, 0xd0));
        c.g[V0][0] = c.lw(s2.wrapping_add(0x780));
        c.g[A1][0] = sx(frame.at(c, 0x10));
        c.mul_s(1, 1, 0);
        c.g[AT][0] = 0xffff_ffff_c2c8_0000;
        c.mtc1(1, 2);
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x140));
        c.g[A2][0] = sx(frame.at(c, 0x20));
        c.g[AT][0] = 0x4348_0000;
        c.mtc1(1, 3);
        c.g[A0][0] = c.g[S1][0];
        c.mul_s(0, 0, 1);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 12);
        c.mfc1(2, 0);
        let sp_ = sp(c);
        // b = pos + side * s  (sp+0x00)
        c.lqc2(4, s2.wrapping_add(0x3b0));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        probe_ground_spill(c, 5, 0x20, 0x10);
        c.g[A3][0] = 0x2;
        c.lqc2(3, c.g[S0][0] as u32);
        c.lqc2(4, sp_.wrapping_add(0x10));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        probe_ground_spill(c, 5, 0x20, 0x00);
        // lo = b + normal * -100  (sp+0x10)
        c.mfc1(3, 2);
        c.lqc2(4, s2.wrapping_add(0x370));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        probe_ground_spill(c, 5, 0x30, 0x20);
        c.lqc2(3, sp_);
        c.lqc2(4, sp_.wrapping_add(0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        probe_ground_spill(c, 5, 0x30, 0x10);
        // hi = b + normal * 200  (sp+0x20)
        c.mfc1(3, 3);
        c.lqc2(4, s2.wrapping_add(0x370));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        probe_ground_spill(c, 5, 0x40, 0x30);
        c.lqc2(3, sp_);
        c.lqc2(4, sp_.wrapping_add(0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        probe_ground_spill(c, 5, 0x40, 0x20);
        call_ptr(c, 0x0032_e100, 0x0012_43f8)?; // CollSegment_ctor(&ray, lo, hi, 2, 0.5)
        c.g[S0][0] = sx(frame.at(c, 0x50));
        c.g[A0][0] = c.lw(s2.wrapping_add(0x860));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0033_42d0, 0x0012_4410)?; // OverlapList_FindClosestContact
        let s2 = c.g[S2][0] as u32;
        let sp_ = sp(c);
        c.mov_s(2, 0);
        c.lq(2, s2.wrapping_add(0x370));
        c.mtc1(0, 0);
        c.g[A1][0] = c.g[S0][0];
        c.c_lt_s(0, 2);
        let ahead = c.fcc();
        c.sq(2, s2.wrapping_add(0x380));
        // The hit object (+0x150 poly id, +0xa word).
        let mut hit_obj = false;
        if ahead {
            c.cycles += 3;
            c.g[V1][0] = c.lw(sp_.wrapping_add(0xa4));
            c.lwc1(0, sp_.wrapping_add(0xbc));
            if c.g[V1][0] != 0 {
                hit_obj = true;
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x150));
                c.lwc1(1, sp_.wrapping_add(0xc0));
                c.sw(s2.wrapping_add(0x430), c.g[V0][0]);
                c.swc1(0, s2.wrapping_add(0xaac));
                c.swc1(1, s2.wrapping_add(0xab0));
                c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0xa));
                c.sh(s2.wrapping_add(0x2d4), c.g[V0][0]);
            }
        }
        if !hit_obj {
            c.cycles += 3;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            c.sw(s2.wrapping_add(0x430), c.g[V0][0]);
        }
        c.cycles += 5;
        c.mtc1(0, 1);
        c.c_lt_s(2, 1);
        let missed = c.fcc();
        c.g[V0][0] = 0x50_0000;
        if missed {
            // No ground: the default up frame, audio silent.
            c.cycles += 62;
            c.sw(s2.wrapping_add(0xb00), 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
            c.lq(3, s2.wrapping_add(0x1b0));
            c.lq(5, c.g[V0][0] as u32);
            c.g[A0][0] = 0x50_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xee0));
            c.sq(5, s2.wrapping_add(0x370));
            c.sq(3, sp_.wrapping_add(0x1a0));
            c.lq(2, c.g[A0][0] as u32);
            c.sq(3, sp_.wrapping_add(0x180));
            c.sq(2, s2.wrapping_add(0x3d0));
            c.lqc2(3, sp_.wrapping_add(0x1a0));
            c.lqc2(5, s2.wrapping_add(0x370));
            probe_ground_dot(c, 5);
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.mfc1(4, 0);
            c.lqc2(4, s2.wrapping_add(0x370));
            c.qmtc2(4, 3);
            probe_ground_frame(c, s2);
            c.swc1(20, s2.wrapping_add(0xb04));
            c.sq(2, s2.wrapping_add(0x3b0));
            c.swc1(1, s2.wrapping_add(0x75c));
            c.sw(s2.wrapping_add(0x438), 0);
            c.swc1(1, s2.wrapping_add(0x750));
            c.swc1(1, s2.wrapping_add(0x754));
            c.swc1(1, s2.wrapping_add(0x758));
        } else {
            // Landing: the spray, once (when enabled and not grounded yet).
            c.cycles += 3;
            c.g[V0][0] = c.lw(s2.wrapping_add(0xafc));
            let spray = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(sp_.wrapping_add(0x9c));
            if spray {
                c.cycles += 3;
                c.g[V0][0] = c.lw(s2.wrapping_add(0xb00));
                let grounded = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(sp_.wrapping_add(0x9c));
                if !grounded {
                    c.cycles += 15;
                    c.g[A0][0] = c.lw(s2.wrapping_add(0x77c));
                    c.lqc2(3, sp_.wrapping_add(0x70));
                    probe_ground_dot(c, 3);
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(2, 22);
                    c.mtc1(2, 12);
                    c.g[A3][0] = c.lw(sp_.wrapping_add(0x9c));
                    c.g[A2][0] = sx(sp_.wrapping_add(0x60));
                    call_ptr(c, 0x0011_1aa0, 0x0012_45c4)?; // RiderBase_TriggerDynamicSpray
                    c.g[V0][0] = c.lw(sp_.wrapping_add(0x9c));
                }
            }
            // Grounded; the surface type (-1 -> 0).
            c.cycles += 5;
            c.g[V1][0] = 0x1;
            c.sw(s2.wrapping_add(0xb00), c.g[V1][0]);
            c.g[A0][0] = 0xffff_ffff_ffff_ffff;
            c.sw(s2.wrapping_add(0x438), c.g[V0][0]);
            if c.g[V0][0] == c.g[A0][0] {
                c.cycles += 1;
                c.sw(s2.wrapping_add(0x438), 0);
            }
            // Normal, contact point, +0x3d0 from the hit; the ground frame.
            c.cycles += 76;
            c.lq(2, sp_.wrapping_add(0x60));
            c.lq(3, sp_.wrapping_add(0x50));
            c.lq(4, sp_.wrapping_add(0x70));
            c.lq(5, s2.wrapping_add(0x1b0));
            c.sq(2, s2.wrapping_add(0x370));
            c.sq(3, s2.wrapping_add(0x460));
            c.sq(4, s2.wrapping_add(0x3d0));
            c.sq(5, sp_.wrapping_add(0x1a0));
            c.sq(5, sp_.wrapping_add(0x180));
            c.lqc2(3, sp_.wrapping_add(0x1a0));
            c.lqc2(5, s2.wrapping_add(0x370));
            probe_ground_dot(c, 5);
            c.qmfc2(2, 4);
            c.mtc1(2, 0);
            c.mfc1(3, 0);
            c.lqc2(4, s2.wrapping_add(0x370));
            c.qmtc2(3, 3);
            probe_ground_frame(c, s2);
            c.sq(2, s2.wrapping_add(0x3b0));
            // Height above ground: (b - contact) . normal.
            c.lqc2(3, sp_);
            c.lqc2(4, sp_.wrapping_add(0x50));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp_.wrapping_add(0x40));
            c.lq(2, sp_.wrapping_add(0x40));
            c.lwc1(0, s2.wrapping_add(0xb04));
            c.sq(2, sp_.wrapping_add(0x30));
            c.lqc2(3, sp_.wrapping_add(0x30));
            c.lqc2(5, s2.wrapping_add(0x370));
            probe_ground_dot(c, 5);
            c.qmfc2(2, 4);
            c.mtc1(2, 1);
            // Braking: the speed lost since last frame * 0.0036, up to 1.
            c.sub_s(2, 0, 20);
            c.sw(s2.wrapping_add(0x750), 0);
            c.swc1(1, s2.wrapping_add(0x454));
            c.lwc1(0, s2.wrapping_add(0x750));
            c.c_lt_s(2, 0);
            if c.fcc() {
                c.cycles += 9;
                c.neg_s(0, 2);
                let gp = c.g[GP][0] as u32;
                c.lwc1(1, gp.wrapping_sub(0x789c));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 2);
                c.mul_s(0, 0, 1);
                c.c_lt_s(2, 0);
                let over = c.fcc();
                c.swc1(0, s2.wrapping_add(0x750));
                if over {
                    c.cycles += 1;
                    c.swc1(2, s2.wrapping_add(0x750));
                }
            }
            // Speed level: speed * 0.00072, up to 1.
            c.cycles += 8;
            let gp = c.g[GP][0] as u32;
            c.lwc1(0, gp.wrapping_sub(0x7898));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.mul_s(0, 20, 0);
            c.c_lt_s(1, 0);
            let over = c.fcc();
            c.swc1(0, s2.wrapping_add(0x75c));
            if over {
                c.cycles += 1;
                c.swc1(1, s2.wrapping_add(0x75c));
            }
            c.cycles += 4;
            c.lwc1(0, gp.wrapping_sub(0x7894));
            c.swc1(20, s2.wrapping_add(0xb04));
            c.swc1(0, s2.wrapping_add(0x758));
            c.sw(s2.wrapping_add(0x754), 0);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x1f0);
        frame.restore(c, S1, 0x1e0);
        frame.restore(c, S2, 0x1d0);
        frame.restore_ra(c, 0x1c0);
        frame.restore_fpr(c, 20, 0x200);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InputParser_LookupBuiltinName (0x321880)

/// How a `<prefix><number>` name parse ended.
#[derive(Clone, Copy, PartialEq)]
enum BuiltinNameParse {
    /// No digit after the prefix.
    NoDigit,
    /// The number runs into a letter.
    Alpha,
    /// Followed by `_`, or the number is too large.
    Rejected,
    /// `v0` = number + base.
    Accepted,
}

/// Parse the decimal number at `name + off` (s0 = name): `n` in register
/// `n_reg`, the multiplier 10 / the letter flag in `ten_reg`, the next char
/// (unsigned) in `ch_reg`. Accepted when the number is followed by neither a
/// letter nor `_` and is below `limit`; `v0` = n + `base`.
#[allow(clippy::too_many_arguments)]
fn builtin_name_parse(
    c: &mut Cpu,
    off: u32,
    n_reg: usize,
    ten_reg: usize,
    ch_reg: usize,
    limit: i64,
    base: u32,
    head: u32,
    second: bool,
) -> BuiltinNameParse {
    let name = c.g[S0][0] as u32;
    c.cycles += 6;
    c.g[V1][0] = c.lb(name.wrapping_add(off));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
    c.g[V0][0] = c.g[V1][0] & 0xff;
    c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
    c.g[n_reg][0] = c.g[V1][0];
    if c.g[V0][0] == 0 {
        return BuiltinNameParse::NoDigit;
    }
    c.cycles += 5;
    c.g[V0][0] = c.lb(name.wrapping_add(off + 1));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
    c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
    c.g[A1][0] = sx(name.wrapping_add(off + 1));
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[ten_reg][0] = 0xa;
        loop {
            // n = n * 10 + (*p++ - '0')
            c.cycles += 9;
            c.mult(0, V1, n_reg, ten_reg);
            c.g[A0][0] = c.lb(c.g[A1][0] as u32);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = c.lb(c.g[A1][0] as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
            let more = c.g[V0][0] != 0;
            c.g[n_reg][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    }
    // A letter after the number?
    c.cycles += 6;
    c.g[V1][0] = c.lb(c.g[A1][0] as u32);
    c.g[ten_reg][0] = 0;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x61));
    c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
    c.g[ch_reg][0] = c.lbu(c.g[A1][0] as u32);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x41));
        c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
    }
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[ten_reg][0] = 0x1;
    }
    c.cycles += 2;
    if second {
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    }
    if c.g[ten_reg][0] != 0 {
        return BuiltinNameParse::Alpha;
    }
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[ch_reg][0] as u32) << 24);
    c.g[V1][0] = 0x5f;
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
    let underscore = c.g[V0][0] == c.g[V1][0];
    c.g[V0][0] = ((c.g[n_reg][0] as i64) < limit) as u64;
    if underscore {
        return BuiltinNameParse::Rejected;
    }
    c.cycles += 2;
    let small = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[n_reg][0] as u32).wrapping_add(base));
    if small {
        BuiltinNameParse::Accepted
    } else {
        BuiltinNameParse::Rejected
    }
}

/// `int InputParser_LookupBuiltinName(cInputMapParser* self, const char*
/// name)` (0x321880): the built-in config names of an input map: `padtype`
/// -> 0, `arg<N>` (N < 8) -> N + 1, `config<N>` (N < 23) -> N + 9, else -1
/// (case-insensitive prefixes; the number may not run into a letter or `_`).
pub fn input_parser_lookup_builtin_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x4a_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3f18)); // "padtype"
        call_ptr(c, 0x0041_aa88, 0x0032_18a0)?; // strcasecmp
        let differs = c.g[V0][0] != 0;
        c.g[A1][0] = 0x4a_0000;
        'exit: {
            if !differs {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'exit;
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3f20)); // "arg"
            c.g[A2][0] = 0x3;
            call_ptr(c, 0x0041_ab78, 0x0032_18c0)?; // strncasecmp
            let differs = c.g[V0][0] != 0;
            c.g[A1][0] = 0x4a_0000;
            if !differs {
                match builtin_name_parse(c, 3, A0, A2, A1, 8, 1, 0x0032_18f8, false) {
                    BuiltinNameParse::NoDigit => {}
                    BuiltinNameParse::Alpha => c.g[A1][0] = 0x4a_0000,
                    BuiltinNameParse::Rejected => {
                        c.cycles += 1;
                        c.g[A1][0] = 0x4a_0000;
                    }
                    BuiltinNameParse::Accepted => break 'exit,
                }
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3f28)); // "config"
            c.g[A2][0] = 0x6;
            call_ptr(c, 0x0041_ab78, 0x0032_1980)?; // strncasecmp
            let differs = c.g[V0][0] != 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if differs {
                break 'exit;
            }
            match builtin_name_parse(c, 6, A2, A3, A0, 23, 9, 0x0032_19b8, true) {
                BuiltinNameParse::Alpha | BuiltinNameParse::Accepted => {}
                BuiltinNameParse::NoDigit | BuiltinNameParse::Rejected => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cWorldSound_InsertEmitter (0x2b7318)

const INSERT_EMITTER_BOUNDS: Callee = Callee {
    addr: 0x002b_6cf8,
    f: ee::s08::cWorldSound_ComputeRegionBounds,
};

/// `void cWorldSound_InsertEmitter(cWorldSound* self, Trigger* trig)`
/// (0x2b7318): put a trigger with a shape region into the game module's
/// spatial hash (+0x20): compute its region bounds, mark it state 5, build
/// the `{min, 1} {max, 1}` box, take its spatial key and insert it at the
/// hash level the key selects (cell 0x50 / 0x28 / 0x14 offsets).
pub fn c_world_sound_insert_emitter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S0, 0x70);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x50);
        'restored: {
            if c.g[S1][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                frame.restore(c, S0, 0x70);
                if c.g[V0][0] == 0 {
                    break 'restored;
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = c.g[SP][0];
                if c.g[V1][0] == 0 {
                    break 'restored;
                }
                c.cycles += 43;
                c.g[A3][0] = sx(frame.at(c, 0x10));
                INSERT_EMITTER_BOUNDS.call(c, 0x002b_7354)?; // (self, trig, &min, &max)
                call_ptr(c, 0x0028_b1c8, 0x002b_735c)?; // SSXAudioSystem_GetGameModule
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = 0x5;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 6);
                let sp_ = sp(c);
                c.g[A0][0] = sx(sp_.wrapping_add(0x40));
                c.lwc1(5, sp_);
                c.g[A1][0] = sx(sp_.wrapping_add(0x20));
                c.lwc1(4, sp_.wrapping_add(0x4));
                c.lwc1(0, sp_.wrapping_add(0x8));
                c.lwc1(1, sp_.wrapping_add(0x10));
                c.lwc1(2, sp_.wrapping_add(0x14));
                c.lwc1(3, sp_.wrapping_add(0x18));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                // box = {min, 1}, {max, 1} at sp+0x20
                c.swc1(5, sp_.wrapping_add(0x20));
                c.swc1(4, sp_.wrapping_add(0x24));
                c.swc1(0, sp_.wrapping_add(0x28));
                c.swc1(1, sp_.wrapping_add(0x30));
                c.swc1(2, sp_.wrapping_add(0x34));
                c.swc1(3, sp_.wrapping_add(0x38));
                c.swc1(6, sp_.wrapping_add(0x3c));
                c.swc1(6, sp_.wrapping_add(0x2c));
                call_ptr(c, 0x0032_8f28, 0x002b_73b4)?; // SpatialKey_FromAABB(&key, &box)
                                                        // The level: key fields (+4, +8, +0xc) > -1 pick 0, 0x28, 0x14 steps.
                c.g[V0][0] = c.lw(sp_.wrapping_add(0x44));
                c.g[A3][0] = 0xffff_ffff_ffff_ffff;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                c.g[A1][0] = c.lw(sp_.wrapping_add(0x48));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[A2][0] = c.lw(sp_.wrapping_add(0x4c));
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.g[S0][0];
                }
                c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[A1][0] != 0 {
                    c.g[A0][0] = c.g[V1][0];
                }
                c.g[A2][0] = c.g[S1][0];
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                c.g[A1][0] = 0x2;
                if c.g[A3][0] == 0 {
                    c.g[A0][0] = c.g[V0][0];
                }
                c.g[A3][0] = sx(sp_.wrapping_add(0x40));
                call_ptr(c, 0x0032_8c20, 0x002b_73f8)?; // SpatialHash_Insert(level, 2, trig, &key)
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE_GetRiderSizeScale (0x14efa8)

const SIZE_SCALE_GET_CHARACTER: Callee = Callee {
    addr: 0x0014_a080,
    f: ee::s01::BE_GetRiderCharacter,
};
const SIZE_SCALE_GET_ALT_CHARACTER: Callee = Callee {
    addr: 0x0014_a0b0,
    f: ee::s01::BE_GetRiderAltCharacter,
};
const SIZE_SCALE_GET_BE: Callee = Callee {
    addr: 0x0014_dc80,
    f: ee::s01::BELibrary_GetInstance,
};
const SIZE_SCALE_GET_INTERFACE: Callee = Callee {
    addr: 0x0014_dd58,
    f: ee::s01::BELibrary_GetInterface,
};
const SIZE_SCALE_IS_SHARING: Callee = Callee {
    addr: 0x0014_77e8,
    f: ee::s01::BENewPlayerInt_IsSharingCharRecord,
};

/// `float BE_GetRiderSizeScale(void* self, int rider)` (0x14efa8): the
/// rider model's scale. A cheat character (the alt character, when the cheat
/// flag at 0x535c11 is set or for riders past the first) has a fixed scale
/// from the jump table at 0x45a660 (0.98, 0.85, 2, 0.7, 1.5, 1.2, 1.3, 0.75,
/// 1, else 0.9); a rider sharing a character record 0.8; otherwise the
/// character's size percentage (+0x48 of its 0x88-byte record at 0x530970)
/// * 0.01.
pub fn be_get_rider_size_scale(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        SIZE_SCALE_GET_CHARACTER.call(c, 0x0014_efc8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = c.g[V0][0];
        SIZE_SCALE_GET_ALT_CHARACTER.call(c, 0x0014_efd4)?;
        c.g[S1][0] = c.g[V0][0];
        SIZE_SCALE_GET_BE.call(c, 0x0014_efdc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        SIZE_SCALE_GET_INTERFACE.call(c, 0x0014_efe8)?;
        let cheat = c.g[S1][0] != 0;
        c.g[V0][0] = 0x53_0000;
        let gp = c.g[GP][0] as u32;
        'exit: {
            if cheat {
                c.cycles += 3;
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                let flag = c.g[V1][0] != 0;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xa));
                let applies = flag || {
                    c.cycles += 2;
                    c.g[S0][0] != 0
                };
                if applies {
                    // switch (cheat - 10) through the jump table.
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < 0x14) as u64;
                    let in_table = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x46_0000;
                    let mut target = 0x0014_f0a4;
                    if in_table {
                        c.cycles += 6;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x59a0));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                        target = c.g[A0][0] as u32;
                    }
                    // (cycles, scale: a .lit4 at gp-off, or an immediate via `at`)
                    let lit = |c: &mut Cpu, off: u32| {
                        c.cycles += 3;
                        c.lwc1(0, gp.wrapping_sub(off));
                    };
                    let imm = |c: &mut Cpu, bits: u64| {
                        c.cycles += 4;
                        c.g[AT][0] = bits;
                        c.mtc1(1, 0);
                    };
                    match target {
                        0x0014_f028 => lit(c, 0x6e0c),
                        0x0014_f034 => lit(c, 0x6e08),
                        0x0014_f040 => imm(c, 0x4000_0000),
                        0x0014_f050 => lit(c, 0x6e04),
                        0x0014_f05c => imm(c, 0x3fc0_0000),
                        0x0014_f06c => lit(c, 0x6e00),
                        0x0014_f078 => lit(c, 0x6dfc),
                        0x0014_f084 => imm(c, 0x3f40_0000),
                        0x0014_f094 => imm(c, 0x3f80_0000),
                        0x0014_f0a4 => lit(c, 0x6df8),
                        t => {
                            c.jump_out(t, 0x0014_efa8, 0x0014_f100);
                            return Ok(());
                        }
                    }
                    frame.restore(c, S0, 0x30);
                    break 'exit;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            SIZE_SCALE_IS_SHARING.call(c, 0x0014_f0b8)?;
            let sharing = c.g[V0][0] != 0;
            c.g[V1][0] = 0x88;
            if sharing {
                c.cycles += 1;
                c.lwc1(0, gp.wrapping_sub(0x6df0));
            } else {
                c.cycles += 9;
                c.g[V0][0] = 0x53_0000;
                c.mult(0, V1, S2, V1);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x970));
                c.lwc1(1, gp.wrapping_sub(0x6df4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x48));
                c.cvt_s_w(0, 0);
                c.mul_s(0, 0, 1);
            }
            c.cycles += 1;
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

// ---------------------------------------------------------------------------
// The input map's expression parser: comparison, additive and
// multiplicative levels (0x3236d8, 0x323900, 0x323a68).

/// Parser fields: the current token kind and the free node list.
pub(crate) const INPUT_PARSER_TOKEN: u32 = 0x98;
/// Parse node type of a constant (float value at +4).
const INPUT_PARSER_CONST: u64 = 0x4;

const INPUT_PARSER_NEXT_TOKEN: Callee = Callee {
    addr: 0x0032_1af0,
    f: ee::s10::InputParser_NextToken,
};
const INPUT_PARSER_FREE_NODE: Callee = Callee {
    addr: 0x0032_1638,
    f: ee::s10::InputParser_FreeNode,
};
const INPUT_PARSER_ALLOC_NODE: Callee = Callee {
    addr: 0x0032_15f8,
    f: ee::s10::InputParser_AllocNode,
};
const INPUT_PARSER_ADDITIVE: Callee = Callee {
    addr: 0x0032_3900,
    f: ee::s10::InputParser_ParseAdditive,
};
const INPUT_PARSER_MULTIPLICATIVE: Callee = Callee {
    addr: 0x0032_3a68,
    f: ee::s10::InputParser_ParseMultiplicative,
};
const INPUT_PARSER_UNARY: Callee = Callee {
    addr: 0x0032_3bd8,
    f: ee::s10::InputParser_ParseUnary,
};

/// One left-associative binary level (`+ -` or `* /`): its operand parser,
/// tokens, node types and the return addresses of its calls.
struct InputParserLevel {
    operand: Callee,
    /// Tokens of the two operators, and their node types.
    tok: [u64; 2],
    op: [u64; 2],
    /// Multiplicative (`* /`, the inverse is 1/x) or additive (`+ -`, -x).
    mul: bool,
    /// Return addresses: first operand, NextToken, operand, free (failure),
    /// free (folded), alloc (constant chain), alloc (node).
    ret: [u32; 7],
    /// Loop heads: outer (reload the token), inner (token already in v1).
    head: [u32; 2],
}

const INPUT_PARSER_ADDITIVE_LEVEL: InputParserLevel = InputParserLevel {
    operand: INPUT_PARSER_MULTIPLICATIVE,
    tok: [0x14, 0x15],
    op: [0xe, 0xf],
    mul: false,
    ret: [
        0x0032_3924,
        0x0032_3974,
        0x0032_397c,
        0x0032_3994,
        0x0032_39f4,
        0x0032_3a1c,
        0x0032_3a34,
    ],
    head: [0x0032_3940, 0x0032_3944],
};

const INPUT_PARSER_MULTIPLICATIVE_LEVEL: InputParserLevel = InputParserLevel {
    operand: INPUT_PARSER_UNARY,
    tok: [0x16, 0x17],
    op: [0x10, 0x11],
    mul: true,
    ret: [
        0x0032_3a8c,
        0x0032_3adc,
        0x0032_3ae4,
        0x0032_3afc,
        0x0032_3b5c,
        0x0032_3b8c,
        0x0032_3ba4,
    ],
    head: [0x0032_3aa8, 0x0032_3aac],
};

/// The shared body of [`input_parser_parse_additive`] and
/// [`input_parser_parse_multiplicative`]: `left (op right)*`. Constants are
/// folded into the first constant operand (s4); the inverse operator turns a
/// constant right operand into `-x` / `1/x` and chains it with the first
/// operator; anything else becomes a binary node. Returns the tree, or 0 (an
/// operand failed; the left tree is freed).
fn input_parser_binary_level(c: &mut Cpu, lv: &InputParserLevel) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        lv.operand.call(c, lv.ret[0])?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = 0;
        'failed: {
            if c.g[S1][0] == 0 {
                break 'failed;
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            c.g[S4][0] = 0;
            c.g[V0][0] ^= INPUT_PARSER_CONST;
            if c.g[V0][0] == 0 {
                c.g[S4][0] = c.g[S1][0];
            }
            loop {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(INPUT_PARSER_TOKEN));
                loop {
                    // The operator, or done.
                    c.cycles += 3;
                    c.g[V0][0] = lv.tok[0];
                    let first = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = lv.tok[1];
                    if first {
                        c.cycles += 2;
                        c.g[S3][0] = lv.op[0];
                    } else {
                        c.cycles += 2;
                        let second = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = c.g[S1][0];
                        if !second {
                            c.cycles += 2;
                            frame.restore(c, S0, 0x50);
                            c.cycles += 7;
                            input_parser_binary_level_epilogue(c, frame);
                            return Ok(());
                        }
                        c.cycles += 1;
                        c.g[S3][0] = lv.op[1];
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    INPUT_PARSER_NEXT_TOKEN.call(c, lv.ret[1])?;
                    c.g[A0][0] = c.g[S2][0];
                    lv.operand.call(c, lv.ret[2])?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0 {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S1][0];
                        INPUT_PARSER_FREE_NODE.call(c, lv.ret[3])?;
                        c.g[V0][0] = 0;
                        break 'failed;
                    }
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.cycles += 3;
                    c.g[V0][0] = INPUT_PARSER_CONST;
                    if c.g[V1][0] != c.g[V0][0] {
                        // A binary node (op, left, right).
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S2][0];
                        INPUT_PARSER_ALLOC_NODE.call(c, lv.ret[6])?;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                        c.sw(c.g[V0][0] as u32, c.g[S3][0]);
                        break;
                    }
                    c.cycles += 2;
                    let no_const = c.g[S4][0] == 0;
                    c.g[V0][0] = lv.op[0];
                    if no_const {
                        // Start a constant chain: invert the right operand
                        // for the second operator, then node(op0, left, right).
                        c.cycles += 3;
                        c.g[V0][0] = lv.op[1];
                        let inverse = c.g[S3][0] == c.g[V0][0];
                        c.g[S4][0] = c.g[S0][0];
                        if inverse {
                            let rv = (c.g[S0][0] as u32).wrapping_add(0x4);
                            if lv.mul {
                                c.cycles += 5;
                                c.lwc1(1, rv);
                                c.g[AT][0] = 0x3f80_0000;
                                c.mtc1(1, 0);
                                c.div_s(0, 0, 1);
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, rv);
                                c.neg_s(0, 0);
                            }
                            c.swc1(0, rv);
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S2][0];
                        INPUT_PARSER_ALLOC_NODE.call(c, lv.ret[5])?;
                        c.g[V1][0] = lv.op[0];
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                        break;
                    }
                    // Fold the right constant into s4 and free it.
                    c.cycles += 2;
                    let is_first = c.g[S3][0] == c.g[V0][0];
                    c.g[V0][0] = lv.op[1];
                    let mut folded = true;
                    let (s4, s0) = (c.g[S4][0] as u32, c.g[S0][0] as u32);
                    if is_first {
                        c.cycles += 4;
                        c.lwc1(0, s4.wrapping_add(0x4));
                        c.lwc1(1, s0.wrapping_add(0x4));
                        if lv.mul {
                            c.mul_s(0, 0, 1);
                        } else {
                            c.add_s(0, 0, 1);
                        }
                    } else {
                        c.cycles += 2;
                        let is_second = c.g[S3][0] == c.g[V0][0];
                        c.g[A1][0] = c.g[S0][0];
                        if is_second {
                            c.cycles += 3;
                            c.lwc1(0, s4.wrapping_add(0x4));
                            c.lwc1(1, s0.wrapping_add(0x4));
                            if lv.mul {
                                c.div_s(0, 0, 1);
                            } else {
                                c.sub_s(0, 0, 1);
                            }
                        } else {
                            c.cycles += 2;
                            folded = false;
                        }
                    }
                    if folded {
                        c.cycles += 2;
                        c.swc1(0, s4.wrapping_add(0x4));
                        c.g[A1][0] = c.g[S0][0];
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    INPUT_PARSER_FREE_NODE.call(c, lv.ret[4])?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(INPUT_PARSER_TOKEN));
                    if event_due(c) {
                        deliver_event_here(c, lv.head[1]);
                    }
                }
                // left = node; node.right = right.
                c.cycles += 3;
                c.g[S1][0] = c.g[V0][0];
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                if event_due(c) {
                    deliver_event_here(c, lv.head[0]);
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x50);
        c.cycles += 7;
        input_parser_binary_level_epilogue(c, frame);
        Ok(())
    })
}

fn input_parser_binary_level_epilogue(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `ParseNode* InputParser_ParseAdditive(cInputMapParser* self)` (0x323900):
/// `term ((+|-) term)*` over [`input_parser_parse_multiplicative`], folding
/// constants (node types 0xe add, 0xf subtract).
pub fn input_parser_parse_additive(c: &mut Cpu) {
    input_parser_binary_level(c, &INPUT_PARSER_ADDITIVE_LEVEL)
}

/// `ParseNode* InputParser_ParseMultiplicative(cInputMapParser* self)`
/// (0x323a68): `unary ((*|/) unary)*`, folding constants (node types 0x10
/// multiply, 0x11 divide).
pub fn input_parser_parse_multiplicative(c: &mut Cpu) {
    input_parser_binary_level(c, &INPUT_PARSER_MULTIPLICATIVE_LEVEL)
}

/// `ParseNode* InputParser_ParseComparison(cInputMapParser* self)`
/// (0x3236d8): `sum (cmp sum)*` with `> >= < <= == !=` (tokens 0xe..0x13,
/// node types 8, 9, 0xa, 0xb, 0xc, 0xd through the jump table at 0x48dfd0).
/// Two constants compare to a constant 1.0 / 0.0 (the left node is reused,
/// the right one freed); otherwise a binary node. 0 when an operand fails.
pub fn input_parser_parse_comparison(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        INPUT_PARSER_ADDITIVE.call(c, 0x0032_36f8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = 0;
        'exit: {
            if c.g[S0][0] == 0 {
                break 'exit;
            }
            c.cycles += 1;
            // 0x323708: reload the token; 0x32370c: dispatch on it.
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(INPUT_PARSER_TOKEN));
            loop {
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                c.g[V1][0] = (c.g[A0][0] < 0x6) as u64;
                let is_cmp = c.g[V1][0] != 0;
                c.g[V0][0] = 0x49_0000;
                if !is_cmp {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    break 'exit;
                }
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2030));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                let op = match c.g[A0][0] as u32 {
                    0x0032_3734 => 0xa,
                    0x0032_373c => 0xb,
                    0x0032_3744 => 0x9,
                    0x0032_374c => 0x8,
                    0x0032_3754 => 0xc,
                    0x0032_375c => 0xd,
                    t => {
                        c.jump_out(t, 0x0032_36d8, 0x0032_3900);
                        return Ok(());
                    }
                };
                c.cycles += 2;
                c.g[S2][0] = op;
                c.cycles += 7;
                c.g[A0][0] = c.g[S3][0];
                INPUT_PARSER_NEXT_TOKEN.call(c, 0x0032_3774)?;
                c.g[A0][0] = c.g[S3][0];
                INPUT_PARSER_ADDITIVE.call(c, 0x0032_377c)?;
                c.g[S1][0] = c.g[V0][0];
                if c.g[S1][0] == 0 {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S0][0];
                    INPUT_PARSER_FREE_NODE.call(c, 0x0032_3794)?;
                    c.g[V0][0] = 0;
                    break 'exit;
                }
                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                let mut fold = false;
                c.cycles += 3;
                c.g[V0][0] = INPUT_PARSER_CONST;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    fold = c.g[V0][0] == c.g[V1][0];
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x8));
                }
                if !fold {
                    // node(op, left, right)
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S3][0];
                    INPUT_PARSER_ALLOC_NODE.call(c, 0x0032_38d0)?;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                    c.sw(c.g[V0][0] as u32, c.g[S2][0]);
                    c.g[S0][0] = c.g[V0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_3708);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(INPUT_PARSER_TOKEN));
                    continue;
                }
                // Both constant: left.value = (left op right) ? 1 : 0.
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < 0x6) as u64;
                let in_table = c.g[V0][0] != 0;
                c.g[V0][0] = 0x49_0000;
                if in_table {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2010));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let target = c.g[A0][0] as u32;
                    if !matches!(
                        target,
                        0x0032_37d8
                            | 0x0032_37fc
                            | 0x0032_3820
                            | 0x0032_3844
                            | 0x0032_3868
                            | 0x0032_388c
                    ) {
                        c.jump_out(target, 0x0032_36d8, 0x0032_3900);
                        return Ok(());
                    }
                    c.cycles += 7;
                    let (l, r) = (
                        (c.g[S0][0] as u32).wrapping_add(4),
                        (c.g[S1][0] as u32).wrapping_add(4),
                    );
                    c.lwc1(1, l);
                    c.lwc1(0, r);
                    c.mtc1(0, 2);
                    // fcc = the condition for 1.0 (f1 = left, f0 = right).
                    let truth = match target {
                        0x0032_37d8 => {
                            c.c_lt_s(1, 0);
                            c.fcc()
                        }
                        0x0032_37fc => {
                            c.c_le_s(1, 0);
                            c.fcc()
                        }
                        0x0032_3820 => {
                            c.c_le_s(0, 1);
                            c.fcc()
                        }
                        0x0032_3844 => {
                            c.c_lt_s(0, 1);
                            c.fcc()
                        }
                        0x0032_3868 => {
                            c.c_eq_s(1, 0);
                            c.fcc()
                        }
                        _ => {
                            c.c_eq_s(1, 0);
                            !c.fcc()
                        }
                    };
                    if !truth {
                        c.swc1(2, l);
                    } else {
                        if target != 0x0032_388c {
                            c.cycles += 2;
                        }
                        c.cycles += 3;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 2);
                        c.swc1(2, l);
                    }
                }
                // Free the right constant.
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S3][0];
                INPUT_PARSER_FREE_NODE.call(c, 0x0032_38c0)?;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(INPUT_PARSER_TOKEN));
                if event_due(c) {
                    deliver_event_here(c, 0x0032_370c);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// HUD_DrawWrappedText (0x1f2aa0)

const WRAPPED_TEXT_ELEM_SIZE: Callee = Callee {
    addr: 0x001e_9220,
    f: ee::s04::HUD_GetElemSize,
};
const WRAPPED_TEXT_ELEM_POS: Callee = Callee {
    addr: 0x001e_91f8,
    f: ee::s04::HUD_GetElemPos,
};
const WRAPPED_TEXT_ELEM_SCALE: Callee = Callee {
    addr: 0x001e_9290,
    f: ee::s04::HUD_GetElemScale,
};
const WRAPPED_TEXT_ELEM_COLOR: Callee = Callee {
    addr: 0x001e_91a8,
    f: ee::s04::HUD_GetElemColor,
};

/// `void HUD_DrawWrappedText(cOvStateHud* self, cFont* font, HudElemDef*
/// defs, int idx, const char* str, int flags, float* rect)` (0x1f2aa0): split
/// `str` into lines (at most 50 chars) that fit the element's width at its
/// scale, set the font colour and the draw state's 5-bit field from the
/// element, align the block vertically (+0x21: 0 centre, >0 bottom), and draw
/// each line aligned by +0x20 (mirrored around x = 640 with `flags & 1`; the
/// alignment flips too unless `flags & 2`). `rect` (if any) receives the
/// block's {min x, top, max width, height}. Resets the font scale to 1.
pub fn hud_draw_wrapped_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0x230);
        frame.save(c, S0, 0x1f0);
        frame.save(c, S7, 0x180);
        c.g[S0][0] = 0x24;
        c.g[S7][0] = c.g[A3][0];
        frame.save(c, S6, 0x190);
        c.mult(0, V0, S7, S0);
        frame.save(c, S3, 0x1c0);
        frame.save(c, S4, 0x1b0);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S1, 0x1e0);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S2, 0x1d0);
        c.g[S3][0] = c.g[A1][0];
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
        frame.save(c, S5, 0x1a0);
        frame.save(c, FP, 0x170);
        c.g[S1][0] = c.g[T0][0];
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x160);
        for (k, f) in [25usize, 24, 23, 22, 21, 20].into_iter().enumerate() {
            if k == 1 {
                c.g[S2][0] = c.g[T2][0];
            }
            if k == 2 {
                c.g[A2][0] = 0;
            }
            if k == 3 {
                c.g[A0][0] = c.g[S0][0];
            }
            if k == 4 {
                c.g[S5][0] = 0;
            }
            frame.save_fpr(c, f, 0x228 - 8 * k as u32);
        }
        let a = frame.at(c, 0x150);
        c.sw(a, c.g[T1][0]); // flags
        WRAPPED_TEXT_ELEM_SIZE.call(c, 0x001f_2b20)?; // wrap[2] at sp+0
        c.g[A1][0] = sx(frame.at(c, 0x10));
        c.g[A0][0] = c.g[S0][0];
        WRAPPED_TEXT_ELEM_POS.call(c, 0x001f_2b2c)?; // pos[2] at sp+0x10
        c.g[A1][0] = sx(frame.at(c, 0x20));
        c.g[A0][0] = c.g[S0][0];
        WRAPPED_TEXT_ELEM_SCALE.call(c, 0x001f_2b38)?; // scale[2] at sp+0x20
                                                       // The wrap font's scale: f38/f3c = f30/f34 * scale.
        let sp_ = sp(c);
        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
        let wf = c.g[V0][0] as u32;
        c.g[FP][0] = sx(sp_.wrapping_add(0x130));
        c.lwc1(1, sp_.wrapping_add(0x20));
        c.g[A1][0] = c.g[S1][0];
        c.lwc1(0, wf.wrapping_add(0x30));
        c.lwc1(2, wf.wrapping_add(0x34));
        c.mul_s(0, 0, 1);
        c.swc1(0, wf.wrapping_add(0x38));
        c.lwc1(1, sp_.wrapping_add(0x24));
        c.mul_s(2, 2, 1);
        c.swc1(2, wf.wrapping_add(0x3c));
        // Split into lines (50 bytes each at sp+0x30).
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            c.g[S0][0] = sx(sp_.wrapping_add(0x30));
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
            loop {
                c.cycles += 10;
                c.g[A2][0] = c.g[S0][0];
                c.lwc1(13, sp_);
                c.g[A3][0] = 0x32;
                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x32));
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x0039_2430, 0x001f_2b8c)?; // Font_WrapLine
                c.g[A1][0] = c.g[V0][0];
                if c.g[A1][0] == 0 {
                    break;
                }
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                if event_due(c) {
                    deliver_event_here(c, 0x001f_2b70);
                }
            }
        }
        // Colour, the draw state's field, the block height and its top.
        c.cycles += 36;
        c.g[S0][0] = 0x24;
        c.g[A1][0] = c.g[FP][0];
        c.mult(0, V0, S7, S0);
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        WRAPPED_TEXT_ELEM_COLOR.call(c, 0x001f_2bb0)?; // colour at sp+0x130
        let font = c.g[S3][0] as u32;
        c.ldl(V1, sp_.wrapping_add(0x137));
        c.ldr(V1, sp_.wrapping_add(0x130));
        c.ldl(A0, sp_.wrapping_add(0x13f));
        c.ldr(A0, sp_.wrapping_add(0x138));
        c.sdl(V1, font.wrapping_add(0x47));
        c.sdr(V1, font.wrapping_add(0x40));
        c.sdl(A0, font.wrapping_add(0x4f));
        c.sdr(A0, font.wrapping_add(0x48));
        c.mtc1(S5, 3);
        c.cvt_s_w(3, 3);
        c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = c.lw(gp.wrapping_sub(0x854));
        c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x22));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] &= 0x3e0;
        c.g[V0][0] &= c.g[A1][0];
        c.g[V0][0] |= c.g[V1][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.lwc1(0, font.wrapping_add(0x14));
        c.cvt_s_w(0, 0);
        c.lwc1(1, font.wrapping_add(0x34));
        c.lwc1(2, sp_.wrapping_add(0x24));
        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x21));
        c.mul_s(0, 0, 1);
        c.lwc1(13, sp_.wrapping_add(0x14));
        c.mul_s(23, 0, 2); // line height
        c.mul_s(1, 23, 3); // block height
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 0);
            c.mul_s(0, 1, 0);
            c.sub_s(13, 13, 0);
        } else {
            c.cycles += 2;
            if (c.g[V0][0] as i64) > 0 {
                c.sub_s(13, 13, 1);
            }
        }
        c.cycles += 2;
        c.mov_s(22, 13); // y
        if c.g[S2][0] != 0 {
            c.cycles += 6;
            let rect = c.g[S2][0] as u32;
            c.g[AT][0] = 0x4780_0000;
            c.mtc1(1, 0);
            c.swc1(1, rect.wrapping_add(0xc));
            c.swc1(0, rect);
            c.swc1(22, rect.wrapping_add(0x4));
            c.sw(rect.wrapping_add(0x8), 0);
        }
        // Draw the lines.
        c.cycles += 2;
        let any = (c.g[S5][0] as i64) > 0;
        c.g[S1][0] = 0;
        if any {
            c.cycles += 11;
            c.g[V0][0] = 0x24;
            c.g[A0][0] = c.lw(sp_.wrapping_add(0x150));
            c.mult(0, V1, S7, V0);
            c.g[AT][0] = 0x4420_0000;
            c.mtc1(1, 25);
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 24);
            c.g[S4][0] = c.g[A0][0] & 0x2;
            c.g[S0][0] = sx(sp_.wrapping_add(0x30));
            c.g[S7][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S6][0] as u32));
            c.g[S6][0] = c.g[A0][0] & 0x1;
            loop {
                c.cycles += 11;
                c.lwc1(12, font.wrapping_add(0x38));
                c.g[A0][0] = c.g[S3][0];
                c.lwc1(13, font.wrapping_add(0x3c));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0039_1fb0, 0x001f_2cb4)?; // Font_MeasureText
                c.mov_s(21, 0); // w
                c.lwc1(20, sp_.wrapping_add(0x10)); // x
                let mirrored = c.g[S6][0] != 0;
                c.g[V1][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x20));
                if mirrored {
                    c.cycles += 3;
                    c.sub_s(20, 25, 20);
                    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
                    if c.g[S4][0] == 0 {
                        c.g[V1][0] = c.g[V0][0];
                    }
                }
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    c.cycles += 3;
                    c.mul_s(0, 21, 24);
                    c.sub_s(20, 20, 0);
                } else {
                    c.cycles += 2;
                    if (c.g[V1][0] as i64) > 0 {
                        c.sub_s(20, 20, 21);
                    }
                }
                c.cycles += 7;
                c.g[A0][0] = c.g[S3][0];
                c.mov_s(12, 20);
                c.mov_s(13, 22);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_1cb0, 0x001f_2d00)?; // Font_DrawText
                if c.g[S2][0] != 0 {
                    let rect = c.g[S2][0] as u32;
                    c.cycles += 5;
                    c.lwc1(0, rect);
                    c.c_lt_s(20, 0);
                    if c.fcc() {
                        c.swc1(20, rect);
                    }
                    c.cycles += 5;
                    c.lwc1(0, rect.wrapping_add(0x8));
                    c.c_lt_s(0, 21);
                    if c.fcc() {
                        c.swc1(21, rect.wrapping_add(0x8));
                    }
                    c.cycles += 1;
                }
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.cycles += 4;
                c.add_s(22, 22, 23);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S5][0] as i64)) as u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x32));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001f_2c98);
                }
            }
        }
        // Scale back to 1.
        c.cycles += 26;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.lwc1(2, font.wrapping_add(0x30));
        c.lwc1(1, font.wrapping_add(0x34));
        c.swc1(0, sp_.wrapping_add(0x144));
        c.swc1(1, font.wrapping_add(0x3c));
        c.swc1(2, font.wrapping_add(0x38));
        frame.restore(c, S0, 0x1f0);
        frame.restore(c, S1, 0x1e0);
        frame.restore(c, S2, 0x1d0);
        frame.restore(c, S3, 0x1c0);
        frame.restore(c, S4, 0x1b0);
        frame.restore(c, S5, 0x1a0);
        frame.restore(c, S6, 0x190);
        frame.restore(c, S7, 0x180);
        frame.restore(c, FP, 0x170);
        frame.restore_ra(c, 0x160);
        for (k, f) in [25usize, 24, 23, 22, 21, 20].into_iter().enumerate() {
            frame.restore_fpr(c, f, 0x228 - 8 * k as u32);
        }
        c.swc1(0, sp_.wrapping_add(0x140));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FEModule_RiderPreviewApplyTransform (0x19f2d0)

/// Preview fields: the rider model, the two pose halves (0x20 bytes each),
/// the two bones reset to the identity, the "single pose" flag, dirty flag.
pub(crate) const RIDER_PREVIEW_MODEL: u32 = 0x8;
pub(crate) const RIDER_PREVIEW_DIRTY: u32 = 0x14;
pub(crate) const RIDER_PREVIEW_POSE: u32 = 0xc30;
pub(crate) const RIDER_PREVIEW_BONES: [u32; 2] = [0xcac, 0xcb0];
pub(crate) const RIDER_PREVIEW_SINGLE_POSE: u32 = 0xccc;

/// `model->table[self->bone] = *(Quad*)src` (table at model + `table`).
fn rider_preview_reset_bone(
    c: &mut Cpu,
    bone: u32,
    table: u32,
    src_reg: usize,
    model_reg: usize,
    last: bool,
) {
    let s0 = c.g[S0][0] as u32;
    c.g[model_reg][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_MODEL));
    c.g[V0][0] = c.lw(s0.wrapping_add(bone));
    let tbl_reg = if model_reg == V1 { A0 } else { A2 };
    c.g[tbl_reg][0] = c.lw((c.g[model_reg][0] as u32).wrapping_add(table));
    if !last {
        c.lq(V1, c.g[src_reg][0] as u32);
    }
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    if last {
        c.lq(V1, c.g[src_reg][0] as u32);
    }
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[tbl_reg][0] as u32));
    c.sq(V1, c.g[V0][0] as u32);
}

/// `void FEModule_RiderPreviewApplyTransform(FERiderPreview* self)`
/// (0x19f2d0): pose the preview rider. With the single-pose flag both halves
/// of the pose get the first one; otherwise the two halves are the preview's
/// two poses and its two marked bones are reset to the identity quads at
/// 0x4ff130 / 0x4ff220 first. Then `RiderModel_ApplyPose` and
/// `RiderModel_BuildBoneMatrices`; marks the preview dirty.
pub fn fe_module_rider_preview_apply_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S0][0] = c.g[A0][0];
        let s0 = c.g[S0][0] as u32;
        c.g[V0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_SINGLE_POSE));
        c.g[A1][0] = c.g[SP][0];
        let sp_ = sp(c);
        if c.g[V0][0] != 0 {
            c.cycles += 12;
            c.lq(V1, s0.wrapping_add(RIDER_PREVIEW_POSE));
            c.lq(V0, s0.wrapping_add(RIDER_PREVIEW_POSE + 0x10));
            c.g[A0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_MODEL));
            c.sq(V1, sp_.wrapping_add(0x20));
            c.sq(V0, sp_.wrapping_add(0x30));
            c.sq(V1, sp_);
            c.sq(V0, sp_.wrapping_add(0x10));
            call_ptr(c, 0x0031_0200, 0x0019_f30c)?; // RiderModel_ApplyPose(model, pose)
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_MODEL));
            call_ptr(c, 0x0031_03f0, 0x0019_f314)?; // RiderModel_BuildBoneMatrices
        } else {
            c.cycles += 45;
            c.lq(A0, s0.wrapping_add(RIDER_PREVIEW_POSE));
            c.g[A3][0] = 0x50_0000;
            c.lq(A2, s0.wrapping_add(RIDER_PREVIEW_POSE + 0x10));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xed0));
            c.lq(V0, s0.wrapping_add(RIDER_PREVIEW_POSE + 0x30));
            c.g[T0][0] = 0x50_0000;
            c.lq(V1, s0.wrapping_add(RIDER_PREVIEW_POSE + 0x20));
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0xde0));
            c.sq(A0, sp_);
            c.sq(A2, sp_.wrapping_add(0x10));
            c.sq(V0, sp_.wrapping_add(0x30));
            c.sq(V1, sp_.wrapping_add(0x20));
            for (k, bone) in RIDER_PREVIEW_BONES.into_iter().enumerate() {
                rider_preview_reset_bone(c, bone, 0x24, A3, if k == 0 { V1 } else { A0 }, false);
                rider_preview_reset_bone(c, bone, 0x28, T0, A0, k == 1);
            }
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_MODEL));
            call_ptr(c, 0x0031_0200, 0x0019_f3c4)?; // RiderModel_ApplyPose(model, pose)
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_MODEL));
            call_ptr(c, 0x0031_03f0, 0x0019_f3cc)?; // RiderModel_BuildBoneMatrices
        }
        c.g[V0][0] = 0x1;
        c.cycles += 5;
        frame.restore_ra(c, 0x40);
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_DIRTY),
            c.g[V0][0],
        );
        frame.restore(c, S0, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FEModule_RiderPreviewRender (0x19ee88)

/// Preview fields read by the render: the rider type, a pending load, the
/// "outfit" override, the draw layer and the attached effects.
pub(crate) const RIDER_PREVIEW_TYPE: u32 = 0x0;
pub(crate) const RIDER_PREVIEW_LAYER: u32 = 0xc2c;
pub(crate) const RIDER_PREVIEW_FX: u32 = 0xc70;
pub(crate) const RIDER_PREVIEW_OUTFIT: u32 = 0xcd0;
pub(crate) const RIDER_PREVIEW_PENDING: u32 = 0xcd4;

/// `void FEModule_RiderPreviewRender(FERiderPreview* self)` (0x19ee88): draw
/// the front-end rider preview once it is loaded and posed: build its light
/// set (the irradiance entry for its type, types 7/8 and outfit-overridden
/// 0/7/8 using type 6's, plus a light relative to the root bone), set it on
/// the graphics manager, draw the model (at the preview's layer, if any) and
/// its attached effects, then restore the draw state's layer field.
pub fn fe_module_rider_preview_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0xb0);
        'exit: {
            let s0 = c.g[S0][0] as u32;
            c.g[V0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_PENDING));
            if (c.g[V0][0] as i64) >= 0 {
                break 'exit;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_MODEL));
            if c.g[V0][0] == 0 {
                break 'exit;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_DIRTY));
            if c.g[V0][0] == 0 {
                break 'exit;
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0038_9260, 0x0019_eec8)?; // Irradiance_Clear(&lights)
                                                    // The light type (a0).
            let s0 = c.g[S0][0] as u32;
            c.g[V0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_OUTFIT));
            let outfit = c.g[V0][0] != 0;
            c.g[A0][0] = c.lw(s0.wrapping_add(RIDER_PREVIEW_TYPE));
            let lt = |c: &mut Cpu, n: i64| ((c.g[A0][0] as i64) < n) as u64;
            // 0: from 0x19eef8 (v0 = type < 9), 1: 0x19eefc (v0 set), 2: 0x19ef0c.
            let mut stage = 0;
            if outfit {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[A0][0] = 0x6;
                } else {
                    c.cycles += 2;
                    let negative = (c.g[A0][0] as i64) < 0;
                    c.g[V0][0] = lt(c, 9);
                    if !negative {
                        c.cycles += 2;
                        let big = c.g[V0][0] == 0;
                        c.g[V0][0] = lt(c, 7);
                        if big {
                            stage = 2;
                        } else {
                            c.cycles += 2;
                            let small = c.g[V0][0] != 0;
                            c.g[V0][0] = lt(c, 9);
                            if small {
                                stage = 1;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = 0x6;
                            }
                        }
                    }
                }
            }
            if stage == 0 {
                c.cycles += 1;
                c.g[V0][0] = lt(c, 9);
                stage = 1;
            }
            if stage == 1 {
                c.cycles += 2;
                let big = c.g[V0][0] == 0;
                c.g[V1][0] = lt(c, 7);
                if !big {
                    c.cycles += 2;
                    c.g[V0][0] = 0x6;
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = c.g[V0][0];
                    }
                }
            }
            // The type's irradiance entry (jump table at 0x460b40).
            c.cycles += 3;
            c.g[V0][0] = (c.g[A0][0] < 0xa) as u64;
            let in_table = c.g[V0][0] != 0;
            c.g[V0][0] = 0x46_0000;
            if in_table {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb40));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                let (cycles, name) = match c.g[A0][0] as u32 {
                    0x0019_ef30 => (5, 0x1620),
                    0x0019_ef44 => (5, 0x1628),
                    0x0019_ef58 => (5, 0x1980),
                    0x0019_ef6c => (5, 0x1638),
                    0x0019_ef80 => (5, 0x1640),
                    0x0019_ef94 => (5, 0x1988),
                    0x0019_efa8 => (5, 0x1650),
                    0x0019_efbc => (5, 0x1990),
                    0x0019_efd0 => (5, 0x1660),
                    0x0019_efe4 => (4, 0x1998),
                    t => {
                        c.jump_out(t, 0x0019_ee88, 0x0019_f134);
                        return Ok(());
                    }
                };
                c.cycles += cycles;
                c.g[A0][0] = 0x50_0000;
                c.g[A1][0] = 0x4a_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x47f8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(name));
                c.cycles += 11;
                rider_preview_add_light(c, 0x0019_effc, 0x0019_f018)?;
            } else {
                c.cycles += 13;
                c.g[A0][0] = 0x50_0000;
                c.g[A1][0] = 0x4a_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x47f8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x19a0));
                rider_preview_add_light(c, 0x0019_f034, 0x0019_f050)?;
            }
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_MODEL));
            // A light at the root bone.
            c.cycles += 30;
            c.g[A2][0] = 0x4a_0000;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1978));
            c.g[A1][0] = 0;
            call_ptr(c, 0x0031_0c48, 0x0019_f064)?; // RiderModel_GetBoneSkelIndex(model, 0, name)
            let sp_ = sp(c);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_MODEL));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
            c.g[A1][0] = sx(sp_.wrapping_add(0xa0));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.lq(V1, c.g[V0][0] as u32);
            c.sq(V1, sp_.wrapping_add(0xa0));
            call_ptr(c, 0x0038_9cb8, 0x0019_f090)?; // Irradiance_AddCameraRelativeLight
                                                    // GrMan->SetLights(&lights, 1) (vtable slot 0x228).
            let gp = c.g[GP][0] as u32;
            c.g[A3][0] = c.lw(gp.wrapping_sub(0x854));
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x228));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x22c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_f0b0)?;
            // The draw state's layer field (bits 5..9): keep the old one in s1.
            let gp = c.g[GP][0] as u32;
            c.g[V0][0] = c.lw(gp.wrapping_sub(0x854));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_LAYER));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.g[A0][0] & 0x3e0;
            let no_layer = (c.g[V1][0] as i64) < 0;
            c.g[S1][0] = sx((c.g[V0][0] as u32) >> 5);
            if !no_layer {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V0][0] = 0xffff_ffff_ffff_fc1f;
                c.g[V0][0] &= c.g[A0][0];
                c.g[V1][0] &= 0x3e0;
                c.g[V0][0] |= c.g[V1][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            }
            c.cycles += 16;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_MODEL));
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_0640, 0x0019_f0f8)?; // RiderModel_Render(model, 0, 0, 0)
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_PREVIEW_FX));
            call_ptr(c, 0x002f_2088, 0x0019_f100)?; // AttachedEmitterFX_Render
            let gp = c.g[GP][0] as u32;
            c.g[A0][0] = c.lw(gp.wrapping_sub(0x854));
            c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
            c.g[A2][0] = sx((c.g[S1][0] as u32) << 5);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] &= c.g[A1][0];
            c.g[V0][0] |= c.g[A2][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        }
        frame.restore(c, S0, 0xd0);
        c.cycles += 4;
        frame.restore(c, S1, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `Irradiance_AddScaled(&lights, Irradiance_GetEntryByName(set, name),
/// 0x504810, 1.0)` (a0/a1 = set, name are set).
fn rider_preview_add_light(c: &mut Cpu, ret_get: u32, ret_add: u32) -> crate::call::Flow {
    call_ptr(c, 0x0038_ac20, ret_get)?; // Irradiance_GetEntryByName
    c.g[A2][0] = 0x50_0000;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 12);
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4810));
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, 0x0038_9590, ret_add)?; // Irradiance_AddScaled
    Ok(())
}

// ---------------------------------------------------------------------------
// RiderGateSub_UpdateCharge (0x12c230)

const GATE_CHARGE_ANGLE: Callee = Callee {
    addr: 0x0011_5ab0,
    f: ee::s00::Rider_GetGateChargeAngle,
};
const GATE_START_TIME_REACHED: Callee = Callee {
    addr: 0x0012_c028,
    f: ee::s00::RiderGateSub_IsStartTimeReached,
};
const GATE_CAN_START: Callee = Callee {
    addr: 0x0012_c078,
    f: ee::s00::RiderGateSub_CanStart,
};

/// Gate sub-state fields: launched flag, time held at the extreme, the
/// lowest / highest lean reached (the launch thrust once launched), the
/// current lean, the rider.
pub(crate) const GATE_LAUNCHED: u32 = 0x0;
pub(crate) const GATE_HELD: u32 = 0x4;
pub(crate) const GATE_LEAN_MIN: u32 = 0x8;
pub(crate) const GATE_LEAN_MAX: u32 = 0xc;
pub(crate) const GATE_LEAN: u32 = 0x10;
pub(crate) const GATE_RIDER: u32 = 0x14;

/// `void RiderGateSub_UpdateCharge(cGateControl* self, const PadCmd* cmd)`
/// (0x12c230): rock in the start gate. The stick (bits 12..17 of the command,
/// / 31) picks a lean target (forward 0.62, back 0, else 0.31); the lean moves
/// towards it at 0.9/s (4.5 * distance/s when 0.2 or more away) per 1/60 s
/// tick; the knee angles follow the gate charge angle. A changed lean resets
/// the hold timer and extends the min/max. Once the start time is reached and
/// a launch is possible, sets the launch thrust (from the rock amplitude when
/// leaning forward and quick, at least 555.6) and the launched flag.
pub fn rider_gate_sub_update_charge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
        let frame = Frame::open(c, 0x30);
        c.lwc1(1, gp.wrapping_sub(0x7834));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 14);
        c.mtc1(0, 2);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
        frame.save(c, S0, 0x10);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x28);
        c.mul_s(0, 0, 1);
        c.c_lt_s(2, 0);
        let forward = c.fcc();
        c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(GATE_LEAN)); // the old lean
                                                                 // f20 = the lean target.
        if forward {
            c.cycles += 3;
            c.lwc1(20, gp.wrapping_sub(0x7830));
        } else {
            c.cycles += 4;
            c.c_lt_s(0, 2);
            c.lwc1(20, gp.wrapping_sub(0x782c));
            if c.fcc() {
                c.mov_s(20, 2);
            }
        }
        c.cycles += 13;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(GATE_RIDER));
        GATE_CHARGE_ANGLE.call(c, 0x0012_c29c)?;
        let gp = c.g[GP][0] as u32;
        let s0 = c.g[S0][0] as u32;
        c.g[V0][0] = c.lw(s0.wrapping_add(GATE_RIDER));
        let rider = c.g[V0][0] as u32;
        c.lwc1(1, gp.wrapping_sub(0x7828));
        c.swc1(0, rider.wrapping_add(0x2c8));
        c.sw(rider.wrapping_add(0x2cc), 0);
        c.swc1(0, rider.wrapping_add(0x2d0));
        // The rate: 0.9, or 4.5 * |target - lean| from 0.2 away.
        c.lwc1(0, s0.wrapping_add(GATE_LEAN));
        c.sub_s(0, 20, 0);
        c.abs_s(2, 0);
        c.c_le_s(1, 2);
        if c.fcc() {
            c.cycles += 4;
            c.g[AT][0] = 0x4090_0000;
            c.mtc1(1, 0);
            c.mul_s(0, 2, 0);
        } else {
            c.cycles += 1;
            c.lwc1(0, gp.wrapping_sub(0x7824));
        }
        // Step towards the target (f2 = the new lean).
        c.cycles += 8;
        c.lwc1(1, gp.wrapping_sub(0x7820));
        c.lwc1(3, s0.wrapping_add(GATE_LEAN));
        c.mul_s(1, 0, 1);
        c.add_s(0, 20, 1);
        c.c_lt_s(0, 3);
        if c.fcc() {
            c.cycles += 2;
            c.sub_s(2, 3, 1);
        } else {
            c.sub_s(0, 20, 1);
            c.cycles += 4;
            c.c_lt_s(3, 0);
            let below = c.fcc();
            c.mov_s(2, 20);
            if below {
                c.cycles += 1;
                c.add_s(2, 3, 1);
            }
        }
        // Store; the hold timer; a changed lean resets it and extends min/max.
        c.cycles += 7;
        c.lwc1(0, s0.wrapping_add(GATE_HELD));
        c.c_eq_s(21, 2);
        c.lwc1(1, gp.wrapping_sub(0x781c));
        c.swc1(2, s0.wrapping_add(GATE_LEAN));
        c.add_s(0, 0, 1);
        let same = c.fcc();
        c.swc1(0, s0.wrapping_add(GATE_HELD));
        if !same {
            c.cycles += 4;
            c.c_lt_s(21, 2);
            let up = c.fcc();
            c.sw(s0.wrapping_add(GATE_HELD), 0);
            if up {
                c.cycles += 2;
                c.swc1(2, s0.wrapping_add(GATE_LEAN_MAX));
            } else {
                c.cycles += 1;
                c.swc1(2, s0.wrapping_add(GATE_LEAN_MIN));
            }
        }
        'exit: {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            GATE_START_TIME_REACHED.call(c, 0x0012_c358)?;
            if c.g[V0][0] == 0 {
                break 'exit;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            GATE_CAN_START.call(c, 0x0012_c368)?;
            if c.g[V0][0] == 0 {
                break 'exit;
            }
            // The launch thrust (f4).
            let gp = c.g[GP][0] as u32;
            let s0 = c.g[S0][0] as u32;
            c.cycles += 6;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 0);
            c.mtc1(0, 4);
            c.c_lt_s(0, 20);
            if c.fcc() {
                c.cycles += 5;
                c.lwc1(5, s0.wrapping_add(GATE_HELD));
                c.lwc1(0, gp.wrapping_sub(0x7818));
                c.c_lt_s(5, 0);
                if c.fcc() {
                    c.cycles += 12;
                    c.lwc1(2, s0.wrapping_add(GATE_LEAN_MIN));
                    c.lwc1(0, s0.wrapping_add(GATE_LEAN_MAX));
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 1);
                    c.sub_s(0, 0, 2);
                    c.lwc1(3, gp.wrapping_sub(0x7814));
                    c.lwc1(2, s0.wrapping_add(GATE_LEAN));
                    c.add_s(4, 5, 1);
                    c.mul_s(0, 0, 3);
                    c.sub_s(1, 1, 2);
                    c.mul_s(0, 0, 1);
                    c.div_s(4, 0, 4);
                }
            }
            c.cycles += 5;
            c.lwc1(0, gp.wrapping_sub(0x7810));
            c.c_lt_s(4, 0);
            if c.fcc() {
                c.mov_s(4, 0);
            }
            c.cycles += 4;
            c.g[V0][0] = 0x1;
            c.swc1(4, s0.wrapping_add(GATE_LEAN_MAX));
            c.sw(s0.wrapping_add(GATE_LAUNCHED), c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderRailslideState_ComputeRailFrame (0x13bd80)

/// VU0 `vf6 = vf4 x vf5` (the outer-product macro), spilled to `sp+tmp`
/// and copied to `sp+dst`.
fn rail_frame_cross(c: &mut Cpu, tmp: u32, dst: u32) {
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    probe_ground_spill(c, 6, tmp, dst);
}

/// VU0 normalise of `vf3` into `vf5` (`rsqrt` of the dot product).
fn rail_frame_normalize(c: &mut Cpu) {
    probe_ground_dot(c, 3);
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
}

/// `vf5 = *(sp+src) * x` with `x` from GPR `xreg` (`qmtc2`), to `sp+tmp`,
/// copied to `sp+dst`.
fn rail_frame_scale(c: &mut Cpu, src: u32, xreg: usize, tmp: u32, dst: u32) {
    let sp_ = sp(c);
    c.lqc2(4, sp_.wrapping_add(src));
    c.qmtc2(xreg, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    probe_ground_spill(c, 5, tmp, dst);
}

/// `Vec4 RiderRailslideState_ComputeRailFrame(Rail* rail, const Vec4* n)`
/// (0x13bd80, returned through `a0`): the rider's forward direction on a
/// rail with normal `n`. For a shallow normal (|n.z| <= 0.92): the up vector
/// (0x4ff160) minus `n * n.z`, normalised and flipped to agree with the
/// rider's up (+0x50 -> +0x1c0); for a steep one: `b * cos - a * sin` of the
/// rail angle (+0x10), with `a = normalize(up x n)` and `b = n x a`.
pub fn rider_railslide_state_compute_rail_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x90);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x80);
        c.g[S2][0] = c.g[A0][0];
        c.g[A0][0] = sx(frame.at(c, 0x70));
        c.g[A1][0] = sx(frame.at(c, 0x74));
        c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_be50, 0x0013_bdb0)?; // Math_SinCos(&s, &c, angle)
        let sp_ = sp(c);
        let n = c.g[S0][0] as u32;
        let gp = c.g[GP][0] as u32;
        c.lwc1(2, n.wrapping_add(0x8));
        c.g[V0][0] = 0x1;
        c.lwc1(1, gp.wrapping_sub(0x7118));
        c.abs_s(0, 2);
        c.c_lt_s(1, 0);
        if !c.fcc() {
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            // Shallow: normalize(up - n * n.z), facing the rider's up.
            c.cycles += 45;
            c.mfc1(2, 2);
            c.lqc2(4, n);
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp_.wrapping_add(0x40));
            c.mtc1(0, 1);
            c.lq(2, sp_.wrapping_add(0x40));
            c.sq(2, sp_.wrapping_add(0x30));
            c.g[V0][0] = 0x50_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
            c.lqc2(3, c.g[V0][0] as u32);
            c.lqc2(4, sp_.wrapping_add(0x30));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            probe_ground_spill(c, 5, 0x40, 0x20);
            c.lqc2(3, sp_.wrapping_add(0x20));
            rail_frame_normalize(c);
            c.sqc2(5, sp_.wrapping_add(0x40));
            c.lq(3, sp_.wrapping_add(0x40));
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x50));
            c.sq(3, sp_.wrapping_add(0x10));
            c.sq(3, sp_);
            c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x1c0));
            c.sq(2, sp_.wrapping_add(0x10));
            c.lqc2(3, sp_);
            c.lqc2(5, sp_.wrapping_add(0x10));
            probe_ground_dot(c, 5);
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.c_lt_s(0, 1);
            let behind = c.fcc();
            c.lq(3, sp_);
            if behind {
                c.cycles += 9;
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.mtc1(1, 0);
                c.mfc1(2, 0);
                c.lqc2(4, sp_);
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp_);
                c.lq(3, sp_);
            }
        } else {
            // Steep: b * cos - a * sin.
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x50));
            c.cycles += 54;
            c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x1c0));
            c.sq(2, sp_.wrapping_add(0x30));
            c.lqc2(4, sp_.wrapping_add(0x30));
            c.lqc2(5, n);
            rail_frame_cross(c, 0x40, 0x20); // up x n
            c.lqc2(3, sp_.wrapping_add(0x20));
            rail_frame_normalize(c);
            probe_ground_spill(c, 5, 0x40, 0x10); // a
            c.lqc2(4, n);
            c.lqc2(5, sp_.wrapping_add(0x10));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, sp_.wrapping_add(0x30));
            c.lq(2, sp_.wrapping_add(0x30));
            c.lwc1(0, sp_.wrapping_add(0x74));
            c.sq(2, sp_.wrapping_add(0x20)); // b
            c.mfc1(3, 0);
            rail_frame_scale(c, 0x20, 3, 0x50, 0x40); // b * cos
            c.lwc1(0, sp_.wrapping_add(0x70));
            c.mfc1(3, 0);
            rail_frame_scale(c, 0x10, 3, 0x60, 0x50); // a * sin
            c.lqc2(3, sp_.wrapping_add(0x40));
            c.lqc2(4, sp_.wrapping_add(0x50));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp_.wrapping_add(0x60));
            c.lq(2, sp_.wrapping_add(0x60));
            c.sq(2, sp_);
            c.sq(2, sp_.wrapping_add(0x30));
            c.lq(3, sp_);
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0xb0);
        c.sq(3, c.g[S2][0] as u32);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_PlayTrickPointsTick (0x29a7d8)

/// Audio system fields: the trick-points counter's last value and its
/// sound handle.
pub(crate) const AUDIO_POINTS_LAST: u32 = 0x5fa8;
pub(crate) const AUDIO_POINTS_HANDLE: u32 = 0x5fac;

/// `void SSXAudioSystem_PlayTrickPointsTick(SSXAudioSystem* self, Rider*
/// rider, int points)` (0x29a7d8): the score counter's tick. Outside replays,
/// for a rider whose score display is on (+0x874 and +0x87c), when
/// `points % 100` wraps below the last value: stop the old tick if it is still
/// playing, push a sound slot (bank sound 0x4d, defaults like the other
/// pushed sounds) and play it; remember the value.
pub fn ssx_audio_system_play_trick_points_tick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0028_8ae0, 0x0029_a808)?; // SSXAudioSystem_IsReplayPlaying
        'exit: {
            if c.g[V0][0] != 0 {
                break 'exit;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x874));
            c.g[A1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x87c));
                c.g[A1][0] = (c.g[V0][0] != 0) as u64;
            }
            c.cycles += 2;
            let shown = c.g[A1][0] != 0;
            c.g[S4][0] = 0x64;
            if shown {
                c.cycles += 3;
                c.div(0, S1, S4);
                c.cycles += 5;
                let s3 = c.g[S3][0] as u32;
                c.g[V0][0] = c.lw(s3.wrapping_add(AUDIO_POINTS_LAST));
                c.g[S5][0] = c.hi[0];
                c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.sw(s3.wrapping_add(AUDIO_POINTS_LAST), c.g[S5][0]);
                } else {
                    c.cycles += 8;
                    call_ptr(c, 0x003b_58a0, 0x0029_a854)?; // SND_Lock
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(AUDIO_POINTS_HANDLE));
                    call_ptr(c, 0x003b_8ab8, 0x0029_a85c)?; // SND_IsHandleDone
                    c.g[S0][0] = (c.g[V0][0] != 0) as u64;
                    call_ptr(c, 0x003b_58d8, 0x0029_a864)?; // SND_Unlock
                    if c.g[S0][0] == 0 {
                        c.cycles += 8;
                        let s3 = c.g[S3][0] as u32;
                        c.g[V0][0] = c.lw(s3.wrapping_add(0x118));
                        c.g[A2][0] = 0x1;
                        c.g[A1][0] = c.lw(s3.wrapping_add(AUDIO_POINTS_HANDLE));
                        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                        c.mtc1(0, 12);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                        call_ptr(c, 0x002a_d5f0, 0x0029_a888)?; // BankMonitor_StopHandle
                    }
                    // Push a slot on the current track (T = *self, S = T + 0x1d8).
                    c.g[S0][0] = c.lw(c.g[S3][0] as u32);
                    c.cycles += 149;
                    let t = c.g[S0][0] as u32;
                    c.g[T1][0] = 0x18;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0x8;
                    c.g[V0][0] = c.lw(t.wrapping_add(0x1f4));
                    c.g[S1][0] = sx(t.wrapping_add(0x1d8));
                    c.g[S2][0] = 0x4d;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw(t.wrapping_add(0x1f4), c.g[V0][0]);
                    c.g[V1][0] = c.lw(t.wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.mult(0, V1, V1, T1);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.ldl(V0, t.wrapping_add(0x1e3));
                    c.ldr(V0, t.wrapping_add(0x1dc));
                    c.ldl(A0, t.wrapping_add(0x1eb));
                    c.ldr(A0, t.wrapping_add(0x1e4));
                    c.ldl(A3, t.wrapping_add(0x1f3));
                    c.ldr(A3, t.wrapping_add(0x1ec));
                    let e = c.g[V1][0] as u32;
                    c.sdl(V0, e.wrapping_add(0x7));
                    c.sdr(V0, e);
                    c.sdl(A0, e.wrapping_add(0xf));
                    c.sdr(A0, e.wrapping_add(0x8));
                    c.sdl(A3, e.wrapping_add(0x17));
                    c.sdr(A3, e.wrapping_add(0x10));
                    c.g[V0][0] = c.lw(t.wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx(t.wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0);
                    rider_update_script_sounds_put(c, 0x34, 0, false);
                    c.g[A3][0] = c.lw(t.wrapping_add(0x1f4));
                    c.g[A0][0] = c.lw(t.wrapping_add(0x1f4));
                    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                    c.mult(0, A0, A0, T1);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                    c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                    rider_update_script_sounds_put(c, 0x3c, 0, true);
                    c.g[A0][0] = c.lw(t.wrapping_add(0x1f4));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                    call_ptr(c, 0x0041_6210, 0x0029_a960)?; // memset
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[A3][0] = 0x5a;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                    c.g[T0][0] = 0x32;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[T1][0] = 0x7f;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[T2][0] = 0x1;
                    c.sw(c.g[V1][0] as u32, 0);
                    c.g[A1][0] = 0x9;
                    c.g[A2][0] = 0;
                    c.g[A0][0] = c.g[S3][0];
                    rider_update_script_sounds_put(c, 0x64, 0, true);
                    rider_update_script_sounds_put(c, 0x68, 0, false);
                    rider_update_script_sounds_put(c, 0x6c, 0, true);
                    rider_update_script_sounds_put70(c, 0x4, 0, false);
                    let (s4, a3, t0) = (c.g[S4][0], c.g[A3][0], c.g[T0][0]);
                    rider_update_script_sounds_put70(c, 0x0, s4, true);
                    rider_update_script_sounds_put70(c, 0x8, a3, false);
                    rider_update_script_sounds_put70(c, 0xc, t0, true);
                    rider_update_script_sounds_put(c, 0x74, 0, false);
                    rider_update_script_sounds_put(c, 0x80, 0, true);
                    rider_update_script_sounds_put(c, 0x84, 0, false);
                    let (t1, t2) = (c.g[T1][0], c.g[T2][0]);
                    rider_update_script_sounds_put(c, 0x78, t1, true);
                    rider_update_script_sounds_put(c, 0x7c, t2, false);
                    // types[cur] = 0, args[cur] = 0x4d.
                    let s3 = c.g[S3][0] as u32;
                    c.g[A3][0] = c.lw(s3);
                    c.g[V1][0] = c.lw(s3.wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0);
                    c.g[V1][0] = c.lw(s3);
                    c.g[A3][0] = c.lw(s3.wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[S2][0]);
                    c.g[S0][0] = c.lw(s3.wrapping_add(0x118));
                    call_ptr(c, 0x0028_7968, 0x0029_aab8)?; // SSXAudioSystem_GetChannelGainPtr(self, 9, 0)
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    call_ptr(c, 0x0029_06b8, 0x0029_aad8)?; // Audio_PlayPushedSound
                    let s3 = c.g[S3][0] as u32;
                    c.sw(s3.wrapping_add(AUDIO_POINTS_HANDLE), c.g[V0][0]);
                    c.sw(s3.wrapping_add(AUDIO_POINTS_LAST), c.g[S5][0]);
                }
            }
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x60);
        c.cycles += 8;
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

// ---------------------------------------------------------------------------
// InSeq_RampTrackUpdate (0x273698)

const RAMP_TRACK_ACTIVATE_KEY: Callee = Callee {
    addr: 0x0027_3860,
    f: ee::s07::InSeq_RampTrackActivateKey,
};
const RAMP_TRACK_NEXT_KEY: Callee = Callee {
    addr: 0x0027_38f8,
    f: ee::s07::InSeq_RampTrackNextKey,
};

/// Ramp track fields: the next key record, the active ramps (a list linked
/// at +0x20: end time int at +8, value at +0x10, speed at +0x14) and their
/// count.
pub(crate) const RAMP_TRACK_KEY: u32 = 0x14;
pub(crate) const RAMP_TRACK_ACTIVE: u32 = 0x20;
pub(crate) const RAMP_TRACK_COUNT: u32 = 0x24;

/// `bool InSeq_RampTrackUpdate(RampTrack* self, uint* flags, float t, float
/// dt)` (0x273698): advance the track to time `t`. Ramps that ended by `t`
/// are unlinked (flags |= 2); the others move by `speed * dt`. Then every key
/// due by `t` starts its ramp (flags |= 1), advanced by the time since the
/// key. Returns whether anything is left (active ramps or keys).
pub fn in_seq_ramp_track_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x38);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.mov_s(21, 12);
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 20, 0x30);
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = c.lw(this.wrapping_add(RAMP_TRACK_ACTIVE));
        c.g[A0][0] = 0; // the previous ramp
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            loop {
                let e = c.g[V1][0] as u32;
                c.cycles += 6;
                c.lwc1(0, e.wrapping_add(0x8));
                c.cvt_s_w(0, 0);
                c.c_le_s(0, 21);
                if !c.fcc() {
                    // Still running: value += speed * dt.
                    c.lwc1(1, e.wrapping_add(0x14));
                    c.cycles += 6;
                    c.g[A0][0] = c.g[V1][0];
                    c.lwc1(0, e.wrapping_add(0x10));
                    c.mul_s(1, 1, 13);
                    c.add_s(0, 0, 1);
                    c.swc1(0, e.wrapping_add(0x10));
                    c.g[V1][0] = c.lw(e.wrapping_add(0x20));
                } else {
                    // Ended: unlink it.
                    c.cycles += 4;
                    let s1 = c.g[S1][0] as u32;
                    c.g[V0][0] = c.lw(s1) | 0x2;
                    let has_prev = c.g[A0][0] != 0;
                    c.sw(s1, c.g[V0][0]);
                    if has_prev {
                        c.cycles += 2;
                        c.g[V1][0] = c.lw(e.wrapping_add(0x20));
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(e.wrapping_add(0x20));
                        c.g[V1][0] = c.g[V0][0];
                        c.sw(this.wrapping_add(RAMP_TRACK_ACTIVE), c.g[V0][0]);
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(this.wrapping_add(RAMP_TRACK_COUNT));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.sw(this.wrapping_add(RAMP_TRACK_COUNT), c.g[V0][0]);
                }
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_36d0);
                }
            }
        }
        // Start the keys that are due.
        c.cycles += 2;
        c.mtc1(0, 22);
        c.g[V1][0] = c.lw(this.wrapping_add(RAMP_TRACK_KEY));
        loop {
            c.cycles += 2;
            if c.g[V1][0] == 0 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RAMP_TRACK_ACTIVE));
                break;
            }
            c.cycles += 8;
            c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.sub_s(20, 21, 0);
            c.c_le_s(22, 20);
            let due = c.fcc();
            c.g[A0][0] = c.g[S0][0];
            if !due {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RAMP_TRACK_ACTIVE));
                break;
            }
            c.cycles += 14;
            let s1 = c.g[S1][0] as u32;
            c.g[V0][0] = c.lw(s1) | 0x1;
            c.sw(s1, c.g[V0][0]);
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RAMP_TRACK_KEY));
            RAMP_TRACK_ACTIVATE_KEY.call(c, 0x0027_3784)?;
            let ramp = c.g[V0][0] as u32;
            c.lwc1(0, ramp.wrapping_add(0x14));
            c.g[A0][0] = c.g[S0][0];
            c.lwc1(1, ramp.wrapping_add(0x10));
            c.mul_s(0, 20, 0);
            c.add_s(1, 1, 0);
            c.swc1(1, ramp.wrapping_add(0x10));
            RAMP_TRACK_NEXT_KEY.call(c, 0x0027_37a0)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RAMP_TRACK_KEY));
            if event_due(c) {
                deliver_event_here(c, 0x0027_3748);
            }
        }
        c.cycles += 2;
        let active = c.g[V0][0] != 0;
        c.g[V0][0] = 0x1;
        if !active {
            c.cycles += 2;
            let no_key = c.g[V1][0] == 0;
            c.g[V0][0] = 0;
            if !no_key {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_CalcSurfaceLoopGroup2 (0x293948)

const SURFACE_LOOP_EVAL_CURVE: Callee = Callee {
    addr: 0x0029_0b58,
    f: ee::s07::Audio_EvalCurve5,
};

/// Copy a 0x28-byte curve (five x, five y) from `src` to `dst` with the
/// compiled code's unaligned doubleword moves through registers `r`.
fn surface_loop_copy_curve(c: &mut Cpu, src: u32, dst: u32, r: [usize; 3]) {
    c.ldl(r[0], src.wrapping_add(0x7));
    c.ldr(r[0], src);
    c.ldl(r[1], src.wrapping_add(0xf));
    c.ldr(r[1], src.wrapping_add(0x8));
    c.ldl(r[2], src.wrapping_add(0x17));
    c.ldr(r[2], src.wrapping_add(0x10));
    c.sdl(r[0], dst.wrapping_add(0x7));
    c.sdr(r[0], dst);
    c.sdl(r[1], dst.wrapping_add(0xf));
    c.sdr(r[1], dst.wrapping_add(0x8));
    c.sdl(r[2], dst.wrapping_add(0x17));
    c.sdr(r[2], dst.wrapping_add(0x10));
    c.ldl(r[0], src.wrapping_add(0x1f));
    c.ldr(r[0], src.wrapping_add(0x18));
    c.ldl(r[1], src.wrapping_add(0x27));
    c.ldr(r[1], src.wrapping_add(0x20));
    c.sdl(r[0], dst.wrapping_add(0x1f));
    c.sdr(r[0], dst.wrapping_add(0x18));
    c.sdl(r[1], dst.wrapping_add(0x27));
    c.sdr(r[1], dst.wrapping_add(0x20));
}

/// `void SSXAudioSystem_CalcSurfaceLoopGroup2(SSXAudioSystem* self, Rider*
/// rider, int* volume, int* pitch)` (0x293948): the board's surface loop
/// level: speed level (+0x758) * (1 - pressure - braking), faded in below a
/// speed of 0.1 (`* speed * 10`), eased by a quarter of the step towards the
/// rider's previous level (+0x59b8 per listener slot from the rider's vtable
/// slot 0x38) when it moved by more than 0.1; stored, then mapped through
/// the volume curve (0x482c18) and the pitch curve (0x482c40, * 4096).
pub fn ssx_audio_system_calc_surface_loop_group2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xb0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A2][0];
        frame.save_fpr(c, 21, 0xa8);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0x60);
        frame.save_fpr(c, 20, 0xa0);
        let gp = c.g[GP][0] as u32;
        let rider = c.g[A1][0] as u32;
        c.lwc1(21, gp.wrapping_sub(0x45bc)); // 0.1
        c.lwc1(1, rider.wrapping_add(0x754));
        c.lwc1(2, rider.wrapping_add(0x750));
        c.lwc1(12, rider.wrapping_add(0x75c));
        c.add_s(1, 1, 2);
        c.lwc1(20, rider.wrapping_add(0x758));
        c.c_lt_s(12, 21);
        c.sub_s(0, 0, 1);
        let slow = c.fcc();
        c.mul_s(20, 20, 0);
        if slow {
            c.cycles += 4;
            c.mul_s(1, 20, 12);
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(1, 0);
            c.mul_s(20, 1, 0);
        }
        c.cycles += 18;
        c.g[V1][0] = c.lw(rider.wrapping_add(0x6c0));
        c.g[A1][0] = sx(rider.wrapping_add(0x6c0));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0029_39c8)?; // the rider's listener slot
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x59b8));
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(2, c.g[V0][0] as u32);
        c.sub_s(1, 2, 20);
        c.abs_s(0, 1);
        c.c_lt_s(21, 0);
        let moved = c.fcc();
        c.g[V1][0] = 0x48_0000;
        if moved {
            c.cycles += 3;
            c.c_lt_s(20, 2);
            if c.fcc() {
                c.cycles += 5;
                c.g[AT][0] = 0x3e80_0000;
                c.mtc1(1, 0);
                c.mul_s(0, 1, 0);
                c.sub_s(20, 2, 0);
            } else {
                c.cycles += 4;
                c.c_lt_s(2, 20);
                let up = c.fcc();
                c.g[V1][0] = 0x48_0000;
                if up {
                    c.cycles += 5;
                    c.sub_s(0, 20, 2);
                    c.g[AT][0] = 0x3e80_0000;
                    c.mtc1(1, 1);
                    c.mul_s(0, 0, 1);
                    c.add_s(20, 2, 0);
                }
            }
        }
        c.cycles += 68;
        let sp_ = sp(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2c18));
        let src = c.g[T0][0] as u32;
        surface_loop_copy_curve(c, src, sp_, [A0, A2, A3]);
        c.swc1(20, c.g[V0][0] as u32);
        c.g[A1][0] = 0x48_0000;
        c.g[S0][0] = sx(sp_.wrapping_add(0x30));
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c40));
        let src = c.g[A3][0] as u32;
        surface_loop_copy_curve(c, src, sp_.wrapping_add(0x30), [V0, V1, A2]);
        c.mov_s(12, 20);
        SURFACE_LOOP_EVAL_CURVE.call(c, 0x0029_3b04)?; // volume
        c.cvt_w_s(1, 0);
        c.swc1(1, c.g[S1][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 20);
        SURFACE_LOOP_EVAL_CURVE.call(c, 0x0029_3b18)?; // pitch
        c.g[AT][0] = 0x4580_0000;
        c.mtc1(1, 1);
        frame.restore(c, S0, 0x90);
        c.mul_s(0, 0, 1);
        frame.restore(c, S1, 0x80);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 21, 0xa8);
        c.cvt_w_s(1, 0);
        c.swc1(1, c.g[S2][0] as u32);
        frame.restore_fpr(c, 20, 0xa0);
        frame.restore(c, S2, 0x70);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::{check, fill, put_words, scramble, Rng, DATA, GAME_GP};

    /// The libc character-class table `tolower` reads (bit 0: uppercase).
    fn oc396_ctype_upper(c: &mut Cpu) {
        for ch in b'A'..=b'Z' {
            c.bus.write8(0x0049_9c81 + ch as u32, 1);
        }
    }

    #[test]
    fn strncasecmp_matches() {
        check(
            ee::s15::f_0041ab78__generated,
            strncasecmp,
            300,
            &|c, seed| {
                let mut r = Rng(0x0396_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0041_ab78;
                oc396_ctype_upper(c);
                let alpha = b"aAbBzZ0_";
                let len = 1 + r.next_u32() % 6;
                let (s1, s2) = (DATA, DATA + 0x100);
                for i in 0..len {
                    let ch = alpha[(r.next_u32() % 8) as usize];
                    c.bus.write8(s1 + i, ch);
                    // Mostly the same letter in the other case, sometimes a different one.
                    let other = if r.next_u32() % 5 == 0 {
                        alpha[(r.next_u32() % 8) as usize]
                    } else if ch.is_ascii_uppercase() {
                        ch.to_ascii_lowercase()
                    } else {
                        ch.to_ascii_uppercase()
                    };
                    c.bus.write8(s2 + i, other);
                }
                // Different lengths: one string ends early.
                let end2 = if seed % 3 == 0 { len - 1 } else { len };
                c.bus.write8(s2 + end2, 0);
                c.bus.write8(s1 + len, 0);
                c.g[A0][0] = sx(s1);
                c.g[A1][0] = sx(s2);
                c.g[A2][0] = match seed % 5 {
                    0 => 0,
                    1 => 1 + (r.next_u32() % 3) as u64,
                    _ => 8,
                };
            },
        );
    }

    /// Lay out a DMA chain the walker terminates on: ref/data pairs (six
    /// slots per batch once split), then end tags.
    #[test]
    fn cps2_chunk_resolve_patch_dma_chain_matches() {
        check(
            ee::s13::f_003ac048__generated,
            cps2_chunk_resolve_patch_dma_chain,
            300,
            &|c, seed| {
                let mut r = Rng(0x0396_0002 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003a_c048;
                let (ctx, obj, chain) = (DATA + 0x40, DATA + 0x80, DATA + 0x100);
                c.bus.write32(ctx + 0x24, DATA);
                let mut split = seed % 4 == 0;
                c.bus.write32(
                    obj,
                    if split {
                        0x2_0000
                    } else {
                        r.next_u32() & !0x6_0000
                    },
                );
                c.bus.write32(obj + 4, chain - DATA);
                // End tags everywhere the walk might go.
                for i in 0..64 {
                    let a = chain + 0x10 * i;
                    put_words(c, a, &[0x6000_0000, 0, 0, 0]);
                }
                let mut slot = 0u32;
                let mut data_off = 0x4000u32;
                let pairs = 1 + r.next_u32() % 4;
                let mut write_pair =
                    |c: &mut Cpu, r: &mut Rng, slot: u32, may_mark: bool| -> bool {
                        let a = chain + 0x10 * slot;
                        let mark = may_mark && r.next_u32() % 4 == 0;
                        let addr = (r.next_u32() & 0xff0) | if mark { 0x8000_0000 } else { 0 };
                        put_words(c, a, &[0x3000_0000 | (r.next_u32() & 0xf), addr, 0, 0]);
                        let qwc = 1 + r.next_u32() % 4;
                        put_words(c, a + 0x10, &[qwc, data_off, 0, 0]);
                        data_off += 0x100;
                        mark
                    };
                for p in 0..pairs {
                    if write_pair(c, &mut r, slot, true) {
                        split = true;
                    }
                    if split {
                        if p + 1 == pairs {
                            // The slot after the pair's follower is the end.
                            put_words(
                                c,
                                chain + 0x10 * (slot + 2),
                                &[0x3000_0000, r.next_u32() & 0xff0, 0, 0],
                            );
                            put_words(c, chain + 0x10 * (slot + 3), &[0x6000_0000, 0, 0, 0]);
                            break;
                        }
                        write_pair(c, &mut r, slot + 2, true);
                        slot += 6;
                    } else {
                        slot += 2;
                    }
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(obj);
                c.g[A2][0] = sx(ctx);
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    // ------------------------------------------------- gated callee fakes

    use crate::difftest::fakes::{covered, fake, fall_back, hook, machine, LOG_COUNT};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// This module's fake gate and scripted results (by fake-call index & 7).
    pub(crate) const OC396_CTRL: u32 = DATA + 0xf_f000;
    pub(crate) const OC396_MAGIC: u32 = 0x0c39_6a11;
    pub(crate) const OC396_RET: u32 = OC396_CTRL + 0x10;

    /// Log the call like every fake, then return the scripted result for
    /// this call index (sign-extended).
    fn oc396_fake_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fake(c, addr, ra);
        if c.pc == ra {
            let w = c.bus.read32(OC396_RET + 4 * (n & 7));
            c.g[V0][0] = sx(w);
            c.f[0] = w; // float results too
        }
    }

    macro_rules! oc396_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC396_CTRL) != OC396_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                oc396_fake_call(c, $addr);
            }
        };
    }

    oc396_fake!(
        oc396_fake_find_instance,
        OC396_PREV_FIND_INSTANCE,
        0x002a_d550,
        ee::s08::f_002ad550__generated
    );
    oc396_fake!(
        oc396_fake_stop_handle,
        OC396_PREV_STOP_HANDLE,
        0x002a_d5f0,
        ee::s08::f_002ad5f0__generated
    );
    oc396_fake!(
        oc396_fake_listener_index,
        OC396_PREV_LISTENER_INDEX,
        0x0028_8a20,
        ee::s07::f_00288a20__generated
    );
    oc396_fake!(
        oc396_fake_play_pushed,
        OC396_PREV_PLAY_PUSHED,
        0x0029_06b8,
        ee::s07::f_002906b8__generated
    );

    oc396_fake!(
        oc396_fake_closest_contact,
        OC396_PREV_CLOSEST_CONTACT,
        0x0033_42d0,
        ee::s11::f_003342d0__generated
    );
    oc396_fake!(
        oc396_fake_dynamic_spray,
        OC396_PREV_DYNAMIC_SPRAY,
        0x0011_1aa0,
        ee::s00::f_00111aa0__generated
    );

    fn oc396_install_audio_fakes() {
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AD550),
            oc396_fake_find_instance,
            &OC396_PREV_FIND_INSTANCE,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AD5F0),
            oc396_fake_stop_handle,
            &OC396_PREV_STOP_HANDLE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00288A20),
            oc396_fake_listener_index,
            &OC396_PREV_LISTENER_INDEX,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_002906B8),
            oc396_fake_play_pushed,
            &OC396_PREV_PLAY_PUSHED,
        );
    }

    /// The rider's script event: up to three sounds (all due at `when` but
    /// sometimes the last), handles alive / gone / unset, on a track whose
    /// slot index starts at -1 or 0 (so it stays inside the arrays).
    #[test]
    fn rider_update_script_sounds_matches() {
        oc396_install_audio_fakes();
        covered(
            ee::s00::f_00125448__generated,
            rider_update_script_sounds,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0012_5448, 8);
                c.pc = 0x0012_5448;
                c.bus.write32(OC396_CTRL, OC396_MAGIC);
                for k in 0..8 {
                    let v = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(OC396_RET + 4 * k, v);
                }
                let snd = DATA + 0x1000;
                let rider = DATA + 0x2000;
                let track = DATA + 0x4000;
                let sub = track + 0x1d8;
                c.bus.write32(GAME_GP + 0x410, snd); // SSXAudioSystem_GetInstance
                c.bus.write32(snd, track);
                c.bus.write32(snd + 4, DATA + 0x5000); // types
                c.bus.write32(snd + 8, DATA + 0x5100); // args
                c.bus.write32(snd + 0x118, snd); // -> current track
                for i in 0..4 {
                    let id = match r.next_u32() % 3 {
                        0 => u32::MAX,
                        _ => r.next_u32() % 100,
                    };
                    c.bus.write32(rider + RIDER_SCRIPT_SOUND_IDS + 4 * i, id);
                }
                // The script object's bank table: arg at +4.
                c.bus.write32(rider + 0x6d0 + 0x20, DATA + 0x3000);
                c.bus.write32(DATA + 0x3004, r.next_u32());
                // The track: slot index, default config, array pointers.
                c.bus.write32(track + 0x1f4, (seed % 2).wrapping_sub(1));
                fill(c, &mut r, track + 0x1dc, 6);
                c.bus.write32(sub + 0x20, DATA + 0x6000);
                for (k, f) in [
                    0x34u32, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(sub + f, DATA + 0x7000 + 0x100 * k as u32);
                }
                // The event's sounds.
                let when = (r.next_u32() % 50) as i32;
                c.f[12] = (when as f32).to_bits();
                let obj = DATA + 0x9000;
                let n = r.next_u32() % 4;
                let mut next = 0u32;
                for k in (0..n).rev() {
                    let e = DATA + 0x9100 + 0x40 * k;
                    c.bus.write32(e, r.next_u32() % 64);
                    let late = k + 1 == n && r.next_u32() % 4 == 0;
                    c.bus.write32(e + 4, (when + late as i32) as u32);
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                    let vol = [0xfbu8, 0, 50, 127, 126, 0x80][(r.next_u32() % 6) as usize];
                    c.bus.write8(DATA + 0x8000 + k, vol);
                    c.bus.write32(e + 0x1c, DATA + 0x8000 + k);
                    c.bus.write32(e + 0x20, next);
                    next = e;
                }
                c.bus.write32(obj + 0xc, next);
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(obj);
                c.g[A2][0] = [0u64, 1, 2, 3][(seed % 4) as usize];
            },
        );
    }

    /// A small bolt database (eight bolts, child lists that only point at
    /// higher ids, so the unequip recursion ends; no dependency links) and a
    /// profile with some bolts equipped or missing.
    #[test]
    fn be_set_char_profile_bolt_equipped_matches() {
        check(
            ee::s01::f_00151c90__generated,
            be_set_char_profile_bolt_equipped,
            300,
            &|c, seed| {
                let mut r = Rng(0x0396_0004 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0015_1c90;
                let db = 0x004a_6750;
                let kind = seed % 2;
                let (defs, idx, first, cnt) = (
                    DATA + 0x1_0000,
                    DATA + 0x1_1000,
                    DATA + 0x1_1100,
                    DATA + 0x1_1200,
                );
                c.bus.write32(db + 4, defs);
                c.bus.write32(db + 0x4dc + 4 * kind, idx);
                c.bus.write32(db + 0x194 + 4 * kind, first);
                c.bus.write32(db + 0x11c + 4 * kind, cnt);
                for id in 0..8u32 {
                    let def = if id == 7 && r.next_u32() % 4 == 0 {
                        0xffff
                    } else {
                        id
                    };
                    c.bus.write16(idx + 2 * id, def as u16);
                    let d = defs + 0x38 * id;
                    c.bus.write16(d + 6, (r.next_u32() % 8) as u16);
                    c.bus.write32(d + 0x34, r.next_u32() & 0x9);
                    // Children of list `id`: entries 8 + 2*id, others > id.
                    let n = if id == 7 { 0 } else { r.next_u32() % 3 };
                    c.bus.write16(first + 2 * id, (8 + 2 * id) as u16);
                    c.bus.write16(cnt + 2 * id, n as u16);
                    for k in 0..2 {
                        let e = defs + 0x38 * (8 + 2 * id + k);
                        let other = id + 1 + r.next_u32() % (7 - id).max(1);
                        c.bus.write16(e + 4, other.min(7) as u16);
                        c.bus.write32(e + 0x34, r.next_u32() & 0x8);
                    }
                }
                let prof = DATA + 0x1_2000;
                let lookup = DATA + 0x1_3000;
                c.bus.write8(prof + BOLT_EQUIP_KIND, kind as u8);
                c.bus.write32(prof + BOLT_EQUIP_LOOKUP, lookup);
                for id in 0..8u32 {
                    let j = if r.next_u32() % 6 == 0 { 0xffff } else { id };
                    c.bus.write16(lookup + 2 * id, j as u16);
                    let flags = (r.next_u32() & 0xffff) as u16 & !0x10
                        | if r.next_u32() % 2 == 0 { 0x10 } else { 0 };
                    c.bus.write16(prof + BOLT_EQUIP_ITEMS + 4 * id + 2, flags);
                }
                c.g[A0][0] = sx(prof);
                c.g[A1][0] = (r.next_u32() % 8) as u64;
                c.g[A2][0] = (seed / 2 % 2) as u64;
            },
        );
    }

    /// The ground probe with the overlap query and the spray faked: a
    /// contact ahead (with or without a hit object), none, or behind; the
    /// spray enabled or not, grounded or not; braking and speed over or under
    /// their clamps.
    #[test]
    fn rider_probe_ground_matches() {
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_003342D0),
            oc396_fake_closest_contact,
            &OC396_PREV_CLOSEST_CONTACT,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00111AA0),
            oc396_fake_dynamic_spray,
            &OC396_PREV_DYNAMIC_SPRAY,
        );
        covered(
            ee::s00::f_001242b0__generated,
            rider_probe_ground,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0012_42b0, 3);
                c.pc = 0x0012_42b0;
                c.bus.write32(OC396_CTRL, OC396_MAGIC);
                let t = [1.5f32, -1.0, 0.0, 0.25][(seed % 4) as usize];
                c.bus.write32(OC396_RET + 4, t.to_bits());
                c.bus.write32(OC396_RET + 8, r.next_u32());
                // GetVelocity (fake call 0) returns the velocity vector.
                let vel = DATA + 0x3100;
                c.bus.write32(crate::difftest::fakes::LOG_RESULTS, vel);
                let speed = [10.0f32, 500.0, 2000.0][(r.next_u32() % 3) as usize];
                put_words(c, vel, &[speed.to_bits(), r.f32(50.0), r.f32(50.0), 0]);
                let rider = DATA + 0x2_0000;
                let vt = DATA + 0x3000;
                c.bus.write16(vt + 0x10, 0);
                c.bus.write32(vt + 0x14, crate::difftest::fakes::FAKE);
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write32(rider + 0x1f0, r.f32(1.0));
                c.bus.write32(rider + 0x780, DATA + 0x3200);
                c.bus.write32(DATA + 0x3200 + 0x140, r.f32(2.0));
                for off in [0x3b0u32, 0x370, 0x1b0] {
                    put_words(
                        c,
                        rider + off,
                        &[r.f32(1.0), r.f32(1.0), r.f32(1.0), 0x3f80_0000],
                    );
                }
                c.bus.write32(rider + 0x860, r.next_u32());
                c.bus.write32(rider + 0xafc, (seed / 4 % 2) as u32);
                c.bus.write32(rider + 0xb00, (seed / 8 % 2) as u32);
                c.bus.write32(rider + 0x77c, r.next_u32());
                let prev_speed = [speed + 400.0, speed - 10.0, speed][(r.next_u32() % 3) as usize];
                c.bus.write32(rider + 0xb04, prev_speed.to_bits());
                let pos = DATA + 0x3300;
                put_words(
                    c,
                    pos,
                    &[r.f32(100.0), r.f32(100.0), r.f32(100.0), 0x3f80_0000],
                );
                // .lit4 constants and the default ground vectors.
                c.bus.write32(GAME_GP - 0x789c, 0.0036f32.to_bits());
                c.bus.write32(GAME_GP - 0x7898, 0.00072f32.to_bits());
                c.bus.write32(GAME_GP - 0x7894, 0.3f32.to_bits());
                put_words(c, 0x004f_f160, &[0, 0x3f80_0000, 0, 0]);
                put_words(c, 0x004f_f120, &[0, 0, 0x3f80_0000, 0]);
                // The hit record the query fills (in the caller's frame).
                let frame = crate::difftest::SP - 0x210;
                for i in 0..0x20 {
                    c.bus.write32(frame + 0x50 + 4 * i, r.f32(5.0));
                }
                let obj = DATA + 0x3400;
                c.bus.write32(obj + 0x150, r.next_u32());
                c.bus.write16(obj + 0xa, r.next_u32() as u16);
                c.bus
                    .write32(frame + 0xa4, if r.next_u32() % 4 == 0 { 0 } else { obj });
                c.bus.write32(
                    frame + 0x9c,
                    if r.next_u32() % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 8
                    },
                );
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(pos);
            },
        );
    }

    /// The input map's built-in names, from a pool of near misses.
    #[test]
    fn input_parser_lookup_builtin_name_matches() {
        const NAMES: &[&[u8]] = &[
            b"padtype",
            b"PadType",
            b"padtyp",
            b"arg0",
            b"ARG7",
            b"arg8",
            b"arg12",
            b"arg3x",
            b"arg3_",
            b"arg",
            b"argx",
            b"config0",
            b"Config22",
            b"config23",
            b"config5Z",
            b"config9_",
            b"config",
            b"configs",
            b"conf1",
            b"x",
            b"",
            b"arg05",
            b"config017",
        ];
        check(
            ee::s10::f_00321880__generated,
            input_parser_lookup_builtin_name,
            300,
            &|c, seed| {
                let mut r = Rng(0x0396_0006 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0032_1880;
                oc396_ctype_upper(c);
                put_str_oc396(c, 0x004a_3f18, b"padtype");
                put_str_oc396(c, 0x004a_3f20, b"arg");
                put_str_oc396(c, 0x004a_3f28, b"config");
                let name = DATA + 0x100;
                put_str_oc396(c, name, NAMES[(seed as usize) % NAMES.len()]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(name);
            },
        );
    }

    /// A NUL-terminated string at `at`.
    fn put_str_oc396(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    oc396_fake!(
        oc396_fake_spatial_key,
        OC396_PREV_SPATIAL_KEY,
        0x0032_8f28,
        ee::s10::f_00328f28__generated
    );
    oc396_fake!(
        oc396_fake_spatial_insert,
        OC396_PREV_SPATIAL_INSERT,
        0x0032_8c20,
        ee::s10::f_00328c20__generated
    );

    /// A trigger with one or two sphere shapes (real bounds), the spatial
    /// key and insert faked; the key words on the stack decide the level.
    #[test]
    fn c_world_sound_insert_emitter_matches() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328F28),
            oc396_fake_spatial_key,
            &OC396_PREV_SPATIAL_KEY,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328C20),
            oc396_fake_spatial_insert,
            &OC396_PREV_SPATIAL_INSERT,
        );
        covered(
            ee::s08::f_002b7318__generated,
            c_world_sound_insert_emitter,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_7318, 2);
                c.pc = 0x002b_7318;
                c.bus.write32(OC396_CTRL, OC396_MAGIC);
                // The game module's spatial hash.
                let world = DATA + 0x1000;
                let module = DATA + 0x1100;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, module);
                c.bus.write32(module + 0x20, DATA + 0x1200);
                let trig = DATA + 0x2000;
                let region = DATA + 0x2100;
                let origin = DATA + 0x2200;
                c.bus
                    .write32(trig + 0xc, if seed % 7 == 0 { 0 } else { origin });
                c.bus.write32(trig + 0x10, region);
                for i in 0..3 {
                    c.bus.write32(origin + 0x40 + 4 * i, r.f32(100.0));
                }
                let n = if seed % 5 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 2
                };
                c.bus.write32(region, 1);
                c.bus.write32(region + 4, n);
                for k in 0..n {
                    let shape = region + 0xc + 0x1c * k;
                    c.bus.write32(shape, 0);
                    for i in 1..7 {
                        c.bus.write32(shape + 4 * i, r.f32(50.0));
                    }
                }
                c.bus.write32(GAME_GP - 0x4544, 1.0e30f32.to_bits());
                c.bus.write32(GAME_GP - 0x4540, (-1.0e30f32).to_bits());
                c.g[A0][0] = sx(DATA + 0x3000);
                c.g[A1][0] = if seed % 11 == 0 { 0 } else { sx(trig) };
            },
        );
    }

    oc396_fake!(
        oc396_fake_get_character,
        OC396_PREV_GET_CHARACTER,
        0x0014_a080,
        ee::s01::f_0014a080__generated
    );
    oc396_fake!(
        oc396_fake_get_alt_character,
        OC396_PREV_GET_ALT_CHARACTER,
        0x0014_a0b0,
        ee::s01::f_0014a0b0__generated
    );
    oc396_fake!(
        oc396_fake_get_be,
        OC396_PREV_GET_BE,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated
    );
    oc396_fake!(
        oc396_fake_get_interface,
        OC396_PREV_GET_INTERFACE,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );
    oc396_fake!(
        oc396_fake_is_sharing,
        OC396_PREV_IS_SHARING,
        0x0014_77e8,
        ee::s01::f_001477e8__generated
    );

    fn oc396_install_be_fakes() {
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014A080),
            oc396_fake_get_character,
            &OC396_PREV_GET_CHARACTER,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014A0B0),
            oc396_fake_get_alt_character,
            &OC396_PREV_GET_ALT_CHARACTER,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc396_fake_get_be,
            &OC396_PREV_GET_BE,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc396_fake_get_interface,
            &OC396_PREV_GET_INTERFACE,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_001477E8),
            oc396_fake_is_sharing,
            &OC396_PREV_IS_SHARING,
        );
    }

    /// Every cheat character (and ids past the table), with and without the
    /// cheat flag, for the first and later riders; shared records; the size
    /// table.
    #[test]
    fn be_get_rider_size_scale_matches() {
        oc396_install_be_fakes();
        covered(
            ee::s01::f_0014efa8__generated,
            be_get_rider_size_scale,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_efa8, 5);
                c.pc = 0x0014_efa8;
                c.bus.write32(OC396_CTRL, OC396_MAGIC);
                let ch = r.next_u32() % 12;
                let cheat = if seed % 4 == 0 { 0 } else { 8 + seed / 4 % 26 };
                put_words(
                    c,
                    OC396_RET,
                    &[ch, cheat, DATA + 0x500, 0, r.next_u32() % 2],
                );
                put_words(
                    c,
                    0x0045_a660,
                    &[
                        0x14f028, 0x14f034, 0x14f0a4, 0x14f094, 0x14f034, 0x14f084, 0x14f0a4,
                        0x14f084, 0x14f094, 0x14f094, 0x14f06c, 0x14f094, 0x14f078, 0x14f078,
                        0x14f0a4, 0x14f0a4, 0x14f05c, 0x14f0a4, 0x14f050, 0x14f040,
                    ],
                );
                for (k, v) in [0.98f32, 0.85, 0.7, 1.2, 1.3, 0.9, 0.01, 0.8]
                    .into_iter()
                    .enumerate()
                {
                    c.bus.write32(GAME_GP - 0x6e0c + 4 * k as u32, v.to_bits());
                }
                c.bus.write8(0x0053_5c11, (r.next_u32() % 2) as u8);
                c.bus
                    .write32(0x0053_0970 + 0x88 * ch + 0x48, 80 + r.next_u32() % 40);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (r.next_u32() % 3) as u64;
            },
        );
    }

    // ------------------------------------------------- expression parser

    /// Run guest function `f` at `addr` with `a0` from a setup, back to `RA`.
    fn oc396_run(c: &mut Cpu, addr: u32, f: GuestFn, a0: u32) {
        c.g[A0][0] = sx(a0);
        c.g[RA][0] = sx(0x0000_4f00);
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, 0x0000_4f00, "setup call {addr:#x} did not return");
        c.g[RA][0] = crate::difftest::RA as u64;
    }

    /// A parser over `text` with its first token read (the real tokenizer),
    /// a free list of 24 nodes, entered at `entry`.
    fn oc396_parser(c: &mut Cpu, seed: u32, text: &[u8], entry: u32) {
        machine(c, seed, entry as u64, 0);
        crate::difftest::elf_image(c);
        crate::game::fe::feasyncfile::tests::env(c);
        let this = DATA + 0x400;
        let buf = DATA + 0x1000;
        for off in (0..0x100).step_by(4) {
            c.bus.write32(this + off, 0);
        }
        put_str_oc396(c, buf, text);
        c.bus.write32(this, buf); // text
        c.bus.write32(this + 4, text.len() as u32); // length
        c.bus.write32(this + 8, buf); // position
        put_str_oc396(c, this + 0xc, b"pad.map");
        c.bus.write32(this + 0x8c, 1); // line
        let nodes = DATA + 0x2000;
        for i in 0..24 {
            let next = if i == 23 { 0 } else { nodes + 0x10 * (i + 1) };
            put_words(c, nodes + 0x10 * i, &[0, next, 0, 0]);
        }
        c.bus.write32(this + 0xe0, nodes);
        oc396_run(c, 0x0032_1af0, ee::s10::f_00321af0, this);
        c.pc = entry;
        c.g[A0][0] = sx(this);
    }

    #[test]
    fn input_parser_parse_comparison_matches() {
        const EXPRS: &[&[u8]] = &[
            b"1<2",
            b"2>=2",
            b"3>4",
            b"1<=0",
            b"2==2",
            b"2!=3",
            b"5==4",
            b"3!=3",
            b"1<2<3",
            b"1+2>2",
            b"arg1<2",
            b"2>arg3",
            b"arg1==arg2",
            b"1<",
            b"",
            b"7",
            b"padtype!=1",
            b"1>=2>=0",
            b"4<=4",
            b"2>1",
            b"0.5<=0.25",
            b"1==1!=0",
        ];
        check(
            ee::s10::f_003236d8__generated,
            input_parser_parse_comparison,
            2 * EXPRS.len() as u32,
            &|c, seed| {
                oc396_parser(c, seed, EXPRS[seed as usize % EXPRS.len()], 0x0032_36d8);
            },
        );
    }

    #[test]
    fn input_parser_parse_additive_matches() {
        const EXPRS: &[&[u8]] = &[
            b"1+2",
            b"5-3",
            b"1+2-4+8",
            b"arg1+2",
            b"2+arg1",
            b"arg1-1+3",
            b"-1+2",
            b"1-",
            b"3",
            b"2*3+1",
            b"1+2*3",
            b"arg2+arg3-1",
            b"1+2<3",
            b"+",
            b"1-arg1",
            b"arg1-arg2",
            b"4-2-1",
        ];
        check(
            ee::s10::f_00323900__generated,
            input_parser_parse_additive,
            2 * EXPRS.len() as u32,
            &|c, seed| {
                oc396_parser(c, seed, EXPRS[seed as usize % EXPRS.len()], 0x0032_3900);
            },
        );
    }

    #[test]
    fn input_parser_parse_multiplicative_matches() {
        const EXPRS: &[&[u8]] = &[
            b"2*3",
            b"8/2",
            b"2*3/4*5",
            b"arg1*2",
            b"2*arg1",
            b"arg1/4*2",
            b"3*",
            b"6",
            b"2/0",
            b"1*2+3",
            b"arg1*arg2",
            b"4/arg1",
            b"*",
            b"2/4/8",
            b"arg2/arg3",
        ];
        check(
            ee::s10::f_00323a68__generated,
            input_parser_parse_multiplicative,
            2 * EXPRS.len() as u32,
            &|c, seed| {
                oc396_parser(c, seed, EXPRS[seed as usize % EXPRS.len()], 0x0032_3a68);
            },
        );
    }

    oc396_fake!(
        oc396_fake_elem_size,
        OC396_PREV_ELEM_SIZE,
        0x001e_9220,
        ee::s04::f_001e9220__generated
    );
    oc396_fake!(
        oc396_fake_elem_pos,
        OC396_PREV_ELEM_POS,
        0x001e_91f8,
        ee::s04::f_001e91f8__generated
    );
    oc396_fake!(
        oc396_fake_elem_scale,
        OC396_PREV_ELEM_SCALE,
        0x001e_9290,
        ee::s04::f_001e9290__generated
    );
    oc396_fake!(
        oc396_fake_elem_color,
        OC396_PREV_ELEM_COLOR,
        0x001e_91a8,
        ee::s04::f_001e91a8__generated
    );
    oc396_fake!(
        oc396_fake_measure_text,
        OC396_PREV_MEASURE_TEXT,
        0x0039_1fb0,
        ee::s12::f_00391fb0__generated
    );
    oc396_fake!(
        oc396_fake_draw_text,
        OC396_PREV_DRAW_TEXT,
        0x0039_1cb0,
        ee::s12::f_00391cb0__generated
    );

    /// Wrapped HUD text with the element queries and the font's measure and
    /// draw faked, the real line splitter on a simple fixed-width font: a few
    /// strings (one to three lines, none), every alignment, mirrored or not,
    /// with or without the rectangle.
    #[test]
    fn hud_draw_wrapped_text_matches() {
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E9220),
            oc396_fake_elem_size,
            &OC396_PREV_ELEM_SIZE,
        );
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E91F8),
            oc396_fake_elem_pos,
            &OC396_PREV_ELEM_POS,
        );
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E9290),
            oc396_fake_elem_scale,
            &OC396_PREV_ELEM_SCALE,
        );
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E91A8),
            oc396_fake_elem_color,
            &OC396_PREV_ELEM_COLOR,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391FB0),
            oc396_fake_measure_text,
            &OC396_PREV_MEASURE_TEXT,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391CB0),
            oc396_fake_draw_text,
            &OC396_PREV_DRAW_TEXT,
        );
        const TEXTS: &[&[u8]] = &[
            b"HELLO WORLD",
            b"THE QUICK BROWN FOX JUMPS",
            b"A",
            b"",
            b"VERYLONGWORDWITHOUTSPACES",
            b"  LEADING SPACES",
            b"ONE TWO THREE FOUR FIVE",
        ];
        covered(
            ee::s04::f_001f2aa0__generated,
            hud_draw_wrapped_text,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001f_2aa0, 10);
                c.pc = 0x001f_2aa0;
                c.bus.write32(OC396_CTRL, OC396_MAGIC);
                for k in 0..8 {
                    let w = (r.next_u32() % 150) as f32;
                    c.bus.write32(OC396_RET + 4 * k, w.to_bits());
                }
                // The element's wrap size, position, scale and colour (the fakes
                // leave the caller's frame as it is).
                let fb = crate::difftest::SP - 0x230;
                put_words(c, fb, &[100.0f32.to_bits(), 20.0f32.to_bits()]);
                put_words(c, fb + 0x10, &[r.f32(300.0), r.f32(200.0)]);
                let sc = [1.0f32, 0.5, 2.0][(r.next_u32() % 3) as usize];
                put_words(c, fb + 0x20, &[sc.to_bits(), 1.0f32.to_bits()]);
                fill(c, &mut r, fb + 0x130, 4);
                // The font: chars 0x20..0x7e, 6 wide, advance 8.
                let this = DATA + 0x100;
                let font = DATA + 0x1000;
                let glyphs = DATA + 0x2000;
                c.bus.write32(this + 0x428, font);
                put_words(c, font, &[0x5f, glyphs, 0, 0x20, 0x7e, 12]);
                c.bus.write32(font + 0x28, 0);
                c.bus.write32(font + 0x30, 1.0f32.to_bits());
                c.bus.write32(font + 0x34, 1.5f32.to_bits());
                for ch in 0..0x5f {
                    let g = glyphs + 12 * ch;
                    c.bus.write8(g + 2, 6);
                    c.bus.write8(g + 8, 8);
                    c.bus.write8(g + 9, 0);
                }
                let defs = DATA + 0x3000;
                let idx = r.next_u32() % 3;
                let d = defs + 0x24 * idx;
                c.bus.write8(d + 0x20, [0xffu8, 0, 1][(seed % 3) as usize]);
                c.bus
                    .write8(d + 0x21, [0u8, 1, 0xff][(seed / 3 % 3) as usize]);
                c.bus.write8(d + 0x22, r.next_u32() as u8);
                let gr = DATA + 0x6000;
                c.bus.write32(GAME_GP - 0x854, gr);
                c.bus.write32(gr + 0xe84, DATA + 0x6100);
                c.bus.write32(DATA + 0x6108, r.next_u32());
                let text = DATA + 0x4000;
                put_str_oc396(c, text, TEXTS[(seed / 9) as usize % TEXTS.len()]);
                let rect = DATA + 0x5000;
                fill(c, &mut r, rect, 4);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(font);
                c.g[A2][0] = sx(defs);
                c.g[A3][0] = idx as u64;
                c.g[T0][0] = if seed % 13 == 0 { 0 } else { sx(text) };
                c.g[T1][0] = (seed % 4) as u64;
                c.g[T2][0] = if seed % 5 == 0 { 0 } else { sx(rect) };
            },
        );
    }

    oc396_fake!(
        oc396_fake_apply_pose,
        OC396_PREV_APPLY_POSE,
        0x0031_0200,
        ee::s10::f_00310200__generated
    );
    oc396_fake!(
        oc396_fake_bone_matrices,
        OC396_PREV_BONE_MATRICES,
        0x0031_03f0,
        ee::s10::f_003103f0__generated
    );

    fn oc396_install_model_fakes() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00310200),
            oc396_fake_apply_pose,
            &OC396_PREV_APPLY_POSE,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_003103F0),
            oc396_fake_bone_matrices,
            &OC396_PREV_BONE_MATRICES,
        );
    }

    /// Both pose modes, the model calls faked.
    #[test]
    fn fe_module_rider_preview_apply_transform_matches() {
        oc396_install_model_fakes();
        covered(
            ee::s03::f_0019f2d0__generated,
            fe_module_rider_preview_apply_transform,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0019_f2d0, 2);
                c.pc = 0x0019_f2d0;
                c.bus.write32(OC396_CTRL, OC396_MAGIC);
                let this = DATA + 0x100;
                let model = DATA + 0x1000;
                c.bus.write32(this + RIDER_PREVIEW_MODEL, model);
                c.bus.write32(model + 0x24, DATA + 0x2000);
                c.bus.write32(model + 0x28, DATA + 0x3000);
                c.bus
                    .write32(this + RIDER_PREVIEW_SINGLE_POSE, (seed % 2) * r.next_u32());
                for b in RIDER_PREVIEW_BONES {
                    c.bus.write32(this + b, r.next_u32() % 16);
                }
                fill(c, &mut r, this + RIDER_PREVIEW_POSE, 16);
                fill(c, &mut r, 0x004f_f130, 4);
                fill(c, &mut r, 0x004f_f220, 4);
                c.g[A0][0] = sx(this);
            },
        );
    }

    oc396_fake!(
        oc396_fake_irr_clear,
        OC396_PREV_IRR_CLEAR,
        0x0038_9260,
        ee::s12::f_00389260__generated
    );
    oc396_fake!(
        oc396_fake_irr_entry,
        OC396_PREV_IRR_ENTRY,
        0x0038_ac20,
        ee::s12::f_0038ac20__generated
    );
    oc396_fake!(
        oc396_fake_irr_add,
        OC396_PREV_IRR_ADD,
        0x0038_9590,
        ee::s12::f_00389590__generated
    );
    oc396_fake!(
        oc396_fake_irr_cam,
        OC396_PREV_IRR_CAM,
        0x0038_9cb8,
        ee::s12::f_00389cb8__generated
    );
    oc396_fake!(
        oc396_fake_model_render,
        OC396_PREV_MODEL_RENDER,
        0x0031_0640,
        ee::s10::f_00310640__generated
    );
    oc396_fake!(
        oc396_fake_fx_render,
        OC396_PREV_FX_RENDER,
        0x002f_2088,
        ee::s09::f_002f2088__generated
    );

    /// The preview render: not ready (pending, no model, not posed) or every
    /// rider type with and without the outfit override; the root bone found
    /// or not (real lookup, `strcmp`); with and without a layer.
    #[test]
    fn fe_module_rider_preview_render_matches() {
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00389260),
            oc396_fake_irr_clear,
            &OC396_PREV_IRR_CLEAR,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038AC20),
            oc396_fake_irr_entry,
            &OC396_PREV_IRR_ENTRY,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00389590),
            oc396_fake_irr_add,
            &OC396_PREV_IRR_ADD,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00389CB8),
            oc396_fake_irr_cam,
            &OC396_PREV_IRR_CAM,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00310640),
            oc396_fake_model_render,
            &OC396_PREV_MODEL_RENDER,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002F2088),
            oc396_fake_fx_render,
            &OC396_PREV_FX_RENDER,
        );
        covered(
            ee::s03::f_0019ee88__generated,
            fe_module_rider_preview_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0019_ee88, 7);
                c.pc = 0x0019_ee88;
                c.bus.write32(OC396_CTRL, OC396_MAGIC);
                fill(c, &mut r, OC396_RET, 8);
                put_words(
                    c,
                    0x0046_0b40,
                    &[
                        0x19ef30, 0x19ef44, 0x19ef58, 0x19ef6c, 0x19ef80, 0x19ef94, 0x19efa8,
                        0x19efbc, 0x19efd0, 0x19efe4,
                    ],
                );
                let this = DATA + 0x100;
                let model = DATA + 0x1000;
                let ready = seed % 8 != 0;
                c.bus.write32(
                    this + RIDER_PREVIEW_PENDING,
                    if ready { u32::MAX } else { seed % 3 },
                );
                c.bus.write32(
                    this + RIDER_PREVIEW_MODEL,
                    if seed % 16 == 1 { 0 } else { model },
                );
                c.bus.write32(
                    this + RIDER_PREVIEW_DIRTY,
                    if seed % 16 == 2 { 0 } else { 1 },
                );
                c.bus
                    .write32(this + RIDER_PREVIEW_TYPE, (seed / 2 % 13).wrapping_sub(1));
                c.bus.write32(this + RIDER_PREVIEW_OUTFIT, seed % 2);
                c.bus.write32(
                    this + RIDER_PREVIEW_LAYER,
                    if r.next_u32() % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 32
                    },
                );
                c.bus.write32(this + RIDER_PREVIEW_FX, r.next_u32());
                // The model's skeleton: the root bone's name list.
                put_str_oc396(c, 0x004a_1978, b"root");
                c.bus.write32(model + 0x1c, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100, 0);
                c.bus.write32(model + 0xc, DATA + 0x1200);
                let skel = DATA + 0x1200;
                let n = r.next_u32() % 3;
                c.bus.write32(skel + 0x44, n);
                c.bus.write32(skel + 0x38, DATA + 0x1400);
                c.bus.write32(skel + 4, r.next_u32() % 4);
                for k in 0..2 {
                    let name: &[u8] = if r.next_u32() % 2 == 0 {
                        b"root"
                    } else {
                        b"spine"
                    };
                    put_str_oc396(c, DATA + 0x1400 + 0x50 * k, name);
                }
                c.bus.write32(model + 0x2c, DATA + 0x1800);
                fill(c, &mut r, DATA + 0x17e0, 0x30);
                // The graphics manager: vtable slot 0x228, draw state.
                let gr = DATA + 0x2000;
                c.bus.write32(GAME_GP - 0x854, gr);
                c.bus.write32(gr + 0x10d8, DATA + 0x2100);
                c.bus.write16(DATA + 0x2100 + 0x228, 0);
                c.bus
                    .write32(DATA + 0x2100 + 0x22c, crate::difftest::fakes::FAKE);
                c.bus.write32(gr + 0xe84, DATA + 0x2200);
                c.bus.write32(DATA + 0x2208, r.next_u32());
                c.g[A0][0] = sx(this);
            },
        );
    }

    oc396_fake!(
        oc396_fake_start_time,
        OC396_PREV_START_TIME,
        0x0012_c028,
        ee::s00::f_0012c028__generated
    );
    oc396_fake!(
        oc396_fake_can_start,
        OC396_PREV_CAN_START,
        0x0012_c078,
        ee::s00::f_0012c078__generated
    );

    /// The gate rock: stick forward / back / centred, the lean near or far
    /// from its target, unchanged or moving up or down; launching (quick or
    /// slow, forward or not) or not yet. The knee angle comes from the real
    /// `Rider_GetGateChargeAngle` over the faked BE queries (calls 0-6).
    #[test]
    fn rider_gate_sub_update_charge_matches() {
        oc396_install_be_fakes();
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0012C028),
            oc396_fake_start_time,
            &OC396_PREV_START_TIME,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0012C078),
            oc396_fake_can_start,
            &OC396_PREV_CAN_START,
        );
        covered(
            ee::s00::f_0012c230__generated,
            rider_gate_sub_update_charge,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0012_c230, 9);
                c.pc = 0x0012_c230;
                c.bus.write32(OC396_CTRL, OC396_MAGIC);
                // Calls: GetBE, GetInterface, (size scale: character, alt = 0,
                // GetBE, GetInterface, sharing), start time reached, can start (#8 -> 0).
                let ret = [
                    (seed / 4 % 2) as u32,
                    DATA + 0x700,
                    r.next_u32() % 8,
                    0,
                    DATA + 0x700,
                    DATA + 0x700,
                    r.next_u32() % 2,
                    (seed / 8 % 4 != 0) as u32,
                ];
                put_words(c, OC396_RET, &ret);
                for (k, v) in [
                    0.032258064f32,
                    0.62,
                    0.31,
                    0.2,
                    0.9,
                    0.016666668,
                    0.016666668,
                    0.4,
                    2027.7778,
                    555.55554,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(GAME_GP - 0x7834 + 4 * k as u32, v.to_bits());
                }
                c.bus.write32(GAME_GP - 0x6df4, 0.01f32.to_bits());
                c.bus.write32(GAME_GP - 0x6df0, 0.8f32.to_bits());
                let this = DATA + 0x100;
                let rider = DATA + 0x400;
                let stick: i32 = [20, -20, 0, 5][(seed % 4) as usize];
                c.bus.write32(
                    DATA,
                    ((stick as u32) & 0x3f) << 12 | (r.next_u32() & !(0x3f << 12)),
                );
                let lean = [0.31f32, 0.62, 0.0, 0.5, 0.1, 0.3][(r.next_u32() % 6) as usize];
                c.bus.write32(this + GATE_LEAN, lean.to_bits());
                c.bus.write32(
                    this + GATE_HELD,
                    [0.1f32, 0.5, 0.0][(r.next_u32() % 3) as usize].to_bits(),
                );
                c.bus.write32(this + GATE_LEAN_MIN, r.f32(0.3));
                c.bus.write32(this + GATE_LEAN_MAX, r.f32(0.7));
                c.bus.write32(this + GATE_RIDER, rider);
                c.bus.write32(rider + 0x86c, r.next_u32() % 4);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA);
            },
        );
    }

    oc396_fake!(
        oc396_fake_sincos,
        OC396_PREV_SINCOS,
        0x0031_be50,
        ee::s10::f_0031be50__generated
    );

    /// Shallow and steep rail normals, facing the rider's up or not; the
    /// sine/cosine faked (they come from the caller's frame).
    #[test]
    fn rider_railslide_state_compute_rail_frame_matches() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_0031BE50),
            oc396_fake_sincos,
            &OC396_PREV_SINCOS,
        );
        covered(
            ee::s01::f_0013bd80__generated,
            rider_railslide_state_compute_rail_frame,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0013_bd80, 1);
                c.pc = 0x0013_bd80;
                c.bus.write32(OC396_CTRL, OC396_MAGIC);
                let fb = crate::difftest::SP - 0xc0;
                put_words(c, fb + 0x70, &[r.f32(1.0), r.f32(1.0)]);
                c.bus.write32(GAME_GP - 0x7118, 0.92f32.to_bits());
                put_words(c, 0x004f_f160, &[0, 0, 0x3f80_0000, 0]);
                let rail = DATA + 0x100;
                let rider = DATA + 0x400;
                let n = DATA + 0x200;
                c.bus.write32(rail + 0x10, r.f32(3.0));
                c.bus.write32(rail + 0x50, rider);
                put_words(c, rider + 0x1c0, &[r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                let nz = if seed % 2 == 0 {
                    r.f32(0.9)
                } else {
                    (0.95f32 + (r.next_u32() % 5) as f32 * 0.01).to_bits() ^ ((seed / 2 % 2) << 31)
                };
                put_words(c, n, &[r.f32(0.5), r.f32(0.5), nz, 0]);
                c.g[A0][0] = sx(DATA + 0x600);
                c.g[A1][0] = sx(rail);
                c.g[A2][0] = sx(n);
            },
        );
    }

    oc396_fake!(
        oc396_fake_replay_playing,
        OC396_PREV_REPLAY_PLAYING,
        0x0028_8ae0,
        ee::s07::f_00288ae0__generated
    );
    oc396_fake!(
        oc396_fake_snd_lock,
        OC396_PREV_SND_LOCK,
        0x003b_58a0,
        ee::s13::f_003b58a0__generated
    );
    oc396_fake!(
        oc396_fake_snd_done,
        OC396_PREV_SND_DONE,
        0x003b_8ab8,
        ee::s13::f_003b8ab8__generated
    );
    oc396_fake!(
        oc396_fake_snd_unlock,
        OC396_PREV_SND_UNLOCK,
        0x003b_58d8,
        ee::s13::f_003b58d8__generated
    );

    /// The score tick: replay or not, display off, no wrap, or a wrap with
    /// the old tick still playing or done; the slot push on a track whose
    /// index starts at -1..1.
    #[test]
    fn ssx_audio_system_play_trick_points_tick_matches() {
        oc396_install_audio_fakes();
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00288AE0),
            oc396_fake_replay_playing,
            &OC396_PREV_REPLAY_PLAYING,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58A0),
            oc396_fake_snd_lock,
            &OC396_PREV_SND_LOCK,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B8AB8),
            oc396_fake_snd_done,
            &OC396_PREV_SND_DONE,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B58D8),
            oc396_fake_snd_unlock,
            &OC396_PREV_SND_UNLOCK,
        );
        covered(
            ee::s08::f_0029a7d8__generated,
            ssx_audio_system_play_trick_points_tick,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_a7d8, 6);
                c.pc = 0x0029_a7d8;
                c.bus.write32(OC396_CTRL, OC396_MAGIC);
                // Calls: replay?, lock, done?, unlock, [stop], play.
                let ret = [
                    (seed % 7 == 0) as u32,
                    0,
                    (seed / 2 % 2) as u32,
                    0,
                    r.next_u32(),
                    r.next_u32(),
                    0,
                    0,
                ];
                put_words(c, OC396_RET, &ret);
                let snd = DATA + 0x10_000;
                let track = DATA + 0x4000;
                let sub = track + 0x1d8;
                c.bus.write32(snd, track);
                c.bus.write32(snd + 4, DATA + 0x5000);
                c.bus.write32(snd + 8, DATA + 0x5100);
                c.bus.write32(snd + 0x118, snd);
                c.bus.write32(snd + AUDIO_POINTS_LAST, r.next_u32() % 100);
                c.bus.write32(snd + AUDIO_POINTS_HANDLE, r.next_u32());
                c.bus
                    .write32(track + 0x1f4, (r.next_u32() % 3).wrapping_sub(1));
                fill(c, &mut r, track + 0x1dc, 6);
                c.bus.write32(sub + 0x20, DATA + 0x6000);
                for (k, f) in [
                    0x34u32, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(sub + f, DATA + 0x7000 + 0x100 * k as u32);
                }
                let rider = DATA + 0x2000;
                c.bus.write32(rider + 0x874, (seed % 5 != 1) as u32);
                c.bus
                    .write32(rider + 0x87c, (seed % 5 != 2) as u32 * r.next_u32());
                c.g[A0][0] = sx(snd);
                c.g[A1][0] = sx(rider);
                c.g[A2][0] = sx(r.next_u32() % 1000);
            },
        );
    }

    /// A ramp track with up to three running ramps (some ended by `t`) and
    /// up to four keys (some due), with the real key activation.
    #[test]
    fn in_seq_ramp_track_update_matches() {
        check(
            ee::s07::f_00273698__generated,
            in_seq_ramp_track_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_3698, 0);
                c.pc = 0x0027_3698;
                let this = DATA + 0x100;
                let header = DATA + 0x200;
                let keys = DATA + 0x400;
                let pool = DATA + 0x1000;
                let flags = DATA + 0x80;
                c.bus.write32(flags, r.next_u32() & 0xfc);
                let t = (r.next_u32() % 20) as f32;
                c.f[12] = t.to_bits();
                c.f[13] = r.f32(2.0);
                // Running ramps.
                let n = r.next_u32() % 4;
                let mut head = 0;
                for k in (0..n).rev() {
                    let e = DATA + 0x1800 + 0x40 * k;
                    put_words(
                        c,
                        e,
                        &[
                            0,
                            0,
                            r.next_u32() % 30,
                            0,
                            r.f32(10.0),
                            r.f32(3.0),
                            0,
                            0,
                            head,
                        ],
                    );
                    head = e;
                }
                c.bus.write32(this + RAMP_TRACK_ACTIVE, head);
                c.bus.write32(this + RAMP_TRACK_COUNT, n);
                // Keys (size 0x18 or 0x20), sorted by time.
                let nk = r.next_u32() % 5;
                c.bus.write16(header, nk.max(1) as u16);
                let mut at = keys;
                let mut time = 0;
                for k in 0..nk {
                    let size = if r.next_u32() % 2 == 0 { 0x18 } else { 0x20 };
                    time += r.next_u32() % 8;
                    c.bus.write16(at, size as u16);
                    c.bus.write16(at + 2, k as u16);
                    c.bus.write16(at + 4, time as u16);
                    c.bus.write16(at + 6, r.next_u32() as u16);
                    put_words(
                        c,
                        at + 8,
                        &[
                            r.next_u32() % 40,
                            r.next_u32() % 100,
                            r.f32(3.0),
                            r.next_u32(),
                        ],
                    );
                    at += size;
                }
                c.bus.write32(this + 0x10, header);
                c.bus
                    .write32(this + RAMP_TRACK_KEY, if nk == 0 { 0 } else { keys });
                c.bus.write32(this + 0x18, 0);
                c.bus.write32(this + 0x1c, pool);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(flags);
            },
        );
    }

    /// The surface loop level: slow or not, eased up, down or not at all;
    /// the listener slot from a faked vtable call, the real curve lookup.
    #[test]
    fn ssx_audio_system_calc_surface_loop_group2_matches() {
        covered(
            ee::s07::f_00293948__generated,
            ssx_audio_system_calc_surface_loop_group2,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_3948, 1);
                c.pc = 0x0029_3948;
                let slot = 1 + r.next_u32() % 3;
                c.bus.write32(crate::difftest::fakes::LOG_RESULTS, slot);
                c.bus.write32(GAME_GP - 0x45bc, 0.1f32.to_bits());
                let curves: [f32; 20] = [
                    0.0185, 0.1, 0.5, 0.6, 0.95, 2.0, 15.0, 30.0, 50.0, 80.0, 0.05, 0.2, 0.5, 0.8,
                    0.95, 1.0, 0.9, 0.75, 0.65, 0.5,
                ];
                for (k, v) in curves.into_iter().enumerate() {
                    c.bus.write32(0x0048_2c18 + 4 * k as u32, v.to_bits());
                }
                let snd = DATA + 0x100;
                let rider = DATA + 0x8000;
                let lv = |r: &mut Rng| ((r.next_u32() % 100) as f32 * 0.01).to_bits();
                c.bus.write32(rider + 0x750, lv(&mut r));
                c.bus.write32(
                    rider + 0x754,
                    (((r.next_u32() % 50) as f32) * 0.01).to_bits(),
                );
                c.bus.write32(rider + 0x758, lv(&mut r));
                c.bus.write32(
                    rider + 0x75c,
                    if seed % 3 == 0 {
                        0.05f32.to_bits()
                    } else {
                        lv(&mut r)
                    },
                );
                let vt = DATA + 0x3000;
                c.bus.write16(vt + 0x38, 0);
                c.bus.write32(vt + 0x3c, crate::difftest::fakes::FAKE);
                c.bus.write32(rider + 0x6c0, vt);
                for k in 0..4 {
                    c.bus.write32(snd + 0x59b8 + 4 * k, lv(&mut r));
                }
                c.g[A0][0] = sx(snd);
                c.g[A1][0] = sx(rider);
                c.g[A2][0] = sx(DATA + 0x40);
                c.g[A3][0] = sx(DATA + 0x44);
            },
        );
    }
}
