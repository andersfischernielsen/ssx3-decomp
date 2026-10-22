//! `src/util/objectinterface.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::abi::r32;
use crate::call::Flow;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
pub(crate) use crate::game::worldpainter::quadtree::create_with;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_1ce0, world_get_instances_man, "World_GetInstancesMan"),
    (0x002d_2058, world_is_occluded, "World_IsOccluded"),
    (0x002d_2100, ssx_stats_man_create, "SSXStatsMan_Create"),
    (0x002d_21b0, stats_process_update_stub, "StatsProcess_UpdateStub"),
    (0x002d_21b8, stats_process_render_stub, "StatsProcess_RenderStub"),
    (0x002d_21c0, ssx_stats_man_ctor, "SSXStatsMan_ctor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::PS2_SOUND_MANAGER_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_SET_FILE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_STOP;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SPEECH_INSTANCE_STOP_STREAM;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_GET_STREAM_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_POST_EVENT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_FADE_OUT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SNDSTRM_GET_REQUEST_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_SET_STATE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::CONSOLE_CONFIG_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STRCPY;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MENU_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STATS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::AI_VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_GET_SECTION_SCRIPT_FUNC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MOD_OBJ_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::INTC_REMOVE_HANDLER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EMITTER_RENDER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_ON_VIF1_DMA_END;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_MAN_CTOR;
}


/// `void* World_GetInstancesMan(void)` (0x2d1ce0): follow the world singleton
/// to the instance manager at +0x88.
pub fn world_get_instances_man(c: &mut Cpu) {
    c.cycles += 4;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = world;
    let man = Addr(world as u32).off(0x84).lw(c);
    c.g[V1][0] = man;
    c.g[V0][0] = Addr(man as u32).off(0x88).lw(c);
    abi::ret(c);
}


/// `SSXStatsMan* SSXStatsMan_ctor(SSXStatsMan* self)` (0x2d21c0): publish the
/// singleton pointer at `gp + 0x910`.
pub fn ssx_stats_man_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr((c.g[GP][0] as u32).wrapping_add(0x910)).sw(c, c.g[A0][0]);
    abi::ret(c);
}


/// A `gp`-relative global address.
#[inline(always)]
pub(crate) fn gp_at(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}


/// `int World_IsOccluded(sQ128* p)` (0x2d2058): over the occlusion manager
/// `*(world + 0x84) -> +0x8c`, for each instance whose bit is set in the
/// candidate mask (`+0x110`, `count` at `+0x10c`), run the VU0 clip-test
/// microprogram on the instance's six quadwords at `+0x50..+0x90` and the
/// point `p`; the microprogram reports "occluded" in `vi1 == 1`. 1 when any
/// instance occludes the point, else 0.
pub fn world_is_occluded(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = Addr(gp_at(c, -0x848)).lw(c);
    c.g[A1][0] = 0;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
    c.g[A3][0] = Addr(c.g[V1][0] as u32).off(0x8c).lw(c);
    let inst = c.g[A3][0] as u32;
    c.g[V0][0] = Addr(inst).off(0x10c).lw(c);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[T1][0] = Addr(inst).off(0x110).lw(c);
    if !empty {
        c.cycles += 5;
        c.g[V0][0] = 0;
        c.g[T2][0] = 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x828));
        c.g[A2][0] = sx(inst.wrapping_add(0x8));
        // The microprogram address, as VI27 expects it (byte address >> 3).
        c.g[T0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
        loop {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[T2][0] as u32) << ((c.g[A1][0] as u32) & 31));
            c.g[V0][0] = c.g[T1][0] & c.g[V0][0];
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr(inst).off(0x10c).lw(c);
            } else {
                c.cycles += 15;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                let m = c.g[V0][0] as u32;
                c.lqc2(1, m.wrapping_add(0x50));
                c.lqc2(2, m.wrapping_add(0x60));
                c.lqc2(3, m.wrapping_add(0x70));
                c.lqc2(4, m.wrapping_add(0x80));
                c.lqc2(5, m.wrapping_add(0x90));
                c.lqc2(6, c.g[A0][0] as u32);
                c.ctc2(8, 27);
                c.vu0().vnop();
                c.vu0().vnop();
                c.vcallmsr();
                c.cfc2(3, 1);
                c.g[V1][0] ^= 0x1;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = Addr(inst).off(0x10c).lw(c);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    abi::ret(c);
                    return;
                }
            }
            c.cycles += 4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_2088);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Stats and visual effects constructors.

/// `void* SSXStatsMan_Create()` (0x2d2100): allocate the one-byte stats
/// manager singleton and construct it.
pub fn ssx_stats_man_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 1, 0x0048_6758, 0, 0x002d_2120, |c, ret| {
            callee::STATS_MAN_CTOR.call(c, ret)
        })
    })
}


/// `void StatsProcess_RenderStub(void)` (0x2d21b8): stats render hook, not
/// installed.
pub fn stats_process_render_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}


/// `void StatsProcess_UpdateStub(void)` (0x2d21b0): stats update hook, not
/// installed.
pub fn stats_process_update_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_vec;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::elf_image;
    use crate::difftest::put_words;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::object::debouncenode::tests::SEEDS;
    pub(crate) use crate::game::object::debouncenode::tests::base;
    pub(crate) use crate::game::worldpainter::quadtree::tests::pool_allocator;
    pub(crate) use crate::game::worldpainter::quadtree::tests::MEM_HEAP;
    pub(crate) use crate::game::worldpainter::quadtree::tests::run;
    pub(crate) use crate::game::worldpainter::quadtree::tests::heap_init;
    pub(crate) use crate::game::worldpainter::quadtree::tests::alloc_machine;
    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::leaf;


    #[test]
    fn world_get_instances_man_matches() {
        check(ssx3_ee::s09::f_002d1ce0, world_get_instances_man, 50, &|c, seed| {
            let mut r = Rng(0x1ce0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = DATA + 0x100;
            let man = DATA + 0x200;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, man);
            c.bus.write32(man + 0x88, r.next_u32());
        });
    }


    #[test]
    fn ssx_stats_man_ctor_matches() {
        check(ssx3_ee::s09::f_002d21c0, ssx_stats_man_ctor, 50, &|c, seed| {
            let mut r = Rng(0x21c0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }


    /// The VU0 microprogram at 0x828 the occlusion test runs: `vi1 = vi14`.
    /// Hand-written microcode has no translation: use the reference
    /// interpreter.
    pub(crate) fn put_occlusion_microprogram(c: &mut Cpu) {
        ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.vu0.fallback =
            Some(ssx3_ps2::vu::reference::run_program);
        let at = 0x1100_0000 + 0x828;
        c.bus.write32(at, 0x8000_7070); // IADD vi1, vi14, vi0
        c.bus.write32(at + 4, 0x4000_02ff); // NOP, E
    }


    #[test]
    fn world_is_occluded_matches() {
        check(ee::s09::f_002d2058__generated, world_is_occluded, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x2d20);
            put_occlusion_microprogram(c);
            // vi14 (the object's clip word the microprogram copies to vi1):
            // 1 means "occluded".
            c.g[14][0] = (seed % 3) as u64;
            c.ctc2(14, 14);
            let (world, mgr, inst, pts) = (DATA, DATA + 0x100, DATA + 0x200, DATA + 0x3000);
            c.bus.write32((GAME_GP as u32).wrapping_sub(0x848), world);
            c.bus.write32(world + 0x84, mgr);
            c.bus.write32(mgr + 0x8c, inst);
            let count = (seed % 4) as u32;
            c.bus.write32(inst + 0x10c, count);
            c.bus.write32(inst + 0x110, if count == 0 { 0 } else { (r.next_u32() | 1) & 0xf });
            for i in 0..3u32 {
                let m = DATA + 0x1000 + 0x100 * i;
                c.bus.write32(inst + 8 + 8 * i, m);
                for q in 0..5u32 {
                    put_vec(
                        c,
                        m + 0x50 + 0x10 * q,
                        [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                    );
                }
            }
            put_vec(c, pts, [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
            c.g[A0][0] = sx(pts);
        });
    }


    #[test]
    fn ssx_stats_man_create_matches() {
        check(ee::s09::f_002d2100__generated, ssx_stats_man_create, 60, &|c, seed| {
            alloc_machine(c, seed, 0x2d21_00);
        });
    }


    #[test]
    fn stats_process_render_stub_matches() {
        leaf(ssx3_ee::s09::f_002d21b8__generated, stats_process_render_stub, 7);
    }


    #[test]
    fn stats_process_update_stub_matches() {
        leaf(ssx3_ee::s09::f_002d21b0__generated, stats_process_update_stub, 8);
    }
}
