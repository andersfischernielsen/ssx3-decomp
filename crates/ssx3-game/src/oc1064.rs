//! Staging module oc1064 (wave oc1064): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1064`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due, Resume};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x003f_4528, dirty_rpc_send, "DirtyRpc_Send")];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::{ee, Callee};

    // Called through the function table (`call_ptr`), as the compiled code does.
    pub(crate) const STRLEN: u32 = 0x0041_6810;
    pub(crate) const WAIT_SEMA: u32 = 0x0042_3de0;
    pub(crate) const SIGNAL_SEMA: u32 = 0x0042_3dc0;
    pub(crate) const GET_THREAD_ID: u32 = 0x0042_3c90;
    pub(crate) const MEMCPY: u32 = 0x0041_605c;
    pub(crate) const SYNC_DCACHE: u32 = 0x0042_4698;
    pub(crate) const SCE_SIF_SET_DMA: u32 = 0x0042_4160;
    pub(crate) const SCE_SIF_SEND_CMD: u32 = 0x0042_61b0;
    pub(crate) const SET_ALARM: u32 = 0x0042_3b20;
    pub(crate) const SLEEP_THREAD: u32 = 0x0042_3cc0;

    pub(crate) const DIRTY_RPC_PROCESS_REPLIES: Callee = Callee {
        addr: 0x003f_4a78,
        f: ee::s14::DirtyRpc_ProcessReplies,
    };

    pub(crate) const NET_TICK: Callee = Callee {
        addr: 0x003f_4de8,
        f: ee::s14::NetTick,
    };
}

// ---------------------------------------------------------------------------
// DirtyRpc: the EE side of DirtySock's IOP RPC transport.

/// The DirtyRpc state block (`lui 0x52; addiu -0xf40`; `DirtyRpc_Send` keeps
/// it in `s2`). The fields below are offsets into it.
pub(crate) const DIRTY_RPC_STATE: u32 = 0x0051_f0c0;
/// Request ids count modulo this word.
pub(crate) const DIRTY_RPC_ID_LIMIT: u32 = 0x4d84;
/// The request descriptors, 0x20 bytes per ring slot: +0 the data's IOP
/// address, +4 its length, +8 the reply buffer, +0xc the reply size, +0x10 and
/// +0x14 two caller words, +0x18 the client's word +0xc, +0x1c the request id.
pub(crate) const DIRTY_RPC_REQUESTS: u32 = 0x4d94;
/// The IOP address of the descriptor table (the descriptor DMA's destination).
pub(crate) const DIRTY_RPC_IOP_REQUESTS: u32 = 0x4d9c;
/// The EE data buffer, and the IOP address of its twin.
pub(crate) const DIRTY_RPC_EE_BUFFER: u32 = 0x4dac;
pub(crate) const DIRTY_RPC_IOP_BUFFER: u32 = 0x4db4;
/// The `sceSifSetDma` id per ring slot (4 bytes each).
pub(crate) const DIRTY_RPC_DMA_IDS: u32 = 0x4db8;
/// Per ring slot, 0x10 bytes: the sender's mode word, its context word, the
/// send flags and its thread id (what `DirtyRpc_ProcessReplies` acts on).
pub(crate) const DIRTY_RPC_WAITERS: u32 = 0x4dc4;
/// The next request id.
pub(crate) const DIRTY_RPC_NEXT_ID: u32 = 0x4dcc;
/// The request ring: the slot the next send fills, the oldest unanswered slot,
/// the slot count.
pub(crate) const DIRTY_RPC_HEAD: u32 = 0x4dd0;
pub(crate) const DIRTY_RPC_TAIL: u32 = 0x4dd4;
pub(crate) const DIRTY_RPC_SLOTS: u32 = 0x4dd8;
/// The data buffer's write and read offsets and its size.
pub(crate) const DIRTY_RPC_WRITE: u32 = 0x4ddc;
pub(crate) const DIRTY_RPC_READ: u32 = 0x4de0;
pub(crate) const DIRTY_RPC_BUFFER_SIZE: u32 = 0x4de4;
/// The poll alarm's id (negative: no alarm running).
pub(crate) const DIRTY_RPC_ALARM: u32 = 0x4dec;
/// The semaphore guarding the ring.
pub(crate) const DIRTY_RPC_SEMA: u32 = 0x4e08;
/// The reply descriptors the IOP writes back (0x20 per slot, the id at +0x1c).
#[cfg(test)]
pub(crate) const DIRTY_RPC_REPLIES: u32 = 0x4da0;
/// `NetTick`'s clock: the count register at the base (a word) and the base
/// itself in count ticks (a doubleword).
#[cfg(test)]
pub(crate) const DIRTY_RPC_CLOCK_COUNT: u32 = 0x4df8;
#[cfg(test)]
pub(crate) const DIRTY_RPC_CLOCK_BASE: u32 = 0x4e00;

/// `gp+0x2a10`: 2 while `DirtyRpc_Send` waits on the semaphore (1 while
/// `DirtyRpc_ProcessReplies` does), else 0.
pub(crate) const DIRTY_RPC_SEMA_WAIT_GP: u32 = 0x2a10;
/// `gp+0x2a18`: 2 while a sender sleeps until its reply, else 0.
pub(crate) const DIRTY_RPC_SLEEP_GP: u32 = 0x2a18;

/// The client's word at +0xc: 0 until it is bound; each request carries it.
pub(crate) const DIRTY_RPC_CLIENT_ID: u32 = 0xc;
/// Send flag 1: after the DMA, send SIF command 1 with the packet at
/// [`DIRTY_RPC_SIF_PACKET`].
pub(crate) const DIRTY_RPC_SEND_SIF_CMD: u64 = 1;
/// Send flag 2: the data is a gather list of `{int len; void* ptr}` pairs
/// ended by a `len <= 0`.
pub(crate) const DIRTY_RPC_SEND_GATHER: u64 = 2;
/// Send flag 4: `SyncDCache` the reply buffer before the DMA.
pub(crate) const DIRTY_RPC_SEND_SYNC_REPLY: u64 = 4;
/// The SIF command packet of flag 1 (its first byte is its size).
pub(crate) const DIRTY_RPC_SIF_PACKET: u32 = 0x0045_1300;
/// `DirtyRpc_AlarmHandler`, the alarm a blocking send starts, and its period.
pub(crate) const DIRTY_RPC_ALARM_HANDLER: u32 = 0x003f_4448;
const DIRTY_RPC_ALARM_TICKS: u64 = 8;

/// `DirtyRpc_Send` and the end of its code.
pub(crate) const DIRTY_RPC_SEND: u32 = 0x003f_4528;
const DIRTY_RPC_SEND_END: u32 = 0x003f_4a78;
/// Its back-edge loop heads, where an unwound call resumes: summing the gather
/// list, waiting for a free ring slot, copying the gather list, retrying the
/// DMA, spinning until the next millisecond.
const DIRTY_RPC_SEND_SUM_HEAD: u32 = 0x003f_45e0;
const DIRTY_RPC_SEND_SLOT_HEAD: u32 = 0x003f_4648;
const DIRTY_RPC_SEND_COPY_HEAD: u32 = 0x003f_4778;
const DIRTY_RPC_SEND_RETRY_HEAD: u32 = 0x003f_48f8;
const DIRTY_RPC_SEND_TICK_HEAD: u32 = 0x003f_4908;

/// Its stack frame (the same at every resume point).
const DIRTY_RPC_SEND_FRAME: Frame = Frame { size: 0xe0 };
/// Frame slots: the two `sceSifSetDma` transfers `{src, dst, size, attr}`
/// (the data, then the descriptor), the caller's a2 and a3, the SIF-command
/// and sync flags, `len + 0x3f`; above the frame the caller's stack arguments
/// (the mode word, the context word).
const DIRTY_RPC_SEND_DMA_DATA: u32 = 0x00;
const DIRTY_RPC_SEND_DMA_REQUEST: u32 = 0x10;
const DIRTY_RPC_SEND_WORD2: u32 = 0x20;
const DIRTY_RPC_SEND_WORD3: u32 = 0x24;
const DIRTY_RPC_SEND_SIF_FLAG: u32 = 0x28;
const DIRTY_RPC_SEND_SYNC_FLAG: u32 = 0x2c;
const DIRTY_RPC_SEND_PADDED: u32 = 0x30;
const DIRTY_RPC_SEND_MODE: u32 = 0xe0;
const DIRTY_RPC_SEND_CONTEXT: u32 = 0xe8;

/// Where `DirtyRpc_Send` goes on after a stage (the entry and each resume
/// head start one).
#[derive(Clone, Copy, PartialEq, Debug)]
enum DirtyRpcSendStage {
    /// 0x3f45e0 (loop head): add up the gather list's lengths (a0 walks it).
    SumGather,
    /// 0x3f4600: take the semaphore (a0) and look for a free ring slot.
    Lock,
    /// 0x3f4648 (loop head): the ring is full; process replies until a slot
    /// frees up.
    WaitForSlot,
    /// 0x3f468c: find room for the data from the write offset (a0).
    Reserve,
    /// 0x3f4778 (loop head): copy the gather piece at s0 to s1.
    CopyGather,
    /// 0x3f47b8: fill the request descriptor (v1 = the head slot), queue the
    /// DMA.
    Describe,
    /// 0x3f48f8 (loop head): the DMA got no id; wait for the next tick.
    DmaRetry,
    /// 0x3f4908 (loop head): spin until `NetTick` changes.
    TickWait,
    /// 0x3f496c: the SIF command (v1 = its flag), advance the ring, block if
    /// asked.
    Finish,
}

type DirtyRpcSendStep = Result<Resume<DirtyRpcSendStage>, Unwound>;

/// A field of the DirtyRpc state block, addressed through `s2` as the
/// compiled code does.
#[inline]
fn dirty_rpc_send_state(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, S2).wrapping_add(off))
}

/// A slot of `DirtyRpc_Send`'s frame.
#[inline]
fn dirty_rpc_send_slot(c: &Cpu, off: u32) -> Addr {
    Addr(DIRTY_RPC_SEND_FRAME.at(c, off))
}

/// A gp-relative global.
#[inline]
fn dirty_rpc_send_gp(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, GP).wrapping_add(off))
}

/// The head slot's entry of the per-slot table at state field `table`
/// (`dst = table + (head << shift)`), reloading the head into `dst` and the
/// table base into `tmp` as the compiled code does before every field store.
fn dirty_rpc_send_head_entry(c: &mut Cpu, dst: usize, tmp: usize, table: u32, shift: u32) -> Addr {
    c.g[dst][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    c.g[tmp][0] = dirty_rpc_send_state(c, table).lw(c);
    c.g[dst][0] = sx(r32(c, dst) << shift);
    c.g[dst][0] = sx(r32(c, dst).wrapping_add(r32(c, tmp)));
    Addr(r32(c, dst))
}

/// `int DirtyRpc_Send(client, flags, w2, w3, data, len, reply, reply_size,
/// mode, ctx)` (0x3f4528): queue an RPC request for the IOP; returns its
/// request id, or -1.
///
/// -1 at once when `client` is null, its word at +0xc is 0, or `reply` is not
/// 16-byte aligned (`reply_size` is rounded down to 16). The data length is
/// `len`, or `strlen(data) + 1` when `len < 0`; with flag 2 `data` is a gather
/// list and the length is the sum of its pieces. Under the ring semaphore it
/// waits for a free ring slot (releasing the semaphore and running
/// `DirtyRpc_ProcessReplies` meanwhile) and looks for room in the data buffer:
/// before the read offset, at the write offset up to the end, or else from 0
/// (each less a 0x40 guard); no room releases the semaphore and returns -1.
/// It fills the slot's waiter entry (`mode`, `ctx`, `flags`, the thread id),
/// copies the data, fills the request descriptor, `SyncDCache`s the reply
/// buffer (flag 4) and DMAs the data and the descriptor to the IOP (just the
/// descriptor when there is no data), waiting a `NetTick` millisecond and
/// retrying while `sceSifSetDma` returns 0. Flag 1 then sends SIF command 1.
/// It moves the write offset past the data (64-byte aligned), advances the
/// request id and the ring head, and releases the semaphore. `mode == 1`
/// waits for the reply: it starts the poll alarm if none runs and sleeps.
pub fn dirty_rpc_send(c: &mut Cpu) {
    use DirtyRpcSendStage as Stage;
    let mut step = match c.pc {
        DIRTY_RPC_SEND => dirty_rpc_send_enter(c),
        DIRTY_RPC_SEND_SUM_HEAD => Ok(Resume::Go(Stage::SumGather)),
        DIRTY_RPC_SEND_SLOT_HEAD => Ok(Resume::Go(Stage::WaitForSlot)),
        DIRTY_RPC_SEND_COPY_HEAD => Ok(Resume::Go(Stage::CopyGather)),
        DIRTY_RPC_SEND_RETRY_HEAD => Ok(Resume::Go(Stage::DmaRetry)),
        DIRTY_RPC_SEND_TICK_HEAD => Ok(Resume::Go(Stage::TickWait)),
        pc => {
            c.jump_out(pc, DIRTY_RPC_SEND, DIRTY_RPC_SEND_END);
            return;
        }
    };
    while let Ok(Resume::Go(stage)) = step {
        step = match stage {
            Stage::SumGather => dirty_rpc_send_sum_gather(c),
            Stage::Lock => dirty_rpc_send_lock(c),
            Stage::WaitForSlot => dirty_rpc_send_wait_for_slot(c),
            Stage::Reserve => dirty_rpc_send_reserve(c),
            Stage::CopyGather => dirty_rpc_send_copy_gather(c),
            Stage::Describe => dirty_rpc_send_describe(c),
            Stage::DmaRetry => dirty_rpc_send_dma_retry(c),
            Stage::TickWait => dirty_rpc_send_tick_wait(c),
            Stage::Finish => dirty_rpc_send_finish(c),
        };
    }
}

/// The epilogue (0x3f4a48).
fn dirty_rpc_send_leave(c: &mut Cpu) -> Resume<DirtyRpcSendStage> {
    let frame = DIRTY_RPC_SEND_FRAME;
    c.cycles += 12;
    frame.restore(c, S0, 0xd0);
    frame.restore(c, S1, 0xc0);
    frame.restore(c, S2, 0xb0);
    frame.restore(c, S3, 0xa0);
    frame.restore(c, S4, 0x90);
    frame.restore(c, S5, 0x80);
    frame.restore(c, S6, 0x70);
    frame.restore(c, S7, 0x60);
    frame.restore(c, FP, 0x50);
    frame.restore_ra(c, 0x40);
    frame.close(c);
    Resume::Done
}

/// The entry (0x3f4528): the frame, the argument checks, the length.
fn dirty_rpc_send_enter(c: &mut Cpu) -> DirtyRpcSendStep {
    use DirtyRpcSendStage as Stage;
    c.cycles += 22;
    let frame = Frame::open(c, DIRTY_RPC_SEND_FRAME.size);
    c.g[V0][0] = 0x52_0000;
    frame.save(c, S1, 0xc0);
    frame.save(c, S2, 0xb0);
    frame.save(c, S3, 0xa0);
    frame.save(c, S5, 0x80);
    frame.save(c, S6, 0x70);
    frame.save(c, S7, 0x60);
    frame.save(c, FP, 0x50);
    frame.save(c, S0, 0xd0);
    frame.save(c, S4, 0x90);
    frame.save_ra(c, 0x40);
    c.g[S1][0] = c.g[A1][0]; // flags
    c.g[S2][0] = sx(DIRTY_RPC_STATE);
    c.g[S3][0] = c.g[T1][0]; // len
    c.g[S5][0] = c.g[T0][0]; // data
    c.g[S6][0] = c.g[T3][0]; // reply size
    c.g[S7][0] = c.g[A0][0]; // client
    c.g[FP][0] = c.g[T2][0]; // reply buffer
    dirty_rpc_send_slot(c, DIRTY_RPC_SEND_WORD2).sw(c, c.g[A2][0]);
    dirty_rpc_send_slot(c, DIRTY_RPC_SEND_WORD3).sw(c, c.g[A3][0]);
    'fail: {
        if c.g[S7][0] == 0 {
            break 'fail;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S7).wrapping_add(DIRTY_RPC_CLIENT_ID)).lw(c);
        let unbound = c.g[V0][0] == 0;
        c.g[V1][0] = c.g[FP][0] & 0xf;
        if unbound {
            break 'fail;
        }
        c.cycles += 4;
        c.g[V0][0] = 0xffff_ffff_ffff_fff0;
        let aligned = c.g[V1][0] == 0;
        c.g[S6][0] = c.g[S6][0] & c.g[V0][0];
        if !aligned {
            break 'fail;
        }
        // The length: strlen + 1 for a negative one; the gather list's sum.
        c.cycles += 2;
        let has_len = (c.g[S3][0] as i64) >= 0;
        c.g[V0][0] = c.g[S1][0] & DIRTY_RPC_SEND_GATHER;
        if !has_len {
            c.cycles += 4;
            c.g[A0][0] = c.g[S5][0];
            call_ptr(c, callee::STRLEN, 0x003f_45b4)?;
            c.g[S3][0] = sx(r32(c, V0).wrapping_add(1));
            c.g[V0][0] = c.g[S1][0] & DIRTY_RPC_SEND_GATHER;
        }
        c.cycles += 4;
        c.g[S0][0] = if c.g[V0][0] != 0 { c.g[S5][0] } else { 0 };
        if c.g[S0][0] == 0 {
            c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
            return Ok(Resume::Go(Stage::Lock));
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 0;
        if empty {
            c.cycles += 1;
            c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
            return Ok(Resume::Go(Stage::Lock));
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        return Ok(Resume::Go(Stage::SumGather));
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    Ok(dirty_rpc_send_leave(c))
}

/// 0x3f45e0: add up the gather pieces' lengths into s3 until a `len <= 0`.
fn dirty_rpc_send_sum_gather(c: &mut Cpu) -> DirtyRpcSendStep {
    loop {
        c.cycles += 7;
        c.g[V1][0] = Addr(r32(c, A0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        let more = (c.g[V0][0] as i64) > 0;
        c.g[S3][0] = sx(r32(c, S3).wrapping_add(r32(c, V1)));
        if !more {
            break;
        }
        if back_edge_unwinds(c, DIRTY_RPC_SEND_SUM_HEAD) {
            return Ok(Resume::Unwind);
        }
    }
    c.cycles += 1;
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
    Ok(Resume::Go(DirtyRpcSendStage::Lock))
}

/// 0x3f4600: `WaitSema(a0)` (marked at `gp+0x2a10`), then is the slot after
/// the head the tail (the ring full)?
fn dirty_rpc_send_lock(c: &mut Cpu) -> DirtyRpcSendStep {
    use DirtyRpcSendStage as Stage;
    c.cycles += 11;
    c.g[V0][0] = 2;
    dirty_rpc_send_gp(c, DIRTY_RPC_SEMA_WAIT_GP).sw(c, c.g[V0][0]);
    call_ptr(c, callee::WAIT_SEMA, 0x003f_4610)?;
    c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    c.g[A1][0] = dirty_rpc_send_state(c, DIRTY_RPC_SLOTS).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    dirty_rpc_send_gp(c, DIRTY_RPC_SEMA_WAIT_GP).sw(c, 0);
    c.div(0, V0, A1);
    c.cycles += 4;
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_TAIL).lw(c);
    c.g[A0][0] = c.hi[0];
    if c.g[A0][0] != c.g[V1][0] {
        c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_WRITE).lw(c);
        return Ok(Resume::Go(Stage::Reserve));
    }
    c.cycles += 3;
    Ok(Resume::Go(Stage::WaitForSlot))
}

/// 0x3f4648: while the ring is full, release the semaphore, process replies
/// and take it again.
fn dirty_rpc_send_wait_for_slot(c: &mut Cpu) -> DirtyRpcSendStep {
    loop {
        c.cycles += 12;
        c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
        call_ptr(c, callee::SIGNAL_SEMA, 0x003f_4650)?;
        callee::DIRTY_RPC_PROCESS_REPLIES.call(c, 0x003f_4658)?;
        c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
        call_ptr(c, callee::WAIT_SEMA, 0x003f_4660)?;
        c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
        c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SLOTS).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        c.div(0, V1, V0);
        c.cycles += 4;
        c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_TAIL).lw(c);
        c.g[A0][0] = c.hi[0];
        if c.g[A0][0] != c.g[V1][0] {
            break;
        }
        if back_edge_unwinds(c, DIRTY_RPC_SEND_SLOT_HEAD) {
            return Ok(Resume::Unwind);
        }
    }
    c.cycles += 1;
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_WRITE).lw(c);
    Ok(Resume::Go(DirtyRpcSendStage::Reserve))
}

/// 0x3f468c: the data's offset (s4) and the room there; then claim the slot.
fn dirty_rpc_send_reserve(c: &mut Cpu) -> DirtyRpcSendStep {
    c.cycles += 4;
    c.g[A1][0] = dirty_rpc_send_state(c, DIRTY_RPC_READ).lw(c);
    let behind = (c.g[A0][0] as i64) < (c.g[A1][0] as i64);
    c.g[V0][0] = behind as u64;
    c.g[S4][0] = c.g[A0][0];
    if behind {
        // Up to the read offset.
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, A1).wrapping_sub(r32(c, S4)));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x40));
    } else {
        // Up to the end, or else from 0 up to the read offset.
        c.cycles += 6;
        c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_BUFFER_SIZE).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, A0)));
        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x40));
        let wrap = (c.g[V0][0] as i64) < (c.g[S3][0] as i64);
        c.g[V0][0] = sx(r32(c, A1).wrapping_sub(0x40));
        if wrap {
            c.cycles += 1;
            c.g[S4][0] = 0;
        } else {
            c.cycles += 3;
            c.g[S4][0] = c.g[A0][0];
            c.g[V0][0] = c.g[V1][0];
        }
    }
    c.cycles += 3;
    let short = (c.g[V0][0] as i64) < (c.g[S3][0] as i64);
    c.g[V0][0] = short as u64;
    if short {
        c.cycles += 4;
        c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
        call_ptr(c, callee::SIGNAL_SEMA, 0x003f_46e4)?;
        c.g[V0][0] = u64::MAX;
        return Ok(dirty_rpc_send_leave(c));
    }
    c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    dirty_rpc_send_claim(c)
}

/// 0x3f46ec: the head slot's waiter entry (v0 = the head), then the data copy.
fn dirty_rpc_send_claim(c: &mut Cpu) -> DirtyRpcSendStep {
    use DirtyRpcSendStage as Stage;
    c.cycles += 29;
    c.g[V1][0] = c.g[S1][0] & DIRTY_RPC_SEND_SIF_CMD;
    dirty_rpc_send_slot(c, DIRTY_RPC_SEND_SIF_FLAG).sw(c, c.g[V1][0]);
    c.g[A0][0] = c.g[S1][0] & DIRTY_RPC_SEND_SYNC_REPLY;
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_WAITERS).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 4);
    dirty_rpc_send_slot(c, DIRTY_RPC_SEND_SYNC_FLAG).sw(c, c.g[A0][0]);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(0x3f));
    c.g[V1][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_MODE).lw(c);
    dirty_rpc_send_slot(c, DIRTY_RPC_SEND_PADDED).sw(c, c.g[A0][0]);
    Addr(r32(c, V0)).sw(c, c.g[V1][0]); // +0 the mode word
    let waiter = dirty_rpc_send_head_entry(c, V1, V0, DIRTY_RPC_WAITERS, 4);
    c.g[A0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_CONTEXT).lw(c);
    waiter.off(4).sw(c, c.g[A0][0]); // +4 the context word
    let waiter = dirty_rpc_send_head_entry(c, V0, V1, DIRTY_RPC_WAITERS, 4);
    waiter.off(8).sw(c, c.g[S1][0]); // +8 the flags
    call_ptr(c, callee::GET_THREAD_ID, 0x003f_4748)?;
    let waiter = dirty_rpc_send_head_entry(c, V1, A0, DIRTY_RPC_WAITERS, 4);
    let no_data = (c.g[S3][0] as i64) <= 0;
    waiter.off(0xc).sw(c, c.g[V0][0]); // +0xc the thread id
    if !no_data {
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            // One block.
            c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_EE_BUFFER).lw(c);
            c.cycles += 4;
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S4)));
            call_ptr(c, callee::MEMCPY, 0x003f_47b4)?;
        } else {
            // The gather pieces, one after the other from EE buffer + s4.
            c.cycles += 4;
            c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_EE_BUFFER).lw(c);
            c.g[V1][0] = Addr(r32(c, S0)).lw(c);
            let empty = (c.g[V1][0] as i64) <= 0;
            c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
            if !empty {
                return Ok(Resume::Go(Stage::CopyGather));
            }
        }
    }
    c.cycles += 1;
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    Ok(Resume::Go(Stage::Describe))
}

/// 0x3f4778: `memcpy(s1, piece.ptr, piece.len)` for each gather piece at s0.
fn dirty_rpc_send_copy_gather(c: &mut Cpu) -> DirtyRpcSendStep {
    loop {
        c.cycles += 9;
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = Addr(r32(c, S0)).lw(c);
        call_ptr(c, callee::MEMCPY, 0x003f_4788)?;
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(8));
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        let more = (c.g[V0][0] as i64) > 0;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
        if !more {
            break;
        }
        if back_edge_unwinds(c, DIRTY_RPC_SEND_COPY_HEAD) {
            return Ok(Resume::Unwind);
        }
    }
    c.cycles += 2;
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    Ok(Resume::Go(DirtyRpcSendStage::Describe))
}

/// 0x3f47b8: the head slot's request descriptor (v1 = the head), the reply
/// buffer sync, the transfer list, the first DMA.
fn dirty_rpc_send_describe(c: &mut Cpu) -> DirtyRpcSendStep {
    c.cycles += 48;
    c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_REQUESTS).lw(c);
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_IOP_BUFFER).lw(c);
    c.g[V1][0] = sx(r32(c, V1) << 5);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S4)));
    Addr(r32(c, V1)).sw(c, c.g[A0][0]); // +0 the data's IOP address
    let req = dirty_rpc_send_head_entry(c, V0, V1, DIRTY_RPC_REQUESTS, 5);
    req.off(4).sw(c, c.g[S3][0]); // +4 the length
    let req = dirty_rpc_send_head_entry(c, V1, V0, DIRTY_RPC_REQUESTS, 5);
    req.off(8).sw(c, c.g[FP][0]); // +8 the reply buffer
    let req = dirty_rpc_send_head_entry(c, V0, V1, DIRTY_RPC_REQUESTS, 5);
    req.off(0xc).sw(c, c.g[S6][0]); // +0xc the reply size
    let req = dirty_rpc_send_head_entry(c, V1, V0, DIRTY_RPC_REQUESTS, 5);
    c.g[A0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_WORD2).lw(c);
    req.off(0x10).sw(c, c.g[A0][0]);
    let req = dirty_rpc_send_head_entry(c, V0, V1, DIRTY_RPC_REQUESTS, 5);
    c.g[V1][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_WORD3).lw(c);
    req.off(0x14).sw(c, c.g[V1][0]);
    let req = dirty_rpc_send_head_entry(c, V1, V0, DIRTY_RPC_REQUESTS, 5);
    c.g[A0][0] = Addr(r32(c, S7).wrapping_add(DIRTY_RPC_CLIENT_ID)).lw(c);
    req.off(0x18).sw(c, c.g[A0][0]);
    let req = dirty_rpc_send_head_entry(c, V0, A0, DIRTY_RPC_REQUESTS, 5);
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_NEXT_ID).lw(c);
    req.off(0x1c).sw(c, c.g[V1][0]); // +0x1c the request id
    c.g[A0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_SYNC_FLAG).lw(c);
    let sync = c.g[A0][0] != 0;
    c.g[A1][0] = sx(r32(c, S6).wrapping_add(0x80));
    if sync {
        c.cycles += 3;
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        call_ptr(c, callee::SYNC_DCACHE, 0x003f_4884)?;
    }
    // The transfers: the data (64-byte padded) and the descriptor.
    c.cycles += 26;
    c.g[V0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_PADDED).lw(c);
    c.g[T0][0] = 0xffff_ffff_ffff_ffc0;
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    c.g[T0][0] = c.g[V0][0] & c.g[T0][0];
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_REQUESTS).lw(c);
    c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_IOP_REQUESTS).lw(c);
    c.g[T1][0] = sx(r32(c, V1) << 5);
    c.g[A2][0] = dirty_rpc_send_state(c, DIRTY_RPC_EE_BUFFER).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T1)));
    c.g[A1][0] = dirty_rpc_send_state(c, DIRTY_RPC_IOP_BUFFER).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T1)));
    c.g[A3][0] = dirty_rpc_send_state(c, DIRTY_RPC_DMA_IDS).lw(c);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, S4)));
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, S4)));
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[T1][0] = 0x20;
    c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, A3))); // the slot's DMA id
    let data = DIRTY_RPC_SEND_DMA_DATA;
    let request = DIRTY_RPC_SEND_DMA_REQUEST;
    dirty_rpc_send_slot(c, data).sw(c, c.g[A2][0]);
    dirty_rpc_send_slot(c, data + 4).sw(c, c.g[A1][0]);
    dirty_rpc_send_slot(c, request).sw(c, c.g[A0][0]);
    dirty_rpc_send_slot(c, request + 4).sw(c, c.g[V0][0]);
    dirty_rpc_send_slot(c, request + 8).sw(c, c.g[T1][0]);
    dirty_rpc_send_slot(c, data + 8).sw(c, c.g[T0][0]);
    dirty_rpc_send_slot(c, data + 0xc).sw(c, 0);
    let with_data = c.g[T0][0] != 0;
    dirty_rpc_send_slot(c, request + 0xc).sw(c, 0);
    if !with_data {
        c.cycles += 2;
        c.g[A0][0] = sx(DIRTY_RPC_SEND_FRAME.at(c, request));
    }
    dirty_rpc_send_dma(c, with_data)
}

/// 0x3f492c / 0x3f4944: `sceSifSetDma` of both transfers (from the frame) or
/// of the descriptor alone (a0), the id into the slot (s0); a 0 id retries
/// after the next tick.
fn dirty_rpc_send_dma(c: &mut Cpu, with_data: bool) -> DirtyRpcSendStep {
    use DirtyRpcSendStage as Stage;
    if with_data {
        c.cycles += 5;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, callee::SCE_SIF_SET_DMA, 0x003f_4938)?;
    } else {
        c.cycles += 3;
        c.g[A1][0] = 1;
        call_ptr(c, callee::SCE_SIF_SET_DMA, 0x003f_494c)?;
    }
    Addr(r32(c, S0)).sw(c, c.g[V0][0]);
    c.cycles += 7;
    let id = dirty_rpc_send_head_entry(c, V0, A0, DIRTY_RPC_DMA_IDS, 2);
    c.g[V1][0] = id.lw(c);
    let queued = c.g[V1][0] != 0;
    c.g[V1][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_SIF_FLAG).lw(c);
    if queued {
        return Ok(Resume::Go(Stage::Finish));
    }
    if back_edge_unwinds(c, DIRTY_RPC_SEND_RETRY_HEAD) {
        return Ok(Resume::Unwind);
    }
    Ok(Resume::Go(Stage::DmaRetry))
}

/// 0x3f48f8: note the current `NetTick` (s0), then wait for it to change.
fn dirty_rpc_send_dma_retry(c: &mut Cpu) -> DirtyRpcSendStep {
    c.cycles += 4;
    callee::NET_TICK.call(c, 0x003f_4900)?;
    c.g[S0][0] = c.g[V0][0];
    dirty_rpc_send_tick_wait(c)
}

/// 0x3f4908: spin on `NetTick` until it differs from s0, then queue the DMA
/// again (both transfers if there is data).
fn dirty_rpc_send_tick_wait(c: &mut Cpu) -> DirtyRpcSendStep {
    loop {
        c.cycles += 4;
        callee::NET_TICK.call(c, 0x003f_4910)?;
        let same = c.g[S0][0] == c.g[V0][0];
        c.g[V1][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_DMA_DATA + 8).lw(c);
        if !same {
            break;
        }
        if back_edge_unwinds(c, DIRTY_RPC_SEND_TICK_HEAD) {
            return Ok(Resume::Unwind);
        }
    }
    c.cycles += 5;
    c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_DMA_IDS).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    let with_data = c.g[V1][0] != 0;
    c.g[S0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    if !with_data {
        c.cycles += 1;
        c.g[A0][0] = sx(DIRTY_RPC_SEND_FRAME.at(c, DIRTY_RPC_SEND_DMA_REQUEST));
    }
    dirty_rpc_send_dma(c, with_data)
}

/// 0x3f496c: the SIF command (v1 = flag 1), the write offset, the next id
/// and ring head, `SignalSema`, then the blocking wait for mode 1.
fn dirty_rpc_send_finish(c: &mut Cpu) -> DirtyRpcSendStep {
    c.cycles += 2;
    let sif_cmd = c.g[V1][0] != 0;
    c.g[A1][0] = sx(DIRTY_RPC_SIF_PACKET & 0xffff_0000);
    if sif_cmd {
        c.cycles += 7;
        c.g[A0][0] = 1;
        c.g[A2][0] = Addr(DIRTY_RPC_SIF_PACKET).lbu(c);
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(DIRTY_RPC_SIF_PACKET);
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        call_ptr(c, callee::SCE_SIF_SEND_CMD, 0x003f_4990)?;
    }
    c.cycles += 2;
    let has_data = (c.g[S3][0] as i64) > 0;
    c.g[S0][0] = dirty_rpc_send_state(c, DIRTY_RPC_NEXT_ID).lw(c); // the result
    if has_data {
        c.cycles += 5;
        c.g[A0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_PADDED).lw(c);
        c.g[V0][0] = 0xffff_ffff_ffff_ffc0;
        c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        dirty_rpc_send_state(c, DIRTY_RPC_WRITE).sw(c, c.g[V0][0]);
    }
    c.cycles += 6;
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_NEXT_ID).lw(c);
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_ID_LIMIT).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
    c.div(0, A0, V1);
    c.cycles += 13;
    c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_SLOTS).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    c.g[A2][0] = c.hi[0];
    c.div(0, V0, V1);
    dirty_rpc_send_state(c, DIRTY_RPC_NEXT_ID).sw(c, c.g[A2][0]);
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
    c.g[A1][0] = c.hi[0];
    dirty_rpc_send_state(c, DIRTY_RPC_HEAD).sw(c, c.g[A1][0]);
    call_ptr(c, callee::SIGNAL_SEMA, 0x003f_49ec)?;
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_ALARM).lw(c);
    let alarm_running = (c.g[V1][0] as i64) >= 0;
    c.g[A0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_MODE).lw(c);
    if !alarm_running {
        c.cycles += 4;
        c.g[V1][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_MODE).lw(c);
        c.g[V0][0] = 1;
        let blocking = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = c.g[S0][0];
        if !blocking {
            return Ok(dirty_rpc_send_leave(c));
        }
        c.cycles += 7;
        c.g[A0][0] = DIRTY_RPC_ALARM_TICKS;
        c.g[A1][0] = sx(DIRTY_RPC_ALARM_HANDLER);
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, callee::SET_ALARM, 0x003f_4a1c)?;
        dirty_rpc_send_state(c, DIRTY_RPC_ALARM).sw(c, c.g[V0][0]);
        c.g[A0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_MODE).lw(c);
    }
    c.cycles += 3;
    c.g[V0][0] = 1;
    let blocking = c.g[A0][0] == c.g[V0][0];
    c.g[V0][0] = c.g[S0][0];
    if blocking {
        c.cycles += 6;
        c.g[V0][0] = 2;
        dirty_rpc_send_gp(c, DIRTY_RPC_SLEEP_GP).sw(c, c.g[V0][0]);
        call_ptr(c, callee::SLEEP_THREAD, 0x003f_4a40)?;
        dirty_rpc_send_gp(c, DIRTY_RPC_SLEEP_GP).sw(c, 0);
        c.g[V0][0] = c.g[S0][0];
    }
    Ok(dirty_rpc_send_leave(c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{
        fall_back, hook, machine, prev_cell, LOG_COUNT, LOG_START, LOG_UNWIND_AT,
    };
    use crate::difftest::{
        check_lohi, elf_image, fill, fresh, run_generated, Rng, CYCLE_BUDGET, DATA,
    };
    use std::collections::BTreeSet;
    use std::ptr::addr_of_mut;

    // ------------------------------------------------------- DirtyRpc_Send

    // The fixture's state tables, buffers and data (all pre-filled with junk).
    pub(crate) const DIRTY_RPC_SEND_T_CLIENT: u32 = DATA + 0x100;
    pub(crate) const DIRTY_RPC_SEND_T_REQUESTS: u32 = DATA + 0x1000;
    pub(crate) const DIRTY_RPC_SEND_T_REPLIES: u32 = DATA + 0x1400;
    pub(crate) const DIRTY_RPC_SEND_T_DMA_IDS: u32 = DATA + 0x1800;
    pub(crate) const DIRTY_RPC_SEND_T_WAITERS: u32 = DATA + 0x1900;
    pub(crate) const DIRTY_RPC_SEND_T_EE_BUFFER: u32 = DATA + 0x4000;
    pub(crate) const DIRTY_RPC_SEND_T_BUFFER_SIZE: u32 = 0x800;
    pub(crate) const DIRTY_RPC_SEND_T_REPLY: u32 = DATA + 0x6000;
    pub(crate) const DIRTY_RPC_SEND_T_DATA: u32 = DATA + 0x8000;
    pub(crate) const DIRTY_RPC_SEND_T_GATHER: u32 = DATA + 0x9000;
    pub(crate) const DIRTY_RPC_SEND_T_PIECES: u32 = DATA + 0xa000;
    /// IOP-side addresses (only stored, never dereferenced on the EE).
    pub(crate) const DIRTY_RPC_SEND_T_IOP_BUFFER: u32 = 0x000a_0000;
    pub(crate) const DIRTY_RPC_SEND_T_IOP_REQUESTS: u32 = 0x000b_8000;

    /// The gated kernel-call fakes' control block (in the test machine's RAM;
    /// the magic word turns them on).
    pub(crate) const DIRTY_RPC_SEND_T_CTRL: u32 = DATA + 0xf_0000;
    pub(crate) const DIRTY_RPC_SEND_T_MAGIC: u32 = 0xd1f7_5e4d;
    /// Control words: WaitSema calls so far; the WaitSema call during which
    /// the ring tail's reply lands, where and what; the WaitSema call after
    /// which an event falls due; sceSifSetDma calls so far, how many of the
    /// first ones return id 0, the call after which an event falls due and
    /// how many cycles later; an event due after GetThreadId; SyncDCache
    /// calls so far.
    const DIRTY_RPC_SEND_T_WAITS: u32 = 0x04;
    const DIRTY_RPC_SEND_T_LAND_AT: u32 = 0x08;
    const DIRTY_RPC_SEND_T_LAND_ADDR: u32 = 0x0c;
    const DIRTY_RPC_SEND_T_LAND_VALUE: u32 = 0x10;
    const DIRTY_RPC_SEND_T_WAIT_ARM_AT: u32 = 0x14;
    const DIRTY_RPC_SEND_T_DMAS: u32 = 0x18;
    const DIRTY_RPC_SEND_T_DMA_ZEROS: u32 = 0x1c;
    const DIRTY_RPC_SEND_T_DMA_ARM_AT: u32 = 0x20;
    const DIRTY_RPC_SEND_T_DMA_ARM_DELAY: u32 = 0x24;
    const DIRTY_RPC_SEND_T_TID_ARM: u32 = 0x28;
    const DIRTY_RPC_SEND_T_SYNCS: u32 = 0x2c;

    /// The `syscall` sites of the kernel wrappers (the fake log's targets).
    const DIRTY_RPC_SEND_T_WAIT_SYSCALL: u32 = 0x0042_3de4;
    const DIRTY_RPC_SEND_T_THREAD_SYSCALL: u32 = 0x0042_3c94;
    const DIRTY_RPC_SEND_T_DMA_SYSCALL: u32 = 0x0042_4164;
    const DIRTY_RPC_SEND_T_ALARM_SYSCALL: u32 = 0x0042_3b24;
    const DIRTY_RPC_SEND_T_SLEEP_SYSCALL: u32 = 0x0042_3cc4;

    fn dirty_rpc_send_t_gated(c: &mut Cpu) -> bool {
        c.bus.read32(DIRTY_RPC_SEND_T_CTRL) == DIRTY_RPC_SEND_T_MAGIC
    }

    fn dirty_rpc_send_t_get(c: &mut Cpu, word: u32) -> u32 {
        c.bus.read32(DIRTY_RPC_SEND_T_CTRL + word)
    }

    fn dirty_rpc_send_t_set(c: &mut Cpu, word: u32, v: u32) {
        c.bus.write32(DIRTY_RPC_SEND_T_CTRL + word, v);
    }

    /// Count a call; returns its index.
    fn dirty_rpc_send_t_bump(c: &mut Cpu, word: u32) -> u32 {
        let n = dirty_rpc_send_t_get(c, word);
        dirty_rpc_send_t_set(c, word, n + 1);
        n
    }

    /// `WaitSema` for the fixture: the real wrapper, then (on the chosen
    /// calls) the IOP's reply for the ring tail lands and an event falls due.
    fn dirty_rpc_send_t_wait_sema(c: &mut Cpu) {
        let prev = prev_cell(dirty_rpc_send_t_wait_sema);
        if !dirty_rpc_send_t_gated(c) {
            return fall_back(c, prev, ee::s15::f_00423de0__generated);
        }
        let ra = r32(c, RA);
        let n = dirty_rpc_send_t_bump(c, DIRTY_RPC_SEND_T_WAITS);
        fall_back(c, prev, ee::s15::f_00423de0__generated);
        if c.pc != ra {
            return;
        }
        if n == dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_LAND_AT) {
            let at = dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_LAND_ADDR);
            let v = dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_LAND_VALUE);
            c.bus.write32(at, v);
        }
        if n == dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_WAIT_ARM_AT) {
            c.next_event = c.cycles;
        }
    }

    /// `sceSifSetDma` for the fixture: the real wrapper, id 0 for the first
    /// calls, and an event due after the chosen call.
    fn dirty_rpc_send_t_set_dma(c: &mut Cpu) {
        let prev = prev_cell(dirty_rpc_send_t_set_dma);
        if !dirty_rpc_send_t_gated(c) {
            return fall_back(c, prev, ee::s15::f_00424160__generated);
        }
        let ra = r32(c, RA);
        let n = dirty_rpc_send_t_bump(c, DIRTY_RPC_SEND_T_DMAS);
        fall_back(c, prev, ee::s15::f_00424160__generated);
        if c.pc != ra {
            return;
        }
        if n < dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_DMA_ZEROS) {
            c.g[V0][0] = 0;
        }
        if n == dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_DMA_ARM_AT) {
            let delay = dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_DMA_ARM_DELAY);
            c.next_event = c.cycles + delay as u64;
        }
    }

    /// `GetThreadId` for the fixture: the real wrapper, then optionally an
    /// event falls due.
    fn dirty_rpc_send_t_thread_id(c: &mut Cpu) {
        let prev = prev_cell(dirty_rpc_send_t_thread_id);
        if !dirty_rpc_send_t_gated(c) {
            return fall_back(c, prev, ee::s15::f_00423c90__generated);
        }
        let ra = r32(c, RA);
        fall_back(c, prev, ee::s15::f_00423c90__generated);
        if c.pc == ra && dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_TID_ARM) != 0 {
            c.next_event = c.cycles;
        }
    }

    /// `SyncDCache` for the fixture: counts the calls (it leaves no trace in
    /// RAM otherwise).
    fn dirty_rpc_send_t_sync_dcache(c: &mut Cpu) {
        let prev = prev_cell(dirty_rpc_send_t_sync_dcache);
        if dirty_rpc_send_t_gated(c) {
            dirty_rpc_send_t_bump(c, DIRTY_RPC_SEND_T_SYNCS);
        }
        fall_back(c, prev, ee::s15::f_00424698__generated)
    }

    pub(crate) fn dirty_rpc_send_t_hook() {
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423DE0),
            dirty_rpc_send_t_wait_sema,
            prev_cell(dirty_rpc_send_t_wait_sema),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00424160),
            dirty_rpc_send_t_set_dma,
            prev_cell(dirty_rpc_send_t_set_dma),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423C90),
            dirty_rpc_send_t_thread_id,
            prev_cell(dirty_rpc_send_t_thread_id),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00424698),
            dirty_rpc_send_t_sync_dcache,
            prev_cell(dirty_rpc_send_t_sync_dcache),
        );
    }

    /// The steering of one `DirtyRpc_Send` fixture.
    #[derive(Clone, Copy)]
    pub(crate) struct DirtyRpcSendPlan {
        /// 0 no client, 1 unbound client (+0xc = 0), 2 unaligned reply buffer,
        /// 3 a string (`len < 0`), 4 and 9 bytes, 5 no bytes, 6 a gather list,
        /// 7 a gather list with `len < 0` (strlen runs first), 8 an empty one.
        shape: u32,
        /// 0 room behind the read offset, 1 the same one byte short, 2 room at
        /// the end, 3 room only from 0, 4 the same one byte short.
        room: u32,
        /// 0: a free slot; else the ring is full and the tail's reply lands
        /// during this WaitSema call (1: one wait round, 2/3: more).
        land: u32,
        /// Gather pieces (1-4), all one byte long if `unit_pieces`.
        pieces: u32,
        unit_pieces: bool,
        sync: bool,
        sif_cmd: bool,
        /// sceSifSetDma calls that return id 0 first.
        dma_zeros: u32,
        /// The mode word (1 blocks).
        mode: u32,
        alarm_running: bool,
    }

    /// The seed digits of `dirty_rpc_send_matches`.
    fn dirty_rpc_send_t_plan(seed: u32) -> DirtyRpcSendPlan {
        DirtyRpcSendPlan {
            shape: seed % 10,
            room: seed / 10 % 5,
            land: [0, 1, 2 + seed % 2][(seed / 50 % 3) as usize],
            pieces: 1 + seed % 4,
            unit_pieces: false,
            sync: seed % 3 == 1,
            sif_cmd: seed % 7 >= 3,
            dma_zeros: seed % 11 % 3,
            mode: [0, 1, 5][(seed % 13 % 3) as usize],
            alarm_running: seed % 17 % 2 == 1,
        }
    }

    /// What a fixture chose, to read the outcome against.
    pub(crate) struct DirtyRpcSendCase {
        plan: DirtyRpcSendPlan,
        /// The length the function arrives at.
        len: u32,
        write: u32,
        head: u32,
        id: u32,
        /// The bytes it copies, in order.
        source: Vec<(u32, u32)>,
    }

    /// `NetTick`'s clock: the next millisecond starts `lead` cycles from now.
    pub(crate) fn dirty_rpc_send_t_clock(c: &mut Cpu, r: &mut Rng, lead: u32) {
        c.bus
            .write32(DIRTY_RPC_STATE + DIRTY_RPC_CLOCK_COUNT, c.cycles as u32);
        let base = (1000 + (r.next_u32() % 1000) as u64) * 0x48000 + 0x48000 - lead as u64;
        c.bus.write64(DIRTY_RPC_STATE + DIRTY_RPC_CLOCK_BASE, base);
    }

    /// A DirtyRpc state block with junk tables, a ring of 2-7 slots, data,
    /// room and arguments as `plan` says, the fakes' control block, and the
    /// game image (the SIF packet's size byte).
    pub(crate) fn dirty_rpc_send_setup(
        c: &mut Cpu,
        seed: u32,
        calls: u32,
        plan: DirtyRpcSendPlan,
    ) -> DirtyRpcSendCase {
        let mut r = machine(c, seed, 0x3f_4528, calls);
        elf_image(c);
        c.pc = DIRTY_RPC_SEND;
        fill(c, &mut r, DATA, 0x4000);
        let slots = 2 + r.next_u32() % 6;
        let head = r.next_u32() % slots;
        let tail = if plan.land == 0 { head } else { (head + 1) % slots };
        let limit = 0x100 + r.next_u32() % 0x100;
        let id = r.next_u32() % limit;
        // No reply has arrived for any slot.
        for i in 0..slots {
            let seq = c.bus.read32(DIRTY_RPC_SEND_T_REQUESTS + 0x20 * i + 0x1c);
            c.bus
                .write32(DIRTY_RPC_SEND_T_REPLIES + 0x20 * i + 0x1c, seq ^ 0x5a5a_0001);
        }
        if plan.land != 0 {
            // The tail's request: no reply callback, no data (the read offset
            // stays where the room plan puts it).
            c.bus.write32(DIRTY_RPC_SEND_T_WAITERS + 0x10 * tail, 0);
            c.bus.write32(DIRTY_RPC_SEND_T_REQUESTS + 0x20 * tail + 4, 0);
        }

        // The data: (pointer, len argument, the length the code computes).
        let mut source = Vec::new();
        let (data, arg_len, len) = match plan.shape {
            3 => {
                let l = 5 + r.next_u32() % 24;
                for i in 0..l {
                    c.bus.write8(DIRTY_RPC_SEND_T_DATA + i, b'a' + (i % 26) as u8);
                }
                c.bus.write8(DIRTY_RPC_SEND_T_DATA + l, 0);
                source.push((DIRTY_RPC_SEND_T_DATA, l + 1));
                let arg = -(1 + (r.next_u32() % 0x100) as i32);
                (DIRTY_RPC_SEND_T_DATA, arg, l + 1)
            }
            5 => (DIRTY_RPC_SEND_T_DATA, 0, 0),
            6..=8 => {
                let n = if plan.shape == 8 { 0 } else { plan.pieces };
                let mut total = 0;
                for i in 0..n {
                    let l = if plan.unit_pieces {
                        1
                    } else {
                        1 + r.next_u32() % 0x30
                    };
                    let p = DIRTY_RPC_SEND_T_PIECES + 0x100 * i;
                    c.bus.write32(DIRTY_RPC_SEND_T_GATHER + 8 * i, l);
                    c.bus.write32(DIRTY_RPC_SEND_T_GATHER + 8 * i + 4, p);
                    source.push((p, l));
                    total += l;
                }
                let end = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    (-(1 + (r.next_u32() % 8) as i32)) as u32
                };
                c.bus.write32(DIRTY_RPC_SEND_T_GATHER + 8 * n, end);
                let arg = if plan.shape == 7 {
                    -(1 + (r.next_u32() % 8) as i32)
                } else {
                    (r.next_u32() % 0x100) as i32
                };
                (DIRTY_RPC_SEND_T_GATHER, arg, total)
            }
            _ => {
                let l = 1 + r.next_u32() % 0x90;
                source.push((DIRTY_RPC_SEND_T_DATA, l));
                (DIRTY_RPC_SEND_T_DATA, l as i32, l)
            }
        };

        // Write and read offsets for the room plan.
        let size = DIRTY_RPC_SEND_T_BUFFER_SIZE;
        let (write, read) = match plan.room {
            0 => {
                let w = 0x100 + (r.next_u32() % 0x10) * 0x10;
                (w, w + 0x40 + len + (r.next_u32() % 3) * 0x10)
            }
            1 => {
                let w = 0x100 + (r.next_u32() % 0x10) * 0x10;
                (w, w + 0x3f + len)
            }
            2 => (
                size - 0x40 - len - (r.next_u32() % 4) * 0x10,
                0x80 + (r.next_u32() % 8) * 0x10,
            ),
            3 => (
                size - 0x3f - len + r.next_u32() % 0x30,
                len + 0x40 + (r.next_u32() % 4) * 0x10,
            ),
            _ => (size - 0x3f - len + r.next_u32() % 0x30, len + 0x3f),
        };

        let alarm = if plan.alarm_running {
            r.next_u32() % 0x40
        } else {
            0x8000_0000 | r.next_u32()
        };
        let sema = 1 + r.next_u32() % 0x40;
        for (off, v) in [
            (DIRTY_RPC_ID_LIMIT, limit),
            (DIRTY_RPC_REQUESTS, DIRTY_RPC_SEND_T_REQUESTS),
            (DIRTY_RPC_IOP_REQUESTS, DIRTY_RPC_SEND_T_IOP_REQUESTS),
            (DIRTY_RPC_REPLIES, DIRTY_RPC_SEND_T_REPLIES),
            (DIRTY_RPC_EE_BUFFER, DIRTY_RPC_SEND_T_EE_BUFFER),
            (DIRTY_RPC_IOP_BUFFER, DIRTY_RPC_SEND_T_IOP_BUFFER),
            (DIRTY_RPC_DMA_IDS, DIRTY_RPC_SEND_T_DMA_IDS),
            (DIRTY_RPC_WAITERS, DIRTY_RPC_SEND_T_WAITERS),
            (DIRTY_RPC_NEXT_ID, id),
            (DIRTY_RPC_HEAD, head),
            (DIRTY_RPC_TAIL, tail),
            (DIRTY_RPC_SLOTS, slots),
            (DIRTY_RPC_WRITE, write),
            (DIRTY_RPC_READ, read),
            (DIRTY_RPC_BUFFER_SIZE, size),
            (DIRTY_RPC_ALARM, alarm),
            (DIRTY_RPC_SEMA, sema),
        ] {
            c.bus.write32(DIRTY_RPC_STATE + off, v);
        }

        // Arguments.
        let bound = if plan.shape == 1 { 0 } else { r.next_u32() | 1 };
        c.bus
            .write32(DIRTY_RPC_SEND_T_CLIENT + DIRTY_RPC_CLIENT_ID, bound);
        c.g[A0][0] = if plan.shape == 0 {
            0
        } else {
            sx(DIRTY_RPC_SEND_T_CLIENT)
        };
        let mut flags = r.next_u32() & !7;
        if matches!(plan.shape, 6..=8) {
            flags |= DIRTY_RPC_SEND_GATHER as u32;
        }
        if plan.sif_cmd {
            flags |= DIRTY_RPC_SEND_SIF_CMD as u32;
        }
        if plan.sync {
            flags |= DIRTY_RPC_SEND_SYNC_REPLY as u32;
        }
        c.g[A1][0] = sx(flags);
        c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        c.g[A3][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        c.g[T0][0] = sx(data);
        c.g[T1][0] = sx(arg_len as u32);
        let reply = if plan.shape == 2 {
            DIRTY_RPC_SEND_T_REPLY + 1 + r.next_u32() % 15
        } else {
            DIRTY_RPC_SEND_T_REPLY
        };
        c.g[T2][0] = sx(reply);
        c.g[T3][0] = sx(0x40 + r.next_u32() % 0x100);
        let stack = r32(c, SP);
        c.bus.write32(stack, plan.mode);
        c.bus.write32(stack + 8, r.next_u32());

        // The fakes.
        let land_at = if plan.land == 0 { u32::MAX } else { plan.land };
        let tail_id = c.bus.read32(DIRTY_RPC_SEND_T_REQUESTS + 0x20 * tail + 0x1c);
        for (word, v) in [
            (0, DIRTY_RPC_SEND_T_MAGIC),
            (DIRTY_RPC_SEND_T_WAITS, 0),
            (DIRTY_RPC_SEND_T_LAND_AT, land_at),
            (
                DIRTY_RPC_SEND_T_LAND_ADDR,
                DIRTY_RPC_SEND_T_REPLIES + 0x20 * tail + 0x1c,
            ),
            (DIRTY_RPC_SEND_T_LAND_VALUE, tail_id),
            (DIRTY_RPC_SEND_T_WAIT_ARM_AT, u32::MAX),
            (DIRTY_RPC_SEND_T_DMAS, 0),
            (DIRTY_RPC_SEND_T_DMA_ZEROS, plan.dma_zeros),
            (DIRTY_RPC_SEND_T_DMA_ARM_AT, u32::MAX),
            (DIRTY_RPC_SEND_T_DMA_ARM_DELAY, 0),
            (DIRTY_RPC_SEND_T_TID_ARM, 0),
            (DIRTY_RPC_SEND_T_SYNCS, 0),
        ] {
            dirty_rpc_send_t_set(c, word, v);
        }
        dirty_rpc_send_t_clock(c, &mut r, 12_000);
        DirtyRpcSendCase {
            plan,
            len,
            write,
            head,
            id,
            source,
        }
    }

    /// The arms a finished generated run took, from its fake-call log, the
    /// request descriptor, the data buffer and the SIF packet.
    fn dirty_rpc_send_t_arms(c: &mut Cpu, case: &DirtyRpcSendCase, arms: &mut BTreeSet<&'static str>) {
        let plan = case.plan;
        let frame = crate::difftest::SP - 0xe0;
        let (mut waits, mut thread, mut alarm, mut sleep) = (0, false, false, false);
        let mut dmas = Vec::new();
        let calls = c.bus.read32(LOG_COUNT);
        for i in 0..calls.min(64) {
            let p = LOG_START + 0x220 * i;
            let target = c.bus.read32(p);
            let a0 = c.bus.read32(p + 0x10 + 16 * A0 as u32);
            let a1 = c.bus.read32(p + 0x10 + 16 * A1 as u32);
            match target {
                DIRTY_RPC_SEND_T_WAIT_SYSCALL if !thread => waits += 1,
                DIRTY_RPC_SEND_T_THREAD_SYSCALL => thread = true,
                DIRTY_RPC_SEND_T_DMA_SYSCALL if a0 == frame || a0 == frame + 0x10 => dmas.push(a1),
                DIRTY_RPC_SEND_T_ALARM_SYSCALL => alarm = true,
                DIRTY_RPC_SEND_T_SLEEP_SYSCALL => sleep = true,
                _ => {}
            }
        }
        let v0 = c.g[V0][0];
        if v0 == u64::MAX && waits >= 1 && !thread {
            match plan.room {
                1 => arms.insert("no room behind the reader"),
                4 => arms.insert("no room after wrapping"),
                _ => false,
            };
        }
        if v0 != case.id as u64 || !thread {
            return;
        }
        let req = DIRTY_RPC_SEND_T_REQUESTS + 0x20 * case.head;
        let at = c.bus.read32(req).wrapping_sub(DIRTY_RPC_SEND_T_IOP_BUFFER);
        let len = c.bus.read32(req + 4);
        match plan.room {
            0 if at == case.write => arms.insert("room behind the reader"),
            2 if at == case.write => arms.insert("room at the end"),
            3 if at == 0 => arms.insert("room after wrapping"),
            _ => false,
        };
        match waits {
            1 => arms.insert("free slot"),
            4 => arms.insert("full ring, one wait"),
            n if n >= 5 => arms.insert("full ring, several waits"),
            _ => false,
        };
        if len == case.len {
            match plan.shape {
                3 => arms.insert("length from strlen"),
                7 => arms.insert("gather after strlen"),
                5 | 8 => arms.insert("no data"),
                _ => false,
            };
        }
        let dst = DIRTY_RPC_SEND_T_EE_BUFFER + at;
        let mut off = 0;
        let mut copied = !case.source.is_empty();
        for &(p, l) in &case.source {
            for k in 0..l {
                if c.bus.read8(dst + off + k) != c.bus.read8(p + k) {
                    copied = false;
                }
            }
            off += l;
        }
        if copied {
            arms.insert(match (plan.shape, case.source.len()) {
                (6 | 7, 1) => "gather copy, one piece",
                (6 | 7, _) => "gather copy loop",
                _ => "flat copy",
            });
        }
        if dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_SYNCS) != 0 {
            arms.insert("reply buffer synced");
        }
        match dmas.first().copied() {
            Some(2) => arms.insert("data and descriptor DMA"),
            Some(1) => arms.insert("descriptor-only DMA"),
            _ => false,
        };
        match dmas.get(1).copied() {
            Some(2) => arms.insert("retried data and descriptor"),
            Some(1) => arms.insert("retried descriptor only"),
            _ => false,
        };
        if c.bus.read32(DIRTY_RPC_SIF_PACKET + 8) == 1 {
            arms.insert("SIF command");
        }
        match (alarm, sleep, plan.alarm_running) {
            (true, true, false) => arms.insert("alarm started, slept"),
            (false, true, true) => arms.insert("slept, alarm running"),
            (false, false, true) => arms.insert("no wait, alarm running"),
            (false, false, false) => arms.insert("no wait, no alarm"),
            _ => false,
        };
    }

    /// Seed digits: `% 10` the client and data shape, `/ 10 % 5` the buffer
    /// room, `/ 50 % 3` the ring (free, full for one wait round, for more);
    /// `% 3` the sync flag, `% 7` the SIF-command flag, `% 11 % 3` the zero
    /// DMA ids before success, `% 13 % 3` the mode (0, 1, 5), `% 17 % 2` a
    /// running alarm. The fakes unwind at one of the first eight kernel calls
    /// in about a third of the seeds. A second pass (no unwinds) checks that
    /// the generated body took every arm: the early returns by their fixed
    /// cycle totals, the rest by their effects.
    #[test]
    fn dirty_rpc_send_matches() {
        const SEEDS: u32 = 150;
        // No client, unbound client, unaligned reply buffer.
        const EARLY: [u64; 3] = [36, 39, 43];
        const ARMS: &[&str] = &[
            "no room behind the reader",
            "no room after wrapping",
            "room behind the reader",
            "room at the end",
            "room after wrapping",
            "free slot",
            "full ring, one wait",
            "full ring, several waits",
            "length from strlen",
            "gather after strlen",
            "no data",
            "flat copy",
            "gather copy, one piece",
            "gather copy loop",
            "reply buffer synced",
            "data and descriptor DMA",
            "descriptor-only DMA",
            "retried data and descriptor",
            "retried descriptor only",
            "SIF command",
            "alarm started, slept",
            "slept, alarm running",
            "no wait, alarm running",
            "no wait, no alarm",
        ];
        dirty_rpc_send_t_hook();
        check_lohi(ee::s14::f_003f4528__generated, dirty_rpc_send, SEEDS, &|c, seed| {
            dirty_rpc_send_setup(c, seed, 8, dirty_rpc_send_t_plan(seed));
        });
        let mut totals = BTreeSet::new();
        let mut arms = BTreeSet::new();
        for seed in 0..SEEDS {
            let mut c = fresh();
            let case = dirty_rpc_send_setup(&mut c, seed, 8, dirty_rpc_send_t_plan(seed));
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            let before = c.cycles;
            run_generated(&mut c, ee::s14::f_003f4528__generated);
            totals.insert(c.cycles - before);
            dirty_rpc_send_t_arms(&mut c, &case, &mut arms);
        }
        for p in EARLY {
            assert!(totals.contains(&p), "early return costing {p} cycles never taken");
        }
        for a in ARMS {
            assert!(arms.contains(a), "arm {a:?} never taken (taken: {arms:?})");
        }
    }

    /// One loop head per `seed / 3 % 5` (`seed % 3`: no event, an unwinding
    /// VBlank, a delivered event). 0x3f45e0: four gather pieces, the
    /// scheduled event (due within 48 cycles, before the third back edge).
    /// 0x3f4648: a full ring, the event due after the first WaitSema in the
    /// wait loop. 0x3f4778: three one-byte gather pieces (memcpy makes no
    /// event check), the event due after GetThreadId. 0x3f48f8: the first DMA
    /// id 0, the event due at once. 0x3f4908: the same, due 8-39 cycles later
    /// (past the retry back edge). The first unwind lands on head `h` at
    /// seeds `15k + 3h + 1`.
    #[test]
    fn dirty_rpc_send_events() {
        driven!(dirty_rpc_send_t_gen, ee::s14::f_003f4528__generated);
        driven!(dirty_rpc_send_t_hand, dirty_rpc_send);
        dirty_rpc_send_t_hook();
        check_events(dirty_rpc_send_t_gen, dirty_rpc_send_t_hand, 60, &|c, seed| {
            let head = seed / 3 % 5;
            let plan = DirtyRpcSendPlan {
                shape: if head == 0 || head == 2 { 6 } else { 4 },
                room: 0,
                land: (head == 1) as u32,
                pieces: if head == 0 { 4 } else { 3 },
                unit_pieces: head == 2,
                sync: false,
                sif_cmd: false,
                dma_zeros: (head >= 3) as u32,
                mode: 0,
                alarm_running: true,
            };
            dirty_rpc_send_setup(c, seed, 0, plan);
            let mut r = Rng(0x3f_4528 ^ (seed as u64) << 8);
            let mode = schedule(c, &mut r, seed, DIRTY_RPC_SEND);
            dirty_rpc_send_t_clock(c, &mut r, if head == 4 { 40_000 } else { 12_000 });
            if mode != Events::Off && head != 0 {
                c.next_event = CYCLE_BUDGET;
                match head {
                    1 => dirty_rpc_send_t_set(c, DIRTY_RPC_SEND_T_WAIT_ARM_AT, 1),
                    2 => dirty_rpc_send_t_set(c, DIRTY_RPC_SEND_T_TID_ARM, 1),
                    3 => dirty_rpc_send_t_set(c, DIRTY_RPC_SEND_T_DMA_ARM_AT, 0),
                    _ => {
                        dirty_rpc_send_t_set(c, DIRTY_RPC_SEND_T_DMA_ARM_AT, 0);
                        dirty_rpc_send_t_set(c, DIRTY_RPC_SEND_T_DMA_ARM_DELAY, 8 + seed / 15 % 32);
                    }
                }
            }
            mode
        });
    }
}
