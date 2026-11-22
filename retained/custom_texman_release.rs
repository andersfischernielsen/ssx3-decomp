//! Retail TexMan_FreeTexture (0x00367bc0), staged for differential validation.

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const BINDING_LOOP: u32 = 0x0036_7c90;
const TEXTURE_SLOTS: u32 = 8;
const BOUND_IMAGE: u32 = 0x1f50;
const BOUND_CLUT: u32 = 0x1f54;
const MAIN_HEAP: u32 = 0x1f60;
const TEMP_HEAP: u32 = 0x4350;
const FORMAT: u32 = 0x0c;
const IMAGE_DATA: u32 = 0x1c;
const CLUT_DATA: u32 = 0x20;
const IMAGE_VRAM: u32 = 0x28;
const CLUT_VRAM: u32 = 0x2c;
const IMAGE_BLOCK: u32 = 0x30;
const CLUT_BLOCK: u32 = 0x34;
const RETAIL_BOUND_SLOTS: i64 = 1500;
const FREE_VRAM: Callee = Callee {
    addr: 0x0036_63d8,
    f: ssx3_ee::s12::VramHeap_FreeBlock,
};
const FREE_BOUND: Callee = Callee {
    addr: 0x0036_72c0,
    f: ssx3_ee::s12::TexMan_FreeBTexInfo,
};
const FREE_NONBOUND: Callee = Callee {
    addr: 0x0036_71c8,
    f: ssx3_ee::s12::TexMan_FreeNBTexInfo,
};
const FREE_MEMORY: u32 = 0x0031_7e98;

/// Release VRAM blocks and owned buffers, retire the TexInfo slot, and
/// invalidate the currently bound image and CLUT registers in the manager.
pub fn tex_man_free_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x40);

        let manager = c.g[S1][0] as u32;
        let slot_offset = (c.g[S2][0] as u32) << 2;
        c.g[V0][0] = sx(manager.wrapping_add(slot_offset));
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(TEXTURE_SLOTS));
        c.g[S3][0] = u64::MAX;
        let info = c.g[S0][0] as u32;
        let image_resident = c.lw(info.wrapping_add(IMAGE_VRAM)) != c.g[S3][0];
        c.g[V1][0] = sx(manager.wrapping_add(TEMP_HEAP));
        if image_resident {
            c.cycles += 6;
            c.g[V0][0] = c.lw(info.wrapping_add(FORMAT)) ^ 9;
            c.g[A0][0] = if c.g[V0][0] == 0 {
                c.g[V1][0]
            } else {
                sx(manager.wrapping_add(MAIN_HEAP))
            };
            c.g[A1][0] = c.lw(info.wrapping_add(IMAGE_BLOCK));
            FREE_VRAM.call(c, 0x0036_7c14)?;
        }

        c.cycles += 3;
        c.g[V0][0] = c.lw(info.wrapping_add(CLUT_VRAM));
        if c.g[V0][0] != c.g[S3][0] {
            c.cycles += 4;
            c.g[A1][0] = c.lw(info.wrapping_add(CLUT_BLOCK));
            c.g[A0][0] = sx(manager.wrapping_add(MAIN_HEAP));
            FREE_VRAM.call(c, 0x0036_7c2c)?;
        }

        c.cycles += 2;
        let owns_buffers = c.lw(info) != 0;
        c.g[V0][0] = ((c.g[S2][0] as i64) < RETAIL_BOUND_SLOTS) as u64;
        if owns_buffers {
            c.cycles += 3;
            c.g[A0][0] = c.lw(info.wrapping_add(IMAGE_DATA));
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, FREE_MEMORY, 0x0036_7c4c)?;
            }

            c.cycles += 2;
            c.g[A0][0] = c.lw(info.wrapping_add(CLUT_DATA));
            let clut_owned = c.g[A0][0] != 0;
            c.g[V0][0] = ((c.g[S2][0] as i64) < RETAIL_BOUND_SLOTS) as u64;
            if clut_owned {
                c.cycles += 3;
                call_ptr(c, FREE_MEMORY, 0x0036_7c60)?;
                c.g[V0][0] = ((c.g[S2][0] as i64) < RETAIL_BOUND_SLOTS) as u64;
            }
        }

        c.cycles += 2;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            FREE_BOUND.call(c, 0x0036_7c74)?;
        } else {
            c.cycles += 3;
            FREE_NONBOUND.call(c, 0x0036_7c84)?;
        }

        c.cycles += 2;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 1;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(BOUND_IMAGE));
        loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if (c.g[V1][0] as i64) < 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, BINDING_LOOP);
            }
        }

        c.cycles += 9;
        c.g[V0][0] = u64::MAX;
        frame.restore(c, S0, 0x40);
        c.sw((c.g[S1][0] as u32).wrapping_add(BOUND_CLUT), c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{compare, fresh, CYCLE_BUDGET, DATA, RA as RETURN_PC};
    use crate::game::fe::feasyncfile::tests::{globals_12, heap_init_12};
    use crate::game::render::ps2graphicsman::tests::{run_2, vram_heap, TEXMAN, VHEAP, VOWNER};
    use ssx3_ee as ee;

    const ENTRY: u32 = 0x0036_7bc0;
    const EVENT_MARKER: u32 = DATA + 0x10;
    const THREAD_ID: u32 = 0x42;
    const MUTEX_OWNER: u32 = 0x0053_8b04;
    const MUTEX_DEPTH: u32 = 0x0053_8b08;

    fn allocate(c: &mut Cpu, size: u64) -> u32 {
        let ptr = run_2(c, 0x0031_7d70, ee::s10::EA_MemAllocNamed, &[size, 0, 0, 0]) as u32;
        assert_ne!(ptr, 0, "fixture allocation failed");
        ptr
    }

    fn allocate_vram(c: &mut Cpu, heap: u32, owner: u64) -> u32 {
        let block = run_2(
            c,
            0x0036_6ce0,
            ee::s12::VramHeap_Alloc,
            &[heap as u64, 0x100, owner, 7, 50, 0],
        ) as u32;
        assert_ne!(block, u32::MAX, "fixture VRAM allocation failed");
        block
    }

    fn fixture(c: &mut Cpu, seed: u32) {
        machine(c, seed, ENTRY as u64, 0);
        globals_12(c);
        heap_init_12(c);
        // GetThreadId returns the current owner, already holding the mutex.
        c.hooks.syscall = |c, _| c.g[V0][0] = THREAD_ID as u64;
        c.bus.write32(MUTEX_OWNER, THREAD_ID);
        c.bus.write32(MUTEX_DEPTH, 1);

        let slot = 1499 + seed % 2;
        // Sizes above 0x200 go directly to the initialized main heap.
        let info = allocate(c, 0x240);
        let buffers = seed / 2 % 5;
        let image = if matches!(buffers, 0 | 2 | 3) {
            allocate(c, 0x400)
        } else {
            0
        };
        let clut = if matches!(buffers, 0 | 2 | 4) {
            allocate(c, 0x240)
        } else {
            0
        };
        c.bus.write32(info, u32::from(buffers != 0));
        c.bus.write32(info + IMAGE_DATA, image);
        c.bus.write32(info + CLUT_DATA, clut);
        c.bus.write32(info + IMAGE_VRAM, u32::MAX);
        c.bus.write32(info + CLUT_VRAM, u32::MAX);

        let image_kind = seed % 3;
        let clut_resident = seed / 3 % 2 == 1;
        if image_kind != 0 || clut_resident {
            vram_heap(c);
        }
        if image_kind != 0 {
            let heap = if image_kind == 1 {
                VHEAP
            } else {
                let heap = TEXMAN + TEMP_HEAP;
                c.bus.write32(heap + 0x1ff0, DATA + 0x3_0000);
                run_2(
                    c,
                    0x0036_5e40,
                    ee::s12::VramHeap_Init,
                    &[heap as u64, 0, 0x10_0000, VOWNER as u64],
                );
                heap
            };
            let block = allocate_vram(c, heap, 0);
            c.bus
                .write32(info + FORMAT, if image_kind == 2 { 9 } else { 0 });
            c.bus.write32(info + IMAGE_VRAM, 0);
            c.bus.write32(info + IMAGE_BLOCK, block);
        }
        if clut_resident {
            let block = allocate_vram(c, VHEAP, 1);
            c.bus.write32(info + CLUT_VRAM, 0);
            c.bus.write32(info + CLUT_BLOCK, block);
        }

        c.bus.write32(TEXMAN, 2);
        c.bus.write32(TEXMAN + 4, 2);
        c.bus.write32(TEXMAN + TEXTURE_SLOTS + 4 * slot, info);
        c.bus.write32(TEXMAN + BOUND_IMAGE - 4, 21);
        c.bus.write32(TEXMAN + BOUND_IMAGE, 22);
        c.bus.write32(TEXMAN + BOUND_CLUT, 23);
        c.g[A0][0] = TEXMAN as u64;
        c.g[A1][0] = slot as u64;
        c.pc = ENTRY;
    }

    fn binding_event(c: &mut Cpu) -> bool {
        assert_eq!(c.pc, BINDING_LOOP);
        c.bus.write32(EVENT_MARKER, 1);
        c.next_event = CYCLE_BUDGET;
        false
    }

    #[test]
    fn tex_man_free_texture_matches() {
        let mut baseline = fresh();
        fixture(&mut baseline, 0);
        ee::s12::f_00367bc0(&mut baseline);
        assert_eq!(baseline.pc, RETURN_PC);
        let binding_event_cycle = baseline.cycles - 16;

        for seed in 0..20 {
            let diff = compare(ee::s12::f_00367bc0, tex_man_free_texture, &|c| {
                fixture(c, seed);
                if seed == 0 {
                    c.next_event = binding_event_cycle;
                    c.hooks.event = binding_event;
                }
            });
            assert!(
                diff.hard.is_empty() && diff.soft.is_empty() && diff.lohi.is_empty(),
                "seed {seed}: {diff:#?}"
            );
        }
    }
}
