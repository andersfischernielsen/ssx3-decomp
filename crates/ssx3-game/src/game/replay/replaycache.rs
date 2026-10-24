//! `src/replay/replaycache.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0026_d628,
        world_state_stream_reset_2,
        "WorldStateStream_Reset",
    ),
    (
        0x0027_05a8,
        replay_empty_rider_hook,
        "Replay_EmptyRiderHook",
    ),
    (0x0027_22e0, in_seq_heap_init, "InSeq_HeapInit"),
    (
        0x0027_42c8,
        in_seq_channel_get_track,
        "InSeq_ChannelGetTrack",
    ),
    (
        0x0027_4348,
        in_seq_channel_get_length,
        "InSeq_ChannelGetLength",
    ),
    (0x0027_43c8, in_seq_channel_get_id, "InSeq_ChannelGetId"),
];

/// `void WorldStateStream_Reset(WorldStateStream *stream)` (0x26d628):
/// reinitialise the stream fields (a scratch 32-bit zero on the stack is used
/// to store the first list word unaligned).
pub fn world_state_stream_reset(c: &mut Cpu) {
    c.cycles += 20;
    let stream = Addr(abi::arg(c, 0));
    let sp = Addr((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[SP][0] = sx(sp.0);
    c.g[V0][0] = u64::MAX; // -1
    sp.sh(c, 0);
    sp.off(2).sb(c, 0);
    sp.off(3).sb(c, 0);
    stream.off(0x30).sw(c, 0);
    stream.off(0x34).sw(c, c.g[V0][0]);
    c.lwl(V1, sp.off(3).0);
    c.lwr(V1, sp.0);
    c.swl(V1, stream.off(0x1f).0);
    c.swr(V1, stream.off(0x1c).0);
    stream.off(0x2c).sw(c, c.g[V0][0]);
    stream.off(0x10).sw(c, 0);
    stream.off(0x20).sw(c, c.g[V0][0]);
    stream.off(0x24).sw(c, c.g[V0][0]);
    stream.off(0x28).sw(c, c.g[V0][0]);
    stream.off(0x8).sw(c, 0);
    stream.off(0xc).sw(c, 0);
    c.g[SP][0] = sx(sp.0.wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Instrument-sequence heap.

/// `void* InSeq_HeapInit(Heap* heap, void* data, uint size)` (0x2722e0):
/// initialise a heap node (head at +0x0, tail +0x8, size +0x4), terminating the
/// block with a free marker at `data + 4` and the end offset.
pub fn in_seq_heap_init(c: &mut Cpu) {
    const TAIL: u32 = 0x8;
    const SIZE: u32 = 0x4;
    const BLOCK_NEXT: u32 = 0x4;
    c.cycles += 9;
    let heap = Addr(abi::arg(c, 0));
    let data = c.g[A1][0];
    heap.off(TAIL).sw(c, data);
    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x8));
    heap.sw(c, data);
    c.g[V0][0] = c.g[A0][0];
    heap.off(SIZE).sw(c, c.g[A2][0]);
    Addr(data as u32).off(BLOCK_NEXT).sw(c, 0);
    c.g[V1][0] = heap.off(TAIL).lw(c);
    Addr(c.g[V1][0] as u32).sw(c, c.g[A3][0]);
    abi::ret(c);
}

/// `void* InSeq_ChannelGetTrack(InSeq_Channel* ch, int index)` (0x2742c8): the
/// track pointer `index` in the table at `ch + 0x24`.
pub fn in_seq_channel_get_track(c: &mut Cpu) {
    const TABLE: u32 = 0x24;
    c.cycles += 5;
    let table = Addr(abi::arg(c, 0)).off(TABLE).lw(c);
    c.g[V0][0] = table;
    let offset = sx((c.g[A1][0] as u32).wrapping_shl(2));
    c.g[A1][0] = offset;
    let slot = sx((offset as u32).wrapping_add(table as u32));
    c.g[A1][0] = slot;
    c.g[V0][0] = Addr(slot as u32).lw(c);
    abi::ret(c);
}

/// `int InSeq_ChannelGetLength(InSeq_Channel* ch)` (0x274348): the length field
/// at `channel->track + 0x4`.
pub fn in_seq_channel_get_length(c: &mut Cpu) {
    const TRACK: u32 = 0x1c;
    c.cycles += 3;
    let track = Addr(abi::arg(c, 0)).off(TRACK).lw(c);
    c.g[V1][0] = track;
    c.g[V0][0] = Addr(track as u32).off(0x4).lw(c);
    abi::ret(c);
}

/// `int InSeq_ChannelGetId(InSeq_Channel* ch)` (0x2743c8): the id byte at
/// `channel->track + 0x3`.
pub fn in_seq_channel_get_id(c: &mut Cpu) {
    const TRACK: u32 = 0x1c;
    c.cycles += 3;
    let track = Addr(abi::arg(c, 0)).off(TRACK).lw(c);
    c.g[V1][0] = track;
    c.g[V0][0] = Addr(track as u32).off(0x3).lbu(c);
    abi::ret(c);
}

/// `WorldStateStream* WorldStateStream_Reset(WorldStateStream *self)`
/// (0x26d628): reset the stream's counters and the 12-byte field at +0x1c.
pub fn world_state_stream_reset_2(c: &mut Cpu) {
    c.cycles += 20;
    let stream = Addr(abi::arg(c, 0));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = u64::MAX;
    let sp = c.g[SP][0] as u32;
    Addr(sp).sh(c, 0);
    Addr(sp.wrapping_add(0x2)).sb(c, 0);
    Addr(sp.wrapping_add(0x3)).sb(c, 0);
    stream.off(0x30).sw(c, 0);
    stream.off(0x34).sw(c, u64::MAX);
    c.lwl(V1, sp.wrapping_add(0x3));
    c.lwr(V1, sp);
    c.swl(V1, stream.0.wrapping_add(0x1f));
    c.swr(V1, stream.0.wrapping_add(0x1c));
    stream.off(0x2c).sw(c, u64::MAX);
    stream.off(0x10).sw(c, 0);
    stream.off(0x20).sw(c, u64::MAX);
    stream.off(0x24).sw(c, u64::MAX);
    stream.off(0x28).sw(c, u64::MAX);
    stream.off(0x8).sw(c, 0);
    stream.off(0xc).sw(c, 0);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

/// `void Replay_EmptyRiderHook(Rider* r)` (0x2705a8): replay rider hook, not
/// installed.
pub fn replay_empty_rider_hook(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::lib::sif::tests::SEEDS;

    #[test]
    fn world_state_stream_reset_matches() {
        check(
            ssx3_ee::s07::f_0026d628,
            world_state_stream_reset,
            100,
            &|c, seed| {
                let mut r = Rng(0x0c13_000b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..16 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn in_seq_heap_init_matches() {
        check(
            ssx3_ee::s07::f_002722e0,
            in_seq_heap_init,
            100,
            &|c, seed| {
                let mut r = Rng(0x722e0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = if seed % 3 == 0 {
                    0xffff_fff8
                } else {
                    r.next_u32()
                } as u64;
            },
        );
    }

    #[test]
    fn in_seq_channel_get_track_matches() {
        check(
            ssx3_ee::s07::f_002742c8,
            in_seq_channel_get_track,
            100,
            &|c, seed| {
                let mut r = Rng(0x742c8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = seed % 4;
                c.g[A1][0] = index as u64;
                let table = DATA + 0x1000;
                c.bus.write32(DATA + 0x24, table);
                c.bus.write32(table + index * 4, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_channel_get_length_matches() {
        check(
            ssx3_ee::s07::f_00274348,
            in_seq_channel_get_length,
            100,
            &|c, seed| {
                let mut r = Rng(0x74348 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let track = DATA + 0x100;
                c.bus.write32(DATA + 0x1c, track);
                c.bus.write32(track + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_channel_get_id_matches() {
        check(
            ssx3_ee::s07::f_002743c8,
            in_seq_channel_get_id,
            100,
            &|c, seed| {
                let mut r = Rng(0x743c8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let track = DATA + 0x100;
                c.bus.write32(DATA + 0x1c, track);
                c.bus.write32(track, r.next_u32());
            },
        );
    }

    #[test]
    fn world_state_stream_reset_matches_2() {
        check(
            ssx3_ee::s07::f_0026d628,
            world_state_stream_reset_2,
            64,
            &|c, seed| {
                let mut r = Rng(0x26d6 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn replay_empty_rider_hook_matches() {
        leaf(
            ssx3_ee::s07::f_002705a8__generated,
            replay_empty_rider_hook,
            14,
        );
    }
}
