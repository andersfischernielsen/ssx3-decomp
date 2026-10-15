//! Work package A6: game state, lists, input and replay
//! (`docs/investigation/phase2-candidates.md`, structs in
//! `docs/investigation/structs/{input,app,world}.md`).
//!
//! Small accessors over global game state (the back end's race setup, the
//! world singleton, the path maps), the pad sample ring, the replay stream
//! and the intrusive lists the UI, snapshot and section code use.
//!
//! These are leaves with no event checks. Each replacement charges the EE
//! cycles block by block along the path it takes, exactly as the generated
//! code does, and leaves the registers a caller can observe (v0, v1, LO/HI
//! after a `mult`/`divu`) with the same values. Guest loads and stores go
//! through `Cpu::lw`/`Cpu::sw` like compiled code (sign-extended words, I/O
//! timing), and 64-bit register compares stay 64-bit.

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_a080, be_get_rider_character, "BE_GetRiderCharacter"),
    (0x0014_a0e0, be_get_rider_profile, "BE_GetRiderProfile"),
    (0x0014_7798, be_new_player_int_get_slot, "BENewPlayerInt_GetSlot"),
    (0x0014_4bc0, be_new_race_int_get_track_info, "BENewRaceInt_GetTrackInfo"),
    (0x002d_1bd8, world_get_instance, "World_GetInstance"),
    (0x0023_1278, game_module_request_overlay_state, "GameModule_RequestOverlayState"),
    (0x002c_c098, menu_sys_is_active, "MenuSys_IsActive"),
    (0x0032_6cc8, pad_queue_get_pad_data, "PadQueue_GetPadData"),
    (0x0032_6ca0, pad_queue_get_pad_status, "PadQueue_GetPadStatus"),
    (0x0032_6b48, pad_queue_advance_write, "PadQueue_AdvanceWrite"),
    (0x0026_f4a0, replay_set_state, "Replay_SetState"),
    (0x0026_e9c0, replay_stream_rewind_write, "ReplayStream_RewindWrite"),
    (0x0026_e5d0, snapshot_list_push_back, "SnapshotList_PushBack"),
    (0x0022_e348, section_man_next_queued, "SectionMan_NextQueued"),
    (0x0039_7840, ui_list_first, "UIList_First"),
    (0x0039_7718, ui_list_node_is_sentinel, "UIListNode_IsSentinel"),
    (0x0039_7740, ui_list_node_unlink, "UIListNode_Unlink"),
    (0x003d_0b18, path_queue_append, "PATH_QueueAppend"),
    (0x003d_5450, path_select_map, "PATH_SelectMap"),
    (0x0034_fc78, world_obj_link_instance, "WorldObj_LinkInstance"),
    (0x0034_fc80, world_obj_unlink_instance, "WorldObj_UnlinkInstance"),
];

// ---------------------------------------------------------------------------
// Back-end race setup (BENewPlayerInt / BENewRaceInt).

/// The back end's race setup globals.
pub mod race_setup {
    use crate::guest::Addr;

    /// `int slot[player]`: player index -> race slot.
    pub const PLAYER_SLOTS: Addr = Addr(0x0053_05b0);
    /// Per-slot rider records (BENewRaceInt), 0x1c bytes each.
    pub const RIDER_RECORDS: Addr = Addr(0x0053_5b20);
    pub const RIDER_RECORD_SIZE: u32 = 0x1c;
    /// Rider record: profile word (-1 = none).
    pub const REC_PROFILE_ID: u32 = 0x0c;
    /// Rider record: flags word (bit 0 = the profile's variant).
    pub const REC_FLAGS: u32 = 0x10;
    /// Rider record: character id (signed byte).
    pub const REC_CHARACTER: u32 = 0x11;
    /// `int` index of the current track.
    pub const TRACK_INDEX: Addr = Addr(0x0053_5c08);
    /// Track-info records, 100 bytes each.
    pub const TRACK_INFOS: Addr = Addr(0x0043_d950);
    pub const TRACK_INFO_SIZE: u32 = 100;

    /// Address of `PLAYER_SLOTS[player]` (32-bit address arithmetic).
    pub fn slot_entry(player: u32) -> Addr {
        Addr((player << 2).wrapping_add(PLAYER_SLOTS.0))
    }
}

/// Rider record of a player's slot: loads the slot, multiplies by the record
/// size (`mult`, so LO/HI change) and returns the record address.
fn rider_record(c: &mut Cpu, player: u32) -> Addr {
    use race_setup::*;
    let slot = slot_entry(player).lw(c) as u32;
    let off = abi::mult(c, slot, RIDER_RECORD_SIZE);
    Addr(off.wrapping_add(RIDER_RECORDS.0))
}

/// `int8_t BE_GetRiderCharacter(int player)` (0x14a080): the character id
/// of the rider in the player's race slot. 12 cycles.
pub fn be_get_rider_character(c: &mut Cpu) {
    c.cycles += 12;
    let rec = rider_record(c, abi::arg(c, 0));
    c.g[V1][0] = sx(rec.0);
    c.g[V0][0] = c.lb(rec.off(race_setup::REC_CHARACTER).0);
    abi::ret(c);
}

/// `int BE_GetRiderProfile(int player)` (0x14a0e0): 2 if the player's
/// rider has no profile (record word +0xc is -1), else bit 0 of the record
/// flags. 14 cycles, then 2 (no profile) or 3.
pub fn be_get_rider_profile(c: &mut Cpu) {
    use race_setup::*;
    c.cycles += 14;
    let rec = rider_record(c, abi::arg(c, 0));
    let profile = rec.off(REC_PROFILE_ID).lw(c);
    c.g[V1][0] = profile;
    if profile == u64::MAX {
        c.cycles += 2;
        c.g[V0][0] = 2;
    } else {
        c.cycles += 3;
        c.g[V0][0] = rec.off(REC_FLAGS).lw(c) & 1;
    }
    abi::ret(c);
}

/// `int BENewPlayerInt_GetSlot(int player)` (0x147798): the player's race
/// slot. 6 cycles.
pub fn be_new_player_int_get_slot(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = race_setup::slot_entry(abi::arg(c, 0)).lw(c);
    abi::ret(c);
}

/// `TrackInfo* BENewRaceInt_GetTrackInfo(void)` (0x144bc0): the info
/// record of the current track. 8 cycles.
pub fn be_new_race_int_get_track_info(c: &mut Cpu) {
    use race_setup::*;
    c.cycles += 8;
    let index = TRACK_INDEX.lw(c) as u32;
    let off = abi::mult(c, index, TRACK_INFO_SIZE);
    c.g[V1][0] = sx(off);
    abi::set_ret(c, off.wrapping_add(TRACK_INFOS.0));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World and game module.

/// `World* World_GetInstance(void)` (0x2d1bd8): `g_World` (gp+0x16c8).
/// 2 cycles.
pub fn world_get_instance(c: &mut Cpu) {
    c.cycles += 2;
    let gp = Addr(abi::reg(c, GP) as u32);
    c.g[V0][0] = gp.off(0x16c8).lw(c);
    abi::ret(c);
}

/// `void GameModule_RequestOverlayState(GameModule* m, int id)` (0x231278):
/// sets `m->pendingOverlayId` (+0x208). 2 cycles.
pub fn game_module_request_overlay_state(c: &mut Cpu) {
    c.cycles += 2;
    let module = Addr(abi::arg(c, 0));
    module.off(0x208).sw(c, abi::reg(c, A1));
    abi::ret(c);
}

/// `bool MenuSys_IsActive(MenuSys* m)` (0x2cc098): false if the first word
/// is 0, else whether the word at +0x54 is non-zero (an unsigned `0 < x`
/// on the sign-extended value). 3, +2 when the first word is set, +2.
pub fn menu_sys_is_active(c: &mut Cpu) {
    c.cycles += 3;
    let menu = Addr(abi::arg(c, 0));
    let mut active = 0;
    if menu.lw(c) != 0 {
        c.cycles += 2;
        active = (menu.off(0x54).lw(c) != 0) as u64;
    }
    c.cycles += 2;
    c.g[V1][0] = active;
    c.g[V0][0] = active;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Pad sample ring (input.md `PadQueue`).

/// The 30-frame ring of raw pad samples (`PadQueue`, 0x2efc bytes).
#[derive(Clone, Copy, Debug)]
pub struct PadQueue(pub Addr);

impl PadQueue {
    pub const FRAMES: u32 = 30;
    /// One ring frame: two pads.
    pub const FRAME_SIZE: u32 = 0x190;
    /// One pad's sample: status word + 24 channel words.
    pub const PAD_SIZE: u32 = 0x64;
    /// Consumer index (advanced by `AdvanceWrite`).
    pub fn read_index(self) -> Addr {
        self.0.off(0x2ee0)
    }
    /// Producer index (written by `Poll`).
    pub fn write_index(self) -> Addr {
        self.0.off(0x2ee4)
    }
}

/// `float* PadQueue_GetPadData(PadQueue* q, int pad)` (0x326cc8): the pad's
/// channel data in the current read frame (`&ring[read].pad[pad].ch[0]`).
/// 9 cycles.
pub fn pad_queue_get_pad_data(c: &mut Cpu) {
    c.cycles += 9;
    let q = PadQueue(Addr(abi::arg(c, 0)));
    let read = q.read_index().lw(c) as u32;
    let pad_off = abi::mult(c, abi::arg(c, 1), PadQueue::PAD_SIZE);
    let frame_off = abi::mult(c, read, PadQueue::FRAME_SIZE);
    c.g[V1][0] = sx(frame_off);
    let p = frame_off.wrapping_add(q.0 .0).wrapping_add(pad_off).wrapping_add(4);
    abi::set_ret(c, p);
    abi::ret(c);
}

/// `int PadQueue_GetPadStatus(PadQueue* q, int pad)` (0x326ca0): the pad's
/// status word in the current read frame. 9 cycles.
pub fn pad_queue_get_pad_status(c: &mut Cpu) {
    c.cycles += 9;
    let q = PadQueue(Addr(abi::arg(c, 0)));
    let read = q.read_index().lw(c) as u32;
    let pad_off = abi::mult(c, abi::arg(c, 1), PadQueue::PAD_SIZE);
    let frame_off = abi::mult(c, read, PadQueue::FRAME_SIZE);
    c.g[V1][0] = PadQueue::FRAME_SIZE as u64;
    let status = Addr(q.0 .0.wrapping_add(pad_off.wrapping_add(frame_off)));
    c.g[V0][0] = status.lw(c);
    abi::ret(c);
}

/// `bool PadQueue_AdvanceWrite(PadQueue* q)` (0x326b48): moves the read
/// index to the next frame (mod 30, unsigned) unless that would reach the
/// write index; returns whether it moved. 5 + 6, then 2 (full) or 3.
pub fn pad_queue_advance_write(c: &mut Cpu) {
    c.cycles += 5;
    let q = PadQueue(Addr(abi::arg(c, 0)));
    c.g[V1][0] = PadQueue::FRAMES as u64;
    let read = q.read_index().lw(c) as u32;
    c.cycles += 6;
    let write = q.write_index().lw(c);
    let next = abi::divu(c, read.wrapping_add(1), PadQueue::FRAMES).1;
    if next as u64 == write {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 3;
        q.read_index().sw(c, next as u64);
        c.g[V0][0] = 1;
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Replay.

/// `void Replay_SetState(Replay* r, int state)` (0x26f4a0): the state word
/// at +0. 2 cycles.
pub fn replay_set_state(c: &mut Cpu) {
    c.cycles += 2;
    Addr(abi::arg(c, 0)).sw(c, abi::reg(c, A1));
    abi::ret(c);
}

/// `void ReplayStream_RewindWrite(ReplayStream* s)` (0x26e9c0): byte count
/// (+0x10) = 0, write cursor (+0x8) = buffer start (+0x4). 4 cycles.
pub fn replay_stream_rewind_write(c: &mut Cpu) {
    c.cycles += 4;
    let s = Addr(abi::arg(c, 0));
    let start = s.off(0x4).lw(c);
    c.g[V0][0] = start;
    s.off(0x10).sw(c, 0);
    s.off(0x8).sw(c, start);
    abi::ret(c);
}

/// Replay snapshot list: `{head +0, tail +4, count +8}`, nodes linked by
/// `next +0x14`, `prev +0x18`.
#[derive(Clone, Copy, Debug)]
pub struct SnapshotList(pub Addr);

impl SnapshotList {
    pub fn head(self) -> Addr {
        self.0
    }
    pub fn tail(self) -> Addr {
        self.0.off(0x4)
    }
    pub fn count(self) -> Addr {
        self.0.off(0x8)
    }
    pub const NODE_NEXT: u32 = 0x14;
    pub const NODE_PREV: u32 = 0x18;
}

/// `void SnapshotList_PushBack(SnapshotList* l, Snapshot* n)` (0x26e5d0):
/// appends `n`. 6, then 3 (+1 when the list was empty), then 4.
pub fn snapshot_list_push_back(c: &mut Cpu) {
    let list = SnapshotList(Addr(abi::arg(c, 0)));
    let node = Addr(abi::arg(c, 1));
    c.cycles += 6;
    let tail = list.tail().lw(c);
    node.off(SnapshotList::NODE_NEXT).sw(c, 0);
    node.off(SnapshotList::NODE_PREV).sw(c, tail);
    // The tail is read again after the node stores (they may alias it).
    let tail = list.tail().lw(c);
    if tail != 0 {
        Addr(tail as u32).off(SnapshotList::NODE_NEXT).sw(c, node.0 as u64);
    }
    c.cycles += 3;
    let head = list.head().lw(c);
    list.tail().sw(c, node.0 as u64);
    if head == 0 {
        c.cycles += 1;
        list.head().sw(c, node.0 as u64);
    }
    c.cycles += 4;
    let count = sx((list.count().lw(c) as u32).wrapping_add(1));
    c.g[V0][0] = count;
    list.count().sw(c, count);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Section manager (world.md `SectionMan`).

/// `Node* SectionMan_NextQueued(SectionMan* m, Node* cur)` (0x22e348): the
/// queued-section list head (+0x19c) when `cur` is null, else `cur->next`
/// (+4). 2 + 2 cycles.
pub fn section_man_next_queued(c: &mut Cpu) {
    c.cycles += 2;
    let cur = abi::reg(c, A1);
    c.cycles += 2;
    c.g[V0][0] = if cur != 0 {
        Addr(cur as u32).off(0x4).lw(c)
    } else {
        Addr(abi::arg(c, 0)).off(0x19c).lw(c)
    };
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI intrusive lists: nodes are `{link0 +0, link1 +4}`; a node whose link
// points at itself is a sentinel/unlinked node.

/// `UINode* UIList_First(UIList* l)` (0x397840): the anchor node at +4, or
/// null if its +4 link points back at itself (empty). 5 cycles.
pub fn ui_list_first(c: &mut Cpu) {
    c.cycles += 5;
    let node = Addr(abi::arg(c, 0)).off(0x4).lw(c);
    let diff = Addr(node as u32).off(0x4).lw(c) ^ node;
    c.g[V1][0] = diff;
    c.g[V0][0] = if diff == 0 { 0 } else { node };
    abi::ret(c);
}

/// `bool UIListNode_IsSentinel(UINode* n)` (0x397718): whether either link
/// (+4 first, then +0) points at the node itself. 3 + (1 | 3 | 3 + 1) + 2.
pub fn ui_list_node_is_sentinel(c: &mut Cpu) {
    c.cycles += 3;
    let me = abi::reg(c, A0);
    let n = Addr(me as u32);
    let link1 = n.off(0x4).lw(c);
    let sentinel = if link1 == me {
        c.cycles += 1;
        true
    } else {
        c.cycles += 3;
        let link0 = n.lw(c);
        if link0 == me {
            c.cycles += 1;
        }
        link0 == me
    };
    c.cycles += 2;
    c.g[V1][0] = sentinel as u64;
    c.g[V0][0] = sentinel as u64;
    abi::ret(c);
}

/// `void UIListNode_Unlink(UINode* n)` (0x397740): splices `n` out between
/// its neighbours (`a = n->link0`, `b = n->link1`), tolerating null and
/// self links, and leaves `n` self-linked.
pub fn ui_list_node_unlink(c: &mut Cpu) {
    c.cycles += 3;
    let me = abi::reg(c, A0);
    let n = Addr(me as u32);
    let a = n.lw(c);
    let b = n.off(0x4).lw(c);
    c.g[V0][0] = a;
    c.g[V1][0] = b;
    // a->link1 = b (or a itself when b is n).
    if a != 0 {
        c.cycles += 2;
        if a != me {
            c.cycles += 2;
            let to = if b != me {
                b
            } else {
                c.cycles += 1;
                a
            };
            Addr(a as u32).off(0x4).sw(c, to);
        }
    }
    // b->link0 = a (or b itself when a is n); n->link0 = n.
    c.cycles += 2;
    if b != 0 {
        c.cycles += 2;
        if b != me {
            c.cycles += 2;
            let to = if a != me {
                a
            } else {
                c.cycles += 1;
                b
            };
            Addr(b as u32).sw(c, to);
            c.cycles += 1;
        }
    }
    n.sw(c, me);
    c.cycles += 2;
    n.off(0x4).sw(c, me);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Paths.

/// `void PATH_QueueAppend(PathQueue* q, const Entry* e)` (0x3d0b18):
/// appends a 16-byte entry to the 64-slot ring at +0x10; the write index
/// is bits 16..21 of the header word at +0xc. The header is rewritten
/// first, then the entry is copied as two unaligned doublewords (both read
/// before either is stored). 23 cycles.
pub fn path_queue_append(c: &mut Cpu) {
    c.cycles += 23;
    let q = Addr(abi::arg(c, 0));
    let src = abi::arg(c, 1);
    let header = q.off(0xc).lw(c) as u32;
    let index = (header >> 16) & 0x3f;
    let next = (index + 1) & 0x3f;
    q.off(0xc).sw(c, ((header & 0xffc0_ffff) | (next << 16)) as u64);
    let slot = (index << 4).wrapping_add(q.off(0x10).0);
    let lo = Addr(src).read_u64_unaligned(c);
    let hi = Addr(src).off(8).read_u64_unaligned(c);
    Addr(slot).write_u64_unaligned(c, lo);
    Addr(slot).off(8).write_u64_unaligned(c, hi);
    c.g[V1][0] = lo;
    c.g[V0][0] = sx(slot);
    abi::ret(c);
}

/// Path-map globals.
pub mod path_maps {
    use crate::guest::Addr;
    /// `PathMap*` of the selected map.
    pub const CURRENT_MAP: Addr = Addr(0x0044_f420);
    /// `int` id of the selected map (0 = unset, -1 = none).
    pub const CURRENT_ID: Addr = Addr(0x0044_f430);
    /// The four path maps, 0x928 bytes each.
    pub const MAPS: Addr = Addr(0x0051_7610);
    pub const MAP_SIZE: u32 = 0x928;
    pub const MAP_ID: u32 = 0x30;
    pub const MAP_LOADED: u32 = 0x34;
}

/// `int PATH_SelectMap(uint idx)` (0x3d5450): selects path map `idx` (0..3)
/// if it is loaded and returns its id; otherwise clears the selection
/// (unless `idx` is -1, a query) and returns the current id. An unset id
/// (0) becomes -1 first.
pub fn path_select_map(c: &mut Cpu) {
    use path_maps::*;
    c.cycles += 4;
    c.g[V1][0] = 0x45_0000;
    if CURRENT_ID.lw(c) == 0 {
        c.cycles += 3;
        CURRENT_ID.sw(c, u64::MAX);
    }
    c.cycles += 3;
    let idx = abi::reg(c, A0);
    if idx < 4 {
        c.cycles += 7;
        let off = abi::mult(c, idx as u32, MAP_SIZE);
        let map = Addr(MAPS.0.wrapping_add(off));
        let loaded = map.off(MAP_LOADED).lw(c);
        c.g[V1][0] = loaded;
        if loaded != 0 {
            c.cycles += 7;
            let id = map.off(MAP_ID).lw(c);
            c.g[V1][0] = 0x45_0000;
            CURRENT_MAP.sw(c, map.0 as u64);
            CURRENT_ID.sw(c, id);
            c.g[V0][0] = id;
            abi::ret(c);
            return;
        }
    } else {
        c.cycles += 1;
    }
    c.cycles += 2;
    if idx != u64::MAX {
        CURRENT_ID.sw(c, 0);
    }
    c.cycles += 2;
    c.g[V0][0] = CURRENT_ID.lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World objects.

/// `void WorldObj_LinkInstance(Instance* inst, WorldObj* obj)` (0x34fc78):
/// `obj->instance (+0xc) = inst`. 2 cycles.
pub fn world_obj_link_instance(c: &mut Cpu) {
    c.cycles += 2;
    let obj = Addr(abi::arg(c, 1));
    obj.off(0xc).sw(c, abi::reg(c, A0));
    abi::ret(c);
}

/// `void WorldObj_UnlinkInstance(Instance* inst, WorldObj* obj)` (0x34fc80):
/// clears `obj->instance` if `obj` is non-null and links `inst`.
/// 2, +3 when obj is non-null, +2.
pub fn world_obj_unlink_instance(c: &mut Cpu) {
    c.cycles += 2;
    let obj = abi::reg(c, A1);
    if obj != 0 {
        c.cycles += 3;
        let field = Addr(obj as u32).off(0xc);
        let inst = field.lw(c);
        c.g[V0][0] = inst;
        if inst == abi::reg(c, A0) {
            field.sw(c, 0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, Rng, DATA};

    const SEEDS: u32 = 400;

    fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 20) ^ salt)
    }

    /// Fill `n` words at `a` with random values.
    fn scribble(c: &mut Cpu, r: &mut Rng, a: u32, n: u32) {
        for i in 0..n {
            c.bus.write32(a + i * 4, r.next_u32());
        }
    }

    /// Random junk in the registers a caller would hand over (v0/v1 too, so
    /// a missing write shows up).
    fn junk_regs(c: &mut Cpu, r: &mut Rng) {
        for reg in [V0, V1, A2, A3, T0, T1] {
            c.g[reg][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        }
        c.lo[0] = r.next_u32() as u64;
        c.hi[0] = r.next_u32() as u64;
    }

    /// A pointer argument; some seeds put junk in the upper 32 bits so the
    /// 64-bit compares are exercised.
    fn ptr(r: &mut Rng, a: u32) -> u64 {
        if r.next_u32() % 5 == 0 {
            (r.next_u32() as u64) << 32 | a as u64
        } else {
            sx(a)
        }
    }


    /// Race setup: random slot table and records (slots 0..15, some
    /// profile words -1).
    fn race_setup(c: &mut Cpu, seed: u32) {
        let mut r = rng(seed, 1);
        junk_regs(c, &mut r);
        for p in 0..8 {
            c.bus.write32(0x0053_05b0 + p * 4, r.next_u32() % 16);
        }
        scribble(c, &mut r, 0x0053_5b20, 16 * 0x1c / 4);
        for s in 0..16 {
            if r.next_u32() % 2 == 0 {
                c.bus.write32(0x0053_5b20 + s * 0x1c + 0xc, u32::MAX);
            }
        }
        c.bus.write32(0x0053_5c08, r.next_u32() % 40);
        c.g[A0][0] = (r.next_u32() % 8) as u64;
    }

    #[test]
    fn be_get_rider_character_matches() {
        check_lohi(ssx3_ee::s01::f_0014a080__generated, be_get_rider_character, SEEDS, &race_setup);
    }

    #[test]
    fn be_get_rider_profile_matches() {
        check_lohi(ssx3_ee::s01::f_0014a0e0__generated, be_get_rider_profile, SEEDS, &race_setup);
    }

    #[test]
    fn be_new_player_int_get_slot_matches() {
        check(ssx3_ee::s01::f_00147798__generated, be_new_player_int_get_slot, SEEDS, &race_setup);
    }

    #[test]
    fn be_new_race_int_get_track_info_matches() {
        check_lohi(ssx3_ee::s01::f_00144bc0__generated, be_new_race_int_get_track_info, SEEDS, &|c, seed| {
            race_setup(c, seed);
            // Any index, including negative ones (32-bit multiply).
            let mut r = rng(seed, 2);
            c.bus.write32(0x0053_5c08, r.next_u32());
        });
    }

    #[test]
    fn world_get_instance_matches() {
        check(ssx3_ee::s09::f_002d1bd8__generated, world_get_instance, SEEDS, &|c, seed| {
            let mut r = rng(seed, 3);
            junk_regs(c, &mut r);
            let gp = if seed % 2 == 0 { crate::difftest::GAME_GP } else { DATA + (r.next_u32() & 0xfffc) };
            c.g[GP][0] = sx(gp);
            c.bus.write32(gp + 0x16c8, r.next_u32());
        });
    }

    #[test]
    fn game_module_request_overlay_state_matches() {
        check(ssx3_ee::s06::f_00231278__generated, game_module_request_overlay_state, SEEDS, &|c, seed| {
            let mut r = rng(seed, 4);
            junk_regs(c, &mut r);
            c.g[A0][0] = { let a = DATA + (r.next_u32() & 0xffc); ptr(&mut r, a) };
            c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        });
    }

    #[test]
    fn menu_sys_is_active_matches() {
        check(ssx3_ee::s09::f_002cc098__generated, menu_sys_is_active, SEEDS, &|c, seed| {
            let mut r = rng(seed, 5);
            junk_regs(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let first = if seed % 3 == 0 { 0 } else { r.next_u32() };
            c.bus.write32(DATA, first);
            let v = match seed % 4 {
                0 => 0,
                1 => 0x8000_0000 | r.next_u32(),
                _ => r.next_u32() >> 1,
            };
            c.bus.write32(DATA + 0x54, v);
        });
    }

    /// A PadQueue at DATA with random ring contents and indices.
    fn pad_setup(c: &mut Cpu, seed: u32) {
        let mut r = rng(seed, 6);
        junk_regs(c, &mut r);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = (r.next_u32() % 2) as u64;
        scribble(c, &mut r, DATA, 0x2ee0 / 4);
        let read = match seed % 4 {
            0 => r.next_u32(),
            1 => 29,
            _ => r.next_u32() % 30,
        };
        let write = if seed % 3 == 0 { (read.wrapping_add(1)) % 30 } else { r.next_u32() % 32 };
        c.bus.write32(DATA + 0x2ee0, read);
        c.bus.write32(DATA + 0x2ee4, write);
    }

    #[test]
    fn pad_queue_get_pad_data_matches() {
        check_lohi(ssx3_ee::s10::f_00326cc8__generated, pad_queue_get_pad_data, SEEDS, &pad_setup);
    }

    #[test]
    fn pad_queue_get_pad_status_matches() {
        check_lohi(ssx3_ee::s10::f_00326ca0__generated, pad_queue_get_pad_status, SEEDS, &|c, seed| {
            pad_setup(c, seed);
            // Keep the status address inside the ring.
            let read = c.bus.read32(DATA + 0x2ee0) % 30;
            c.bus.write32(DATA + 0x2ee0, read);
        });
    }

    #[test]
    fn pad_queue_advance_write_matches() {
        check_lohi(ssx3_ee::s10::f_00326b48__generated, pad_queue_advance_write, SEEDS, &pad_setup);
    }

    #[test]
    fn replay_set_state_matches() {
        check(ssx3_ee::s07::f_0026f4a0__generated, replay_set_state, SEEDS, &|c, seed| {
            let mut r = rng(seed, 7);
            junk_regs(c, &mut r);
            c.g[A0][0] = { let a = DATA + (r.next_u32() & 0xffc); ptr(&mut r, a) };
            c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        });
    }

    #[test]
    fn replay_stream_rewind_write_matches() {
        check(ssx3_ee::s07::f_0026e9c0__generated, replay_stream_rewind_write, SEEDS, &|c, seed| {
            let mut r = rng(seed, 8);
            junk_regs(c, &mut r);
            c.g[A0][0] = ptr(&mut r, DATA);
            scribble(c, &mut r, DATA, 8);
        });
    }

    #[test]
    fn snapshot_list_push_back_matches() {
        check(ssx3_ee::s07::f_0026e5d0__generated, snapshot_list_push_back, SEEDS, &|c, seed| {
            let mut r = rng(seed, 9);
            junk_regs(c, &mut r);
            let (list, node) = (DATA, DATA + 0x40);
            scribble(c, &mut r, DATA, 0x100 / 4);
            let tail = [0, DATA + 0x80, node, list][(seed % 4) as usize];
            let head = [0, DATA + 0xc0][(seed / 4 % 2) as usize];
            c.bus.write32(list, head);
            c.bus.write32(list + 4, tail);
            c.g[A0][0] = ptr(&mut r, list);
            c.g[A1][0] = sx(node);
        });
    }

    #[test]
    fn section_man_next_queued_matches() {
        check(ssx3_ee::s05::f_0022e348__generated, section_man_next_queued, SEEDS, &|c, seed| {
            let mut r = rng(seed, 10);
            junk_regs(c, &mut r);
            scribble(c, &mut r, DATA, 0x200 / 4);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = match seed % 3 {
                0 => 0,
                1 => sx(DATA + 0x1a0),
                _ => 1 << 40, // non-null on 64 bits, address 0
            };
        });
    }

    /// A UI list node at DATA+0x100 whose links are each one of: null, the
    /// node itself, or another node; the list anchor at DATA.
    fn ui_setup(c: &mut Cpu, seed: u32) {
        let mut r = rng(seed, 11);
        junk_regs(c, &mut r);
        scribble(c, &mut r, DATA, 0x400 / 4);
        let n = DATA + 0x100;
        let pick = |k: u32| [0, n, DATA + 0x200, DATA + 0x300][k as usize % 4];
        c.bus.write32(n, pick(seed));
        c.bus.write32(n + 4, pick(seed / 4));
        c.bus.write32(DATA + 4, if seed % 2 == 0 { n } else { DATA + 0x200 });
        // Empty list: anchor's +4 points back at the anchor node.
        if seed % 5 == 0 {
            c.bus.write32(DATA + 0x204, DATA + 0x200);
        }
        c.g[A0][0] = ptr(&mut r, n);
    }

    #[test]
    fn ui_list_first_matches() {
        check(ssx3_ee::s13::f_00397840__generated, ui_list_first, SEEDS, &|c, seed| {
            ui_setup(c, seed);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn ui_list_node_is_sentinel_matches() {
        check(ssx3_ee::s13::f_00397718__generated, ui_list_node_is_sentinel, SEEDS, &ui_setup);
    }

    #[test]
    fn ui_list_node_unlink_matches() {
        check(ssx3_ee::s13::f_00397740__generated, ui_list_node_unlink, SEEDS, &ui_setup);
    }

    #[test]
    fn path_queue_append_matches() {
        check(ssx3_ee::s14::f_003d0b18__generated, path_queue_append, SEEDS, &|c, seed| {
            let mut r = rng(seed, 12);
            junk_regs(c, &mut r);
            let q = DATA + (seed % 4) * 4;
            scribble(c, &mut r, DATA, 0x500 / 4);
            scribble(c, &mut r, DATA + 0x800, 0x40 / 4);
            // Source: unaligned, or overlapping the ring.
            let src = if seed % 7 == 0 {
                q + 0x10 + (r.next_u32() % 0x400)
            } else {
                DATA + 0x800 + r.next_u32() % 16
            };
            c.g[A0][0] = sx(q);
            c.g[A1][0] = sx(src);
        });
    }

    #[test]
    fn path_select_map_matches() {
        check_lohi(ssx3_ee::s14::f_003d5450__generated, path_select_map, SEEDS, &|c, seed| {
            let mut r = rng(seed, 13);
            junk_regs(c, &mut r);
            for m in 0..4 {
                let map = 0x0051_7610 + m * 0x928;
                c.bus.write32(map + 0x30, r.next_u32());
                c.bus.write32(map + 0x34, if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() });
            }
            c.bus.write32(0x0044_f420, r.next_u32());
            c.bus.write32(0x0044_f430, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.g[A0][0] = match seed % 6 {
                0 => u64::MAX,
                1 => 0xffff_ffff, // -1 on 32 bits only
                2 => 4 + r.next_u32() as u64 % 100,
                _ => (r.next_u32() % 4) as u64,
            };
        });
    }

    #[test]
    fn world_obj_link_instance_matches() {
        check(ssx3_ee::s11::f_0034fc78__generated, world_obj_link_instance, SEEDS, &|c, seed| {
            let mut r = rng(seed, 14);
            junk_regs(c, &mut r);
            c.g[A0][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.g[A1][0] = { let a = DATA + (r.next_u32() & 0xffc); ptr(&mut r, a) };
        });
    }

    #[test]
    fn world_obj_unlink_instance_matches() {
        check(ssx3_ee::s11::f_0034fc80__generated, world_obj_unlink_instance, SEEDS, &|c, seed| {
            let mut r = rng(seed, 15);
            junk_regs(c, &mut r);
            let inst = DATA + 0x400;
            c.g[A0][0] = ptr(&mut r, inst);
            c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA) };
            c.bus.write32(DATA + 0xc, if seed % 2 == 0 { inst } else { r.next_u32() });
        });
    }
}
