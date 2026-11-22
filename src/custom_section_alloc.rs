//! Section-local allocation from the world's fixed-size geometry blocks.

use crate::call::{call_ptr, guest, Flow, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003a_7878, section_allocator_alloc, "SectionAllocator_Alloc")];

const ALLOC_BLOCK: u32 = 0x003a_7768;
const BLOCK_SIZE: u32 = 4;
const BLOCK_POSITION: u32 = 8;
const BLOCK_PADDING: u32 = 12;
const BLOCK_NEXT: u32 = 20;
const BLOCK_COUNT: u32 = 8;
const BLOCK_HEAD: u32 = 12;
const BLOCK_CURRENT: u32 = 16;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void* SectionAllocator_Alloc(SectionAllocator* self, int bytes, u32 flags)`
/// (0x3a7878). Align within the current block, then search the section's
/// newest-first block chain, then claim another block from its world pool.
/// A fit requires the end to be strictly below the block's size. The alignment
/// is `4 << ((flags >> 24) & 15)`; a newly claimed block starts at offset zero.
/// Search back edges preserve the original resumable event addresses.
pub fn section_allocator_alloc(c: &mut Cpu) {
    guest(c, |c| {
        match c.pc {
            0x003a_7878 => {
                c.cycles += 10;
                let frame = Frame::open(c, 0x40);
                frame.save(c, S0, 0x30);
                frame.save(c, S1, 0x20);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x10);
                c.g[S1][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                c.g[V0][0] = c.lw(at(c, S0, BLOCK_COUNT));
                let has_blocks = c.g[V0][0] != 0;
                c.g[S2][0] = c.g[A2][0];
                if !has_blocks {
                    claim_first_block(c)?;
                }

                c.cycles += 9;
                c.g[T3][0] = c.lw(at(c, S0, BLOCK_CURRENT));
                c.g[V0][0] = sx(r(c, S2) >> 24);
                c.g[V0][0] &= 15;
                c.g[V1][0] = 4;
                c.g[A3][0] = c.lw(at(c, T3, BLOCK_POSITION));
                c.g[A2][0] = sx(r(c, V1) << (r(c, V0) & 31));
                c.g[T1][0] = 0;
                if current_block_fits(c) {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(r(c, T3));
                    c.g[V1][0] = sx(r(c, A3).wrapping_add(r(c, V1)));
                    c.sw(at(c, T3, BLOCK_POSITION), c.g[V1][0]);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A3)));
                    finish_aligned_allocation(c);
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(at(c, S0, BLOCK_COUNT));
                    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                    let mut search = false;
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S0, BLOCK_HEAD));
                        search = c.g[V0][0] != 0;
                        c.sw(at(c, S0, BLOCK_CURRENT), c.g[V0][0]);
                        if search {
                            c.cycles += 2;
                            c.g[T4][0] = 0;
                        }
                    }
                    if search {
                        search_other_blocks(c)?;
                    } else {
                        claim_and_allocate_block(c)?;
                    }
                }
            }
            0x003a_7978 => search_other_blocks(c)?,
            0x003a_792c => reuse_block(c),
            pc => {
                c.jump_out(pc, 0x003a_7878, 0x003a_7a20);
                return Ok(());
            }
        }
        let frame = Frame { size: 0x40 };
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

fn claim_first_block(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    c.g[RA][0] = sx(0x003a_78a8);
    c.g[A0][0] = c.lw(r(c, S0));
    call_ptr(c, ALLOC_BLOCK, 0x003a_78a8)?;
    c.sw(at(c, V0, BLOCK_POSITION), 0);
    c.sw(at(c, V0, BLOCK_PADDING), 0);
    c.g[V1][0] = c.lw(at(c, S0, BLOCK_HEAD));
    c.sw(at(c, V0, BLOCK_NEXT), c.g[V1][0]);
    c.sw(at(c, S0, BLOCK_CURRENT), c.g[V0][0]);
    c.g[V1][0] = c.lw(at(c, S0, BLOCK_COUNT));
    c.sw(at(c, S0, BLOCK_HEAD), c.g[V0][0]);
    c.g[V1][0] = sx(r(c, V1).wrapping_add(1));
    c.sw(at(c, S0, BLOCK_COUNT), c.g[V1][0]);
    Ok(())
}

fn current_block_fits(c: &mut Cpu) -> bool {
    c.cycles += 10;
    c.div(0, A3, A2);
    c.g[A1][0] = c.lw(at(c, T3, BLOCK_SIZE));
    c.g[A0][0] = c.hi[0];
    c.g[V1][0] = sx(r(c, A2).wrapping_sub(r(c, A0)));
    if c.g[A0][0] != 0 {
        c.g[T1][0] = c.g[V1][0];
    }
    c.g[V0][0] = sx(r(c, A3).wrapping_add(r(c, T1)));
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
    let fits = c.g[V0][0] != 0;
    c.g[V1][0] = sx(r(c, T1).wrapping_add(r(c, S1)));
    fits
}

fn search_other_blocks(c: &mut Cpu) -> Flow {
    loop {
        c.cycles += 3;
        c.g[A3][0] = c.lw(at(c, S0, BLOCK_CURRENT));
        let original_block = c.g[A3][0] == c.g[T3][0];
        c.g[T2][0] = c.g[A3][0];
        if !original_block {
            c.cycles += 4;
            c.g[T0][0] = c.lw(at(c, A3, BLOCK_POSITION));
            c.g[T1][0] = 0;
            c.cycles += 10;
            c.div(0, T0, A2);
            c.g[A1][0] = c.lw(at(c, A3, BLOCK_SIZE));
            c.g[A0][0] = c.hi[0];
            c.g[V1][0] = sx(r(c, A2).wrapping_sub(r(c, A0)));
            if c.g[A0][0] != 0 {
                c.g[T1][0] = c.g[V1][0];
            }
            c.g[V0][0] = sx(r(c, T0).wrapping_add(r(c, T1)));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let fits = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r(c, T1).wrapping_add(r(c, S1)));
            if fits {
                if back_edge_unwinds(c, 0x003a_792c) {
                    return Err(Unwound);
                }
                reuse_block(c);
                return Ok(());
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(at(c, T2, BLOCK_NEXT));
        let next_block = c.g[V0][0] != 0;
        c.sw(at(c, S0, BLOCK_CURRENT), c.g[V0][0]);
        if !next_block {
            return claim_and_allocate_block(c);
        }
        if back_edge_unwinds(c, 0x003a_7978) {
            return Err(Unwound);
        }
    }
}

fn reuse_block(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw(r(c, A3));
    c.g[V1][0] = sx(r(c, T0).wrapping_add(r(c, V1)));
    c.sw(at(c, A3, BLOCK_POSITION), c.g[V1][0]);
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
    finish_aligned_allocation(c);
}

fn finish_aligned_allocation(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T1)));
    c.g[A0][0] = c.lw(at(c, S0, BLOCK_CURRENT));
    c.g[V1][0] = c.lw(at(c, A0, BLOCK_PADDING));
    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, T1)));
    c.sw(at(c, A0, BLOCK_PADDING), c.g[V1][0]);
}

fn claim_and_allocate_block(c: &mut Cpu) -> Flow {
    c.cycles += 16;
    c.g[RA][0] = sx(0x003a_79d0);
    c.g[A0][0] = c.lw(r(c, S0));
    call_ptr(c, ALLOC_BLOCK, 0x003a_79d0)?;
    c.sw(at(c, V0, BLOCK_POSITION), 0);
    c.sw(at(c, V0, BLOCK_PADDING), 0);
    c.g[V1][0] = c.lw(at(c, S0, BLOCK_HEAD));
    c.sw(at(c, V0, BLOCK_NEXT), c.g[V1][0]);
    c.sw(at(c, S0, BLOCK_CURRENT), c.g[V0][0]);
    c.g[A0][0] = c.lw(at(c, S0, BLOCK_COUNT));
    c.sw(at(c, S0, BLOCK_HEAD), c.g[V0][0]);
    c.g[A0][0] = sx(r(c, A0).wrapping_add(1));
    c.sw(at(c, S0, BLOCK_COUNT), c.g[A0][0]);
    c.g[V1][0] = c.lw(at(c, V0, BLOCK_POSITION));
    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, S1)));
    c.sw(at(c, V0, BLOCK_POSITION), c.g[V1][0]);
    c.g[A0][0] = c.lw(at(c, S0, BLOCK_CURRENT));
    c.g[V0][0] = c.lw(r(c, A0));
    Ok(())
}
