//! `src/main/sectionman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0022_e228,
        section_man_init_node_pool,
        "SectionMan_InitNodePool",
    ),
    (
        0x0022_e278,
        section_man_pop_free_node,
        "SectionMan_PopFreeNode",
    ),
    (
        0x0022_e288,
        section_man_push_free_node,
        "SectionMan_PushFreeNode",
    ),
    (
        0x0022_e298,
        section_man_queue_section,
        "SectionMan_QueueSection",
    ),
];

// ---------------------------------------------------------------------------
// Callees (address + the generated shard function, which dispatches through
// its override slot). Entries for calls that the generator emitted as
// `call_indirect` are kept for reference next to the `call_ptr` sites.

#[allow(dead_code)]
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_CTOR_COPY;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_DTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_INIT_FROM_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAM_CONTROLLER_RESTORE_DEFAULT;
    pub(crate) use crate::game::bx::bxstring2::callee::CHUNK_RESOLVE_RELOCATE_DMA;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN3D_GROUP_BUILD_MATRICES;
    pub(crate) use crate::game::bx::bxstring2::callee::CUTSCENE_TRANSITION_MAN_FORCE_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::bx::bxstring2::callee::FLAG_MANAGER_GROUP_BUILD_MESH;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_SKIP_WHITESPACE;
    pub(crate) use crate::game::bx::bxstring2::callee::MEMSET;
    pub(crate) use crate::game::bx::bxstring2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_MAN_CLEAR;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_SET_EVENT_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_MANAGER;
    pub(crate) use crate::game::bx::bxstring2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::bx::bxstring2::callee::OPTIONS_SET_VIBRATION;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_FLOAT_3 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WRAPPED_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT;
    pub(crate) use crate::game::bx::bxstring2::callee::READ_PRIVATE_MESSAGE_SET_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::RIDER_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_DEQUEUE_SECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_POP_FREE_NODE;
    pub(crate) use crate::game::bx::bxstring2::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN;
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_DESIRED_EYE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_LOOK_AT;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_ADD_ITEM_STR_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_FIND_ITEM_BY_VALUE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2 as USTR_FROM_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2 as WSCRIPT_PROCESS_LIST_PUSH_FRONT;
}

/// Offset of the section manager's free-node list head.
pub(crate) const SECTION_MAN_FREE_LIST: u32 = 0x198;

/// Offset of a free node's `next` link.
pub(crate) const SECTION_FREE_NODE_NEXT: u32 = 0x4;

/// `void SectionMan_PushFreeNode(SectionMan* m, void* node)` (0x0022e288):
/// pushes `node` onto the manager's singly linked free list; `v0` is left
/// holding the previous head.
pub fn section_man_push_free_node(c: &mut Cpu) {
    c.cycles += 4;
    let man = Addr(abi::arg(c, 0));
    let node = abi::arg(c, 1);
    let head = man.off(SECTION_MAN_FREE_LIST).lw(c);
    c.g[V0][0] = head;
    Addr(node).off(SECTION_FREE_NODE_NEXT).sw(c, head);
    man.off(SECTION_MAN_FREE_LIST).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void *SectionMan_PopFreeNode(SectionMan *man)` (0x22e278): pop the free
/// list head at `+0x198`, returning it and making its `next` (`+4`) the new
/// head.
pub fn section_man_pop_free_node(c: &mut Cpu) {
    const FREE_HEAD: u32 = 0x198;
    let man = Addr(abi::arg(c, 0));
    c.cycles += 4;
    let head = man.off(FREE_HEAD).lw(c);
    c.g[V0][0] = head;
    let next = Addr(head as u32).off(0x4).lw(c);
    c.g[V1][0] = next;
    man.off(FREE_HEAD).sw(c, next);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Sections and game state.

/// `void SectionMan_InitNodePool(SectionMan* self)` (0x22e228): link the 50
/// free-list nodes into a chain, then publish the free-list head.
pub fn section_man_init_node_pool(c: &mut Cpu) {
    c.cycles += 6;
    let this = r32(c, A0);
    Addr(this).off(0x198).sw(c, c.g[A0][0]);
    c.g[A2][0] = 0x31;
    c.g[V1][0] = sx(this.wrapping_add(8));
    c.g[V0][0] = c.g[A0][0];
    c.g[A1][0] = 0x31;
    loop {
        c.cycles += 7;
        Addr(r32(c, V0)).off(4).sw(c, c.g[V1][0]);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        Addr(r32(c, V0)).sw(c, c.g[A2][0]);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(8));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(8));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0022_e240);
        }
    }
    c.cycles += 6;
    c.g[V0][0] = 0x31;
    Addr(this).off(0x1a0).sw(c, 0);
    Addr(this).off(0x190).sw(c, c.g[V0][0]);
    Addr(this).off(0x194).sw(c, 0);
    Addr(this).off(0x19c).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// `SectionMan_QueueSection` (0x22e298): remove every node of `id` from the
// live list (dequeuing each) and append a fresh node for it at the end.
pub fn section_man_queue_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x19c));
        let empty = c.g[S0][0] == 0;
        c.g[S3][0] = 0u64;
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
            loop {
                if c.g[V0][0] != c.g[S2][0] {
                    c.cycles += 2;
                    c.g[S3][0] = c.g[S0][0];
                    c.cycles += 1;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                } else {
                    c.cycles += 2;
                    c.cycles += 7;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = c.g[S3][0];
                    callee::SECTION_MAN_DEQUEUE_SECTION.call(c, 0x0022_e2e4)?;
                }
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                if event_due(c) {
                    deliver_event_here(c, 0x0022_e2c8);
                }
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        callee::SECTION_MAN_POP_FREE_NODE.call(c, 0x0022_e300)?;
        c.g[S0][0] = c.g[V0][0];
        c.sw((c.g[S0][0] as u32), c.g[S2][0]);
        let no_prev = c.g[S3][0] == 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        if !no_prev {
            c.cycles += 2;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[S0][0]);
        } else {
            c.cycles += 1;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x19c), c.g[S0][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1a0));
        c.lq(16, frame.at(c, 0x40));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.lq(18, frame.at(c, 0x20));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1a0), c.g[V0][0]);
        c.lq(17, frame.at(c, 0x30));
        c.lq(19, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::check_paths;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_2;
    pub(crate) use crate::game::bx::bxstring2::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn section_man_push_free_node_matches() {
        check(
            ssx3_ee::s05::f_0022e288__generated,
            section_man_push_free_node,
            300,
            &|c, seed| {
                let mut r = Rng(0x3456_789a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let head = if seed % 2 == 0 {
                    0
                } else {
                    (DATA + 0x200) as u64
                };
                c.bus.write32(DATA + SECTION_MAN_FREE_LIST, head as u32);
                fill(c, &mut r, DATA + 0x100, 8);
            },
        );
    }

    #[test]
    fn section_man_pop_free_node_matches() {
        check(
            ssx3_ee::s05::f_0022e278,
            section_man_pop_free_node,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c11_0005 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0x198, 0);
                } else {
                    let head = DATA + 0x100;
                    c.bus.write32(DATA + 0x198, head);
                    c.bus.write32(head + 0x4, r.next_u32());
                }
            },
        );
    }

    // ------------------------------------------------------------------
    // Sections and game state.

    #[test]
    fn section_man_init_node_pool_matches() {
        check(
            ee::s05::f_0022e228__generated,
            section_man_init_node_pool,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x22e2_28);
                let this = DATA + 0x1000;
                clear(c, this, 0x80);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn section_man_queue_section_matches() {
        check(
            ee::s05::f_0022e298__generated,
            section_man_queue_section,
            80,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x22e2);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                let n = (seed / 4 % 4) as u32;
                c.bus
                    .write32(DATA + 0x19c, if n == 0 { 0 } else { DATA + 0x1000 });
                for i in 0..3u32 {
                    let node = DATA + 0x1000 + 0x10 * i;
                    c.bus.write32(node, (i + 1) % 4);
                    c.bus
                        .write32(node + 4, if i + 1 < n { node + 0x10 } else { 0 });
                }
                c.bus.write32(DATA + 0x198, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, 0);
                c.bus.write32(DATA + 0x2000 + 4, 0);
                c.bus.write32(DATA + 0x1a0, 0);
                fill(c, &mut r, DATA + 0x2000, 0x20);
            },
        );
    }
}
