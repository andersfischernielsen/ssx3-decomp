//! Tier B1: memory and string utilities with loops (`memset`, `memcpy`,
//! `EA_MemCopy`, `EA_MemSet`, `str*`, the UTF-16 `UStr_*` helpers;
//! `docs/investigation/phase2-candidates.md`).
//!
//! These are leaves, but unlike Tier A they loop, and a loop is where the
//! machine delivers interrupts and thread switches. Each replacement keeps
//! the original's chunking (bytes until aligned, then quadwords/doublewords,
//! then a byte tail), charges the EE cycles block by block and per
//! iteration exactly as the generated dispatcher does, and makes the
//! back-edge event checks at the same loop heads.
//!
//! # Event checks
//!
//! The generated code has two shapes, and each replacement copies the shape
//! of its original:
//!
//! - **Structured loops** (`'l_…: loop`): at a back edge,
//!   `if cycles >= next_event { pc = head; event(); }`. The result of
//!   `event()` is ignored (a requested unwind is only honoured by a later
//!   check elsewhere) and the function cannot be entered in the middle.
//! - **Dispatcher loops** (`match pc`): the same check, but an unwind request
//!   makes the function return at once with `pc` still at the loop head.
//!   The trampoline later re-enters the function *at that head*, so the
//!   replacement must accept those resume addresses as entry points and pick
//!   up its loop state from the registers.
//!
//! Either way the registers are the interface: before an event is delivered
//! every register the loop has changed holds the value the original would
//! have there (a handler or a thread switch saves the context), and after a
//! non-unwinding event the loop state is reloaded from the registers, as the
//! generated code reads them.
//!
//! Registers that the original leaves behind (loop flags in v0, the last
//! loaded quadword in a temporary, 0/-1 sentinels) are reproduced too: the
//! differential tests compare v0/v1 and the upper GPR halves as hard state.

use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0041_6210, memset, "memset"),
    (0x0041_605c, memcpy, "memcpy"),
    (0x003e_6574, ea_mem_copy, "EA_MemCopy"),
    (0x003e_6448, ea_mem_set, "EA_MemSet"),
    (0x0041_6810, strlen, "strlen"),
    (0x0041_65a8, strcmp, "strcmp"),
    (0x0041_66f4, strcpy, "strcpy"),
    (0x0041_6ce0, strncpy, "strncpy"),
    (0x003d_bca0, ustr_len, "UStr_Len"),
    (0x002c_2508, ustr_copy, "UStr_Copy"),
    (0x0037_1d10, dma_to_scratchpad, "Dma_ToScratchpad"),
    (0x0037_1dd8, dma_from_scratchpad, "Dma_FromScratchpad"),
];

// ---------------------------------------------------------------------------
// Register helpers (candidates for `abi`).

/// The low 32 bits of a GPR (a pointer or a 32-bit count).
#[inline]
fn r32(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// Leave a 32-bit value in a GPR as `addiu`/`lw` would (sign-extended,
/// upper 64 bits untouched).
#[inline]
fn set32(c: &mut Cpu, r: usize, v: u32) {
    c.g[r][0] = sx(v);
}

/// A 128-bit GPR as one value (lane 0 in the low bits).
#[inline]
fn q(c: &Cpu, r: usize) -> u128 {
    c.g[r][0] as u128 | (c.g[r][1] as u128) << 64
}

/// Leave a 128-bit value in a GPR (what `lq` or an MMI op leaves).
#[inline]
fn set_q(c: &mut Cpu, r: usize, v: u128) {
    c.g[r] = [v as u64, (v >> 64) as u64];
}

/// Is an event due? (`cycles >= next_event`, tested at back edges.)
#[inline]
fn event_due(c: &Cpu) -> bool {
    c.cycles >= c.next_event
}

/// Deliver an event with `pc` at the loop head `head`; true when the thread
/// must unwind. Callers spill their loop registers first and reload them
/// afterwards.
#[inline]
fn deliver_event(c: &mut Cpu, head: u32) -> bool {
    c.pc = head;
    c.event()
}

/// Return to the caller (`jr ra`).
#[inline]
fn ret(c: &mut Cpu) {
    c.pc = c.g[RA][0] as u32;
}

// ---------------------------------------------------------------------------
// UTF-16 strings.

/// `UStr_Len(s)` (0x3dbca0): the number of UTF-16 units before the
/// terminator.
///
/// Leaves a0 at the terminator and the length in v0 and v1.
pub fn ustr_len(c: &mut Cpu) {
    const HEAD: u32 = 0x003d_bcb0;
    c.cycles += 3;
    let mut ch = c.lhu(r32(c, A0));
    c.g[V0][0] = ch;
    c.g[V1][0] = 0;
    if ch != 0 {
        c.cycles += 1;
        let mut p = r32(c, A0);
        let mut len = 0u32;
        loop {
            c.cycles += 7;
            p = p.wrapping_add(2);
            ch = c.lhu(p);
            len = len.wrapping_add(1);
            if ch == 0 {
                break;
            }
            if event_due(c) {
                set32(c, A0, p);
                c.g[V0][0] = ch;
                set32(c, V1, len);
                deliver_event(c, HEAD);
                p = r32(c, A0);
                len = r32(c, V1);
            }
        }
        set32(c, A0, p);
        set32(c, V1, len);
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    ret(c);
}

/// `UStr_Copy(dst, src)` (0x2c2508): copies the UTF-16 string at `src`,
/// terminator included, and returns `dst`.
///
/// Leaves a0/a1 at the terminators and the (zero) last unit in v1.
pub fn ustr_copy(c: &mut Cpu) {
    const HEAD: u32 = 0x002c_2518;
    c.cycles += 3;
    let mut ch = c.lhu(r32(c, A1));
    c.g[V1][0] = ch;
    c.g[V0][0] = c.g[A0][0];
    if ch != 0 {
        c.cycles += 1;
        let (mut dst, mut src) = (r32(c, A0), r32(c, A1));
        loop {
            c.cycles += 7;
            c.sh(dst, ch);
            src = src.wrapping_add(2);
            ch = c.lhu(src);
            dst = dst.wrapping_add(2);
            if ch == 0 {
                break;
            }
            if event_due(c) {
                set32(c, A0, dst);
                set32(c, A1, src);
                c.g[V1][0] = ch;
                deliver_event(c, HEAD);
                dst = r32(c, A0);
                src = r32(c, A1);
                ch = c.g[V1][0];
            }
        }
        set32(c, A0, dst);
        set32(c, A1, src);
        c.g[V1][0] = ch;
    }
    c.cycles += 2;
    c.sh(r32(c, A0), 0);
    ret(c);
}

// ---------------------------------------------------------------------------
// memcpy / EA_MemCopy.

/// `memcpy(dst, src, n)` (0x41605c, newlib): returns `dst`.
///
/// When `n >= 32` and both pointers are 16-byte aligned it copies 32 bytes
/// per iteration (two `lq`/`sq` pairs, load-store-load-store, which matters
/// for overlapping buffers), then 8 bytes per iteration while at least 8
/// remain; the rest (or everything, otherwise) goes byte by byte. Forward
/// copy only.
///
/// Leaves a0 = -1, v1 at the end of `dst`, and the last quadwords copied in
/// v1/v0 (upper halves).
pub fn memcpy(c: &mut Cpu) {
    const QUAD_HEAD: u32 = 0x0041_6080;
    const DOUBLE_HEAD: u32 = 0x0041_60b8;
    const BYTE_HEAD: u32 = 0x0041_60e8;
    c.cycles += 4;
    let dst0 = c.g[A0][0];
    c.g[T0][0] = dst0;
    let small = c.g[A2][0] < 0x20;
    c.g[V0][0] = small as u64;
    c.g[V1][0] = dst0;
    // Each path below leaves `n` one less than the byte count still to copy,
    // as the byte loop expects (it counts down to -1).
    'bytes: {
        if !small {
            c.cycles += 4;
            let misaligned = (c.g[A1][0] | dst0) & 0xf;
            c.g[V0][0] = misaligned;
            if misaligned != 0 {
                set32(c, A2, r32(c, A2).wrapping_sub(1));
                break 'bytes;
            }
            c.cycles += 1;
            let (mut dst, mut src, mut n) = (dst0 as u32, r32(c, A1), r32(c, A2));
            // 32 bytes per iteration.
            loop {
                c.cycles += 11;
                let lo = c.bus.read128(src);
                n = n.wrapping_sub(0x20);
                src = src.wrapping_add(0x10);
                c.bus.write128(dst, lo);
                dst = dst.wrapping_add(0x10);
                let hi = c.bus.read128(src);
                src = src.wrapping_add(0x10);
                c.bus.write128(dst, hi);
                dst = dst.wrapping_add(0x10);
                set_q(c, V1, lo);
                set_q(c, V0, hi);
                let done = n < 0x20;
                c.g[A0][0] = done as u64;
                if done {
                    break;
                }
                if event_due(c) {
                    set32(c, A1, src);
                    set32(c, A2, n);
                    set32(c, A3, dst);
                    deliver_event(c, QUAD_HEAD);
                    (src, n, dst) = (r32(c, A1), r32(c, A2), r32(c, A3));
                }
            }
            c.cycles += 3;
            let lt8 = n < 8;
            c.g[V0][0] = lt8 as u64;
            set32(c, V1, dst);
            if !lt8 {
                // 8 bytes per iteration.
                loop {
                    c.cycles += 7;
                    let d = c.ld(src);
                    c.g[V1][0] = d;
                    n = n.wrapping_sub(8);
                    src = src.wrapping_add(8);
                    let done = n < 8;
                    c.g[V0][0] = done as u64;
                    c.sd(dst, d);
                    dst = dst.wrapping_add(8);
                    if done {
                        break;
                    }
                    if event_due(c) {
                        set32(c, A1, src);
                        set32(c, A2, n);
                        set32(c, A3, dst);
                        deliver_event(c, DOUBLE_HEAD);
                        (src, n, dst) = (r32(c, A1), r32(c, A2), r32(c, A3));
                    }
                }
                c.cycles += 1;
                set32(c, V1, dst);
            }
            set32(c, A1, src);
            set32(c, A2, n);
            set32(c, A3, dst);
        }
        c.cycles += 1;
        set32(c, A2, r32(c, A2).wrapping_sub(1));
    }
    // The byte tail: `n` counts down to -1 (kept in a0).
    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    c.g[A0][0] = u64::MAX;
    let mut n = r32(c, A2);
    if n != u32::MAX {
        let (mut dst, mut src) = (r32(c, V1), r32(c, A1));
        loop {
            c.cycles += 7;
            let b = c.lbu(src);
            c.g[V0][0] = b;
            n = n.wrapping_sub(1);
            src = src.wrapping_add(1);
            c.sb(dst, b);
            dst = dst.wrapping_add(1);
            if n == u32::MAX {
                break;
            }
            if event_due(c) {
                set32(c, A1, src);
                set32(c, A2, n);
                set32(c, V1, dst);
                deliver_event(c, BYTE_HEAD);
                (src, n, dst) = (r32(c, A1), r32(c, A2), r32(c, V1));
            }
        }
        set32(c, A1, src);
        set32(c, A2, n);
        set32(c, V1, dst);
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T0][0];
    ret(c);
}

/// `EA_MemCopy(dst, src, n)` (0x3e6574): EA's copy, no return value.
///
/// For `n >= 18` it first copies single bytes until `dst` is 16-byte
/// aligned (testing `--n <= 0` before each byte, an exit that cannot fire
/// for such `n`); if `src` is then aligned too it copies 128 bytes per
/// iteration (four `lq` then four `sq`, twice), then 8 bytes per iteration.
/// Everything else goes byte by byte. Counts are signed.
///
/// Leaves t0 = the alignment test, t1..t4 = the last data loaded.
pub fn ea_mem_copy(c: &mut Cpu) {
    const ALIGN_HEAD: u32 = 0x003e_665c;
    const BLOCK_HEAD: u32 = 0x003e_65a0;
    const DOUBLE_HEAD: u32 = 0x003e_65fc;
    const BYTE_HEAD: u32 = 0x003e_6630;
    c.cycles += 3;
    let small = (c.g[A2][0] as i64) < 18;
    c.g[T0][0] = c.g[A0][0] & 0xf;
    'bytes: {
        if small {
            break 'bytes;
        }
        c.cycles += 2;
        let (mut dst, mut src, mut n) = (r32(c, A0), r32(c, A1), r32(c, A2));
        if c.g[T0][0] != 0 {
            // Single bytes until dst is aligned.
            loop {
                c.cycles += 3;
                c.g[T0][0] = (dst & 0xf) as u64;
                if dst & 0xf == 0 {
                    break;
                }
                c.cycles += 3;
                n = n.wrapping_sub(1);
                set32(c, A2, n);
                let b = c.lbu(src);
                c.g[T1][0] = b;
                if n as i32 <= 0 {
                    c.cycles += 2;
                    ret(c);
                    return;
                }
                c.cycles += 4;
                src = src.wrapping_add(1);
                c.sb(dst, b);
                dst = dst.wrapping_add(1);
                set32(c, A0, dst);
                set32(c, A1, src);
                if event_due(c) {
                    deliver_event(c, ALIGN_HEAD);
                    (dst, src, n) = (r32(c, A0), r32(c, A1), r32(c, A2));
                }
            }
        }
        c.cycles += 3;
        c.g[T0][0] = (src & 0xf) as u64;
        if src & 0xf != 0 {
            set32(c, A0, dst);
            set32(c, A1, src);
            set32(c, A2, n);
            break 'bytes;
        }
        // 128 bytes per iteration while n >= 128 (n is biased by -128).
        c.cycles += 3;
        n = n.wrapping_sub(0x80);
        if n as i32 >= 0 {
            loop {
                c.cycles += 20;
                for half in [0u32, 0x40] {
                    let mut v = [0u128; 4];
                    for (i, x) in v.iter_mut().enumerate() {
                        *x = c.bus.read128(src.wrapping_add(half + 0x10 * i as u32));
                    }
                    for (i, x) in v.iter().enumerate() {
                        c.bus.write128(dst.wrapping_add(half + 0x10 * i as u32), *x);
                    }
                    for (i, x) in v.iter().enumerate() {
                        set_q(c, T1 + i, *x);
                    }
                }
                n = n.wrapping_sub(0x80);
                src = src.wrapping_add(0x80);
                dst = dst.wrapping_add(0x80);
                if (n as i32) < 0 {
                    break;
                }
                if event_due(c) {
                    set32(c, A0, dst);
                    set32(c, A1, src);
                    set32(c, A2, n);
                    deliver_event(c, BLOCK_HEAD);
                    (dst, src, n) = (r32(c, A0), r32(c, A1), r32(c, A2));
                }
            }
        }
        // 8 bytes per iteration while n >= 8 (n is biased by -8).
        c.cycles += 3;
        n = n.wrapping_add(0x78);
        if n as i32 >= 0 {
            loop {
                c.cycles += 7;
                let d = c.ld(src);
                c.g[T1][0] = d;
                src = src.wrapping_add(8);
                c.sd(dst, d);
                n = n.wrapping_sub(8);
                dst = dst.wrapping_add(8);
                if (n as i32) < 0 {
                    // The generated code re-tests the (false) condition.
                    c.cycles += 2;
                    break;
                }
                if event_due(c) {
                    set32(c, A0, dst);
                    set32(c, A1, src);
                    set32(c, A2, n);
                    deliver_event(c, DOUBLE_HEAD);
                    (dst, src, n) = (r32(c, A0), r32(c, A1), r32(c, A2));
                }
            }
        }
        c.cycles += 1;
        n = n.wrapping_add(8);
        set32(c, A0, dst);
        set32(c, A1, src);
        set32(c, A2, n);
    }
    // The byte tail: `n` is decremented before the test, so the loop runs
    // while the count before the decrement was positive.
    c.cycles += 3;
    let mut n = r32(c, A2).wrapping_sub(1);
    set32(c, A2, n);
    if n as i32 >= 0 {
        let (mut dst, mut src) = (r32(c, A0), r32(c, A1));
        loop {
            c.cycles += 7;
            n = n.wrapping_sub(1);
            let b = c.lbu(src);
            c.g[T1][0] = b;
            src = src.wrapping_add(1);
            c.sb(dst, b);
            dst = dst.wrapping_add(1);
            if (n as i32) < 0 {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                set32(c, A0, dst);
                set32(c, A1, src);
                set32(c, A2, n);
                deliver_event(c, BYTE_HEAD);
                (dst, src, n) = (r32(c, A0), r32(c, A1), r32(c, A2));
            }
        }
        set32(c, A0, dst);
        set32(c, A1, src);
        set32(c, A2, n);
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// memset / EA_MemSet (dispatcher shape: resumable at their loop heads).

/// Where a resumable function continues: a stage of the original, or out.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Flow<S> {
    Go(S),
    /// Returned to the caller.
    Done,
    /// An event asked the thread to unwind; `pc` is the loop head to resume at.
    Unwind,
}

/// Deliver a due event at a back edge of a dispatcher-shaped loop (the
/// caller has spilled its registers). True when the function must return
/// now; it is re-entered at `head` later.
#[inline]
fn back_edge_unwinds(c: &mut Cpu, head: u32) -> bool {
    event_due(c) && deliver_event(c, head)
}

const MEMSET: u32 = 0x0041_6210;
const MEMSET_END: u32 = 0x0041_62d0;
/// Resume points (loop heads with an event check).
const MEMSET_QUADS: u32 = 0x0041_624c;
const MEMSET_DOUBLE_STEP: u32 = 0x0041_6270;
const MEMSET_BYTES: u32 = 0x0041_62ac;

#[derive(Clone, Copy, PartialEq, Debug)]
enum MemsetStage {
    Entry,
    /// 32 bytes per iteration from the replicated quadword in t0.
    Quads,
    /// 8 bytes per iteration from the replicated doubleword in v1; `resume`
    /// enters at the count step after a store (0x416270).
    Doubles { resume: bool },
    /// The byte tail; `resume` enters the loop body (0x4162ac).
    Bytes { resume: bool },
}

/// `memset(dst, c, n)` (0x416210, newlib): fills `n` bytes with the low byte
/// of `c` and returns `dst`.
///
/// When `n >= 8` and `dst` is 16-byte aligned the byte is replicated
/// (`pcpyh`, then `pcpyld` into a quadword) and stored 32 bytes per
/// iteration while `n >= 32`, then 8 bytes per iteration while `n >= 8`;
/// the rest (or everything, otherwise) goes byte by byte.
///
/// The original is a `match pc` dispatcher whose loops return on an unwind
/// request; the trampoline re-enters it at the loop head, so those heads are
/// entry points here too.
pub fn memset(c: &mut Cpu) {
    let mut stage = match c.pc {
        MEMSET => MemsetStage::Entry,
        MEMSET_QUADS => MemsetStage::Quads,
        MEMSET_DOUBLE_STEP => MemsetStage::Doubles { resume: true },
        MEMSET_BYTES => MemsetStage::Bytes { resume: true },
        pc => {
            c.jump_out(pc, MEMSET, MEMSET_END);
            return;
        }
    };
    loop {
        let flow = match stage {
            MemsetStage::Entry => memset_entry(c),
            MemsetStage::Quads => memset_quads(c),
            MemsetStage::Doubles { resume } => memset_doubles(c, resume),
            MemsetStage::Bytes { resume } => memset_bytes(c, resume),
        };
        match flow {
            Flow::Go(s) => stage = s,
            Flow::Done => {
                c.cycles += 2;
                c.g[V0][0] = c.g[A0][0];
                ret(c);
                return;
            }
            Flow::Unwind => return,
        }
    }
}

/// Size and alignment tests; builds the fill pattern for the wide paths.
fn memset_entry(c: &mut Cpu) -> Flow<MemsetStage> {
    let n = c.g[A2][0];
    c.cycles += 3;
    c.g[V0][0] = (n < 8) as u64;
    c.g[V1][0] = c.g[A0][0];
    if n < 8 {
        return Flow::Go(MemsetStage::Bytes { resume: false });
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0] & 0xf;
    c.g[A3][0] = c.g[A0][0];
    if c.g[V0][0] != 0 {
        return Flow::Go(MemsetStage::Bytes { resume: false });
    }
    c.cycles += 8;
    let byte = c.g[A1][0] & 0xff;
    c.g[T1][0] = byte;
    c.g[T2][0] = (n < 0x20) as u64;
    c.g[T0][0] = byte << 8 | byte;
    // pcpyh v1, t0: each 64-bit lane gets its lowest halfword four times
    // (the upper lane comes from t0's stale upper half).
    let rep = |x: u64| (x & 0xffff) * 0x0001_0001_0001_0001;
    c.g[V1] = [rep(c.g[T0][0]), rep(c.g[T0][1])];
    c.g[V0][0] = (n < 8) as u64;
    if n < 0x20 {
        return Flow::Go(MemsetStage::Doubles { resume: false });
    }
    c.cycles += 1;
    // pcpyld t0, v1, v1: the 8-byte pattern in both lanes.
    c.g[T0] = [c.g[V1][0], c.g[V1][0]];
    Flow::Go(MemsetStage::Quads)
}

/// 32 bytes per iteration (two `sq` of t0) while `n >= 32`.
fn memset_quads(c: &mut Cpu) -> Flow<MemsetStage> {
    let (mut dst, mut n) = (r32(c, A3), r32(c, A2));
    loop {
        // The loop state lives in the registers (re-read after an event).
        let pattern = q(c, T0);
        c.cycles += 7;
        c.bus.write128(dst, pattern);
        n = n.wrapping_sub(0x20);
        dst = dst.wrapping_add(0x10);
        let done = n < 0x20;
        c.bus.write128(dst, pattern);
        dst = dst.wrapping_add(0x10);
        c.g[V0][0] = done as u64;
        set32(c, A2, n);
        set32(c, A3, dst);
        if done {
            break;
        }
        if back_edge_unwinds(c, MEMSET_QUADS) {
            return Flow::Unwind;
        }
        (dst, n) = (r32(c, A3), r32(c, A2));
    }
    c.cycles += 2;
    c.g[V0][0] = (n < 8) as u64;
    Flow::Go(MemsetStage::Doubles { resume: false })
}

/// 8 bytes per iteration (`sd` of v1) while `n >= 8`; enters with the
/// `n < 8` flag in v0.
fn memset_doubles(c: &mut Cpu, resume: bool) -> Flow<MemsetStage> {
    let (mut dst, mut n) = (r32(c, A3), r32(c, A2));
    let mut store = !resume;
    loop {
        if store {
            c.cycles += 4;
            if c.g[V0][0] != 0 {
                break;
            }
            c.sd(dst, c.g[V1][0]);
            if back_edge_unwinds(c, MEMSET_DOUBLE_STEP) {
                return Flow::Unwind;
            }
            (dst, n) = (r32(c, A3), r32(c, A2));
        }
        store = true;
        c.cycles += 3;
        n = n.wrapping_sub(8);
        dst = dst.wrapping_add(8);
        c.g[V0][0] = (n < 8) as u64;
        set32(c, A2, n);
        set32(c, A3, dst);
    }
    c.cycles += 1;
    c.g[V1][0] = c.g[A3][0];
    Flow::Go(MemsetStage::Bytes { resume: false })
}

/// The byte tail: `n` counts down to -1 (kept in v0); the pointer is v1.
fn memset_bytes(c: &mut Cpu, resume: bool) -> Flow<MemsetStage> {
    if !resume {
        c.cycles += 5;
        c.g[V0][0] = u64::MAX;
        set32(c, A2, r32(c, A2).wrapping_sub(1));
        if r32(c, A2) == u32::MAX {
            return Flow::Done;
        }
        c.cycles += 2;
    }
    let (mut p, mut n) = (r32(c, V1), r32(c, A2));
    loop {
        c.cycles += 7;
        c.sb(p, c.g[A1][0]);
        n = n.wrapping_sub(1);
        p = p.wrapping_add(1);
        set32(c, A2, n);
        set32(c, V1, p);
        if n == u32::MAX {
            break;
        }
        if back_edge_unwinds(c, MEMSET_BYTES) {
            return Flow::Unwind;
        }
        (p, n) = (r32(c, V1), r32(c, A2));
    }
    Flow::Done
}

const EA_MEM_SET: u32 = 0x003e_6448;
const EA_MEM_SET_END: u32 = 0x003e_6574;
/// Resume points (loop heads with an event check).
const EA_MEM_SET_BLOCKS: u32 = 0x003e_64c8;
const EA_MEM_SET_WORDS16: u32 = 0x003e_6508;
const EA_MEM_SET_WORD_STEP: u32 = 0x003e_6530;

#[derive(Clone, Copy, PartialEq, Debug)]
enum EaMemSetStage {
    Entry,
    /// 64 bytes per iteration (eight `sd` of the doubled word in v1).
    Blocks,
    /// 16 bytes per iteration (four `sw`).
    Words16,
    /// 4 bytes per iteration; `resume` enters at the count step after a
    /// store (0x3e6530).
    Words4 { resume: bool },
}

/// The signed count in a2 (the original compares the whole register).
#[inline]
fn count(c: &Cpu) -> i64 {
    c.g[A2][0] as i64
}

/// `a2 += k`, `a0 += step` (32-bit adds, as `addiu` leaves them).
#[inline]
fn advance(c: &mut Cpu, k: i32, step: u32) {
    set32(c, A2, r32(c, A2).wrapping_add(k as u32));
    set32(c, A0, r32(c, A0).wrapping_add(step));
}

/// `EA_MemSet(dst, word, n)` (0x3e6448): fills `n` bytes (a signed count)
/// with the 32-bit pattern `word`, no return value.
///
/// It aligns `dst` with at most one byte, one halfword and one word store
/// (each only if enough bytes remain), then, if at least 64 bytes remain,
/// stores 64 bytes per iteration from a doubleword built on the stack (the
/// word written twice below `sp` and read back with `ld`), then 16 bytes,
/// then 4 bytes per iteration, and finishes with a halfword and a byte.
/// The stores take the low bytes of `word` regardless of the address.
///
/// Like `memset` it is a `match pc` dispatcher whose loops return on an
/// unwind request, so its loop heads are entry points too.
pub fn ea_mem_set(c: &mut Cpu) {
    let mut stage = match c.pc {
        EA_MEM_SET => EaMemSetStage::Entry,
        EA_MEM_SET_BLOCKS => EaMemSetStage::Blocks,
        EA_MEM_SET_WORDS16 => EaMemSetStage::Words16,
        EA_MEM_SET_WORD_STEP => EaMemSetStage::Words4 { resume: true },
        pc => {
            c.jump_out(pc, EA_MEM_SET, EA_MEM_SET_END);
            return;
        }
    };
    loop {
        let flow = match stage {
            EaMemSetStage::Entry => ea_mem_set_entry(c),
            EaMemSetStage::Blocks => ea_mem_set_blocks(c),
            EaMemSetStage::Words16 => ea_mem_set_words16(c),
            EaMemSetStage::Words4 { resume } => ea_mem_set_words4(c, resume),
        };
        match flow {
            Flow::Go(s) => stage = s,
            Flow::Done => {
                ea_mem_set_tail(c);
                return;
            }
            Flow::Unwind => return,
        }
    }
}

/// One alignment step: if the previous size test (v0) passed and `dst` is
/// misaligned at `width`, store `width` bytes. Leaves v0 = `n < next`.
fn ea_mem_set_align(c: &mut Cpu, width: u32, next: i64) {
    c.cycles += 2;
    let enough = c.g[V0][0] == 0;
    c.g[V0][0] = (count(c) < next) as u64;
    if !enough {
        return;
    }
    c.cycles += 3;
    let misaligned = c.g[A0][0] & width as u64;
    c.g[V0][0] = (count(c) < next) as u64;
    if misaligned == 0 {
        return;
    }
    c.cycles += 4;
    match width {
        2 => c.sh(r32(c, A0), c.g[A1][0]),
        _ => c.sw(r32(c, A0), c.g[A1][0]),
    }
    advance(c, -(width as i32), width);
    c.g[V0][0] = (count(c) < next) as u64;
}

/// Reserve the stack slot, align `dst`, and pick the widest loop.
fn ea_mem_set_entry(c: &mut Cpu) -> Flow<EaMemSetStage> {
    c.cycles += 2;
    set32(c, SP, r32(c, SP).wrapping_sub(0x10));
    // A byte if n > 0 and dst is odd. Leaves v0 = n < 2.
    if count(c) <= 0 {
        c.cycles += 1;
        c.g[V0][0] = (count(c) < 2) as u64;
    } else {
        c.cycles += 3;
        c.g[V0][0] = (count(c) < 2) as u64;
        if c.g[A0][0] & 1 != 0 {
            c.cycles += 3;
            c.sb(r32(c, A0), c.g[A1][0]);
            advance(c, -1, 1);
            c.cycles += 1;
            c.g[V0][0] = (count(c) < 2) as u64;
        }
    }
    // A halfword, then a word.
    ea_mem_set_align(c, 2, 4);
    ea_mem_set_align(c, 4, 64);
    // The 64-byte loop needs at least 64 bytes (v0 = n < 64).
    c.cycles += 2;
    let blocks = c.g[V0][0] == 0;
    c.g[V0][0] = (count(c) < 16) as u64;
    if !blocks {
        return ea_mem_set_words16_check(c);
    }
    // Double the word through the stack slot: sw sp+4, sw sp, ld sp.
    c.cycles += 3;
    let sp = r32(c, SP);
    c.sw(sp.wrapping_add(4), c.g[A1][0]);
    c.sw(sp, c.g[A1][0]);
    c.g[V1][0] = c.ld(sp);
    Flow::Go(EaMemSetStage::Blocks)
}

/// 64 bytes per iteration while `n >= 64`.
fn ea_mem_set_blocks(c: &mut Cpu) -> Flow<EaMemSetStage> {
    loop {
        c.cycles += 12;
        let (dst, pattern) = (r32(c, A0), c.g[V1][0]);
        for i in 0..8 {
            c.sd(dst.wrapping_add(8 * i), pattern);
        }
        advance(c, -0x40, 0x40);
        let done = count(c) < 64;
        c.g[V0][0] = done as u64;
        if done {
            break;
        }
        if back_edge_unwinds(c, EA_MEM_SET_BLOCKS) {
            return Flow::Unwind;
        }
    }
    c.cycles += 1;
    c.g[V0][0] = (count(c) < 16) as u64;
    ea_mem_set_words16_check(c)
}

/// Enter the 16-byte loop if at least 16 bytes remain (v0 = n < 16).
fn ea_mem_set_words16_check(c: &mut Cpu) -> Flow<EaMemSetStage> {
    c.cycles += 2;
    let enough = c.g[V0][0] == 0;
    c.g[V0][0] = (count(c) < 4) as u64;
    if !enough {
        return Flow::Go(EaMemSetStage::Words4 { resume: false });
    }
    c.cycles += 1;
    Flow::Go(EaMemSetStage::Words16)
}

/// 16 bytes per iteration (four `sw`) while `n >= 16`.
fn ea_mem_set_words16(c: &mut Cpu) -> Flow<EaMemSetStage> {
    loop {
        c.cycles += 8;
        let dst = r32(c, A0);
        for i in 0..4 {
            c.sw(dst.wrapping_add(4 * i), c.g[A1][0]);
        }
        advance(c, -0x10, 0x10);
        let done = count(c) < 16;
        c.g[V0][0] = done as u64;
        if done {
            break;
        }
        if back_edge_unwinds(c, EA_MEM_SET_WORDS16) {
            return Flow::Unwind;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = (count(c) < 4) as u64;
    Flow::Go(EaMemSetStage::Words4 { resume: false })
}

/// 4 bytes per iteration while `n >= 4` (enters with v0 = n < 4).
fn ea_mem_set_words4(c: &mut Cpu, resume: bool) -> Flow<EaMemSetStage> {
    let mut store = !resume;
    loop {
        if store {
            c.cycles += 4;
            if c.g[V0][0] != 0 {
                break;
            }
            c.sw(r32(c, A0), c.g[A1][0]);
            if back_edge_unwinds(c, EA_MEM_SET_WORD_STEP) {
                return Flow::Unwind;
            }
        }
        store = true;
        c.cycles += 3;
        advance(c, -4, 4);
        c.g[V0][0] = (count(c) < 4) as u64;
    }
    Flow::Done
}

/// A halfword and a byte if they remain; free the stack slot and return.
fn ea_mem_set_tail(c: &mut Cpu) {
    c.cycles += 3;
    let lt2 = count(c) < 2;
    c.g[V0][0] = lt2 as u64;
    if !lt2 {
        c.cycles += 3;
        c.sh(r32(c, A0), c.g[A1][0]);
        advance(c, -2, 2);
    }
    c.cycles += 2;
    if count(c) > 0 {
        c.sb(r32(c, A0), c.g[A1][0]);
    }
    c.cycles += 2;
    set32(c, SP, r32(c, SP).wrapping_add(0x10));
    ret(c);
}

// ---------------------------------------------------------------------------
// Scratchpad DMA (channels 8 = fromSPR at 0x1000d000, 9 = toSPR at 0x1000d400).

/// `CHCR.STR`: the channel is busy.
const CHCR_STR: u64 = 0x100;
/// The `flags` argument: wait for the channel before starting / after.
const SPR_WAIT_BEFORE: u64 = 4;
const SPR_WAIT_AFTER: u64 = 2;

/// One scratchpad DMA channel's registers and the loop heads of its
/// function's two busy-wait loops.
struct SprChannel {
    chcr: u32,
    madr: u32,
    qwc: u32,
    sadr: u32,
    wait_before_head: u32,
    wait_after_head: u32,
}

/// Spin while `CHCR.STR` is set (v1 = the CHCR address, v0 = the busy
/// bit). The poll goes through `Cpu::lw`, so the DMA model sees the same
/// cycle counts as with the original; the iteration count is whatever the
/// model makes it.
fn spr_spin(c: &mut Cpu, ch: &SprChannel, head: u32) {
    c.g[V1][0] = ch.chcr as u64;
    loop {
        c.cycles += 7;
        let busy = c.lw(ch.chcr) & CHCR_STR;
        c.g[V0][0] = busy;
        if busy == 0 {
            return;
        }
        if event_due(c) {
            deliver_event(c, head);
        }
    }
}

/// The shared body of `Dma_ToScratchpad` / `Dma_FromScratchpad` after the
/// first block: optionally wait for the channel, program MADR/SADR/QWC,
/// start it, optionally wait for completion.
///
/// `mem` is the main-memory address and `spr` the scratchpad offset; the
/// original keeps `spr & 0x3fff` in a0.
fn spr_transfer(c: &mut Cpu, ch: &SprChannel, wait_cycles: (u64, u64), mem: u64, spr: u64) {
    if c.g[A3][0] & SPR_WAIT_BEFORE != 0 {
        c.cycles += wait_cycles.0;
        let busy = c.lw(ch.chcr) & CHCR_STR;
        c.g[V0][0] = ch.chcr as u64;
        c.g[V1][0] = 0x1000_0000;
        if busy != 0 {
            c.cycles += wait_cycles.1;
            spr_spin(c, ch, ch.wait_before_head);
        }
    }
    c.cycles += 17;
    c.g[A0][0] = spr & 0x3fff;
    c.sw(ch.madr, mem);
    c.sw(ch.sadr, c.g[A0][0]);
    c.sw(ch.qwc, c.g[A2][0]);
    c.sw(ch.chcr, CHCR_STR);
    c.g[V0][0] = CHCR_STR;
    c.g[V1][0] = ch.chcr as u64;
    c.g[A0][0] = c.g[A3][0] & SPR_WAIT_AFTER;
    if c.g[A0][0] != 0 {
        c.cycles += 5;
        let busy = c.lw(ch.chcr) & CHCR_STR;
        c.g[V0][0] = busy;
        c.g[V1][0] = 0x1000_0000;
        if busy != 0 {
            c.cycles += 1;
            spr_spin(c, ch, ch.wait_after_head);
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `Dma_ToScratchpad(mem, spr, qwc, flags)` (0x371d10): copies `qwc`
/// quadwords from main memory to scratchpad offset `spr` with DMA channel 9.
/// `flags & 4`: first wait until the channel is idle; `flags & 2`: wait for
/// the transfer to finish.
pub fn dma_to_scratchpad(c: &mut Cpu) {
    const CH: SprChannel = SprChannel {
        chcr: 0x1000_d400,
        madr: 0x1000_d410,
        qwc: 0x1000_d420,
        sadr: 0x1000_d480,
        wait_before_head: 0x0037_1d38,
        wait_after_head: 0x0037_1db0,
    };
    c.cycles += 3;
    c.g[V0][0] = c.g[A3][0] & SPR_WAIT_BEFORE;
    c.g[T0][0] = c.g[A0][0];
    spr_transfer(c, &CH, (6, 1), c.g[T0][0], c.g[A1][0]);
}

/// `Dma_FromScratchpad(spr, mem, qwc, flags)` (0x371dd8): copies `qwc`
/// quadwords from scratchpad offset `spr` to main memory with DMA channel 8.
/// Flags as for [`dma_to_scratchpad`].
pub fn dma_from_scratchpad(c: &mut Cpu) {
    const CH: SprChannel = SprChannel {
        chcr: 0x1000_d000,
        madr: 0x1000_d010,
        qwc: 0x1000_d020,
        sadr: 0x1000_d080,
        wait_before_head: 0x0037_1e00,
        wait_after_head: 0x0037_1e78,
    };
    c.cycles += 3;
    c.g[V0][0] = 0x1000_0000;
    spr_transfer(c, &CH, (5, 2), c.g[A1][0], c.g[A0][0]);
}

// ---------------------------------------------------------------------------
// Strings: newlib's word-at-a-time scans.
//
// A byte is zero iff the high bit of `(b - 1) & !b` is set; newlib tests
// 8 bytes at once with `(x - 0x0101..01) & !x & 0x8080..80` (a 64-bit
// subtraction, whose borrows can only cause false positives above a real
// zero byte) and 16 bytes at once with the same formula in MMI lanes
// (`psubb`: per-byte, no borrows). A hit only says "a zero is somewhere in
// here"; the exact position is then found by a byte loop.

/// 0x0101..01 and 0x8080..80 as the original builds them in a register.
const ONES: u64 = 0x0101_0101_0101_0101;
const HIGHS: u64 = 0x8080_8080_8080_8080;

/// A 128-bit GPR image (lane 0 first), as MMI instructions see it.
type Gpr = [u64; 2];

/// `psubb`: per-byte wrapping subtraction.
fn psubb(a: Gpr, b: Gpr) -> Gpr {
    let (x, y) = (gpr_bytes(a), gpr_bytes(b));
    let mut o = [0u8; 16];
    for i in 0..16 {
        o[i] = x[i].wrapping_sub(y[i]);
    }
    let v = u128::from_le_bytes(o);
    [v as u64, (v >> 64) as u64]
}

fn gpr_bytes(a: Gpr) -> [u8; 16] {
    (a[0] as u128 | (a[1] as u128) << 64).to_le_bytes()
}

/// `psubw`: per-word wrapping subtraction.
fn psubw(a: Gpr, b: Gpr) -> Gpr {
    let w = |x: u64, y: u64| {
        ((x as u32).wrapping_sub(y as u32) as u64)
            | (((x >> 32) as u32).wrapping_sub((y >> 32) as u32) as u64) << 32
    };
    [w(a[0], b[0]), w(a[1], b[1])]
}

/// `pand`.
fn pand(a: Gpr, b: Gpr) -> Gpr {
    [a[0] & b[0], a[1] & b[1]]
}

/// `pnor rd, $zero, x`: the complement.
fn pnot(a: Gpr) -> Gpr {
    [!a[0], !a[1]]
}

/// `pcpyld rd, rs, rt`: lower lanes, rt's below rs's.
fn pcpyld(rs: Gpr, rt: Gpr) -> Gpr {
    [rt[0], rs[0]]
}

/// `pcpyud rd, rs, rt`: upper lanes, rs's below rt's.
fn pcpyud(rs: Gpr, rt: Gpr) -> Gpr {
    [rs[1], rt[1]]
}

/// `lq` as a register image.
fn lq(c: &mut Cpu, a: u32) -> Gpr {
    let v = c.bus.read128(a);
    [v as u64, (v >> 64) as u64]
}

/// `strlen(s)` (0x416810, newlib): the number of bytes before the NUL.
///
/// From an 8-byte aligned `s` it scans 16 bytes per iteration (MMI) when
/// `s` is 16-byte aligned, else 8 bytes per iteration, until a block holds a
/// zero byte; the final position is found byte by byte. Unaligned strings
/// are scanned byte by byte from the start.
pub fn strlen(c: &mut Cpu) {
    const QUAD_HEAD: u32 = 0x0041_6890;
    const DOUBLE_HEAD: u32 = 0x0041_6914;
    const BYTE_HEAD: u32 = 0x0041_6934;
    let s = c.g[A0][0];
    c.cycles += 3;
    c.g[V0][0] = s & 7;
    c.g[A3][0] = s;
    // Each wide path ends with a0 at the block that holds the NUL.
    'bytes: {
        if s & 7 != 0 {
            break 'bytes;
        }
        c.cycles += 10;
        c.g[V1][0] = s & 0xf;
        c.g[V0][0] = ONES;
        c.g[A1][0] = s;
        if s & 0xf == 0 {
            // The first 16 bytes.
            c.cycles += 18;
            let x = lq(c, s as u32);
            c.g[T0] = pcpyld(c.g[V0], c.g[V0]);
            c.g[A0][0] = HIGHS;
            let zeros = pand(psubb(x, c.g[T0]), pnot(x));
            c.g[T1] = pcpyld(c.g[A0], c.g[A0]);
            c.g[V0] = pand(zeros, c.g[T1]);
            c.g[V1] = pcpyud(c.g[V0], c.g[T0]);
            c.g[A2][0] = c.g[V1][0] | c.g[V0][0];
            if c.g[A2][0] != 0 {
                c.g[A0][0] = c.g[A1][0];
                break 'bytes;
            }
            // 16 bytes per iteration.
            c.cycles += 1;
            set32(c, A1, r32(c, A1).wrapping_add(0x10));
            loop {
                c.cycles += 9;
                let x = lq(c, r32(c, A1));
                c.g[V1] = pnot(x);
                c.g[V0] = pand(psubb(x, c.g[T0]), c.g[V1]);
                c.g[A0] = pand(c.g[V0], c.g[T1]);
                c.g[V1] = pcpyud(c.g[A0], c.g[A2]);
                c.g[V1][0] |= c.g[A0][0];
                if c.g[V1][0] != 0 {
                    break;
                }
                set32(c, A1, r32(c, A1).wrapping_add(0x10));
                if event_due(c) {
                    deliver_event(c, QUAD_HEAD);
                }
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[A1][0];
            break 'bytes;
        }
        // The first 8 bytes.
        c.cycles += 14;
        let x = c.ld(r32(c, A1));
        c.g[A0][0] = HIGHS;
        c.g[V1][0] = !x;
        c.g[V0][0] = x.wrapping_sub(c.g[V0][0]) & !x & HIGHS;
        if c.g[V0][0] != 0 {
            c.g[A0][0] = c.g[A1][0];
            break 'bytes;
        }
        // 8 bytes per iteration.
        c.cycles += 8;
        c.g[A2][0] = ONES;
        set32(c, A1, r32(c, A1).wrapping_add(8));
        loop {
            c.cycles += 7;
            let x = c.ld(r32(c, A1));
            c.g[V1][0] = !x;
            c.g[V0][0] = x.wrapping_sub(c.g[A2][0]) & !x & c.g[A0][0];
            if c.g[V0][0] != 0 {
                break;
            }
            set32(c, A1, r32(c, A1).wrapping_add(8));
            if event_due(c) {
                deliver_event(c, DOUBLE_HEAD);
            }
        }
        c.cycles += 1;
        c.g[A0][0] = c.g[A1][0];
    }
    // Byte by byte to the NUL.
    loop {
        c.cycles += 7;
        c.g[V0][0] = c.lb(r32(c, A0));
        if c.g[V0][0] == 0 {
            break;
        }
        set32(c, A0, r32(c, A0).wrapping_add(1));
        if event_due(c) {
            deliver_event(c, BYTE_HEAD);
        }
    }
    c.cycles += 2;
    set32(c, V0, r32(c, A0).wrapping_sub(r32(c, A3)));
    ret(c);
}

/// `addiu r, r, k` on a register.
#[inline]
fn bump(c: &mut Cpu, r: usize, k: i32) {
    set32(c, r, r32(c, r).wrapping_add(k as u32));
}

/// `strcpy(dst, src)` (0x4166f4, newlib): copies `src` with its NUL and
/// returns `dst`.
///
/// When both pointers are 8-byte aligned it copies whole blocks that hold
/// no zero byte: 16 bytes per iteration (MMI test, `sq`) if both are 16-byte
/// aligned, else 8 bytes (`sd`). The block with the NUL, and unaligned
/// strings, go byte by byte. A block is stored only after it was tested.
pub fn strcpy(c: &mut Cpu) {
    const QUAD_HEAD: u32 = 0x0041_6778;
    const DOUBLE_HEAD: u32 = 0x0041_67c4;
    const BYTE_HEAD: u32 = 0x0041_67ec;
    c.cycles += 5;
    c.g[A3][0] = c.g[A0][0];
    c.g[T0][0] = c.g[A1][0] | c.g[A3][0];
    c.g[V0][0] = c.g[T0][0] & 7;
    c.g[V1][0] = c.g[A3][0];
    // The wide paths leave v1 = dst and a1 = src at the block with the NUL.
    'bytes: {
        if c.g[V0][0] != 0 {
            break 'bytes;
        }
        c.cycles += 17;
        c.g[V0][0] = c.g[T0][0] & 0xf;
        c.g[T1][0] = ONES;
        c.g[A0][0] = HIGHS;
        if c.g[V0][0] == 0 {
            // 16-byte blocks: t1 = the block, t2/t0 = the constants.
            c.cycles += 11;
            c.g[T2] = pcpyld(c.g[T1], c.g[T1]);
            c.g[T1] = lq(c, r32(c, A1));
            c.g[T0] = pcpyld(c.g[A0], c.g[A0]);
            c.g[V1] = pnot(c.g[T1]);
            c.g[V0] = pand(pand(psubb(c.g[T1], c.g[T2]), c.g[V1]), c.g[T0]);
            c.g[A0] = pcpyud(c.g[V0], c.g[T1]);
            c.g[V1][0] = c.g[V0][0] | c.g[A0][0];
            c.g[A2][0] = c.g[A3][0];
            if c.g[V1][0] == 0 {
                loop {
                    c.cycles += 11;
                    c.sq(T1, r32(c, A2));
                    bump(c, A1, 0x10);
                    c.g[T1] = lq(c, r32(c, A1));
                    c.g[V1] = pnot(c.g[T1]);
                    c.g[V0] = pand(pand(psubb(c.g[T1], c.g[T2]), c.g[V1]), c.g[T0]);
                    c.g[A0] = pcpyud(c.g[V0], c.g[T1]);
                    c.g[V1][0] = c.g[V0][0] | c.g[A0][0];
                    bump(c, A2, 0x10);
                    if c.g[V1][0] != 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event(c, QUAD_HEAD);
                    }
                }
                c.cycles += 2;
                c.g[V1][0] = c.g[A2][0];
                break 'bytes;
            }
        } else {
            // 8-byte blocks: t2 = the block.
            c.g[T2][0] = c.ld(r32(c, A1));
            c.cycles += 6;
            let x = c.g[T2][0];
            c.g[V1][0] = !x;
            c.g[V0][0] = x.wrapping_sub(c.g[T1][0]) & !x & c.g[A0][0];
            c.g[A2][0] = c.g[A3][0];
            if c.g[V0][0] == 0 {
                loop {
                    c.cycles += 9;
                    c.sd(r32(c, A2), c.g[T2][0]);
                    bump(c, A1, 8);
                    let x = c.ld(r32(c, A1));
                    c.g[T2][0] = x;
                    c.g[V0][0] = !x;
                    c.g[V1][0] = x.wrapping_sub(c.g[T1][0]) & !x & c.g[A0][0];
                    bump(c, A2, 8);
                    if c.g[V1][0] != 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event(c, DOUBLE_HEAD);
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[V1][0] = c.g[A2][0];
    }
    // Byte by byte, NUL included.
    loop {
        c.cycles += 7;
        let b = c.lbu(r32(c, A1));
        bump(c, A1, 1);
        c.sb(r32(c, V1), b);
        c.g[V0][0] = sx((b as u32) << 24);
        bump(c, V1, 1);
        if b == 0 {
            break;
        }
        if event_due(c) {
            deliver_event(c, BYTE_HEAD);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    ret(c);
}

/// `strncpy(dst, src, n)` (0x416ce0, newlib): copies at most `n` bytes of
/// `src`, then pads with zeros up to `n`; returns `dst`.
///
/// When both pointers are 8-byte aligned and enough bytes remain it copies
/// blocks that hold no zero byte: if both are 16-byte aligned and `n >= 16`
/// one 16-byte block first (see the note below), then 8 bytes per iteration
/// while `n >= 8`. The rest goes byte by byte, then the zero padding.
///
/// Note: the 16-byte test applies the zero-byte formula to the complemented
/// block (`(!x - 1) & !x`, i.e. it looks for 0xff bytes), so a first block
/// with a NUL can be copied whole; this port keeps that behaviour.
pub fn strncpy(c: &mut Cpu) {
    const DOUBLE_HEAD: u32 = 0x0041_6e20;
    const BYTE_HEAD: u32 = 0x0041_6e58;
    const PAD_HEAD: u32 = 0x0041_6e88;
    c.cycles += 7;
    c.g[T0][0] = c.g[A0][0];
    c.g[A3][0] = c.g[A1][0] | c.g[A0][0];
    c.g[T2][0] = 0x10;
    c.g[V0][0] = c.g[A3][0] & 7;
    c.g[T1][0] = 8;
    let unaligned = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[A3][0] & 0xf;
    // The wide paths leave a0 = dst, a1 = src for the byte loop.
    'bytes: {
        if unaligned {
            break 'bytes;
        }
        // Block size: 16 if both are 16-byte aligned, else 8.
        c.cycles += 3;
        if c.g[V0][0] == 0 {
            c.g[T1][0] = c.g[T2][0];
        }
        let quad = c.g[V0][0] == 0;
        c.g[V0][0] = (c.g[A2][0] < c.g[T1][0]) as u64;
        'doubles: {
            if quad {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    break 'bytes;
                }
                c.cycles += 25;
                c.g[A3][0] = ONES;
                c.g[V1] = lq(c, r32(c, A1));
                c.g[T1] = pcpyld(c.g[A3], c.g[A3]);
                c.g[V1] = pnot(c.g[V1]);
                c.g[A3][0] = HIGHS;
                c.g[V0] = psubb(c.g[V1], c.g[T1]);
                c.g[T2] = pcpyld(c.g[A3], c.g[A3]);
                c.g[V0] = pand(pand(c.g[V0], c.g[V1]), c.g[T2]);
                c.g[V1] = pcpyud(c.g[V0], c.g[A0]);
                c.g[V1][0] |= c.g[V0][0];
                c.g[A3][0] = c.g[T0][0];
                if c.g[V1][0] != 0 {
                    break 'doubles;
                }
                // Copy the first 16 bytes.
                c.cycles += 7;
                c.g[V1] = lq(c, r32(c, A1));
                bump(c, A2, -0x10);
                bump(c, A1, 0x10);
                c.g[V0][0] = (c.g[A2][0] < 0x10) as u64;
                c.sq(V1, r32(c, A3));
                bump(c, A3, 0x10);
                if c.g[V0][0] != 0 {
                    break 'doubles;
                }
                // Test the next 16 (correctly this time).
                c.cycles += 9;
                let x = lq(c, r32(c, A1));
                c.g[V1] = pnot(x);
                c.g[V0] = pand(pand(psubb(x, c.g[T1]), c.g[V1]), c.g[T2]);
                c.g[V1] = pcpyud(c.g[V0], c.g[A0]);
                c.g[V0][0] |= c.g[V1][0];
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[A3][0];
                    break 'bytes;
                }
                c.g[V1] = lq(c, r32(c, A1));
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    break 'bytes;
                }
                // Test the first 8 bytes.
                c.cycles += 21;
                let x = c.ld(r32(c, A1));
                c.g[T1][0] = ONES;
                c.g[T2][0] = HIGHS;
                c.g[V1][0] = !x;
                c.g[V0][0] = x.wrapping_sub(ONES) & !x & HIGHS;
                c.g[A3][0] = c.g[T0][0];
                if c.g[V0][0] != 0 {
                    break 'doubles;
                }
                c.cycles += 2;
                c.g[V1][0] = c.ld(r32(c, A1));
            }
            // 8 bytes per iteration: store the tested block (v1), test the next.
            loop {
                c.cycles += 6;
                bump(c, A2, -8);
                bump(c, A1, 8);
                c.g[V0][0] = (c.g[A2][0] < 8) as u64;
                c.sd(r32(c, A3), c.g[V1][0]);
                bump(c, A3, 8);
                if c.g[V0][0] != 0 {
                    break;
                }
                c.cycles += 7;
                let x = c.ld(r32(c, A1));
                c.g[V1][0] = !x;
                c.g[V0][0] = x.wrapping_sub(c.g[T1][0]) & !x & c.g[T2][0];
                if c.g[V0][0] != 0 {
                    break;
                }
                c.g[V1][0] = c.ld(r32(c, A1));
                if event_due(c) {
                    deliver_event(c, DOUBLE_HEAD);
                }
            }
        }
        c.cycles += 1;
        c.g[A0][0] = c.g[A3][0];
    }
    // Byte by byte until the NUL or n, then zeros up to n.
    loop {
        c.cycles += 2;
        c.g[V0][0] = c.g[A2][0];
        if c.g[A2][0] == 0 {
            break;
        }
        c.cycles += 7;
        let b = c.lbu(r32(c, A1));
        bump(c, A2, -1);
        bump(c, A1, 1);
        c.sb(r32(c, A0), b);
        c.g[V0][0] = sx((b as u32) << 24);
        bump(c, A0, 1);
        if b != 0 {
            if event_due(c) {
                deliver_event(c, BYTE_HEAD);
            }
            continue;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A2][0];
        bump(c, A2, -1);
        if c.g[V0][0] != 0 {
            loop {
                c.cycles += 7;
                c.sb(r32(c, A0), 0);
                c.g[V0][0] = c.g[A2][0];
                bump(c, A0, 1);
                bump(c, A2, -1);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event(c, PAD_HEAD);
                }
            }
        }
        break;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T0][0];
    ret(c);
}

const STRCMP: u32 = 0x0041_65a8;
const STRCMP_END: u32 = 0x0041_66f4;
/// Resume points (loop heads with an event check).
const STRCMP_QUADS: u32 = 0x0041_6628;
const STRCMP_DOUBLES: u32 = 0x0041_6688;
const STRCMP_BYTES: u32 = 0x0041_66c0;

#[derive(Clone, Copy, PartialEq, Debug)]
enum StrcmpStage {
    Entry,
    /// 16 bytes per iteration: v0 = the block of `a` just compared equal,
    /// v1 = its complement; looks for a NUL in it, then compares the next.
    Quads,
    /// The same with 8-byte blocks (v0 = the block, t0 = its complement).
    Doubles,
    /// Byte loop head: v0 = `*a` (signed); fetch it unsigned into v1.
    ByteTest,
    /// Compare `*a` (v1) with `*b`.
    Bytes,
}

/// `strcmp(a, b)` (0x4165a8, newlib): `*a - *b` (as unsigned bytes) at the
/// first difference, or 0.
///
/// When both pointers are 8-byte aligned it compares whole blocks (16 bytes
/// with `psubw` if both are 16-byte aligned, else 8 bytes) and stops with 0
/// when an equal block holds a NUL; at the first unequal block (or for
/// unaligned strings from the start) it continues byte by byte.
///
/// Like `memset` it is a `match pc` dispatcher whose loops return on an
/// unwind request, so its loop heads are entry points too.
pub fn strcmp(c: &mut Cpu) {
    let mut stage = match c.pc {
        STRCMP => StrcmpStage::Entry,
        STRCMP_QUADS => StrcmpStage::Quads,
        STRCMP_DOUBLES => StrcmpStage::Doubles,
        STRCMP_BYTES => StrcmpStage::Bytes,
        pc => {
            c.jump_out(pc, STRCMP, STRCMP_END);
            return;
        }
    };
    loop {
        let flow = match stage {
            StrcmpStage::Entry => strcmp_entry(c),
            StrcmpStage::Quads => strcmp_quads(c),
            StrcmpStage::Doubles => strcmp_doubles(c),
            StrcmpStage::ByteTest => strcmp_byte_test(c),
            StrcmpStage::Bytes => strcmp_bytes(c),
        };
        match flow {
            Flow::Go(s) => stage = s,
            Flow::Done | Flow::Unwind => return,
        }
    }
}

/// Return `v` (equal strings: 0).
fn strcmp_return(c: &mut Cpu, v: u64) -> Flow<StrcmpStage> {
    c.cycles += 2;
    c.g[V0][0] = v;
    ret(c);
    Flow::Done
}

/// Leave the block loops for the byte loop at `a` (v0 = `*a`, signed).
fn strcmp_to_bytes(c: &mut Cpu) -> Flow<StrcmpStage> {
    c.g[V0][0] = c.lb(r32(c, A0));
    Flow::Go(StrcmpStage::ByteTest)
}

/// Alignment tests; compares the first block.
fn strcmp_entry(c: &mut Cpu) -> Flow<StrcmpStage> {
    c.cycles += 4;
    c.g[T0][0] = c.g[A0][0] | c.g[A1][0];
    c.g[V0][0] = c.g[T0][0] & 7;
    if c.g[V0][0] != 0 {
        return strcmp_to_bytes(c);
    }
    c.cycles += 17;
    c.g[T1][0] = c.g[T0][0] & 0xf;
    c.g[A3][0] = ONES;
    c.g[A2][0] = HIGHS;
    c.g[V0][0] = c.ld(r32(c, A1));
    if c.g[T1][0] != 0 {
        // 8-byte blocks: compare the first.
        c.cycles += 3;
        c.g[V1][0] = c.ld(r32(c, A0));
        if c.g[V1][0] != c.g[V0][0] {
            return strcmp_to_bytes(c);
        }
        c.cycles += 2;
        c.g[V0][0] = c.ld(r32(c, A0));
        c.g[T0][0] = !c.g[V0][0];
        return Flow::Go(StrcmpStage::Doubles);
    }
    // 16-byte blocks: compare the first (a3 = word differences).
    c.cycles += 9;
    c.g[V1] = lq(c, r32(c, A0));
    c.g[T0] = pcpyld(c.g[A3], c.g[A3]);
    c.g[V0] = lq(c, r32(c, A1));
    c.g[T2] = pcpyld(c.g[A2], c.g[A2]);
    c.g[A3] = psubw(c.g[V0], c.g[V1]);
    c.g[A2] = pcpyud(c.g[A3], c.g[A0]);
    c.g[V1][0] = c.g[A2][0] | c.g[A3][0];
    if c.g[V1][0] != 0 {
        return strcmp_to_bytes(c);
    }
    c.cycles += 2;
    c.g[V0] = lq(c, r32(c, A0));
    c.g[V1] = pnot(c.g[V0]);
    Flow::Go(StrcmpStage::Quads)
}

fn strcmp_quads(c: &mut Cpu) -> Flow<StrcmpStage> {
    loop {
        // A NUL in the (equal) block ends the strings.
        c.cycles += 7;
        c.g[V0] = pand(pand(psubb(c.g[V0], c.g[T0]), c.g[V1]), c.g[T2]);
        c.g[V1] = pcpyud(c.g[V0], c.g[A0]);
        c.g[A2][0] = c.g[V1][0] | c.g[V0][0];
        bump(c, A0, 0x10);
        if c.g[A2][0] != 0 {
            return strcmp_return(c, 0);
        }
        // Compare the next blocks.
        c.cycles += 8;
        bump(c, A1, 0x10);
        c.g[V0] = lq(c, r32(c, A0));
        c.g[V1] = lq(c, r32(c, A1));
        c.g[A3] = psubw(c.g[V0], c.g[V1]);
        c.g[A2] = pcpyud(c.g[A3], c.g[A0]);
        c.g[T1][0] = c.g[A2][0] | c.g[A3][0];
        if c.g[T1][0] != 0 {
            c.cycles += 2;
            return strcmp_to_bytes(c);
        }
        c.g[V1] = pnot(c.g[V0]);
        if back_edge_unwinds(c, STRCMP_QUADS) {
            return Flow::Unwind;
        }
    }
}

fn strcmp_doubles(c: &mut Cpu) -> Flow<StrcmpStage> {
    loop {
        c.cycles += 5;
        c.g[V0][0] = c.g[V0][0].wrapping_sub(c.g[A3][0]) & c.g[T0][0] & c.g[A2][0];
        bump(c, A0, 8);
        if c.g[V0][0] != 0 {
            return strcmp_return(c, 0);
        }
        c.cycles += 5;
        bump(c, A1, 8);
        c.g[V0][0] = c.ld(r32(c, A0));
        c.g[V1][0] = c.ld(r32(c, A1));
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 2;
            return strcmp_to_bytes(c);
        }
        c.g[T0][0] = !c.g[V0][0];
        if back_edge_unwinds(c, STRCMP_DOUBLES) {
            return Flow::Unwind;
        }
    }
}

fn strcmp_byte_test(c: &mut Cpu) -> Flow<StrcmpStage> {
    c.cycles += 2;
    c.g[V1][0] = c.lbu(r32(c, A0));
    if c.g[V0][0] == 0 {
        return strcmp_diff(c);
    }
    if back_edge_unwinds(c, STRCMP_BYTES) {
        return Flow::Unwind;
    }
    Flow::Go(StrcmpStage::Bytes)
}

fn strcmp_bytes(c: &mut Cpu) -> Flow<StrcmpStage> {
    c.cycles += 5;
    // `*a` sign-extended (sll 24, sra 24) against `*b` loaded signed.
    let a = c.g[V1][0] as u8 as i8 as i64 as u64;
    c.g[V1][0] = c.lb(r32(c, A1));
    c.g[V0][0] = a;
    if c.g[V0][0] != c.g[V1][0] {
        c.g[V1][0] = c.lbu(r32(c, A0));
        return strcmp_diff(c);
    }
    c.cycles += 3;
    bump(c, A0, 1);
    bump(c, A1, 1);
    c.g[V0][0] = c.lb(r32(c, A0));
    Flow::Go(StrcmpStage::ByteTest)
}

/// `*a - *b` as unsigned bytes (v1 = `*a`).
fn strcmp_diff(c: &mut Cpu) -> Flow<StrcmpStage> {
    c.cycles += 3;
    c.g[V0][0] = c.lbu(r32(c, A1));
    set32(c, V0, r32(c, V1).wrapping_sub(r32(c, V0)));
    ret(c);
    Flow::Done
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{compare, fill, fresh, scramble, Rng, DATA, RA};

    /// The `UNWIND` bit the kernel sets in `pc`.
    const UNWIND: u32 = 1;

    thread_local! {
        /// Re-entries at a resume point made by [`drive`] (this test thread).
        static RESUMES: std::cell::RefCell<std::collections::BTreeMap<u32, u32>> =
            const { std::cell::RefCell::new(std::collections::BTreeMap::new()) };
    }

    /// Run `f` the way the trampoline does: after an unwind, deliver any due
    /// event and re-enter the function at the (cleared) `pc` until it returns.
    fn drive(c: &mut Cpu, f: GuestFn) {
        f(c);
        for _ in 0..10_000 {
            if c.pc == RA {
                return;
            }
            assert!(c.pc & UNWIND != 0, "stopped at {:#x} without an unwind", c.pc);
            c.pc &= !UNWIND;
            if event_due(c) && c.event() {
                continue;
            }
            RESUMES.with(|r| *r.borrow_mut().entry(c.pc).or_default() += 1);
            f(c);
        }
        panic!("no return after 10000 re-entries (pc {:#x}, cycles {}, next {})", c.pc, c.cycles, c.next_event);
    }

    /// Wrap a function in [`drive`] (difftest takes plain `fn`s).
    macro_rules! driven {
        ($name:ident, $f:path) => {
            fn $name(c: &mut Cpu) {
                drive(c, $f)
            }
        };
    }

    /// How a seed exercises event delivery.
    #[derive(Clone, Copy, PartialEq, Debug)]
    enum Events {
        /// No event while the function runs.
        Off,
        /// The first back edge after `k` cycles delivers a VBlank, which
        /// requests an unwind (a frame is ready); later checks unwind too.
        Unwind,
        /// Events are delivered but never unwind (no VBlank due).
        Deliver,
    }

    /// Enter at `entry` on a running guest thread (so a delivered VBlank has
    /// a thread to come back to), with the cycle counter and event schedule
    /// for `seed`. Returns the event mode.
    fn schedule(c: &mut Cpu, r: &mut Rng, seed: u32, entry: u32) -> Events {
        let saved = (c.g[A0], c.g[A1], c.g[31]);
        ssx3_ps2::kernel::start_main_thread(c, entry, 0, 0);
        (c.g[A0], c.g[A1], c.g[31]) = saved;
        c.pc = entry;
        c.cycles = 1000 + (r.next_u32() % 5000) as u64;
        let k = (r.next_u32() % 48) as u64;
        match seed % 3 {
            0 => Events::Off,
            1 => {
                c.next_event = c.cycles + k;
                Events::Unwind
            }
            _ => {
                ssx3_ps2::platform::machine_of(&mut c.bus.plat).sched.next_vblank = 1 << 40;
                c.next_event = c.cycles + k;
                Events::Deliver
            }
        }
    }

    /// A length biased toward the edges of each chunking stage.
    fn pick_len(r: &mut Rng) -> u32 {
        match r.next_u32() % 10 {
            0 => 0,
            1 => 1,
            2 => r.next_u32() % 8,
            3 => r.next_u32() % 40,
            4 => 14 + r.next_u32() % 24,
            5 => [7, 8, 9, 15, 16, 17, 18, 31, 32, 33, 63, 64, 127, 128, 129][(r.next_u32() % 15) as usize],
            6 => r.next_u32() % 300,
            7 => 120 + r.next_u32() % 400,
            8 => r.next_u32() % 2000,
            _ => 3000 + r.next_u32() % 3000,
        }
    }

    /// Like `difftest::check` (and also requiring equal temporaries), and
    /// asserts that the event seeds really delivered an event inside the
    /// function often enough to mean something.
    fn check_events(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32) -> Events,
    ) {
        let (mut evented, mut event_seeds) = (0, 0);
        for seed in 0..seeds {
            let r = compare(generated, hand, &|c| {
                setup(c, seed);
            });
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            // Temporaries too: the replacements reproduce every register.
            assert!(r.soft.is_empty(), "seed {seed} (soft): {:#?}", r.soft);
            let mut c = fresh();
            let mode = setup(&mut c, seed);
            let first = c.next_event;
            generated(&mut c);
            if mode != Events::Off {
                event_seeds += 1;
                // Delivering an event reschedules the next one.
                if c.next_event != first {
                    evented += 1;
                }
            }
        }
        assert!(
            evented * 4 >= event_seeds,
            "only {evented} of {event_seeds} event seeds reached an event"
        );
        let resumes = RESUMES.with(|r| std::mem::take(&mut *r.borrow_mut()));
        eprintln!("{evented} of {event_seeds} event seeds delivered an event; resumes (pc: count): {resumes:x?}");
    }

    /// Random bytes in the test area, a source and a destination (possibly
    /// overlapping, possibly unaligned) and a length.
    fn copy_setup(c: &mut Cpu, r: &mut Rng, allow_overlap: bool) -> (u32, u32, u32) {
        fill(c, r, DATA, 0x6000);
        let n = pick_len(r);
        let aligned = r.next_u32() % 2 == 0;
        let off = |r: &mut Rng| if aligned { 0 } else { r.next_u32() % 16 };
        let src = DATA + 0x100 + 0x10 * (r.next_u32() % 16) + off(r);
        let dst = if allow_overlap && r.next_u32() % 3 == 0 {
            let d = (r.next_u32() % 81) as i32 - 40;
            let d = if aligned { d & !15 } else { d };
            src.wrapping_add(d as u32)
        } else {
            DATA + 0x2000 + 0x10 * (r.next_u32() % 16) + off(r)
        };
        (dst, src, n)
    }

    driven!(gen_memcpy, ssx3_ee::s15::f_0041605c__generated);
    driven!(hand_memcpy, memcpy);
    driven!(gen_ea_mem_copy, ssx3_ee::s14::f_003e6574__generated);
    driven!(hand_ea_mem_copy, ea_mem_copy);

    #[test]
    fn memcpy_matches() {
        check_events(gen_memcpy, hand_memcpy, 600, &|c, seed| {
            let mut r = Rng(0x6d65_6d63_7079 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            let (dst, src, n) = copy_setup(c, &mut r, true);
            c.g[A0][0] = sx(dst);
            c.g[A1][0] = sx(src);
            c.g[A2][0] = sx(n);
            schedule(c, &mut r, seed, 0x0041_605c)
        });
    }

    #[test]
    fn ea_mem_copy_matches() {
        check_events(gen_ea_mem_copy, hand_ea_mem_copy, 600, &|c, seed| {
            let mut r = Rng(0x4541_636f_7079 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            let (dst, src, n) = copy_setup(c, &mut r, true);
            c.g[A0][0] = sx(dst);
            c.g[A1][0] = sx(src);
            // Negative counts copy nothing.
            let n = if r.next_u32() % 20 == 0 { 0u32.wrapping_sub(r.next_u32() % 5) } else { n };
            c.g[A2][0] = sx(n);
            schedule(c, &mut r, seed, 0x003e_6574)
        });
    }

    /// A UTF-16 string of `len` non-zero units at `at`, then the terminator.
    fn put_ustr(c: &mut Cpu, r: &mut Rng, at: u32, len: u32) {
        for i in 0..len {
            c.bus.write16(at + 2 * i, (r.next_u32() as u16).max(1));
        }
        c.bus.write16(at + 2 * len, 0);
    }

    driven!(gen_ustr_len, ssx3_ee::s14::f_003dbca0__generated);
    driven!(hand_ustr_len, ustr_len);
    driven!(gen_ustr_copy, ssx3_ee::s08::f_002c2508__generated);
    driven!(hand_ustr_copy, ustr_copy);

    #[test]
    fn ustr_len_matches() {
        check_events(gen_ustr_len, hand_ustr_len, 300, &|c, seed| {
            let mut r = Rng(0x7573_6c65_6e ^ (seed as u64) << 20);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x2000);
            let at = DATA + 2 * (r.next_u32() % 64);
            let len = pick_len(&mut r) % 3000;
            put_ustr(c, &mut r, at, len);
            c.g[A0][0] = sx(at);
            schedule(c, &mut r, seed, 0x003d_bca0)
        });
    }

    #[test]
    fn ustr_copy_matches() {
        check_events(gen_ustr_copy, hand_ustr_copy, 300, &|c, seed| {
            let mut r = Rng(0x7573_6370_79 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x4000);
            let src = DATA + 2 * (r.next_u32() % 64);
            let dst = DATA + 0x2000 + 2 * (r.next_u32() % 64);
            let len = pick_len(&mut r) % 3000;
            put_ustr(c, &mut r, src, len);
            c.g[A0][0] = sx(dst);
            c.g[A1][0] = sx(src);
            schedule(c, &mut r, seed, 0x002c_2508)
        });
    }

    driven!(gen_memset, ssx3_ee::s15::f_00416210__generated);
    driven!(hand_memset, memset);
    driven!(gen_ea_mem_set, ssx3_ee::s14::f_003e6448__generated);
    driven!(hand_ea_mem_set, ea_mem_set);

    #[test]
    fn memset_matches() {
        check_events(gen_memset, hand_memset, 600, &|c, seed| {
            let mut r = Rng(0x6d65_6d73_6574 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            let (dst, _, n) = copy_setup(c, &mut r, false);
            c.g[A0][0] = sx(dst);
            c.g[A2][0] = sx(n);
            schedule(c, &mut r, seed, 0x0041_6210)
        });
    }

    #[test]
    fn ea_mem_set_matches() {
        check_events(gen_ea_mem_set, hand_ea_mem_set, 600, &|c, seed| {
            let mut r = Rng(0x4541_7365_74 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            let (dst, _, n) = copy_setup(c, &mut r, false);
            let n = if r.next_u32() % 20 == 0 { 0u32.wrapping_sub(r.next_u32() % 5) } else { n };
            c.g[A0][0] = sx(dst);
            c.g[A2][0] = sx(n);
            schedule(c, &mut r, seed, 0x003e_6448)
        });
    }

    driven!(gen_dma_to_spr, ssx3_ee::s12::f_00371d10__generated);
    driven!(hand_dma_to_spr, dma_to_scratchpad);
    driven!(gen_dma_from_spr, ssx3_ee::s12::f_00371dd8__generated);
    driven!(hand_dma_from_spr, dma_from_scratchpad);

    /// Random RAM and scratchpad contents and transfer arguments
    /// (mem, spr offset, qwc, flags). Returns (main-memory address, SPR offset).
    fn spr_setup(c: &mut Cpu, r: &mut Rng) -> (u32, u32) {
        fill(c, r, DATA, 0x2000);
        for i in 0..0x1000 {
            c.bus.spr[i * 4..i * 4 + 4].copy_from_slice(&r.next_u32().to_le_bytes());
        }
        let mem = DATA + 0x10 * (r.next_u32() % 0x100);
        let spr = (r.next_u32() & 0xfff0) | (r.next_u32() % 2) << 14;
        c.g[A2][0] = sx(r.next_u32() % 64);
        c.g[A3][0] = sx(r.next_u32() % 8 | r.next_u32() & 0xff00);
        (mem, spr)
    }

    #[test]
    fn dma_to_scratchpad_matches() {
        for seed in 0..300 {
            let r = compare(gen_dma_to_spr, hand_dma_to_spr, &|c| {
                let mut r = Rng(0x746f_7370_72 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                let (mem, spr) = spr_setup(c, &mut r);
                c.g[A0][0] = sx(mem);
                c.g[A1][0] = sx(spr);
                schedule(c, &mut r, seed, 0x0037_1d10);
            });
            assert!(r.hard.is_empty() && r.soft.is_empty(), "seed {seed}: {r:#?}");
        }
    }

    #[test]
    fn dma_from_scratchpad_matches() {
        for seed in 0..300 {
            let r = compare(gen_dma_from_spr, hand_dma_from_spr, &|c| {
                let mut r = Rng(0x6672_7370_72 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                let (mem, spr) = spr_setup(c, &mut r);
                c.g[A0][0] = sx(spr);
                c.g[A1][0] = sx(mem);
                schedule(c, &mut r, seed, 0x0037_1dd8);
            });
            assert!(r.hard.is_empty() && r.soft.is_empty(), "seed {seed}: {r:#?}");
        }
    }

    /// A string of `len` non-zero bytes (biased toward 0x01, 0x7f, 0x80, 0xff,
    /// the edges of the zero-byte tests) at `at`, then the NUL.
    fn put_str(c: &mut Cpu, r: &mut Rng, at: u32, len: u32) {
        for i in 0..len {
            let b = match r.next_u32() % 6 {
                0 => [1u8, 0x7f, 0x80, 0xff, 0xfe, 2][(r.next_u32() % 6) as usize],
                _ => (r.next_u32() as u8).max(1),
            };
            c.bus.write8(at + i, b);
        }
        c.bus.write8(at + len, 0);
    }

    /// Two addresses in separate areas with the same, random or 8/16-byte
    /// alignment.
    fn str_ptrs(r: &mut Rng) -> (u32, u32) {
        let (a, b) = (DATA + 0x10 * (r.next_u32() % 32), DATA + 0x3000 + 0x10 * (r.next_u32() % 32));
        match r.next_u32() % 4 {
            0 => (a, b),
            1 => (a + 8, b + 8 * (r.next_u32() % 2)),
            2 => {
                let o = r.next_u32() % 16;
                (a + o, b + o)
            }
            _ => (a + r.next_u32() % 16, b + r.next_u32() % 16),
        }
    }

    driven!(gen_strlen, ssx3_ee::s15::f_00416810__generated);
    driven!(hand_strlen, strlen);
    driven!(gen_strcpy, ssx3_ee::s15::f_004166f4__generated);
    driven!(hand_strcpy, strcpy);
    driven!(gen_strncpy, ssx3_ee::s15::f_00416ce0__generated);
    driven!(hand_strncpy, strncpy);
    driven!(gen_strcmp, ssx3_ee::s15::f_004165a8__generated);
    driven!(hand_strcmp, strcmp);

    #[test]
    fn strlen_matches() {
        check_events(gen_strlen, hand_strlen, 600, &|c, seed| {
            let mut r = Rng(0x7374_726c_656e ^ (seed as u64) << 20);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x2000);
            let (s, _) = str_ptrs(&mut r);
            let len = pick_len(&mut r) % 3000;
            put_str(c, &mut r, s, len);
            c.g[A0][0] = sx(s);
            schedule(c, &mut r, seed, 0x0041_6810)
        });
    }

    #[test]
    fn strcpy_matches() {
        check_events(gen_strcpy, hand_strcpy, 600, &|c, seed| {
            let mut r = Rng(0x7374_7263_7079 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x2000);
            let (src, dst) = str_ptrs(&mut r);
            let len = pick_len(&mut r) % 3000;
            put_str(c, &mut r, src, len);
            c.g[A0][0] = sx(dst);
            c.g[A1][0] = sx(src);
            schedule(c, &mut r, seed, 0x0041_66f4)
        });
    }

    #[test]
    fn strncpy_matches() {
        check_events(gen_strncpy, hand_strncpy, 600, &|c, seed| {
            let mut r = Rng(0x7374_726e_6370 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x2000);
            let (src, dst) = str_ptrs(&mut r);
            let len = pick_len(&mut r) % 3000;
            put_str(c, &mut r, src, len);
            let n = match r.next_u32() % 4 {
                0 => len,
                1 => len + 1 + r.next_u32() % 64,
                2 => len.saturating_sub(r.next_u32() % 40),
                _ => pick_len(&mut r) % 3000,
            };
            c.g[A0][0] = sx(dst);
            c.g[A1][0] = sx(src);
            c.g[A2][0] = sx(n);
            schedule(c, &mut r, seed, 0x0041_6ce0)
        });
    }

    #[test]
    fn strcmp_matches() {
        check_events(gen_strcmp, hand_strcmp, 900, &|c, seed| {
            let mut r = Rng(0x7374_7263_6d70 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x2000);
            let (a, b) = str_ptrs(&mut r);
            let len = pick_len(&mut r) % 3000;
            put_str(c, &mut r, a, len);
            // b: a copy of a, then maybe changed at one position (a byte,
            // the NUL moved earlier or later).
            for i in 0..=len {
                let v = c.bus.read8(a + i);
                c.bus.write8(b + i, v);
            }
            match r.next_u32() % 4 {
                0 => {}
                1 if len > 0 => {
                    let i = r.next_u32() % len;
                    let v = (r.next_u32() as u8).max(1);
                    c.bus.write8(b + i, v);
                }
                2 if len > 0 => c.bus.write8(b + r.next_u32() % len, 0),
                _ => {
                    let extra = 1 + r.next_u32() % 20;
                    put_str(c, &mut r, b + len, extra)
                }
            }
            let (a, b) = if r.next_u32() % 2 == 0 { (a, b) } else { (b, a) };
            c.g[A0][0] = sx(a);
            c.g[A1][0] = sx(b);
            schedule(c, &mut r, seed, 0x0041_65a8)
        });
    }
}
