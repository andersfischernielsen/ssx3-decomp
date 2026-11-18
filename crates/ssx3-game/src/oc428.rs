//! Staging module oc428 (wave wave220): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc428`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0022_f6b0, game_module_load_world, "GameModule_LoadWorld"),
    (0x0027_c070, script_data_get_info, "ScriptData_GetInfo"),
    (0x0022_5248, c_luno_table_find, "cLunoTable_Find"),
    (
        0x0012_9e20,
        ai_create_computer_riders,
        "AI_CreateComputerRiders",
    ),
    (
        0x0030_0f50,
        w_script_fn_add_collect_items,
        "WScriptFn_AddCollectItems",
    ),
    (0x0027_6868, script_list_stop, "ScriptList_Stop"),
    (
        0x0027_a4a0,
        script_engine_get_transition_target,
        "ScriptEngine_GetTransitionTarget",
    ),
    (
        0x0028_3c58,
        script_cmd_queue_is_full,
        "ScriptCmdQueue_IsFull",
    ),
    (0x0028_2f60, script_binding_ctor, "ScriptBinding_ctor"),
    (0x0036_5e40, vram_heap_init, "VramHeap_Init"),
    (0x0027_c888, script_view_get_params, "ScriptView_GetParams"),
    (
        0x0027_c098,
        script_cam_get_manual_params,
        "ScriptCam_GetManualParams",
    ),
    (
        0x0023_32c8,
        c_game_load_screen_try_exit,
        "cGameLoadScreen_TryExit",
    ),
    (0x0028_14f8, script_view_obj_apply, "ScriptViewObj_Apply"),
    (
        0x002f_f9a8,
        w_script_fn_object_slot36,
        "WScriptFn_ObjectSlot36",
    ),
    (
        0x0027_b0c0,
        script_engine_choose_script,
        "ScriptEngine_ChooseScript",
    ),
    (
        0x0030_8b78,
        w_script_man_create_local_table,
        "WScriptMan_CreateLocalTable",
    ),
    (0x0012_98e0, ai_create_players, "AI_CreatePlayers"),
    (0x0036_29b8, defer_man_init, "DeferMan_Init"),
    (
        0x0030_8db8,
        w_script_man_run_mission_setup_scripts,
        "WScriptMan_RunMissionSetupScripts",
    ),
];

/// `base + off` as a 32-bit guest address.
#[inline(always)]
fn oc428_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `ScriptData::info` offset.
pub(crate) const SCRIPT_DATA_INFO: u32 = 0x14;

/// `ScriptData* ScriptData_GetInfo(ScriptData* self)` (0x27c070): the word at
/// `+0x14`.
pub fn script_data_get_info(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw(oc428_at(c.g[A0][0], SCRIPT_DATA_INFO));
    ret(c);
}

/// `ScriptCmdQueue` full-flag offset.
pub(crate) const SCRIPT_CMD_QUEUE_FULL: u32 = 0x94;

/// `int ScriptCmdQueue_IsFull(ScriptCmdQueue* self)` (0x283c58): the word at
/// `+0x94`.
pub fn script_cmd_queue_is_full(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw(oc428_at(c.g[A0][0], SCRIPT_CMD_QUEUE_FULL));
    ret(c);
}

/// `ScriptBinding* ScriptBinding_ctor(ScriptBinding* self)` (0x282f60): the
/// vtable `0x482418`, an id of -1 and a zero third word; returns `self`.
pub fn script_binding_ctor(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x48_0000;
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2418));
    c.sw(oc428_at(c.g[A0][0], 4), c.g[V1][0]);
    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    ret(c);
    c.sw(oc428_at(c.g[A0][0], 8), 0);
}

/// `ScriptView` parameter block offset.
pub(crate) const SCRIPT_VIEW_PARAMS: u32 = 0x20;

/// `void* ScriptView_GetParams(ScriptView* self)` (0x27c888): the word at
/// `+0x20`.
pub fn script_view_get_params(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw(oc428_at(c.g[A0][0], SCRIPT_VIEW_PARAMS));
    ret(c);
}

/// `ScriptCam` manual-camera parameter block offset.
pub(crate) const SCRIPT_CAM_MANUAL_PARAMS: u32 = 0x20;

/// `void* ScriptCam_GetManualParams(ScriptCam* self)` (0x27c098): the word at
/// `+0x20`.
pub fn script_cam_get_manual_params(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw(oc428_at(c.g[A0][0], SCRIPT_CAM_MANUAL_PARAMS));
    ret(c);
}

/// `AppModule_TryExitWhenLoaded`.
const OC428_APP_MODULE_TRY_EXIT_WHEN_LOADED: Callee = Callee {
    addr: 0x0023_1c70,
    f: ee::s06::AppModule_TryExitWhenLoaded,
};

/// `bool cGameLoadScreen_TryExit(cGameLoadScreen* self)` (0x2332c8):
/// `AppModule_TryExitWhenLoaded(self) != 0`.
pub fn c_game_load_screen_try_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        OC428_APP_MODULE_TRY_EXIT_WHEN_LOADED.call(c, 0x0023_32d8)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `cLunoTable` fields: modulus, node array; node: key word (+0), key type
/// (+8), value (+0xc, its type at +0x14), next (+0x18); a node is 0x20 bytes.
pub(crate) const LUNO_TABLE_MODULUS: u32 = 0xc;
pub(crate) const LUNO_TABLE_NODES: u32 = 0x10;
pub(crate) const LUNO_NODE_KEY_TYPE: u32 = 0x8;
pub(crate) const LUNO_NODE_VALUE: u32 = 0xc;
pub(crate) const LUNO_NODE_VALUE_TYPE: u32 = 0x14;
pub(crate) const LUNO_NODE_NEXT: u32 = 0x18;

/// `cLunoValue* cLunoTable_Find(cLunoVM* self, cLunoKey* key)` (0x225248):
/// hash lookup. The bucket is `nodes[key.word0 % modulus]`; if its value is
/// non-nil, walk the chain for a node whose key matches (same type and
/// word; an int key matches a float node by value and a float key an int
/// node after truncation). Returns the node's value slot or `luno_gLunoNil`.
pub fn c_luno_table_find(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A2][0] = c.lw(c.g[A0][0] as u32);
    c.g[A0][0] = c.lw(c.g[A1][0] as u32);
    c.g[V0][0] = c.lw(oc428_at(c.g[A2][0], LUNO_TABLE_MODULUS));
    c.divu(0, A0, V0);
    'nil: {
        c.cycles += 7;
        c.g[A0][0] = c.lw(oc428_at(c.g[A2][0], LUNO_TABLE_NODES));
        c.g[V1][0] = c.hi[0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = c.lw(oc428_at(c.g[A0][0], LUNO_NODE_VALUE_TYPE));
        let empty = c.g[V0][0] == 0;
        c.g[T0][0] = 2;
        if empty {
            break 'nil;
        }
        c.cycles += 5;
        c.g[A2][0] = c.lw(oc428_at(c.g[A1][0], LUNO_NODE_KEY_TYPE));
        c.g[A3][0] = 1;
        c.g[T1][0] = 3;
        c.g[V0][0] = c.lw(oc428_at(c.g[A0][0], LUNO_NODE_KEY_TYPE));
        'walk: loop {
            // `v0` = this node's key type, `a2` the key's, `a3` = 1, `t0` = 2,
            // `t1` = 3. The comparison leaves a result in `v0`: 1 = match.
            'matched: {
                'xor_words: {
                    'same_type: {
                        c.cycles += 2;
                        if c.g[A2][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                            break 'same_type;
                        }
                        'int_key_float_node: {
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[T0][0] {
                                break 'int_key_float_node;
                            }
                            c.cycles += 2;
                            if c.g[A2][0] != c.g[A3][0] {
                                break 'int_key_float_node;
                            }
                            // Float node, int key: compare as floats.
                            c.cycles += 7;
                            c.lwc1(1, c.g[A1][0] as u32);
                            c.cvt_s_w(1, 1);
                            c.lwc1(0, c.g[A0][0] as u32);
                            c.c_eq_s(0, 1);
                            let eq = c.fcc();
                            c.g[V0][0] = 1;
                            if eq {
                                break 'matched;
                            }
                            c.cycles += 2;
                            c.g[V0][0] = 0;
                            break 'matched;
                        }
                        'float_key_int_node: {
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[A3][0] {
                                break 'float_key_int_node;
                            }
                            c.cycles += 2;
                            if c.g[A2][0] != c.g[T0][0] {
                                break 'float_key_int_node;
                            }
                            // Int node, float key: truncate the key.
                            c.cycles += 5;
                            c.lwc1(0, c.g[A1][0] as u32);
                            c.cvt_w_s(1, 0);
                            c.mfc1(V1, 1);
                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                            break 'xor_words;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[T1][0] {
                            c.g[V0][0] = 0;
                            break 'matched;
                        }
                        c.cycles += 2;
                        if c.g[A2][0] != c.g[A3][0] {
                            c.g[V0][0] = 0;
                            break 'matched;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    }
                    // Same key type: compare the words.
                    c.cycles += 1;
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                }
                c.cycles += 2;
                c.g[V0][0] ^= c.g[V1][0];
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = sx(oc428_at(c.g[A0][0], LUNO_NODE_VALUE));
                ret(c);
                return;
            }
            c.g[A0][0] = c.lw(oc428_at(c.g[A0][0], LUNO_NODE_NEXT));
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                break 'nil;
            }
            c.g[V0][0] = c.lw(oc428_at(c.g[A0][0], LUNO_NODE_KEY_TYPE));
            if event_due(c) {
                deliver_event_here(c, 0x0022_5290);
            }
            continue 'walk;
        }
    }
    c.cycles += 3;
    c.g[V0][0] = 0x4d_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6f68));
    ret(c);
}

/// `void VramHeap_Init(VramHeap* self, int base, int size, void* pool)`
/// (0x365e40): set up the GS video-memory heap. Fills the first of 292 block
/// links with (base, size), chains the rest into a free list (`links[i].next
/// = i + 1`, their in-use bit cleared), sets the 17 + 17 size-class slots to
/// -1, resets the list heads and the pool pointer, then links the first block
/// into the free lists with `VramHeap_LinkFreeBlock(self, 0)`.
#[allow(unreachable_code)]
pub fn vram_heap_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[T0][0] = 0x1;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[T1][0] = 0xfffffffffffffffe;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = 0xffffffffffffffff;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S2][0] = c.g[A3][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1ff4), c.g[A2][0]);
        c.g[A3][0] = 0x1c;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ff0));
        c.sw(c.g[V0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ff0));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ff0));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[A0][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ff0));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ff0));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        'l_365e98: loop {
            c.cycles += 13;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ff0));
            c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            c.g[T0][0] = c.g[A1][0];
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A2][0] = ((c.g[T0][0] as i64) < 291i64) as u64;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ff0));
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            let t = c.g[A2][0] != 0u64;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1c));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00365e98);
                }
                continue 'l_365e98;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ff0));
                c.g[V0][0] = 0xffffffffffffffff;
                c.g[A3][0] = 0xffffffffffffffff;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x21e8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1fe4), c.g[V0][0]);
                c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x21ec));
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x200c));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2010));
                c.g[T0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x23c4));
                'l_365ef0: loop {
                    c.cycles += 10;
                    c.sw(c.g[V1][0] as u32, c.g[A3][0]);
                    c.sw(c.g[A1][0] as u32, c.g[A3][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.sw(c.g[A2][0] as u32, c.g[A3][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c));
                    c.sw(c.g[A0][0] as u32, c.g[A3][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1c));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00365ef0);
                        }
                        continue 'l_365ef0;
                    } else {
                        c.cycles += 20;
                        c.g[S0][0] = 0xffffffffffffffff;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x23c0), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x23bc), c.g[S0][0]);
                        c.g[A0][0] = c.g[S1][0];
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x23c4), c.g[S0][0]);
                        c.g[A1][0] = 0u64;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x23c8), c.g[S0][0]);
                        Callee {
                            addr: 0x003660a8,
                            f: ee::s12::VramHeap_LinkFreeBlock,
                        }
                        .call(c, 0x00365f38)?;
                        c.g[V0][0] = 0x1;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x23e8), c.g[S2][0]);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x23e4), c.g[S0][0]);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x23d8), c.g[S0][0]);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x23e0), c.g[S0][0]);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x23dc), c.g[V0][0]);
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    })
}

/// `void DeferMan_Init(DeferMan* self)` (0x3629b8): reset the deferred-packet
/// manager (`DeferMan_ResetFrame`), then copy the template packet at
/// `0x501420` into the GS register context at `gp[0x2a90] + 0xe84` and
/// patch its bit fields (mode, texture, alpha, depth and test bits) three
/// times, saving the context after each variant (+0x7488, +0x748c, +0x7490
/// of the manager's buffer) and counting them at `+0x1480`.
#[allow(unreachable_code)]
pub fn defer_man_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 195;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S0][0] = 0x50000;
        c.g[S1][0] = c.g[A0][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x7484), 0u64);
        Callee {
            addr: 0x00362cc8,
            f: ee::s12::DeferMan_ResetFrame,
        }
        .call(c, 0x003629f0)?;
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
        c.g[V1][0] = 0x500000;
        c.g[S3][0] = 0xfffffffffffffffc;
        c.g[T2][0] = 0xffffffffe0000000;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[T2][0] = c.g[T2][0] | 0x3ff;
        c.g[T3][0] = 0xfffffffffe7f0000;
        c.g[S4][0] = 0x800000;
        c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1420));
        c.ldl(4, (c.g[T0][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[T0][0] as u32);
        c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
        c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
        c.g[A3][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
        c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[V0][0] as u32);
        c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
        c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
        c.g[T3][0] = c.g[T3][0] | 0xffff;
        c.g[T4][0] = 0xffffffffffcf0000;
        c.g[T5][0] = 0xfffffffffff00000;
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[T4][0] = c.g[T4][0] | 0xffff;
        c.g[T5][0] = c.g[T5][0] | 0xfff;
        c.g[A1][0] = 0x10000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = c.g[A1][0] | 0x4000;
        c.g[T7][0] = 0xfffffffffffffc3f;
        c.g[S2][0] = 0xffffffffffffff83;
        c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
        c.g[T1][0] = 0xfffffffffffff07f;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[T0][0] = 0x18;
        c.g[A3][0] = 0x50000;
        c.g[S5][0] = 0x300000;
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[A3][0] = c.g[A3][0] | 0x1484;
        c.g[T6][0] = 0x50000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[T6][0] = c.g[T6][0] | 0xc000;
        c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
        c.g[V0][0] = c.g[V0][0] | c.g[S4][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = c.g[V0][0] | 0x30;
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        c.g[V0][0] = c.g[V0][0] & c.g[T7][0];
        c.g[V0][0] = c.g[V0][0] | 0xc0;
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
        c.g[V0][0] = c.g[V0][0] | 0x4;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
        c.g[V0][0] = c.g[V0][0] | 0x880;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1480));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.mult(0, 2, 3, 8);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1480), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.ldl(15, (c.g[A0][0] as u32).wrapping_add(0x7));
        c.ldr(15, c.g[A0][0] as u32);
        c.ldl(22, (c.g[A0][0] as u32).wrapping_add(0xf));
        c.ldr(22, (c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
        c.sdl(15, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.sdr(15, c.g[V0][0] as u32);
        c.sdl(22, (c.g[V0][0] as u32).wrapping_add(0xf));
        c.sdr(22, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x7488), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
        c.g[V0][0] = c.g[V0][0] | 0x100;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1480));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.mult(0, 5, 3, 8);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1480), c.g[V1][0]);
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.ldl(15, (c.g[A0][0] as u32).wrapping_add(0x7));
        c.ldr(15, c.g[A0][0] as u32);
        c.ldl(22, (c.g[A0][0] as u32).wrapping_add(0xf));
        c.ldr(22, (c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
        c.sdl(15, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.sdr(15, c.g[V0][0] as u32);
        c.sdl(22, (c.g[V0][0] as u32).wrapping_add(0xf));
        c.sdr(22, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x748c), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
        c.g[V0][0] = c.g[V0][0] | 0x14;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
        c.g[V0][0] = c.g[V0][0] | 0x180;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
        c.g[V0][0] = c.g[V0][0] | 0x1;
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
        c.g[V0][0] = c.g[V0][0] | c.g[S4][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
        c.g[V0][0] = c.g[V0][0] | c.g[S5][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.g[V0][0] | c.g[T6][0];
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1480));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
        c.mult(0, 4, 3, 8);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1480), c.g[V1][0]);
        c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.ldr(5, c.g[V0][0] as u32);
        c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
        c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
        c.sdl(5, (c.g[S1][0] as u32).wrapping_add(0x7));
        c.sdr(5, c.g[S1][0] as u32);
        c.sdl(6, (c.g[S1][0] as u32).wrapping_add(0xf));
        c.sdr(6, (c.g[S1][0] as u32).wrapping_add(0x8));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1480));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x7490), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x7484), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        Ok(())
    })
}

/// `void ScriptList_Stop(ScriptList* self, int resetTransition)` (0x276868):
/// stop the running script list: unpause it (`ScriptList_SetPaused`), and
/// for a list in state 3 with a cut-scene transition restore the regular
/// mode (`cCutSceneTransitionMan_ForceOff`, stop the transition manager) and
/// re-show the `hud` UI state (virtual slot 0x18); then pop all entries and
/// clear the list's state words (`+0xa4`..`+0xc8`).
#[allow(unreachable_code)]
pub fn script_list_stop(c: &mut Cpu) {
    guest(c, |c| {
        'b_276894: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa8));
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = c.g[A1][0];
            if t {
                break 'b_276894;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0x1;
                Callee {
                    addr: 0x00277c08,
                    f: ee::s07::ScriptList_SetPaused,
                }
                .call(c, 0x00276894)?;
                break 'b_276894;
            }
        }
        'b_2768b4: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc4));
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_2768b4;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                Callee {
                    addr: 0x00277c08,
                    f: ee::s07::ScriptList_SetPaused,
                }
                .call(c, 0x002768b4)?;
                break 'b_2768b4;
            }
        }
        'b_276954: {
            'b_276950: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x3;
                if t {
                    break 'b_276950;
                } else {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        break 'b_276954;
                    } else {
                        'b_276904: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            let t = c.g[S0][0] == 0u64;
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                            if t {
                                break 'b_276904;
                            } else {
                                c.cycles += 5;
                                call_ptr(c, 0x002ea820, 0x002768e4)?;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb0));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_276904;
                                } else {
                                    c.cycles += 5;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                    call_ptr(c, 0x00230698, 0x002768fc)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x002e4578, 0x00276904)?;
                                    break 'b_276904;
                                }
                            }
                        }
                        c.cycles += 13;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = 0x4a0000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x34a0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x00317670, 0x00276920)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039f9d8, 0x0027692c)?;
                        c.g[A3][0] = c.g[V0][0];
                        let t = c.g[A3][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_276950;
                        } else {
                            c.cycles += 6;
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00276950)?;
                            break 'b_276950;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
            break 'b_276954;
        }
        c.cycles += 17;
        c.g[A1][0] = 0x5;
        Callee {
            addr: 0x00277598,
            f: ee::s07::ScriptList_PopEntries,
        }
        .call(c, 0x0027695c)?;
        c.g[V0][0] = 0xffffffffffffffff;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xb4), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xa4), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc8), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xb0), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xb8), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xac), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc0), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc4), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void ScriptViewObj_Apply(ScriptViewObj* self, ScriptView* view, int
/// flags, int currFrame)` (0x2814f8): apply a script view node. Flags bit 2/3
/// (0xc): read the view's fields. Bit 2: cut to the shot (`CutToShot`),
/// unless a type-8 node makes it wait (`+0xec` set, if `+0xf0` is clear and
/// the view has parameters). Bit 3: with no shot, stop the fade and unbind
/// both camera slots; otherwise do the pending cut and unbind the slots whose
/// bit is not in the shots' mask.
#[allow(unreachable_code)]
pub fn script_view_obj_apply(c: &mut Cpu) {
    guest(c, |c| {
        'b_281688: {
            'b_281684: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A3][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A1][0];
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[V0][0] = c.g[S2][0] & 0xc;
                let t = c.g[V0][0] == 0u64;
                c.mov_s(20, 12);
                if t {
                    break 'b_281684;
                } else {
                    'b_2815a0: {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[SP][0];
                        Callee {
                            addr: 0x0027c8e0,
                            f: ee::s07::ScriptView_ReadFields,
                        }
                        .call(c, 0x00281534)?;
                        c.g[V0][0] = c.g[S2][0] & 0x4;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.g[S2][0] & 0x8;
                        if t {
                            break 'b_2815a0;
                        } else {
                            'b_281588: {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x0027c888,
                                    f: ee::s07::ScriptView_GetParams,
                                }
                                .call(c, 0x00281548)?;
                                c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                                let t = c.g[V1][0] == 0u64;
                                c.g[S0][0] = 0x1;
                                if t {
                                    break 'b_281588;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    if t {
                                        break 'b_281588;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf0));
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A0][0] = 0x8;
                                        if t {
                                            break 'b_281588;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                            c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                                            let t = c.g[V1][0] != c.g[A0][0];

                                            if t {
                                                break 'b_281588;
                                            } else {
                                                c.cycles += 2;
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0xec),
                                                    c.g[S0][0],
                                                );
                                                c.g[S0][0] = 0u64;
                                                break 'b_281588;
                                            }
                                        }
                                    }
                                }
                            }
                            'b_28159c: {
                                c.cycles += 2;
                                let t = c.g[S0][0] == 0u64;
                                c.g[A0][0] = c.g[S1][0];
                                if t {
                                    break 'b_28159c;
                                } else {
                                    c.cycles += 3;
                                    c.g[A1][0] = c.g[SP][0];
                                    c.mov_s(12, 20);
                                    Callee {
                                        addr: 0x00281c68,
                                        f: ee::s07::ScriptViewObj_CutToShot,
                                    }
                                    .call(c, 0x0028159c)?;
                                    break 'b_28159c;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.g[S2][0] & 0x8;
                            break 'b_2815a0;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        break 'b_281684;
                    } else {
                        'b_281604: {
                            c.cycles += 2;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xec));
                                break 'b_281604;
                            } else {
                                'b_2815d4: {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd8));
                                    let t = c.g[A0][0] == 0u64;
                                    c.g[A1][0] = 0xffffffffffffffff;
                                    if t {
                                        break 'b_2815d4;
                                    } else {
                                        c.cycles += 6;
                                        c.mtc1(0, 12);
                                        c.g[A2][0] = 0x9;
                                        c.mov_s(13, 12);
                                        c.mov_s(14, 12);
                                        c.mov_s(15, 12);
                                        Callee {
                                            addr: 0x00282db0,
                                            f: ee::s07::cTransitionMan_StopTransition,
                                        }
                                        .call(c, 0x002815d4)?;
                                        break 'b_2815d4;
                                    }
                                }
                                c.cycles += 3;
                                c.g[S0][0] = 0u64;
                                c.g[A1][0] = c.g[S0][0];
                                'l_2815e0: loop {
                                    c.cycles += 7;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A2][0] = 0x9;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    Callee {
                                        addr: 0x00281ac8,
                                        f: ee::s07::ScriptViewObj_BindCamSlot,
                                    }
                                    .call(c, 0x002815f0)?;
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A1][0] = c.g[S0][0];
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002815e0);
                                        }
                                        continue 'l_2815e0;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        if t {
                                            break 'b_281688;
                                        } else {
                                            break 'b_281604;
                                        }
                                    }
                                }
                            }
                        }
                        'b_28161c: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.mov_s(12, 20);
                            if t {
                                break 'b_28161c;
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[SP][0];
                                Callee {
                                    addr: 0x00281c68,
                                    f: ee::s07::ScriptViewObj_CutToShot,
                                }
                                .call(c, 0x00281618)?;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0xec), 0u64);
                                break 'b_28161c;
                            }
                        }
                        'b_28164c: {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            let t = c.g[V1][0] == 0u64;
                            c.g[S2][0] = 0u64;
                            if t {
                                break 'b_28164c;
                            } else {
                                c.cycles += 2;
                                c.g[A0][0] = 0x1;
                                'l_281630: loop {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] =
                                        sx((c.g[A0][0] as u32) << ((c.g[V0][0] as u32) & 31));
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[S2][0] = c.g[S2][0] | c.g[V0][0];
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00281630);
                                        }
                                        continue 'l_281630;
                                    } else {
                                        break 'b_28164c;
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[S0][0] = 0u64;
                        c.g[V0][0] =
                            sx(((c.g[S2][0] as u32) as i32 >> ((c.g[S0][0] as u32) & 31)) as u32);
                        'l_281658: loop {
                            'b_281678: {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                if c.g[V0][0] != 0u64 {
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    break 'b_281678;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A2][0] = 0x9;
                                    Callee {
                                        addr: 0x00281ac8,
                                        f: ee::s07::ScriptViewObj_BindCamSlot,
                                    }
                                    .call(c, 0x00281674)?;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    break 'b_281678;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32
                                >> ((c.g[S0][0] as u32) & 31))
                                as u32);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00281658);
                                }
                                continue 'l_281658;
                            } else {
                                break 'b_281684;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_281688;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    })
}

/// `ScriptValue* WScriptFn_ObjectSlot36(ScriptValue* out, int n, Entry* e)`
/// (0x2ff9a8): script call "object slot 36". Builds an argument block of four
/// words (initialised to -1 on first use from the `0x4fbaf8` table, copied
/// with `EA_MemCopy`), fills `args[e.idx]` from the n entries (an int entry
/// whose slot type is 2 becomes a float), looks the object up by `args[0]`
/// (handle: low byte = slot, high = generation; through `gp[0x16c8]`) and
/// calls its method at vtable `+0x120` with `(args[1], args[2])`. Returns
/// `luno_gLunoNil` (copied into `out`).
#[allow(unreachable_code)]
pub fn w_script_fn_object_slot36(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ffa08: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2550));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = 0x500000;
            if t {
                break 'b_2ffa08;
            } else {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4508));
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.sw((c.g[A2][0] as u32).wrapping_sub(0x4508), c.g[V0][0]);
                c.g[A0][0] = 0xffffffffffffffff;
                c.sb((c.g[V1][0] as u32).wrapping_add(0xc), c.g[A1][0]);
                c.g[V0][0] = 0x1;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2550), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.sb(c.g[SP][0] as u32, c.g[A1][0]);
                break 'b_2ffa08;
            }
        }
        'b_2ffa88: {
            c.cycles += 10;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4508));
            c.g[A2][0] = 0x10;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e6574, 0x002ffa28)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if t {
                break 'b_2ffa88;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6540));
                c.g[A3][0] = 0x2;
                'l_2ffa40: loop {
                    'b_2ffa7c: {
                        'b_2ffa78: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_2ffa78;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_2ffa78;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, c.g[A0][0] as u32);
                                    if t {
                                        break 'b_2ffa7c;
                                    } else {
                                        break 'b_2ffa78;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        break 'b_2ffa7c;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002ffa40);
                        }
                        continue 'l_2ffa40;
                    } else {
                        break 'b_2ffa88;
                    }
                }
            }
        }
        'b_2ffae4: {
            'b_2ffaa4: {
                c.cycles += 4;
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                if t {
                    break 'b_2ffaa4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                    if t {
                        break 'b_2ffae4;
                    } else {
                        break 'b_2ffaa4;
                    }
                }
            }
            'b_2ffae0: {
                c.cycles += 8;
                c.g[V0][0] = c.g[A2][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                if t {
                    break 'b_2ffae0;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        break 'b_2ffae4;
                    } else {
                        break 'b_2ffae0;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_2ffae4;
        }
        'b_2ffb10: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let t = c.g[A2][0] == 0u64;
            c.g[V1][0] = 0x4d0000;
            if t {
                break 'b_2ffb10;
            } else {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x120));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002ffb0c)?;
                c.g[V1][0] = 0x4d0000;
                break 'b_2ffb10;
            }
        }
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S3][0] as u32);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `cLunoTable* WScriptMan_CreateLocalTable(WScriptMan* self, int i)`
/// (0x308b78): replace the i-th local Luno table (`self + 0x3cc + 4*i`): the
/// old one, if any, is destroyed (`cLunoTable_dtor(t, 3)`), then a new one
/// of 0x40 buckets is allocated (named allocation, 4 bytes) and constructed
/// (`cLunoTable_ctor(p, 0x40)`). Returns and stores the new table.
#[allow(unreachable_code)]
pub fn w_script_man_create_local_table(c: &mut Cpu) {
    guest(c, |c| {
        'b_308ba8: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3cc));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            if c.g[A0][0] == 0u64 {
                c.g[A1][0] = 0x490000;
                break 'b_308ba8;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0x3;
                call_ptr(c, 0x00224df0, 0x00308ba4)?;
                c.g[A1][0] = 0x490000;
                break 'b_308ba8;
            }
        }
        c.cycles += 13;
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0x4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6738));
        Callee {
            addr: 0x00317d70,
            f: ee::s10::EA_MemAllocNamed,
        }
        .call(c, 0x00308bbc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x40;
        call_ptr(c, 0x00224da0, 0x00308bc8)?;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// Copy 16 bytes from `src` to `dst` with two unaligned doubleword pairs
/// (`ldl/ldr` into `lo` and `hi`, then `sdl/sdr`).
#[inline(always)]
fn oc428_copy16(c: &mut Cpu, src: u32, dst: u32, lo: usize, hi: usize) {
    c.ldl(lo, src.wrapping_add(0x7));
    c.ldr(lo, src);
    c.ldl(hi, src.wrapping_add(0xf));
    c.ldr(hi, src.wrapping_add(0x8));
    c.sdl(lo, dst.wrapping_add(0x7));
    c.sdr(lo, dst);
    c.sdl(hi, dst.wrapping_add(0xf));
    c.sdr(hi, dst.wrapping_add(0x8));
}

/// `ScriptEngine_GetTransitionTarget` jump-table entries (the code
/// addresses the table at 0x481df0 holds for the transition types).
pub(crate) const TRANSITION_FADE: u32 = 0x0027_a51c;
pub(crate) const TRANSITION_WIPE_LEFT: u32 = 0x0027_a550;
pub(crate) const TRANSITION_WIPE_RIGHT: u32 = 0x0027_a568;
pub(crate) const TRANSITION_WIPE_UP: u32 = 0x0027_a58c;
pub(crate) const TRANSITION_WIPE_DOWN: u32 = 0x0027_a5b0;
pub(crate) const TRANSITION_WIPE_RANDOM: u32 = 0x0027_a5d4;
pub(crate) const TRANSITION_WHITE_FADE: u32 = 0x0027_a5f0;
pub(crate) const TRANSITION_NONE: u32 = 0x0027_a654;

/// `TransitionEffect* ScriptEngine_GetTransitionTarget(ScriptEngine*, int
/// type, int mode)` (0x27a4a0): build a transition effect object for the
/// script's transition `type` (0 and 8: none, 1: fade, 2-5: wipes in four
/// directions, 6: random wipe, 7: white fade). The colour is red
/// (1,0,0,0) for `mode` 0, white (1,1,1,1) for `mode` 1, and the stack's
/// previous contents otherwise. The type is dispatched through the jump
/// table at 0x481df0 (read from RAM like the original: an unknown target
/// leaves the function through `jump_out`).
pub fn script_engine_get_transition_target(c: &mut Cpu) {
    if c.pc != 0x0027_a4a0 {
        return ee::s07::f_0027a4a0__generated(c);
    }
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        let mode_zero = c.g[A2][0] == 0;
        frame.save_ra(c, 0x20);
        // The colour lives at sp .. sp + 0x10; the effect gets a copy at sp + 0x10.
        let color = frame.at(c, 0);
        if mode_zero {
            // Red: (1, 0, 0, 0).
            c.cycles += 7;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.sw(color + 4, 0);
            c.swc1(0, color);
            c.sw(color + 8, 0);
            c.sw(color + 0xc, 0);
            c.cycles += 1;
            c.g[V0][0] = (c.g[A1][0] < 9) as u64;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let white = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = (c.g[A1][0] < 9) as u64;
            if white {
                // White: (1, 1, 1, 1).
                c.cycles += 6;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.swc1(0, color + 0xc);
                c.swc1(0, color);
                c.swc1(0, color + 4);
                c.swc1(0, color + 8);
                c.cycles += 1;
                c.g[V0][0] = (c.g[A1][0] < 9) as u64;
            } else {
                c.cycles += 2;
            }
        }
        // `done` (0x27a65c): the result is in v0 and s0 restored. `tail`
        // (0x27a658) adds one cycle and restores s0; `none` returns 0.
        'done: {
            'tail: {
                'none: {
                    c.cycles += 2;
                    let known = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x48_0000;
                    if !known {
                        break 'none;
                    }
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1df0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let target = c.g[A0][0] as u32;
                    match target {
                        TRANSITION_FADE | TRANSITION_WHITE_FADE => {
                            if target == TRANSITION_FADE {
                                c.cycles += 13;
                                c.g[A0][0] = 0x14;
                                call_ptr(c, 0x002e_4ce8, 0x0027_a524)?;
                                c.g[S0][0] = c.g[V0][0];
                                oc428_copy16(c, color, color + 0x10, V0, V1);
                                c.g[A0][0] = c.g[S0][0];
                            } else {
                                c.cycles += 10;
                                c.g[A0][0] = 0x14;
                                call_ptr(c, 0x002e_4ce8, 0x0027_a5f8)?;
                                c.g[AT][0] = 0x3f80_0000;
                                c.mtc1(1, 0);
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S0][0];
                                c.swc1(0, color + 0x1c);
                                c.swc1(0, color + 0x10);
                                c.swc1(0, color + 0x14);
                                c.swc1(0, color + 0x18);
                            }
                            // The fade effect: constructor, vtable, colour.
                            c.cycles += 15;
                            call_ptr(c, 0x002e_4d70, 0x0027_a620)?;
                            c.g[V1][0] = 0x49_0000;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7f40));
                            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                            let s0 = c.g[S0][0] as u32;
                            oc428_copy16(c, color + 0x10, s0 + 4, 6, V1);
                            c.g[V0][0] = c.g[S0][0];
                            break 'tail;
                        }
                        TRANSITION_WIPE_LEFT | TRANSITION_WIPE_RIGHT => {
                            let (alloc_ret, dir) = if target == TRANSITION_WIPE_LEFT {
                                c.cycles += 6;
                                (0x0027_a558, u64::MAX)
                            } else {
                                c.cycles += 5;
                                (0x0027_a570, 1)
                            };
                            c.g[A0][0] = 0x1c;
                            call_ptr(c, 0x002e_4ce8, alloc_ret)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[SP][0];
                            c.g[A2][0] = dir;
                            // Horizontal wipe: (dir, 0).
                            c.cycles += 4;
                            c.g[A3][0] = 0;
                            call_ptr(c, 0x002e_bba8, 0x0027_a584)?;
                            frame.restore(c, S0, 0x30);
                            break 'done;
                        }
                        TRANSITION_WIPE_UP | TRANSITION_WIPE_DOWN => {
                            let (alloc_ret, wipe_ret, dir) = if target == TRANSITION_WIPE_UP {
                                (0x0027_a594, 0x0027_a5a8, u64::MAX)
                            } else {
                                (0x0027_a5b8, 0x0027_a5cc, 1)
                            };
                            // Vertical wipe: (0, dir).
                            c.cycles += 9;
                            c.g[A0][0] = 0x1c;
                            call_ptr(c, 0x002e_4ce8, alloc_ret)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[SP][0];
                            c.g[A2][0] = 0;
                            c.g[A3][0] = dir;
                            call_ptr(c, 0x002e_bba8, wipe_ret)?;
                            frame.restore(c, S0, 0x30);
                            break 'done;
                        }
                        TRANSITION_WIPE_RANDOM => {
                            c.cycles += 7;
                            c.g[A0][0] = 0x1c;
                            call_ptr(c, 0x002e_4ce8, 0x0027_a5dc)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[SP][0];
                            call_ptr(c, 0x002e_be20, 0x0027_a5e8)?;
                            frame.restore(c, S0, 0x30);
                            break 'done;
                        }
                        TRANSITION_NONE => break 'none,
                        _ => {
                            c.jump_out(target, 0x0027_a4a0, 0x0027_a668);
                            return Ok(());
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptValue* WScriptFn_AddCollectItems(ScriptValue* out, int n, Entry*
/// e)` (0x300f50): script call "add collect items". Builds a 22-word
/// argument block initialised to -1 (first call: the template at 0x4fbb78,
/// flag `gp[0x25c0]`; copied with `EA_MemCopy`), fills `args[e.idx]` from the
/// n entries (an int entry whose slot type, table 0x446570, is 2 becomes a
/// float), and, when `args[0]` is set, calls `WScriptMan_AddCollectItems(
/// gWScriptMan, args[0], args + 1, 20)`. Returns `luno_gLunoNil` in `out`.
#[allow(unreachable_code)]
pub fn w_script_fn_add_collect_items(c: &mut Cpu) {
    guest(c, |c| {
        'b_300ff4: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x25c0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S2][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S3][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = 0x500000;
            if t {
                break 'b_300ff4;
            } else {
                c.cycles += 29;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4488));
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x54));
                c.sw((c.g[A2][0] as u32).wrapping_sub(0x4488), c.g[V0][0]);
                c.g[A0][0] = 0x1;
                c.sb((c.g[V1][0] as u32).wrapping_add(0x54), c.g[A1][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x25c0), c.g[A0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                c.sb(c.g[SP][0] as u32, c.g[A1][0]);
                break 'b_300ff4;
            }
        }
        'b_3010c0: {
            c.cycles += 29;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4488));
            c.g[A2][0] = 0x58;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x60), c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x50), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x54), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
            call_ptr(c, 0x003e6574, 0x00301060)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if t {
                break 'b_3010c0;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6570));
                c.g[A3][0] = 0x2;
                'l_301078: loop {
                    'b_3010b4: {
                        'b_3010b0: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_3010b0;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_3010b0;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, c.g[A0][0] as u32);
                                    if t {
                                        break 'b_3010b4;
                                    } else {
                                        break 'b_3010b0;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        break 'b_3010b4;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00301078);
                        }
                        continue 'l_301078;
                    } else {
                        break 'b_3010c0;
                    }
                }
            }
        }
        'b_3010e0: {
            c.cycles += 5;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            let t = c.g[A1][0] == c.g[V0][0];
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            if t {
                break 'b_3010e0;
            } else {
                c.cycles += 3;
                c.g[A2][0] = c.g[S0][0] | 0x4;
                c.g[A3][0] = 0x14;
                Callee {
                    addr: 0x0030bac8,
                    f: ee::s10::WScriptMan_AddCollectItems,
                }
                .call(c, 0x003010e0)?;
                break 'b_3010e0;
            }
        }
        c.cycles += 16;
        c.g[V1][0] = 0x4d0000;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S3][0] as u32);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        Ok(())
    })
}

/// `void WScriptMan_RunMissionSetupScripts(WScriptMan* self)` (0x308db8):
/// for every world section, and every mission definition (0x54 bytes) of that
/// section whose id differs from `self[0x2a0]` (the current mission): make a
/// local Luno table (named allocation, 8 buckets), look the mission's setup
/// script up by its name (`def[0x4c]`), and if found run it through a fresh
/// `cLunoVM` with the table, destroying the result and then the table.
/// Finally clears the section/mission fields `+0x290/0x294/0x298/0x2a4/0x2a8`.
#[allow(unreachable_code)]
pub fn w_script_man_run_mission_setup_scripts(c: &mut Cpu) {
    guest(c, |c| {
        'b_308ef8: {
            c.cycles += 20;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S0][0] = 0u64;
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28c));
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x003a9800, 0x00308dfc)?;
            c.g[S7][0] = c.g[V0][0];
            if (c.g[S7][0] as i64) <= 0 {
                c.sw((c.g[S2][0] as u32).wrapping_add(0x2a8), 0u64);
                break 'b_308ef8;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28c));
                'l_308e10: loop {
                    'b_308ee8: {
                        c.cycles += 9;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        call_ptr(c, 0x003a6c68, 0x00308e1c)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28c));
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x003a6c98, 0x00308e2c)?;
                        if (c.g[S1][0] as i64) <= 0 {
                            c.g[S0][0] = c.g[S6][0];
                            break 'b_308ee8;
                        } else {
                            c.cycles += 5;
                            c.g[S3][0] = c.g[V0][0];
                            c.g[FP][0] = 0x490000;
                            c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                            'l_308e48: loop {
                                'b_308ed8: {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x2a0));
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[A2][0] = 0x20000000;
                                    if t {
                                        break 'b_308ed8;
                                    } else {
                                        'b_308ec8: {
                                            c.cycles += 13;
                                            c.g[A3][0] = 0u64;
                                            c.g[A0][0] = 0x4;
                                            c.g[A1][0] =
                                                sx((c.g[FP][0] as u32).wrapping_sub(0x6728));
                                            Callee {
                                                addr: 0x00317d70,
                                                f: ee::s10::EA_MemAllocNamed,
                                            }
                                            .call(c, 0x00308e68)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A1][0] = 0x8;
                                            call_ptr(c, 0x00224da0, 0x00308e74)?;
                                            c.g[A1][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x4c));
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S2][0];
                                            Callee {
                                                addr: 0x00309b70,
                                                f: ee::s10::WScriptMan_LookupFunc,
                                            }
                                            .call(c, 0x00308e84)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A0][0] = c.g[SP][0];
                                            if t {
                                                break 'b_308ec8;
                                            } else {
                                                c.cycles += 15;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x00226600, 0x00308e98)?;
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                                                c.g[A0][0] = c.g[S5][0];
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                                                call_ptr(c, 0x002224a8, 0x00308ea8)?;
                                                c.g[A0][0] = c.g[S5][0];
                                                c.g[A1][0] = c.g[SP][0];
                                                c.g[A2][0] = c.g[S0][0];
                                                c.g[A3][0] = c.g[S4][0];
                                                call_ptr(c, 0x002224b8, 0x00308ebc)?;
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[A1][0] = 0x2;
                                                call_ptr(c, 0x00225b90, 0x00308ec8)?;
                                                break 'b_308ec8;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[S0][0] == 0u64;
                                        c.g[A0][0] = c.g[S0][0];
                                        if t {
                                            break 'b_308ed8;
                                        } else {
                                            c.cycles += 2;
                                            c.g[A1][0] = 0x3;
                                            call_ptr(c, 0x00224df0, 0x00308ed8)?;
                                            break 'b_308ed8;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                let t = c.g[S1][0] != 0u64;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x54));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00308e48);
                                    }
                                    continue 'l_308e48;
                                } else {
                                    c.cycles += 1;
                                    c.g[S0][0] = c.g[S6][0];
                                    break 'b_308ee8;
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S7][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28c));
                        if event_due(c) {
                            deliver_event_here(c, 0x00308e10);
                        }
                        continue 'l_308e10;
                    } else {
                        c.cycles += 1;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x2a8), 0u64);
                        break 'b_308ef8;
                    }
                }
            }
        }
        c.cycles += 16;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x290), 0u64);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x294), 0u64);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x298), 0u64);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x2a4), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        Ok(())
    })
}

/// `void AI_CreateComputerRiders(AI* self)` (0x129e20): create the computer
/// riders up to the race's player count (`0x535c04`): for each missing one
/// (index `self[0x78]` upward) read its character from the player
/// interface, build a `ComputerActorRider` (named allocation of 0x1a90
/// bytes, constructed with `(1, 0)`), store its rider in `self[0x28 + 4i]`
/// with its index (`rider + 0x86c`) and controller -1 (`+0x870`), clear the
/// interface's controller/character, create its metrix, set attributes and
/// difficulty, apply the AI rules, assign the race slot, and record the
/// actor in `self[0x48 + 4 * numComputers]`. Finally both interfaces'
/// `update` (virtual +0x8/+0xc) run, the count is stored at `+0x78` and the
/// "created" flag at `+0x10`.
#[allow(unreachable_code)]
pub fn ai_create_computer_riders(c: &mut Cpu) {
    guest(c, |c| {
        'b_129f94: {
            c.cycles += 28;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S2][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x00129e54)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00129e60)?;
            c.g[S7][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x00129e68)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x00129e74)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x80));
            c.g[V0][0] = 0x530000;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c04));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            let t = (c.g[A0][0] as i64) <= 0;
            c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x78));
            if t {
                break 'b_129f94;
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[S6][0] = c.g[A0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[FP][0] = 0x450000;
                c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                'l_129ea8: loop {
                    'b_129eec: {
                        c.cycles += 16;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x00147398, 0x00129eb4)?;
                        c.g[A3][0] = 0u64;
                        c.g[A0][0] = 0x1a90;
                        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x7f30));
                        c.g[A2][0] = 0xffffffff80000000;
                        c.g[S5][0] = c.g[V0][0];
                        call_ptr(c, 0x00317d70, 0x00129ecc)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0x1;
                        c.g[A2][0] = 0u64;
                        Callee {
                            addr: 0x00108c80,
                            f: ee::s00::ComputerActorRider_ctor,
                        }
                        .call(c, 0x00129edc)?;
                        c.g[S0][0] = c.g[V0][0];
                        let t = c.g[S0][0] == 0u64;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_129eec;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                            break 'b_129eec;
                        }
                    }
                    c.cycles += 42;
                    c.sw(c.g[S4][0] as u32, c.g[V0][0]);
                    c.g[A1][0] = c.g[S1][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x86c), c.g[S1][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = 0xffffffffffffffff;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1));
                    call_ptr(c, 0x00147220, 0x00129f08)?;
                    c.g[A2][0] = c.g[S5][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x00147338, 0x00129f18)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00129f2c)?;
                    c.g[V0][0] = c.lw(c.g[S4][0] as u32);
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x870), c.g[V1][0]);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    Callee {
                        addr: 0x0011bb98,
                        f: ee::s00::Rider_CreateRiderMetrix,
                    }
                    .call(c, 0x00129f44)?;
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0010c9a8,
                        f: ee::s00::ComputerActorRider_SetAttribsForPeak,
                    }
                    .call(c, 0x00129f4c)?;
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0010c758,
                        f: ee::s00::ComputerActorRider_InitDifficulty,
                    }
                    .call(c, 0x00129f54)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0012b698,
                        f: ee::s00::AI_ApplyComputerRiderSettings,
                    }
                    .call(c, 0x00129f60)?;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S7][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00144d70, 0x00129f74)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x80));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x48), c.g[S0][0]);
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x80));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    let t = c.g[S6][0] != 0u64;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x80), c.g[V1][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129ea8);
                        }
                        continue 'l_129ea8;
                    } else {
                        break 'b_129f94;
                    }
                }
            }
        }
        c.cycles += 25;
        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00129fa8)?;
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00129fbc)?;
        c.g[V0][0] = 0x1;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x78), c.g[S1][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    })
}

/// `void AI_CreatePlayers(AI* self)` (0x1298e0): create the human players
/// (count at `0x535bfc`). Clears the two player slots (`+0x40/+0x44`), gets
/// the race, player and input interfaces, and for each player: reads its
/// controller id, builds a `Player` (named allocation of 0x1950 bytes,
/// constructed with 1), stores its rider in `self[0x28 + 4i]` with its
/// index/controller (`rider + 0x86c/0x870`), and wires its input: in a
/// network game the local pad (`InputSync_AddLocalPad`) or the remote pad
/// (`InputSync_AddRemotePad`, `rider + 0x87c = 0`) depending on host and
/// slot, otherwise the pad from the cache. Then the focus box, metrix and
/// rules (`AI_ApplyPlayerRiderSettings`), `Rider_Load`, and the race slot
/// assignment. After the loop the network players' controllers are set
/// (0 / -2) and the race interface's `update` runs; `+0x7c` and `+0x78` get
/// the player count and the next index.
#[allow(unreachable_code)]
pub fn ai_create_players(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
        c.g[V1][0] = 0x1;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x44));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        'l_129920: loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00129920);
                }
                continue 'l_129920;
            } else {
                'b_129b5c: {
                    c.cycles += 22;
                    c.g[S3][0] = 0u64;
                    call_ptr(c, 0x0014dc80, 0x00129944)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x00129950)?;
                    c.g[S5][0] = 0u64;
                    c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                    c.g[FP][0] = 0u64;
                    call_ptr(c, 0x0014dc80, 0x00129960)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014dd58, 0x0012996c)?;
                    c.g[S7][0] = c.g[V0][0];
                    c.g[A1][0] = 0x7;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x00129980)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    if t {
                        break 'b_129b5c;
                    } else {
                        c.cycles += 3;
                        c.g[S6][0] = c.g[S0][0];
                        c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x28));
                        'l_1299a0: loop {
                            'b_1299e4: {
                                c.cycles += 16;
                                c.g[A0][0] = c.g[S7][0];
                                c.g[A1][0] = c.g[S5][0];
                                call_ptr(c, 0x00147290, 0x001299ac)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A3][0] = 0u64;
                                c.g[V0][0] = 0x4a0000;
                                c.g[A0][0] = 0x1950;
                                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1108));
                                c.g[A2][0] = 0xffffffff80000000;
                                call_ptr(c, 0x00317d70, 0x001299c8)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x1;
                                Callee {
                                    addr: 0x00125c70,
                                    f: ee::s00::Player_ctor,
                                }
                                .call(c, 0x001299d4)?;
                                c.g[S1][0] = c.g[V0][0];
                                let t = c.g[S1][0] == 0u64;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_1299e4;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    break 'b_1299e4;
                                }
                            }
                            'b_129af4: {
                                'b_129ab0: {
                                    c.cycles += 12;
                                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                                    c.g[A0][0] = c.g[S7][0];
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x86c), c.g[S3][0]);
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x870), c.g[S3][0]);
                                    call_ptr(c, 0x00147618, 0x00129a00)?;
                                    c.g[A1][0] = 0x530000;
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4b30));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4b30));
                                    if t {
                                        break 'b_129ab0;
                                    } else {
                                        'b_129a84: {
                                            'b_129a3c: {
                                                'b_129a30: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_129a30;
                                                    } else {
                                                        c.cycles += 2;
                                                        if c.g[S5][0] != c.g[V0][0] {
                                                            c.g[A2][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                            break 'b_129a3c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x18));
                                                            if t {
                                                                break 'b_129a84;
                                                            } else {
                                                                break 'b_129a30;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                if c.g[S5][0] == 0u64 {
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                                    break 'b_129a84;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[A2][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    break 'b_129a3c;
                                                }
                                            }
                                            c.cycles += 18;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xb0));
                                            c.g[V1][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                            c.g[A1][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xb8));
                                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                                            c.g[V1][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x84));
                                            c.g[A1][0] = c.lw(c.g[A0][0] as u32);
                                            c.g[A0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                                            call_ptr(c, 0x00237140, 0x00129a6c)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x850));
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                            c.g[A0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x2eec));
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0xdf0),
                                                c.g[V0][0],
                                            );
                                            let t = true;
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0xdf4),
                                                c.g[A0][0],
                                            );
                                            if t {
                                                break 'b_129af4;
                                            } else {
                                                break 'b_129a84;
                                            }
                                        }
                                        c.cycles += 11;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.sw((c.g[A0][0] as u32).wrapping_add(0x87c), 0u64);
                                        c.g[V0][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xb8));
                                        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x7c));
                                        call_ptr(c, 0x00237280, 0x00129aa4)?;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0xdf0), c.g[V0][0]);
                                        let t = true;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0xdf4), 0u64);
                                        if t {
                                            break 'b_129af4;
                                        } else {
                                            break 'b_129ab0;
                                        }
                                    }
                                }
                                c.cycles += 17;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[A3][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x850));
                                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xb8));
                                c.g[A1][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2eec));
                                c.sw((c.g[S1][0] as u32).wrapping_add(0xdf0), c.g[V0][0]);
                                c.sw((c.g[S1][0] as u32).wrapping_add(0xdf4), c.g[A0][0]);
                                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x870), c.g[S3][0]);
                                break 'b_129af4;
                            }
                            c.cycles += 26;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1));
                            Callee {
                                addr: 0x0011bb40,
                                f: ee::s00::Rider_CreateFocusBox,
                            }
                            .call(c, 0x00129b00)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            Callee {
                                addr: 0x0011bb98,
                                f: ee::s00::Rider_CreateRiderMetrix,
                            }
                            .call(c, 0x00129b08)?;
                            c.g[A2][0] = c.g[S5][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S1][0];
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                            Callee {
                                addr: 0x0012b498,
                                f: ee::s00::AI_ApplyPlayerRiderSettings,
                            }
                            .call(c, 0x00129b1c)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            c.g[A1][0] = 0u64;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x2f8), 0u64);
                            c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                            Callee {
                                addr: 0x0011c230,
                                f: ee::s00::Rider_Load,
                            }
                            .call(c, 0x00129b34)?;
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[A1][0] = c.g[S3][0];
                            c.g[A2][0] = c.g[S3][0];
                            call_ptr(c, 0x00144d70, 0x00129b44)?;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            c.sw(c.g[S6][0] as u32, c.g[S1][0]);
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001299a0);
                                }
                                continue 'l_1299a0;
                            } else {
                                break 'b_129b5c;
                            }
                        }
                    }
                }
                'b_129bac: {
                    c.cycles += 4;
                    c.g[A0][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4b30));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4b30));
                    if t {
                        break 'b_129bac;
                    } else {
                        'b_129b94: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_129b94;
                            } else {
                                c.cycles += 7;
                                c.g[A0][0] = 0xfffffffffffffffe;
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x870), 0u64);
                                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x44));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                let t = true;
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x870), c.g[A0][0]);
                                if t {
                                    break 'b_129bac;
                                } else {
                                    break 'b_129b94;
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = 0xfffffffffffffffe;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x870), c.g[A0][0]);
                        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x44));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x870), 0u64);
                        break 'b_129bac;
                    }
                }
                c.cycles += 20;
                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00129bc4)?;
                c.sw((c.g[S4][0] as u32).wrapping_add(0x7c), c.g[FP][0]);
                c.sw((c.g[S4][0] as u32).wrapping_add(0x78), c.g[S3][0]);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.pc = t;
                return Ok(());
            }
        }
        Ok(())
    })
}

/// `ScriptEngine_ChooseScript` offsets: the category filter table header
/// (`+0x53c`), its table of (count, scripts) pairs (`+0x540`) and the master
/// table of 0xc-byte entries whose first word is the priority (`+0x530`).
pub(crate) const SCRIPT_ENGINE_FILTER_HEADER: u32 = 0x53c;
pub(crate) const SCRIPT_ENGINE_FILTER_TABLE: u32 = 0x540;
pub(crate) const SCRIPT_ENGINE_MASTER_TABLE: u32 = 0x530;

/// `Script* ScriptEngine_ChooseScript(ScriptEngine* self, int category)`
/// (0x27b0c0): pick a script of a category for the current game state.
/// Evaluates the 21 choice flags (`ScriptEngine_EvalChoiceParams`), then
/// scans the category's scripts (0x30 bytes: id, then 21 halfword masks at
/// `+4`): a script qualifies when every non-zero mask has a bit set in the
/// matching flag word; only the best (lowest) priority of the master table is
/// kept (a new best resets the list, ties append). The winner is one of the
/// candidates, chosen with `Rand_NextMain() % n`, or 0. The candidate list
/// (100 pointers) is a temporary named allocation.
///
/// The generated body is a `pc` dispatcher whose loop heads (0x27b160,
/// 0x27b190, 0x27b194) are resume points: entered anywhere else, this
/// runs the generated body.
pub fn script_engine_choose_script(c: &mut Cpu) {
    if c.pc != 0x0027_b0c0 {
        return ee::s07::f_0027b0c0__generated(c);
    }
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S0, 0xb0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x90);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save_ra(c, 0x60);
        c.g[V0][0] = c.lw(oc428_at(c.g[S1][0], SCRIPT_ENGINE_FILTER_HEADER));
        let no_filter = c.g[V0][0] == 0;
        c.g[S3][0] = sx((c.g[A1][0] as u32) << 3);
        'epilogue: {
            'no_script: {
                if no_filter {
                    break 'no_script;
                }
                c.cycles += 5;
                c.g[V0][0] = c.lw(oc428_at(c.g[S1][0], SCRIPT_ENGINE_FILTER_TABLE));
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A1][0] = c.g[SP][0];
                if c.g[V1][0] == 0 {
                    break 'no_script;
                }
                // The 21 choice flags go to sp[0..21].
                c.cycles += 17;
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0027_bdb8,
                    f: ee::s07::ScriptEngine_EvalChoiceParams,
                }
                .call(c, 0x0027_b110)?;
                c.g[S4][0] = 0xffff; // best priority so far
                c.g[A1][0] = 0x48_0000;
                c.g[A0][0] = 0x190;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1ea0));
                c.g[A2][0] = 0x100;
                c.g[A3][0] = 0;
                c.g[S0][0] = 0; // candidates
                call_ptr(c, 0x0031_7e30, 0x0027_b130)?;
                c.g[S2][0] = c.g[V0][0]; // candidate list
                c.g[V1][0] = 0;
                c.g[V0][0] = c.lw(oc428_at(c.g[S1][0], SCRIPT_ENGINE_FILTER_TABLE));
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = c.lw(c.g[V0][0] as u32); // script count
                let empty = (c.g[A1][0] as i64) <= 0;
                c.g[T4][0] = c.lw(oc428_at(c.g[V0][0], 4)); // scripts
                if !empty {
                    c.cycles += 5;
                    c.g[T7][0] = c.lw(oc428_at(c.g[S1][0], SCRIPT_ENGINE_MASTER_TABLE));
                    c.g[T6][0] = 0x30;
                    c.g[T5][0] = 0xc;
                    c.mult(0, V0, V1, T6);
                    'scripts: loop {
                        c.cycles += 9;
                        c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                        c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
                        c.g[V0][0] = c.lw(c.g[T1][0] as u32); // script id
                        c.mult(0, V1, V0, T5);
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T7][0] as u32));
                        c.g[T0][0] = c.lw(c.g[V0][0] as u32); // priority
                        let worse = (c.g[S4][0] as i64) < (c.g[T0][0] as i64);
                        c.g[V1][0] = c.g[T3][0];
                        if !worse {
                            c.cycles += 3;
                            c.g[T2][0] = 0;
                            c.g[A3][0] = 0;
                            // The 21 masks against the flags.
                            'masks: loop {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A3][0] as i64) < 21) as u64;
                                let all_pass = c.g[V0][0] == 0;
                                c.g[V0][0] = sx((c.g[A3][0] as u32) << 1);
                                if all_pass {
                                    break 'masks;
                                }
                                c.cycles += 4;
                                c.g[V0][0] =
                                    sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A2][0] = c.lh(oc428_at(c.g[V0][0], 4));
                                let no_mask = c.g[A2][0] == 0;
                                c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
                                if no_mask {
                                    if event_due(c) && back_edge_unwinds(c, 0x0027_b190) {
                                        return Ok(());
                                    }
                                    c.cycles += 1;
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                                    continue 'masks;
                                }
                                c.cycles += 5;
                                c.g[A0][0] =
                                    sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                                c.g[V1][0] &= c.g[A2][0];
                                if c.g[V1][0] != 0 {
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                                    if back_edge_unwinds(c, 0x0027_b194) {
                                        return Ok(());
                                    }
                                    continue 'masks;
                                }
                                c.cycles += 1;
                                c.g[T2][0] = 1; // this script fails
                                break 'masks;
                            }
                            c.cycles += 2;
                            let failed = c.g[T2][0] != 0;
                            c.g[V1][0] = c.g[T3][0];
                            if !failed {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                                let not_better = c.g[V0][0] == 0;
                                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                                if !not_better {
                                    // A better priority: restart the candidate list.
                                    c.cycles += 3;
                                    c.g[S4][0] = c.g[T0][0];
                                    c.g[S0][0] = 0;
                                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                                }
                                c.cycles += 4;
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                                c.sw(c.g[V0][0] as u32, c.g[T1][0]);
                                c.g[V1][0] = c.g[T3][0];
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.mult(0, V0, V1, T6);
                        if more {
                            if event_due(c) && back_edge_unwinds(c, 0x0027_b160) {
                                return Ok(());
                            }
                            continue 'scripts;
                        }
                        break 'scripts;
                    }
                }
                // Pick one of the candidates at random.
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    c.g[S0][0] = 0;
                } else {
                    c.cycles += 5;
                    call_ptr(c, 0x0031_77f0, 0x0027_b214)?;
                    c.divu(0, V0, S0);
                    c.cycles += 4;
                    c.g[V1][0] = c.hi[0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[S0][0] = c.lw(c.g[V1][0] as u32);
                }
                c.cycles += 2;
                let no_list = c.g[S2][0] == 0;
                c.g[V0][0] = c.g[S0][0];
                if !no_list {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0031_7e98, 0x0027_b240)?;
                    c.g[V0][0] = c.g[S0][0];
                }
                break 'epilogue;
            }
            // 0x27b100: nothing to choose from.
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        // 0x27b244: epilogue.
        c.cycles += 8;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `void cGame_loadTrack(Game* self)` (0x22f6b0): load the race track's
/// world. Builds the world path `data/worlds/<location>` (the location name
/// from the race interface: `+0x44` of its record, or `+0x34` when
/// gp[-0x1f38] is zero), opens `<path>.big` in the game's big file, loads the
/// world (`World_Load`), then creates the sky box (0x18 bytes, +0x14), the
/// section manager (0x1d4, +0x78), the surface-property cache
/// (`SurfPropCache_InitDefaults`), the world sound (0x250, +0x4c) and the
/// forward-diff cache, loads the world's current section (event) and spins
/// `SectionMan_Update` / `World_Update` / `EAThread_Yield(0)` until it is
/// loaded, loads the camera triggers (`<path>/cam`), creates the fog
/// particle manager (0x3010 bytes) with its lens effect (+0x1c) and the view
/// queries.
///
/// The generated body is a `pc` dispatcher whose loop heads (0x22f890,
/// 0x22f978) are resume points: entered anywhere else, this runs the
/// generated body.
#[allow(unreachable_code)]
pub fn game_module_load_world(c: &mut Cpu) {
    if c.pc != 0x0022_f6b0 {
        return ee::s06::f_0022f6b0__generated(c);
    }
    guest(c, |c| {
        c.cycles += 21;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
        c.g[S4][0] = c.g[A0][0];
        call_ptr(c, 0x0014dc80, 0x0022f6e4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0014dd58, 0x0022f6f0)?;
        c.g[S5][0] = c.g[V0][0];
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f38));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        let t = c.g[V1][0] == 0u64;
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        if t {
            c.cycles += 25;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.g[S5][0];
            c.g[S3][0] = 0x480000;
            call_ptr(c, 0x00144bc0, 0x0022f780)?;
            c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4b80));
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x003182d8, 0x0022f790)?;
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[S6][0] = c.g[S0][0];
            call_ptr(c, 0x003186d0, 0x0022f7a4)?;
            c.g[FP][0] = c.g[S3][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x00318448, 0x0022f7b4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x00318260, 0x0022f7c0)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x00318260, 0x0022f7cc)?;
            c.g[A2][0] = 0x4a0000;
        } else {
            c.cycles += 26;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.g[S5][0];
            c.g[S3][0] = 0x480000;
            call_ptr(c, 0x00144bc0, 0x0022f718)?;
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4b80));
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003182d8, 0x0022f728)?;
            c.g[S6][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[FP][0] = c.g[S3][0];
            call_ptr(c, 0x003186d0, 0x0022f740)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x00318448, 0x0022f74c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x00318260, 0x0022f758)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x00318260, 0x0022f764)?;
            c.g[A2][0] = 0x4a0000;
        }
        c.cycles += 48;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2a90));
        c.g[A0][0] = c.g[S6][0];
        call_ptr(c, 0x00318630, 0x0022f7e0)?;
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x44));
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 0u64;
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xa8));
        c.g[S7][0] = sx((c.g[S4][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x00316978, 0x0022f7f8)?;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0022f804)?;
        c.g[A1][0] = c.lw(c.g[SP][0] as u32);
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
        call_ptr(c, 0x003a6ae0, 0x0022f814)?;
        c.g[A1][0] = 0x4a0000;
        c.g[A2][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2a98));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0x18;
        call_ptr(c, 0x00317d70, 0x0022f82c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x00353ac0, 0x0022f834)?;
        c.g[A1][0] = 0x480000;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48d8));
        c.g[A0][0] = 0x1d4;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        call_ptr(c, 0x00317d70, 0x0022f850)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0022cce8, 0x0022f85c)?;
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x78), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48c8));
        c.g[A0][0] = 0xd10;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x0022f878)?;
        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[A1][0] = 0xffffffffffffffff;
        c.g[V1][0] = 0x12;
        // 19 passes of an empty counting loop (the compiler kept the loop).
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.g[V1][0] = 0x3;
            c.g[V1][0] = c.g[A0][0];
            let t = c.g[V1][0] != c.g[A1][0];
            if t {
                if back_edge_unwinds(c, 0x0022_f890) {
                    return Ok(());
                }
                continue;
            }
            break;
        }
        c.cycles += 50;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[S0][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x4b80));
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x44));
        call_ptr(c, 0x002c2f18, 0x0022f8bc)?;
        c.g[A1][0] = 0x480000;
        c.g[A3][0] = 0u64;
        c.g[A2][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48b8));
        c.g[A0][0] = 0x250;
        call_ptr(c, 0x00317d70, 0x0022f8d4)?;
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002b6aa0, 0x0022f8e0)?;
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
        c.sw((c.g[S4][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
        call_ptr(c, 0x00334800, 0x0022f8ec)?;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        c.g[A0][0] = c.g[S5][0];
        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x3d4), 0u64);
        call_ptr(c, 0x00144bc0, 0x0022f908)?;
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003182d8, 0x0022f914)?;
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x003186d0, 0x0022f924)?;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x00318448, 0x0022f930)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0022f93c)?;
        c.g[A1][0] = 0x2;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00318260, 0x0022f948)?;
        c.g[A0][0] = c.g[S5][0];
        call_ptr(c, 0x00144bc0, 0x0022f950)?;
        c.g[S0][0] = c.lw(c.g[V0][0] as u32);
        c.g[A2][0] = 0u64;
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x78));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0022d088, 0x0022f964)?;
        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x78));
        c.g[V0][0] = 0x1;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1a4), c.g[V0][0]);
        // Wait for the section to load (the first pass skips the update).
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x78));
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0022d278, 0x0022f99c)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A3][0] = 0x1;
            if t {
                if back_edge_unwinds(c, 0x0022_f978) {
                    return Ok(());
                }
                c.cycles += 6;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x78));
                call_ptr(c, 0x0022d8d8, 0x0022f980)?;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x003a6928, 0x0022f988)?;
                c.g[A0][0] = 0u64;
                call_ptr(c, 0x003e5398, 0x0022f990)?;
                continue;
            }
            break;
        }
        c.cycles += 61;
        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 0x4a0000;
        c.g[A2][0] = c.g[S6][0];
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2aa0));
        c.g[A0][0] = c.g[S3][0];
        c.g[S0][0] = 0x4c0000;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x3d4), c.g[A3][0]);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5830));
        call_ptr(c, 0x003186d0, 0x0022f9cc)?;
        c.g[A2][0] = 0x4a0000;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2aa8));
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x00318630, 0x0022f9e0)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016ca10, 0x0022f9ec)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0022f9f8)?;
        c.g[A1][0] = 0x2;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x00318260, 0x0022fa04)?;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = 0x3010;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x48a8));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x0022fa1c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002dbf80, 0x0022fa28)?;
        c.g[V0][0] = 0x490000;
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7d20));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V0][0] = 0xff;
        c.sw(c.g[S7][0] as u32, c.g[S0][0]);
        call_ptr(c, 0x002ec3e0, 0x0022fa48)?;
        c.sw((c.g[S4][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        call_ptr(c, 0x002ed048, 0x0022fa50)?;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0022fa5c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0022fa68)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// A 32-bit word of test data.
    fn oc428_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// The BELibrary dummy (`gp - 0x1f04`): without it a library lookup in a
    /// real callee loads from disc forever.
    fn oc428_belib(c: &mut Cpu) {
        oc428_w(c, GAME_GP - 0x1f04, DATA + 0x8000);
    }

    /// The one-word getters read a field of `a0`.
    fn oc428_getter_test(generated: GuestFn, hand: GuestFn, salt: u64, off: u32) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = Rng(salt ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x100);
            c.bus.write32(DATA + 0x100 + off, r.next_u32());
        });
    }

    /// `ScriptData_GetInfo` reads `+0x14`.
    #[test]
    fn script_data_get_info_matches() {
        oc428_getter_test(
            ee::s07::f_0027c070__generated,
            script_data_get_info,
            0x27c070,
            SCRIPT_DATA_INFO,
        );
    }

    /// `ScriptCmdQueue_IsFull` reads `+0x94`.
    #[test]
    fn script_cmd_queue_is_full_matches() {
        oc428_getter_test(
            ee::s07::f_00283c58__generated,
            script_cmd_queue_is_full,
            0x283c58,
            SCRIPT_CMD_QUEUE_FULL,
        );
    }

    /// `ScriptView_GetParams` reads `+0x20`.
    #[test]
    fn script_view_get_params_matches() {
        oc428_getter_test(
            ee::s07::f_0027c888__generated,
            script_view_get_params,
            0x27c888,
            SCRIPT_VIEW_PARAMS,
        );
    }

    /// `ScriptCam_GetManualParams` reads `+0x20`.
    #[test]
    fn script_cam_get_manual_params_matches() {
        oc428_getter_test(
            ee::s07::f_0027c098__generated,
            script_cam_get_manual_params,
            0x27c098,
            SCRIPT_CAM_MANUAL_PARAMS,
        );
    }

    /// `ScriptBinding_ctor` writes the vtable, -1 and 0 into the object.
    #[test]
    fn script_binding_ctor_matches() {
        check(ee::s07::f_00282f60__generated, script_binding_ctor, 20, &|c, seed| {
            let mut r = Rng(0x282f60 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x100);
            for k in 0..4 {
                c.bus.write32(DATA + 0x100 + 4 * k, r.next_u32());
            }
        });
    }

    /// `cGameLoadScreen_TryExit` against the generated body: the progress
    /// global at 100% (the exit request runs) or not.
    #[test]
    fn c_game_load_screen_try_exit_matches() {
        crate::difftest::check(
            ee::s06::f_002332c8__generated,
            c_game_load_screen_try_exit,
            60,
            &|c, seed| {
                machine(c, seed, 0x2332c8, 1);
                oc428_belib(c);
                c.g[A0][0] = DATA as u64;
                oc428_w(c, GAME_GP + 0x2a74, DATA + 0x100);
                let progress: f32 = if seed % 2 == 0 { 100.0 } else { 1.0 };
                oc428_w(c, DATA + 0x100 + 0x38, progress.to_bits());
            },
        );
    }

    /// One `cLunoTable_Find` fixture: a table of `m` buckets in `DATA +
    /// 0x1000`, each with a chain of up to 3 nodes; keys of every type pair
    /// (equal types, int key / float node, float key / int node, type 3,
    /// others), some matching in value, some not; some empty buckets.
    fn oc428_luno_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x225248 ^ (seed as u64) << 4);
        scramble(c, &mut r);
        let (vm, table, nodes, key) = (DATA + 0x100, DATA + 0x200, DATA + 0x1000, DATA + 0x300);
        let m = 1 + r.next_u32() % 4;
        c.g[A0][0] = sx(vm);
        c.g[A1][0] = sx(key);
        oc428_w(c, vm, table);
        oc428_w(c, table + LUNO_TABLE_MODULUS, m);
        oc428_w(c, table + LUNO_TABLE_NODES, nodes);
        let key_type = [1u32, 2, 3, 1, 2, 0, 5][(r.next_u32() % 7) as usize];
        let small = r.next_u32() % 6;
        let key_word = if key_type == 2 {
            (small as f32).to_bits().wrapping_add(r.next_u32() % 2)
        } else {
            small
        };
        oc428_w(c, key, key_word);
        oc428_w(c, key + 8, key_type);
        let bucket = key_word % m;
        // Chain: the bucket node, then up to two spare nodes.
        let mut node = nodes + bucket * 0x20;
        let len = 1 + r.next_u32() % 3;
        for i in 0..len {
            let node_type = [1u32, 2, 3, 1, 2, 0, 5][(r.next_u32() % 7) as usize];
            let word = match r.next_u32() % 4 {
                0 => small,
                1 => (small as f32).to_bits(),
                2 => key_word,
                _ => r.next_u32(),
            };
            let value_type = if seed % 9 == 0 && i == 0 {
                0
            } else {
                1 + r.next_u32() % 4
            };
            oc428_w(c, node, word);
            oc428_w(c, node + LUNO_NODE_KEY_TYPE, node_type);
            oc428_w(c, node + LUNO_NODE_VALUE, r.next_u32());
            oc428_w(c, node + LUNO_NODE_VALUE_TYPE, value_type);
            let next = nodes + (m + i) * 0x20;
            oc428_w(c, node + LUNO_NODE_NEXT, if i + 1 < len { next } else { 0 });
            node = next;
        }
    }

    /// `cLunoTable_Find` against the generated body.
    #[test]
    fn c_luno_table_find_matches() {
        check(
            ee::s05::f_00225248__generated,
            c_luno_table_find,
            600,
            &oc428_luno_setup,
        );
    }

    use std::ptr::addr_of_mut;

    /// Fixture controls (above the fakes' log, `DATA + 0x200000`).
    const OC428_CTRL: u32 = DATA + 0x20_0000;
    const OC428_MAGIC: u32 = 0x0c42_8c42;
    /// Forced results of fake callees, keyed by callee address.
    const OC428_FORCE: u32 = OC428_CTRL + 0x100;
    /// Scratch words the fakes' side effects read (`ReadFields` results...).
    const OC428_AUX: u32 = OC428_CTRL + 0x8000;

    /// Switch the gated fakes on for this machine.
    fn oc428_gate(c: &mut Cpu) {
        c.bus.write32(OC428_CTRL, OC428_MAGIC);
    }

    /// Force the result (`v0`) of fake callee `addr` (a pointer-sized value,
    /// also 0: the fakes alone never return 0).
    fn oc428_force(c: &mut Cpu, addr: u32, v: u32) {
        let slot = OC428_FORCE + ((addr >> 2) % 0x7ff) * 8;
        c.bus.write32(slot, 1);
        c.bus.write32(slot + 4, v);
    }

    /// A fake callee: log (`fakes::fake`), then the forced result, then
    /// the callee-specific side effects.
    fn oc428_fake_call(c: &mut Cpu, addr: u32, ra: u32) {
        let (a0, a1) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let slot = OC428_FORCE + ((addr >> 2) % 0x7ff) * 8;
        if c.bus.read32(slot) != 0 {
            c.g[V0][0] = c.bus.read32(slot + 4) as u64;
        }
        match addr {
            // ScriptView_ReadFields(view, info): info = { count, first node }.
            0x0027_c8e0 => {
                let count = c.bus.read32(OC428_AUX);
                let first = c.bus.read32(OC428_AUX + 4);
                c.bus.write32(a1, count);
                c.bus.write32(a1 + 4, first);
            }
            // BELibrary_GetInterface(lib, n): the fixture's interface objects.
            0x0014_dd58 => {
                let w = if a1 == 0 {
                    OC428_AUX + 0x10
                } else {
                    OC428_AUX + 0x14
                };
                c.g[V0][0] = c.bus.read32(w) as u64;
            }
            // ComputerActorRider_ctor: a new object whose +0x18 is its rider.
            0x0010_8c80 => {
                let obj = c.g[V0][0] as u32;
                if obj != 0 {
                    c.bus.write32(obj + 0x18, OC428_AUX + 0x8000);
                }
            }
            // Rider_LoadStep(rider): the rider is loaded (state 7).
            0x0011_c298 => c.bus.write32(a0 + 0x880, 7),
            _ => {}
        }
    }

    macro_rules! oc428_fakes {
        ($(($name:ident, $addr:expr, $shard:ident, $slot:ident, $gen:ident)),* $(,)?) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC428_CTRL) != OC428_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), ee::$shard::$gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    oc428_fake_call(c, $addr, ra);
                }
            )*

            /// Hooks the gated fakes into the callees' override slots (once
            /// per process; with the gate off they run what the slots held).
            fn oc428_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc428_fakes! {
        (oc428_fake_00224da0, 0x0022_4da0, s05, OVERRIDE_00224DA0, f_00224da0__generated),
        (oc428_fake_00224df0, 0x0022_4df0, s05, OVERRIDE_00224DF0, f_00224df0__generated),
        (oc428_fake_00317d70, 0x0031_7d70, s10, OVERRIDE_00317D70, f_00317d70__generated),
        (oc428_fake_003660a8, 0x0036_60a8, s12, OVERRIDE_003660A8, f_003660a8__generated),
        (oc428_fake_00362cc8, 0x0036_2cc8, s12, OVERRIDE_00362CC8, f_00362cc8__generated),
        (oc428_fake_00230698, 0x0023_0698, s06, OVERRIDE_00230698, f_00230698__generated),
        (oc428_fake_002e4578, 0x002e_4578, s09, OVERRIDE_002E4578, f_002e4578__generated),
        (oc428_fake_002ea820, 0x002e_a820, s09, OVERRIDE_002EA820, f_002ea820__generated),
        (oc428_fake_00317670, 0x0031_7670, s10, OVERRIDE_00317670, f_00317670__generated),
        (oc428_fake_0039f9d8, 0x0039_f9d8, s13, OVERRIDE_0039F9D8, f_0039f9d8__generated),
        (oc428_fake_00277598, 0x0027_7598, s07, OVERRIDE_00277598, f_00277598__generated),
        (oc428_fake_002e4ce8, 0x002e_4ce8, s09, OVERRIDE_002E4CE8, f_002e4ce8__generated),
        (oc428_fake_002e4d70, 0x002e_4d70, s09, OVERRIDE_002E4D70, f_002e4d70__generated),
        (oc428_fake_002ebba8, 0x002e_bba8, s09, OVERRIDE_002EBBA8, f_002ebba8__generated),
        (oc428_fake_002ebe20, 0x002e_be20, s09, OVERRIDE_002EBE20, f_002ebe20__generated),
        (oc428_fake_0027c8e0, 0x0027_c8e0, s07, OVERRIDE_0027C8E0, f_0027c8e0__generated),
        (oc428_fake_00281ac8, 0x0028_1ac8, s07, OVERRIDE_00281AC8, f_00281ac8__generated),
        (oc428_fake_00281c68, 0x0028_1c68, s07, OVERRIDE_00281C68, f_00281c68__generated),
        (oc428_fake_003177f0, 0x0031_77f0, s10, OVERRIDE_003177F0, f_003177f0__generated),
        (oc428_fake_00317e30, 0x0031_7e30, s10, OVERRIDE_00317E30, f_00317e30__generated),
        (oc428_fake_00317e98, 0x0031_7e98, s10, OVERRIDE_00317E98, f_00317e98__generated),
        (oc428_fake_0030bac8, 0x0030_bac8, s10, OVERRIDE_0030BAC8, f_0030bac8__generated),
        (oc428_fake_002224a8, 0x0022_24a8, s05, OVERRIDE_002224A8, f_002224a8__generated),
        (oc428_fake_002224b8, 0x0022_24b8, s05, OVERRIDE_002224B8, f_002224b8__generated),
        (oc428_fake_00225b90, 0x0022_5b90, s05, OVERRIDE_00225B90, f_00225b90__generated),
        (oc428_fake_00226600, 0x0022_6600, s05, OVERRIDE_00226600, f_00226600__generated),
        (oc428_fake_003a6c68, 0x003a_6c68, s13, OVERRIDE_003A6C68, f_003a6c68__generated),
        (oc428_fake_003a6c98, 0x003a_6c98, s13, OVERRIDE_003A6C98, f_003a6c98__generated),
        (oc428_fake_003a9800, 0x003a_9800, s13, OVERRIDE_003A9800, f_003a9800__generated),
        (oc428_fake_00309b70, 0x0030_9b70, s10, OVERRIDE_00309B70, f_00309b70__generated),
        (oc428_fake_00144d70, 0x0014_4d70, s01, OVERRIDE_00144D70, f_00144d70__generated),
        (oc428_fake_00147220, 0x0014_7220, s01, OVERRIDE_00147220, f_00147220__generated),
        (oc428_fake_00147338, 0x0014_7338, s01, OVERRIDE_00147338, f_00147338__generated),
        (oc428_fake_00147398, 0x0014_7398, s01, OVERRIDE_00147398, f_00147398__generated),
        (oc428_fake_0014dc80, 0x0014_dc80, s01, OVERRIDE_0014DC80, f_0014dc80__generated),
        (oc428_fake_0014dd58, 0x0014_dd58, s01, OVERRIDE_0014DD58, f_0014dd58__generated),
        (oc428_fake_00108c80, 0x0010_8c80, s00, OVERRIDE_00108C80, f_00108c80__generated),
        (oc428_fake_0010c758, 0x0010_c758, s00, OVERRIDE_0010C758, f_0010c758__generated),
        (oc428_fake_0010c9a8, 0x0010_c9a8, s00, OVERRIDE_0010C9A8, f_0010c9a8__generated),
        (oc428_fake_0012b698, 0x0012_b698, s00, OVERRIDE_0012B698, f_0012b698__generated),
        (oc428_fake_00147290, 0x0014_7290, s01, OVERRIDE_00147290, f_00147290__generated),
        (oc428_fake_00147618, 0x0014_7618, s01, OVERRIDE_00147618, f_00147618__generated),
        (oc428_fake_0011bb40, 0x0011_bb40, s00, OVERRIDE_0011BB40, f_0011bb40__generated),
        (oc428_fake_00144bc0, 0x0014_4bc0, s01, OVERRIDE_00144BC0, f_00144bc0__generated),
        (oc428_fake_0022cce8, 0x0022_cce8, s05, OVERRIDE_0022CCE8, f_0022cce8__generated),
        (oc428_fake_0022d278, 0x0022_d278, s05, OVERRIDE_0022D278, f_0022d278__generated),
        (oc428_fake_0022d8d8, 0x0022_d8d8, s05, OVERRIDE_0022D8D8, f_0022d8d8__generated),
        (oc428_fake_002b6aa0, 0x002b_6aa0, s08, OVERRIDE_002B6AA0, f_002b6aa0__generated),
        (oc428_fake_002dbf80, 0x002d_bf80, s09, OVERRIDE_002DBF80, f_002dbf80__generated),
        (oc428_fake_00318260, 0x0031_8260, s10, OVERRIDE_00318260, f_00318260__generated),
        (oc428_fake_003182d8, 0x0031_82d8, s10, OVERRIDE_003182D8, f_003182d8__generated),
        (oc428_fake_00318448, 0x0031_8448, s10, OVERRIDE_00318448, f_00318448__generated),
        (oc428_fake_00318630, 0x0031_8630, s10, OVERRIDE_00318630, f_00318630__generated),
        (oc428_fake_003186d0, 0x0031_86d0, s10, OVERRIDE_003186D0, f_003186d0__generated),
        (oc428_fake_00334800, 0x0033_4800, s11, OVERRIDE_00334800, f_00334800__generated),
        (oc428_fake_00353ac0, 0x0035_3ac0, s11, OVERRIDE_00353AC0, f_00353ac0__generated),
        (oc428_fake_003a6928, 0x003a_6928, s13, OVERRIDE_003A6928, f_003a6928__generated),
        (oc428_fake_003e5398, 0x003e_5398, s14, OVERRIDE_003E5398, f_003e5398__generated),
        (oc428_fake_00117540, 0x0011_7540, s00, OVERRIDE_00117540, f_00117540__generated),
        (oc428_fake_001172e0, 0x0011_72e0, s00, OVERRIDE_001172E0, f_001172e0__generated),
        (oc428_fake_0011b698, 0x0011_b698, s00, OVERRIDE_0011B698, f_0011b698__generated),
        (oc428_fake_0010fcd8, 0x0010_fcd8, s00, OVERRIDE_0010FCD8, f_0010fcd8__generated),
        (oc428_fake_00270070, 0x0027_0070, s07, OVERRIDE_00270070, f_00270070__generated),
        (oc428_fake_0014f7a8, 0x0014_f7a8, s01, OVERRIDE_0014F7A8, f_0014f7a8__generated),
        (oc428_fake_00117520, 0x0011_7520, s00, OVERRIDE_00117520, f_00117520__generated),
        (oc428_fake_00120d58, 0x0012_0d58, s00, OVERRIDE_00120D58, f_00120d58__generated),
        (oc428_fake_00125958, 0x0012_5958, s00, OVERRIDE_00125958, f_00125958__generated),
        (oc428_fake_0011c298, 0x0011_c298, s00, OVERRIDE_0011C298, f_0011c298__generated),
    }

    use crate::difftest::{fill, put_vec};

    /// `WScriptMan_CreateLocalTable` against the generated body: slot `i`
    /// empty (no destroy) or holding a table (destroyed first), over all
    /// four slots; the destructor, allocator and constructor are fakes.
    #[test]
    fn w_script_man_create_local_table_matches() {
        oc428_install_fakes();
        fakes::covered(
            ee::s10::f_00308b78__generated,
            w_script_man_create_local_table,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x308b78, 3);
                oc428_gate(c);
                let this = DATA + 0x1000;
                let i = seed % 4;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = i as u64;
                let old = if seed % 3 == 0 { 0 } else { fakes::ptr(&mut r) };
                c.bus.write32(this + 0x3cc + 4 * i, old);
            },
        );
    }

    /// `VramHeap_Init` against the generated body: the link array (any
    /// previous contents), the slot arrays and heads, and the one call to
    /// `VramHeap_LinkFreeBlock` (a fake, sometimes unwinding).
    #[test]
    fn vram_heap_init_matches() {
        oc428_install_fakes();
        fakes::covered(ee::s12::f_00365e40__generated, vram_heap_init, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x365e40, 1);
            oc428_gate(c);
            let this = DATA + 0x3_0000;
            let links = DATA + 0x4_0000;
            fill(c, &mut r, this, 0x900);
            fill(c, &mut r, links, 0x800);
            c.bus.write32(this + 0x1ff0, links);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(r.next_u32());
            c.g[A2][0] = sx(r.next_u32());
            c.g[A3][0] = sx(r.next_u32());
        });
    }

    /// `DeferMan_Init` against the generated body: the GS context object
    /// behind `gp[0x2a90] + 0xe84` with random old contents, the template at
    /// 0x501420, the packet count 0..7; `DeferMan_ResetFrame` is a fake.
    #[test]
    fn defer_man_init_matches() {
        oc428_install_fakes();
        fakes::covered(ee::s12::f_003629b8__generated, defer_man_init, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3629b8, 1);
            oc428_gate(c);
            let this = DATA + 0x4_0000;
            let gfx = DATA + 0x1000;
            let ctx = DATA + 0x2000;
            fill(c, &mut r, ctx, 8);
            c.bus.write32(GAME_GP + 0x2a90, gfx);
            c.bus.write32(gfx + 0xe84, ctx);
            fill(c, &mut r, 0x0050_1420, 5);
            c.bus.write32(this + 0x5_1480, r.next_u32() % 8);
            c.g[A0][0] = sx(this);
        });
    }

    /// `ScriptList_Stop` against the generated body, six situations by seed:
    /// paused/pending-unpause lists (state not 3, so the real
    /// `ScriptList_SetPaused` returns at once), and lists in state 3 with a
    /// cut-scene transition (reset requested or not, with and without the
    /// transition manager's stop, `hud` state found or missing); the other
    /// callees are fakes.
    #[test]
    fn script_list_stop_matches() {
        oc428_install_fakes();
        fakes::covered(ee::s07::f_00276868__generated, script_list_stop, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x276868, 7);
            oc428_gate(c);
            let (list, app, game, ui) =
                (DATA + 0x1000, DATA + 0x2000, DATA + 0x2100, DATA + 0x2200);
            let (hud, hud_vt) = (DATA + 0x3000, DATA + 0x3100);
            fill(c, &mut r, list, 0x40);
            let kind = seed % 6;
            let (state, unpause, pending, active) = match kind {
                0 => (1, 1, 1, 1),
                1..=3 => (3, 0, 0, 1),
                4 => (3, 0, 0, 0),
                _ => (2, 1, 0, 1),
            };
            c.bus.write32(list + 4, active);
            c.bus.write32(list + 0xa4, state);
            c.bus.write32(list + 0xa8, unpause);
            c.bus.write32(list + 0xc4, pending);
            c.bus.write32(list + 0xb0, (kind != 2) as u32);
            c.bus.write32(GAME_GP - 0x848, app);
            c.bus.write32(app + 0x84, game);
            c.bus.write32(game + 0x64, fakes::ptr(&mut r));
            c.bus.write32(game + 0x48, ui);
            c.bus.write32(hud + 8, hud_vt);
            c.bus.write16(hud_vt + 0xc0, (r.next_u32() & 0xfc) as u16);
            c.bus.write32(hud_vt + 0xc4, FAKE + 0x20);
            oc428_force(c, 0x0039_f9d8, if (seed / 6) % 2 == 0 { hud } else { 0 });
            c.g[A0][0] = sx(list);
            c.g[A1][0] = (kind != 3) as u64;
        });
    }

    /// `ScriptViewObj_Apply` against the generated body: every flag
    /// combination of bits 2 and 3, views with and without parameters, a
    /// node list with the type-8 head, shot counts 0-3, a fade object (the
    /// real `cTransitionMan_StopTransition` calls its vtable) or none; the
    /// field reader, slot binder and shot cutter are fakes.
    #[test]
    fn script_view_obj_apply_matches() {
        oc428_install_fakes();
        fakes::covered(
            ee::s07::f_002814f8__generated,
            script_view_obj_apply,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2814f8, 6);
                oc428_gate(c);
                let (this, view, params) = (DATA + 0x1000, DATA + 0x1100, DATA + 0x1200);
                let (nodes, kind, fade, fade_vt) =
                    (DATA + 0x4000, DATA + 0x4800, DATA + 0x5000, DATA + 0x5100);
                fill(c, &mut r, this, 0x80);
                c.bus.write32(this + 0xec, r.next_u32() % 2);
                c.bus.write32(this + 0xf0, (seed % 5 == 0) as u32);
                let has_fade = seed % 3 != 0;
                c.bus.write32(this + 0xd8, if has_fade { fade } else { 0 });
                c.bus.write32(view + 0x20, params);
                c.bus.write8(params, (seed % 7 != 0) as u8);
                // Shots: count and a chain of 0-3 nodes (bit, kind pointer, next).
                let count = [0u32, 1, 2, 3, 2, 1][((seed / 3) % 6) as usize];
                let len = (r.next_u32() % 4).min(3);
                for k in 0..3u32 {
                    let n = nodes + 0x100 * k;
                    c.bus.write32(n + 4, r.next_u32() % 3);
                    c.bus.write32(n + 0x14, kind + 4 * k);
                    c.bus.write8(
                        kind + 4 * k,
                        if k == 0 && seed % 2 == 0 {
                            8
                        } else {
                            r.next_u32() as u8
                        },
                    );
                    c.bus.write32(
                        n + 0x18,
                        if k + 1 < len {
                            nodes + 0x100 * (k + 1)
                        } else {
                            0
                        },
                    );
                }
                c.bus.write32(OC428_AUX, count);
                c.bus
                    .write32(OC428_AUX + 4, if len == 0 { 0 } else { nodes });
                // The fade object: its transition state and virtual calls.
                fill(c, &mut r, fade, 0x20);
                c.bus
                    .write32(fade + 0x18, if seed % 4 == 0 { 9 } else { 0 });
                c.bus.write32(fade + 0xc, fade_vt);
                c.bus.write16(fade_vt + 0x28, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(fade_vt + 0x2c, FAKE + 0x30);
                c.bus.write16(fade_vt + 0x20, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(fade_vt + 0x24, FAKE + 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(view);
                let flags = [0u32, 4, 8, 12][(seed % 4) as usize] | (r.next_u32() & 0xf0);
                c.g[A3][0] = sx(flags);
            },
        );
    }

    /// `WScriptFn_ObjectSlot36` against the generated body: the first-call
    /// initialisation (flag at `gp + 0x2550`) or a table left in RAM, 0-4
    /// argument entries (int/float slot types), and the target object found
    /// by handle through the two lookup tables, through the global object,
    /// or missing (`EA_MemCopy` real; the object's method is a fake call).
    #[test]
    fn w_script_fn_object_slot36_matches() {
        oc428_install_fakes();
        fakes::covered(
            ee::s10::f_002ff9a8__generated,
            w_script_fn_object_slot36,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ff9a8, 1);
                oc428_gate(c);
                let (out, entries) = (DATA + 0x7000, DATA + 0x7100);
                let (ht, h, arr, ent, back) = (
                    DATA + 0x8000,
                    DATA + 0x8100,
                    DATA + 0x8200,
                    DATA + 0x8300,
                    DATA + 0x8400,
                );
                let (global, obj, p, vt) =
                    (DATA + 0x8500, DATA + 0x6000, DATA + 0x6100, DATA + 0x6200);
                let table = 0x004f_baf8;
                let flag = seed % 2;
                c.bus.write32(GAME_GP + 0x2550, flag);
                // The handle in the table: none, found, or a dead slot.
                let handle = [
                    0xffff_ffffu32,
                    0x0000_0102,
                    0x0000_0103,
                    0x0000_0001,
                    0x0000_0201,
                ][((seed / 2) % 5) as usize];
                fill(c, &mut r, table, 8);
                c.bus.write32(table, handle);
                // Slot types of the argument indices 0..3.
                for i in 0..4 {
                    c.bus.write32(0x0044_6540 + 4 * i, 1 + r.next_u32() % 3);
                }
                // Lookup tables: handle -> slot 2 -> generation 1 -> object.
                c.bus.write32(GAME_GP + 0x16c8, ht);
                c.bus.write32(ht, h);
                c.bus.write32(h + 8, arr);
                c.bus.write32(arr + 8, ent);
                c.bus.write32(arr + 4, ent);
                c.bus.write32(arr + 0xc, 0);
                c.bus.write32(ent + 0x1c, back);
                c.bus.write32(back + 4, ((obj >> 2) << 8) | 5);
                c.bus.write32(back + 8, 0x0000_0005);
                c.bus.write32(back + 12, 0);
                c.bus.write32(GAME_GP + 0xce8, global);
                c.bus.write32(global + 0x290, obj);
                c.bus.write32(obj + 0xc, if seed % 7 == 0 { 0 } else { p });
                c.bus.write32(p + 0xc, vt);
                c.bus.write16(vt + 0x120, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(vt + 0x124, FAKE + 0x50);
                // Entries: (index, value, _, type).
                let n = (seed / 3) % 5;
                for k in 0..4u32 {
                    let e = entries + 0x10 * k;
                    let idx = if seed % 5 == 0 && k == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 3
                    };
                    c.bus.write32(e, idx);
                    c.bus
                        .write32(e + 4, if idx == 0 { handle } else { r.next_u32() % 100 });
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, 1 + r.next_u32() % 3);
                }
                c.g[A0][0] = sx(out);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(entries);
            },
        );
    }

    /// `ScriptEngine_GetTransitionTarget` against the generated body: every
    /// transition type 0-10 (0/8/9/10: none) in the three colour modes, the
    /// jump table written to RAM (with an unknown target for a few seeds:
    /// the generated `jump_out`); the allocator, the effect constructors and
    /// the wipe constructors are fakes.
    #[test]
    fn script_engine_get_transition_target_matches() {
        oc428_install_fakes();
        fakes::covered(
            ee::s07::f_0027a4a0__generated,
            script_engine_get_transition_target,
            264,
            &|c, seed| {
                let _r = machine(c, seed, 0x27a4a0, 3);
                oc428_gate(c);
                c.pc = 0x0027_a4a0;
                let kind = seed % 11;
                let mode = (seed / 11) % 3;
                let table = [
                    TRANSITION_NONE,
                    TRANSITION_FADE,
                    TRANSITION_WIPE_LEFT,
                    TRANSITION_WIPE_RIGHT,
                    TRANSITION_WIPE_UP,
                    TRANSITION_WIPE_DOWN,
                    TRANSITION_WIPE_RANDOM,
                    TRANSITION_WHITE_FADE,
                    TRANSITION_NONE,
                ];
                for (i, t) in table.iter().enumerate() {
                    c.bus.write32(0x0048_1df0 + 4 * i as u32, *t);
                }
                if seed % 37 == 5 && kind < 9 {
                    c.bus.write32(0x0048_1df0 + 4 * kind, 0x0027_a999);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = kind as u64;
                c.g[A2][0] = mode as u64;
            },
        );
    }

    /// `WScriptFn_AddCollectItems` against the generated body: first-call
    /// initialisation or not, 0-4 entries of int/float slot types (indices
    /// 0-20), with `args[0]` set (the real `EA_MemCopy`d table's first word,
    /// or an entry's value) or still -1; `WScriptMan_AddCollectItems` is a
    /// fake.
    #[test]
    fn w_script_fn_add_collect_items_matches() {
        oc428_install_fakes();
        fakes::covered(
            ee::s10::f_00300f50__generated,
            w_script_fn_add_collect_items,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x300f50, 1);
                oc428_gate(c);
                let (out, entries) = (DATA + 0x7000, DATA + 0x7100);
                c.bus.write32(GAME_GP + 0x25c0, seed % 2);
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x8000);
                let table = 0x004f_bb78;
                fill(c, &mut r, table, 0x16);
                if seed % 2 == 0 {
                    c.bus.write32(table, 0xffff_ffff);
                }
                for i in 0..21 {
                    c.bus.write32(0x0044_6570 + 4 * i, 1 + r.next_u32() % 3);
                }
                let n = (seed / 2) % 5;
                for k in 0..4u32 {
                    let e = entries + 0x10 * k;
                    let idx = if seed % 4 == 1 && k == 0 {
                        0
                    } else {
                        r.next_u32() % 21
                    };
                    c.bus.write32(e, idx);
                    c.bus.write32(e + 4, r.next_u32() % 1000);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, 1 + r.next_u32() % 3);
                }
                c.g[A0][0] = sx(out);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(entries);
            },
        );
    }

    /// `WScriptMan_RunMissionSetupScripts` against the generated body: 0-2
    /// sections with 0-2 missions each (the current one skipped for some),
    /// setup script found or not, local table allocated or not; every callee
    /// (world queries, allocator, Luno table/VM calls) is a fake.
    #[test]
    fn w_script_man_run_mission_setup_scripts_matches() {
        oc428_install_fakes();
        fakes::covered(
            ee::s10::f_00308db8__generated,
            w_script_man_run_mission_setup_scripts,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x308db8, 14);
                oc428_gate(c);
                let (this, world, defs) = (DATA + 0x1000, DATA + 0x2000, DATA + 0x9000);
                fill(c, &mut r, this + 0x280, 0x20);
                c.bus.write32(this + 0x28c, world);
                c.bus.write32(world, DATA + 0x2100);
                let current = r.next_u32();
                c.bus.write32(this + 0x2a0, current);
                for j in 0..2u32 {
                    let d = defs + 0x54 * j;
                    fill(c, &mut r, d, 0x15);
                    if (seed >> j) % 3 == 0 {
                        c.bus.write32(d, current);
                    }
                }
                oc428_force(c, 0x003a_9800, [0u32, 1, 2, 2][(seed % 4) as usize]);
                oc428_force(c, 0x003a_6c68, [0u32, 1, 2][((seed / 4) % 3) as usize]);
                oc428_force(c, 0x003a_6c98, defs);
                oc428_force(
                    c,
                    0x0030_9b70,
                    if (seed / 12) % 2 == 0 {
                        DATA + 0x9800
                    } else {
                        0
                    },
                );
                oc428_force(
                    c,
                    0x0022_4da0,
                    if (seed / 24) % 3 == 0 {
                        0
                    } else {
                        DATA + 0x9900
                    },
                );
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// The race-setup interfaces the AI creators use: `BELibrary_GetInterface`
    /// (fake) returns these two objects, each with an `update` method (a
    /// vtable slot at `+8`, delta and a fake target).
    fn oc428_interfaces(c: &mut Cpu, r: &mut Rng) -> (u32, u32) {
        let (rm, iface, vt) = (DATA + 0x5_1000, DATA + 0x5_1100, DATA + 0x5_1200);
        c.bus.write32(OC428_AUX + 0x10, rm);
        c.bus.write32(OC428_AUX + 0x14, iface);
        for o in [rm, iface] {
            c.bus.write32(o + 0xc, vt);
        }
        c.bus.write16(vt + 8, (r.next_u32() & 0xfc) as u16);
        c.bus.write32(vt + 0xc, FAKE + 0x10);
        (rm, iface)
    }

    /// `AI_CreateComputerRiders` against the generated body: 0-3 computer
    /// riders to create (the `ComputerActorRider` constructor sometimes
    /// failing: forced to 0), starting at index 0-3; the interface and the
    /// constructors are fakes, `Rider_CreateRiderMetrix` is real (its own
    /// allocator and `RiderMetrix_Reset` are fakes).
    #[test]
    fn ai_create_computer_riders_matches() {
        oc428_install_fakes();
        fakes::covered(
            ee::s00::f_00129e20__generated,
            ai_create_computer_riders,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x129e20, 12);
                oc428_gate(c);
                let this = DATA + 0x5_0000;
                fill(c, &mut r, this, 0x40);
                oc428_interfaces(c, &mut r);
                let done = r.next_u32() % 3;
                let first = r.next_u32() % 4;
                let want = [0u32, 1, 2, 3][(seed % 4) as usize];
                c.bus.write32(this + 0x78, first);
                c.bus.write32(this + 0x80, done);
                c.bus.write32(0x0053_5c04, done + want);
                if seed % 5 == 3 {
                    oc428_force(c, 0x0010_8c80, 0);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `AI_CreatePlayers` against the generated body: 0-2 players, local
    /// and network games (host or client; two players), controller and
    /// pad-cache ids 0-1. The interfaces, the rider/player base
    /// constructors and the allocator are fakes; `Player_ctor`,
    /// `Rider_CreateRiderMetrix`, `AI_ApplyPlayerRiderSettings`,
    /// `Rider_Load` and the `InputSync_Add*Pad` calls run for real (the load
    /// step fake marks the rider loaded).
    #[test]
    fn ai_create_players_matches() {
        oc428_install_fakes();
        fakes::covered(ee::s00::f_001298e0__generated, ai_create_players, 96, &|c, seed| {
            let mut r = machine(c, seed, 0x1298e0, 24);
            oc428_gate(c);
            let this = DATA + 0x5_0000;
            let (app, game, sync, pad) = (
                DATA + 0x5_2000,
                DATA + 0x5_2200,
                DATA + 0x5_8000,
                DATA + 0x5_2800,
            );
            fill(c, &mut r, this, 0x40);
            fill(c, &mut r, app, 0x80);
            fill(c, &mut r, pad + 0x2ee0, 0x20);
            oc428_interfaces(c, &mut r);
            let network = (seed / 4) % 2 == 1;
            let host = (seed / 8) % 2 == 1;
            let players = if network { 2 } else { (seed / 16) % 3 };
            let (ch, ctrl) = (seed % 2, (seed / 2) % 2);
            c.bus.write32(0x0053_4b30, network as u32);
            c.bus.write32(0x0053_4b38, host as u32);
            c.bus.write32(0x0053_5bfc, players);
            c.bus.write32(GAME_GP - 0x848, app);
            c.bus.write32(GAME_GP - 0x850, pad);
            c.bus.write32(app + 0x84, game);
            c.bus.write32(game + 0x7c, sync);
            c.bus.write32(sync + 4, 1 + r.next_u32() % 3);
            c.bus.write32(sync, r.next_u32() & 0xffff);
            c.bus.write32(sync + 0x10, r.next_u32() & 3);
            for k in 0..2u32 {
                c.bus
                    .write32(app + 0xb0 + 4 * k, DATA + 0x5_2400 + 0x100 * k);
                fill(c, &mut r, DATA + 0x5_2400 + 0x100 * k, 4);
            }
            oc428_force(c, 0x0014_7290, ch);
            oc428_force(c, 0x0014_7618, ctrl);
            c.g[A0][0] = sx(this);
        });
    }

    /// `ScriptEngine_ChooseScript` against the generated body: no filter
    /// table, an empty category, and categories of 1-4 scripts with random
    /// priorities (ties and improvements) and masks (single-bit masks
    /// against the stack's flag words; `ScriptEngine_EvalChoiceParams` runs for
    /// real and leaves through its `jump_out` on the empty jump table); the
    /// list allocation, `Rand_NextMain` and the free are fakes.
    #[test]
    fn script_engine_choose_script_matches() {
        oc428_install_fakes();
        fakes::covered(
            ee::s07::f_0027b0c0__generated,
            script_engine_choose_script,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27b0c0, 4);
                oc428_gate(c);
                c.pc = 0x0027_b0c0;
                let (engine, table, master, scripts) = (
                    DATA + 0x5_1000,
                    DATA + 0x5_2000,
                    DATA + 0x5_3000,
                    DATA + 0x5_4000,
                );
                fill(c, &mut r, engine + 0x520, 0x20);
                c.bus.write32(
                    engine + SCRIPT_ENGINE_FILTER_HEADER,
                    (seed % 11 != 0) as u32,
                );
                c.bus.write32(engine + SCRIPT_ENGINE_FILTER_TABLE, table);
                c.bus.write32(engine + SCRIPT_ENGINE_MASTER_TABLE, master);
                let category = seed % 3;
                let count = if seed % 7 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 4
                };
                c.bus.write32(table + 8 * category, count);
                c.bus.write32(table + 8 * category + 4, scripts);
                for k in 0..4u32 {
                    c.bus.write32(master + 0xc * k, r.next_u32() % 3);
                    let s = scripts + 0x30 * k;
                    c.bus.write32(s, r.next_u32() % 4);
                    for j in 0..21u32 {
                        let mask = if r.next_u32() % 6 == 0 {
                            1u16 << (r.next_u32() % 16)
                        } else {
                            0
                        };
                        c.bus.write16(s + 4 + 2 * j, mask);
                    }
                }
                c.g[A0][0] = sx(engine);
                c.g[A1][0] = category as u64;
            },
        );
    }

    use ssx3_ee_rt::funcs::{register_functions, FnEntry};

    /// `cGame_loadTrack` makes about 40 calls, most unported. Like
    /// `cGame_load`'s test it runs in a child process (this test binary, this
    /// one test, `--exact`) whose function table holds only the stub below:
    /// every other callee misses the table and lands in the logging fake. A
    /// table swap in the shared test process would race with the other tests.
    const OC428_CHILD_ENV: &str = "SSX3_OC428_LOADWORLD_CHILD";

    /// `SectionMan_IsEventLoaded`: 0 for the first `n` calls (`n` at
    /// `CTRL + 0x20`), then the fake's non-zero pointer.
    fn oc428_stub_event_loaded(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        let n = c.bus.read32(OC428_CTRL + 0x24);
        c.bus.write32(OC428_CTRL + 0x24, n + 1);
        fakes::fake(c, 0x0022_d278, ret);
        if c.pc == ret && n < c.bus.read32(OC428_CTRL + 0x20) {
            c.g[V0][0] = 0;
        }
    }

    static OC428_STUBS: [FnEntry; 1] = [FnEntry {
        start: 0x0022_d278,
        end: 0x0022_d27c,
        f: oc428_stub_event_loaded,
    }];

    /// The fixture for one seed: the game object (world, big file, section
    /// manager fields), the replay switch, the wait loop's length (0-2 extra
    /// rounds).
    fn oc428_load_world_setup(c: &mut Cpu, seed: u32) {
        if std::env::var_os(OC428_CHILD_ENV).is_some() {
            register_functions(&OC428_STUBS);
        }
        let mut r = machine(c, seed, 0x22f6b0, 44);
        c.pc = 0x0022_f6b0;
        let (this, world, world_data) = (DATA + 0x6_0000, DATA + 0x6_1000, DATA + 0x6_2000);
        fill(c, &mut r, this, 0x40);
        c.bus.write32(this + 0x10, world);
        c.bus.write32(world, world_data);
        c.bus.write32(GAME_GP - 0x1f38, (seed % 2) as u32);
        c.bus.write32(GAME_GP + 0xda0, r.next_u32());
        c.bus.write32(OC428_CTRL + 0x20, (seed / 2) % 3);
        c.g[A0][0] = sx(this);
    }

    fn oc428_load_world_checked() {
        fakes::covered(
            ee::s06::f_0022f6b0__generated,
            game_module_load_world,
            90,
            &oc428_load_world_setup,
        );
    }

    /// `cGame_loadTrack` against the generated body (strict: the arguments
    /// of all calls): replay and race location names, the section wait loop
    /// of 0-2 rounds, unwinds at random call indices.
    #[test]
    fn game_module_load_world_matches() {
        if std::env::var_os(OC428_CHILD_ENV).is_some() {
            return oc428_load_world_checked();
        }
        let name = format!(
            "{}::game_module_load_world_matches",
            module_path!().split_once("::").map_or("", |p| p.1)
        );
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([name.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(OC428_CHILD_ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child run of {name}:\n{text}"
        );
    }
}
