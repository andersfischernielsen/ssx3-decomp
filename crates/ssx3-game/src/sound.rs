//! Work package A5: sound, streaming and files
//! (`docs/investigation/phase2-candidates.md`, names from
//! `docs/investigation/naming.md` pass 3).
//!
//! The EA sound driver's bookkeeping (voice handles, intrusive lists, banks,
//! MIDI slots, streams, speech ids, the channel volume), the stream-file and
//! async-file validators, the BIG archive header check and the EA timer.
//!
//! All of these are leaves without event checks. Each replacement charges the
//! EE cycles block by block along the path it takes, exactly as the generated
//! code does, and leaves the registers a caller can observe (v0, v1, f0/f1,
//! LO/HI after a `mult`) with the same values. Lines marked "leftover"
//! reproduce incidental register contents (the upper half of a constant the
//! original built with `lui`, a value loaded for a test). Guest loads and
//! stores go through the `Cpu` paths like compiled code (sign-extended words,
//! I/O timing), and 64-bit register compares stay 64-bit.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_5440, snd_is_initialized, "SND_IsInitialized"),
    (0x003b_a020, snd_init_play_params, "SND_InitPlayParams"),
    (0x003c_6278, snd_list_init, "SND_ListInit"),
    (0x003c_6320, snd_list_remove, "SND_ListRemove"),
    (0x003b_a6e8, snd_handle_to_channel, "SND_HandleToChannel"),
    (
        0x003b_a7b0,
        snd_channel_calc_volume,
        "SND_ChannelCalcVolume",
    ),
    (0x003c_62b8, snd_list_push_back, "SND_ListPushBack"),
    (0x003c_6288, snd_list_push_front, "SND_ListPushFront"),
    (0x003c_62e8, snd_list_pop_front, "SND_ListPopFront"),
    (0x003c_6540, snd_bank_get_patch, "SND_BankGetPatch"),
    (0x003b_6528, snd_bank_get_size, "SND_BankGetSize"),
    (0x003b_64d8, snd_bank_validate, "SND_BankValidate"),
    (0x003b_8d98, sndmidi_get_slot, "SNDMIDI_GetSlot"),
    (0x003b_6a38, sndstrm_get_stream, "SNDSTRM_GetStream"),
    (0x003d_8008, spch_make_id, "SPCH_MakeId"),
    (0x003d_fbd0, stream_file_validate, "StreamFile_Validate"),
    (
        0x003d_fc08,
        stream_file_ring_in_range,
        "StreamFile_RingInRange",
    ),
    (0x003d_ef70, async_file_from_handle, "AsyncFile_FromHandle"),
    (0x003e_2500, big_file_get_type, "BigFile_GetType"),
    (0x003e_4e98, ea_timer_get_ticks, "EATimer_GetTicks"),
];

/// The driver's "bad handle / bad id" result (`SNDERR`-style -8).
const ERR_INVALID: u64 = -8i64 as u64;
/// `SND_BankValidate`: the bank exists but is locked (-18).
const ERR_BANK_LOCKED: u64 = -18i64 as u64;

// ---------------------------------------------------------------------------
// Globals.

/// The sound driver's state block at 0x50a8e8 (the code addresses it as
/// `0x510000 - 0x5718`, so `v1` is often left holding one of those two).
pub mod snd {
    use crate::guest::Addr;

    /// Base of the state block.
    pub const STATE: Addr = Addr(0x0050_a8e8);
    /// The `lui` half the code builds `STATE` from.
    pub const STATE_HI: u64 = 0x0051_0000;

    /// `s8`: nonzero once `SND_Init` has run.
    pub const INITIALIZED: Addr = Addr(0x0050_aa5c);
    /// `s8`: master volume, 0..127.
    pub const MASTER_VOLUME: Addr = Addr(0x0050_aa5d);
    /// `s16`: number of voice channels.
    pub const CHANNEL_COUNT: Addr = Addr(0x0050_aa66);
    /// `SndChannel*`: the channel array.
    pub const CHANNELS: Addr = Addr(0x0050_aad8);
    /// `u16`: number of bank slots.
    pub const BANK_COUNT: Addr = Addr(0x0050_a910);
    /// `SndBankSlot*`: the bank slot array.
    pub const BANKS: Addr = Addr(0x0050_aadc);
    /// `u8`: number of streams.
    pub const STREAM_COUNT: Addr = Addr(0x0050_a92e);
    /// `SndStream*[]`: stream pointers.
    pub const STREAMS: Addr = Addr(0x0050_a0a0);
    /// MIDI player slots (16 of them).
    pub const MIDI_SLOTS: Addr = Addr(0x0050_a120);

    /// One voice channel (0x8c bytes).
    pub const CHANNEL_SIZE: u32 = 0x8c;
    /// Channel: the voice handle it was started with (`int`).
    pub const CH_HANDLE: u32 = 0x00;
    /// Channel: volume (`s8`).
    pub const CH_VOLUME: u32 = 0x2b;
    /// Channel: two `s16` gain factors (patch and play volume, 0..0x7fff).
    pub const CH_GAIN_A: u32 = 0x3a;
    pub const CH_GAIN_B: u32 = 0x42;
    /// Channel: the computed float volume.
    pub const CH_FINAL_VOLUME: u32 = 0x48;
    /// Channel: index into the scale table at `CH_SCALE_TABLE` (`u8`).
    pub const CH_SCALE_INDEX: u32 = 0x67;
    /// Channel: active flag (`s8`).
    pub const CH_ACTIVE: u32 = 0x69;
    /// Channel: volume curve (`s8[]`, indexed by volume * 127), or null.
    pub const CH_CURVE_TABLE: u32 = 0x70;
    /// Channel: scale table (`s8[]`), or null.
    pub const CH_SCALE_TABLE: u32 = 0x78;

    /// One bank slot (0x10 bytes): header pointer at +0, lock flag at +0xc.
    pub const BANK_SLOT_SIZE: u32 = 0x10;
    pub const BANK_SLOT_LOCKED: u32 = 0x0c;
    /// Bank header: size word.
    pub const BANK_HDR_SIZE: u32 = 0x08;

    /// One MIDI slot (0x70 bytes); the handle that owns it at +0x10.
    pub const MIDI_SLOT_SIZE: u32 = 0x70;
    pub const MIDI_SLOT_HANDLE: u32 = 0x10;
}

/// Async file slots (EAFile).
pub mod async_file {
    use crate::guest::Addr;

    /// `AsyncFile*`: the slot array.
    pub const SLOTS: Addr = Addr(0x0045_0c34);
    /// `int`: number of slots.
    pub const COUNT: Addr = Addr(0x0051_9bf0);
    pub const SLOT_SIZE: u32 = 0x30;
}

/// `int`: the EA timer's tick counter.
pub const EA_TIMER_TICKS: Addr = Addr(0x0045_0dcc);

// ---------------------------------------------------------------------------
// Driver state.

/// `SND_IsInitialized` 0x3b5440: the driver's initialised flag (`s8`).
pub fn snd_is_initialized(c: &mut Cpu) {
    c.cycles += 3;
    abi::set_reg(c, V1, snd::STATE_HI); // leftover
    let v = snd::INITIALIZED.lb(c);
    abi::set_reg(c, V0, v);
    abi::ret(c);
}

/// `SND_InitPlayParams` 0x3ba020: fill a play-parameter block with the
/// defaults (full volume 0x7f, centre pan 0x40, middle C 0x3c, unity pitch
/// and gains 0x1000, no loop 0xffff). Bytes 6 and 7 are left alone. Returns 0.
pub fn snd_init_play_params(c: &mut Cpu) {
    c.cycles += 21;
    let p = Addr(abi::arg(c, 0));
    // The original's store order.
    p.off(0x02).sb(c, 0x3c);
    p.off(0x10).sh(c, 0x1000);
    p.off(0x14).sh(c, 0xffff);
    p.off(0x01).sb(c, 0x40);
    p.off(0x05).sb(c, 0x7f);
    p.off(0x0a).sh(c, 0);
    p.off(0x0c).sh(c, 0x1000);
    p.off(0x0e).sh(c, 0x1000);
    p.off(0x12).sh(c, 0);
    p.off(0x16).sh(c, 0);
    p.off(0x03).sb(c, 0x7f);
    p.off(0x00).sb(c, 0x7f);
    p.off(0x04).sb(c, 0x7f);
    p.off(0x08).sh(c, 0);
    abi::set_reg(c, V0, 0);
    abi::set_reg(c, V1, 0x1000); // leftover
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Intrusive doubly linked lists: header {head +0, tail +4, count +8},
// node {next +0, prev +4}.

/// A `SndList` header in guest memory.
#[derive(Clone, Copy)]
struct SndList(Addr);

impl SndList {
    fn head(self) -> Addr {
        self.0
    }
    fn tail(self) -> Addr {
        self.0.off(4)
    }
    fn count(self) -> Addr {
        self.0.off(8)
    }
}

/// A node's `next` link.
#[inline]
fn next_of(n: u64) -> Addr {
    Addr(n as u32)
}

/// A node's `prev` link.
#[inline]
fn prev_of(n: u64) -> Addr {
    Addr(n as u32).off(4)
}

/// `SND_ListInit` 0x3c6278: an empty list.
pub fn snd_list_init(c: &mut Cpu) {
    c.cycles += 4;
    let l = SndList(Addr(abi::arg(c, 0)));
    l.count().sw(c, 0);
    l.head().sw(c, 0);
    l.tail().sw(c, 0);
    abi::ret(c);
}

/// `SND_ListPushFront` 0x3c6288: insert node a1 at the head of list a0.
pub fn snd_list_push_front(c: &mut Cpu) {
    let l = SndList(Addr(abi::arg(c, 0)));
    let n = abi::reg(c, A1);
    c.cycles += 6;
    let head = l.head().lw(c);
    prev_of(n).sw(c, 0);
    next_of(n).sw(c, head);
    let head = l.head().lw(c);
    if head == 0 {
        l.tail().sw(c, n);
    } else {
        c.cycles += 1;
        prev_of(head).sw(c, n);
    }
    c.cycles += 5;
    let count = l.count().lw(c);
    l.head().sw(c, n);
    let count = abi::addu(count, 1);
    l.count().sw(c, count);
    abi::set_reg(c, V0, count); // leftover
    abi::ret(c);
}

/// `SND_ListPushBack` 0x3c62b8: append node a1 to list a0.
pub fn snd_list_push_back(c: &mut Cpu) {
    let l = SndList(Addr(abi::arg(c, 0)));
    let n = abi::reg(c, A1);
    c.cycles += 6;
    next_of(n).sw(c, 0);
    let tail = l.tail().lw(c);
    prev_of(n).sw(c, tail);
    let tail = l.tail().lw(c);
    if tail == 0 {
        l.head().sw(c, n);
    } else {
        c.cycles += 1;
        next_of(tail).sw(c, n);
    }
    c.cycles += 5;
    let count = l.count().lw(c);
    l.tail().sw(c, n);
    let count = abi::addu(count, 1);
    l.count().sw(c, count);
    abi::set_reg(c, V0, count); // leftover
    abi::ret(c);
}

/// `SND_ListPopFront` 0x3c62e8: unlink and return the head node of list a0
/// (0 when empty). The popped node keeps its links.
pub fn snd_list_pop_front(c: &mut Cpu) {
    let l = SndList(Addr(abi::arg(c, 0)));
    c.cycles += 3;
    let node = l.head().lw(c);
    if node != 0 {
        c.cycles += 3;
        let next = next_of(node).lw(c);
        l.head().sw(c, next);
        if next != 0 {
            c.cycles += 1;
            prev_of(next).sw(c, 0);
        } else {
            c.cycles += 2;
            l.tail().sw(c, 0);
        }
        c.cycles += 3;
        let count = abi::addu(l.count().lw(c), u64::MAX);
        l.count().sw(c, count);
    }
    c.cycles += 2;
    abi::set_reg(c, V0, node);
    abi::set_reg(c, V1, node); // leftover
    abi::ret(c);
}

/// `SND_ListRemove` 0x3c6320: unlink node a1 from list a0 (no membership
/// check; the node keeps its links). The original re-reads each link after
/// it may have been rewritten, so the reads below follow it exactly.
pub fn snd_list_remove(c: &mut Cpu) {
    let l = SndList(Addr(abi::arg(c, 0)));
    let n = abi::reg(c, A1);

    c.cycles += 3;
    if n == l.head().lw(c) {
        c.cycles += 3;
        let next = next_of(n).lw(c);
        l.head().sw(c, next);
    }
    let tail = l.tail().lw(c);

    c.cycles += 2;
    if n == tail {
        c.cycles += 3;
        let prev = prev_of(n).lw(c);
        l.tail().sw(c, prev);
    }
    let prev = prev_of(n).lw(c);

    c.cycles += 2;
    if prev != 0 {
        c.cycles += 3;
        let next = next_of(n).lw(c);
        next_of(prev).sw(c, next);
    }
    let next = next_of(n).lw(c);

    c.cycles += 2;
    if next != 0 {
        c.cycles += 3;
        let prev = prev_of(n).lw(c);
        prev_of(next).sw(c, prev);
    }
    let count = l.count().lw(c);

    c.cycles += 3;
    let count = abi::addu(count, u64::MAX);
    l.count().sw(c, count);
    abi::set_reg(c, V0, count);
    abi::set_reg(c, V1, next); // leftover
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Voice channels.

/// `SND_HandleToChannel` 0x3ba6e8: the channel index of voice handle a0
/// (low byte = channel, whole word = the id stored in the channel), or -8
/// when the handle is negative, out of range, the channel is idle or the
/// handle is stale.
pub fn snd_handle_to_channel(c: &mut Cpu) {
    let handle = abi::reg(c, A0);
    c.cycles += 3;
    let mut v1 = snd::STATE_HI;
    let result = 'r: {
        if (handle as i64) < 0 {
            break 'r ERR_INVALID;
        }
        c.cycles += 6;
        let index = handle & 0xff;
        v1 = sx(snd::STATE.0);
        let count = snd::CHANNEL_COUNT.lh(c);
        if (index as i64) >= (count as i64) {
            break 'r ERR_INVALID;
        }
        c.cycles += 6;
        let table = snd::CHANNELS.lw(c);
        let offset = abi::mult(c, index as u32, snd::CHANNEL_SIZE);
        let channel = Addr(table as u32).off(offset);
        v1 = channel.off(snd::CH_ACTIVE).lb(c);
        let ok = v1 != 0 && {
            c.cycles += 3;
            channel.off(snd::CH_HANDLE).lw(c) == handle
        };
        if ok {
            index
        } else {
            ERR_INVALID
        }
    };
    // Every path ends in a two-cycle return block.
    c.cycles += 2;
    abi::set_reg(c, V0, result);
    abi::set_reg(c, V1, v1); // leftover
    abi::ret(c);
}

/// `SND_ChannelCalcVolume` 0x3ba7b0: the float volume of channel a0 (an
/// index), stored to the channel's `CH_FINAL_VOLUME`:
///
/// ```text
/// v = (volume * gain_a * gain_b) / 2048383          (127 * 0x3fff * 0x3fff ≈ 1.0)
/// if scale table:  v = v * scale[scale_index] / 127
/// if curve table:  v = curve[(int)(v * 127)]
/// v = v * master_volume / 127
/// ```
///
/// Integer products wrap at 32 bits; all float steps use the PS2 FPU.
pub fn snd_channel_calc_volume(c: &mut Cpu) {
    const DIVISOR: FpuF32 = FpuF32(0x49fa_0bf8); // 2048383.0
    const MAX_VOLUME: FpuF32 = FpuF32(0x42fe_0000); // 127.0

    let index = abi::arg(c, 0);
    c.cycles += 20;
    let offset = abi::mult(c, index, snd::CHANNEL_SIZE);
    let table = snd::CHANNELS.lw(c);
    let ch = Addr(table as u32).off(offset);
    let volume = ch.off(snd::CH_VOLUME).lb(c);
    let gain_a = ch.off(snd::CH_GAIN_A).lh(c);
    let gain_b = ch.off(snd::CH_GAIN_B).lh(c);
    let p = abi::mult(c, volume as u32, gain_a as u32);
    let scale_table = ch.off(snd::CH_SCALE_TABLE).lw(c);
    let p = abi::mult(c, p, gain_b as u32);
    let mut v = FpuF32::from_i32(p as i32).div(DIVISOR);
    let out = ch.off(snd::CH_FINAL_VOLUME);
    out.write_fpu(c, v);

    if scale_table != 0 {
        c.cycles += 10;
        let i = ch.off(snd::CH_SCALE_INDEX).lbu(c);
        let s = Addr(scale_table as u32).off(i as u32).lb(c);
        v = (v * FpuF32::from_i32(s as i32)).div(MAX_VOLUME);
        out.write_fpu(c, v);
    }

    c.cycles += 3;
    let curve = ch.off(snd::CH_CURVE_TABLE).lw(c);
    let mut v1 = curve;
    if curve != 0 {
        c.cycles += 12;
        let i = (out.read_fpu(c) * MAX_VOLUME).to_i32();
        v1 = Addr(curve as u32).off(i as u32).lb(c);
        v = FpuF32::from_i32(v1 as i32);
        out.write_fpu(c, v);
    }
    let master = snd::MASTER_VOLUME.lb(c);

    c.cycles += 9;
    let master_f = FpuF32::from_i32(master as i32);
    let v = (out.read_fpu(c) * master_f).div(MAX_VOLUME);
    out.write_fpu(c, v);

    c.f[0] = v.0;
    c.f[1] = master_f.0;
    abi::set_reg(c, V0, master); // leftover
    abi::set_reg(c, V1, v1); // leftover
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Banks.

/// `SND_BankGetPatch` 0x3c6540: patch a1 of the bank header at a0, found
/// through the self-relative offset table at +0x14 (count at +6), or 0. The
/// range check is signed, so negative indices read before the table.
pub fn snd_bank_get_patch(c: &mut Cpu) {
    let bank = abi::reg(c, A0);
    let index = abi::reg(c, A1);
    c.cycles += 4;
    let count = Addr(bank as u32).off(6).lhu(c);
    let entry = sx((index as u32) << 2);
    let result = if (index as i64) < (count as i64) {
        c.cycles += 4;
        let slot = abi::addu(bank, entry);
        let rel = Addr(slot as u32).off(0x14).lw(c);
        abi::set_reg(c, V1, rel); // leftover
        if rel == 0 {
            c.cycles += 2;
            0
        } else {
            c.cycles += 3;
            abi::addu(abi::addu(bank, abi::addu(entry, 0x14)), rel)
        }
    } else {
        c.cycles += 2;
        0
    };
    abi::set_reg(c, V0, result);
    abi::ret(c);
}

/// `SND_BankGetSize` 0x3b6528: the size word of bank a0's header (no checks).
pub fn snd_bank_get_size(c: &mut Cpu) {
    let id = abi::arg(c, 0);
    c.cycles += 7;
    let banks = snd::BANKS.lw(c);
    let slot = Addr((id << 4).wrapping_add(banks as u32));
    let header = slot.lw(c);
    let size = Addr(header as u32).off(snd::BANK_HDR_SIZE).lw(c);
    abi::set_reg(c, V0, size);
    abi::set_reg(c, V1, header); // leftover
    abi::ret(c);
}

/// `SND_BankValidate` 0x3b64d8: 0 when bank a0 is loaded and unlocked, -18
/// when it is locked, -8 when the id is out of range or the slot is empty.
pub fn snd_bank_validate(c: &mut Cpu) {
    let id = abi::reg(c, A0);
    c.cycles += 2;
    abi::set_reg(c, V1, snd::STATE_HI); // leftover
    if (id as i64) >= 0 {
        c.cycles += 5;
        abi::set_reg(c, V1, sx(snd::STATE.0)); // leftover
        let count = snd::BANK_COUNT.lhu(c);
        if (id as i64) < (count as i64) {
            let banks = snd::BANKS.lw(c);
            c.cycles += 5;
            let offset = sx((id as u32) << 4);
            abi::set_reg(c, V1, offset); // leftover
            let slot = Addr(abi::addu(offset, banks) as u32);
            let header = slot.lw(c);
            if header == 0 {
                c.cycles += 2;
                abi::set_reg(c, V0, ERR_INVALID);
            } else {
                c.cycles += 4;
                let locked = slot.off(snd::BANK_SLOT_LOCKED).lb(c);
                abi::set_reg(c, V1, locked); // leftover
                abi::set_reg(c, V0, if locked == 0 { 0 } else { ERR_BANK_LOCKED });
            }
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::set_reg(c, V0, ERR_INVALID);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// MIDI, streams, speech.

/// `SNDMIDI_GetSlot` 0x3b8d98: the MIDI slot (0..15) of handle a0 (low five
/// bits = slot, whole value = the id stored in the slot), or -8.
pub fn sndmidi_get_slot(c: &mut Cpu) {
    let handle = abi::reg(c, A0);
    let slot = handle & 0x1f;
    c.cycles += 3;
    let result = if (handle as i64) < 0 {
        c.cycles += 2;
        ERR_INVALID
    } else if slot >= 16 {
        c.cycles += 3 + 2;
        ERR_INVALID
    } else {
        c.cycles += 3 + 9;
        let offset = abi::mult(c, slot as u32, snd::MIDI_SLOT_SIZE);
        let owner = snd::MIDI_SLOTS.off(offset).off(snd::MIDI_SLOT_HANDLE).lw(c);
        let diff = owner ^ handle;
        abi::set_reg(c, V1, diff); // leftover
        if diff == 0 {
            slot
        } else {
            ERR_INVALID
        }
    };
    abi::set_reg(c, V0, result);
    abi::ret(c);
}

/// `SNDSTRM_GetStream` 0x3b6a38: the stream object of stream a0, or 0 when
/// out of range.
pub fn sndstrm_get_stream(c: &mut Cpu) {
    let index = abi::reg(c, A0);
    c.cycles += 5;
    abi::set_reg(c, V1, snd::STATE_HI); // leftover
    let count = snd::STREAM_COUNT.lbu(c);
    let result = if (index as i64) >= (count as i64) {
        c.cycles += 2;
        0
    } else if (index as i64) < 0 {
        c.cycles += 2 + 2;
        0
    } else {
        c.cycles += 2 + 5;
        let entry = abi::addu(sx((index as u32) << 2), sx(snd::STREAMS.0));
        abi::set_reg(c, V1, entry); // leftover
        Addr(entry as u32).lw(c)
    };
    abi::set_reg(c, V0, result);
    abi::ret(c);
}

/// `SPCH_MakeId` 0x3d8008: a speech event id `a0 << 24 | a1 << 16 | a2`
/// (bytes, bytes, halfword). The original assembles it in a dead stack slot
/// and reloads it as a word; the slot is written the same way here.
pub fn spch_make_id(c: &mut Cpu) {
    c.cycles += 7;
    let sp = abi::addu(abi::reg(c, SP), (-0x10i64) as u64);
    let scratch = Addr(sp as u32);
    let (a, b, d) = (abi::reg(c, A0), abi::reg(c, A1), abi::reg(c, A2));
    scratch.off(3).sb(c, a);
    scratch.off(2).sb(c, b);
    scratch.sh(c, d);
    let id = scratch.lw(c);
    abi::set_reg(c, V0, id);
    abi::set_reg(c, SP, abi::addu(sp, 0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Files.

/// "STRM": the stream-file header magic.
const STREAM_MAGIC: u64 = 0x4d52_5453;

/// `StreamFile_Validate` 0x3dfbd0: if stream file a0 is non-null and its
/// header starts with "STRM", store the file to `*a2` and its header to
/// `*a1` and return 0; otherwise return 1.
pub fn stream_file_validate(c: &mut Cpu) {
    let file = abi::reg(c, A0);
    c.cycles += 2;
    if file == 0 {
        c.cycles += 2;
        abi::set_reg(c, V0, 1);
        abi::ret(c);
        return;
    }
    let header = Addr(file as u32).lw(c);
    c.cycles += 5;
    let magic = Addr(header as u32).lw(c);
    abi::set_reg(c, V1, magic); // leftover
    if magic != STREAM_MAGIC {
        c.cycles += 2;
        abi::set_reg(c, V0, 1);
    } else {
        c.cycles += 3;
        Addr(abi::arg(c, 2)).sw(c, file);
        Addr(abi::arg(c, 1)).sw(c, header);
        abi::set_reg(c, V0, 0);
    }
    abi::ret(c);
}

/// `StreamFile_RingInRange` 0x3dfc08: whether ring position a2 lies in
/// `[a0, a1)`, wrapping when `a1 < a0` (unsigned 64-bit compares).
pub fn stream_file_ring_in_range(c: &mut Cpu) {
    let (start, end, pos) = (abi::reg(c, A0), abi::reg(c, A1), abi::reg(c, A2));
    c.cycles += 3;
    let inside = if end < start {
        // Wrapped: [start, top) or [0, end).
        c.cycles += 2;
        if pos >= start {
            c.cycles += 1;
            true
        } else {
            c.cycles += 3;
            let r = pos < end;
            if r {
                c.cycles += 1;
            }
            r
        }
    } else {
        c.cycles += 2;
        if pos < start {
            false
        } else {
            c.cycles += 1;
            pos < end
        }
    };
    c.cycles += 2;
    abi::set_reg(c, V0, inside as u64);
    abi::set_reg(c, V1, inside as u64); // leftover
    abi::ret(c);
}

/// `AsyncFile_FromHandle` 0x3def70: the async-file slot of handle a0 (>= 256,
/// low byte = slot, whole value = the id stored in the slot), or 0.
pub fn async_file_from_handle(c: &mut Cpu) {
    let handle = abi::reg(c, A0);
    let index = handle & 0xff;
    c.cycles += 4;
    let result = 'r: {
        if (handle as i64) < 256 {
            break 'r None;
        }
        c.cycles += 5;
        abi::set_reg(c, V1, 0x0052_0000); // leftover
        let count = async_file::COUNT.lw(c);
        if (index as i64) >= (count as i64) {
            break 'r None;
        }
        c.cycles += 8;
        let offset = abi::mult(c, index as u32, async_file::SLOT_SIZE);
        let slots = async_file::SLOTS.lw(c);
        let slot = abi::addu(slots, offset as u64);
        let diff = Addr(slot as u32).lw(c) ^ handle;
        abi::set_reg(c, V1, diff); // leftover
        Some(if diff == 0 { slot } else { 0 })
    };
    let v0 = match result {
        Some(v) => v,
        None => {
            c.cycles += 2;
            0
        }
    };
    abi::set_reg(c, V0, v0);
    abi::ret(c);
}

/// What `BigFile_GetType` recognises.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u64)]
pub enum BigFileType {
    Unknown = 0,
    /// Starts with the bytes c0 fb (compressed RefPack-style archive).
    C0fb = 1,
    /// "BIGF".
    Bigf = 2,
    /// "BIG" followed by any other byte.
    BigOther = 3,
}

/// `BigFile_GetType` 0x3e2500: classify the archive header at a0 by its
/// magic bytes (big-endian).
pub fn big_file_get_type(c: &mut Cpu) {
    let hdr = Addr(abi::arg(c, 0));
    c.cycles += 8;
    let b0 = hdr.lbu(c);
    let b1 = hdr.off(1).lbu(c);
    let kind = 'k: {
        if (b0 << 8 | b1) == 0xc0fb {
            c.cycles += 2;
            abi::set_reg(c, V1, 0xc0fb); // leftover
            break 'k BigFileType::C0fb;
        }
        let b2 = hdr.off(2).lbu(c);
        c.cycles += 11;
        let b3 = hdr.off(3).lbu(c);
        // `sll 24` sign-extends: a header starting >= 0x80 is negative.
        let word = sx((b0 as u32) << 24) | (b1 << 16) | (b2 << 8) | b3;
        abi::set_reg(c, V1, 0x4249_0000); // leftover
        if word == 0x4249_4746 {
            c.cycles += 2;
            break 'k BigFileType::Bigf;
        }
        c.cycles += 7;
        abi::set_reg(c, V1, 0x4249_4700); // leftover
        if word & !0xff == 0x4249_4700 {
            BigFileType::BigOther
        } else {
            BigFileType::Unknown
        }
    };
    c.cycles += 2;
    abi::set_reg(c, V0, kind as u64);
    abi::ret(c);
}

/// `EATimer_GetTicks` 0x3e4e98: the EA timer's tick counter.
pub fn ea_timer_get_ticks(c: &mut Cpu) {
    c.cycles += 4;
    abi::set_reg(c, V1, 0x0045_0000); // leftover
    let t = EA_TIMER_TICKS.lw(c);
    abi::set_reg(c, V0, t);
    abi::ret(c);
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_paths, check_paths_lohi, fill, scramble, Rng, DATA};

    const SEEDS: u32 = 400;

    /// A scrambled machine and the seed's RNG.
    fn start(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 20) ^ salt);
        scramble(c, &mut r);
        r
    }

    /// A 64-bit register value: usually a sign-extended word, sometimes junk
    /// in the upper half (exercises the 64-bit compares).
    fn reg64(r: &mut Rng, v: u32) -> u64 {
        if r.next_u32() % 6 == 0 {
            (r.next_u32() as u64) << 32 | v as u64
        } else {
            sx(v)
        }
    }

    /// A random 32-bit value biased toward boundaries.
    fn edge_u32(r: &mut Rng) -> u32 {
        match r.next_u32() % 6 {
            0 => 0,
            1 => 0xffff_ffff,
            2 => 0x8000_0000 | (r.next_u32() & 0xff),
            3 => r.next_u32() & 0xff,
            4 => r.next_u32() & 0x3ff,
            _ => r.next_u32(),
        }
    }

    // ------------------------------------------------------------- state

    #[test]
    fn snd_is_initialized_matches() {
        check_paths(
            ssx3_ee::s13::f_003b5440__generated,
            snd_is_initialized,
            SEEDS,
            &[3],
            &|c, seed| {
                let mut r = start(c, seed, 1);
                c.bus.write8(snd::INITIALIZED.0, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn snd_init_play_params_matches() {
        check_paths(
            ssx3_ee::s13::f_003ba020__generated,
            snd_init_play_params,
            SEEDS,
            &[21],
            &|c, seed| {
                let mut r = start(c, seed, 2);
                let p = DATA + (r.next_u32() & 0xffe);
                fill(c, &mut r, DATA, 0x1000 / 4 + 8);
                c.g[A0][0] = reg64(&mut r, p);
            },
        );
    }

    #[test]
    fn ea_timer_get_ticks_matches() {
        check_paths(
            ssx3_ee::s14::f_003e4e98__generated,
            ea_timer_get_ticks,
            SEEDS,
            &[4],
            &|c, seed| {
                let mut r = start(c, seed, 3);
                c.bus.write32(EA_TIMER_TICKS.0, edge_u32(&mut r));
            },
        );
    }

    // ------------------------------------------------------------- lists

    /// A list header at DATA with 0..5 nodes at DATA+0x100+i*0x40 (in
    /// random order); returns the node addresses in list order. Some seeds
    /// leave the counts or links inconsistent (the functions don't care).
    fn list_setup(c: &mut Cpu, r: &mut Rng) -> Vec<u32> {
        fill(c, r, DATA, 0x400 / 4);
        let n = r.next_u32() % 6;
        let mut slots: Vec<u32> = (0..6).collect();
        for i in (1..slots.len()).rev() {
            let j = (r.next_u32() as usize) % (i + 1);
            slots.swap(i, j);
        }
        let nodes: Vec<u32> = slots[..n as usize]
            .iter()
            .map(|s| DATA + 0x100 + s * 0x40)
            .collect();
        for (i, &a) in nodes.iter().enumerate() {
            let next = nodes.get(i + 1).copied().unwrap_or(0);
            let prev = if i == 0 { 0 } else { nodes[i - 1] };
            c.bus.write32(a, next);
            c.bus.write32(a + 4, prev);
        }
        c.bus.write32(DATA, nodes.first().copied().unwrap_or(0));
        c.bus.write32(DATA + 4, nodes.last().copied().unwrap_or(0));
        let count = if r.next_u32() % 4 == 0 {
            r.next_u32()
        } else {
            n
        };
        c.bus.write32(DATA + 8, count);
        c.g[A0][0] = sx(DATA);
        nodes
    }

    #[test]
    fn snd_list_init_matches() {
        check_paths(
            ssx3_ee::s13::f_003c6278__generated,
            snd_list_init,
            SEEDS,
            &[4],
            &|c, seed| {
                let mut r = start(c, seed, 10);
                list_setup(c, &mut r);
            },
        );
    }

    #[test]
    fn snd_list_push_front_matches() {
        check_paths(
            ssx3_ee::s13::f_003c6288__generated,
            snd_list_push_front,
            SEEDS,
            &[11, 12],
            &|c, seed| {
                let mut r = start(c, seed, 11);
                list_setup(c, &mut r);
                c.g[A1][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn snd_list_push_back_matches() {
        check_paths(
            ssx3_ee::s13::f_003c62b8__generated,
            snd_list_push_back,
            SEEDS,
            &[11, 12],
            &|c, seed| {
                let mut r = start(c, seed, 12);
                list_setup(c, &mut r);
                c.g[A1][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn snd_list_pop_front_matches() {
        check_paths(
            ssx3_ee::s13::f_003c62e8__generated,
            snd_list_pop_front,
            SEEDS,
            &[5, 12, 13],
            &|c, seed| {
                let mut r = start(c, seed, 13);
                list_setup(c, &mut r);
            },
        );
    }

    #[test]
    fn snd_list_remove_matches() {
        check_paths(
            ssx3_ee::s13::f_003c6320__generated,
            snd_list_remove,
            SEEDS,
            &[12, 15, 18, 21, 24],
            &|c, seed| {
                let mut r = start(c, seed, 14);
                let nodes = list_setup(c, &mut r);
                // A member (head, tail, middle or the only node), or a node with
                // upper-half junk in the register (never matches head/tail).
                let n = if nodes.is_empty() {
                    DATA + 0x100
                } else {
                    nodes[(r.next_u32() as usize) % nodes.len()]
                };
                if seed % 3 == 0 {
                    // Inconsistent lists (head/tail and links in any
                    // combination), so every branch combination occurs.
                    let pick = |r: &mut Rng| {
                        if r.next_u32() % 2 == 0 {
                            0
                        } else {
                            DATA + 0x100 + (r.next_u32() % 6) * 0x40
                        }
                    };
                    let (next, prev) = (pick(&mut r), pick(&mut r));
                    c.bus.write32(n, next);
                    c.bus.write32(n + 4, prev);
                    if r.next_u32() % 2 == 0 {
                        c.bus.write32(DATA, n);
                    }
                    if r.next_u32() % 2 == 0 {
                        c.bus.write32(DATA + 4, n);
                    }
                }
                c.g[A1][0] = reg64(&mut r, n);
            },
        );
    }

    // ------------------------------------------------------------ channels

    /// A channel table at DATA+0x1000 with 8 channels whose handles mostly
    /// match their index.
    fn channel_setup(c: &mut Cpu, r: &mut Rng) {
        let table = DATA + 0x1000;
        fill(c, r, table, 8 * snd::CHANNEL_SIZE / 4);
        c.bus.write32(snd::CHANNELS.0, table);
        c.bus.write16(
            snd::CHANNEL_COUNT.0,
            match r.next_u32() % 4 {
                0 => r.next_u32() as u16,
                1 => 0,
                _ => 8,
            },
        );
        for i in 0..8 {
            let ch = table + i * snd::CHANNEL_SIZE;
            let id = (r.next_u32() & 0x7fff_ff00) | i;
            c.bus.write32(
                ch,
                if r.next_u32() % 4 == 0 {
                    r.next_u32()
                } else {
                    id
                },
            );
            c.bus.write8(
                ch + snd::CH_ACTIVE,
                if r.next_u32() % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u8
                },
            );
        }
    }

    #[test]
    fn snd_handle_to_channel_matches() {
        check_paths_lohi(
            ssx3_ee::s13::f_003ba6e8__generated,
            snd_handle_to_channel,
            SEEDS,
            &[5, 11, 17, 20],
            &|c, seed| {
                let mut r = start(c, seed, 20);
                channel_setup(c, &mut r);
                let handle = match seed % 5 {
                    0 => r.next_u32(),
                    1 => r.next_u32() & 0x7fff_ffff,
                    _ => {
                        let i = r.next_u32() % 9;
                        let ch = DATA + 0x1000 + (i % 8) * snd::CHANNEL_SIZE;
                        c.bus.read32(ch) & !0xff | i
                    }
                };
                c.g[A0][0] = reg64(&mut r, handle);
            },
        );
    }

    #[test]
    fn snd_channel_calc_volume_matches() {
        check_paths_lohi(
            ssx3_ee::s13::f_003ba7b0__generated,
            snd_channel_calc_volume,
            SEEDS * 2,
            &[32, 42, 44, 54],
            &|c, seed| {
                let mut r = start(c, seed, 21);
                channel_setup(c, &mut r);
                let idx = r.next_u32() % 8;
                let ch = DATA + 0x1000 + idx * snd::CHANNEL_SIZE;
                // Realistic gains most of the time, anything otherwise.
                let small = seed % 3 != 0;
                let g = |r: &mut Rng| {
                    if small {
                        (r.next_u32() % 0x1001) as u16
                    } else {
                        r.next_u32() as u16
                    }
                };
                let (ga, gb) = (g(&mut r), g(&mut r));
                c.bus.write16(ch + snd::CH_GAIN_A, ga);
                c.bus.write16(ch + snd::CH_GAIN_B, gb);
                if small {
                    c.bus
                        .write8(ch + snd::CH_VOLUME, (r.next_u32() % 128) as u8);
                }
                // Scale and curve tables around 16 MB (the curve index can be
                // about ±140k for wrapped products).
                let scale = 0x0100_0000;
                let curve = 0x0110_0000;
                fill(c, &mut r, scale, 0x40);
                fill(c, &mut r, curve - 0x400, 0x200);
                c.bus.write32(
                    ch + snd::CH_SCALE_TABLE,
                    if r.next_u32() % 2 == 0 { 0 } else { scale },
                );
                c.bus.write32(
                    ch + snd::CH_CURVE_TABLE,
                    if r.next_u32() % 2 == 0 { 0 } else { curve },
                );
                c.bus.write8(
                    snd::MASTER_VOLUME.0,
                    if r.next_u32() % 4 == 0 {
                        r.next_u32() as u8
                    } else {
                        0x7f
                    },
                );
                c.g[A0][0] = reg64(&mut r, idx);
            },
        );
    }

    // --------------------------------------------------------------- banks

    #[test]
    fn snd_bank_get_patch_matches() {
        check_paths(
            ssx3_ee::s13::f_003c6540__generated,
            snd_bank_get_patch,
            SEEDS,
            &[6, 10, 11],
            &|c, seed| {
                let mut r = start(c, seed, 30);
                let bank = DATA + 0x100;
                fill(c, &mut r, DATA, 0x200 / 4);
                c.bus.write16(bank + 6, (r.next_u32() % 40) as u16);
                for i in 0..32 {
                    let off = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32() & 0xffff
                    };
                    c.bus.write32(bank + 0x14 + i * 4, off);
                }
                let index = match seed % 4 {
                    0 => (r.next_u32() % 64).wrapping_sub(32), // negatives
                    _ => r.next_u32() % 34,
                };
                c.g[A0][0] = sx(bank);
                c.g[A1][0] = reg64(&mut r, index);
            },
        );
    }

    /// Bank slots at DATA+0x2000 (16 of them), headers at DATA+0x3000.
    fn bank_setup(c: &mut Cpu, r: &mut Rng) {
        let slots = DATA + 0x2000;
        fill(c, r, slots, 16 * 4);
        fill(c, r, DATA + 0x3000, 16 * 4);
        c.bus.write32(snd::BANKS.0, slots);
        c.bus.write16(
            snd::BANK_COUNT.0,
            if r.next_u32() % 4 == 0 {
                r.next_u32() as u16
            } else {
                12
            },
        );
        for i in 0..16 {
            let hdr = if r.next_u32() % 3 == 0 {
                0
            } else {
                DATA + 0x3000 + i * 0x10
            };
            c.bus.write32(slots + i * 0x10, hdr);
            if r.next_u32() % 2 == 0 {
                c.bus.write8(slots + i * 0x10 + 0xc, 0);
            }
        }
    }

    #[test]
    fn snd_bank_get_size_matches() {
        check_paths(
            ssx3_ee::s13::f_003b6528__generated,
            snd_bank_get_size,
            SEEDS,
            &[7],
            &|c, seed| {
                let mut r = start(c, seed, 31);
                bank_setup(c, &mut r);
                let id = r.next_u32() % 16;
                // Keep the header pointer valid.
                c.bus
                    .write32(DATA + 0x2000 + id * 0x10, DATA + 0x3000 + id * 0x10);
                c.g[A0][0] = reg64(&mut r, id);
            },
        );
    }

    #[test]
    fn snd_bank_validate_matches() {
        check_paths(
            ssx3_ee::s13::f_003b64d8__generated,
            snd_bank_validate,
            SEEDS,
            &[4, 9, 14, 16],
            &|c, seed| {
                let mut r = start(c, seed, 32);
                bank_setup(c, &mut r);
                let id = match seed % 4 {
                    0 => r.next_u32(),
                    _ => r.next_u32() % 16,
                };
                c.g[A0][0] = reg64(&mut r, id);
            },
        );
    }

    // ---------------------------------------------- MIDI, streams, speech

    #[test]
    fn sndmidi_get_slot_matches() {
        check_paths_lohi(
            ssx3_ee::s13::f_003b8d98__generated,
            sndmidi_get_slot,
            SEEDS,
            &[5, 8, 15],
            &|c, seed| {
                let mut r = start(c, seed, 40);
                for i in 0..16 {
                    let id = (r.next_u32() & 0x7fff_ffe0) | i;
                    let a = snd::MIDI_SLOTS.0 + i * snd::MIDI_SLOT_SIZE + 0x10;
                    c.bus.write32(
                        a,
                        if r.next_u32() % 4 == 0 {
                            r.next_u32()
                        } else {
                            id
                        },
                    );
                }
                let handle = match seed % 4 {
                    0 => r.next_u32(),
                    1 => (r.next_u32() & 0x7fff_ffe0) | (16 + r.next_u32() % 16),
                    _ => {
                        let i = r.next_u32() % 16;
                        c.bus
                            .read32(snd::MIDI_SLOTS.0 + i * snd::MIDI_SLOT_SIZE + 0x10)
                    }
                };
                c.g[A0][0] = reg64(&mut r, handle);
            },
        );
    }

    #[test]
    fn sndstrm_get_stream_matches() {
        check_paths(
            ssx3_ee::s13::f_003b6a38__generated,
            sndstrm_get_stream,
            SEEDS,
            &[7, 9, 12],
            &|c, seed| {
                let mut r = start(c, seed, 41);
                fill(c, &mut r, snd::STREAMS.0, 0x40);
                c.bus
                    .write8(snd::STREAM_COUNT.0, (r.next_u32() % 0x40) as u8);
                let idx = match seed % 4 {
                    0 => r.next_u32(),
                    1 => 0u32.wrapping_sub(r.next_u32() % 4),
                    _ => r.next_u32() % 0x48,
                };
                c.g[A0][0] = reg64(&mut r, idx);
            },
        );
    }

    #[test]
    fn spch_make_id_matches() {
        check_paths(
            ssx3_ee::s14::f_003d8008__generated,
            spch_make_id,
            SEEDS,
            &[7],
            &|c, seed| {
                let mut r = start(c, seed, 42);
                fill(c, &mut r, crate::difftest::SP - 0x20, 0x10);
                // Registers keep their scrambled upper bits.
                let _ = &mut r;
            },
        );
    }

    // --------------------------------------------------------------- files

    #[test]
    fn stream_file_validate_matches() {
        check_paths(
            ssx3_ee::s14::f_003dfbd0__generated,
            stream_file_validate,
            SEEDS,
            &[4, 9, 10],
            &|c, seed| {
                let mut r = start(c, seed, 50);
                let (file, hdr) = (DATA + 0x100, DATA + 0x200);
                fill(c, &mut r, DATA, 0x400 / 4);
                c.bus.write32(file, hdr);
                c.bus.write32(
                    hdr,
                    if seed % 3 == 0 {
                        r.next_u32()
                    } else {
                        STREAM_MAGIC as u32
                    },
                );
                c.g[A0][0] = if seed % 5 == 0 {
                    0
                } else {
                    reg64(&mut r, file)
                };
                c.g[A1][0] = reg64(&mut r, DATA + 0x300);
                c.g[A2][0] = reg64(&mut r, DATA + 0x304);
            },
        );
    }

    #[test]
    fn stream_file_ring_in_range_matches() {
        check_paths(
            ssx3_ee::s14::f_003dfc08__generated,
            stream_file_ring_in_range,
            SEEDS * 2,
            &[7, 8, 10, 11],
            &|c, seed| {
                let mut r = start(c, seed, 51);
                // Small values so all orderings (and equality) occur; sometimes
                // full 64-bit values.
                let v = |r: &mut Rng| -> u64 {
                    match r.next_u32() % 5 {
                        0 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                        1 => sx(0x8000_0000 | r.next_u32() % 4),
                        _ => (r.next_u32() % 6) as u64,
                    }
                };
                c.g[A0][0] = v(&mut r);
                c.g[A1][0] = v(&mut r);
                c.g[A2][0] = v(&mut r);
            },
        );
    }

    #[test]
    fn async_file_from_handle_matches() {
        check_paths_lohi(
            ssx3_ee::s14::f_003def70__generated,
            async_file_from_handle,
            SEEDS,
            &[6, 11, 17],
            &|c, seed| {
                let mut r = start(c, seed, 52);
                let slots = DATA + 0x1000;
                fill(c, &mut r, slots, 0x100 * 0x30 / 4);
                c.bus.write32(async_file::SLOTS.0, slots);
                c.bus.write32(
                    async_file::COUNT.0,
                    match r.next_u32() % 4 {
                        0 => r.next_u32(),
                        _ => 1 + r.next_u32() % 0x20,
                    },
                );
                for i in 0..0x20 {
                    let id = (r.next_u32() & 0x7fff_ff00) | 0x100 | i;
                    c.bus.write32(slots + i * 0x30, id);
                }
                let handle = match seed % 5 {
                    0 => r.next_u32(),
                    1 => r.next_u32() % 0x200,
                    _ => c.bus.read32(slots + (r.next_u32() % 0x24) * 0x30),
                };
                c.g[A0][0] = reg64(&mut r, handle);
            },
        );
    }

    #[test]
    fn big_file_get_type_matches() {
        check_paths(
            ssx3_ee::s14::f_003e2500__generated,
            big_file_get_type,
            SEEDS,
            &[12, 23, 28],
            &|c, seed| {
                let mut r = start(c, seed, 53);
                let mut hdr: [u8; 4] = r.next_u32().to_le_bytes();
                match seed % 6 {
                    0 => hdr[..2].copy_from_slice(&[0xc0, 0xfb]),
                    1 => hdr = *b"BIGF",
                    2 => hdr[..3].copy_from_slice(b"BIG"),
                    3 => hdr[..2].copy_from_slice(&[0xc0, r.next_u32() as u8]),
                    4 => hdr[..3].copy_from_slice(&[b'B', b'I', r.next_u32() as u8]),
                    _ => {}
                }
                let a = DATA + (r.next_u32() & 0xfff);
                for (i, b) in hdr.iter().enumerate() {
                    c.bus.write8(a + i as u32, *b);
                }
                c.g[A0][0] = reg64(&mut r, a);
            },
        );
    }
}
