//! Hooks for the fork's in-place edits of the load-screen, results, world
//! cache and texture code (the Select Peak menu edits are gone: the engine's
//! plugin menu builds the rows from `plugin.toml`). Every row of the table is one upstream function
//! the SSX Tricky fork changed inside its body; here each becomes a hook.
//!
//! Almost everything is a wrapper (`CHAINS`): code before and/or after
//! `PREV.call(c)`. A wrapper is entered at the function's address
//! (`c.pc == entry`); when a callee unwound and the function is re-entered in
//! the middle, the wrapper forwards to `PREV` untouched. Only the plugin's own
//! paths (custom course active) deviate from the retail
//! cycle and register contract.
//!
//! | function | address | kind | what changed | uncertain |
//! |---|---|---|---|---|
//! | `GameStateFinish_Enter` | 0x00233c50 | chain (diagnostic) | finish trace `finish-enter`, `finish-delay-armed` | the second observation is made at the function's end |
//! | `GameUI_OpenFinishScreen` | 0x0020cc60 | chain (diagnostic) | finish trace; the per-branch `request`/`returned` details (0xc, 0xd) are reduced to enter/returned | diagnostic only |
//! | `SectionMan_Init` | 0x0022cd40 | chain, after | `remap_section(section_man)` | the fork called it before `SectionMan_InitNodePool`; it only reads memory and writes the slot map, so after is equivalent; a resumed call takes the manager from S2 |
//! | `SectionMan_GetEventWorldSection` | 0x0022e098 | chain, after | `v0 = event_world_section(v0)` | none |
//! | `GameStateResults_Update` | 0x00236e60 | chain, handles two custom cases itself | pending results screen retry; custom course leaves through `GameModule_QuitToLoadScreen(module, 0)` | cycles of the custom paths are approximate |
//! | `cPreFELoadScreen_Update` | 0x00232a08 | chain, before | `deactivate` when the app has no next module | the fork deactivated after the fade step of the same call |
//! | `TexMan_FreeNBTexInfo` | 0x003671c8 | chain, before | world texture slots 1500..world_texture_slots go to `TexMan_FreeBTexInfo` | none |
//! | `WorldBlockPool_AllocBlock` | 0x003a7768 | chain, before/after | lazy geometry blocks (128 KiB pools): allocate the payload on claim | none |
//! | `WorldBlockPool_FreeBlock` | 0x003a7790 | chain, before/after | free that payload on release | none |
//! | `WorldBlockPool_Init` | 0x003a7598 | chain; own copy of upstream's body for lazy pools only | no per-block payload allocation for lazy pools; other pools run PREV | the copy follows upstream at the time of writing |
//!
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use ssx3_game::call::{call_ptr, guest, sp, Frame};
use ssx3_game::events::{deliver_event_here, event_due};
use ssx3_plugin::Prev;

/// No full replacements: everything is a wrapper.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

static PREV_FINISH_ENTER: Prev = Prev::new();
static PREV_OPEN_FINISH_SCREEN: Prev = Prev::new();
static PREV_SECTION_MAN_INIT: Prev = Prev::new();
static PREV_SECTION_EVENT_WORLD: Prev = Prev::new();
static PREV_RESULTS_UPDATE: Prev = Prev::new();
static PREV_PRE_FE_UPDATE: Prev = Prev::new();
static PREV_FREE_NB_TEX_INFO: Prev = Prev::new();
static PREV_ALLOC_BLOCK: Prev = Prev::new();
static PREV_FREE_BLOCK: Prev = Prev::new();
static PREV_POOL_INIT: Prev = Prev::new();

pub static CHAINS: &[crate::Chain] = &[
    crate::Chain {
        addr: 0x0023_3c50,
        f: game_state_finish_enter,
        prev: &PREV_FINISH_ENTER,
        name: "GameStateFinish_Enter",
    },
    crate::Chain {
        addr: 0x0020_cc60,
        f: game_ui_open_finish_screen,
        prev: &PREV_OPEN_FINISH_SCREEN,
        name: "GameUI_OpenFinishScreen",
    },
    crate::Chain {
        addr: 0x0022_cd40,
        f: section_man_init,
        prev: &PREV_SECTION_MAN_INIT,
        name: "SectionMan_Init",
    },
    crate::Chain {
        addr: 0x0022_e098,
        f: section_man_get_event_world_section,
        prev: &PREV_SECTION_EVENT_WORLD,
        name: "SectionMan_GetEventWorldSection",
    },
    crate::Chain {
        addr: 0x0023_6e60,
        f: game_state_results_update,
        prev: &PREV_RESULTS_UPDATE,
        name: "GameStateResults_Update",
    },
    crate::Chain {
        addr: 0x0023_2a08,
        f: c_pre_fe_load_screen_update,
        prev: &PREV_PRE_FE_UPDATE,
        name: "cPreFELoadScreen_Update",
    },
    crate::Chain {
        addr: 0x0036_71c8,
        f: tex_man_free_nb_tex_info,
        prev: &PREV_FREE_NB_TEX_INFO,
        name: "TexMan_FreeNBTexInfo",
    },
    crate::Chain {
        addr: 0x003a_7768,
        f: world_block_pool_alloc_block,
        prev: &PREV_ALLOC_BLOCK,
        name: "WorldBlockPool_AllocBlock",
    },
    crate::Chain {
        addr: 0x003a_7790,
        f: world_block_pool_free_block,
        prev: &PREV_FREE_BLOCK,
        name: "WorldBlockPool_FreeBlock",
    },
    crate::Chain {
        addr: 0x003a_7598,
        f: world_block_pool_init,
        prev: &PREV_POOL_INIT,
        name: "WorldBlockPool_Init",
    },
];

// ---------------------------------------------------------------- helpers

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// The word at `gp - negative_offset`.
#[inline]
fn gp_word(c: &mut Cpu, negative_offset: u32) -> u32 {
    let address = r(c, GP).wrapping_sub(negative_offset);
    c.bus.read32(address)
}

// ------------------------------------------------------ finish traces

fn game_state_finish_enter(c: &mut Cpu) {
    let this = r(c, A0);
    let back = r(c, RA);
    let at_entry = c.pc == 0x0023_3c50;
    if at_entry {
        crate::custom_finish_trace::observe(c, "finish-enter", this, 0);
    }
    PREV_FINISH_ENTER.call(c);
    if at_entry && c.pc == back {
        crate::custom_finish_trace::observe(c, "finish-delay-armed", this, 0);
    }
}

fn game_ui_open_finish_screen(c: &mut Cpu) {
    let back = r(c, RA);
    let at_entry = c.pc == 0x0020_cc60;
    if at_entry {
        crate::custom_finish_trace::observe(c, "finish-screen-enter", 0, 0);
    }
    PREV_OPEN_FINISH_SCREEN.call(c);
    if at_entry && c.pc == back {
        crate::custom_finish_trace::observe(c, "finish-screen-returned", 0, 0);
    }
}

// ----------------------------------------------------------- SectionMan

const SECTION_MAN_INIT: u32 = 0x0022_cd40;
const SECTION_MAN_INIT_END: u32 = 0x0022_cea8;

/// After the retail init has built the logical section map, lease the
/// template's slot to the custom world section. Re-entered in the middle
/// (after a yield), the manager is in S2.
fn section_man_init(c: &mut Cpu) {
    let man = if c.pc == SECTION_MAN_INIT { r(c, A0) } else { r(c, S2) };
    PREV_SECTION_MAN_INIT.call(c);
    if !(SECTION_MAN_INIT..SECTION_MAN_INIT_END).contains(&c.pc) {
        crate::custom_tracks::remap_section(c, man);
    }
}

fn section_man_get_event_world_section(c: &mut Cpu) {
    PREV_SECTION_EVENT_WORLD.call(c);
    c.g[V0][0] = sx(crate::custom_tracks::event_world_section(r(c, V0)));
}

// ------------------------------------------------ GameStateResults_Update

const RESULTS_UPDATE: u32 = 0x0023_6e60;
const OPEN_SCREEN_FOR_STATE: u32 = 0x0020_a8f8;
const QUIT_TO_LOAD_SCREEN: u32 = 0x0023_0488;
const RESULTS_UPDATE_CALL_RETURN: u32 = 0x0023_6e90;

fn game_state_results_update(c: &mut Cpu) {
    if c.pc != RESULTS_UPDATE {
        PREV_RESULTS_UPDATE.call(c);
        return;
    }
    let results = r(c, A0);
    if crate::custom_results_enter::retry_pending_screen(c, results) {
        // Retry the results screen request that was rejected earlier.
        guest(c, |c| {
            c.cycles += 6;
            let frame = Frame::open(c, 0x10);
            c.g[V0][0] = 1;
            frame.save_ra(c, 0);
            c.g[A0][0] = 7;
            crate::custom_finish_trace::observe(c, "results-screen-retry", 0, results);
            call_ptr(c, OPEN_SCREEN_FOR_STATE, RESULTS_UPDATE_CALL_RETURN)?;
            crate::custom_finish_trace::observe(c, "results-screen-retry-returned", 0, results);
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            Ok(())
        });
        return;
    }
    if crate::custom_tracks::active().is_some() && c.bus.read32(results.wrapping_add(8)) == 1 {
        // Additional courses have no Conquer-the-Mountain transport
        // destination: leave through the native front-end load path and keep
        // the custom context until the world has shut down.
        guest(c, |c| {
            c.cycles += 13;
            let frame = Frame::open(c, 0x10);
            frame.save_ra(c, 0);
            let world = gp_word(c, 0x848);
            let module = c.bus.read32(world.wrapping_add(0x84));
            c.g[A0][0] = sx(module);
            c.bus.write32(results.wrapping_add(8), 0);
            c.g[A1][0] = 0;
            call_ptr(c, QUIT_TO_LOAD_SCREEN, RESULTS_UPDATE_CALL_RETURN)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            Ok(())
        });
        return;
    }
    PREV_RESULTS_UPDATE.call(c);
}

// ------------------------------------------------ cPreFELoadScreen_Update

/// Once the app has no next module the front end is about to be built:
/// restore the section and residency mappings of the custom course first.
fn c_pre_fe_load_screen_update(c: &mut Cpu) {
    if c.pc == 0x0023_2a08 && crate::custom_tracks::active().is_some() {
        let app = gp_word(c, 0x848);
        if app != 0 && c.bus.read32(app.wrapping_add(4)) == 0 {
            crate::custom_tracks::deactivate(c);
        }
    }
    PREV_PRE_FE_UPDATE.call(c);
}

// ----------------------------------------------- TexMan_FreeNBTexInfo

/// TexMan_FreeTexture still dispatches by the retail boundary: the extended
/// world slots (1500 up to the imported count) have bound TexInfos and must
/// not join the dynamic free list.
fn tex_man_free_nb_tex_info(c: &mut Cpu) {
    if c.pc == 0x0036_71c8 {
        let id = r(c, A1);
        if (1500..crate::custom_tracks::world_texture_slots()).contains(&id) {
            crate::thunk::<0x0036_72c0>(c);
            return;
        }
    }
    PREV_FREE_NB_TEX_INFO.call(c);
}

// ------------------------------------------------------- WorldBlockPool

const ALLOC_BLOCK: u32 = 0x003a_7768;
const FREE_BLOCK: u32 = 0x003a_7790;
const POOL_INIT: u32 = 0x003a_7598;
const HEAP_ALLOC: u32 = 0x0031_7e30;
const HEAP_FREE: u32 = 0x0031_7e98;

// Converted Tricky worlds keep their full-resolution shared textures in the
// named heap. Reserving all 53 geometry payloads at startup strands
// megabytes in the pool (Alaska uses only 27), starving its final lightmaps.
// Keep the native block count/packing; allocate only claimed payloads and
// return them when their section is released, including across course swaps.
//
// Only pools made while a Tricky course is active are lazy: the game's own
// courses keep the game's pools (a plugin that is merely installed must not
// change them). The pools made lazy are remembered by address.
static LAZY_POOLS: std::sync::Mutex<Vec<u32>> = std::sync::Mutex::new(Vec::new());

fn lazy_geometry_pool(_c: &mut Cpu, pool: u32) -> bool {
    LAZY_POOLS.lock().unwrap().contains(&pool)
}

/// A heap call can yield. Returning to the entry with its own address in RA
/// resumes the saved frame rather than claiming a second block.
fn world_block_pool_alloc_block(c: &mut Cpu) {
    if r(c, RA) == ALLOC_BLOCK {
        finish_lazy_block_alloc(c);
        return;
    }
    let pool = ssx3_game::abi::arg(c, 0);
    let block = c.bus.read32(pool.wrapping_add(0x10));
    if lazy_geometry_pool(c, pool) && c.bus.read32(block) == 0 {
        // SectionAllocator keeps its request in S1 at both block-claim
        // sites. Retail assumes every single resource fits the pool size;
        // an imported WorldScript can exceed it without being splittable.
        // Give that descriptor a large enough backing allocation.
        let bytes = if matches!(r(c, RA), 0x003a_78a8 | 0x003a_79d0) {
            // Its first-block fit check is strict (<), so keep at least one
            // byte spare, rounded to the native sixteen-byte alignment.
            c.bus.read32(pool).max(r(c, S1).saturating_add(16) & !15)
        } else {
            c.bus.read32(pool)
        };
        guest(c, |c| {
            save_lazy_block_frame(c);
            c.g[S0][0] = c.g[A0][0];
            c.g[S1][0] = c.lw(r(c, S0).wrapping_add(0x10));
            c.g[S2][0] = sx(bytes);
            c.g[A0][0] = sx(bytes);
            c.g[A1][0] = 0x0049_4dd8;
            c.g[A2][0] = 0x2500_0000;
            c.g[A3][0] = 0;
            call_ptr(c, HEAP_ALLOC, ALLOC_BLOCK)?;
            finish_lazy_block_alloc(c);
            Ok(())
        });
        return;
    }
    PREV_ALLOC_BLOCK.call(c);
}

fn finish_lazy_block_alloc(c: &mut Cpu) {
    c.sw(r(c, S1), c.g[V0][0]);
    c.sw(r(c, S1).wrapping_add(4), c.g[S2][0]);
    restore_lazy_block_frame(c);
    PREV_ALLOC_BLOCK.call(c);
}

fn world_block_pool_free_block(c: &mut Cpu) {
    if r(c, RA) == FREE_BLOCK {
        finish_lazy_block_free(c);
        return;
    }
    let pool = ssx3_game::abi::arg(c, 0);
    let block = ssx3_game::abi::arg(c, 1);
    if lazy_geometry_pool(c, pool) && c.bus.read32(block) != 0 {
        guest(c, |c| {
            save_lazy_block_frame(c);
            c.g[S1][0] = c.g[A1][0];
            c.g[A0][0] = c.lw(r(c, S1));
            call_ptr(c, HEAP_FREE, FREE_BLOCK)?;
            finish_lazy_block_free(c);
            Ok(())
        });
        return;
    }
    PREV_FREE_BLOCK.call(c);
}

fn finish_lazy_block_free(c: &mut Cpu) {
    c.sw(r(c, S1), 0);
    restore_lazy_block_frame(c);
    // A later ordinary allocation must not inherit a released oversized
    // block's capacity while receiving only the usual 128 KiB payload.
    let bytes = c.bus.read32(ssx3_game::abi::arg(c, 0));
    c.sw(ssx3_game::abi::arg(c, 1).wrapping_add(4), bytes as u64);
    PREV_FREE_BLOCK.call(c);
}

// The added heap calls must leave the leaf allocator's callers' scratch
// registers intact as well as the callee-saved registers. Keep this frame in
// game memory so a thread switch can resume it at either leaf entry.
fn save_lazy_block_frame(c: &mut Cpu) {
    let frame = Frame::open(c, 0x240);
    for register in 1..32 {
        if register != SP {
            frame.save(c, register, register as u32 * 16);
        }
    }
    for unit in 0..2 {
        c.sd(sp(c).wrapping_add(0x200 + unit as u32 * 16), c.lo[unit]);
        c.sd(sp(c).wrapping_add(0x208 + unit as u32 * 16), c.hi[unit]);
    }
}

fn restore_lazy_block_frame(c: &mut Cpu) {
    let frame = Frame { size: 0x240 };
    for unit in 0..2 {
        c.lo[unit] = c.ld(sp(c).wrapping_add(0x200 + unit as u32 * 16));
        c.hi[unit] = c.ld(sp(c).wrapping_add(0x208 + unit as u32 * 16));
    }
    for register in 1..32 {
        if register != SP {
            frame.restore(c, register, register as u32 * 16);
        }
    }
    frame.close(c);
}

/// Retail pools run the previous function. A lazy pool (block size 0x20000,
/// custom assets present) is initialised by this copy of the body, which
/// allocates no per-block payload (`+0` stays 0 until the block is claimed).
fn world_block_pool_init(c: &mut Cpu) {
    if c.pc == POOL_INIT {
        let pool = r(c, A0);
        let lazy = crate::custom_tracks::active().is_some() && r(c, A2) == 0x20000;
        let mut pools = LAZY_POOLS.lock().unwrap();
        pools.retain(|p| *p != pool);
        if lazy {
            pools.push(pool);
            drop(pools);
            eprintln!("[tricky] pool {pool:#x}: payloads are allocated when claimed");
            lazy_pool_init(c);
            return;
        }
    }
    PREV_POOL_INIT.call(c);
}

/// `WorldBlockPool_Init` (0x003a7598) as upstream's body, with the per-block
/// payload allocation of the loop skipped (`v0 = 0` instead of the heap call).
fn lazy_pool_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.g[A1][0]; // count
        frame.save(c, S1, 0x50);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A0][0]; // this
        frame.save(c, S0, 0x60);
        c.g[A0][0] = 0x18; // block size
        frame.save(c, S2, 0x40);
        c.mult(0, 0, 2, 4); // blocks * 0x18
        frame.save(c, S4, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4dc0));
        frame.save(c, S5, 0x10);
        c.g[A3][0] = 0u64;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0u64;
        c.sw(c.g[S1][0] as u32, c.g[A2][0]);
        c.g[A0][0] = c.lo[0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A2][0] = 0x2000_0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, HEAP_ALLOC, 0x003a_75f0)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let empty = c.g[V1][0] == 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        if !empty {
            c.cycles += 3;
            c.g[S5][0] = 0x49_0000;
            c.g[S4][0] = 0x18;
            c.g[S2][0] = 0u64;
            loop {
                c.cycles += 35;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4dd8));
                c.g[A2][0] = 0x2500_0000;
                c.g[A3][0] = 0u64;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), 0u64);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                // Lazy pool: no payload until the block is claimed.
                c.g[V0][0] = 0;
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[V1][0] = (c.g[S3][0] < c.g[V1][0]) as u64;
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = 0u64;
                }
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_7608);
                }
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        frame.restore(c, S0, 0x60);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
