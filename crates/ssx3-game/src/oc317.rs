//! Staging module oc317 (wave wave213): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc317`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003d_95b8,
        spch_resolve_event_parts,
        "SPCH_ResolveEventParts",
    ),
    (
        0x0011_7ae8,
        rider_metrix_show_hud_text,
        "RiderMetrix_ShowHUDText",
    ),
    (0x0013_99e0, rider_air_state_enter, "RiderAirState_Enter"),
    (
        0x003c_8b80,
        snd_ee_voice_set_distortion,
        "SND_EeVoiceSetDistortion",
    ),
    (
        0x003c_3250,
        snd_channel_apply_highpass,
        "SND_ChannelApplyHighpass",
    ),
    (
        0x003c_30c8,
        snd_channel_apply_distortion,
        "SND_ChannelApplyDistortion",
    ),
    (0x0035_0570, world_obj_get_bound_box, "WorldObj_GetBoundBox"),
    (
        0x0034_5c90,
        dynamic_particle_modifier_ctor,
        "DynamicParticleModifier_ctor",
    ),
    (0x003d_7110, spch_find_event, "SPCH_FindEvent"),
];

// ---------------------------------------------------------------------------
// SPCH_ResolveEventParts (0x3d95b8).

/// A speech layer: 4 selection entries per `cnt` unit at +4, the
/// "sentence in progress" flag at +5, the entry index list at +8.
pub(crate) const SPCH_RESOLVE_LAYER_CNT: u32 = 4;
pub(crate) const SPCH_RESOLVE_LAYER_N5: u32 = 5;
pub(crate) const SPCH_RESOLVE_LAYER_LIST: u32 = 8;

/// A selection entry (8 bytes): `i16 x` (bank index), `u8 start`, `u8 len`,
/// `u8 cur`, `u8 flag`.
pub(crate) const SPCH_RESOLVE_ENT_SIZE: u32 = 8;
pub(crate) const SPCH_RESOLVE_ENT_START: u32 = 4;
pub(crate) const SPCH_RESOLVE_ENT_LEN: u32 = 5;
pub(crate) const SPCH_RESOLVE_ENT_CUR: u32 = 6;

/// The variation table inside the selection set (right after 12 entries).
pub(crate) const SPCH_RESOLVE_TBL: u32 = 0x62;

/// A variation list: `s8 n` at +4, 8 ids at +8, 8 output columns at +0x10.
pub(crate) const SPCH_RESOLVE_LIST_N: u32 = 4;
pub(crate) const SPCH_RESOLVE_LIST_IDS: u32 = 8;
pub(crate) const SPCH_RESOLVE_LIST_COLS: u32 = 0x10;

/// The decoded sample record: the per-variation column bytes at +0xc.
pub(crate) const SPCH_RESOLVE_SAMPLE_COLS: u32 = 0xc;

/// The variation id that picks a decoded column.
pub(crate) const SPCH_RESOLVE_WILD: u64 = 0xfe;

/// gp offset of the speech bank pointer table (indexed by entry `x`).
pub(crate) const SPCH_RESOLVE_GP_BANKS: u32 = 0x1734;

/// `SPCH_RandomNoRepeat(int range, int key)`.
pub(crate) const SPCH_RESOLVE_RANDOM_NO_REPEAT: Callee = Callee {
    addr: 0x003d_b5c0,
    f: ee::s14::f_003db5c0,
};

/// `SPCH_AssignVariations(Layer *, SelectionSet *, int threshold)`.
pub(crate) const SPCH_RESOLVE_ASSIGN_VARIATIONS: Callee = Callee {
    addr: 0x003d_93c0,
    f: ee::s14::f_003d93c0,
};

/// `SPCH_FindBankDataPtr(u8 *id, void **data)`.
pub(crate) const SPCH_RESOLVE_FIND_BANK_DATA: Callee = Callee {
    addr: 0x003d_83b8,
    f: ee::s14::f_003d83b8,
};

/// `SPCH_DecodeSampleEntry(bank, index, out)`.
pub(crate) const SPCH_RESOLVE_DECODE_ENTRY: Callee = Callee {
    addr: 0x003d_b838,
    f: ee::s14::f_003db838,
};

/// `int SPCH_ResolveEventParts(u8* request, SelectionSet* h, Layer* l,
/// int mode, int threshold)` (0x3d95b8): pick each layer entry's variation
/// (shuffled and assigned through `SPCH_AssignVariations` when `mode == 1`
/// or the layer says a sentence is in progress, otherwise `start +
/// SPCH_RandomNoRepeat(len, x)`), then write every requested variation's
/// decoded column byte into the bank output buffer.
pub fn spch_resolve_event_parts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xe0);
        c.g[V0][0] = 1;
        frame.save(c, FP, 0xc0);
        frame.save(c, S7, 0xb0);
        c.g[FP][0] = c.g[T0][0]; // threshold
        frame.save(c, S6, 0xa0);
        c.g[S7][0] = c.g[A2][0]; // layer
        frame.save(c, RA, 0xd0);
        c.g[S6][0] = c.g[A1][0]; // selection set
        frame.save(c, S5, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x40);
        let mode_shuffles = c.g[A3][0] == c.g[V0][0];
        Addr(frame.at(c, 0x14)).sw(c, c.g[A0][0]); // the request
        let shuffles = if mode_shuffles {
            true
        } else {
            c.cycles += 3;
            let n5 = Addr(r32(c, S7).wrapping_add(SPCH_RESOLVE_LAYER_N5)).lbu(c);
            c.g[V0][0] = n5;
            if n5 == 0 {
                c.g[V0][0] = Addr(r32(c, S7).wrapping_add(SPCH_RESOLVE_LAYER_CNT)).lbu(c);
                false
            } else {
                true
            }
        };
        let mut failed = false;
        if shuffles {
            // 0x3d9608: shuffle each entry's variation table slice, then
            // assign the variations.
            c.cycles += 5;
            let cnt = Addr(r32(c, S7).wrapping_add(SPCH_RESOLVE_LAYER_CNT)).lbu(c);
            c.g[V0][0] = cnt;
            c.g[S5][0] = c.g[A3][0] ^ 2; // mode != 2: pass the threshold
            c.g[S4][0] = sx((c.g[V0][0] as u32) >> 2);
            c.g[V1][0] = 0;
            if c.g[S4][0] != 0 {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
                loop {
                    c.cycles += 8;
                    c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(SPCH_RESOLVE_ENT_START)).lbu(c);
                    c.g[S0][0] = Addr(r32(c, V0).wrapping_add(SPCH_RESOLVE_ENT_LEN)).lbu(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(SPCH_RESOLVE_TBL));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 2) as u64;
                    c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if c.g[V0][0] == 0 {
                        // len >= 2: Fisher-Yates over the entry's slice.
                        c.cycles += 2;
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        loop {
                            c.cycles += 12;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = u64::MAX;
                            SPCH_RESOLVE_RANDOM_NO_REPEAT.call(c, 0x003d_9654)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A0][0] = Addr(r32(c, S1).wrapping_sub(1)).lbu(c);
                            c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                            c.g[A1][0] = ((c.g[S0][0] as i64) < 2) as u64;
                            Addr(r32(c, S1).wrapping_sub(1)).sb(c, c.g[V1][0]);
                            Addr(r32(c, V0)).sb(c, c.g[A0][0]);
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                            if c.g[A1][0] != 0 {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x003d_9648);
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.g[S3][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_9620);
                    }
                }
            }
            c.cycles += 7;
            c.g[A2][0] = 0;
            c.g[A0][0] = c.g[S7][0];
            if c.g[S5][0] != 0 {
                c.g[A2][0] = c.g[FP][0];
            }
            c.g[A1][0] = c.g[S6][0];
            SPCH_RESOLVE_ASSIGN_VARIATIONS.call(c, 0x003d_969c)?;
            Addr(frame.at(c, 0x18)).sw(c, c.g[V0][0]);
        } else {
            // 0x3d96ac: cur = start + SPCH_RandomNoRepeat(len, x) per entry.
            c.cycles += 5;
            c.g[V1][0] = 1;
            Addr(frame.at(c, 0x18)).sw(c, c.g[V1][0]);
            c.g[S2][0] = sx((c.g[V0][0] as u32) >> 2);
            c.g[S1][0] = 0;
            if c.g[S2][0] != 0 {
                c.cycles += 2;
                c.g[S0][0] = c.g[S6][0];
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(SPCH_RESOLVE_ENT_LEN)).lbu(c);
                    c.g[A0][0] = c.g[V0][0];
                    if c.g[V0][0] == 0 {
                        // An empty entry: nothing to play at all (0x3d96a4,
                        // charged below).
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_96a4);
                        }
                        failed = true;
                        break;
                    }
                    c.cycles += 9;
                    c.g[A1][0] = Addr(r32(c, S0)).lh(c);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    SPCH_RESOLVE_RANDOM_NO_REPEAT.call(c, 0x003d_96e0)?;
                    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(SPCH_RESOLVE_ENT_START)).lbu(c);
                    c.g[A0][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    Addr(r32(c, S0).wrapping_add(SPCH_RESOLVE_ENT_CUR)).sb(c, c.g[V1][0]);
                    let more = c.g[A0][0] != 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(SPCH_RESOLVE_ENT_SIZE));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_96c8);
                    }
                }
            }
        }
        // 0x3d96f8: write the chosen columns into the bank's output buffer.
        let mut tail_charge = true;
        if !failed {
            c.cycles += 5;
            c.g[A0][0] = Addr(frame.at(c, 0x14)).lw(c);
            c.g[A1][0] = sx(frame.at(c, 0x10));
            SPCH_RESOLVE_FIND_BANK_DATA.call(c, 0x003d_9704)?;
            let found = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(frame.at(c, 0x18)).lw(c);
            tail_charge = found;
            if found {
                // 0x3d970c: per entry of the layer's list.
                c.cycles += 5;
                c.g[V0][0] = Addr(r32(c, S7).wrapping_add(SPCH_RESOLVE_LAYER_CNT)).lbu(c);
                c.g[A0][0] = 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
                Addr(frame.at(c, 0x1c)).sw(c, c.g[V0][0]);
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(SPCH_RESOLVE_LAYER_LIST));
                    Addr(frame.at(c, 0x20)).sw(c, c.g[V0][0]);
                    c.g[V1][0] = Addr(frame.at(c, 0x20)).lw(c);
                    loop {
                        c.cycles += 8;
                        c.g[S1][0] = 0;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[S3][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(SPCH_RESOLVE_LIST_N)).lb(c);
                        let empty = (c.g[V0][0] as i64) <= 0;
                        c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                        if !empty {
                            c.cycles += 6;
                            c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
                            c.g[A3][0] = SPCH_RESOLVE_WILD;
                            c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[S5][0] = sx((c.g[S6][0] as u32).wrapping_add(SPCH_RESOLVE_TBL));
                            c.g[S4][0] =
                                sx((c.g[SP][0] as u32).wrapping_add(SPCH_RESOLVE_SAMPLE_COLS));
                            c.g[S0][0] =
                                sx((c.g[S3][0] as u32).wrapping_add(SPCH_RESOLVE_LIST_COLS));
                            loop {
                                c.cycles += 3;
                                c.g[V0][0] = Addr(
                                    r32(c, S3)
                                        .wrapping_add(SPCH_RESOLVE_LIST_IDS)
                                        .wrapping_add(r32(c, S1)),
                                )
                                .lbu(c);
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] =
                                        Addr(r32(c, S3).wrapping_add(SPCH_RESOLVE_LIST_N)).lb(c);
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
                                    c.g[V0][0] &= 0x80;
                                    if c.g[V0][0] != 0 {
                                        c.g[V0][0] =
                                            Addr(r32(c, S3).wrapping_add(SPCH_RESOLVE_LIST_N))
                                                .lb(c);
                                    } else {
                                        // Decode the chosen sample entry and
                                        // take its column byte.
                                        c.cycles += 13;
                                        c.g[V0][0] = Addr(r32(c, S2)).lh(c);
                                        c.g[A2][0] = c.g[SP][0];
                                        c.g[A0][0] =
                                            Addr(r32(c, GP).wrapping_add(SPCH_RESOLVE_GP_BANKS))
                                                .lw(c);
                                        c.g[V1][0] =
                                            Addr(r32(c, S2).wrapping_add(SPCH_RESOLVE_ENT_CUR))
                                                .lbu(c);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        c.g[V1][0] =
                                            sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                                        c.g[A1][0] = Addr(r32(c, V1)).lbu(c);
                                        frame.save(c, A3, 0x30);
                                        SPCH_RESOLVE_DECODE_ENTRY.call(c, 0x003d_97b0)?;
                                        let decoded = c.g[V0][0] != 0;
                                        frame.restore(c, A3, 0x30);
                                        if decoded {
                                            c.cycles += 6;
                                            c.g[A1][0] = Addr(r32(c, S0)).lbu(c);
                                            c.g[V1][0] =
                                                sx((c.g[S4][0] as u32)
                                                    .wrapping_add(c.g[S1][0] as u32));
                                            c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
                                            c.g[A0][0] = Addr(r32(c, V1)).lbu(c);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A1][0] as u32));
                                            Addr(r32(c, V0)).sb(c, c.g[A0][0]);
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] =
                                            Addr(r32(c, S3).wrapping_add(SPCH_RESOLVE_LIST_N))
                                                .lb(c);
                                    }
                                }
                                c.cycles += 4;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let more = c.g[V0][0] != 0;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                                if !more {
                                    break;
                                }
                                if event_due(c) {
                                    deliver_event_here(c, 0x003d_9768);
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[V1][0] = Addr(frame.at(c, 0x1c)).lw(c);
                        c.g[A0][0] = c.g[FP][0];
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V1][0] = Addr(frame.at(c, 0x20)).lw(c);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_9730);
                        }
                    }
                }
            }
        } else {
            // 0x3d96a4: an empty entry leaves ok = 0.
            c.cycles += 2;
            Addr(frame.at(c, 0x18)).sw(c, 0);
        }
        if tail_charge {
            c.cycles += 1;
            c.g[V0][0] = Addr(frame.at(c, 0x18)).lw(c);
        }
        c.cycles += 12;
        frame.restore(c, RA, 0xd0);
        frame.restore(c, FP, 0xc0);
        frame.restore(c, S7, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderMetrix_ShowHUDText (0x117ae8).

/// `RiderMetrix` field: the HUD message slot table (0: no messages).
pub(crate) const RIDER_METRIX_HUD_TABLE: u32 = 0x1b0;

/// A HUD message slot is 0x9c bytes.
pub(crate) const RIDER_METRIX_HUD_SLOT_SIZE: u32 = 0x9c;

/// `RiderMetrix_FindHUDSlot(RiderMetrix*, int type)`.
pub(crate) const RIDER_METRIX_FIND_HUD_SLOT: Callee = Callee {
    addr: 0x0011_7a58,
    f: ee::s00::f_00117a58,
};

/// `cHUDFX_Show(fx, int type, const char *text, int style, float time)`.
pub(crate) const C_HUDFX_SHOW: Callee = Callee {
    addr: 0x0011_7048,
    f: ee::s00::f_00117048,
};

/// `int RiderMetrix_ShowHUDText(RiderMetrix* self, int type, const char*
/// text, int style, float time)` (0x117ae8): show a HUD message in the slot
/// `RiderMetrix_FindHUDSlot` picks, and return that slot index (-1 when the
/// rider has no message table).
pub fn rider_metrix_show_hud_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x60);
        c.g[S4][0] = c.g[A3][0];
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(RIDER_METRIX_HUD_TABLE)).lw(c);
        let has_table = c.g[V0][0] != 0;
        c.f[20] = c.f[12]; // the decay time
        if !has_table {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 14;
            RIDER_METRIX_FIND_HUD_SLOT.call(c, 0x0011_7b2c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = RIDER_METRIX_HUD_SLOT_SIZE as u64;
            c.mult(0, A0, S0, A0); // slot index * slot size
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(RIDER_METRIX_HUD_TABLE)).lw(c);
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            c.f[12] = c.f[20];
            c.g[A3][0] = c.g[S4][0];
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            C_HUDFX_SHOW.call(c, 0x0011_7b54)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAirState_Enter (0x1399e0).

/// `AirPredictor_Prime` (cAirPredictor's start-launch-into-air, another
/// shard: through the table).
pub(crate) const RIDER_AIR_START_LAUNCH: u32 = 0x0011_35b8;

/// gp offset of the launch speed the air predictor is primed with.
pub(crate) const RIDER_AIR_LAUNCH_SPEED: u32 = 0x71d8;

/// `RiderAirState` fields: the "in the air" flag at +0, the rider at +4.
pub(crate) const RIDER_AIR_ACTIVE: u32 = 0;
pub(crate) const RIDER_AIR_RIDER: u32 = 4;

/// Rider fields: the air predictor (+0x788), the launch state (+0x110) and
/// the position (+0x1e0).
pub(crate) const RIDER_AIR_PREDICTOR: u32 = 0x788;
pub(crate) const RIDER_AIR_LAUNCH_STATE: u32 = 0x110;
pub(crate) const RIDER_AIR_POSITION: u32 = 0x1e0;

/// `void RiderAirState_Enter(RiderAirState* self, eMotionState prev)`
/// (0x1399e0): the state is in the air when the previous state was air (5),
/// then prime the rider's air predictor from its launch state and position.
pub fn rider_air_state_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A1][0] ^ 5; // prev == 5
        frame.save_ra(c, 0);
        c.g[A1][0] = (c.g[A1][0] < 1) as u64;
        Addr(r32(c, A0).wrapping_add(RIDER_AIR_ACTIVE)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(RIDER_AIR_RIDER)).lw(c);
        c.lwc1(12, r32(c, GP).wrapping_sub(RIDER_AIR_LAUNCH_SPEED));
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(RIDER_AIR_PREDICTOR)).lw(c);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(RIDER_AIR_POSITION));
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(RIDER_AIR_LAUNCH_STATE));
        call_ptr(c, RIDER_AIR_START_LAUNCH, 0x0013_9a0c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_EeVoiceSetDistortion (0x3c8b80).

/// The EE voice table pointer; a voice entry is 0x60 bytes: its DSP chain
/// head at +0x40 and its distortion DSP node at +0x54.
pub(crate) const SND_DIST_VOICES: u32 = 0x0051_5d1c;

pub(crate) const SND_DIST_VOICE_SIZE: u32 = 0x60;

pub(crate) const SND_DIST_VOICE_CHAIN: u32 = 0x40;

pub(crate) const SND_DIST_VOICE_DSP: u32 = 0x54;

/// `SND_DspDistortion_Init(node)`.
pub(crate) const SND_DIST_INIT: Callee = Callee {
    addr: 0x003c_b528,
    f: ee::s14::f_003cb528,
};

/// `SND_DspDistortion_SetAmount(node, int amount)`.
pub(crate) const SND_DIST_SET_AMOUNT: Callee = Callee {
    addr: 0x003c_b4a8,
    f: ee::s14::f_003cb4a8,
};

/// `SND_DspInsertSorted(chain_head, node)`.
pub(crate) const SND_DIST_INSERT_SORTED: Callee = Callee {
    addr: 0x003c_8f48,
    f: ee::s14::SND_DspInsertSorted,
};

/// `SND_DspRemove(chain_head)`.
pub(crate) const SND_DIST_DSP_REMOVE: Callee = Callee {
    addr: 0x003c_8fb0,
    f: ee::s14::SND_DspRemove,
};

/// `void SND_EeVoiceSetDistortion(int voice, int amount)` (0x3c8b80): an
/// amount above 0 gives the voice a distortion DSP (created on first use:
/// `SND_MemAlloc(0x50)`, priority 0x78, `SND_DspDistortion_Init`, inserted
/// into the voice's DSP chain) and sets its amount; an amount of 0 or less
/// removes and frees that DSP, if any. Frame 0x30 (s0 = voice entry, s1 =
/// amount); the entry offset uses `mult`.
pub fn snd_ee_voice_set_distortion(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[V0][0] = SND_DIST_VOICE_SIZE as u64;
        c.g[V1][0] = 0x51_0000;
        c.mult(0, A0, A0, V0);
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = Addr(SND_DIST_VOICES).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        if (c.g[S1][0] as i64) <= 0 {
            // Amount 0: drop the voice's distortion DSP.
            c.cycles += 3;
            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(SND_DIST_VOICE_DSP)).lw(c);
            if c.g[A1][0] != 0 {
                c.cycles += 6;
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(SND_DIST_VOICE_CHAIN));
                SND_DIST_DSP_REMOVE.call(c, 0x003c_8c10)?;
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SND_DIST_VOICE_DSP)).lw(c);
                call_ptr(c, 0x003b_5e98, 0x003c_8c18)?; // SND_MemFree
                Addr(r32(c, S0).wrapping_add(SND_DIST_VOICE_DSP)).sw(c, 0);
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(SND_DIST_VOICE_DSP)).lw(c);
            if c.g[V0][0] == 0 {
                // Create the distortion DSP and insert it into the chain.
                c.cycles += 13;
                c.g[A0][0] = 0x50;
                call_ptr(c, 0x003b_5c60, 0x003c_8bc0)?; // SND_MemAlloc
                let dsp = c.g[V0][0] as u32;
                let slot = Addr(r32(c, S0).wrapping_add(SND_DIST_VOICE_DSP));
                slot.sw(c, dsp as u64);
                c.g[V1][0] = 0x78; // priority
                Addr(dsp.wrapping_add(4)).sw(c, 0);
                c.g[V0][0] = slot.lw(c);
                Addr(r32(c, V0).wrapping_add(0x18)).sh(c, c.g[V1][0]);
                c.g[A0][0] = slot.lw(c);
                SND_DIST_INIT.call(c, 0x003c_8bdc)?;
                c.g[A1][0] = slot.lw(c);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(SND_DIST_VOICE_CHAIN));
                SND_DIST_INSERT_SORTED.call(c, 0x003c_8be8)?;
                c.g[A0][0] = slot.lw(c);
            } else {
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SND_DIST_VOICE_DSP)).lw(c);
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            SND_DIST_SET_AMOUNT.call(c, 0x003c_8bf4)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_ChannelApplyDistortion (0x3c30c8) and SND_ChannelApplyHighpass
// (0x3c3250).

/// The SND driver state (0x50a8e8): the channel table at +0x1f0 and the two
/// voice index bases at +0x2f and +0x30.
pub(crate) const SND_APPLY_STATE: u32 = 0x0050_a8e8;

pub(crate) const SND_APPLY_CHANNELS: u32 = 0x1f0;

pub(crate) const SND_APPLY_VOICE_BASE_A: u32 = 0x2f;

pub(crate) const SND_APPLY_VOICE_BASE_B: u32 = 0x30;

/// A channel is 0x8c bytes: its voices (+4, s16 each), the voice count
/// (+0x23) and the flags (+0x24; bit 2 = filtered).
pub(crate) const SND_APPLY_CHANNEL_SIZE: u32 = 0x8c;

pub(crate) const SND_APPLY_VOICES: u32 = 0x4;

pub(crate) const SND_APPLY_VOICE_COUNT: u32 = 0x23;

pub(crate) const SND_APPLY_FLAGS: u32 = 0x24;

/// `SND_EeVoiceSetDistortion` (0x3c8b80) through the table.
pub(crate) const SND_APPLY_SET_DISTORTION: u32 = 0x003c_8b80;

/// `SND_EeVoiceSetHighpass` (0x3c8c30) through the table.
pub(crate) const SND_APPLY_SET_HIGHPASS: u32 = 0x003c_8c30;

/// `void SND_ChannelApplyDistortion(int channel, int amount)` (0x3c30c8):
/// for a filtered channel (flags bit 2), set every voice's distortion
/// amount (`SND_EeVoiceSetDistortion(voice - bases, amount)`). Frame 0x60
/// (s0 = voice list, s1 = voices done, s2 = channel, s3 = the driver
/// state); the channel offset uses `mult`.
pub fn snd_channel_apply_distortion(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = SND_APPLY_CHANNEL_SIZE as u64;
        frame.save(c, S2, 0x30);
        c.mult(0, A0, A0, V0);
        frame.save(c, S4, 0x10);
        c.g[V1][0] = SND_APPLY_STATE as u64;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(SND_APPLY_STATE + SND_APPLY_CHANNELS).lw(c);
        c.g[S2][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SND_APPLY_FLAGS)).lhu(c);
        c.g[V0][0] &= 4;
        let filtered = c.g[V0][0] != 0;
        c.g[S4][0] = c.g[A1][0];
        if filtered {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SND_APPLY_VOICE_COUNT)).lbu(c);
            c.g[S1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[S3][0] = c.g[V1][0];
                c.g[S0][0] = sx(r32(c, S2).wrapping_add(SND_APPLY_VOICES));
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(SND_APPLY_VOICE_BASE_A)).lbu(c);
                loop {
                    c.cycles += 12;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = Addr(r32(c, S3).wrapping_add(SND_APPLY_VOICE_BASE_B)).lbu(c);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[V1][0] = Addr(r32(c, S0)).lh(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    call_ptr(c, SND_APPLY_SET_DISTORTION, 0x003c_3148)?;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SND_APPLY_VOICE_COUNT)).lbu(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(SND_APPLY_VOICE_BASE_A)).lbu(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_3128);
                    }
                }
            }
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

/// `void SND_ChannelApplyHighpass(int channel, int frequency)` (0x3c3250):
/// for a filtered channel (flags bit 2), set every voice's highpass cutoff
/// (`SND_EeVoiceSetHighpass(voice - bases, frequency)`). Same shape as
/// [`snd_channel_apply_distortion`] with the highpass setter.
pub fn snd_channel_apply_highpass(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = SND_APPLY_CHANNEL_SIZE as u64;
        frame.save(c, S2, 0x30);
        c.mult(0, A0, A0, V0);
        frame.save(c, S4, 0x10);
        c.g[V1][0] = SND_APPLY_STATE as u64;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(SND_APPLY_STATE + SND_APPLY_CHANNELS).lw(c);
        c.g[S2][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SND_APPLY_FLAGS)).lhu(c);
        c.g[V0][0] &= 4;
        let filtered = c.g[V0][0] != 0;
        c.g[S4][0] = c.g[A1][0];
        if filtered {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SND_APPLY_VOICE_COUNT)).lbu(c);
            c.g[S1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[S3][0] = c.g[V1][0];
                c.g[S0][0] = sx(r32(c, S2).wrapping_add(SND_APPLY_VOICES));
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(SND_APPLY_VOICE_BASE_A)).lbu(c);
                loop {
                    c.cycles += 12;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = Addr(r32(c, S3).wrapping_add(SND_APPLY_VOICE_BASE_B)).lbu(c);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[V1][0] = Addr(r32(c, S0)).lh(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    call_ptr(c, SND_APPLY_SET_HIGHPASS, 0x003c_32d0)?;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SND_APPLY_VOICE_COUNT)).lbu(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(SND_APPLY_VOICE_BASE_A)).lbu(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_32b0);
                    }
                }
            }
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
// WorldObj_GetBoundBox (0x350570).

/// `cInstanceNode` fields: the node's matrix data (+0x18) with the centre
/// (world translation) at +0x40, the first box corner at +0x60..+0x68 and
/// the second at +0x6c..+0x74.
pub(crate) const WORLDOBJ_NODE: u32 = 0x18;
pub(crate) const WORLDOBJ_CENTRE: u32 = 0x40;
pub(crate) const WORLDOBJ_CORNER_0: u32 = 0x60;
pub(crate) const WORLDOBJ_CORNER_1: u32 = 0x6c;

/// `void cInstanceNode_getBoundBoxInfo(cInstanceNode* self, sIbVec4 box[2],
/// float* radius)` (0x350570): fill the two box corners (w = 1) and return
/// the largest distance from the node's centre to a corner of the box they
/// span. Frame 0x40 (the probe vector at +0x10, the centre at +0).
pub fn world_obj_get_bound_box(c: &mut Cpu) {
    c.cycles += 32;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(WORLDOBJ_NODE)).lw(c);
    let frame = Frame::open(c, 0x40);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 2); // f2 = 1.0 (w)
    c.g[T0][0] = sx(r32(c, A1).wrapping_add(0x10)); // mx = box[1]
    c.lwc1(0, r32(c, V0).wrapping_add(WORLDOBJ_CORNER_0));
    c.g[T1][0] = c.g[SP][0]; // &centre
    c.mtc1(0, 3); // f3 = best = 0.0 (through the zero register)
    c.mov_s(4, 2); // f4 = 1.0
    c.swc1(0, frame.at(c, 0));
    c.g[A3][0] = 0;
    c.lwc1(0, r32(c, V0).wrapping_add(WORLDOBJ_CORNER_0 + 4));
    c.swc1(0, frame.at(c, 4));
    c.lwc1(1, r32(c, V0).wrapping_add(WORLDOBJ_CORNER_0 + 8));
    c.swc1(2, frame.at(c, 0xc));
    c.swc1(1, frame.at(c, 8));
    c.lq(2, frame.at(c, 0));
    c.sq(2, r32(c, A1));
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(WORLDOBJ_NODE)).lw(c);
    c.lwc1(0, r32(c, V1).wrapping_add(WORLDOBJ_CORNER_1));
    c.swc1(0, frame.at(c, 0));
    c.lwc1(1, r32(c, V1).wrapping_add(WORLDOBJ_CORNER_1 + 4));
    c.swc1(1, frame.at(c, 4));
    c.lwc1(0, r32(c, V1).wrapping_add(WORLDOBJ_CORNER_1 + 8));
    c.swc1(0, frame.at(c, 8));
    c.lq(2, frame.at(c, 0));
    c.sq(2, r32(c, A1).wrapping_add(0x10));
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(WORLDOBJ_NODE)).lw(c);
    c.lq(2, r32(c, V1).wrapping_add(WORLDOBJ_CENTRE));
    c.sq(2, frame.at(c, 0));
    c.g[V0][0] = c.g[A3][0] & 1;
    loop {
        // x/y/z of corner (A3 & 1/2/4 ? box[0] : box[1]).
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lwc1(2, r32(c, T0));
        } else {
            c.cycles += 1;
            c.lwc1(2, r32(c, A1));
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A3][0] & 2;
        if c.g[V0][0] == 0 {
            c.lwc1(1, r32(c, T0).wrapping_add(4));
        } else {
            c.cycles += 1;
            c.lwc1(1, r32(c, A1).wrapping_add(4));
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A3][0] & 4;
        if c.g[V0][0] == 0 {
            c.lwc1(0, r32(c, T0).wrapping_add(8));
        } else {
            c.cycles += 1;
            c.lwc1(0, r32(c, A1).wrapping_add(8));
        }
        c.cycles += 23;
        c.swc1(2, frame.at(c, 0x10));
        c.swc1(1, frame.at(c, 0x14));
        c.swc1(0, frame.at(c, 0x18));
        c.swc1(4, frame.at(c, 0x1c));
        c.lqc2(3, frame.at(c, 0x10));
        c.lqc2(4, c.g[T1][0] as u32);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4)); // c - centre
        c.sqc2(5, frame.at(c, 0x30));
        c.lq(2, frame.at(c, 0x30));
        c.sq(2, frame.at(c, 0x20));
        c.lqc2(3, frame.at(c, 0x20));
        c.lqc2(5, frame.at(c, 0x20));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W)); // dot(c - centre)
        c.qmfc2(3, 4);
        c.mtc1(3, 0);
        c.c_lt_s(3, 0);
        if c.fcc() {
            c.mov_s(3, 0); // best = l
        }
        c.cycles += 4;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[A3][0] as i64) < 8) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.sqrt_s(0, 3);
            c.swc1(0, r32(c, A2));
            frame.close(c);
            return;
        }
        c.g[V0][0] = c.g[A3][0] & 1;
        if event_due(c) {
            deliver_event_here(c, 0x0035_05f0);
        }
    }
}

// ---------------------------------------------------------------------------
// DynamicParticleModifier_ctor (0x345c90).

/// `ParticleModifier_BaseCtor(ParticleModifier*, void *o)`.
pub(crate) const DPM_BASE_CTOR: Callee = Callee {
    addr: 0x0034_5798,
    f: ee::s11::ParticleModifier_BaseCtor,
};

/// `DynamicEmitter_ctor(Emitter*)` (another shard: through the table).
pub(crate) const DPM_EMITTER_CTOR: u32 = 0x0037_0b60;

/// `DynamicEmitter_Setup(Emitter*, EmitterDef*, float time)`.
pub(crate) const DPM_EMITTER_SETUP: u32 = 0x0037_0dc8;

/// `DynamicParticleModifier_TransformPoints(DynamicParticleModifier*)`.
pub(crate) const DPM_TRANSFORM_POINTS: Callee = Callee {
    addr: 0x0034_5ef8,
    f: ee::s11::DynamicParticleModifier_TransformPoints,
};

/// Modifier fields: the node at +0x10, the base-ctor counter at +0x18, the
/// local box points at +0x20 and +0x50, the emitter sub-object at +0x60 (its
/// vtable +0x1f8 = the modifier's +0x258, its definition +0x260).
pub(crate) const DPM_NODE: u32 = 0x10;
pub(crate) const DPM_DONE: u32 = 0x18;
pub(crate) const DPM_EMITTER: u32 = 0x60;
pub(crate) const DPM_DEFINITION: u32 = 0x260;

/// The definition's local point (x, y, z at +0x48..+0x50, zeroed after).
pub(crate) const DPM_DEF_POINT: u32 = 0x48;

/// `void* DynamicParticleModifier_ctor(DynamicParticleModifier* mem, void*
/// desc, void* inst, int node, sIbVec4* pos)` (0x345c90): initialise the
/// base particle modifier, the emitter sub-object and the two local points,
/// hand the definition's point to the emitter, set it up with no time left,
/// transform the points into world space and count the instance. Frame 0x70
/// (s0 = self, s1 = desc, s2 = emitter, s3 = node, s4 = pos).
pub fn dynamic_particle_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 59;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A3][0];
        c.g[S4][0] = c.g[T0][0];
        frame.save(c, S2, 0x40);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[A2][0];
        c.g[S2][0] = sx(r32(c, S0).wrapping_add(DPM_EMITTER));
        DPM_BASE_CTOR.call(c, 0x0034_5cc8)?;
        c.g[V0][0] = 0x49_0000;
        c.g[A0][0] = c.g[S2][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1268));
        Addr(r32(c, S0).wrapping_add(8)).sw(c, c.g[V0][0]);
        call_ptr(c, DPM_EMITTER_CTOR, 0x0034_5cdc)?;
        c.g[V0][0] = 0x49_0000;
        Addr(r32(c, S0).wrapping_add(DPM_DEFINITION)).sw(c, c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1370));
        Addr(r32(c, S0).wrapping_add(DPM_NODE)).sw(c, c.g[S3][0]);
        Addr(r32(c, S0).wrapping_add(0x258)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        Addr(frame.at(c, 0xc)).sw(c, 0);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 12); // f12 = -1.0 (no time left)
        c.lq(2, r32(c, S4));
        c.sq(2, r32(c, S0).wrapping_add(0x20)); // localA = *pos
        c.lwc1(0, r32(c, S1).wrapping_add(DPM_DEF_POINT));
        c.lwc1(1, r32(c, S1).wrapping_add(DPM_DEF_POINT + 4));
        c.lwc1(2, r32(c, S1).wrapping_add(DPM_DEF_POINT + 8));
        c.swc1(0, frame.at(c, 0));
        c.swc1(1, frame.at(c, 4));
        c.swc1(2, frame.at(c, 8));
        Addr(r32(c, S0).wrapping_add(0x14)).sw(c, 0);
        c.lq(2, frame.at(c, 0));
        c.sq(2, r32(c, S0).wrapping_add(0x50)); // localB
        Addr(r32(c, S1).wrapping_add(DPM_DEF_POINT + 8)).sw(c, 0);
        Addr(r32(c, S1).wrapping_add(DPM_DEF_POINT)).sw(c, 0);
        Addr(r32(c, S1).wrapping_add(DPM_DEF_POINT + 4)).sw(c, 0);
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(DPM_DEFINITION)).lw(c);
        call_ptr(c, DPM_EMITTER_SETUP, 0x0034_5d40)?;
        c.g[A0][0] = c.g[S0][0];
        DPM_TRANSFORM_POINTS.call(c, 0x0034_5d48)?;
        c.g[V1][0] = Addr(r32(c, GP).wrapping_add(0xf0c)).lw(c);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x50);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        frame.restore(c, S2, 0x40);
        Addr(r32(c, GP).wrapping_add(0xf0c)).sw(c, c.g[V1][0]);
        Addr(r32(c, S0).wrapping_add(DPM_DONE)).sw(c, 0);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SPCH_FindEvent (0x3d7110).

/// The eight speech bank slots (a word per bank, 0: unused).
pub(crate) const SPCH_FIND_EVENT_BANKS: u32 = 0x0044_ffb8;

/// A speech bank: the bank id bytes at +8/+9, the event count at +0x10
/// (u16) and the event index table at +0x18 (u16 each, a `u16` index into
/// the bank, 4 bytes per entry).
pub(crate) const SPCH_FIND_EVENT_COUNT: u32 = 0x10;
pub(crate) const SPCH_FIND_EVENT_ENTRIES: u32 = 0x18;

/// `u16* SPCH_FindEvent(u8* request)` (0x3d7110): find the bank whose id
/// (bytes +8/+9) matches the request's (bytes +3/+2), then the bank's first
/// event entry whose value equals the request's event id (the `u16` at +0);
/// null when the bank or the event is not found. Frame 0x10 (the found slot
/// at +0).
pub fn spch_find_event(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x45_0000;
    c.g[A2][0] = sx(SPCH_FIND_EVENT_BANKS);
    c.g[T1][0] = 0;
    c.g[T2][0] = c.g[SP][0];
    c.g[T0][0] = 0;
    c.g[A3][0] = 0;
    'slots: loop {
        c.cycles += 3;
        c.g[A1][0] = Addr(r32(c, A2)).lw(c);
        if c.g[A1][0] == 0 {
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, A1).wrapping_add(9)).lbu(c);
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(2)).lbu(c);
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, A1).wrapping_add(8)).lbu(c);
                c.g[V0][0] = Addr(r32(c, A0).wrapping_add(3)).lbu(c);
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 3;
                    Addr(r32(c, T2)).sw(c, c.g[A2][0]);
                    c.g[T0][0] = 1;
                    break 'slots;
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A3][0] as i64) < 8) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(8));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003d_7130);
        }
    }
    c.cycles += 2;
    let no_bank = c.g[T0][0] == 0;
    c.g[V0][0] = Addr(r32(c, T2)).lw(c);
    let mut found = false;
    if !no_bank {
        c.cycles += 5;
        c.g[A2][0] = 0;
        c.g[A3][0] = Addr(r32(c, V0)).lw(c); // the bank
        c.g[T0][0] = Addr(r32(c, A3).wrapping_add(SPCH_FIND_EVENT_COUNT)).lhu(c);
        let empty = c.g[T0][0] == 0;
        c.g[A0][0] = Addr(r32(c, A0)).lhu(c); // the event id
        if !empty {
            c.cycles += 2;
            c.g[A1][0] = sx(r32(c, A3).wrapping_add(SPCH_FIND_EVENT_ENTRIES));
            loop {
                c.cycles += 6;
                c.g[V0][0] = Addr(r32(c, A1)).lhu(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = Addr(r32(c, V0)).lhu(c);
                if c.g[V1][0] != c.g[A0][0] {
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 2;
                    c.g[T1][0] = c.g[V0][0];
                    found = true;
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[T0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_7198);
                }
            }
        }
        // The `+1` and the cleared result belong to the bank branch only:
        // with no bank the original breaks out before them (and `t1` is
        // still 0 from the prologue), and a found event skips them too.
        if !found {
            c.cycles += 1;
            c.g[T1][0] = 0;
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[T1][0];
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- 0x00117ae8 RiderMetrix_ShowHUDText -----------------------------------

    /// `RiderMetrix_ShowHUDText`: no message table (-1), a fixed type below
    /// 34 and a searched one (a free slot, or the smallest ratio), the real
    /// `RiderMetrix_FindHUDSlot` and `cHUDFX_Show` with a text or a null
    /// one.
    #[test]
    fn rider_metrix_show_hud_text_matches() {
        use crate::difftest::{check_lohi, scramble, Rng, DATA};
        const SELF: u32 = DATA;
        const TABLE: u32 = DATA + 0x2000;
        const TEXT: u32 = DATA + 0x4000;
        check_lohi(
            ee::s00::f_00117ae8__generated,
            rider_metrix_show_hud_text,
            240,
            &|c, seed| {
                let mut r = Rng(0x117a_e800 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0011_7ae8;
                if seed % 5 == 0 {
                    c.bus.write32(SELF + RIDER_METRIX_HUD_TABLE, 0);
                } else {
                    c.bus.write32(SELF + RIDER_METRIX_HUD_TABLE, TABLE);
                    let count = 0x23 + r.next_u32() % 4;
                    c.bus.write32(SELF + 0x1b4, count);
                    // A free slot among the searched ones (35..count), if any.
                    let free = if seed % 3 == 2 && count > 0x23 {
                        0x23 + r.next_u32() % (count - 0x23)
                    } else {
                        u32::MAX
                    };
                    for i in 0..count {
                        let slot = TABLE + RIDER_METRIX_HUD_SLOT_SIZE * i;
                        c.bus.write32(slot, if i == free { 0x34 } else { 0x35 });
                        c.bus.write32(slot + 4, r.edge_f32());
                        c.bus.write32(slot + 8, r.edge_f32());
                        c.bus.write32(slot + 0xc, r.next_u32());
                        c.bus.write32(slot + 0x18, r.next_u32());
                    }
                }
                let n = r.next_u32() % 30;
                for i in 0..n {
                    c.bus.write8(TEXT + i, (r.next_u32() % 255 + 1) as u8);
                }
                c.bus.write8(TEXT + n, 0);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = if seed % 3 == 0 {
                    (34 + r.next_u32() % 4) as u64
                } else {
                    (r.next_u32() % 34) as u64
                };
                c.g[A2][0] = if r.next_u32() % 8 == 0 { 0 } else { sx(TEXT) };
                c.g[A3][0] = r.next_u32() as u64;
                c.f[12] = r.edge_f32();
            },
        );
    }

    // -- 0x001399e0 RiderAirState_Enter ---------------------------------------

    /// `RiderAirState_Enter`: previous states 0..8 (air only for 5) and the
    /// real `AirPredictor_Prime` on random predictor data.
    #[test]
    fn rider_air_state_enter_matches() {
        use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};
        const SELF: u32 = DATA;
        const RIDER: u32 = DATA + 0x100;
        check(
            ee::s01::f_001399e0__generated,
            rider_air_state_enter,
            240,
            &|c, seed| {
                let mut r = Rng(0x1399_e000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0013_99e0;
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(
                    GAME_GP.wrapping_sub(RIDER_AIR_LAUNCH_SPEED),
                    3333.3335f32.to_bits(),
                );
                fill(c, &mut r, DATA, 0x800 / 4);
                c.bus.write32(SELF + RIDER_AIR_RIDER, RIDER);
                c.bus.write32(RIDER + RIDER_AIR_PREDICTOR, DATA + 0x400);
                for i in 0..4 {
                    c.bus.write32(0x4ff_160 + 4 * i, r.next_u32());
                    c.bus.write32(0x4ff_120 + 4 * i, r.next_u32());
                }
                for i in 0..2 {
                    c.bus.write32(0x4a_5960 + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = (seed % 9) as u64;
            },
        );
    }

    // -- 0x003c8b80 SND_EeVoiceSetDistortion ----------------------------------

    /// `SND_EeVoiceSetDistortion`: amounts above 0, 0 and negative; the
    /// voice's DSP chain of 0-3 nodes; no distortion DSP yet (created and
    /// inserted through the real `SND_MemAlloc` and the driver heap), one
    /// already on the chain, or a detached one.
    #[test]
    fn snd_ee_voice_set_distortion_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check_lohi, fill, DATA};
        const VOICES: u32 = DATA + 0x20_0000;
        const NODES: u32 = DATA + 0x20_1000;
        const HEAP: u32 = DATA + 0x20_2000;
        check_lohi(
            ee::s14::f_003c8b80__generated,
            snd_ee_voice_set_distortion,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_8b80, 0);
                c.pc = 0x003c_8b80;
                let index = seed % 8;
                c.g[A0][0] = index as u64;
                c.bus.write32(SND_DIST_VOICES, VOICES);
                fill(c, &mut r, VOICES, SND_DIST_VOICE_SIZE * 8 / 4);
                let voice = VOICES + SND_DIST_VOICE_SIZE * index;
                // The DSP chain: 0-3 nodes {next +8, priority +0x18}; the
                // stride leaves room for the 0x50-byte distortion node.
                let n = seed / 8 % 4;
                for i in 0..n {
                    let node = NODES + 0x60 * i;
                    fill(c, &mut r, node, 0x18);
                    c.bus
                        .write32(node + 8, if i + 1 == n { 0 } else { node + 0x60 });
                    c.bus
                        .write16(node + 0x18, (0x10 + 0x20 * i + r.next_u32() % 0x20) as u16);
                }
                c.bus
                    .write32(voice + SND_DIST_VOICE_CHAIN, if n == 0 { 0 } else { NODES });
                let dsp = match seed % 3 {
                    0 => 0,
                    1 if n > 0 => NODES + 0x60 * (r.next_u32() % n),
                    _ => NODES + 0x200,
                };
                if dsp == NODES + 0x200 {
                    fill(c, &mut r, dsp, 0x14);
                }
                c.bus.write32(voice + SND_DIST_VOICE_DSP, dsp);
                // The driver heap: empty, room for the block.
                for state in [0x0051_5b40u32, SND_APPLY_STATE] {
                    c.bus.write32(state + 0x234, HEAP);
                }
                c.bus.write32(HEAP, DATA + 0x20_4000);
                c.bus.write32(HEAP + 0x4, DATA + 0x20_3000);
                c.bus.write32(HEAP + 0xc, 0x8_0000);
                c.bus.write32(HEAP + 0x10, 0x100);
                c.bus.write32(HEAP + 0x14, 0);
                c.g[A1][0] = match seed / 5 % 5 {
                    0 => 0,
                    1 => sx(0x8000_0000 | r.next_u32()),
                    2 => (r.next_u32() as u64) << 32 | (r.next_u32() % 20000) as u64,
                    _ => 1 + (r.next_u32() % 0x2000) as u64,
                };
            },
        );
    }

    // -- 0x003c30c8 SND_ChannelApplyDistortion, 0x003c3250 highpass ----------

    /// A channel with a voice list whose numbers map onto voices 0-7 of a
    /// zeroed voice table, the DSP nodes and the driver heap for the two
    /// voice setters (their own tests' fixtures).
    fn snd_apply_fixture(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        max_voices: u32,
    ) -> crate::difftest::Rng {
        use crate::difftest::fakes::machine;
        use crate::difftest::fill;
        const CHANNELS: u32 = crate::difftest::DATA + 0x1000;
        let mut r = machine(c, seed, salt, 0);
        let ch = seed % 8;
        c.g[A0][0] = ch as u64;
        c.bus
            .write32(SND_APPLY_STATE + SND_APPLY_CHANNELS, CHANNELS);
        let chan = CHANNELS + SND_APPLY_CHANNEL_SIZE * ch;
        c.bus.write16(
            chan + SND_APPLY_FLAGS,
            if seed % 2 == 0 {
                4
            } else {
                (r.next_u32() & 0xffff & !4) as u16
            },
        );
        let count = seed % (max_voices + 1);
        c.bus.write8(chan + SND_APPLY_VOICE_COUNT, count as u8);
        c.bus.write8(SND_APPLY_STATE + SND_APPLY_VOICE_BASE_A, 0);
        c.bus.write8(SND_APPLY_STATE + SND_APPLY_VOICE_BASE_B, 0);
        for i in 0..max_voices {
            c.bus
                .write16(chan + SND_APPLY_VOICES + 2 * i, (r.next_u32() % 8) as u16);
        }
        // The voice table: 8 entries, each with a 0-2 node chain and a
        // filter/DSP node (sometimes detached, sometimes absent). A DSP node
        // is up to 0x120 bytes (the FIR highpass), so chains and detached
        // nodes get a stride no write can overrun.
        const VOICES: u32 = crate::difftest::DATA + 0x20_0000;
        const NODES: u32 = crate::difftest::DATA + 0x20_1000;
        const NODE_STRIDE: u32 = 0x140;
        const VOICE_STRIDE: u32 = 0x500;
        const DSP_DETACHED: u32 = crate::difftest::DATA + 0x20_4000;
        const HP_DETACHED: u32 = crate::difftest::DATA + 0x20_5000;
        const HEAP: u32 = crate::difftest::DATA + 0x20_6000;
        const BLOCKS: u32 = crate::difftest::DATA + 0x20_6400;
        const BASE: u32 = crate::difftest::DATA + 0x20_7000;
        c.bus.write32(SND_DIST_VOICES, VOICES);
        fill(c, &mut r, VOICES, SND_DIST_VOICE_SIZE * 8 / 4);
        for i in 0..8u32 {
            let voice = VOICES + SND_DIST_VOICE_SIZE * i;
            let chain = NODES + VOICE_STRIDE * i;
            let n = r.next_u32() % 3;
            for k in 0..n {
                let node = chain + NODE_STRIDE * k;
                fill(c, &mut r, node, NODE_STRIDE / 4);
                c.bus
                    .write32(node + 8, if k + 1 == n { 0 } else { node + NODE_STRIDE });
                c.bus
                    .write16(node + 0x18, (0x10 + 0x20 * k + r.next_u32() % 0x20) as u16);
            }
            c.bus
                .write32(voice + SND_DIST_VOICE_CHAIN, if n == 0 { 0 } else { chain });
            let dsp = match r.next_u32() % 3 {
                0 => 0,
                1 if n > 0 => chain + NODE_STRIDE * (r.next_u32() % n),
                _ => DSP_DETACHED + 0x80 * i,
            };
            if dsp == DSP_DETACHED + 0x80 * i {
                fill(c, &mut r, dsp, 0x14);
            }
            c.bus.write32(voice + SND_DIST_VOICE_DSP, dsp);
            // The same voice's highpass DSP node lives at +0x5c.
            let hp = match r.next_u32() % 3 {
                0 => 0,
                1 if n > 0 => chain + NODE_STRIDE * (r.next_u32() % n),
                _ => HP_DETACHED + NODE_STRIDE * i,
            };
            if hp == HP_DETACHED + NODE_STRIDE * i {
                fill(c, &mut r, hp, 0x48);
            }
            c.bus.write32(voice + 0x5c, hp);
        }
        // The driver heap: empty, a 0x4000 arena past every node region.
        for state in [0x0051_5b40u32, SND_APPLY_STATE] {
            c.bus.write32(state + 0x234, HEAP);
        }
        c.bus.write32(HEAP, BASE);
        c.bus.write32(HEAP + 0x4, BLOCKS);
        c.bus.write32(HEAP + 0xc, 0x4000);
        c.bus.write32(HEAP + 0x10, 0x100);
        c.bus.write32(HEAP + 0x14, 0);
        c.bus.write16(
            SND_APPLY_STATE + 0x2a,
            [48000u32, 44100, 24000, r.next_u32() % 0x1_0000][(seed / 3 % 4) as usize] as u16,
        );
        r
    }

    /// `SND_ChannelApplyDistortion`: channels with and without the flag, 0-3
    /// voices, the real `SND_EeVoiceSetDistortion` per voice with a positive,
    /// zero or negative amount.
    #[test]
    fn snd_channel_apply_distortion_matches() {
        use crate::difftest::check_lohi;
        check_lohi(
            ee::s13::f_003c30c8__generated,
            snd_channel_apply_distortion,
            240,
            &|c, seed| {
                let mut r = snd_apply_fixture(c, seed, 0x3c_30c8, 3);
                c.pc = 0x003c_30c8;
                c.g[A1][0] = match seed / 5 % 5 {
                    0 => 0,
                    1 => sx(0x8000_0000 | r.next_u32()),
                    2 => (r.next_u32() as u64) << 32 | (r.next_u32() % 20000) as u64,
                    _ => 1 + (r.next_u32() % 0x2000) as u64,
                };
            },
        );
    }

    /// `SND_ChannelApplyHighpass`: the same fixture, the real
    /// `SND_EeVoiceSetHighpass` per voice with a cutoff of 0, a negative
    /// value or a frequency.
    #[test]
    fn snd_channel_apply_highpass_matches() {
        use crate::difftest::check_lohi;
        check_lohi(
            ee::s13::f_003c3250__generated,
            snd_channel_apply_highpass,
            240,
            &|c, seed| {
                let mut r = snd_apply_fixture(c, seed, 0x3c_3250, 3);
                c.pc = 0x003c_3250;
                c.g[A1][0] = match seed / 5 % 5 {
                    0 => 0,
                    1 => sx(0x8000_0000 | r.next_u32()),
                    2 => (r.next_u32() as u64) << 32 | (r.next_u32() % 20000) as u64,
                    _ => 1 + (r.next_u32() % 20000) as u64,
                };
            },
        );
    }

    // -- 0x00345c90 DynamicParticleModifier_ctor ------------------------------

    /// `DynamicParticleModifier_ctor`: the real base ctor, the emitter ctor
    /// (`DynamicEmitter_ctor`, an `Emitter_Init` thunk and `memset`), the
    /// real `DynamicEmitter_Setup` (its own test's definition fixture, with
    /// the emitter's reset methods at the modifier's +0x258 = 0x491370 as
    /// fakes) and the real
    /// `DynamicParticleModifier_TransformPoints` (an instance with no owner
    /// and node -1 or a 0-2 parent chain).
    #[test]
    fn dynamic_particle_modifier_ctor_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        const SELF: u32 = DATA + 0x2000;
        const DEF: u32 = DATA + 0x1000;
        const POS: u32 = DATA + 0x500;
        const INST: u32 = DATA;
        const SKEL: u32 = DATA + 0x400;
        const NODES: u32 = DATA + 0x600;
        const MATS: u32 = DATA + 0x800;
        const DYN_VTABLE: u32 = 0x0049_1370;
        check(
            ee::s11::f_00345c90__generated,
            dynamic_particle_modifier_ctor,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x345c90, 0);
                c.pc = 0x0034_5c90;
                c.g[GP][0] = GAME_GP as u64;
                // The emitter's reset methods, at the vtable the ctor
                // installs (slot -> {delta, method}).
                for (slot, i) in [(0x8u32, 0u32), (0x28, 1), (0x10, 2)] {
                    c.bus.write16(DYN_VTABLE + slot, 0);
                    c.bus.write32(DYN_VTABLE + slot + 4, FAKE + 0x10 * i);
                }
                // The modifier and its definition (the setup test's fixture).
                fill(c, &mut r, SELF, 0x300 / 4);
                for i in 0..0x40u32 {
                    c.bus.write32(DEF + 4 * i, r.f32(50.0));
                }
                c.bus.write32(DEF, 1 + r.next_u32() % 4);
                c.bus.write32(DEF + 4, r.next_u32() % 4);
                if seed % 5 != 0 {
                    c.bus.write32(DEF + 0xc, 1.5f32.to_bits());
                }
                c.bus.write32(DEF + 0xd0, seed % 4);
                c.bus.write32(DEF + 0xc4, r.next_u32());
                c.bus.write32(DEF + 0xc8, r.next_u32());
                let start = match seed % 4 {
                    0 => 1.0f32.to_bits(),
                    1 => r.f32(5.0) | 0x8000_0000,
                    _ => r.f32(5.0) & 0x7fff_ffff,
                };
                c.bus.write32(DEF + 0x8, start);
                c.bus.write32(
                    SELF + DPM_EMITTER + 0x178,
                    if seed % 8 == 0 {
                        60
                    } else {
                        r.next_u32() % 400
                    },
                );
                // Rand_NextMain's state, the descriptor tables and templates.
                fill(c, &mut r, 0x004f_f018, 6);
                let base = (GAME_GP as i32 - 0x2868) as u32;
                for i in 0..24u32 {
                    c.bus.write32(base + 4 * i, r.f32(10.0));
                }
                fill(c, &mut r, 0x0050_0000 - 0xee0, 4);
                for i in 0..0x10u32 {
                    c.bus.write32(0x0050_0000 - 0xe60 + 4 * i, r.f32(4.0));
                }
                fill(c, &mut r, 0x0050_1420, 5);
                // The instance and its (optional) skeleton node chain.
                for k in 0..0x10 {
                    c.bus.write32(INST + 0x10 + 4 * k, r.f32(4.0));
                }
                c.bus.write32(INST + 0xc, 0);
                c.bus.write32(INST + 0x84, r.f32(2.0));
                c.bus.write32(INST + 0x80, SKEL);
                c.bus.write32(SKEL + 8, NODES);
                for (n, parent) in [(0u32, u32::MAX), (1, 0), (2, 1), (3, 0)] {
                    c.bus.write32(NODES + 0x10 * n, parent);
                    c.bus.write32(NODES + 0x10 * n + 0xc, MATS + 0x40 * n);
                }
                for k in 0..0x40 {
                    c.bus.write32(MATS + 4 * k, r.f32(4.0));
                }
                // The position, and the definition point the ctor zeroes.
                for k in 0..4 {
                    c.bus.write32(POS + 4 * k, r.f32(10.0));
                }
                for k in 0..3 {
                    c.bus.write32(DEF + DPM_DEF_POINT + 4 * k, r.edge_f32());
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(DEF);
                c.g[A2][0] = sx(INST);
                c.g[A3][0] = match seed % 5 {
                    0 => u64::MAX,
                    k => (k - 1) as u64,
                };
                c.g[T0][0] = sx(POS);
            },
        );
    }

    // -- 0x003d7110 SPCH_FindEvent --------------------------------------------

    /// `SPCH_FindEvent`: eight bank slots (some empty, and the request's id
    /// matching one bank's or none), 0-4 event entries per bank with the
    /// request's event id present or absent.
    #[test]
    fn spch_find_event_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        const REQUEST: u32 = DATA + 0x100;
        const BANKS: u32 = DATA + 0x1000;
        check(
            ee::s14::f_003d7110__generated,
            spch_find_event,
            240,
            &|c, seed| {
                let mut r = Rng(0x3d71_1000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003d_7110;
                let ev = r.next_u32() as u16;
                let sel = (seed % 8) as u32;
                let id_bytes = (r.next_u32() as u8, r.next_u32() as u8);
                for i in 0..8u32 {
                    let bank = BANKS + 0x100 * i;
                    let slot = SPCH_FIND_EVENT_BANKS + 8 * i;
                    c.bus.write32(
                        slot,
                        if i == sel || r.next_u32() % 3 != 0 {
                            bank
                        } else {
                            0
                        },
                    );
                    let (b9, b8) = if i == sel {
                        id_bytes
                    } else {
                        (r.next_u32() as u8, r.next_u32() as u8)
                    };
                    c.bus.write8(bank + 9, b9);
                    c.bus.write8(bank + 8, b8);
                    let count = (r.next_u32() % 5) as u16;
                    c.bus.write16(bank + SPCH_FIND_EVENT_COUNT, count);
                    let mut first = u32::MAX;
                    for k in 0..count {
                        let index = 0x20 + (r.next_u32() % 0x18) as u16;
                        if k == 0 {
                            first = index as u32;
                        }
                        c.bus
                            .write16(bank + SPCH_FIND_EVENT_ENTRIES + 2 * k as u32, index);
                    }
                    if i == sel && count > 0 && seed % 3 == 0 {
                        c.bus.write16(bank + 4 * first, ev);
                    }
                }
                let (req2, req3) = if seed % 2 == 0 {
                    id_bytes
                } else {
                    (r.next_u32() as u8, r.next_u32() as u8)
                };
                c.bus.write16(REQUEST, ev);
                c.bus.write8(REQUEST + 2, req2);
                c.bus.write8(REQUEST + 3, req3);
                c.g[A0][0] = sx(REQUEST);
            },
        );
    }

    // -- 0x00350570 WorldObj_GetBoundBox --------------------------------------

    /// `WorldObj_GetBoundBox`: random corners and centre (edge-case floats,
    /// zeroes, wide ranges) on the node's matrix data; the eight probe
    /// corners are the combinations of the two box corners.
    #[test]
    fn world_obj_get_bound_box_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        const SELF: u32 = DATA;
        const NODE: u32 = DATA + 0x100;
        const BOX: u32 = DATA + 0x400;
        const RADIUS: u32 = DATA + 0x500;
        check(
            ee::s11::f_00350570__generated,
            world_obj_get_bound_box,
            240,
            &|c, seed| {
                let mut r = Rng(0x3505_7000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(BOX);
                c.g[A2][0] = sx(RADIUS);
                c.bus.write32(SELF + WORLDOBJ_NODE, NODE);
                for k in 0..0x80u32 {
                    let v = match (seed as u32 + k) % 7 {
                        0 => r.edge_f32(),
                        1 => 0,
                        _ => r.f32(1e5),
                    };
                    c.bus.write32(NODE + 4 * k, v);
                }
            },
        );
    }

    // -- 0x003d95b8 SPCH_ResolveEventParts ------------------------------------

    /// `SPCH_ResolveEventParts`: the shuffle/assign path (`mode == 1`, and
    /// `mode == 3` with the layer's in-progress flag set) and the plain path
    /// (three entries, the first sometimes empty), a request whose bank id
    /// matches the first bank slot or not, variation lists with `0xfe` ids
    /// whose column bit is clear or set, and real callees:
    /// `SPCH_RandomNoRepeat`, `SPCH_AssignVariations`,
    /// `SPCH_FindBankDataPtr` and `SPCH_DecodeSampleEntry` over a real
    /// speech bank.
    #[test]
    fn spch_resolve_event_parts_matches() {
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        // Speech bank header offsets (`SPCH_DecodeSampleEntry`'s view).
        const BANK_FLAGS: u32 = 0x4;
        const BANK_COUNT: u32 = 0x5;
        const BANK_BLOCKS: u32 = 0x7;
        const BANK_END: u32 = 0x8;
        const REQUEST: u32 = DATA + 0x1000;
        const H: u32 = DATA + 0x2000;
        const LAYER: u32 = DATA + 0x4000;
        const TABLE: u32 = DATA + 0x8000;
        const BANK: u32 = DATA + 0x1_0000;
        const BANK_OUT: u32 = DATA + 0x2_0000;
        const BANK2: u32 = DATA + 0x3_0000;
        const SLOTS: u32 = 0x0044_ffb8;
        check(
            ee::s14::f_003d95b8__generated,
            spch_resolve_event_parts,
            240,
            &|c, seed| {
                let mut r = Rng(0x3d95_b800 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003d_95b8;
                c.g[GP][0] = GAME_GP as u64;
                // The speech bank: header fields over random entries.
                for i in 0..0x240u32 {
                    c.bus.write8(BANK + i, r.next_u32() as u8);
                }
                // Up to 4 extra bytes: the decode record's extras stay below
                // the out pointer at sp+0x10 and the result at sp+0x18.
                c.bus.write8(BANK + BANK_FLAGS, (r.next_u32() % 5) as u8);
                c.bus.write8(BANK + BANK_COUNT, 0xff);
                c.bus.write8(BANK + BANK_BLOCKS, r.next_u32() as u8);
                c.bus
                    .write16(BANK + BANK_END, (r.next_u32() % 0x200) as u16);
                // The bank's decoded-data buffer: `SPCH_FindBankDataPtr`
                // returns `lw(bank) + 0x18 + round2(u16 at lw(bank) + 0x10)`
                // (0 with a zero count), and the columns are written there.
                c.bus.write32(BANK, BANK_OUT);
                c.bus.write32(BANK_OUT + 0x10, 0);
                c.bus.write32(TABLE, BANK);
                c.bus.write32(GAME_GP + SPCH_RESOLVE_GP_BANKS, TABLE);
                // The eight bank slots: this bank, one other (its id does not
                // match), and empty ones.
                for i in 0..8u32 {
                    c.bus.write32(
                        SLOTS + 8 * i,
                        match i {
                            0 => BANK,
                            1 => BANK2,
                            _ => 0,
                        },
                    );
                }
                c.bus.write8(BANK2 + 8, r.next_u32() as u8);
                c.bus.write8(BANK2 + 9, r.next_u32() as u8);
                // The request's bank id (bytes 2, 3) matches or does not.
                let end = c.bus.read16(BANK + BANK_END);
                let (b2, b3) = if seed % 2 == 0 {
                    ((end >> 8) as u8, end as u8)
                } else {
                    (r.next_u32() as u8, r.next_u32() as u8)
                };
                c.bus.write32(REQUEST, r.next_u32());
                c.bus.write8(REQUEST + 2, b2);
                c.bus.write8(REQUEST + 3, b3);
                // Three selection entries (cnt 14), all in bank 0.
                for i in 0..3u32 {
                    let e = H + SPCH_RESOLVE_ENT_SIZE * i;
                    c.bus.write16(e, 0);
                    c.bus
                        .write8(e + SPCH_RESOLVE_ENT_START, (r.next_u32() % 0x40) as u8);
                    let len = if seed % 5 == 0 && i == 0 {
                        0
                    } else {
                        1 + (r.next_u32() % 8) as u8
                    };
                    c.bus.write8(e + SPCH_RESOLVE_ENT_LEN, len);
                    c.bus.write8(e + SPCH_RESOLVE_ENT_CUR, r.next_u32() as u8);
                }
                for i in 0..0x100u32 {
                    c.bus.write8(H + SPCH_RESOLVE_TBL + i, r.next_u32() as u8);
                }
                // Three entries normally; a count below 4 (no entry) also
                // covers the empty selection set.
                let cnt = if seed % 7 == 3 { (seed % 4) as u8 } else { 14 };
                c.bus.write8(LAYER + SPCH_RESOLVE_LAYER_CNT, cnt);
                let n5 = if seed % 8 == 7 {
                    1 + (r.next_u32() % 3) as u8
                } else {
                    0
                };
                c.bus.write8(LAYER + SPCH_RESOLVE_LAYER_N5, n5);
                let mut lists = [0u8; 3];
                for (i, slot) in lists.iter_mut().enumerate() {
                    *slot = (r.next_u32() % 0x40) as u8;
                    c.bus
                        .write8(LAYER + SPCH_RESOLVE_LAYER_LIST + i as u32, *slot);
                }
                for li in lists {
                    let base = LAYER + 4 * li as u32;
                    for j in 0..0x18u32 {
                        c.bus.write8(base + j, r.next_u32() as u8);
                    }
                    let n = if seed % 6 == 0 {
                        0
                    } else {
                        1 + (r.next_u32() % 8) as u8
                    };
                    c.bus.write8(base + SPCH_RESOLVE_LIST_N, n);
                    for j in 0..8u32 {
                        let id = if seed % 3 == 0 || r.next_u32() % 2 == 0 {
                            0xfe
                        } else {
                            0x7f
                        };
                        c.bus.write8(base + SPCH_RESOLVE_LIST_IDS + j, id);
                        let col = if r.next_u32() % 3 == 0 {
                            r.next_u32() as u8 | 0x80
                        } else {
                            r.next_u32() as u8 & 0x7f
                        };
                        c.bus.write8(base + SPCH_RESOLVE_LIST_COLS + j, col);
                    }
                }
                c.g[A0][0] = sx(REQUEST);
                c.g[A1][0] = sx(H);
                c.g[A2][0] = sx(LAYER);
                c.g[A3][0] = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    _ => 3,
                };
                c.g[T0][0] = 0;
            },
        );
    }
}
