//! Native DLHeap_Memalign0x31f2c8, retained uninstalled for the Next Event
//! loading investigation. Reference only: no override registration or
//! automated tests, builds, replays or game/window operations.

use crate::call::{call_ptr, guest, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0031_f2c8;
const END: u32 = 0x0031_f4e4;
const MALLOC: u32 = 0x0031_e818;
const FREE: u32 = 0x0031_ed60;
const ERRNO: u32 = 0x0041_5fb8;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }
#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void* DLHeap_Memalign(Heap* heap, uint alignment, uint bytes)`.
/// A0 is the heap handle, A1 alignment, and A2 the original byte request.
/// Alignments at most16 delegate directly to malloc. Larger non-powers of
/// two round upwards to a power of two; the chunk requirement rounds to16,
/// and malloc reserves that requirement plus alignment plus16 bytes.
/// Leading and trailing fragments are returned through the real native
/// free routine. The mmapped-chunk offset path is preserved too.
///
/// Frame50 bytes: caller RA at0, S3 at10, S2 at20, S1 at30, S0 at40.
/// At malloc return31f3ac, S0 holds normalized alignment, S2 normalized
/// chunk bytes, S3 the heap. Its caller's original byte count is not saved
/// separately in this frame. All generated continuation/event PCs remain
/// accepted; no allocation policy or corruption workaround is added.
pub fn dlheap_memalign(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x50 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 11;
                    Frame::open(c, 0x50);
                    frame.save(c, S0, 0x40);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, S3, 0x10);
                    frame.save(c, S1, 0x30);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, S2, 0x20);
                    c.g[A1][0] = c.g[A2][0];
                    c.g[V0][0] = (c.g[S0][0] < 0x11) as u64;
                    let ordinary_alignment = c.g[V0][0] != 0;
                    frame.save_ra(c, 0);
                    pc = if ordinary_alignment { 0x0031_f2f4 } else { 0x0031_f304 };
                }
                0x0031_f2f4 => {
                    c.cycles += 4;
                    call_ptr(c, MALLOC, 0x0031_f2fc)?;
                    pc = 0x0031_f2fc;
                }
                0x0031_f2fc => {
                    frame.restore(c, S0, 0x40);
                    pc = 0x0031_f4cc;
                }
                0x0031_f304 => {
                    c.cycles += 8;
                    c.g[V0][0] = 0xf;
                    c.g[A0][0] = 0x10;
                    c.g[V0][0] = (c.g[V0][0] < c.g[S0][0]) as u64;
                    if c.g[V0][0] == 0 { c.g[S0][0] = c.g[A0][0]; }
                    c.g[V1][0] = sx(r(c, S0).wrapping_sub(1));
                    c.g[V1][0] = c.g[S0][0] & c.g[V1][0];
                    let power_of_two = c.g[V1][0] == 0;
                    c.g[V1][0] = 0x20;
                    pc = if power_of_two { 0x0031_f358 } else { 0x0031_f324 };
                }
                0x0031_f324 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < c.g[S0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[V1][0];
                        pc = 0x0031_f358;
                    } else {
                        pc = 0x0031_f330;
                    }
                }
                0x0031_f330 => {
                    c.cycles += 2;
                    c.g[V1][0] = sx(r(c, V1) << 1);
                    pc = 0x0031_f338;
                }
                0x0031_f338 => {
                    c.cycles += 7;
                    c.g[V0][0] = (c.g[V1][0] < c.g[S0][0]) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = sx(r(c, V1) << 1);
                        if back_edge_unwinds(c, 0x0031_f338) { return Err(Unwound); }
                        pc = 0x0031_f338;
                    } else {
                        pc = 0x0031_f354;
                    }
                }
                0x0031_f354 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.g[V1][0];
                    pc = 0x0031_f358;
                }
                0x0031_f358 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[V0][0] |= 0xffdf;
                    c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
                    let fits = c.g[V0][0] == 0;
                    c.g[V1][0] = sx(r(c, A1).wrapping_add(0x13));
                    pc = if fits { 0x0031_f384 } else { 0x0031_f36c };
                }
                0x0031_f36c => {
                    c.cycles += 6;
                    call_ptr(c, ERRNO, 0x0031_f374)?;
                    pc = 0x0031_f374;
                }
                0x0031_f374 => {
                    c.g[V1][0] = 0xc;
                    c.sw(r(c, V0), c.g[V1][0]);
                    c.g[V0][0] = 0;
                    pc = 0x0031_f4c8;
                }
                0x0031_f384 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < 0x10) as u64;
                    let minimal = c.g[V0][0] != 0;
                    c.g[S2][0] = 0x10;
                    pc = if minimal { 0x0031_f39c } else { 0x0031_f390 };
                }
                0x0031_f390 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[V0][0] |= 0xfff0;
                    c.g[S2][0] = c.g[V1][0] & c.g[V0][0];
                    pc = 0x0031_f39c;
                }
                0x0031_f39c => {
                    c.cycles += 7;
                    c.g[A1][0] = sx(r(c, S2).wrapping_add(r(c, S0)));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0031_f3ac);
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(0x10));
                    call_ptr(c, MALLOC, 0x0031_f3ac)?;
                    pc = 0x0031_f3ac;
                }
                0x0031_f3ac => {
                    c.g[A3][0] = c.g[V0][0];
                    if c.g[A3][0] != 0 {
                        c.divu(0, A3, S0);
                        pc = 0x0031_f3c0;
                    } else {
                        pc = 0x0031_f3b8;
                    }
                }
                0x0031_f3b8 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    pc = 0x0031_f4c8;
                }
                0x0031_f3c0 => {
                    // Both branches of the native zero-divisor check join here.
                    c.cycles += 2;
                    pc = 0x0031_f3c8;
                }
                0x0031_f3c8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.hi[0];
                    let already_aligned = c.g[V0][0] == 0;
                    c.g[S1][0] = sx(r(c, A3).wrapping_sub(8));
                    pc = if already_aligned { 0x0031_f470 } else { 0x0031_f3d4 };
                }
                0x0031_f3d4 => {
                    c.cycles += 18;
                    c.g[V0][0] = sx(r(c, A3).wrapping_add(r(c, S0)));
                    c.g[V1][0] = sx(0u32.wrapping_sub(r(c, S0)));
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
                    c.g[A1][0] = 0xf;
                    c.g[V0][0] &= c.g[V1][0];
                    c.g[A2][0] = c.lw(at(c, A3, u32::MAX - 3));
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(8));
                    c.g[V1][0] = 0xffff_ffff_ffff_0000;
                    c.g[A0][0] = sx(r(c, V0).wrapping_sub(r(c, S1)));
                    c.g[S0][0] = sx(r(c, V0).wrapping_add(r(c, S0)));
                    c.g[A1][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
                    c.g[V1][0] |= 0xfffc;
                    if c.g[A1][0] != 0 { c.g[S0][0] = c.g[V0][0]; }
                    c.g[V1][0] = c.g[A2][0] & c.g[V1][0];
                    c.g[T0][0] = sx(r(c, S0).wrapping_sub(r(c, S1)));
                    c.g[A2][0] &= 2;
                    let ordinary_chunk = c.g[A2][0] == 0;
                    c.g[A0][0] = sx(r(c, V1).wrapping_sub(r(c, T0)));
                    pc = if ordinary_chunk { 0x0031_f438 } else { 0x0031_f41c };
                }
                0x0031_f41c => {
                    // mmapped chunk: adjust its stored offset instead of freeing a prefix.
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(at(c, A3, u32::MAX - 7));
                    c.g[V0][0] = c.g[A0][0] | 2;
                    c.sw(at(c, S0, 4), c.g[V0][0]);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, T0)));
                    c.g[V0][0] = sx(at(c, S0, 8));
                    c.sw(r(c, S0), c.g[V1][0]);
                    pc = 0x0031_f4c8;
                }
                0x0031_f438 => {
                    // Split the leading fragment, preserving the old PREV_INUSE bit.
                    c.cycles += 14;
                    c.g[V0][0] = c.g[A0][0] | 1;
                    c.g[A2][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    c.sw(at(c, S0, 4), c.g[V0][0]);
                    c.g[A1][0] = c.g[A3][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[S1][0] = c.g[S0][0];
                    c.g[V0][0] = c.lw(at(c, A2, 4));
                    c.g[V0][0] |= 1;
                    c.sw(at(c, A2, 4), c.g[V0][0]);
                    c.g[V1][0] = c.lw(at(c, A3, u32::MAX - 3));
                    c.g[V1][0] &= 1;
                    c.g[V1][0] |= c.g[T0][0];
                    c.g[RA][0] = sx(0x0031_f470);
                    c.sw(at(c, A3, u32::MAX - 3), c.g[V1][0]);
                    call_ptr(c, FREE, 0x0031_f470)?;
                    pc = 0x0031_f470;
                }
                0x0031_f470 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(at(c, S1, 4));
                    c.g[V0][0] = c.g[A0][0] & 2;
                    let mmapped = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(at(c, S1, 8));
                    pc = if mmapped { 0x0031_f4c8 } else { 0x0031_f480 };
                }
                0x0031_f480 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[V1][0] = sx(at(c, S2, 0x10));
                    c.g[V0][0] |= 0xfffc;
                    c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    let no_tail = c.g[V1][0] == 0;
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(r(c, S2)));
                    pc = if no_tail { 0x0031_f4c4 } else { 0x0031_f49c };
                }
                0x0031_f49c => {
                    // Split and free the remaining tail, keeping the requested chunk.
                    c.cycles += 10;
                    c.g[A1][0] = sx(r(c, S1).wrapping_add(r(c, S2)));
                    c.g[V0][0] |= 1;
                    c.g[A0][0] = c.g[S3][0];
                    c.sw(at(c, A1, 4), c.g[V0][0]);
                    c.g[A1][0] = sx(at(c, A1, 8));
                    c.g[V0][0] = c.lw(at(c, S1, 4));
                    c.g[V0][0] &= 1;
                    c.g[V0][0] |= c.g[S2][0];
                    c.g[RA][0] = sx(0x0031_f4c4);
                    c.sw(at(c, S1, 4), c.g[V0][0]);
                    call_ptr(c, FREE, 0x0031_f4c4)?;
                    pc = 0x0031_f4c4;
                }
                0x0031_f4c4 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx(at(c, S1, 8));
                    pc = 0x0031_f4c8;
                }
                0x0031_f4c8 => {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x40);
                    pc = 0x0031_f4cc;
                }
                0x0031_f4cc => {
                    c.cycles += 6;
                    frame.restore(c, S1, 0x30);
                    frame.restore(c, S2, 0x20);
                    frame.restore(c, S3, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
