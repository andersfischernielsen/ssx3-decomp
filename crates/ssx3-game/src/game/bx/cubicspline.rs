//! `src/bx/cubicspline.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::float;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_e1d8, dlheap_large_bin_index, "DLHeap_LargeBinIndex"),
    (0x0031_fbb8, dlheap_usable_size, "DLHeap_UsableSize"),
    (0x0031_ff60, dl_heap_init_arena, "DLHeap_InitArena"),
    (0x0032_0488, mem_pool8_free, "MEM_Pool8Free"),
    (0x0032_0498, mem_pool16_free, "MEM_Pool16Free"),
    (0x0032_04a8, mem_pool32_free, "MEM_Pool32Free"),
    (0x0032_04b8, mem_pool64_free, "MEM_Pool64Free"),
    (0x0032_04d0, mem_pool128_free, "MEM_Pool128Free"),
    (0x0032_04e8, mem_pool256_free, "MEM_Pool256Free"),
    (0x0032_0500, mem_pool512_free, "MEM_Pool512Free"),
];

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPRINTF;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_TEARDOWN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
}

/// `int DLHeap_LargeBinIndex(uint size)` (0x31e1d8): the free-list bin for a
/// large allocation of `size` bytes (0x5f when the size index is too big).
pub fn dlheap_large_bin_index(c: &mut Cpu) {
    c.cycles += 6;
    let size = abi::arg(c, 0);
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = 0xffff;
    c.g[A0][0] = sx(size >> 8);
    c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
    let too_big = c.g[V0][0] != 0;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x100));
    if too_big {
        c.cycles += 2;
        c.g[V0][0] = 0x5f;
        abi::ret(c);
        return;
    }
    c.cycles += 26;
    c.g[V1][0] = sx((c.g[V1][0] as u32) >> 16);
    c.g[V1][0] = c.g[V1][0] & 0x8;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[V1][0] as u32) & 31));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1000));
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 16);
    c.g[A1][0] = c.g[V0][0] & 0x4;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[A1][0] as u32) & 31));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4000));
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 16);
    c.g[A1][0] = c.g[V0][0] & 0x2;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[A1][0] as u32) & 31));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = sx((c.g[A0][0] as u32) >> 15);
    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 14);
    c.g[V0][0] = !(0u64 | c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xd));
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6));
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> ((c.g[V0][0] as u32) & 31));
    c.g[V0][0] = c.g[V0][0] & 0x3;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `uint DLHeap_UsableSize(void *block)` (0x31fbb8): usable payload size of a
/// heap block, from its header word at `block-4`.
pub fn dlheap_usable_size(c: &mut Cpu) {
    let a1 = abi::arg(c, 1);
    'b_31fc18: {
        c.cycles += 2;
        if a1 == 0 {
            break 'b_31fc18;
        }
        c.cycles += 4;
        c.g[A0][0] = Addr(a1.wrapping_sub(4)).lw(c);
        c.g[V0][0] = c.g[A0][0] & 0x2;
        let small = c.g[V0][0] == 0;
        c.g[A1][0] = sx(a1.wrapping_sub(8));
        if small {
            c.cycles += 8;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[V0][0] = c.g[V0][0] | 0xfffe;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4).lw(c);
            c.g[V1][0] = c.g[V1][0] & 0x1;
            if c.g[V1][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = 0xffff_ffff_ffff_0000;
                c.g[V0][0] = c.g[V0][0] | 0xfffc;
                c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
                abi::ret(c);
                return;
            }
        } else {
            c.cycles += 5;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[V0][0] = c.g[V0][0] | 0xfffc;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(8));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void MEM_Pool8Free(MEM_Pool8 *pool, void *ptr)` (0x320488): push `ptr` on
/// the pool's free list at +0x800.
pub fn mem_pool8_free(c: &mut Cpu) {
    c.cycles += 4;
    let pool = abi::arg(c, 0);
    c.g[V0][0] = Addr(pool).off(0x800).lw(c);
    Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
    Addr(pool).off(0x800).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void MEM_Pool16Free(MEM_Pool16 *pool, void *ptr)` (0x320498): push `ptr`
/// on the pool's free list at +0x2000.
pub fn mem_pool16_free(c: &mut Cpu) {
    c.cycles += 4;
    let pool = abi::arg(c, 0);
    c.g[V0][0] = Addr(pool).off(0x2000).lw(c);
    Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
    Addr(pool).off(0x2000).sw(c, c.g[A1][0]);
    abi::ret(c);
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

/// `DLHeap_InitArena` (0x31FF60): carve a 16-byte aligned block out of an arena
/// and initialise the heap header in it.
pub fn dl_heap_init_arena(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = low(c, A1).wrapping_add(0xf);
        c.g[V1][0] |= 0xfff0;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x20));
        c.g[V0][0] &= c.g[V1][0];
        c.sq(16, sp(c).wrapping_add(0x10));
        c.g[A1][0] = low(c, V0).wrapping_sub(low(c, A1));
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = low(c, A2).wrapping_sub(low(c, A1));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A2][0] &= c.g[V1][0];
        c.g[A0][0] = c.g[V0][0];
        sw_at(c, S0, 0xc, c.g[A2][0]);
        c.g[A1][0] = 0;
        sw_at(c, S0, 0x4, c.g[V0][0]);
        c.g[A2][0] = 0x378;
        sw_at(c, S0, 0, c.g[V0][0]);
        call_ptr(c, addr::MEMSET, 0x0031_ffa8)?;
        c.g[V1][0] = lw_at(c, S0, 0x4);
        c.g[V0][0] = lw_at(c, S0, 0xc);
        c.g[V1][0] = low(c, V1).wrapping_add(0x380);
        c.g[RA][0] = Addr(sp(c)).ld(c);
        c.g[V0][0] = low(c, V0).wrapping_sub(0x380);
        sw_at(c, S0, 0x8, c.g[V1][0]);
        sw_at(c, S0, 0xc, c.g[V0][0]);
        sw_at(c, S0, 0x4, c.g[V1][0]);
        c.lq(16, sp(c).wrapping_add(0x10));
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x20));
        c.pc = ret;
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS;

    #[test]
    fn dlheap_large_bin_index_matches() {
        check(
            ssx3_ee::s10::f_0031e1d8,
            dlheap_large_bin_index,
            300,
            &|c, seed| {
                let mut r = Rng(0x201b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 6 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    2 => 0x0100_0000,
                    3 => 0x00ff_ffff,
                    _ => r.next_u32(),
                } as u64;
            },
        );
    }

    #[test]
    fn dlheap_usable_size_matches() {
        check(
            ssx3_ee::s10::f_0031fbb8,
            dlheap_usable_size,
            300,
            &|c, seed| {
                let mut r = Rng(0x201c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = if seed % 8 == 0 {
                    0
                } else {
                    (DATA + 0x100) as u64
                };
                c.bus.write32(DATA + 0xfc, r.next_u32());
                for i in 0..16 {
                    c.bus.write32(DATA + 0x100 + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn mem_pool8_free_matches() {
        check(ssx3_ee::s10::f_00320488, mem_pool8_free, 100, &|c, seed| {
            let mut r = Rng(0x201d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA + 0x800, r.next_u32());
        });
    }

    #[test]
    fn mem_pool16_free_matches() {
        check(
            ssx3_ee::s10::f_00320498,
            mem_pool16_free,
            100,
            &|c, seed| {
                let mut r = Rng(0x201e ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.bus.write32(DATA + 0x2000, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_pool32_free_matches() {
        check(
            ssx3_ee::s10::f_003204a8,
            mem_pool32_free,
            200,
            &|c, seed| {
                let mut r = Rng(0x1001 ^ seed as u64);
                scramble(c, &mut r);
                let pool = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = pool as u64;
                c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
                c.bus.write32(pool + 0x6000, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_pool64_free_matches() {
        check(
            ssx3_ee::s10::f_003204b8,
            mem_pool64_free,
            200,
            &|c, seed| {
                let mut r = Rng(0x1002 ^ seed as u64);
                scramble(c, &mut r);
                let pool = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = pool as u64;
                c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
                c.bus.write32(pool + 0x30000 + 0x2000, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_pool128_free_matches() {
        check(
            ssx3_ee::s10::f_003204d0,
            mem_pool128_free,
            200,
            &|c, seed| {
                let mut r = Rng(0x1003 ^ seed as u64);
                scramble(c, &mut r);
                let pool = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = pool as u64;
                c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
                c.bus.write32(pool + 0x20000 + 0x5800, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_pool256_free_matches() {
        check(
            ssx3_ee::s10::f_003204e8,
            mem_pool256_free,
            200,
            &|c, seed| {
                let mut r = Rng(0x1004 ^ seed as u64);
                scramble(c, &mut r);
                let pool = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = pool as u64;
                c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
                c.bus.write32(pool + 0x20000, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_pool512_free_matches() {
        check(
            ssx3_ee::s10::f_00320500,
            mem_pool512_free,
            200,
            &|c, seed| {
                let mut r = Rng(0x1005 ^ seed as u64);
                scramble(c, &mut r);
                let pool = DATA + 0x100;
                let block = DATA + 0x200;
                c.g[A0][0] = pool as u64;
                c.g[A1][0] = (block as u64) | ((r.next_u32() as u64) << 32);
                c.bus.write32(pool + 0x20000, r.next_u32());
            },
        );
    }

    #[test]
    fn dl_heap_init_arena_matches() {
        check(
            ee::s10::f_0031ff60__generated,
            dl_heap_init_arena,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x31_ff60, 1);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x3000 + 0x1000 * (seed % 4) + seed % 8) as u64;
                c.g[A2][0] = (0x4000 + 0x1000 * (seed % 3)) as u64;
                c.pc = 0x0031_ff60;
            },
        );
    }
}
