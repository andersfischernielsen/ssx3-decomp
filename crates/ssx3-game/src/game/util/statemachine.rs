//! `src/util/statemachine.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::sp;
use crate::ps2f::FpuF32;
pub(crate) use crate::game::worldpainter::quadtree::create_with;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_6f78, console_clock_format_time, "ConsoleClock_FormatTime"),
    (0x002c_7440, c_ps2_console_config_create, "cPS2ConsoleConfig_Create"),
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


/// `void* cPS2ConsoleConfig_Create()` (0x2c7440): allocate the 0x38-byte
/// console configuration and construct it.
pub fn c_ps2_console_config_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 0x38, 0x0048_6560, 0, 0x002c_7460, |c, ret| {
            callee::CONSOLE_CONFIG_CTOR.call(c, ret)
        })
    })
}


// ---------------------------------------------------------------------------
// `ConsoleClock_FormatTime` (0x002c6f78).
//
// `void ConsoleClock::FormatTime(char* out, int pad, int h, int m, int s,
//  int mode)`: format the clock into `out`.

pub fn console_clock_format_time(c: &mut Cpu) {
    guest(c, |c| {

        'b_2c701c: {
        'b_2c7018: {
        'b_2c6fcc: {
        'b_2c6fb4: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S1][0] = c.g[A2][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[A2][0] = c.g[A3][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
        c.g[A3][0] = c.g[T0][0];
        c.g[S0][0] = c.g[A1][0];
        let t = c.g[T2][0] == 0u64;
        c.g[T0][0] = c.g[T1][0];
        if t {
        break 'b_2c6fb4;
        } else {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        let t = c.g[T2][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        if t {
        break 'b_2c6fcc;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[A2][0] = c.g[S1][0];
        if t {
        break 'b_2c701c;
        } else {
        break 'b_2c6fb4;
        }
        }
        }
        }
        c.cycles += 6;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x002c6fc4);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6500));
        call_ptr(c, 0x002c27c0, 0x002c6fc4)?;
        let t = true;
        c.g[A0][0] = c.g[S0][0];
        if t {
        break 'b_2c7018;
        } else {
        break 'b_2c6fcc;
        }
        }
        'b_2c6fec: {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A2][0] as i64) < 12i64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[SP][0];
        if t {
        break 'b_2c6fec;
        } else {
        c.cycles += 5;
        c.g[A1][0] = 0x480000;
        c.g[RA][0] = sx(0x002c6fe4);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6510));
        call_ptr(c, 0x002c27c0, 0x002c6fe4)?;
        let t = true;
        c.g[A0][0] = c.g[S0][0];
        if t {
        break 'b_2c7018;
        } else {
        break 'b_2c6fec;
        }
        }
        }
        'b_2c7000: {
        c.cycles += 5;
        c.g[V0][0] = 0xd;
        c.g[A1][0] = 0x480000;
        c.div(0, 6, 2);
        if c.g[V0][0] == 0u64 {
        break 'b_2c7000;
        } else {
        break 'b_2c7000;
        }
        }
        c.cycles += 6;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6528));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.hi[0];
        c.g[RA][0] = sx(0x002c7014);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xc));
        call_ptr(c, 0x002c27c0, 0x002c7014)?;
        c.g[A0][0] = c.g[S0][0];
        break 'b_2c7018;
        }
        c.cycles += 1;
        c.g[A2][0] = c.g[S1][0];
        break 'b_2c701c;
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x002c7024);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x002c25b8, 0x002c7024)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t; return Ok(());

        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::elf_image;
    use crate::difftest::scramble;
    pub(crate) use crate::game::worldpainter::quadtree::tests::pool_allocator;
    pub(crate) use crate::game::worldpainter::quadtree::tests::MEM_HEAP;
    pub(crate) use crate::game::worldpainter::quadtree::tests::run;
    pub(crate) use crate::game::worldpainter::quadtree::tests::heap_init;
    pub(crate) use crate::game::worldpainter::quadtree::tests::alloc_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;


    #[test]
    fn c_ps2_console_config_create_matches() {
        check(ee::s08::f_002c7440__generated, c_ps2_console_config_create, 60, &|c, seed| {
            alloc_machine(c, seed, 0x2c74_40);
        });
    }


    #[test]
    fn console_clock_format_time_matches() {
        strict(ee::s08::f_002c6f78__generated, console_clock_format_time, 80, &|c, seed| {
            let mut r = Rng(0x002c_6f78 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A1][0] = sx(DATA);
            c.g[A2][0] = (r.next_u32() % 16) as u64;
            c.g[A3][0] = (r.next_u32() % 60) as u64;
            c.g[T0][0] = (r.next_u32() % 60) as u64;
            c.g[T1][0] = (r.next_u32() % 8) as u64;
            c.g[T2][0] = [0u64, 1, 2][(seed % 3) as usize];
        });
    }
}
