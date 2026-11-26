//! Staging module oc1330 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1330`.

use crate::call::{guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::game::misc::{
    HUFF_CODE, HUFF_CODE_LEN, HUFF_CUR, HUFF_DELTA_BASE, HUFF_DELTA_MAX, HUFF_DELTA_MIN,
    HUFF_IN_END, HUFF_IN_PTR, HUFF_LIT_SEL, HUFF_MODE, HUFF_NUM_BITS, HUFF_NUM_COUNT,
    HUFF_ORDER_COUNT, HUFF_ORDER_TAB, HUFF_SYM_FLAG,
};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_76c0, huff_decode_stream, "HUFF_DecodeStream"),
    (0x002c_8e48, huff_analysis, "HUFF_analysis"),
];


// ---------------------------------------------------------------------------
// HUFF_analysis (0x2c8e48)

/// Stack frame size of `HUFF_analysis`.
const ANALYSIS_FRAME: u32 = 0x8c0;

/// Per-run input counter in the analysis state.
const AN_RUN_COUNT: u32 = 0x900;

/// Sum of the input bytes in the analysis state.
const AN_BYTE_SUM: u32 = 0x2dbc;

/// Slot holding the input size (`a2` on entry) in the analysis state.
const AN_SIZE_SLOT: u32 = 0x2dc8;

/// Delta-window search source in the analysis state.
const AN_DELTA_SRC: u32 = 0x2de8;

/// Per-length symbol counts (`s1 + 0xd00`).
const AN_LEN_COUNTS: u32 = 0xd00;

/// Scratch table low end (`s4`-relative) for the delta search.
const AN_SCR_LO: u32 = 0x404;

/// Scratch table high end (`s4`-relative) for the delta search.
const AN_SCR_HI: u32 = 0x7fc;

/// Scratch table output (`s4`-relative); aliases the run counter word.
const AN_SCR_OUT: u32 = 0x800;

/// Div-result spill in the analysis frame.
const AN_SP_LO: u32 = 0x800;

/// Code-length base (`s5`) spill in the analysis frame.
const AN_SP_CODETAB: u32 = 0x804;

/// Small-input flag in the analysis frame.
const AN_SP_SMALL: u32 = 0x808;

/// Scratch buffer base (`sp + 0x400`) in the analysis frame.
const AN_SP_SCRATCH: u32 = 0x80c;

/// Length-counts base (`s1 + 0xd00`) spill in the analysis frame.
const AN_SP_LENTAB: u32 = 0x810;

/// Entry-`a1` flag word (`a1 & 0x20`) in the analysis frame.
const AN_SP_FLAGS: u32 = 0x814;

/// Callees of this module's ports (functions.tsv names; `HUFF_MINREP` reuses
/// the crate's existing slot).
mod callee {
    use super::Callee;
    use super::ee;
    pub(crate) use crate::game::misc::callee::HUFF_MINREP;

    /// `void HUFF_maketree(s)` (0x2c8770): rebuild the decode tables.
    pub(crate) const HUFF_MAKETREE: Callee = Callee {
        addr: 0x002c_8770,
        f: ee::s09::f_002c8770,
    };
}

// ---------------------------------------------------------------------------
// HUFF_DecodeStream (0x2c76c0)

/// Stack frame size of `HUFF_DecodeStream` (saves s0-s7/fp, tables, spills).
const DECODE_FRAME: u32 = 0x570;

/// First level-table entry (`sp + 0x4`) in the decode frame: the class-1
/// count priming the symbol-table init after the output table is filled.
const DS_LVL1: u32 = 0x4;

/// Word holding the output pointer (`a1` on entry) in the decode frame.
const DS_OUT: u32 = 0x4c0;

/// Word holding the decoded length (the `v0` result) in the decode frame.
const DS_LEN: u32 = 0x4c4;

/// Spill slot for the match-run bit (`t8`) in the decode frame.
const DS_T8: u32 = 0x4c8;

/// Pointer to the level table (`sp + 0x40`) in the decode frame.
const DS_BASE: u32 = 0x4cc;

/// Pointer to the length table (`sp + 0x80`) in the decode frame.
const DS_TAB: u32 = 0x4d0;

/// Resume entries of `HUFF_DecodeStream`: the entry plus every loop head an
/// event check unwinds to (the `c.pc` values in its back edges).
mod decode {
    pub const ENTRY: u32 = 0x002c_76c0;
    pub const END: u32 = 0x002c_8344;
    /// Outer table-build loop.
    pub const TBL_BUILD: u32 = 0x002c_7880;
    /// Per-level bit loop.
    pub const TBL_STEP: u32 = 0x002c_7884;
    /// First code-length bit loop.
    pub const BIT_LOOP_B: u32 = 0x002c_78f8;
    /// Second code-length bit loop.
    pub const BIT_LOOP_A: u32 = 0x002c_7958;
    /// Scratch-table clear loop.
    pub const CLEAR_TBL: u32 = 0x002c_7ae8;
    /// Second table-build pass.
    pub const TBL2: u32 = 0x002c_7b18;
    /// Third code-length bit loop.
    pub const BIT_LOOP_C: u32 = 0x002c_7b78;
    /// Fourth code-length bit loop.
    pub const BIT_LOOP_D: u32 = 0x002c_7bd8;
    /// Symbol-scan loop.
    pub const SCAN_SYM: u32 = 0x002c_7d08;
    /// Output-table fill loop.
    pub const OUT_FILL: u32 = 0x002c_7d58;
    /// Byte-copy loop.
    pub const COPY_PAIR: u32 = 0x002c_7dd8;
    /// Copy-restart test.
    pub const COPY_RESTART: u32 = 0x002c_7db8;
    /// Symbol-table init loop.
    pub const SYM_INIT: u32 = 0x002c_7d98;
    /// Main decode loop.
    pub const MAIN: u32 = 0x002c_7e60;
    /// Multi-symbol decode chain.
    pub const SYMBOLS: u32 = 0x002c_7e78;
    /// Single-symbol refill step.
    pub const SYMBOL: u32 = 0x002c_7e28;
    /// Match-length bit loop.
    pub const MATCH_LEN: u32 = 0x002c_7f48;
    /// First length-class bit loop.
    pub const LEN_BITS: u32 = 0x002c_8040;
    /// Second length-class bit loop.
    pub const LEN_BITS2: u32 = 0x002c_80a0;
    /// Match-run fill loop.
    pub const RUN_FILL: u32 = 0x002c_81d8;
    /// Checksum accumulation loop.
    pub const CHECKSUM: u32 = 0x002c_82a8;
    /// Delta accumulation loop.
    pub const DELTA_ACC: u32 = 0x002c_82f8;
}

/// Stages of `HUFF_DecodeStream` between its resume entries: each stage runs
/// forward from its head to the next head (shared regions live in helpers
/// below, which is why several stages call them).
#[derive(Clone, Copy, PartialEq, Debug)]
enum DecodeStage {
    Entry,
    TblBuild,
    TblStep,
    BitLoopB,
    BitLoopA,
    ClearTbl,
    Tbl2,
    BitLoopC,
    BitLoopD,
    ScanSym,
    OutFill,
    CopyPair,
    CopyRestart,
    SymInit,
    Main,
    Symbols,
    Symbol,
    MatchLen,
    LenBits,
    LenBits2,
    RunFill,
    Checksum,
    DeltaAcc,
}

/// Where a shared decode region transfers control: resume another stage, or
/// stop (returned from the function, or unwound for an event).
type TailNext = Option<DecodeStage>;

/// Refill the bit buffer from the stream: the next two bytes extend `a3`,
/// shifted into `a2` by the bit deficit. One identical fragment at 18 sites
/// (the charge stays at the call site: +10, or +11/+9/+1/+2 around splits).
#[inline(always)]
fn decode_refill_bits(c: &mut Cpu) {
    c.g[V1][0] = c.lbu(c.g[T1][0] as u32);
    c.g[V0][0] = sx((c.g[A3][0] as u32) << 8);
    c.g[A0][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x1));
    c.g[A1][0] = sx(0u32.wrapping_sub(c.g[T2][0] as u32));
    c.g[A3][0] = c.g[V1][0] | c.g[V0][0];
    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2));
    c.g[V0][0] = sx((c.g[A3][0] as u32) << 8);
    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x10));
    c.g[A3][0] = c.g[A0][0] | c.g[V0][0];
    c.g[A2][0] = sx((c.g[A3][0] as u32) << ((c.g[A1][0] as u32) & 31));
}

/// Run-cost region (0x2c7998..0x2c7a78): decode the repeat count (`t4`) into
/// a table index (`v0`), by direct shift or by refill-assisted bit tests,
/// then fall into the table store below. Entered at the top from the first
/// code-length bit loops.
fn decode_run_cost(c: &mut Cpu) -> TailNext {
    // 0x002c7998.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[T4][0] as i64) < 17i64) as u64;
    let short = c.g[V0][0] != 0u64;
    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[T4][0] as u32));
    if short {
        // 0x002c7a38.
        c.cycles += 4;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[T4][0] as u32));
        c.g[T0][0] = sx((c.g[A2][0] as u32) >> ((c.g[V0][0] as u32) & 31));
        c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[T4][0] as u32) & 31));
        if (c.g[T2][0] as i64) >= 0 {
            // 0x002c7a70.
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[S3][0] as u32) << ((c.g[T4][0] as u32) & 31));
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        } else {
            // 0x002c7a48: refill, then the same join.
            c.cycles += 10;
            decode_refill_bits(c);
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[S3][0] as u32) << ((c.g[T4][0] as u32) & 31));
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
    } else {
        // 0x002c79a4.
        c.cycles += 8;
        c.g[V0][0] = 0x30;
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[T4][0] as u32));
        c.g[V1][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x10));
        c.g[T0][0] = sx((c.g[A2][0] as u32) >> ((c.g[V0][0] as u32) & 31));
        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T4][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[V1][0] as u32) & 31));
        if (c.g[T2][0] as i64) >= 0 {
            // 0x002c79ec.
            c.cycles += 4;
            c.g[T5][0] = sx((c.g[A2][0] as u32) >> 16);
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x10));
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 16);
            if (c.g[T2][0] as i64) < 0 {
                // 0x002c79fc: refill, then the same join.
                c.cycles += 10;
                decode_refill_bits(c);
            }
        } else {
            // 0x002c79c4: refill, then the 0x002c79ec join.
            c.cycles += 10;
            decode_refill_bits(c);
            c.cycles += 4;
            c.g[T5][0] = sx((c.g[A2][0] as u32) >> 16);
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x10));
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 16);
            if (c.g[T2][0] as i64) < 0 {
                c.cycles += 10;
                decode_refill_bits(c);
            }
        }
        // 0x002c7a24.
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[T0][0] as u32) << 16);
        c.g[V1][0] = sx((c.g[S3][0] as u32) << ((c.g[T4][0] as u32) & 31));
        c.g[V0][0] = c.g[T5][0] | c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    }
    // 0x002c7a78.
    c.cycles += 1;
    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    decode_table_store(c)
}

/// Table-store region (0x2c7a7c..0x2c7ac0): file the decoded index into the
/// level table, advance the level, and either restart the build, step to the
/// next level entry, or leave for the table clear. Entered at the top from
/// the run-cost region and from the per-level bit loop.
fn decode_table_store(c: &mut Cpu) -> TailNext {
    // 0x002c7a7c.
    c.cycles += 6;
    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[T9][0] as u32));
    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[T0][0]);
    c.g[T8][0] = sx((c.g[T8][0] as u32).wrapping_add(c.g[T0][0] as u32));
    let level_done = c.g[T0][0] == 0u64;
    c.g[A0][0] = 0u64;
    if !level_done {
        // 0x002c7a94.
        c.cycles += 4;
        c.g[V0][0] = 0x10;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[T3][0] as u32));
        c.g[V0][0] = sx((c.g[T8][0] as u32) << ((c.g[V0][0] as u32) & 31));
        c.g[A0][0] = c.g[V0][0] & 0xffff;
    }
    // 0x002c7aa4.
    c.cycles += 5;
    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_TAB));
    c.g[T3][0] = c.g[T7][0];
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T9][0] as u32));
    let restart = c.g[T0][0] == 0u64;
    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
    if restart {
        if back_edge_unwinds(c, decode::TBL_BUILD) {
            return None;
        }
        return Some(DecodeStage::TblBuild);
    }
    // 0x002c7ab8.
    c.cycles += 2;
    let step_next = c.g[A0][0] != 0u64;
    c.g[V1][0] = sx((c.g[T3][0] as u32) << 2);
    if step_next {
        if back_edge_unwinds(c, decode::TBL_STEP) {
            return None;
        }
        return Some(DecodeStage::TblStep);
    }
    // 0x002c7ac0.
    c.cycles += 10;
    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_TAB));
    c.g[S3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x1));
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = c.g[V1][0] | 0xffff;
    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    c.g[A0][0] = 0x10;
    c.g[V0][0] = c.g[S4][0];
    Some(DecodeStage::ClearTbl)
}

/// Symbol-cost region (0x2c7c18..0x2c7cfc): decode the second table's repeat
/// count (`t3`) into an index and file it, by direct shift or by
/// refill-assisted bit tests. Entered at the top from the second
/// code-length bit loops; falls through to the symbol scan.
fn decode_symbol_cost(c: &mut Cpu) {
    // 0x002c7c18.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[T3][0] as i64) < 17i64) as u64;
    let short = c.g[V0][0] != 0u64;
    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[T3][0] as u32));
    if short {
        // 0x002c7cb8.
        c.cycles += 4;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[T3][0] as u32));
        c.g[T0][0] = sx((c.g[A2][0] as u32) >> ((c.g[V0][0] as u32) & 31));
        c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[T3][0] as u32) & 31));
        if (c.g[T2][0] as i64) >= 0 {
            // 0x002c7cf0.
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[S5][0] as u32) << ((c.g[T3][0] as u32) & 31));
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        } else {
            // 0x002c7cc8: refill, then the same join.
            c.cycles += 10;
            decode_refill_bits(c);
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[S5][0] as u32) << ((c.g[T3][0] as u32) & 31));
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
    } else {
        // 0x002c7c24.
        c.cycles += 8;
        c.g[V0][0] = 0x30;
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[T3][0] as u32));
        c.g[V1][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x10));
        c.g[T0][0] = sx((c.g[A2][0] as u32) >> ((c.g[V0][0] as u32) & 31));
        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T3][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[V1][0] as u32) & 31));
        if (c.g[T2][0] as i64) >= 0 {
            // 0x002c7c6c.
            c.cycles += 4;
            c.g[T4][0] = sx((c.g[A2][0] as u32) >> 16);
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x10));
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 16);
            if (c.g[T2][0] as i64) < 0 {
                // 0x002c7c7c: refill, then the same join.
                c.cycles += 10;
                decode_refill_bits(c);
            }
        } else {
            // 0x002c7c44: refill, then the 0x002c7c6c join.
            c.cycles += 10;
            decode_refill_bits(c);
            c.cycles += 4;
            c.g[T4][0] = sx((c.g[A2][0] as u32) >> 16);
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x10));
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 16);
            if (c.g[T2][0] as i64) < 0 {
                c.cycles += 10;
                decode_refill_bits(c);
            }
        }
        // 0x002c7ca4.
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[T0][0] as u32) << 16);
        c.g[V1][0] = sx((c.g[S5][0] as u32) << ((c.g[T3][0] as u32) & 31));
        c.g[V0][0] = c.g[T4][0] | c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    }
    // 0x002c7cf8.
    c.cycles += 1;
    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    // 0x002c7cfc.
    c.cycles += 3;
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
    c.g[A1][0] = c.g[S4][0];
    c.g[V1][0] = sx((c.g[T8][0] as u32).wrapping_add(0x1));
}

/// Symbol-tail region (0x2c7dfc..0x2c7e08): the symbol-table init's exit
/// test: short classes loop back to the init, others join the main loop.
/// Entered at the top from the copy-restart paths.
fn decode_symbol_tail(c: &mut Cpu) -> TailNext {
    // 0x002c7dfc.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[A0][0] as i64)) as u64;
    let to_init = c.g[V0][0] == 0u64;
    c.g[T8][0] = c.g[A0][0];
    if !to_init {
        // 0x002c7e1c: leave the init with the default table window.
        c.cycles += 3;
        c.g[S2][0] = 0x20;
        c.g[T7][0] = 0x1;
        return Some(DecodeStage::Main);
    }
    // 0x002c7e08.
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = ((c.g[A0][0] as i64) < 9i64) as u64;
    c.g[V1][0] = c.lw(c.g[V1][0] as u32);
    if c.g[A0][0] != 0u64 {
        if back_edge_unwinds(c, decode::SYM_INIT) {
            return None;
        }
        return Some(DecodeStage::SymInit);
    }
    Some(DecodeStage::Main)
}

/// Main-step region (0x2c7f20..0x2c7f6c): after a symbol decode, either take
/// another symbol bit, or dispatch on the length class: the end marker
/// joins the match path, other classes search the length table. Entered at
/// the top from the main loop and the symbol chain.
fn decode_main_step(c: &mut Cpu) -> TailNext {
    // 0x002c7f20.
    c.cycles += 2;
    let need_bit = (c.g[T2][0] as i64) >= 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 24);
    if need_bit {
        if back_edge_unwinds(c, decode::SYMBOL) {
            return None;
        }
        return Some(DecodeStage::Symbol);
    }
    // 0x002c7f28.
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x10));
    c.g[V1][0] = 0x60;
    let end_marker = c.g[T3][0] == c.g[V1][0];
    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    if end_marker {
        // 0x002c7f6c.
        c.cycles += 2;
        c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_T8));
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_BASE));
        return decode_symbol_outcome(c);
    }
    // 0x002c7f38: look the class up in the length table.
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_TAB));
    c.g[A0][0] = sx((c.g[A2][0] as u32) >> 16);
    c.g[T3][0] = 0x8;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
    Some(DecodeStage::MatchLen)
}

/// Symbol-outcome region (0x2c7f74..0x2c8094): resolve the decoded class:
/// a literal byte is stored and the main loop resumes, the repeat class
/// reads its bit and dispatches to a length loop, and other classes fall
/// into the match-length tail. Entered at the top from the match-length
/// stage and the main step.
fn decode_symbol_outcome(c: &mut Cpu) -> TailNext {
    // 0x002c7f74.
    c.cycles += 11;
    c.g[V0][0] = sx((c.g[T3][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[T3][0] as u32));
    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[T3][0] as u32));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A2][0] as u32) >> ((c.g[A0][0] as u32) & 31));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[T0][0] = c.lbu(c.g[V1][0] as u32);
    let is_repeat = c.g[T0][0] == c.g[S6][0];
    c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[T3][0] as u32) & 31));
    if !is_repeat {
        // 0x002c7fa0: a literal byte; refill first when the buffer is empty.
        c.cycles += 2;
        if (c.g[T2][0] as i64) >= 0 {
            // 0x002c7fa8: store it.
            c.cycles += 2;
            c.sb(c.g[T6][0] as u32, c.g[T0][0]);
        } else {
            // Refill edge of 0x002c7fa0 (V1 first, no extra charge), the
            // 0x002c7fbc tail, and the 0x002c7fe0 store (T0 differs here).
            c.g[V1][0] = c.lbu(c.g[T1][0] as u32);
            c.cycles += 9;
            c.g[V0][0] = sx((c.g[A3][0] as u32) << 8);
            c.g[A0][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = sx(0u32.wrapping_sub(c.g[T2][0] as u32));
            c.g[A3][0] = c.g[V1][0] | c.g[V0][0];
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2));
            c.g[V0][0] = sx((c.g[A3][0] as u32) << 8);
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x10));
            c.g[A3][0] = c.g[A0][0] | c.g[V0][0];
            c.g[A2][0] = sx((c.g[A3][0] as u32) << ((c.g[A1][0] as u32) & 31));
            c.cycles += 2;
            c.sb(c.g[T6][0] as u32, c.g[T0][0]);
        }
        // 0x002c8274: count the byte and rejoin the main loop.
        c.cycles += 2;
        c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
        if back_edge_unwinds(c, decode::MAIN) {
            return None;
        }
        return Some(DecodeStage::Main);
    }
    // Repeat class: 0x002c7fb0, refilling first when the buffer is empty
    // (0x002c7fb8 takes the V1 load, 0x002c7fbc the tail below).
    c.cycles += 2;
    if (c.g[T2][0] as i64) < 0 {
        c.cycles += 1;
        c.g[V1][0] = c.lbu(c.g[T1][0] as u32);
        c.cycles += 9;
        c.g[V0][0] = sx((c.g[A3][0] as u32) << 8);
        c.g[A0][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x1));
        c.g[A1][0] = sx(0u32.wrapping_sub(c.g[T2][0] as u32));
        c.g[A3][0] = c.g[V1][0] | c.g[V0][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2));
        c.g[V0][0] = sx((c.g[A3][0] as u32) << 8);
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x10));
        c.g[A3][0] = c.g[A0][0] | c.g[V0][0];
        c.g[A2][0] = sx((c.g[A3][0] as u32) << ((c.g[A1][0] as u32) & 31));
    }
    // 0x002c7fe0: T0 still equals the repeat symbol here, so no store.
    c.cycles += 2;
    // 0x002c7fe8: repeat class; T4 marks the output point for the run.
    c.cycles += 2;
    c.g[T4][0] = c.g[T6][0];
    if (c.g[A2][0] as i64) >= 0 {
        // 0x002c8030.
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 16);
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
        if c.g[V0][0] == 0u64 {
            // 0x002c8094.
            c.cycles += 3;
            c.g[T0][0] = 0x2;
            c.g[T3][0] = sx((c.g[A2][0] as u32) >> 31);
            return Some(DecodeStage::LenBits2);
        }
        // 0x002c803c.
        c.cycles += 1;
        c.g[T0][0] = 0x2;
        return Some(DecodeStage::LenBits);
    }
    // 0x002c7ff0.
    c.cycles += 4;
    c.g[T3][0] = sx((c.g[A2][0] as u32) >> 29);
    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x3));
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
    if (c.g[T2][0] as i64) >= 0 {
        // 0x002c8028: drop the low bits, then the match-length tail.
        c.cycles += 2;
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x4));
    } else {
        // 0x002c8000: refill, then the same join.
        c.cycles += 10;
        decode_refill_bits(c);
        c.cycles += 2;
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x4));
    }
    decode_length_tail(c)
}

/// Length-value region (0x2c80e0..0x2c81c0): decode the match-length class
/// (`t0`) into a byte count (`t3`), by direct shift or by refill-assisted
/// bit tests. Entered at the top from the length loops; falls through to
/// the match-length tail.
fn decode_length_value(c: &mut Cpu) -> TailNext {
    // 0x002c80e0.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[T0][0] as i64) < 17i64) as u64;
    let short = c.g[V0][0] != 0u64;
    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[T0][0] as u32));
    if short {
        // 0x002c8180.
        c.cycles += 4;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[T0][0] as u32));
        c.g[T3][0] = sx((c.g[A2][0] as u32) >> ((c.g[V0][0] as u32) & 31));
        c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[T0][0] as u32) & 31));
        if (c.g[T2][0] as i64) >= 0 {
            // 0x002c81b8.
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[T7][0] as u32) << ((c.g[T0][0] as u32) & 31));
            c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        } else {
            // 0x002c8190: refill, then the same join.
            c.cycles += 10;
            decode_refill_bits(c);
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[T7][0] as u32) << ((c.g[T0][0] as u32) & 31));
            c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
    } else {
        // 0x002c80ec.
        c.cycles += 8;
        c.g[V0][0] = 0x30;
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x10));
        c.g[T3][0] = sx((c.g[A2][0] as u32) >> ((c.g[V0][0] as u32) & 31));
        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[V1][0] as u32) & 31));
        if (c.g[T2][0] as i64) >= 0 {
            // 0x002c8134.
            c.cycles += 4;
            c.g[T5][0] = sx((c.g[A2][0] as u32) >> 16);
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x10));
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 16);
            if (c.g[T2][0] as i64) < 0 {
                // 0x002c8144: refill, then the same join.
                c.cycles += 10;
                decode_refill_bits(c);
            }
        } else {
            // 0x002c810c: refill, then the 0x002c8134 join.
            c.cycles += 10;
            decode_refill_bits(c);
            c.cycles += 4;
            c.g[T5][0] = sx((c.g[A2][0] as u32) >> 16);
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x10));
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 16);
            if (c.g[T2][0] as i64) < 0 {
                c.cycles += 10;
                decode_refill_bits(c);
            }
        }
        // 0x002c816c.
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[T3][0] as u32) << 16);
        c.g[V1][0] = sx((c.g[T7][0] as u32) << ((c.g[T0][0] as u32) & 31));
        c.g[V0][0] = c.g[T5][0] | c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    }
    // 0x002c81c0.
    c.cycles += 1;
    c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    decode_length_tail(c)
}

/// Match-length tail (0x2c81c4..0x2c8318): turn the class (`t3`) into a
/// match: a zero class reads one more bit, otherwise the previous byte is
/// repeated; the end marker selects the checksum or delta post-pass, and
/// anything else rejoins the main loop. Entered at the top from the
/// length-value region and the symbol outcome.
fn decode_length_tail(c: &mut Cpu) -> TailNext {
    // 0x002c81c4.
    c.cycles += 2;
    let no_match = c.g[T3][0] == 0u64;
    c.g[V1][0] = sx((c.g[T6][0] as u32).wrapping_add(c.g[T3][0] as u32));
    if !no_match {
        // 0x002c81cc: repeat the previous byte over the run.
        c.cycles += 3;
        c.g[T0][0] = c.lbu((c.g[T6][0] as u32).wrapping_sub(0x1));
        c.sb(c.g[T4][0] as u32, c.g[T0][0]);
        return Some(DecodeStage::RunFill);
    }
    // 0x002c81fc.
    c.cycles += 4;
    c.g[T0][0] = sx((c.g[A2][0] as u32) >> 31);
    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
    if (c.g[T2][0] as i64) >= 0 {
        // 0x002c8234.
        c.cycles += 2;
    } else {
        // 0x002c820c: refill, then the same join.
        c.cycles += 10;
        decode_refill_bits(c);
        c.cycles += 2;
    }
    let end_stream = c.g[T0][0] != 0u64;
    c.g[T0][0] = sx((c.g[A2][0] as u32) >> 24);
    if end_stream {
        // 0x002c827c: the magic selects the post-pass.
        c.cycles += 3;
        c.g[V0][0] = 0x32fb;
        let not_raw = c.g[S7][0] != c.g[V0][0];
        c.g[V0][0] = 0x34fb;
        if not_raw {
            // 0x002c82cc.
            c.cycles += 2;
            let not_delta = c.g[S7][0] != c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_LEN));
            if not_delta {
                decode_epilogue(c);
                return None;
            }
            // 0x002c82d4: bounds-check the delta accumulation.
            c.cycles += 7;
            c.g[T6][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_OUT));
            c.g[V1][0] = 0u64;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_LEN));
            c.g[A1][0] = sx((c.g[T6][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = (c.g[T6][0] < c.g[A1][0]) as u64;
            c.g[A0][0] = 0u64;
            if c.g[V0][0] == 0u64 {
                // 0x002c8314: report the decoded length.
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_LEN));
                decode_epilogue(c);
                return None;
            }
            // 0x002c82f0.
            c.cycles += 2;
            c.g[V0][0] = c.lbu(c.g[T6][0] as u32);
            return Some(DecodeStage::DeltaAcc);
        }
        // 0x002c8288: bounds-check the checksum accumulation.
        c.cycles += 6;
        c.g[T6][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_OUT));
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_LEN));
        c.g[A0][0] = sx((c.g[T6][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = (c.g[T6][0] < c.g[A0][0]) as u64;
        c.g[V1][0] = 0u64;
        if c.g[V0][0] == 0u64 {
            // 0x002c8314: report the decoded length.
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_LEN));
            decode_epilogue(c);
            return None;
        }
        // 0x002c82a0.
        c.cycles += 2;
        c.g[V0][0] = c.lbu(c.g[T6][0] as u32);
        return Some(DecodeStage::Checksum);
    }
    // 0x002c823c.
    c.cycles += 3;
    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x8));
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 8);
    if (c.g[T2][0] as i64) >= 0 {
        // 0x002c8270: store the byte and rejoin the main loop.
        c.cycles += 1;
    } else {
        // 0x002c8248: refill, then the same join.
        c.cycles += 10;
        decode_refill_bits(c);
        c.cycles += 1;
    }
    c.sb(c.g[T6][0] as u32, c.g[T0][0]);
    // 0x002c8274: count the byte and rejoin the main loop.
    c.cycles += 2;
    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
    if back_edge_unwinds(c, decode::MAIN) {
        return None;
    }
    Some(DecodeStage::Main)
}

/// Epilogue (0x2c8318): restore the callee-saved registers, drop the frame
/// and return through the live return address, exactly as the generated
/// body does (`ra` is never spilled: it returns `jr ra` with `sp + 0x570`).
/// Callers landing at 0x2c8314 load the decoded length first.
fn decode_epilogue(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame { size: DECODE_FRAME };
    frame.restore(c, S0, 0x560);
    frame.restore(c, S1, 0x550);
    frame.restore(c, S2, 0x540);
    frame.restore(c, S3, 0x530);
    frame.restore(c, S4, 0x520);
    frame.restore(c, S5, 0x510);
    frame.restore(c, S6, 0x500);
    frame.restore(c, S7, 0x4f0);
    frame.restore(c, FP, 0x4e0);
    frame.close(c);
}

/// `int HUFF_DecodeStream(src, out)` (0x2c76c0): decode one Huffman block
/// from the stream at `src`, writing the output bytes at `out`; returns the
/// decoded length (`[sp + 0x4c4]`). A null `src` returns 0 at once. The bit
/// buffer (`a2`/`t2`) refills from the stream as the table builders and the
/// main symbol loop consume it; the magic (`s7`) selects the raw, checksum
/// or delta post-pass.
pub fn huff_decode_stream(c: &mut Cpu) {
    let mut st = match c.pc {
        decode::ENTRY => DecodeStage::Entry,
        decode::TBL_BUILD => DecodeStage::TblBuild,
        decode::TBL_STEP => DecodeStage::TblStep,
        decode::BIT_LOOP_B => DecodeStage::BitLoopB,
        decode::BIT_LOOP_A => DecodeStage::BitLoopA,
        decode::CLEAR_TBL => DecodeStage::ClearTbl,
        decode::TBL2 => DecodeStage::Tbl2,
        decode::BIT_LOOP_C => DecodeStage::BitLoopC,
        decode::BIT_LOOP_D => DecodeStage::BitLoopD,
        decode::SCAN_SYM => DecodeStage::ScanSym,
        decode::OUT_FILL => DecodeStage::OutFill,
        decode::COPY_PAIR => DecodeStage::CopyPair,
        decode::COPY_RESTART => DecodeStage::CopyRestart,
        decode::SYM_INIT => DecodeStage::SymInit,
        decode::MAIN => DecodeStage::Main,
        decode::SYMBOLS => DecodeStage::Symbols,
        decode::SYMBOL => DecodeStage::Symbol,
        decode::MATCH_LEN => DecodeStage::MatchLen,
        decode::LEN_BITS => DecodeStage::LenBits,
        decode::LEN_BITS2 => DecodeStage::LenBits2,
        decode::RUN_FILL => DecodeStage::RunFill,
        decode::CHECKSUM => DecodeStage::Checksum,
        decode::DELTA_ACC => DecodeStage::DeltaAcc,
        pc => {
            c.jump_out(pc, decode::ENTRY, decode::END);
            return;
        }
    };
    loop {
        st = match st {
            DecodeStage::Entry => {
                // 0x002c76c0: frame and the null-source exit.
                c.cycles += 16;
                let frame = Frame::open(c, DECODE_FRAME);
                c.g[T1][0] = c.g[A0][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(DS_OUT), c.g[A1][0]);
                frame.save(c, S0, 0x560);
                frame.save(c, S1, 0x550);
                c.g[T6][0] = c.g[A1][0];
                frame.save(c, S2, 0x540);
                frame.save(c, S3, 0x530);
                frame.save(c, S4, 0x520);
                frame.save(c, S5, 0x510);
                frame.save(c, S6, 0x500);
                frame.save(c, S7, 0x4f0);
                frame.save(c, FP, 0x4e0);
                c.sw((c.g[SP][0] as u32).wrapping_add(DS_T8), 0u64);
                let null_src = c.g[T1][0] == 0u64;
                c.sw((c.g[SP][0] as u32).wrapping_add(DS_LEN), 0u64);
                if null_src {
                    // 0x002c8314: report the decoded length.
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_LEN));
                    decode_epilogue(c);
                    return;
                }
                // 0x002c7700: read the stream magic and seed the buffer.
                c.cycles += 17;
                c.g[A3][0] = c.lbu(c.g[T1][0] as u32);
                c.g[T2][0] = 0u64;
                c.g[V1][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 8);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2));
                c.g[A3][0] = c.g[V1][0] | c.g[V0][0];
                c.g[A0][0] = c.lbu(c.g[T1][0] as u32);
                c.g[S7][0] = c.g[A3][0];
                c.g[A1][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[S7][0] as u32) << 8);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2));
                c.g[A3][0] = c.g[A0][0] | c.g[V0][0];
                c.g[V1][0] = c.g[S7][0] & 0x100;
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 8);
                c.g[A3][0] = c.g[A1][0] | c.g[V0][0];
                let short_hdr = c.g[V1][0] == 0u64;
                c.g[A2][0] = sx((c.g[A3][0] as u32) << 16);
                if !short_hdr {
                    // 0x002c7744: long header, four more stream bytes.
                    c.cycles += 16;
                    c.g[V1][0] = c.lbu(c.g[T1][0] as u32);
                    c.g[V0][0] = sx((c.g[A3][0] as u32) << 8);
                    c.g[A2][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x1));
                    c.g[T2][0] = 0x8;
                    c.g[A3][0] = c.g[V1][0] | c.g[V0][0];
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = sx((c.g[A3][0] as u32) << 8);
                    c.g[A0][0] = c.lbu(c.g[T1][0] as u32);
                    c.g[A3][0] = c.g[A2][0] | c.g[V0][0];
                    c.g[A1][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[A3][0] as u32) << 8);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2));
                    c.g[A3][0] = c.g[A0][0] | c.g[V1][0];
                    c.g[V0][0] = sx((c.g[A3][0] as u32) << 8);
                    c.g[A3][0] = c.g[A1][0] | c.g[V0][0];
                    c.g[A2][0] = sx((c.g[A3][0] as u32) << 8);
                }
                // 0x002c7784: fold the top byte out, masking the magic.
                c.cycles += 7;
                c.g[V0][0] = 0xffff_ffff_ffff_0000;
                c.g[T0][0] = sx((c.g[A2][0] as u32) >> 24);
                c.g[V0][0] = c.g[V0][0] | 0xfeff;
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 8);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x8));
                c.g[S7][0] = c.g[S7][0] & c.g[V0][0];
                if (c.g[T2][0] as i64) < 0 {
                    // 0x002c77a0: refill, then the same join.
                    c.cycles += 10;
                    decode_refill_bits(c);
                }
                // 0x002c77c8.
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 16);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x10));
                c.sw((c.g[SP][0] as u32).wrapping_add(DS_LEN), c.g[V0][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 16);
                if (c.g[T2][0] as i64) < 0 {
                    // 0x002c77dc: refill, then the same join.
                    c.cycles += 10;
                    decode_refill_bits(c);
                }
                // 0x002c7804.
                c.cycles += 8;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_LEN));
                c.g[V0][0] = sx((c.g[T0][0] as u32) << 16);
                c.g[T0][0] = sx((c.g[A2][0] as u32) >> 24);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x8));
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 8);
                c.sw((c.g[SP][0] as u32).wrapping_add(DS_LEN), c.g[V1][0]);
                if (c.g[T2][0] as i64) < 0 {
                    // 0x002c7824: refill, then the same join.
                    c.cycles += 10;
                    decode_refill_bits(c);
                }
                // 0x002c784c: point the four tables at the frame.
                c.cycles += 13;
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.sw((c.g[SP][0] as u32).wrapping_add(DS_BASE), c.g[A0][0]);
                c.g[S6][0] = c.g[T0][0] & 0xff;
                c.sw((c.g[SP][0] as u32).wrapping_add(DS_TAB), c.g[V0][0]);
                c.g[S2][0] = 0u64;
                c.g[T3][0] = 0x1;
                c.g[T8][0] = 0u64;
                c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3c0));
                c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2c0));
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.g[S3][0] = 0x1;
                DecodeStage::TblBuild
            }
            DecodeStage::TblBuild => {
                // 0x002c7880: next level's slot.
                c.cycles += 1;
                c.g[V1][0] = sx((c.g[T3][0] as u32) << 2);
                DecodeStage::TblStep
            }
            DecodeStage::TblStep => {
                // 0x002c7884: file the bit-run length for this level.
                c.cycles += 7;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_BASE));
                c.g[T8][0] = sx((c.g[T8][0] as u32) << 1);
                c.g[T9][0] = c.g[V1][0];
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[T8][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                if (c.g[A2][0] as i64) >= 0 {
                    // 0x002c78e4.
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 16);
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                    if c.g[V0][0] == 0u64 {
                        // 0x002c794c: second code-length bit loop.
                        c.cycles += 3;
                        c.g[T4][0] = 0x2;
                        c.g[T7][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                        c.g[T0][0] = sx((c.g[A2][0] as u32) >> 31);
                        DecodeStage::BitLoopA
                    } else {
                        // 0x002c78f0: first code-length bit loop.
                        c.cycles += 2;
                        c.g[T4][0] = 0x2;
                        c.g[T7][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                        DecodeStage::BitLoopB
                    }
                } else {
                    // 0x002c78a0.
                    c.cycles += 4;
                    c.g[T0][0] = sx((c.g[A2][0] as u32) >> 29);
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x3));
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
                    if (c.g[T2][0] as i64) < 0 {
                        // 0x002c78b0: refill, then the same join.
                        c.cycles += 10;
                        decode_refill_bits(c);
                    }
                    // 0x002c78d8: store the level entry directly.
                    c.cycles += 3;
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4));
                    c.g[T7][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                    match decode_table_store(c) {
                        Some(s) => s,
                        None => return,
                    }
                }
            }
            DecodeStage::BitLoopB => {
                loop {
                    // 0x002c78f8.
                    c.cycles += 7;
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
                    let more_bits = (c.g[A2][0] as i64) >= 0;
                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                    if !more_bits {
                        break;
                    }
                    if back_edge_unwinds(c, decode::BIT_LOOP_B) {
                        return;
                    }
                }
                // 0x002c7914.
                c.cycles += 3;
                c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T4][0] as u32));
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
                if (c.g[T2][0] as i64) < 0 {
                    // 0x002c7920: refill, then the run-cost region.
                    c.cycles += 11;
                    decode_refill_bits(c);
                }
                match decode_run_cost(c) {
                    Some(s) => s,
                    None => return,
                }
            }
            DecodeStage::BitLoopA => loop {
                // 0x002c7958.
                c.cycles += 4;
                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
                if (c.g[T2][0] as i64) < 0 {
                    // 0x002c7968: refill, then the same join.
                    c.cycles += 10;
                    decode_refill_bits(c);
                }
                // 0x002c7990.
                c.cycles += 2;
                let stay = c.g[T0][0] == 0u64;
                c.g[T0][0] = sx((c.g[A2][0] as u32) >> 31);
                if stay {
                    if back_edge_unwinds(c, decode::BIT_LOOP_A) {
                        return;
                    }
                    continue;
                }
                break match decode_run_cost(c) {
                    Some(s) => s,
                    None => return,
                };
            },
            DecodeStage::ClearTbl => {
                loop {
                    // 0x002c7ae8: clear one scratch row.
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                    let more = c.g[A0][0] != 0u64;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                    if !more {
                        break;
                    }
                    if back_edge_unwinds(c, decode::CLEAR_TBL) {
                        return;
                    }
                }
                // 0x002c7b04: skip the second pass when the table is empty.
                c.cycles += 4;
                c.g[T8][0] = 0xff;
                c.g[T5][0] = 0u64;
                let empty = (c.g[S2][0] as i64) <= 0;
                c.g[T9][0] = ((c.g[S3][0] as i64) < 1i64) as u64;
                if empty {
                    // 0x002c7d44: fill the output table with the gap marker.
                    c.cycles += 5;
                    c.g[V1][0] = 0x4040_0000;
                    c.g[A0][0] = 0x10;
                    c.g[V1][0] = c.g[V1][0] | 0x4040;
                    c.g[V0][0] = c.g[S0][0];
                    DecodeStage::OutFill
                } else {
                    // 0x002c7b14.
                    c.cycles += 1;
                    c.g[S5][0] = 0x1;
                    DecodeStage::Tbl2
                }
            }
            DecodeStage::Tbl2 => {
                // 0x002c7b18: second table-build pass over the levels.
                c.cycles += 2;
                let level_bit = (c.g[A2][0] as i64) >= 0;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 16);
                if level_bit {
                    // 0x002c7b64.
                    c.cycles += 2;
                    let done = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                    if done {
                        // 0x002c7bcc.
                        c.cycles += 3;
                        c.g[T3][0] = 0x2;
                        c.g[T7][0] = sx((c.g[T5][0] as u32).wrapping_add(0x1));
                        c.g[T0][0] = sx((c.g[A2][0] as u32) >> 31);
                        DecodeStage::BitLoopD
                    } else {
                        // 0x002c7b6c: third code-length bit loop.
                        c.cycles += 3;
                        c.g[T3][0] = 0x2;
                        c.g[T7][0] = sx((c.g[T5][0] as u32).wrapping_add(0x1));
                        DecodeStage::BitLoopC
                    }
                } else {
                    // 0x002c7b20.
                    c.cycles += 4;
                    c.g[T0][0] = sx((c.g[A2][0] as u32) >> 29);
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x3));
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
                    if (c.g[T2][0] as i64) < 0 {
                        // 0x002c7b30: refill, then the same join.
                        c.cycles += 10;
                        decode_refill_bits(c);
                    }
                    // 0x002c7b58.
                    c.cycles += 3;
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4));
                    c.g[T7][0] = sx((c.g[T5][0] as u32).wrapping_add(0x1));
                    // 0x002c7cfc.
                    c.cycles += 3;
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[V1][0] = sx((c.g[T8][0] as u32).wrapping_add(0x1));
                    DecodeStage::ScanSym
                }
            }
            DecodeStage::BitLoopC => {
                loop {
                    // 0x002c7b78.
                    c.cycles += 7;
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
                    let more_bits = (c.g[A2][0] as i64) >= 0;
                    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                    if !more_bits {
                        break;
                    }
                    if back_edge_unwinds(c, decode::BIT_LOOP_C) {
                        return;
                    }
                }
                // 0x002c7b94.
                c.cycles += 3;
                c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T3][0] as u32));
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
                if (c.g[T2][0] as i64) < 0 {
                    // 0x002c7ba0: refill, then the symbol-cost region.
                    c.cycles += 11;
                    decode_refill_bits(c);
                }
                decode_symbol_cost(c);
                DecodeStage::ScanSym
            }
            DecodeStage::BitLoopD => loop {
                // 0x002c7bd8.
                c.cycles += 4;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
                if (c.g[T2][0] as i64) < 0 {
                    // 0x002c7be8: refill, then the same join.
                    c.cycles += 10;
                    decode_refill_bits(c);
                }
                // 0x002c7c10.
                c.cycles += 2;
                let stay = c.g[T0][0] == 0u64;
                c.g[T0][0] = sx((c.g[A2][0] as u32) >> 31);
                if stay {
                    if back_edge_unwinds(c, decode::BIT_LOOP_D) {
                        return;
                    }
                    continue;
                }
                break {
                    decode_symbol_cost(c);
                    DecodeStage::ScanSym
                };
            },
            DecodeStage::ScanSym => loop {
                // 0x002c7d08: scan the level table for the next symbol.
                c.cycles += 7;
                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                c.g[T8][0] = c.g[V1][0] & 0xff;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T8][0] as u32));
                c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                if c.g[V1][0] == 0 {
                    c.g[T0][0] = c.g[A0][0];
                }
                let more = c.g[T0][0] != 0u64;
                c.g[V1][0] = sx((c.g[T8][0] as u32).wrapping_add(0x1));
                if more {
                    if back_edge_unwinds(c, decode::SCAN_SYM) {
                        return;
                    }
                    continue;
                }
                // 0x002c7d24: file the symbol, or loop the table pass.
                c.cycles += 8;
                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[T5][0] as u32));
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[T8][0] as u32));
                c.g[V0][0] = 0x1;
                c.g[T5][0] = c.g[T7][0];
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = ((c.g[T5][0] as i64) < (c.g[S2][0] as i64)) as u64;
                c.sb(c.g[A1][0] as u32, c.g[T8][0]);
                if c.g[V1][0] != 0u64 {
                    if back_edge_unwinds(c, decode::TBL2) {
                        return;
                    }
                    break DecodeStage::Tbl2;
                }
                // 0x002c7d44: fill the output table with the gap marker.
                c.cycles += 5;
                c.g[V1][0] = 0x4040_0000;
                c.g[A0][0] = 0x10;
                c.g[V1][0] = c.g[V1][0] | 0x4040;
                c.g[V0][0] = c.g[S0][0];
                break DecodeStage::OutFill;
            },
            DecodeStage::OutFill => {
                loop {
                    // 0x002c7d58.
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    let more = c.g[A0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                    if !more {
                        break;
                    }
                    if back_edge_unwinds(c, decode::OUT_FILL) {
                        return;
                    }
                }
                // 0x002c7d74.
                c.cycles += 5;
                c.g[T7][0] = c.g[FP][0];
                c.g[T5][0] = c.g[S1][0];
                c.g[T4][0] = c.g[S0][0];
                let blocked = c.g[T9][0] != 0u64;
                c.g[T8][0] = 0x1;
                if blocked {
                    DecodeStage::Main
                } else {
                    // 0x002c7d88: prime the symbol-table init.
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_LVL1));
                    c.g[S5][0] = 0x8;
                    c.g[S4][0] = 0x1;
                    c.g[S2][0] = 0xffff_ffff_ffff_ffff;
                    DecodeStage::SymInit
                }
            }
            DecodeStage::SymInit => {
                // 0x002c7d98: count the live entries of each class.
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[T8][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.g[T3][0] = sx((c.g[S4][0] as u32) << ((c.g[V0][0] as u32) & 31));
                let to_tail = c.g[V1][0] == c.g[S2][0];
                c.g[A0][0] = sx((c.g[T8][0] as u32).wrapping_add(0x1));
                if to_tail {
                    match decode_symbol_tail(c) {
                        Some(s) => s,
                        None => return,
                    }
                } else {
                    // 0x002c7dac: restart the copy test below the end.
                    c.cycles += 3;
                    c.g[T9][0] = 0xffff_ffff_ffff_ffff;
                    c.g[T0][0] = c.lbu(c.g[T7][0] as u32);
                    DecodeStage::CopyRestart
                }
            }
            DecodeStage::CopyRestart => loop {
                // 0x002c7db8: test the next byte against the repeat mark.
                c.cycles += 3;
                c.g[A1][0] = c.g[T8][0];
                let mismatch = c.g[T0][0] != c.g[S6][0];
                c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x1));
                if !mismatch {
                    // 0x002c7dc4: remember the run length.
                    c.cycles += 2;
                    c.sw((c.g[SP][0] as u32).wrapping_add(DS_T8), c.g[T8][0]);
                    c.g[A1][0] = 0x60;
                }
                // 0x002c7dcc.
                c.cycles += 2;
                let short = (c.g[T3][0] as i64) <= 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                if !short {
                    // 0x002c7dd4: a live run heads for the copy loop.
                    c.cycles += 1;
                    c.g[V0][0] = c.g[T3][0];
                    break DecodeStage::CopyPair;
                }
                // 0x002c7df4 (shared with the copy loop below).
                c.cycles += 2;
                if c.g[V1][0] != c.g[T9][0] {
                    c.g[T0][0] = c.lbu(c.g[T7][0] as u32);
                    if back_edge_unwinds(c, decode::COPY_RESTART) {
                        return;
                    }
                    continue;
                }
                break match decode_symbol_tail(c) {
                    Some(s) => s,
                    None => return,
                };
            },
            DecodeStage::CopyPair => {
                loop {
                    // 0x002c7dd8: copy one byte of the run pair.
                    c.cycles += 7;
                    c.sb(c.g[T5][0] as u32, c.g[T0][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.sb(c.g[T4][0] as u32, c.g[A1][0]);
                    c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x1));
                    let more = c.g[V0][0] != 0u64;
                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                    if !more {
                        break;
                    }
                    if back_edge_unwinds(c, decode::COPY_PAIR) {
                        return;
                    }
                }
                // 0x002c7df4 (shared with the restart test above).
                c.cycles += 2;
                if c.g[V1][0] != c.g[T9][0] {
                    c.g[T0][0] = c.lbu(c.g[T7][0] as u32);
                    if back_edge_unwinds(c, decode::COPY_RESTART) {
                        return;
                    }
                    DecodeStage::CopyRestart
                } else {
                    match decode_symbol_tail(c) {
                        Some(s) => s,
                        None => return,
                    }
                }
            }
            DecodeStage::Main => {
                // 0x002c7e60: index the output table with the top bits.
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 24);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[T3][0] = c.lbu(c.g[V0][0] as u32);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[T3][0] as u32));
                if (c.g[T2][0] as i64) < 0 {
                    // By way of 0x002c7f20: fold the top bits back in, then
                    // take the main-step region (which charges its cycles).
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x10));
                    match decode_main_step(c) {
                        Some(s) => s,
                        None => return,
                    }
                } else {
                    DecodeStage::Symbols
                }
            }
            DecodeStage::Symbols => {
                loop {
                    // 0x002c7e78: decode one symbol of the chain.
                    c.cycles += 11;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 24);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[T3][0] as u32) & 31));
                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                    c.g[A0][0] = sx((c.g[A2][0] as u32) >> 24);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sb(c.g[T6][0] as u32, c.g[V1][0]);
                    c.g[T3][0] = c.lbu(c.g[V0][0] as u32);
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[T3][0] as u32));
                    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
                    if (c.g[T2][0] as i64) < 0 {
                        break;
                    }
                    // 0x002c7ea4: decode the next symbol of the chain.
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[T3][0] as u32) & 31));
                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                    c.g[A0][0] = sx((c.g[A2][0] as u32) >> 24);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sb(c.g[T6][0] as u32, c.g[V1][0]);
                    c.g[T3][0] = c.lbu(c.g[V0][0] as u32);
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[T3][0] as u32));
                    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
                    if (c.g[T2][0] as i64) < 0 {
                        break;
                    }
                    // 0x002c7ecc: decode the next symbol of the chain.
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[T3][0] as u32) & 31));
                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                    c.g[A0][0] = sx((c.g[A2][0] as u32) >> 24);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sb(c.g[T6][0] as u32, c.g[V1][0]);
                    c.g[T3][0] = c.lbu(c.g[V0][0] as u32);
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[T3][0] as u32));
                    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
                    if (c.g[T2][0] as i64) < 0 {
                        break;
                    }
                    // 0x002c7ef4: decode the last symbol of the chain.
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[T3][0] as u32) & 31));
                    c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                    c.g[V1][0] = sx((c.g[A2][0] as u32) >> 24);
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sb(c.g[T6][0] as u32, c.g[A0][0]);
                    c.g[T3][0] = c.lbu(c.g[V1][0] as u32);
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[T3][0] as u32));
                    let chain_more = (c.g[T2][0] as i64) >= 0;
                    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
                    if chain_more {
                        if back_edge_unwinds(c, decode::SYMBOLS) {
                            return;
                        }
                        continue;
                    }
                    break;
                }
                // 0x002c7f1c.
                c.cycles += 1;
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x10));
                match decode_main_step(c) {
                    Some(s) => s,
                    None => return,
                }
            }
            DecodeStage::Symbol => {
                // 0x002c7e28: store one symbol byte, then refill and rejoin.
                c.cycles += 14;
                c.g[A1][0] = sx((c.g[A3][0] as u32) << 8);
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = 0x10;
                c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T2][0] as u32));
                c.sb(c.g[T6][0] as u32, c.g[V1][0]);
                c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lbu(c.g[T1][0] as u32);
                c.g[V1][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[A3][0] = c.g[V0][0] | c.g[A1][0];
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 8);
                c.g[A3][0] = c.g[V1][0] | c.g[V0][0];
                c.g[A2][0] = sx((c.g[A3][0] as u32) << ((c.g[A0][0] as u32) & 31));
                DecodeStage::Main
            }
            DecodeStage::MatchLen => {
                loop {
                    // 0x002c7f48: search the length table for the class.
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                    let found = c.g[V0][0] == 0u64;
                    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                    if !found {
                        break;
                    }
                    if back_edge_unwinds(c, decode::MATCH_LEN) {
                        return;
                    }
                }
                // 0x002c7f64.
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_BASE));
                match decode_symbol_outcome(c) {
                    Some(s) => s,
                    None => return,
                }
            }
            DecodeStage::LenBits => {
                loop {
                    // 0x002c8040.
                    c.cycles += 7;
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
                    let more_bits = (c.g[A2][0] as i64) >= 0;
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    if !more_bits {
                        break;
                    }
                    if back_edge_unwinds(c, decode::LEN_BITS) {
                        return;
                    }
                }
                // 0x002c805c.
                c.cycles += 3;
                c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
                if (c.g[T2][0] as i64) < 0 {
                    // 0x002c8068: refill, then the length-value region.
                    c.cycles += 11;
                    decode_refill_bits(c);
                }
                match decode_length_value(c) {
                    Some(s) => s,
                    None => return,
                }
            }
            DecodeStage::LenBits2 => loop {
                // 0x002c80a0.
                c.cycles += 4;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
                if (c.g[T2][0] as i64) < 0 {
                    // 0x002c80b0: refill, then the same join.
                    c.cycles += 10;
                    decode_refill_bits(c);
                }
                // 0x002c80d8.
                c.cycles += 2;
                let stay = c.g[T3][0] == 0u64;
                c.g[T3][0] = sx((c.g[A2][0] as u32) >> 31);
                if stay {
                    if back_edge_unwinds(c, decode::LEN_BITS2) {
                        return;
                    }
                    continue;
                }
                break match decode_length_value(c) {
                    Some(s) => s,
                    None => return,
                };
            },
            DecodeStage::RunFill => {
                loop {
                    // 0x002c81d8: repeat the previous byte over the run.
                    c.cycles += 7;
                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                    let more = c.g[T4][0] < c.g[V1][0];
                    if !more {
                        break;
                    }
                    c.sb(c.g[T4][0] as u32, c.g[T0][0]);
                    if back_edge_unwinds(c, decode::RUN_FILL) {
                        return;
                    }
                }
                // 0x002c81f4: the run reached the output point.
                c.cycles += 2;
                c.g[T6][0] = c.g[T4][0];
                if back_edge_unwinds(c, decode::MAIN) {
                    return;
                }
                DecodeStage::Main
            }
            DecodeStage::Checksum => {
                loop {
                    // 0x002c82a8: accumulate the checksum over the output.
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sb(c.g[T6][0] as u32, c.g[V1][0]);
                    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
                    let more = c.g[T6][0] < c.g[A0][0];
                    if !more {
                        break;
                    }
                    c.g[V0][0] = c.lbu(c.g[T6][0] as u32);
                    if back_edge_unwinds(c, decode::CHECKSUM) {
                        return;
                    }
                }
                // 0x002c82c4: the length is already in `v0`; return it.
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_LEN));
                decode_epilogue(c);
                return;
            }
            DecodeStage::DeltaAcc => {
                loop {
                    // 0x002c82f8: accumulate the deltas over the output.
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sb(c.g[T6][0] as u32, c.g[A0][0]);
                    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
                    let more = c.g[T6][0] < c.g[A1][0];
                    if !more {
                        break;
                    }
                    c.g[V0][0] = c.lbu(c.g[T6][0] as u32);
                    if back_edge_unwinds(c, decode::DELTA_ACC) {
                        return;
                    }
                }
                // 0x002c8314: report the decoded length.
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(DS_LEN));
                decode_epilogue(c);
                return;
            }
        };
    }
}

/// `void HUFF_analysis(state, flags, size)` (0x2c8e48): histogram the
/// pending input bytes, pick the pack mode (raw, repeat, multi-level) and
/// the delta window, then size the code tables with `HUFF_maketree` and
/// `HUFF_minrep`. Event checks never unwind (their result is ignored), so
/// the body is straight-line structured code with three calls.
pub fn huff_analysis(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: frame, scratch pointers and the zeroed length area.
        c.cycles += 20;
        let frame = Frame::open(c, ANALYSIS_FRAME);
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x8a0));
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x870));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x8b0));
        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x100));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x840));
        c.g[S0][0] = 0u64;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x890));
        c.g[S7][0] = c.g[A2][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x880));
        c.g[V1][0] = c.g[S4][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x860));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x850));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x830));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x820), c.g[RA][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(AN_SP_LO), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(AN_SP_SCRATCH), c.g[V0][0]);
        // 0x002c8e98: clear the 0x300 length words at the scratch base.
        'clear_lens: loop {
            c.cycles += 7;
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = (c.g[S0][0] < 0x300u64) as u64;
            let more = c.g[V0][0] != 0u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            if !more {
                break 'clear_lens;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_8e98);
            }
        }
        // Histogram over the input range: per-byte counts, run lengths and
        // the byte sum. Skipped when the range is empty.
        c.cycles += 14;
        c.g[FP][0] = !(0u64 | c.g[A1][0]);
        c.g[T3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_IN_PTR));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_IN_END));
        c.g[A1][0] = c.g[A1][0] & 0x20;
        c.sw((c.g[SP][0] as u32).wrapping_add(AN_SP_FLAGS), c.g[A1][0]);
        c.g[A0][0] = (c.g[S7][0] < 0x63u64) as u64;
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(AN_LEN_COUNTS));
        c.g[A2][0] = 0x100;
        c.g[S2][0] = 0u64;
        c.g[V0][0] = (c.g[T3][0] < c.g[V1][0]) as u64;
        c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(HUFF_CODE_LEN));
        c.sw((c.g[SP][0] as u32).wrapping_add(AN_SP_SMALL), c.g[A0][0]);
        let has_input = c.g[V0][0] == 0u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(AN_SP_LENTAB), c.g[A1][0]);
        if !has_input {
            c.cycles += 1;
            c.g[A3][0] = c.g[S4][0];
            // 0x002c8ef0: one pass per input position.
            'histogram: loop {
                // Inner run/byte handling: only the run path files a run
                // length. The run count below bumps only on the long-run
                // path (generated `break 'b_2c8f60` falls into it); a
                // filed run and a non-run byte both rejoin at the byte
                // filing without bumping (generated `break 'b_2c8f6c`).
                c.cycles += 5;
                c.g[S0][0] = c.lbu(c.g[T3][0] as u32);
                c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x100));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                let mut filed = false;
                let mut run_path = false;
                if c.g[S0][0] == c.g[A2][0] {
                    run_path = true;
                    // 0x002c8f3c: count the run of the fill byte.
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7530));
                    c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.g[V1][0];
                    }
                    c.g[V1][0] = (c.g[T3][0] < c.g[A0][0]) as u64;
                    c.g[S3][0] = 0u64;
                    if c.g[V1][0] != 0u64 {
                        c.cycles += 1;
                        c.g[S0][0] = c.lbu(c.g[T3][0] as u32);
                        // 0x002c8f20: extend the run while bytes repeat.
                        'run: loop {
                            c.cycles += 4;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                            c.g[S2][0] = sx((c.g[S2][0] as u32)
                                .wrapping_add(c.g[S0][0] as u32));
                            if c.g[S0][0] == c.g[A2][0] {
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[T3][0] < c.g[A0][0]) as u64;
                                if c.g[V0][0] == 0u64 {
                                    break 'run;
                                }
                                c.g[S0][0] = c.lbu(c.g[T3][0] as u32);
                                if event_due(c) {
                                    deliver_event_here(c, 0x002c_8f20);
                                }
                                continue 'run;
                            }
                            break 'run;
                        }
                    }
                    // 0x002c8f60: file the run length (long runs skip it).
                    // A filed run skips the run count below; a non-run
                    // byte never reaches it.
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S3][0] < 0xffu64) as u64;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x200));
                    filed = c.g[V0][0] != 0u64;
                    if filed {
                        c.cycles += 6;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    }
                }
                if run_path && !filed {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(AN_RUN_COUNT));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S1][0] as u32).wrapping_add(AN_RUN_COUNT), c.g[V0][0]);
                }
                // 0x002c8ef0 tail: file the byte's own count.
                c.cycles += 17;
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V1][0] = c.g[V1][0] & 0xff;
                c.g[A0][0] = c.lw(c.g[A1][0] as u32);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x100));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw(c.g[A1][0] as u32, c.g[A0][0]);
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_IN_END));
                c.g[V0][0] = (c.g[T3][0] < c.g[V1][0]) as u64;
                c.g[A2][0] = c.g[S0][0];
                if c.g[V0][0] == 0u64 {
                    break 'histogram;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_8ef0);
                }
            }
        }
        // 0x002c8fc4: at least one run; stash the byte sum.
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(AN_RUN_COUNT));
        c.sw((c.g[S1][0] as u32).wrapping_add(AN_BYTE_SUM), c.g[S2][0]);
        if c.g[V0][0] == 0u64 {
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            c.sw((c.g[S1][0] as u32).wrapping_add(AN_RUN_COUNT), c.g[V0][0]);
        }
        // Mode select: the most frequent byte becomes the literal
        // selector, with its count and the repeat/delta bases.
        c.cycles += 7;
        c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE), 0u64);
        c.g[S2][0] = 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL), 0u64);
        c.g[S0][0] = 0u64;
        c.g[A3][0] = c.g[S4][0];
        c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
        // 0x002c8fe0: one pass per candidate byte.
        'mode_select: loop {
            // 0x002c8fe0/0x002c903c: test this byte's count.
            c.cycles += 10;
            c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            c.g[A2][0] = c.g[S0][0];
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[S3][0] = 0u64;
            c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
            let smaller = c.g[A0][0] != 0u64;
            if c.g[V0][0] != 0 {
                c.g[S2][0] = c.g[S0][0];
            }
            if smaller {
                // Found a live count: file it and scan on.
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL));
            } else {
                // No count here: scan forward for the next live one.
                c.cycles += 3;
                c.g[V0][0] = (c.g[S0][0] < 0x100u64) as u64;
                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL));
                    // Fall through to the select test below.
                } else {
                    c.cycles += 3;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    // 0x002c9020: scan for a live count.
                    'live_scan: loop {
                        c.cycles += 4;
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        if c.g[V0][0] != 0u64 {
                            break 'live_scan;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[S0][0] < 0x100u64) as u64;
                        if c.g[V0][0] == 0u64 {
                            break 'live_scan;
                        }
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                        if event_due(c) {
                            deliver_event_here(c, 0x002c_9020);
                        }
                        continue 'live_scan;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL));
                }
            }
            // 0x002c9078/0x002c9074: keep the best byte so far.
            c.cycles += 3;
            c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
            if c.g[V0][0] == 0u64 {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_BASE), c.g[A2][0]);
                c.g[V0][0] = (c.g[S3][0] < c.g[V1][0]) as u64;
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL), c.g[S3][0]);
                if c.g[V0][0] == 0u64 {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR));
                    c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL), c.g[V1][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_BASE), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR), c.g[A2][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE), c.g[S3][0]);
                }
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            } else {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[S0][0] < 0x100u64) as u64;
            c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
            if c.g[V0][0] == 0u64 {
                break 'mode_select;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_8fe0);
            }
        }
        // Flag ladders: the entry flags enable the delta mode and level
        // adjustments. Each compare is captured before the delay slot
        // overwrites `v0` with the next flag bit.
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(AN_SP_FLAGS));
        let skip_delta = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.g[FP][0] & 0x2;
        if !skip_delta {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
            let keep_mode = c.g[V0][0] != 0u64;
            c.g[V0][0] = c.g[FP][0] & 0x2;
            if !keep_mode {
                c.cycles += 4;
                c.g[V0][0] = 0x1;
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR), c.g[S2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE), c.g[V0][0]);
                c.g[V0][0] = c.g[FP][0] & 0x2;
            }
        }
        c.cycles += 2;
        let skip_levels = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.g[FP][0] & 0x4;
        if !skip_levels {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
            let single_level = c.g[V0][0] != 0u64;
            c.g[V0][0] = c.g[FP][0] & 0x1;
            if !single_level {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE), c.g[V0][0]);
                c.g[V0][0] = c.g[FP][0] & 0x1;
            }
            c.cycles += 2;
            if c.g[V0][0] != 0u64 {
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE), 0u64);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[FP][0] & 0x4;
        }
        // Mode/value cross-check: large literal selectors demote the mode.
        // A set flag bit clears the selector and skips the cross-check
        // (`v0` still holds the flag bit from the ladder above).
        c.cycles += 2;
        if c.g[V0][0] == 0u64 {
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL));
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < 0xbu64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
            } else {
                c.cycles += 8;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR));
                c.g[S3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_BASE));
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE), c.g[V1][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR), c.g[V0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_BASE), c.g[A2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL), c.g[S3][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
            }
            // Re-halve the rebalanced selector into the window (generated
            // `'b_2c9144` tail: reached only on this (flag-clear) path; the
            // flag-set path breaks out past it, to the delta window).
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
            c.g[V0][0] = sx((c.g[A0][0] as u32) >> 2);
            if c.g[V0][0] != 0u64 {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_BASE));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE), c.g[V0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL), c.g[A0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR), c.g[V1][0]);
            }
        } else {
            // Set flag bit: clear the selector and skip the cross-check
            // and halve above (the generated `break 'b_2c9144` lands past
            // both, at the delta window).
            c.cycles += 2;
            c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL), 0u64);
        }
        // Delta window: derive the delta min/max from the halved range,
        // then search the scratch tables for the window edges.
        c.cycles += 3;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL));
        c.g[V1][0] = 0x19;
        if c.g[A1][0] != 0u64 {
            // Halve the selector (rounding toward negative infinity).
            c.cycles += 12;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(AN_DELTA_SRC));
            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 31);
            c.g[S0][0] = 0x1;
            c.divu(0, 4, 3);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_MIN), c.g[V0][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_MAX), c.g[A0][0]);
            // 0x002c91d4: pin the low edge into the scratch table.
            c.cycles += 3;
            c.g[A2][0] = c.lo[0];
            c.sw((c.g[SP][0] as u32).wrapping_add(AN_SP_LO), c.g[A2][0]);
            if c.g[A0][0] != 0u64 {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(AN_SCR_LO));
                // 0x002c9198: walk up while the entry is below the edge.
                'pin_low: loop {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V0][0] = (c.g[A2][0] < c.g[V1][0]) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_MAX));
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_BASE));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_MAX));
                    }
                    c.cycles += 5;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = (c.g[V0][0] < c.g[S0][0]) as u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] != 0u64 {
                        break 'pin_low;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002c_9198);
                    }
                }
            }
            // 0x002c922c: pin the high edge into the scratch table.
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_MIN));
            c.g[S0][0] = 0x1;
            if c.g[V0][0] != 0u64 {
                c.cycles += 2;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(AN_SCR_HI));
                // 0x002c91e8: walk down while the entry is above the edge.
                'pin_high: loop {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V0][0] = (c.g[A2][0] < c.g[V1][0]) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_MIN));
                    } else {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_BASE));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_MIN));
                    }
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
                    c.g[V0][0] = (c.g[V0][0] < c.g[S0][0]) as u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4));
                    if c.g[V0][0] != 0u64 {
                        break 'pin_high;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002c_91e8);
                    }
                }
            }
            // 0x002c9264: find the last live level below the selector.
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL));
            c.g[S3][0] = 0u64;
            c.g[S0][0] = 0u64;
            if c.g[A0][0] != 0u64 {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_BASE));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                // 0x002c9248: scan the levels for the last live one.
                'last_live: loop {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    if c.g[V0][0] != 0 {
                        c.g[S3][0] = c.g[S0][0];
                    }
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[S0][0] < c.g[A0][0]) as u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] == 0u64 {
                        break 'last_live;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002c_9248);
                    }
                }
            }
            // 0x002c92a0: rebalance the window around the last live level.
            c.cycles += 10;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_BASE));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S3][0] as u32));
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR));
            c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL), c.g[V0][0]);
            if c.g[A1][0] == c.g[V1][0] {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR), c.g[V1][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE), c.g[V0][0]);
            }
            // Re-halve the rebalanced selector into the window.
            c.cycles += 8;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL));
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 31);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.g[V0][0] = sx(0u32.wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_MIN), c.g[V0][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_MAX), c.g[V1][0]);
        }
        // 0x002c930c: copy the windowed levels to the code table.
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
        if c.g[V0][0] != 0u64 {
            c.cycles += 2;
            c.g[S0][0] = 0u64;
            c.cycles += 3;
            c.g[A2][0] = c.g[S4][0];
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(AN_SCR_OUT));
            // 0x002c92e0: copy each level's base word across.
            'copy_levels: loop {
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR));
                c.g[A0][0] = c.lw(c.g[A1][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
                c.g[V1][0] = (c.g[S0][0] < c.g[V1][0]) as u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                if c.g[V1][0] == 0u64 {
                    break 'copy_levels;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_92e0);
                }
            }
        }
        // Table build: size every level's repeat code, then clear the
        // working tables. Small modes skip the level loop.
        c.cycles += 6;
        c.g[RA][0] = sx(0x002c_9314);
        c.g[A0][0] = c.g[S1][0];
        callee::HUFF_MAKETREE.call(c, 0x002c_9314)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(AN_SP_SCRATCH));
        if c.g[V1][0] >= 0x2u64 {
            c.cycles += 4;
            c.g[S0][0] = 0x1;
            if c.g[S0][0] >= c.g[V1][0] {
                // Unreachable (the mode is at least 2 here).
                c.g[S0][0] = 0u64;
            } else {
                c.cycles += 3;
                c.g[S6][0] = 0x8;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR));
                // 0x002c9340: price one level per pass.
                'level_cost: loop {
                    c.cycles += 9;
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = (c.g[A2][0] < 0x9u64) as u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    if c.g[V1][0] == 0 {
                        c.g[A2][0] = c.g[S6][0];
                    }
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A0][0] = c.g[S1][0];
                    if c.g[V1][0] == 0u64 {
                        // Empty level: file nothing, take the join below.
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
                    } else {
                        c.cycles += 11;
                        c.g[RA][0] = sx(0x002c_936c);
                        c.g[A1][0] = c.g[S0][0];
                        callee::HUFF_MINREP.call(c, 0x002c_936c)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR));
                        c.g[A2][0] = c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[A3][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A3][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                        c.g[V1][0] = (c.g[V0][0] < c.g[A2][0]) as u64;
                        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        if c.g[V1][0] == 0u64 {
                            // Repeat costs more: file nothing.
                            c.cycles += 2;
                            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.sw(c.g[V0][0] as u32, 0u64);
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
                        } else {
                            c.cycles += 7;
                            c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                            c.g[A0][0] = sx((c.g[S0][0] as u32) >> 1);
                            c.mult(0, 2, 2, 5);
                            c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] =
                                    sx((c.g[S4][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                c.sw(c.g[V0][0] as u32, 0u64);
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.cycles += 1;
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(AN_SP_SCRATCH));
                        break 'level_cost;
                    }
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR));
                    if event_due(c) {
                        deliver_event_here(c, 0x002c_9340);
                    }
                }
            }
        }
        // Level-cost shared exit (0x002c93d0 tail): the generated skip and
        // loop-completion breaks both land on `cycles += 1; S0 = 0`.
        c.cycles += 1;
        c.g[S0][0] = 0u64;
        // 0x002c93d8: clear the working tables, saving each word aside.
        c.cycles += 2;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S4][0];
        'clear_tables: loop {
            c.cycles += 12;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.sw(c.g[A0][0] as u32, 0u64);
            c.g[V1][0] = (c.g[S0][0] < 0x100u64) as u64;
            c.sw(c.g[A1][0] as u32, c.g[V0][0]);
            c.sw(c.g[A2][0] as u32, 0u64);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x400), 0u64);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x800), 0u64);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            if c.g[V1][0] == 0u64 {
                break 'clear_tables;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_93d8);
            }
        }
        // Second histogram: re-scan the input with the windowed tables,
        // charging each run's multi-level cost and filing the delta index.
        c.cycles += 5;
        c.g[T3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_IN_PTR));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_IN_END));
        c.g[V0][0] = (c.g[T3][0] < c.g[V1][0]) as u64;
        c.g[A2][0] = 0x100;
        if c.g[V0][0] != 0u64 {
            c.cycles += 3;
            c.sw((c.g[SP][0] as u32).wrapping_add(AN_SP_CODETAB), c.g[S5][0]);
            c.g[S6][0] = c.g[S5][0];
            c.g[T7][0] = c.g[S4][0];
            // 0x002c9428: one pass per input position.
            'rescan: loop {
                // 0x002c9604: run-length handling for this byte.
                c.cycles += 3;
                c.g[S0][0] = c.lbu(c.g[T3][0] as u32);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                if c.g[S0][0] == c.g[A2][0] {
                    // 0x002c9470: count the run against the window end.
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x7530));
                    c.g[S3][0] = 0u64;
                    c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
                    c.g[T4][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_MODE));
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = (c.g[T3][0] < c.g[A0][0]) as u64;
                    c.g[T9][0] = sx((c.g[S0][0] as u32) << 2);
                    if c.g[V0][0] != 0u64 {
                        c.cycles += 1;
                        c.g[S0][0] = c.lbu(c.g[T3][0] as u32);
                        // 0x002c9458: extend the run while bytes repeat.
                        'run2: loop {
                            c.cycles += 3;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                            if c.g[S0][0] == c.g[A2][0] {
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[T3][0] < c.g[A0][0]) as u64;
                                if c.g[V0][0] == 0u64 {
                                    break 'run2;
                                }
                                c.g[S0][0] = c.lbu(c.g[T3][0] as u32);
                                if event_due(c) {
                                    deliver_event_here(c, 0x002c_9458);
                                }
                                continue 'run2;
                            }
                            break 'run2;
                        }
                    }
                    // 0x002c9558/0x002c9554: charge the run's level cost.
                    // A single level left skips the split and the settle
                    // below, rejoining at the cheaper-cost test.
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[T9][0] as u32));
                    c.g[T6][0] = 0x7d00;
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[T2][0] = 0x7d00;
                    c.mult(0, 24, 19, 3);
                    // With no live levels the head cost stands and only
                    // the tail below runs; a single level skips the split,
                    // the settle and the tail.
                    let t4_live = c.g[T4][0] != 0u64;
                    let mut settle_costs = t4_live;
                    if c.g[T4][0] != 0u64 {
                        // 0x002c94cc: table cost against the live levels.
                        c.cycles += 7;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR));
                        c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A3][0] = c.g[V0][0];
                        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        let table_empty = c.g[V0][0] == 0u64;
                        c.g[V0][0] = (c.g[S3][0] < 0xfcu64) as u64;
                        if table_empty {
                            // Empty level: keep the fallback cost.
                        } else if c.g[V0][0] == 0u64 {
                            // Long run: keep the fallback cost.
                            c.cycles += 2;
                            c.g[T6][0] = 0x14;
                        } else {
                            c.cycles += 2;
                            c.g[T6][0] = 0x14;
                            c.cycles += 8;
                            c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[A1][0] = c.lw(c.g[A0][0] as u32);
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(HUFF_NUM_BITS));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3));
                            c.g[T6][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        }
                        // Charge the multi-level split when more levels wait.
                        // A single level left skips the split and the
                        // settle below, rejoining at the cheaper-cost test.
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[T4][0] < 0x2u64) as u64;
                        let single_level = c.g[V0][0] != 0u64;
                        c.g[V0][0] = (c.g[T6][0] < c.g[T8][0]) as u64;
                        if single_level {
                            settle_costs = false;
                        } else {
                            c.cycles += 4;
                            c.g[T0][0] = c.g[S3][0];
                            c.g[S2][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x1));
                            c.g[T2][0] = 0u64;
                            if c.g[S2][0] != 0u64 {
                                c.cycles += 8;
                                c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(AN_SP_CODETAB));
                                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[SP][0] as u32));
                                c.g[A0][0] = sx((c.g[S2][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[T5][0] = 0u64;
                                // 0x002c9508: one pass per remaining level.
                                'split_cost: loop {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw(c.g[T1][0] as u32);
                                    if c.g[V0][0] != 0u64 {
                                        c.cycles += 3;
                                        c.divu(0, 8, 18);
                                        c.cycles += 8;
                                        c.g[A0][0] = c.lw(c.g[A3][0] as u32);
                                        c.g[V1][0] = c.lo[0];
                                        c.g[A1][0] = c.lo[0];
                                        c.mult(0, 2, 5, 18);
                                        c.mult(0, 3, 5, 4);
                                        c.g[T0][0] = sx((c.g[T0][0] as u32)
                                            .wrapping_sub(c.g[V0][0] as u32));
                                        c.g[T2][0] = sx((c.g[V1][0] as u32)
                                            .wrapping_add(c.g[T2][0] as u32));
                                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4));
                                    } else {
                                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4));
                                    }
                                    c.cycles += 3;
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x4));
                                    if c.g[S2][0] == 0u64 {
                                        break 'split_cost;
                                    }
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002c_9508);
                                    }
                                }
                            }
                        }
                        if settle_costs {
                            c.cycles += 2;
                            c.g[V0][0] = 0x7d00;
                            if c.g[T0][0] != 0 {
                                c.g[T2][0] = c.g[V0][0];
                            }
                        }
                        // The tail rejoins every path that ran the head
                        // cost; a single level keeps its pre-split compare.
                        // (Inside `T4 != 0`, `t4_live` is always true, so
                        // this is just `settle_costs`.)
                        if settle_costs {
                            c.cycles += 1;
                            c.g[V0][0] = (c.g[T6][0] < c.g[T8][0]) as u64;
                        }
                    } else {
                        // No live levels: generated breaks `'b_2c9554`
                        // straight onto the shared `+1; V0 = (T6 < T8)` tail.
                        c.cycles += 1;
                        c.g[V0][0] = (c.g[T6][0] < c.g[T8][0]) as u64;
                    }
                    // 0x002c9580: keep the cheaper of the two run costs. A
                    // stored run skips the filing and the spread below.
                    c.cycles += 2;
                    let skip_cheaper = c.g[V0][0] != 0u64;
                    c.g[V0][0] = (c.g[T6][0] < c.g[T2][0]) as u64;
                    let mut to_delta = false;
                    if !skip_cheaper {
                        c.cycles += 3;
                        let skip_store = (c.g[T2][0] < c.g[T8][0]) as u64 != 0;
                        c.g[V0][0] = (c.g[T6][0] < c.g[T2][0]) as u64;
                        if !skip_store {
                            c.cycles += 5;
                            c.g[V1][0] = sx((c.g[T7][0] as u32).wrapping_add(c.g[T9][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                            to_delta = true;
                        }
                    }
                    // 0x002c95a4: file the level counts for the next pass.
                    // A filed count skips the spread below as well.
                    if !to_delta {
                        c.cycles += 2;
                        if c.g[V0][0] != 0u64 {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[T7][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                            to_delta = true;
                        } else {
                            c.g[S2][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x1));
                        }
                    }
                    // A stored or filed count skips the spread below; the
                    // delta index still runs either way.
                    if !to_delta {
                        c.cycles += 2;
                        c.g[T0][0] = c.g[S3][0];
                        if c.g[S2][0] != 0u64 {
                            c.cycles += 3;
                            c.g[T2][0] = 0u64;
                            c.g[T1][0] = c.g[S4][0];
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR));
                            // 0x002c95b8: spread the remainder over the levels.
                            'spread_levels: loop {
                                c.cycles += 6;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                c.g[A3][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                if c.g[V0][0] != 0u64 {
                                    c.cycles += 3;
                                    c.divu(0, 8, 18);
                                    c.cycles += 8;
                                    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                    c.g[A1][0] = c.lo[0];
                                    c.mult(0, 4, 5, 18);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                } else {
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                }
                                c.cycles += 2;
                                if c.g[S2][0] == 0u64 {
                                    break 'spread_levels;
                                }
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR));
                                if event_due(c) {
                                    deliver_event_here(c, 0x002c_95b8);
                                }
                            }
                        }
                    }
                }
                // 0x002c96cc: pick the delta index for this byte.
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_LIT_SEL));
                c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                if c.g[V0][0] == 0u64 {
                    // No selector: keep the previous index.
                    c.cycles += 1;
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                }
                if c.g[V0][0] != 0u64 {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_MAX));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    c.g[S2][0] = 0u64;
                    if c.g[V0][0] == 0u64 {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_MIN));
                        c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let below_window = c.g[V0][0] != 0u64;
                        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                        if !below_window {
                            // Inside the window: settle the delta index.
                            c.cycles += 5;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_DELTA_BASE));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                            c.g[V1][0] = (c.g[S0][0] < c.g[A2][0]) as u64;
                            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            if c.g[V1][0] == 0u64 {
                                c.cycles += 5;
                                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            }
                        }
                        // Settle the winning index against the live tables.
                        c.cycles += 7;
                        c.g[T2][0] = sx((c.g[T0][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[T2][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(AN_SP_LO));
                        c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                        c.g[T1][0] = sx((c.g[S0][0] as u32) << 2);
                        if c.g[V0][0] == 0u64 {
                            // Keep the settled index.
                        } else {
                            c.cycles += 13;
                            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[T2][0] as u32));
                            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[T1][0] as u32));
                            c.g[A1][0] = c.lw(c.g[A0][0] as u32);
                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                            c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[T1][0] as u32));
                            c.g[V1][0] = 0x3;
                            c.g[A0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[S2][0] = 0x1;
                            c.g[V1][0] = (c.g[V1][0] < c.g[A3][0]) as u64;
                            if c.g[V1][0] != 0 {
                                c.g[S2][0] = 0u64;
                            }
                            c.g[V0][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            if c.g[A1][0] == c.g[A0][0] {
                                c.cycles += 6;
                                c.g[V1][0] = sx((c.g[T7][0] as u32).wrapping_add(c.g[T2][0] as u32));
                                c.g[A0][0] = sx((c.g[T7][0] as u32).wrapping_add(c.g[T1][0] as u32));
                                c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            }
                        }
                    }
                    c.cycles += 2;
                    let settled = c.g[S2][0] != 0u64;
                    c.g[V1][0] = sx((c.g[T0][0] as u32) << 2);
                    if !settled {
                        c.cycles += 1;
                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                    }
                }
                // File the winning index and loop over the input.
                c.cycles += 8;
                c.g[V1][0] = sx((c.g[T7][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_IN_END));
                c.g[V0][0] = (c.g[T3][0] < c.g[V1][0]) as u64;
                c.g[A2][0] = c.g[S0][0];
                if c.g[V0][0] == 0u64 {
                    break 'rescan;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_9428);
                }
            }
        }
        // Flag-gated count bump: with the small-input flag set, grow the
        // live-symbol count once.
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(AN_SP_FLAGS));
        if c.g[A0][0] != 0u64 {
            c.cycles += 6;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_CUR));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        }
        // Second table build: rebuild the decode tables, then fix the
        // per-symbol counts against the winning indices.
        c.cycles += 6;
        c.g[RA][0] = sx(0x002c_9718);
        c.g[A0][0] = c.g[S1][0];
        callee::HUFF_MAKETREE.call(c, 0x002c_9718)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(AN_SIZE_SLOT), 0u64);
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(AN_SP_SMALL));
        c.g[V0][0] = c.g[FP][0] & 0x8;
        if c.g[A1][0] != 0u64 {
            c.cycles += 6;
            c.g[T3][0] = c.g[S4][0];
            c.g[T6][0] = c.g[A1][0];
            c.g[T5][0] = c.g[T3][0];
            c.g[T0][0] = c.g[S5][0];
            c.g[T4][0] = c.g[S5][0];
            // 0x002c9740: one pass per table slot.
            'fix_counts: loop {
                c.cycles += 4;
                c.g[A2][0] = 0u64;
                c.g[S0][0] = 0u64;
                c.g[A1][0] = c.g[T0][0];
                c.g[A0][0] = c.g[T3][0];
                // 0x002c9750: find the slowest live level.
                'slowest_level: loop {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    if c.g[V0][0] != 0u64 {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                        c.g[V0][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
                        if c.g[V0][0] == 0u64 {
                            c.cycles += 4;
                            c.g[S2][0] = c.g[S3][0];
                            c.g[A2][0] = c.g[V1][0];
                            c.g[S3][0] = c.g[S0][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        } else {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        }
                    } else {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    }
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = (c.g[S0][0] < 0x100u64) as u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] == 0u64 {
                        break 'slowest_level;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002c_9750);
                    }
                }
                // 0x002c987c: settle the winning index for this slot.
                c.cycles += 3;
                c.g[V0][0] = (c.g[S7][0] < c.g[A2][0]) as u64;
                c.g[A2][0] = 0u64;
                if c.g[V0][0] != 0u64 {
                    // 0x002c97f0/0x002c97b8: probe the live tables, unless
                    // one already decides the index.
                    let mut scan_win = true;
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_SYM_FLAG));
                    c.g[T2][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[T1][0] = sx((c.g[S3][0] as u32) << 2);
                    if c.g[V0][0] != 0u64 {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2564));
                        c.g[V0][0] = (c.g[V0][0] < c.g[S7][0]) as u64;
                        if c.g[V0][0] == 0u64 {
                            scan_win = true;
                        } else {
                            c.g[S0][0] = c.g[A2][0];
                            scan_win = false;
                        }
                    }
                    if scan_win {
                        // 0x002c97bc: scan for the winning index, starting
                        // past the already-probed slot.
                        c.cycles += 1;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        'winning_index: loop {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[A2][0] < 0x100u64) as u64;
                            c.g[A0][0] = sx((c.g[A2][0] as u32) << 2);
                            if c.g[V0][0] == 0u64 {
                                break 'winning_index;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            if c.g[V1][0] != 0u64 {
                                c.cycles += 5;
                                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                c.g[V0][0] = (c.g[V0][0] < c.g[S7][0]) as u64;
                                if c.g[V0][0] != 0u64 {
                                    break 'winning_index;
                                }
                            }
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            if event_due(c) {
                                deliver_event_here(c, 0x002c_97bc);
                            }
                        }
                    }
                    // 0x002c9848/0x002c9808: probe the code tables.
                    c.cycles += 1;
                    c.g[S0][0] = c.g[A2][0];
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S0][0] < 0x100u64) as u64;
                    // The generated `t = (V0 == 0)` tests the bound above;
                    // `V0` is recomputed for the scan addresses below.
                    let scan_codes = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    if scan_codes {
                        c.cycles += 3;
                        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
                        // 0x002c9808: scan for the cheapest code.
                        'cheapest_code: loop {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                            if c.g[V0][0] == 0u64 {
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = c.lw(c.g[A1][0] as u32);
                                c.g[V0][0] = (c.g[A0][0] < c.g[S7][0]) as u64;
                                // The generated `t = (V0 == 0)` tests the
                                // code bound above; `V0` is recomputed for
                                // the length-table address below.
                                let code_live = c.g[V0][0] != 0u64;
                                c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                                if code_live {
                                    c.cycles += 4;
                                    c.g[V0][0] =
                                        sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[V1][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
                                    if c.g[V1][0] != 0 {
                                        c.g[A2][0] = c.g[S0][0];
                                    }
                                }
                                c.cycles += 1;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            }
                            c.cycles += 4;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = (c.g[S0][0] < 0x100u64) as u64;
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4));
                            if c.g[V0][0] == 0u64 {
                                break 'cheapest_code;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x002c_9808);
                            }
                        }
                    }
                    // File the settled counts.
                    c.cycles += 13;
                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[V1][0] as u32, c.g[S0][0]);
                    c.sw(c.g[A1][0] as u32, c.g[S0][0]);
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(AN_SIZE_SLOT), c.g[S7][0]);
                    if c.g[T6][0] == 0u64 {
                        c.cycles += 1;
                        c.g[V0][0] = c.g[FP][0] & 0x8;
                        break 'fix_counts;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002c_9740);
                    }
                    continue 'fix_counts;
                }
                // The slowest level settled at or below the input size:
                // leave the fix-up region, as the generated body breaks
                // out to the flag test below.
                c.cycles += 1;
                c.g[V0][0] = c.g[FP][0] & 0x8;
                break 'fix_counts;
            }
        }
        // Count tables: zero the length histogram, rebuild it from the
        // code lengths, then emit the order list and the final counts.
        c.cycles += 2;
        c.g[S0][0] = 0u64;
        if c.g[V0][0] != 0u64 {
            c.cycles += 2;
            c.g[A2][0] = 0x8;
            c.g[V1][0] = c.g[S5][0];
            // 0x002c9890: stamp the small-length band.
            'stamp_band: loop {
                c.cycles += 7;
                c.sw(c.g[V1][0] as u32, c.g[A2][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[S0][0] < 0x100u64) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                if c.g[V0][0] == 0u64 {
                    break 'stamp_band;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_9890);
                }
            }
        }
        c.cycles += 3;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(AN_SP_LENTAB));
        c.g[S0][0] = 0u64;
        // 0x002c98b8: zero the 17 length counts.
        'zero_counts: loop {
            c.cycles += 7;
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = (c.g[S0][0] < 0x11u64) as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            if c.g[V0][0] == 0u64 {
                break 'zero_counts;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_98b8);
            }
        }
        c.cycles += 3;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(AN_SP_LENTAB));
        c.g[S0][0] = 0u64;
        c.g[A0][0] = c.g[S5][0];
        // 0x002c98e0: histogram the code lengths per length value.
        'hist_lengths: loop {
            c.cycles += 4;
            c.g[V1][0] = c.lw(c.g[A0][0] as u32);
            c.g[V0][0] = (c.g[V1][0] < 0x11u64) as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            if c.g[V0][0] != 0u64 {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            }
            c.cycles += 4;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = (c.g[S0][0] < 0x100u64) as u64;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            if c.g[V0][0] == 0u64 {
                break 'hist_lengths;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_98e0);
            }
        }
        c.cycles += 6;
        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(AN_SP_LENTAB));
        c.g[S2][0] = 0u64;
        c.g[S3][0] = 0x1;
        c.g[T0][0] = c.g[S5][0];
        c.g[A1][0] = 0x2dec;
        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
        // 0x002c9928: emit the order list for each newly seen length.
        // Unseen lengths file nothing but still advance to the next
        // length; the loop only ends past length 16.
        'order_lengths: loop {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            if c.g[V1][0] != 0u64 {
                c.cycles += 4;
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.g[T0][0];
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                // 0x002c9948: file this length's symbols in order.
                'file_length: loop {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    if c.g[V0][0] == c.g[S3][0] {
                        c.cycles += 4;
                        c.sw(c.g[V1][0] as u32, c.g[A2][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    }
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[A2][0] < 0x100u64) as u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] == 0u64 {
                        break 'file_length;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002c_9948);
                    }
                }
                c.cycles += 1;
                c.g[S2][0] = c.g[S3][0];
            }
            c.cycles += 4;
            c.g[S3][0] = c.g[A3][0];
            c.g[V0][0] = (c.g[S3][0] < 0x11u64) as u64;
            let more_lengths = c.g[V0][0] != 0u64;
            c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
            if !more_lengths {
                break 'order_lengths;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_9928);
            }
        }
        // 0x002c9a14: emit the pending order-list counts.
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_ORDER_COUNT));
        c.g[T0][0] = 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(HUFF_NUM_COUNT), c.g[S2][0]);
        c.g[S3][0] = 0u64;
        c.g[S0][0] = 0u64;
        if c.g[V0][0] != 0u64 {
            c.cycles += 3;
            c.g[T2][0] = sx((c.g[S1][0] as u32).wrapping_add(HUFF_ORDER_TAB));
            c.g[T1][0] = c.g[S5][0];
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(HUFF_CODE));
            // 0x002c99a8: walk the order list, accumulating code bits.
            'walk_order: loop {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                c.g[A3][0] = sx((c.g[A2][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if c.g[V0][0] == 0u64 {
                    // Length class already covered: file the running code.
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    // 0x002c99d8: accumulate this code's bits.
                    'accum_bits: loop {
                        c.cycles += 7;
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = (c.g[S3][0] < c.g[V1][0]) as u64;
                        c.g[T0][0] = sx((c.g[T0][0] as u32) << 1);
                        if c.g[V0][0] == 0u64 {
                            break 'accum_bits;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x002c_99d8);
                        }
                    }
                }
                c.cycles += 8;
                c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                c.g[S0][0] = c.g[A1][0];
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw(c.g[V0][0] as u32, c.g[T0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(HUFF_ORDER_COUNT));
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                if c.g[V0][0] == 0u64 {
                    break 'walk_order;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_99a8);
                }
            }
        }
        // Epilogue: restore the callee-saved registers and return.
        c.cycles += 12;
        frame.restore(c, S0, 0x8b0);
        frame.restore(c, S1, 0x8a0);
        frame.restore(c, S2, 0x890);
        frame.restore(c, S3, 0x880);
        frame.restore(c, S4, 0x870);
        frame.restore(c, S5, 0x860);
        frame.restore(c, S6, 0x850);
        frame.restore(c, S7, 0x840);
        frame.restore(c, FP, 0x830);
        frame.restore_ra(c, 0x820);
        frame.close(c);
        Ok(())
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, check_lohi, check_paths, Rng, DATA};
    use crate::difftest::RA as SENTINEL;
    use crate::game::lib::eafile::tests::ea_fixed_costs;
    use crate::game::misc::tests::{
        huffmem_install_fakes, HUFFMEM_AREA, HUFFMEM_CTRL, HUFFMEM_MAGIC, HUFFMEM_RET_ALLOC,
        HUFFMEM_RET_NEW, HUFFMEM_TMP, HUFF_IN, HUFF_S,
    };
    use crate::game::misc::{
        HUFF_CODE, HUFF_CODE_LEN, HUFF_CUR, HUFF_DELTA_MAX, HUFF_DELTA_MIN, HUFF_IN_END,
        HUFF_IN_PTR, HUFF_LIT_SEL, HUFF_MODE,
    };

    driven!(gen_huff_decode_stream, ee::s08::f_002c76c0__generated);
    driven!(hand_huff_decode_stream, huff_decode_stream);

    /// Decode input: null source (early exit) or a block built by the real
    /// encoder, mirroring `huff_decode_matches` but calling the stream
    /// decoder directly.
    pub(crate) fn decode_stream_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        if seed % 4 == 0 {
            c.g[A0][0] = 0;
        } else {
            huffmem_install_fakes();
            ea_fixed_costs(c);
            c.bus.write32(HUFFMEM_CTRL, HUFFMEM_MAGIC);
            for i in 0..0x3200u32 {
                c.bus.write8(HUFFMEM_AREA + i, 0x5a);
            }
            for i in 0..0x400u32 {
                c.bus.write8(HUFFMEM_TMP + i, 0xa5);
            }
            const SRC: u32 = DATA + 0x500;
            for i in 0..16u32 {
                c.bus.write8(SRC + i, (r.next_u32() | 1) as u8);
            }
            const DST: u32 = DATA + 0x600;
            for i in 0..0x400u32 {
                c.bus.write8(DST + i, 0xa5);
            }
            c.bus.write32(HUFFMEM_RET_ALLOC, HUFFMEM_AREA);
            c.bus.write32(HUFFMEM_RET_NEW, HUFFMEM_TMP);
            c.g[A0][0] = SRC as u64;
            c.g[A1][0] = [1, 8, 16][(seed % 3) as usize] as u64;
            c.g[A2][0] = DST as u64;
            c.g[A3][0] = 0;
            c.g[RA][0] = SENTINEL as u64;
            ee::s09::f_002ca100__generated(c);
            c.g[A0][0] = DST as u64;
        }
        for i in 0..16u32 {
            c.bus.write8(DATA + 0x300 + i, 0x5a);
        }
        c.g[A1][0] = (DATA + 0x300) as u64;
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x002c_76c0;
    }

    /// Null source plus encoder-built streams (modes from `lenclass`).
    #[test]
    fn huff_decode_stream_matches() {
        check(
            gen_huff_decode_stream,
            hand_huff_decode_stream,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002c_76c0, 0);
                decode_stream_fixture(c, &mut r, seed);
            },
        );
    }

    /// Null-source early exit only (generated total 28).
    #[test]
    fn huff_decode_stream_paths() {
        check_paths(
            gen_huff_decode_stream,
            hand_huff_decode_stream,
            8,
            &[28],
            &|c, seed| {
                let _r = machine(c, seed, 0x002c_76c0, 0);
                c.g[A0][0] = 0;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x002c_76c0;
            },
        );
    }

    /// Event delivery and unwinds over the stream decode.
    #[test]
    fn huff_decode_stream_events() {
        check_events(
            gen_huff_decode_stream,
            hand_huff_decode_stream,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002c_76c0, 0);
                decode_stream_fixture(c, &mut r, seed);
                schedule(c, &mut r, seed, 0x002c_76c0)
            },
        );
    }

    driven!(gen_huff_analysis, ee::s09::f_002c8e48__generated);
    driven!(hand_huff_analysis, huff_analysis);

    /// Valid analysis state: scratch tables zeroed, code lengths and
    /// frequencies filled small, input bytes, flags and sizes steered by
    /// seed. Covers the histogram, mode select, delta window, both table
    /// builds and the small-input fix-up entry/exit (every loop bound
    /// stays tiny so every seed terminates).
    pub(crate) fn huff_analysis_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        for i in 0..0x3000u32 {
            c.bus.write8(HUFF_S + i, 0);
        }
        // Code lengths 0..4 and frequencies 0..2 keep maketree/minrep short.
        for sym in 0..256u32 {
            c.bus.write32(HUFF_S + HUFF_CODE_LEN + sym * 4, r.next_u32() % 5);
            c.bus.write32(HUFF_S + 0x100 + sym * 4, r.next_u32() % 3);
            c.bus.write32(HUFF_S + HUFF_CODE + sym * 4, 0);
        }
        // Input bytes: zeros, runs, or arbitrary bytes by seed class.
        let in_len: u32 = [0, 1, 8, 16, 64][(seed % 5) as usize];
        for i in 0..in_len {
            let b = match seed % 3 {
                0 => 0u8,
                1 => (i % 4) as u8,
                _ => (r.next_u32() & 0xff) as u8,
            };
            c.bus.write8(HUFF_IN + i, b);
        }
        for i in in_len..72u32 {
            c.bus.write8(HUFF_IN + i, 0xa5);
        }
        c.bus.write32(HUFF_S + HUFF_IN_PTR, HUFF_IN);
        c.bus.write32(HUFF_S + HUFF_IN_END, HUFF_IN + in_len);
        // Mode/selector/delta fields stay tiny (they bound loops).
        c.bus.write32(HUFF_S + HUFF_MODE, (seed % 4) as u32);
        c.bus.write32(HUFF_S + HUFF_LIT_SEL, (seed % 3) as u32);
        c.bus.write32(HUFF_S + HUFF_CUR, (seed % 3) as u32);
        c.bus.write32(HUFF_S + HUFF_DELTA_MIN, (seed % 2) as u32);
        c.bus.write32(HUFF_S + HUFF_DELTA_MAX, (seed % 2) as u32);
        c.bus.write32(HUFF_S + AN_BYTE_SUM, 0);
        c.bus.write32(HUFF_S + AN_SIZE_SLOT, 0);
        c.bus.write32(HUFF_S + AN_DELTA_SRC, (seed % 3) as u32);
        c.g[A0][0] = HUFF_S as u64;
        c.g[A1][0] = [0, 1, 0x20, 0x21][(seed % 4) as usize] as u64;
        // Sizes the generated body exits on: at or above 0x63 the small
        // fix-up region is skipped; below 0x63 it is entered but every
        // code length here is at most 4, so the slowest level settles at
        // or below the size and the region exits. (Smaller sizes hang the
        // generated body in that region, as on hardware.)
        c.g[A2][0] = [0x63, 0x64, 0x100, 5, 10, 0x62][(seed % 6) as usize] as u64;
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x002c_8e48;
    }

    /// Histogram, mode select, delta window and both table builds, with
    /// LO/HI compared (the delta window divides).
    #[test]
    fn huff_analysis_matches() {
        check_lohi(gen_huff_analysis, hand_huff_analysis, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x002c_8e48, 0);
            huff_analysis_fixture(c, &mut r, seed);
        });
    }

    /// Event delivery over the analysis (its checks never unwind).
    #[test]
    fn huff_analysis_events() {
        check_events(gen_huff_analysis, hand_huff_analysis, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x002c_8e48, 0);
            huff_analysis_fixture(c, &mut r, seed);
            schedule(c, &mut r, seed, 0x002c_8e48)
        });
    }
}
