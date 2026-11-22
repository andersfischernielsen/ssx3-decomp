//! Native DLHeap_Malloc0x31e818, retained uninstalled for the Next Event
//! loading investigation. Reference only: empty overrides and no automated
//! test, build, replay or game/window operation.

use crate::call::{call_ptr, guest, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0031_e818;
const END: u32 = 0x0031_ed5c;
const STATE_TOP: u32 = 0x30;
const LAST_REMAINDER: u32 = 0x34;
const UNSORTED: u32 = 0x38;
const UNSORTED_FD: u32 = 0x40;
const UNSORTED_BK: u32 = 0x44;
const BINMAP: u32 = 0x338;
const SIZE: u32 = 4;
const FD: u32 = 8;
const BK: u32 = 12;
const SIZE_MASK: u64 = 0xffff_ffff_ffff_fffc;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }
#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 { r(c, register).wrapping_add(offset) }
#[inline]
fn back_edge(c: &mut Cpu, head: u32) -> Result<(), Unwound> {
    if back_edge_unwinds(c, head) { Err(Unwound) } else { Ok(()) }
}

/// `void* DLHeap_Malloc(Heap* heap, uint bytes)`.
///
/// FP retains the heap handle (A0); S2 is its dereferenced malloc state.
/// S3 is the request rounded to16 bytes, at least16; S7 says it is below256.
/// Fastbins precede exact small bins, the unsorted list, sorted size bins,
/// bitmap-selected larger bins and finally the top chunk / system allocator.
/// A chunk's size word is at+4, forward link+8 and backward link+c; returned
/// payload is chunk+8. Native metadata and all real callee paths are retained.
///
/// The persistent-PC31e968 case is unsorted-list draining, not an I/O wait:
/// S5=state+38 is its sentinel and S1=*(state+44) its tail. Every ordinary
/// iteration removes that tail, bins it, then reloads state+44. The backward
/// chain must terminate at the sentinel; the native routine has no cycle
/// guard or repair policy. A self-cycle/corrupt link can keep this loop live.
///
/// Frame a0 bytes: caller RA at0, FP at10, S7..S0 at20..90. It is deliberately
/// dispatcher-shaped so all generated continuation/event PCs remain valid.
pub fn dlheap_malloc(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xa0 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 17;
                    Frame::open(c, frame.size);
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    frame.save(c, FP, 0x10);
                    c.g[V0][0] |= 0xffdf;
                    frame.save(c, S0, 0x90);
                    c.g[FP][0] = c.g[A0][0];
                    frame.save(c, S1, 0x80);
                    c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                    frame.save(c, S3, 0x60);
                    frame.save(c, S4, 0x50);
                    frame.save(c, S5, 0x40);
                    frame.save(c, S6, 0x30);
                    frame.save(c, S7, 0x20);
                    frame.save_ra(c, 0);
                    frame.save(c, S2, 0x70);
                    let oversized = c.g[V0][0] != 0;
                    c.g[S2][0] = c.lw(r(c, FP));
                    pc = if oversized { 0x0031_e85c } else { 0x0031_e874 };
                }
                0x0031_e85c => {
                    c.cycles += 6;
                    call_ptr(c, 0x0041_5fb8, 0x0031_e864)?; // errno address
                    pc = 0x0031_e864;
                }
                0x0031_e864 => {
                    c.g[V1][0] = 12;
                    c.sw(r(c, V0), c.g[V1][0]);
                    c.g[V0][0] = 0;
                    pc = 0x0031_ed2c;
                }
                0x0031_e874 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx(at(c, A1, 0x13));
                    c.g[V0][0] = (c.g[V1][0] < 0x10) as u64;
                    let minimum = c.g[V0][0] != 0;
                    c.g[S3][0] = 0x10;
                    pc = if minimum { 0x0031_e890 } else { 0x0031_e884 };
                }
                0x0031_e884 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[V0][0] |= 0xfff0;
                    c.g[S3][0] = c.g[V1][0] & c.g[V0][0];
                    pc = 0x0031_e890;
                }
                0x0031_e890 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(r(c, S2));
                    c.g[V0][0] = (c.g[V1][0] ^ 1) & 1;
                    let initialized = c.g[V0][0] == 0;
                    c.g[V0][0] = (c.g[V1][0] < c.g[S3][0]) as u64;
                    pc = if initialized { 0x0031_e8c0 } else { 0x0031_e8a4 };
                }
                0x0031_e8a4 => {
                    c.cycles += 2;
                    if c.g[V1][0] != 0 {
                        c.g[S1][0] = c.lw(at(c, S2, STATE_TOP));
                        pc = 0x0031_ec74;
                    } else { pc = 0x0031_e8ac; }
                }
                0x0031_e8ac => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x0031_eee8, 0x0031_e8b8)?;
                    pc = 0x0031_e8b8;
                }
                0x0031_e8b8 => {
                    c.g[S1][0] = c.lw(at(c, S2, STATE_TOP));
                    pc = 0x0031_ec74;
                }
                0x0031_e8c0 => {
                    c.cycles += 2;
                    let beyond_fast = c.g[V0][0] != 0;
                    c.g[S7][0] = (c.g[S3][0] < 0x100) as u64;
                    pc = if beyond_fast { 0x0031_e8ec } else { 0x0031_e8c8 };
                }
                0x0031_e8c8 => {
                    // Fastbin heads occupy state+4..; chunks retain fd at+8.
                    c.cycles += 6;
                    c.g[V0][0] = sx(r(c, S3) >> 1);
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(4));
                    c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, V0)));
                    c.g[S1][0] = c.lw(r(c, A0));
                    let empty = c.g[S1][0] == 0;
                    c.g[V0][0] = sx(at(c, S1, FD));
                    pc = if empty { 0x0031_e8ec } else { 0x0031_e8e0 };
                }
                0x0031_e8e0 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(at(c, S1, FD));
                    c.sw(r(c, A0), c.g[V1][0]);
                    pc = 0x0031_ed2c;
                }
                0x0031_e8ec => {
                    c.cycles += 2;
                    let large = c.g[S7][0] == 0;
                    c.g[V0][0] = sx(at(c, S3, UNSORTED));
                    pc = if large { 0x0031_e92c } else { 0x0031_e8f4 };
                }
                0x0031_e8f4 => {
                    c.cycles += 5;
                    c.g[A2][0] = sx(r(c, S2).wrapping_add(r(c, V0)));
                    c.g[S1][0] = c.lw(at(c, A2, 4));
                    c.g[T0][0] = sx(r(c, A2).wrapping_sub(8));
                    let empty = c.g[S1][0] == c.g[T0][0];
                    c.g[S6][0] = sx(r(c, S3) >> 3);
                    pc = if empty { 0x0031_e950 } else { 0x0031_e908 };
                }
                0x0031_e908 => {
                    c.cycles += 9;
                    c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, S3)));
                    c.g[A3][0] = c.lw(at(c, S1, BK));
                    c.g[V1][0] = c.lw(at(c, A0, SIZE));
                    c.g[V0][0] = sx(at(c, S1, FD));
                    c.g[V1][0] |= 1;
                    c.sw(at(c, A0, SIZE), c.g[V1][0]);
                    c.sw(at(c, A2, 4), c.g[A3][0]);
                    c.sw(at(c, A3, FD), c.g[T0][0]);
                    pc = 0x0031_ed2c;
                }
                0x0031_e92c => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0031_e1d8, 0x0031_e934)?;
                    pc = 0x0031_e934;
                }
                0x0031_e934 => {
                    c.g[V1][0] = c.lw(r(c, S2));
                    c.g[V1][0] &= 2;
                    let no_fast_chunks = c.g[V1][0] == 0;
                    c.g[S6][0] = c.g[V0][0];
                    pc = if no_fast_chunks { 0x0031_e950 } else { 0x0031_e944 };
                }
                0x0031_e944 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x0031_eee8, 0x0031_e950)?;
                    pc = 0x0031_e950;
                }
                0x0031_e950 => {
                    // Drain unsorted tail into its appropriate sorted bin.
                    c.cycles += 4;
                    c.g[S1][0] = c.lw(at(c, S2, UNSORTED_BK));
                    c.g[S5][0] = sx(at(c, S2, UNSORTED));
                    pc = if c.g[S1][0] == c.g[S5][0] { 0x0031_eaa0 } else { 0x0031_e960 };
                }
                0x0031_e960 => {
                    c.cycles += 2;
                    c.g[V1][0] = c.lw(at(c, S1, SIZE));
                    pc = 0x0031_e968;
                }
                0x0031_e968 => {
                    c.cycles += 5;
                    c.g[V0][0] = SIZE_MASK;
                    c.g[A3][0] = c.lw(at(c, S1, BK));
                    let large_request = c.g[S7][0] == 0;
                    c.g[S0][0] = c.g[V1][0] & c.g[V0][0];
                    pc = if large_request { 0x0031_e9a0 } else { 0x0031_e97c };
                }
                0x0031_e97c => {
                    c.cycles += 2;
                    if c.g[A3][0] != c.g[S5][0] {
                        c.sw(at(c, S2, UNSORTED_BK), c.g[A3][0]);
                        pc = 0x0031_e9a4;
                    } else { pc = 0x0031_e984; }
                }
                0x0031_e984 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, S2, LAST_REMAINDER));
                    if c.g[S1][0] != c.g[V0][0] {
                        c.sw(at(c, S2, UNSORTED_BK), c.g[A3][0]);
                        pc = 0x0031_e9a4;
                    } else { pc = 0x0031_e990; }
                }
                0x0031_e990 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(at(c, S3, 0x10));
                    c.g[V0][0] = (c.g[V0][0] < c.g[S0][0]) as u64;
                    let split_remainder = c.g[V0][0] != 0;
                    c.g[A2][0] = sx(r(c, S1).wrapping_add(r(c, S3)));
                    pc = if split_remainder { 0x0031_eca8 } else { 0x0031_e9a0 };
                }
                0x0031_e9a0 => {
                    c.cycles += 1;
                    c.sw(at(c, S2, UNSORTED_BK), c.g[A3][0]);
                    pc = 0x0031_e9a4;
                }
                0x0031_e9a4 => {
                    c.cycles += 2;
                    let exact_fit = c.g[S0][0] == c.g[S3][0];
                    c.sw(at(c, A3, FD), c.g[S5][0]);
                    pc = if exact_fit { 0x0031_ece0 } else { 0x0031_e9ac };
                }
                0x0031_e9ac => {
                    c.cycles += 3;
                    c.g[S4][0] = (c.g[S0][0] < 0x100) as u64;
                    let large_chunk = c.g[S4][0] == 0;
                    c.g[A1][0] = sx(r(c, S0) >> 3);
                    pc = if large_chunk { 0x0031_e9d4 } else { 0x0031_e9b8 };
                }
                0x0031_e9b8 => {
                    c.cycles += 7;
                    c.g[T0][0] = sx(at(c, S2, BINMAP));
                    c.g[V0][0] = sx(r(c, A1) << 3);
                    c.g[V0][0] = sx(at(c, V0, UNSORTED));
                    c.g[V0][0] = sx(r(c, S2).wrapping_add(r(c, V0)));
                    c.g[A2][0] = c.lw(r(c, V0));
                    c.g[A3][0] = sx(r(c, V0).wrapping_sub(8));
                    pc = 0x0031_ea60;
                }
                0x0031_e9d4 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_e1d8, 0x0031_e9dc)?;
                    pc = 0x0031_e9dc;
                }
                0x0031_e9dc => {
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = sx(r(c, A1) << 3);
                    c.g[V0][0] = sx(at(c, V0, UNSORTED));
                    c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, V0)));
                    c.g[A2][0] = c.lw(r(c, A0));
                    c.g[A3][0] = sx(r(c, A0).wrapping_sub(8));
                    let empty = c.g[A2][0] == c.g[A3][0];
                    c.g[T0][0] = sx(at(c, S2, BINMAP));
                    pc = if empty { 0x0031_ea60 } else { 0x0031_e9fc };
                }
                0x0031_e9fc => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(at(c, A3, BK));
                    c.g[V0][0] = c.lw(at(c, V1, SIZE));
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    pc = if c.g[V0][0] == 0 { 0x0031_ea1c } else { 0x0031_ea10 };
                }
                0x0031_ea10 => {
                    c.cycles += 3;
                    c.g[A2][0] = c.g[A3][0];
                    c.g[A3][0] = c.lw(at(c, A0, 4));
                    pc = 0x0031_ea60;
                }
                0x0031_ea1c => {
                    c.cycles += 2;
                    let small_chunk = c.g[S4][0] != 0;
                    c.g[A0][0] = sx((r(c, A1) as i32 >> 5) as u32);
                    pc = if small_chunk { 0x0031_ea64 } else { 0x0031_ea24 };
                }
                0x0031_ea24 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(at(c, A2, SIZE));
                    c.g[S0][0] |= 1;
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A3][0] = c.lw(at(c, A2, BK));
                        pc = 0x0031_ea64;
                    } else { pc = 0x0031_ea38; }
                }
                0x0031_ea38 => {
                    c.cycles += 2;
                    c.g[A2][0] = c.lw(at(c, A2, FD));
                    pc = 0x0031_ea40;
                }
                0x0031_ea40 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(at(c, A2, SIZE));
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A2][0] = c.lw(at(c, A2, FD));
                        back_edge(c, 0x0031_ea40)?;
                    } else { pc = 0x0031_ea5c; }
                }
                0x0031_ea5c => {
                    c.cycles += 1;
                    c.g[A3][0] = c.lw(at(c, A2, BK));
                    pc = 0x0031_ea60;
                }
                0x0031_ea60 => {
                    c.cycles += 1;
                    c.g[A0][0] = sx((r(c, A1) as i32 >> 5) as u32);
                    pc = 0x0031_ea64;
                }
                0x0031_ea64 => {
                    c.cycles += 15;
                    c.g[V1][0] = 1;
                    c.g[A0][0] = sx(r(c, A0) << 2);
                    c.g[A1][0] &= 31;
                    c.g[A0][0] = sx(r(c, T0).wrapping_add(r(c, A0)));
                    c.g[V1][0] = sx(r(c, V1) << (r(c, A1) & 31));
                    c.g[V0][0] = c.lw(r(c, A0));
                    c.g[V0][0] |= c.g[V1][0];
                    c.sw(r(c, A0), c.g[V0][0]);
                    c.sw(at(c, S1, BK), c.g[A3][0]);
                    c.sw(at(c, S1, FD), c.g[A2][0]);
                    c.sw(at(c, A2, BK), c.g[S1][0]);
                    c.sw(at(c, A3, FD), c.g[S1][0]);
                    c.g[S1][0] = c.lw(at(c, S2, UNSORTED_BK));
                    if c.g[S1][0] != c.g[S5][0] {
                        c.g[V1][0] = c.lw(at(c, S1, SIZE));
                        back_edge(c, 0x0031_e968)?;
                        pc = 0x0031_e968;
                    } else { pc = 0x0031_eaa0; }
                }
                0x0031_eaa0 => {
                    c.cycles += 2;
                    if c.g[S7][0] != 0 {
                        c.g[S6][0] = sx(at(c, S6, 1));
                        pc = 0x0031_eb3c;
                    } else { pc = 0x0031_eaa8; }
                }
                0x0031_eaa8 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r(c, S6) << 3);
                    c.g[V0][0] = sx(at(c, V0, UNSORTED));
                    c.g[V0][0] = sx(r(c, S2).wrapping_add(r(c, V0)));
                    c.g[S1][0] = c.lw(at(c, V0, 4));
                    c.g[T0][0] = sx(r(c, V0).wrapping_sub(8));
                    let empty = c.g[S1][0] == c.g[T0][0];
                    c.g[T2][0] = c.g[S5][0];
                    pc = if empty { 0x0031_eb38 } else { 0x0031_eac4 };
                }
                0x0031_eac4 => {
                    c.cycles += 5;
                    c.g[A0][0] = SIZE_MASK;
                    c.g[T3][0] = c.g[S3][0] | 1;
                    c.g[V0][0] = c.lw(at(c, S1, SIZE));
                    pc = 0x0031_ead8;
                }
                0x0031_ead8 => {
                    c.cycles += 4;
                    c.g[S0][0] = c.g[V0][0] & c.g[A0][0];
                    c.g[V1][0] = (c.g[S0][0] < c.g[S3][0]) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[S1][0] = c.lw(at(c, S1, BK));
                        pc = 0x0031_eb30;
                    } else { pc = 0x0031_eae8; }
                }
                0x0031_eae8 => {
                    c.cycles += 8;
                    c.g[A2][0] = c.lw(at(c, S1, FD));
                    c.g[T1][0] = sx(r(c, S0).wrapping_sub(r(c, S3)));
                    c.g[A3][0] = c.lw(at(c, S1, BK));
                    c.g[A1][0] = sx(at(c, S1, FD));
                    c.g[V0][0] = (c.g[T1][0] < 0x10) as u64;
                    c.sw(at(c, A2, BK), c.g[A3][0]);
                    let too_small_to_split = c.g[V0][0] != 0;
                    c.sw(at(c, A3, FD), c.g[A2][0]);
                    pc = if too_small_to_split { 0x0031_ece8 } else { 0x0031_eb08 };
                }
                0x0031_eb08 => {
                    c.cycles += 10;
                    c.g[A2][0] = sx(r(c, S1).wrapping_add(r(c, S3)));
                    c.g[A0][0] = c.g[T1][0] | 1;
                    c.sw(at(c, S2, UNSORTED_BK), c.g[A2][0]);
                    c.g[V1][0] = sx(r(c, A2).wrapping_add(r(c, T1)));
                    c.sw(at(c, S2, UNSORTED_FD), c.g[A2][0]);
                    c.g[V0][0] = c.g[A1][0];
                    c.sw(at(c, A2, BK), c.g[T2][0]);
                    c.sw(at(c, A2, FD), c.g[T2][0]);
                    c.sw(at(c, S1, SIZE), c.g[T3][0]);
                    pc = 0x0031_ecd4;
                }
                0x0031_eb30 => {
                    c.cycles += 2;
                    if c.g[S1][0] != c.g[T0][0] {
                        c.g[V0][0] = c.lw(at(c, S1, SIZE));
                        back_edge(c, 0x0031_ead8)?;
                        pc = 0x0031_ead8;
                    } else { pc = 0x0031_eb38; }
                }
                0x0031_eb38 => {
                    c.cycles += 1;
                    c.g[S6][0] = sx(at(c, S6, 1));
                    pc = 0x0031_eb3c;
                }
                0x0031_eb3c => {
                    // Bitmap search for the first larger nonempty bin.
                    c.cycles += 21;
                    c.g[A0][0] = sx(at(c, S2, BINMAP));
                    c.g[T1][0] = sx(r(c, S6) >> 5);
                    c.g[T5][0] = c.g[A0][0];
                    c.g[V0][0] = sx(r(c, T1) << 2);
                    c.g[V1][0] = sx(r(c, S6) << 3);
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
                    c.g[A1][0] = sx(r(c, T1) << 8);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, S2)));
                    c.g[A2][0] = c.g[S6][0] & 31;
                    c.g[A1][0] = sx(at(c, A1, UNSORTED));
                    c.g[V0][0] = 1;
                    c.g[T2][0] = 0xffff_ffff_ffff_0000;
                    c.g[A3][0] = c.lw(r(c, A0));
                    c.g[A2][0] = sx(r(c, V0) << (r(c, A2) & 31));
                    c.g[T0][0] = sx(at(c, V1, 0x30));
                    c.g[T3][0] = c.g[S5][0];
                    c.g[T6][0] = c.g[S7][0];
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, S2)));
                    c.g[T2][0] |= 0xfffc;
                    c.g[T4][0] = c.g[S3][0] | 1;
                    pc = 0x0031_eb90;
                }
                0x0031_eb90 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[A3][0] < c.g[A2][0]) as u64;
                    let beyond_map = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r(c, T1) << 2);
                    pc = if beyond_map { 0x0031_eba8 } else { 0x0031_eb9c };
                }
                0x0031_eb9c => {
                    c.cycles += 2;
                    let bit_nonzero = c.g[A2][0] != 0;
                    c.g[V0][0] = c.g[A2][0] & c.g[A3][0];
                    pc = if bit_nonzero { 0x0031_ebe8 } else { 0x0031_eba4 };
                }
                0x0031_eba4 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx(r(c, T1) << 2);
                    pc = 0x0031_eba8;
                }
                0x0031_eba8 => {
                    c.cycles += 2;
                    c.g[V1][0] = sx(r(c, V0).wrapping_add(r(c, T5)));
                    c.g[T1][0] = sx(at(c, T1, 1));
                    pc = 0x0031_ebb0;
                }
                0x0031_ebb0 => {
                    c.cycles += 5;
                    c.g[V1][0] = sx(at(c, V1, 4));
                    c.g[A0][0] = sx(at(c, A0, 4));
                    c.g[V0][0] = (c.g[T1][0] < 3) as u64;
                    let exhausted = c.g[V0][0] == 0;
                    c.g[A1][0] = sx(at(c, A1, 0x100));
                    pc = if exhausted { 0x0031_ec70 } else { 0x0031_ebc4 };
                }
                0x0031_ebc4 => {
                    c.cycles += 3;
                    c.g[A3][0] = c.lw(r(c, V1));
                    if c.g[A3][0] == 0 {
                        c.g[T1][0] = sx(at(c, T1, 1));
                        back_edge(c, 0x0031_ebb0)?;
                        pc = 0x0031_ebb0;
                    } else { pc = 0x0031_ebd0; }
                }
                0x0031_ebd0 => {
                    c.cycles += 3;
                    c.g[T0][0] = sx(r(c, A1).wrapping_sub(8));
                    c.g[A2][0] = 1;
                    pc = 0x0031_ebe4;
                }
                0x0031_ebdc => { c.cycles += 1; pc = 0x0031_ebe0; }
                0x0031_ebe0 => {
                    c.cycles += 1;
                    c.g[T0][0] = sx(at(c, T0, 8));
                    pc = 0x0031_ebe4;
                }
                0x0031_ebe4 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[A2][0] & c.g[A3][0];
                    pc = 0x0031_ebe8;
                }
                0x0031_ebe8 => {
                    c.cycles += 5;
                    if c.g[V0][0] == 0 {
                        c.g[A2][0] = sx(r(c, A2) << 1);
                        back_edge(c, 0x0031_ebe0)?;
                        pc = 0x0031_ebe0;
                    } else { pc = 0x0031_ebfc; }
                }
                0x0031_ebfc => {
                    c.cycles += 3;
                    c.g[S1][0] = c.lw(at(c, T0, BK));
                    if c.g[S1][0] != c.g[T0][0] {
                        c.g[V0][0] = c.lw(at(c, S1, SIZE));
                        pc = 0x0031_ec20;
                    } else { pc = 0x0031_ec08; }
                }
                0x0031_ec08 => {
                    c.cycles += 6;
                    c.g[V0][0] = !c.g[A2][0];
                    c.g[T0][0] = sx(at(c, S1, 8));
                    c.g[A3][0] &= c.g[V0][0];
                    c.g[A2][0] = sx(r(c, A2) << 1);
                    c.sw(r(c, A0), c.g[A3][0]);
                    back_edge(c, 0x0031_eb90)?;
                    pc = 0x0031_eb90;
                }
                0x0031_ec20 => {
                    c.cycles += 7;
                    c.g[A3][0] = c.lw(at(c, S1, BK));
                    c.g[S0][0] = c.g[V0][0] & c.g[T2][0];
                    c.g[T1][0] = sx(r(c, S0).wrapping_sub(r(c, S3)));
                    c.sw(at(c, T0, BK), c.g[A3][0]);
                    c.g[V0][0] = (c.g[T1][0] < 0x10) as u64;
                    let too_small_to_split = c.g[V0][0] != 0;
                    c.sw(at(c, A3, FD), c.g[T0][0]);
                    pc = if too_small_to_split { 0x0031_ecf4 } else { 0x0031_ec3c };
                }
                0x0031_ec3c => {
                    c.cycles += 6;
                    c.g[A2][0] = sx(r(c, S1).wrapping_add(r(c, S3)));
                    c.sw(at(c, S2, UNSORTED_FD), c.g[A2][0]);
                    c.sw(at(c, S2, UNSORTED_BK), c.g[A2][0]);
                    c.sw(at(c, A2, BK), c.g[T3][0]);
                    let large_request = c.g[T6][0] == 0;
                    c.sw(at(c, A2, FD), c.g[T3][0]);
                    pc = if large_request { 0x0031_ec58 } else { 0x0031_ec54 };
                }
                0x0031_ec54 => {
                    c.cycles += 1;
                    c.sw(at(c, S2, LAST_REMAINDER), c.g[A2][0]);
                    pc = 0x0031_ec58;
                }
                0x0031_ec58 => {
                    c.cycles += 6;
                    c.sw(at(c, S1, SIZE), c.g[T4][0]);
                    c.g[V0][0] = c.g[T1][0] | 1;
                    c.sw(at(c, A2, SIZE), c.g[V0][0]);
                    c.g[V1][0] = sx(r(c, A2).wrapping_add(r(c, T1)));
                    c.sw(r(c, V1), c.g[T1][0]);
                    pc = 0x0031_ed28;
                }
                0x0031_ec70 => {
                    c.cycles += 1;
                    c.g[S1][0] = c.lw(at(c, S2, STATE_TOP));
                    pc = 0x0031_ec74;
                }
                0x0031_ec74 => {
                    c.cycles += 8;
                    c.g[V0][0] = SIZE_MASK;
                    c.g[V1][0] = sx(at(c, S3, 0x10));
                    c.g[A0][0] = c.lw(at(c, S1, SIZE));
                    c.g[S0][0] = c.g[A0][0] & c.g[V0][0];
                    c.g[V1][0] = (c.g[S0][0] < c.g[V1][0]) as u64;
                    let top_sufficient = c.g[V1][0] == 0;
                    c.g[A0][0] = c.g[FP][0];
                    pc = if top_sufficient { 0x0031_ed0c } else { 0x0031_ec94 };
                }
                0x0031_ec94 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x0031_e2d8, 0x0031_eca0)?;
                    pc = 0x0031_eca0;
                }
                0x0031_eca0 => {
                    frame.restore(c, S0, 0x90);
                    pc = 0x0031_ed30;
                }
                0x0031_eca8 => {
                    c.cycles += 11;
                    c.g[T1][0] = sx(r(c, S0).wrapping_sub(r(c, S3)));
                    c.sw(at(c, S2, LAST_REMAINDER), c.g[A2][0]);
                    c.g[A1][0] = c.g[S3][0] | 1;
                    c.sw(at(c, S2, UNSORTED_FD), c.g[A2][0]);
                    c.g[A0][0] = c.g[T1][0] | 1;
                    c.sw(at(c, S2, UNSORTED_BK), c.g[A2][0]);
                    c.g[V1][0] = sx(r(c, A2).wrapping_add(r(c, T1)));
                    c.sw(at(c, A2, BK), c.g[A3][0]);
                    c.g[V0][0] = sx(at(c, S1, FD));
                    c.sw(at(c, A2, FD), c.g[A3][0]);
                    c.sw(at(c, S1, SIZE), c.g[A1][0]);
                    pc = 0x0031_ecd4;
                }
                0x0031_ecd4 => {
                    c.cycles += 3;
                    c.sw(at(c, A2, SIZE), c.g[A0][0]);
                    c.sw(r(c, V1), c.g[T1][0]);
                    pc = 0x0031_ed2c;
                }
                0x0031_ece0 => {
                    c.cycles += 2;
                    c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, S3)));
                    pc = 0x0031_ecf8;
                }
                0x0031_ece8 => {
                    c.cycles += 3;
                    c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, S0)));
                    c.g[V0][0] = c.g[A1][0];
                    pc = 0x0031_ecfc;
                }
                0x0031_ecf4 => {
                    c.cycles += 1;
                    c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, S0)));
                    pc = 0x0031_ecf8;
                }
                0x0031_ecf8 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx(at(c, S1, FD));
                    pc = 0x0031_ecfc;
                }
                0x0031_ecfc => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(at(c, A0, SIZE));
                    c.g[V1][0] |= 1;
                    c.sw(at(c, A0, SIZE), c.g[V1][0]);
                    pc = 0x0031_ed2c;
                }
                0x0031_ed0c => {
                    c.cycles += 7;
                    c.g[A2][0] = sx(r(c, S1).wrapping_add(r(c, S3)));
                    c.g[V0][0] = c.g[S3][0] | 1;
                    c.sw(at(c, S2, STATE_TOP), c.g[A2][0]);
                    c.g[T1][0] = sx(r(c, S0).wrapping_sub(r(c, S3)));
                    c.sw(at(c, S1, SIZE), c.g[V0][0]);
                    c.g[V1][0] = c.g[T1][0] | 1;
                    c.sw(at(c, A2, SIZE), c.g[V1][0]);
                    pc = 0x0031_ed28;
                }
                0x0031_ed28 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx(at(c, S1, FD));
                    pc = 0x0031_ed2c;
                }
                0x0031_ed2c => {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x90);
                    pc = 0x0031_ed30;
                }
                0x0031_ed30 => {
                    c.cycles += 11;
                    frame.restore(c, S1, 0x80);
                    frame.restore(c, S2, 0x70);
                    frame.restore(c, S3, 0x60);
                    frame.restore(c, S4, 0x50);
                    frame.restore(c, S5, 0x40);
                    frame.restore(c, S6, 0x30);
                    frame.restore(c, S7, 0x20);
                    frame.restore(c, FP, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    });
}
