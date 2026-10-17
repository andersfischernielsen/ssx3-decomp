//! Work package E1: the EA SND sound library core
//! (`docs/investigation/phase2-candidates-3.md`, names from
//! `docs/investigation/naming.md` pass 3).
//!
//! The EE side of EA's SND driver as SSX 3 links it:
//!
//! - the sound thread (`SND_EeThread`) and the 100 Hz driver tick
//!   (`SND_Tick`: tick callbacks, per-channel LFOs, envelopes and timed
//!   stops), voice pitch (`SND_ChannelApplyPitch`), stopping and releasing
//!   channels, handle queries;
//! - the driver's block allocator (`SND_MemAlloc`/`SND_MemFree`, a sorted
//!   table of `{offset, size}` blocks) and its quadword `SND_MemCopy`;
//! - the software mixer (`SND_EeMix`), the DSP chain helpers and the
//!   clip-to-16-bit stage;
//! - the sample decoders: EA-XA ADPCM (`SND_EaxaDecode`,
//!   `SND_DecEaxa_DecodeStream`) and the UTK (MicroTalk) speech codec (bit
//!   reader, excitation, reflection coefficients to LPC, interpolation,
//!   synthesis, the decode loops);
//! - the SIF command handler and the VU0 context saver the sound thread
//!   uses around its work.
//!
//! Every replacement charges the EE cycles block by block along the path it
//! takes, makes the same calls with the same registers (the call protocol
//! of [`crate::call`]), checks events at the same back edges (the shapes in
//! [`crate::events`]) and leaves the same incidental register values (lines
//! marked "leftover"). The driver's state block comes from
//! [`crate::sound::snd`] (package A5).

use crate::abi::{self, addu, reg, set_reg};
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use crate::sound::snd;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_8ab8, snd_is_handle_done, "SND_IsHandleDone"),
    (0x003c_d1f8, snd_utk_read_bits, "SND_UtkReadBits"),
    (0x003c_d260, snd_utk_skip_bits, "SND_UtkSkipBits"),
    (0x003c_d518, snd_utk_interpolate, "SND_UtkInterpolate"),
    (0x003c_d590, snd_utk_rc_to_lpc, "SND_UtkRcToLpc"),
    (0x003c_9360, snd_dec_utk_decode_mem, "SND_DecUtk_DecodeMem"),
    (0x003c_0858, snd_save_vu0_context, "SND_SaveVu0Context"),
    (0x003c_d690, snd_utk_synthesize, "SND_UtkSynthesize"),
    (0x003c_1578, snd_sif_cmd_handler, "SND_SifCmdHandler"),
    (0x003b_89e0, snd_stop, "SND_Stop"),
    (0x003c_8f48, snd_dsp_insert_sorted, "SND_DspInsertSorted"),
    (0x003b_5e98, snd_mem_free, "SND_MemFree"),
    (0x003b_5c60, snd_mem_alloc, "SND_MemAlloc"),
    (0x003c_51e8, snd_mem_copy, "SND_MemCopy"),
    (0x003b_a550, snd_channel_release, "SND_ChannelRelease"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const SND_HANDLE_TO_CHANNEL: Callee = Callee { addr: 0x003b_a6e8, f: ee::s13::SND_HandleToChannel };
    pub const SND_NEXT_HANDLE_CHANNEL: Callee = Callee { addr: 0x003b_aa00, f: ee::s13::SND_NextHandleChannel };
    pub const SND_CHANNEL_STOP_VOICES: Callee = Callee { addr: 0x003c_2440, f: ee::s13::SND_ChannelStopVoices };
    pub const SND_SIF_DMA_COMPLETE: Callee = Callee { addr: 0x003c_4918, f: ee::s13::SND_SifDmaComplete };
    pub const SND_MEM_CLAMP_BLOCK: Callee = Callee { addr: 0x003b_5ba0, f: ee::s13::SND_MemClampBlock };
    pub const SND_UTK_RC_TO_LPC: Callee = Callee { addr: 0x003c_d590, f: ee::s14::SND_UtkRcToLpc };
    pub const SND_UTK_LPC_FILTER: Callee = Callee { addr: 0x003c_e410, f: ee::s14::SND_UtkLpcFilter };
    pub const SND_UTK_DECODE: Callee = Callee { addr: 0x003c_de68, f: ee::s14::SND_UtkDecode };
    pub const SND_UTK_SET_STATE: Callee = Callee { addr: 0x003c_e0b8, f: ee::s14::SND_UtkSetState };
    pub const SND_UTK_SET_INPUT: Callee = Callee { addr: 0x003c_dde8, f: ee::s14::SND_UtkSetInput };
    pub const SND_UTK_DECODE_FRAME: Callee = Callee { addr: 0x003c_d878, f: ee::s14::SND_UtkDecodeFrame };
    pub const SND_EE_MIX_BLOCK: Callee = Callee { addr: 0x003c_85d0, f: ee::s14::SND_EeMixBlock };
    pub const SND_EAXA_DECODE_BLOCKS: Callee = Callee { addr: 0x003c_ca08, f: ee::s14::SND_EaxaDecodeBlocks };
    pub const SND_EAXA_SET_INPUT: Callee = Callee { addr: 0x003c_cf60, f: ee::s14::SND_EaxaSetInput };
    pub const SND_EAXA_SET_HISTORY: Callee = Callee { addr: 0x003c_d190, f: ee::s14::SND_EaxaSetHistory };
}

/// Cross-shard calls the compiler emitted as `call_indirect` (function table).
mod table_callee {
    /// Kernel syscall wrapper `iSignalSema(sema)`.
    pub const I_SIGNAL_SEMA: u32 = 0x0042_3dd0;
}

// ---------------------------------------------------------------------------
// Views.

/// More fields of the driver's state block ([`snd::STATE`]), as offsets.
mod snd_state {
    /// `s16`: number of voice channels.
    pub const CHANNEL_COUNT: u32 = 0x17e;
    /// `u32`: driver ticks run so far.
    pub const TICKS: u32 = 0x180;
    /// `SndChannel*`: the channel array.
    pub const CHANNELS: u32 = 0x1f0;
}

/// The driver's second global block (`0x510000 - 0x5300`, the sound
/// thread's synchronisation state), as offsets.
mod snd_sync {
    /// Base address (`lui 0x51; addiu -0x5300`).
    pub const BASE: u32 = 0x0050_ad00;
    /// The sound thread's wake-up semaphore.
    pub const SEMA: u32 = 0x1aa4;
    /// The tag of the last completed SIF transfer.
    pub const SIF_DONE_TAG: u32 = 0x1b04;
}

/// The UTK bit reader `{ u8* ptr; u32 bits; int count; }`.
mod bits {
    /// Next byte to shift in.
    pub const PTR: u32 = 0x0;
    /// Bit buffer (consumed from the low end).
    pub const BUF: u32 = 0x4;
    /// Valid bits in the buffer.
    pub const COUNT: u32 = 0x8;
    /// The `(1 << n) - 1` mask table (`0x450000 - 0x1740`).
    pub const MASKS: u32 = 0x0044_e8c0;
}

/// UTK decoder state offsets.
mod utk {
    /// The 12 reflection coefficients.
    pub const RC: u32 = 0x114;
    /// The synthesis filter history.
    pub const HISTORY: u32 = 0x144;
    /// The excitation buffer (`float[]`).
    pub const EXCITATION: u32 = 0x684;
}

/// A DSP node in the driver's sorted chains: next at +8, prev at +0x10,
/// priority (`u16`) at +0x18.
mod dsp_node {
    pub const NEXT: u32 = 0x08;
    pub const PREV: u32 = 0x10;
    pub const PRIORITY: u32 = 0x18;
}

// ---------------------------------------------------------------------------
// Small helpers.

/// `lw base+off` (sign-extended).
#[inline(always)]
fn lw_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lw(c)
}

/// `sw v, base+off`.
#[inline(always)]
fn sw_at(c: &mut Cpu, base: u64, off: u32, v: u64) {
    Addr((base as u32).wrapping_add(off)).sw(c, v)
}

/// `slt`/`slti` on whole registers.
#[inline(always)]
fn slt(a: u64, b: u64) -> u64 {
    ((a as i64) < (b as i64)) as u64
}

/// `sltu` on whole registers.
#[inline(always)]
fn sltu(a: u64, b: u64) -> u64 {
    (a < b) as u64
}

/// `addiu`: 32-bit add of an immediate, sign-extended.
#[inline(always)]
fn addiu(a: u64, imm: i32) -> u64 {
    sx((a as u32).wrapping_add(imm as u32))
}

/// `sll`.
#[inline(always)]
fn sll(v: u64, n: u32) -> u64 {
    sx((v as u32) << (n & 31))
}

/// `srl`.
#[inline(always)]
fn srl(v: u64, n: u32) -> u64 {
    sx((v as u32) >> (n & 31))
}

/// `sra`.
#[inline(always)]
fn sra(v: u64, n: u32) -> u64 {
    sx(((v as u32) as i32 >> (n & 31)) as u32)
}

// ---------------------------------------------------------------------------
// Handles.

/// `bool SND_IsHandleDone(handle)` 0x3b8ab8: whether the handle no longer
/// names a playing channel (`SND_HandleToChannel` failed, i.e. returned a
/// negative error).
pub fn snd_is_handle_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SND_HANDLE_TO_CHANNEL.call(c, 0x003b_8ac8)?;
        frame.restore_ra(c, 0);
        let done = srl(reg(c, V0), 31);
        set_reg(c, V0, done);
        frame.close(c);
        Ok(())
    })
}

/// Addresses of `SND_Stop`.
mod stop {
    pub const START: u32 = 0x003b_89e0;
    pub const END: u32 = 0x003b_8a38;
    /// Loop head (stop the cursor's channel), the resume point after an
    /// unwind.
    pub const HEAD: u32 = 0x003b_8a08;
}

/// `int SND_Stop(handle)` 0x3b89e0: stop the voices of every channel of a
/// handle. Returns the handle's first channel, or the error from
/// `SND_HandleToChannel`.
///
/// The original is a `match pc` dispatcher whose loop returns on an unwind
/// request, so the loop head is an entry point too (registers: s0 the
/// first channel, the cursor at `0(sp)`).
pub fn snd_stop(c: &mut Cpu) {
    let frame = Frame { size: 0x30 };
    let resumed = match c.pc {
        stop::START => false,
        stop::HEAD => true,
        pc => {
            c.jump_out(pc, stop::START, stop::END);
            return;
        }
    };
    guest(c, |c| {
        if !resumed {
            c.cycles += 8;
            Frame::open(c, frame.size);
            frame.save(c, S0, 0x20);
            frame.save_ra(c, 0x10);
            callee::SND_HANDLE_TO_CHANNEL.call(c, 0x003b_89f4)?;
            let first = reg(c, V0);
            set_reg(c, S0, first);
            set_reg(c, V0, u64::MAX);
            if (first as i64) >= 0 {
                // The cursor starts at -1; the first step finds the first
                // channel.
                c.cycles += 2;
                Addr(frame.at(c, 0)).sw(c, u64::MAX);
                if !stop_next(c)? {
                    return stop_exit(c, frame);
                }
                if back_edge_unwinds(c, stop::HEAD) {
                    return Ok(());
                }
            } else {
                return stop_exit(c, frame);
            }
        }
        loop {
            // Loop head: stop the voices of the cursor's channel.
            c.cycles += 2;
            let cursor = Addr(frame.at(c, 0)).lw(c);
            set_reg(c, A0, cursor);
            callee::SND_CHANNEL_STOP_VOICES.call(c, 0x003b_8a10)?;
            if !stop_next(c)? {
                return stop_exit(c, frame);
            }
            if back_edge_unwinds(c, stop::HEAD) {
                return Ok(());
            }
        }
    })
}

/// `SND_NextHandleChannel(first, sp)` for [`snd_stop`]: true when there is
/// another channel.
fn stop_next(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 5;
    set_reg(c, A0, reg(c, S0));
    set_reg(c, A1, reg(c, SP));
    callee::SND_NEXT_HANDLE_CHANNEL.call(c, 0x003b_8a1c)?;
    Ok(reg(c, V0) != 0)
}

/// The epilogue of [`snd_stop`]: return the first channel.
fn stop_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    set_reg(c, V0, reg(c, S0));
    frame.restore_ra(c, 0x10);
    frame.restore(c, S0, 0x20);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// SIF and VU0 context.

/// `int SND_SifCmdHandler(SifCmdPacket* pkt)` 0x3c1578: the SIF command
/// handler of the sound library (interrupt context). Command 2 reports a
/// finished SIF DMA (`SND_SifDmaComplete(pkt->tag)`); command 0 records the
/// tag the IOP finished and wakes the sound thread (`iSignalSema`); others
/// are ignored.
pub fn snd_sif_cmd_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let pkt = reg(c, A0);
        frame.save_ra(c, 0);
        let cmd = lw_at(c, pkt, 0x10);
        set_reg(c, V1, cmd);
        set_reg(c, V0, slt(cmd, 3)); // leftover
        if cmd == 2 {
            c.cycles += 4;
            let tag = lw_at(c, pkt, 0xc);
            set_reg(c, A0, tag);
            callee::SND_SIF_DMA_COMPLETE.call(c, 0x003c_15b0)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 2;
            frame.restore_ra(c, 0);
            if (cmd as i64) < 3 {
                c.cycles += 2;
                set_reg(c, V0, snd::STATE_HI); // leftover
                if cmd == 0 {
                    c.cycles += 6;
                    let tag = lw_at(c, pkt, 0xc);
                    set_reg(c, V1, tag);
                    let sync = sx(snd_sync::BASE);
                    set_reg(c, V0, sync);
                    let sema = lw_at(c, sync, snd_sync::SEMA);
                    set_reg(c, A0, sema);
                    sw_at(c, sync, snd_sync::SIF_DONE_TAG, tag);
                    call_ptr(c, table_callee::I_SIGNAL_SEMA, 0x003c_15cc)?;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `SND_SaveVu0Context(Vu0Context* ctx)` 0x3c0858: save the VU0 macro-mode
/// state the sound thread may clobber: vf1..vf31 at `ctx + 0x00..0x1e0`,
/// the MAC flags at +0x200 and +0x204, and ACC (through `vmaddx vf1, vf0,
/// vf0x`, i.e. vf1 = ACC + 0) at +0x1f0. The MAC value is also written to
/// the status register (`ctc2 t1, vi16`), as the original does. VU0
/// instructions are issued through the machine's VU0 so its flags follow.
pub fn snd_save_vu0_context(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, X};
    c.cycles += 40;
    let ctx = reg(c, A0);
    set_reg(c, T0, ctx);
    let at = |o: u32| (ctx as u32).wrapping_add(o);
    c.cfc2(T1, 17);
    c.sqc2(1, at(0x00));
    c.sqc2(2, at(0x10));
    let mac = reg(c, T1);
    Addr(at(0x200)).sw(c, mac);
    c.sqc2(3, at(0x20));
    c.sqc2(4, at(0x30));
    c.ctc2(T1, 16);
    c.sqc2(5, at(0x40));
    c.sqc2(6, at(0x50));
    Addr(at(0x204)).sw(c, mac);
    c.sqc2(7, at(0x60));
    c.sqc2(8, at(0x70));
    // vf1 = ACC + vf0 * vf0.x (vf0 = (0,0,0,1), so ACC.xyz and ACC.w + 0).
    c.vu0().madd(0b1111, 1, 0, Src::Bc(0, X));
    for vf in 9..32u32 {
        c.sqc2(vf as usize, at(0x80 + (vf - 9) * 0x10));
    }
    c.sqc2(1, at(0x1f0));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// DSP chains.

/// `SND_DspInsertSorted(DspNode** head, DspNode* node)` 0x3c8f48: insert a
/// node into a doubly linked chain kept sorted by ascending priority
/// (`u16` +0x18); the node goes before the first node whose priority is not
/// lower than its own.
pub fn snd_dsp_insert_sorted(c: &mut Cpu) {
    let head = reg(c, A0);
    let node = reg(c, A1);
    c.cycles += 3;
    let mut next = lw_at(c, head, 0);
    set_reg(c, V1, next);
    let mut prev = 0u64;
    set_reg(c, A3, 0);
    // Walk while the next node sorts before `node`.
    let mut at_end = false;
    if next != 0 {
        c.cycles += 5;
        let first_prio = Addr((next as u32).wrapping_add(dsp_node::PRIORITY)).lhu(c);
        let prio = Addr((node as u32).wrapping_add(dsp_node::PRIORITY)).lhu(c);
        set_reg(c, A2, prio);
        let before = sltu(first_prio, prio);
        set_reg(c, V0, before);
        if before != 0 {
            c.cycles += 2;
            prev = next;
            set_reg(c, A3, prev);
            loop {
                c.cycles += 3;
                next = lw_at(c, prev, dsp_node::NEXT);
                set_reg(c, V1, next);
                if next == 0 {
                    sw_at(c, node, dsp_node::NEXT, next);
                    at_end = true;
                    break;
                }
                c.cycles += 4;
                let p = Addr((next as u32).wrapping_add(dsp_node::PRIORITY)).lhu(c);
                let before = sltu(p, reg(c, A2));
                set_reg(c, V0, before);
                if before == 0 {
                    break;
                }
                prev = next;
                set_reg(c, A3, prev);
                if event_due(c) {
                    deliver_event_here(c, 0x003c_8f70);
                    prev = reg(c, A3);
                }
            }
        }
    }
    if !at_end {
        // Link `node` in front of `next` (if any).
        c.cycles += 2;
        sw_at(c, node, dsp_node::NEXT, next);
        if next != 0 {
            c.cycles += 1;
            sw_at(c, next, dsp_node::PREV, node);
        }
    }
    c.cycles += 2;
    let prev = reg(c, A3);
    if prev != 0 {
        sw_at(c, prev, dsp_node::NEXT, node);
        c.cycles += 2;
        sw_at(c, node, dsp_node::PREV, prev);
    } else {
        c.cycles += 2;
        sw_at(c, head, 0, node);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UTK (MicroTalk) speech decoder.

/// `u32 SND_UtkReadBits(BitReader* br, int n)` 0x3cd1f8: take the low `n`
/// bits of the bit buffer and shift them out; when fewer than 8 bits are
/// left, refill one byte from the input.
pub fn snd_utk_read_bits(c: &mut Cpu) {
    c.cycles += 15;
    let br = reg(c, A0);
    let n = reg(c, A1);
    set_reg(c, A3, br);
    let mask_at = addu(sll(n, 2), sx(bits::MASKS));
    set_reg(c, V1, mask_at);
    let buf = lw_at(c, br, bits::BUF);
    let mask = Addr(mask_at as u32).lw(c);
    set_reg(c, V0, mask);
    let count = lw_at(c, br, bits::COUNT);
    set_reg(c, A0, count);
    let rest = srl(buf, n as u32);
    set_reg(c, T0, rest);
    let value = buf & mask;
    set_reg(c, A2, value);
    sw_at(c, br, bits::BUF, rest);
    let left = sx((count as u32).wrapping_sub(n as u32));
    set_reg(c, A1, left);
    let low = slt(left, 8);
    set_reg(c, V0, low);
    sw_at(c, br, bits::COUNT, left);
    if low != 0 {
        c.cycles += 9;
        refill_byte(c, br, left, rest);
    }
    c.cycles += 2;
    set_reg(c, V0, value);
    abi::ret(c);
}

/// The bit reader's one-byte refill (shared tail of read/skip): `buf |=
/// *ptr++ << count; count += 8`, leaving the registers as the original
/// (a0 count+8, v0 ptr+1, v1 the new buffer).
fn refill_byte(c: &mut Cpu, br: u64, left: u64, rest: u64) {
    let p = lw_at(c, br, bits::PTR);
    let count = addiu(left, 8);
    set_reg(c, A0, count);
    let byte = Addr(p as u32).lbu(c);
    let next = addiu(p, 1);
    set_reg(c, V0, next);
    sw_at(c, br, bits::COUNT, count);
    let shifted = sll(byte, left as u32);
    sw_at(c, br, bits::PTR, next);
    let buf = rest | shifted;
    set_reg(c, V1, buf);
    sw_at(c, br, bits::BUF, buf);
}

/// `SND_UtkSkipBits(BitReader* br, int n)` 0x3cd260: drop `n` bits, refilling
/// a byte when fewer than 8 are left.
pub fn snd_utk_skip_bits(c: &mut Cpu) {
    c.cycles += 9;
    let br = reg(c, A0);
    let n = reg(c, A1);
    set_reg(c, A2, br);
    let buf = lw_at(c, br, bits::BUF);
    set_reg(c, V0, buf);
    let count = lw_at(c, br, bits::COUNT);
    set_reg(c, V1, count);
    let rest = srl(buf, n as u32);
    set_reg(c, A3, rest);
    let left = sx((count as u32).wrapping_sub(n as u32));
    set_reg(c, A1, left);
    sw_at(c, br, bits::BUF, rest);
    let low = slt(left, 8);
    set_reg(c, V0, low);
    sw_at(c, br, bits::COUNT, left);
    if low != 0 {
        c.cycles += 9;
        refill_byte(c, br, left, rest);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `SND_UtkSynthesize(UtkState* s, int offset, int blocks)` 0x3cd690:
/// convert the frame's reflection coefficients to LPC coefficients (on the
/// stack) and run the 12-tap LPC synthesis filter in place over `blocks`
/// blocks of 12 excitation samples starting at sample `offset`.
pub fn snd_utk_synthesize(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        let state = reg(c, A0);
        set_reg(c, S0, state);
        frame.save(c, S2, 0x40);
        set_reg(c, S1, reg(c, A1));
        set_reg(c, S2, reg(c, A2));
        frame.save_ra(c, 0x30);
        set_reg(c, A0, addiu(state, utk::RC as i32));
        set_reg(c, A1, reg(c, SP));
        callee::SND_UTK_RC_TO_LPC.call(c, 0x003c_d6bc)?;
        let exc = addiu(sll(reg(c, S1), 2), utk::EXCITATION as i32);
        set_reg(c, S1, exc);
        set_reg(c, A0, reg(c, S2));
        set_reg(c, A2, addiu(state, utk::HISTORY as i32));
        set_reg(c, A3, addu(state, exc));
        set_reg(c, A1, reg(c, SP));
        callee::SND_UTK_LPC_FILTER.call(c, 0x003c_d6d8)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// The driver's block allocator.
//
// `SndHeap` (`*(STATE + 0x234)`): `base` (+0) the arena, `blocks` (+4) a
// table of `{offset, size}` entries indexed downwards from 0 to `lowest`
// (+0x14, <= 0) and sorted by offset, `limit` (+0xc) the end of the
// allocatable area (the table itself grows down from the top: every block
// takes 8 bytes), `low_water` (+0x10) the smallest free tail seen.

/// `SndHeap` fields.
mod heap {
    /// `SndHeap*` in the state block.
    pub const PTR: u32 = 0x234;
    pub const BASE: u32 = 0x0;
    pub const BLOCKS: u32 = 0x4;
    pub const LIMIT: u32 = 0xc;
    pub const LOW_WATER: u32 = 0x10;
    pub const LOWEST: u32 = 0x14;
}

/// The heap descriptor (`lw STATE+0x234`).
#[inline]
fn heap_ptr(c: &mut Cpu) -> u64 {
    snd::STATE.off(heap::PTR).lw(c)
}

/// `void SND_MemFree(void* p)` 0x3b5e98: free the block starting at `p`:
/// find its table entry (searching from index 0 down to `lowest`), give
/// its 8 table bytes back to `limit` and close the gap by moving the
/// entries below it up by one (unaligned doubleword copies, as compiled).
/// Unknown pointers are ignored.
pub fn snd_mem_free(c: &mut Cpu) {
    c.cycles += 8;
    let p = reg(c, A0);
    set_reg(c, T1, snd::STATE_HI);
    set_reg(c, A2, 0);
    let state = sx(snd::STATE.0);
    set_reg(c, A3, state);
    let h = heap_ptr(c);
    set_reg(c, V0, h);
    let base = lw_at(c, h, heap::BASE);
    set_reg(c, V1, base);
    let lowest = lw_at(c, h, heap::LOWEST);
    set_reg(c, A1, lowest);
    let off = sx((p as u32).wrapping_sub(base as u32));
    set_reg(c, T0, off);
    if (lowest as i64) < 0 {
        c.cycles += 2;
        let v = heap_ptr(c);
        set_reg(c, A1, v);
        // Find the entry (registers: a2 index, a1 heap, t0 offset).
        loop {
            c.cycles += 6;
            let i = reg(c, A2);
            let h = reg(c, A1);
            set_reg(c, V0, sll(i, 3));
            let entry = addu(lw_at(c, h, heap::BLOCKS), sll(i, 3));
            set_reg(c, V1, entry);
            let entry_off = lw_at(c, entry, 0);
            set_reg(c, A0, entry_off);
            let lowest = lw_at(c, h, heap::LOWEST);
            set_reg(c, V0, lowest);
            if entry_off != reg(c, T0) {
                c.cycles += 4;
                let i = addiu(i, -1);
                set_reg(c, A2, i);
                let more = slt(lowest, i);
                set_reg(c, V0, more);
                if more == 0 {
                    // Not found.
                    c.cycles += 2;
                    abi::ret(c);
                    return;
                }
                let v = heap_ptr(c);
                set_reg(c, A1, v);
                if event_due(c) {
                    deliver_event_here(c, 0x003b_5ec0);
                }
                continue;
            }
            // Found entry i: one entry less, 8 bytes more.
            c.cycles += 11;
            sw_at(c, h, heap::LOWEST, addiu(lowest, 1));
            let h = heap_ptr(c);
            set_reg(c, A0, h);
            let limit = addiu(lw_at(c, h, heap::LIMIT), 8);
            sw_at(c, h, heap::LIMIT, limit);
            let h = heap_ptr(c);
            set_reg(c, V1, h);
            let more = slt(lw_at(c, h, heap::LOWEST), i);
            set_reg(c, V0, more);
            set_reg(c, A1, state);
            if more != 0 {
                // Move entries [lowest .. i-1] up by one (registers: a2
                // index, a0 its byte offset, v1 heap, a1 state).
                c.cycles += 3;
                let v = heap_ptr(c);
                set_reg(c, V1, v);
                set_reg(c, A0, sll(i, 3));
                loop {
                    c.cycles += 12;
                    let i = addiu(reg(c, A2), -1);
                    set_reg(c, A2, i);
                    let table = lw_at(c, reg(c, V1), heap::BLOCKS);
                    let dst = addu(reg(c, A0), table);
                    set_reg(c, A0, dst);
                    let e = Addr((dst as u32).wrapping_sub(8)).read_u64_unaligned(c);
                    set_reg(c, V0, e);
                    Addr(dst as u32).write_u64_unaligned(c, e);
                    let h = lw_at(c, reg(c, A1), heap::PTR);
                    set_reg(c, V1, h);
                    let more = slt(lw_at(c, h, heap::LOWEST), i);
                    set_reg(c, V0, more);
                    set_reg(c, A0, sll(i, 3));
                    if more == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_5f10);
                    }
                }
            }
            break;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void* SND_MemAlloc(int size)` 0x3b5c60: allocate `size` bytes rounded
/// up to 16 from the driver's arena, first fit. Each gap (between entry
/// i+1's end and entry i's start, starting at 0 for entry 0) is clamped
/// to `limit` by `SND_MemClampBlock`; the new block goes into the first
/// gap that is large enough, else after the last block. The entries below
/// the insertion point move down by one (unaligned doubleword copies),
/// `limit` shrinks by the 8 table bytes and the low-water mark of the free
/// tail is updated. Returns the block (`base + offset`) or 0.
///
/// The candidate `{start, size}` lives at `0(sp)`/`4(sp)`, as
/// `SND_MemClampBlock` takes pointers to it.
pub fn snd_mem_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x90);
        set_reg(c, V0, snd::STATE_HI);
        frame.save(c, S1, 0x70);
        frame.save(c, S0, 0x80);
        let size = addiu(reg(c, A0), 0xf) & !0xf;
        frame.save(c, S5, 0x30);
        let state = sx(snd::STATE.0);
        set_reg(c, A0, state);
        frame.save(c, S2, 0x60);
        set_reg(c, S5, snd::STATE_HI);
        frame.save(c, S3, 0x50);
        set_reg(c, V0, -16i64 as u64);
        frame.save(c, S4, 0x40);
        set_reg(c, S1, size);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0x10);
        let h = heap_ptr(c);
        set_reg(c, V1, h);
        let lowest = lw_at(c, h, heap::LOWEST);
        set_reg(c, V0, lowest);
        set_reg(c, S0, 0);
        let cand = |c: &Cpu, o: u32| Addr(frame.at(c, o));

        // Where the block goes: `Some(())` to insert at index s0, `None`
        // when nothing fits.
        let fits = if lowest == 0 {
            // Empty table: the gap is the whole arena.
            c.cycles += 10;
            let limit = lw_at(c, h, heap::LIMIT);
            set_reg(c, V0, limit);
            set_reg(c, A0, reg(c, SP));
            cand(c, 0).sw(c, 0);
            set_reg(c, A1, reg(c, SP) | 4);
            cand(c, 4).sw(c, limit);
            callee::SND_MEM_CLAMP_BLOCK.call(c, 0x003b_5cc4)?;
            let small = slt(cand(c, 4).lw(c), reg(c, S1));
            set_reg(c, V0, 0);
            if small != 0 {
                c.cycles += 2;
                frame.restore(c, S0, 0x80);
                return mem_alloc_exit(c, frame);
            }
            true
        } else {
            c.cycles += 2;
            set_reg(c, S4, addiu(reg(c, SP), 4));
            let mut placed = false;
            if (lowest as i64) < 0 {
                c.cycles += 3;
                set_reg(c, S3, reg(c, A0));
                set_reg(c, S2, 8);
                set_reg(c, S6, reg(c, S5));
                // First fit over the gaps (registers: s0 index i, s2 the
                // byte offset of entry i+1).
                loop {
                    c.cycles += 5;
                    let i = reg(c, S0);
                    let h = lw_at(c, reg(c, S3), heap::PTR);
                    set_reg(c, V1, sll(i, 3));
                    let table = lw_at(c, h, heap::BLOCKS);
                    set_reg(c, V0, table);
                    let entry = addu(table, sll(i, 3));
                    set_reg(c, A2, entry);
                    let gap = if i == 0 {
                        c.cycles += 3;
                        cand(c, 0).sw(c, 0);
                        lw_at(c, entry, 0)
                    } else {
                        c.cycles += 7;
                        let above = addu(table, reg(c, S2));
                        let end = addu(lw_at(c, above, 0), lw_at(c, above, 4));
                        set_reg(c, V1, end);
                        cand(c, 0).sw(c, end);
                        sx((lw_at(c, entry, 0) as u32).wrapping_sub(end as u32))
                    };
                    set_reg(c, V0, gap);
                    c.cycles += 8;
                    cand(c, 4).sw(c, gap);
                    set_reg(c, A0, reg(c, SP));
                    set_reg(c, A1, reg(c, S4));
                    callee::SND_MEM_CLAMP_BLOCK.call(c, 0x003b_5d3c)?;
                    let small = slt(cand(c, 4).lw(c), reg(c, S1));
                    set_reg(c, V0, small);
                    if small == 0 {
                        // Fits: move entries [lowest .. i-1] down by one.
                        c.cycles += 5;
                        let h = lw_at(c, reg(c, S3), heap::PTR);
                        set_reg(c, V0, h);
                        let lowest = lw_at(c, h, heap::LOWEST);
                        set_reg(c, A2, lowest);
                        let shift = slt(lowest, reg(c, S0));
                        set_reg(c, V1, shift);
                        let st = addiu(reg(c, S6), -0x5718);
                        set_reg(c, A3, st);
                        if shift != 0 {
                            c.cycles += 2;
                            let v = lw_at(c, st, heap::PTR);
                            set_reg(c, A0, v);
                            // Registers: a2 index j, a0 heap, a3 state.
                            loop {
                                c.cycles += 11;
                                let j = reg(c, A2);
                                set_reg(c, V0, sll(j, 3));
                                let j1 = addiu(j, 1);
                                set_reg(c, A2, j1);
                                let table = lw_at(c, reg(c, A0), heap::BLOCKS);
                                set_reg(c, V1, table);
                                let more = slt(j1, reg(c, S0));
                                set_reg(c, A1, more);
                                let at = addu(sll(j, 3), table);
                                set_reg(c, V0, at);
                                let e = Addr((at as u32).wrapping_add(8)).read_u64_unaligned(c);
                                set_reg(c, V1, e);
                                Addr(at as u32).write_u64_unaligned(c, e);
                                if more == 0 {
                                    c.cycles += 2;
                                    set_reg(c, A1, addiu(reg(c, S5), -0x5718));
                                    break;
                                }
                                let v = lw_at(c, reg(c, A3), heap::PTR);
                                set_reg(c, A0, v);
                                if event_due(c) {
                                    deliver_event_here(c, 0x003b_5d68);
                                }
                            }
                            return mem_alloc_insert(c, frame);
                        }
                        placed = true;
                        break;
                    }
                    let v = lw_at(c, reg(c, S3), heap::PTR);
                    set_reg(c, V1, v);
                    c.cycles += 5;
                    let i = addiu(reg(c, S0), -1);
                    set_reg(c, S0, i);
                    let more = slt(lw_at(c, reg(c, V1), heap::LOWEST), i);
                    set_reg(c, V0, more);
                    set_reg(c, S2, addiu(reg(c, S2), -8));
                    if more == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_5cf0);
                    }
                }
            }
            if placed {
                true
            } else {
                // After the last block.
                c.cycles += 20;
                let st = addiu(reg(c, S5), -0x5718);
                set_reg(c, V0, st);
                let h = lw_at(c, st, heap::PTR);
                set_reg(c, A3, h);
                set_reg(c, A1, reg(c, S4));
                set_reg(c, A0, reg(c, SP));
                let table = lw_at(c, h, heap::BLOCKS);
                set_reg(c, V0, table);
                let last = addu(table, addiu(sll(reg(c, S0), 3), 8));
                let last_size = lw_at(c, last, 4);
                set_reg(c, A2, last_size);
                let end = addu(lw_at(c, last, 0), last_size);
                set_reg(c, V1, end);
                cand(c, 0).sw(c, end);
                let room = sx((lw_at(c, h, heap::LIMIT) as u32).wrapping_sub(end as u32));
                set_reg(c, V0, room);
                cand(c, 4).sw(c, room);
                callee::SND_MEM_CLAMP_BLOCK.call(c, 0x003b_5df0)?;
                let small = slt(cand(c, 4).lw(c), reg(c, S1));
                set_reg(c, V0, 0);
                if small != 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x80);
                    return mem_alloc_exit(c, frame);
                }
                true
            }
        };
        debug_assert!(fits);
        c.cycles += 1;
        set_reg(c, A1, addiu(reg(c, S5), -0x5718));
        mem_alloc_insert(c, frame)
    })
}

/// [`snd_mem_alloc`]: write the entry `{start, size}` at index s0, update
/// the table bounds and the low-water mark, return `base + start`.
fn mem_alloc_insert(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 25;
    let state = reg(c, A1);
    let start = Addr(frame.at(c, 0)).lw(c);
    let h = lw_at(c, state, heap::PTR);
    let entry = addu(lw_at(c, h, heap::BLOCKS), sll(reg(c, S0), 3));
    let size = reg(c, S1);
    sw_at(c, entry, 0, start);
    sw_at(c, entry, 4, size);
    let h = lw_at(c, state, heap::PTR);
    let v = addiu(lw_at(c, h, heap::LOWEST), -1);
    sw_at(c, h, heap::LOWEST, v);
    let h = lw_at(c, state, heap::PTR);
    let limit = addiu(lw_at(c, h, heap::LIMIT), -8);
    set_reg(c, V0, limit);
    sw_at(c, h, heap::LIMIT, limit);
    let h = lw_at(c, state, heap::PTR);
    set_reg(c, A3, h);
    let start = Addr(frame.at(c, 0)).lw(c);
    set_reg(c, A2, start);
    let end = addu(start, size);
    set_reg(c, V1, end);
    let base = lw_at(c, h, heap::BASE);
    let tail = sx((lw_at(c, h, heap::LIMIT) as u32).wrapping_sub(end as u32));
    set_reg(c, A0, tail);
    let low = slt(tail, lw_at(c, h, heap::LOW_WATER));
    set_reg(c, V0, low);
    let block = addu(base, start);
    set_reg(c, A1, block);
    if low != 0 {
        c.cycles += 1;
        sw_at(c, h, heap::LOW_WATER, tail);
    }
    c.cycles += 1;
    set_reg(c, V0, block);
    c.cycles += 1;
    frame.restore(c, S0, 0x80);
    mem_alloc_exit(c, frame)
}

/// The epilogue of [`snd_mem_alloc`] (s0 is restored on the paths).
fn mem_alloc_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 9;
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore(c, S4, 0x40);
    frame.restore(c, S5, 0x30);
    frame.restore(c, S6, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// `void SND_MemCopy(void* dst, const void* src, int n)` 0x3c51e8: copy
/// `n` bytes. From 18 bytes on, it first copies bytes until `dst` is
/// 16-byte aligned; if `src` is then aligned too it copies 128-byte chunks
/// with `lq`/`sq` (through t1..t4) and then doublewords; the rest goes
/// byte by byte. Every loop checks events at its back edge (registers: a0
/// dst, a1 src, a2 count).
pub fn snd_mem_copy(c: &mut Cpu) {
    /// The back-edge event check of a copy loop.
    fn back_edge(c: &mut Cpu, head: u32) {
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
    let step = |c: &mut Cpu, r: usize, d: i32| set_reg(c, r, addiu(reg(c, r), d));

    c.cycles += 3;
    let n = reg(c, A2);
    set_reg(c, T0, slt(n, 18)); // leftover
    let dst_misalign = reg(c, A0) & 0xf;
    set_reg(c, T0, dst_misalign);
    let mut bytewise = (n as i64) < 18;
    if !bytewise {
        c.cycles += 2;
        if dst_misalign != 0 {
            // Align dst.
            loop {
                c.cycles += 3;
                let m = reg(c, A0) & 0xf;
                set_reg(c, T0, m);
                if m == 0 {
                    break;
                }
                c.cycles += 3;
                step(c, A2, -1);
                let b = Addr(abi::r32(c, A1)).lbu(c);
                set_reg(c, T1, b);
                if (reg(c, A2) as i64) <= 0 {
                    c.cycles += 2;
                    abi::ret(c);
                    return;
                }
                c.cycles += 4;
                step(c, A1, 1);
                Addr(abi::r32(c, A0)).sb(c, b);
                step(c, A0, 1);
                back_edge(c, 0x003c_52c8);
            }
        }
        c.cycles += 3;
        let src_misalign = reg(c, A1) & 0xf;
        set_reg(c, T0, src_misalign);
        if src_misalign != 0 {
            bytewise = true;
        } else {
            // 128-byte chunks through t1..t4.
            c.cycles += 3;
            step(c, A2, -0x80);
            if (reg(c, A2) as i64) >= 0 {
                loop {
                    c.cycles += 20;
                    let (src, dst) = (abi::r32(c, A1), abi::r32(c, A0));
                    for half in [0u32, 0x40] {
                        for k in 0..4u32 {
                            c.lq(T1 + k as usize, src.wrapping_add(half + 0x10 * k));
                        }
                        for k in 0..4u32 {
                            c.sq(T1 + k as usize, dst.wrapping_add(half + 0x10 * k));
                        }
                    }
                    step(c, A2, -0x80);
                    step(c, A1, 0x80);
                    step(c, A0, 0x80);
                    if (reg(c, A2) as i64) < 0 {
                        break;
                    }
                    back_edge(c, 0x003c_5214);
                }
            }
            // Doublewords.
            c.cycles += 3;
            step(c, A2, 0x78);
            if (reg(c, A2) as i64) >= 0 {
                loop {
                    c.cycles += 6;
                    let d = Addr(abi::r32(c, A1)).ld(c);
                    set_reg(c, T1, d);
                    step(c, A1, 8);
                    Addr(abi::r32(c, A0)).sd(c, d);
                    step(c, A2, -8);
                    step(c, A0, 8);
                    if (reg(c, A2) as i64) < 0 {
                        c.cycles += 2;
                        break;
                    }
                    back_edge(c, 0x003c_5270);
                }
            }
            c.cycles += 1;
            step(c, A2, 8);
            bytewise = true;
        }
    }
    debug_assert!(bytewise);
    // The remaining bytes.
    c.cycles += 3;
    step(c, A2, -1);
    if (reg(c, A2) as i64) >= 0 {
        loop {
            c.cycles += 6;
            step(c, A2, -1);
            let b = Addr(abi::r32(c, A1)).lbu(c);
            set_reg(c, T1, b);
            step(c, A1, 1);
            Addr(abi::r32(c, A0)).sb(c, b);
            step(c, A0, 1);
            if (reg(c, A2) as i64) < 0 {
                c.cycles += 2;
                break;
            }
            back_edge(c, 0x003c_52a0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Channels.

/// Voice channel fields (0x8c bytes; more in [`snd`]).
mod chan {
    /// Voice group (`u8`): channels of one multi-voice handle share it.
    pub const GROUP: u32 = 0x26;
    /// Group leader flag (`u8`): the channel that owns the group.
    pub const LEADER: u32 = 0x27;
    /// The driver tick at which the channel was released (`int`).
    pub const RELEASE_TICK: u32 = 0x2c;
    /// Allocation state (`s8`): 0 free, 1 playing, 2 released leader
    /// waiting for its group.
    pub const ALLOCATED: u32 = 0x69;
}

/// Mark channel `ch` free: not allocated, no group, not a leader, and
/// stamp the release tick (`lw state+0x180`, left in `tick_reg`).
fn free_channel(c: &mut Cpu, ch: u64, state: u64, tick_reg: usize) {
    Addr((ch as u32).wrapping_add(chan::ALLOCATED)).sb(c, 0);
    Addr((ch as u32).wrapping_add(chan::GROUP)).sb(c, 0);
    Addr((ch as u32).wrapping_add(chan::LEADER)).sb(c, 0);
    let tick = lw_at(c, state, snd_state::TICKS);
    set_reg(c, tick_reg, tick);
    sw_at(c, ch, chan::RELEASE_TICK, tick);
}

/// `void SND_ChannelRelease(int index)` 0x3ba550: release a voice channel.
/// A channel without a group is simply freed. For a group the function
/// counts the group's live members (allocated, with a handle) and finds
/// its leader:
///
/// - the last live member is freed;
/// - when only the leader (state 2, waiting) and this channel remain, both
///   are freed;
/// - a leader releasing itself while others still play is parked in state
///   2 (and keeps its group);
/// - any other member just leaves (state 0, no group).
pub fn snd_channel_release(c: &mut Cpu) {
    c.cycles += 12;
    let index = reg(c, A0);
    set_reg(c, T2, index);
    set_reg(c, T3, snd::STATE_HI);
    let off = sx(abi::mult(c, index as u32, snd::CHANNEL_SIZE));
    set_reg(c, V1, off);
    let state = sx(snd::STATE.0);
    set_reg(c, V0, state);
    let table = lw_at(c, state, snd_state::CHANNELS);
    set_reg(c, A3, table);
    set_reg(c, T1, 0);
    let ch = addu(table, off);
    set_reg(c, A2, ch);
    let group = Addr((ch as u32).wrapping_add(chan::GROUP)).lbu(c);
    set_reg(c, A1, group);
    set_reg(c, T0, u64::MAX);
    if group == 0 {
        c.cycles += 6;
        free_channel(c, ch, state, V0);
        abi::ret(c);
        return;
    }

    // Count the group's live members (t1) and find its leader (t0).
    c.cycles += 3;
    let count = Addr(snd::STATE.0 + snd_state::CHANNEL_COUNT).lh(c);
    set_reg(c, V0, count);
    set_reg(c, A0, 0);
    if (count as i64) > 0 {
        c.cycles += 3;
        set_reg(c, A2, count);
        set_reg(c, V1, table);
        // Registers: a0 index, v1 channel, a2 count, t1 members, t0 leader.
        loop {
            c.cycles += 3;
            let other = reg(c, V1);
            let i = reg(c, A0);
            let field = |o: u32| Addr((other as u32).wrapping_add(o));
            let g = field(chan::GROUP).lbu(c);
            set_reg(c, V0, g);
            if g == reg(c, A1) {
                c.cycles += 3;
                let h = field(0).lw(c);
                set_reg(c, V0, h);
                if (h as i64) >= 0 {
                    c.cycles += 3;
                    let alloc = field(chan::ALLOCATED).lb(c);
                    set_reg(c, V0, alloc);
                    if alloc != 0 {
                        c.cycles += 4;
                        let leader = field(chan::LEADER).lbu(c);
                        set_reg(c, V0, leader);
                        set_reg(c, T1, addiu(reg(c, T1), 1));
                        if leader != 0 {
                            set_reg(c, T0, i);
                        }
                    }
                }
            }
            set_reg(c, A0, addiu(i, 1));
            c.cycles += 3;
            let more = slt(reg(c, A0), reg(c, A2));
            set_reg(c, V0, more);
            set_reg(c, V1, addiu(other, snd::CHANNEL_SIZE as i32));
            if more == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_a598);
            }
        }
    }

    c.cycles += 7;
    set_reg(c, A0, snd::CHANNEL_SIZE as u64);
    let state = addiu(reg(c, T3), -0x5718);
    set_reg(c, A3, state);
    let index = reg(c, T2);
    let off = sx(abi::mult(c, index as u32, snd::CHANNEL_SIZE));
    set_reg(c, V1, off);
    let table = lw_at(c, state, snd_state::CHANNELS);
    set_reg(c, A1, table);
    set_reg(c, V0, 1);
    let ch = addu(table, off);
    set_reg(c, A2, ch);
    let members = reg(c, T1);
    let leader = reg(c, T0);
    if members == 1 {
        // The last live member.
        c.cycles += 6;
        free_channel(c, ch, state, V0);
        abi::ret(c);
        return;
    }

    c.cycles += 6;
    let leader_off = sx(abi::mult(c, leader as u32, snd::CHANNEL_SIZE));
    set_reg(c, A0, leader_off);
    set_reg(c, V1, 2);
    let leader_state = Addr((addu(leader_off, table) as u32).wrapping_add(chan::ALLOCATED)).lb(c);
    set_reg(c, V0, leader_state);
    set_reg(c, V1, snd::CHANNEL_SIZE as u64);
    if leader_state != 2 {
        c.cycles += 1;
    } else {
        c.cycles += 2;
        if index != leader {
            c.cycles += 2;
            if members == 2 {
                // Only the waiting leader is left: free both.
                c.cycles += 19;
                Addr((ch as u32).wrapping_add(chan::ALLOCATED)).sb(c, 0);
                Addr((ch as u32).wrapping_add(chan::GROUP)).sb(c, 0);
                Addr((ch as u32).wrapping_add(chan::LEADER)).sb(c, 0);
                let tick = lw_at(c, state, snd_state::TICKS);
                set_reg(c, V1, tick);
                sw_at(c, ch, chan::RELEASE_TICK, tick);
                for f in [chan::ALLOCATED, chan::GROUP, chan::LEADER] {
                    let l = addu(leader_off, lw_at(c, state, snd_state::CHANNELS));
                    set_reg(c, if f == chan::GROUP { V1 } else { V0 }, l);
                    Addr((l as u32).wrapping_add(f)).sb(c, 0);
                }
                let tbl = lw_at(c, state, snd_state::CHANNELS);
                let tick = lw_at(c, state, snd_state::TICKS);
                set_reg(c, V0, tick);
                let l = addu(leader_off, tbl);
                set_reg(c, V1, l);
                sw_at(c, l, chan::RELEASE_TICK, tick);
                abi::ret(c);
                return;
            }
        }
    }
    let state = addiu(reg(c, T3), -0x5718);
    set_reg(c, V0, state);

    c.cycles += 7;
    let leader_off = sx(abi::mult(c, leader as u32, snd::CHANNEL_SIZE));
    let tbl = lw_at(c, state, snd_state::CHANNELS);
    set_reg(c, A0, tbl);
    set_reg(c, A1, 1);
    let l = addu(leader_off, tbl);
    set_reg(c, V1, l);
    let leader_state = Addr((l as u32).wrapping_add(chan::ALLOCATED)).lb(c);
    set_reg(c, V0, leader_state);
    if leader_state == 1 {
        c.cycles += 2;
        if index == leader {
            // The leader leaves while members still play: park it.
            c.cycles += 3;
            set_reg(c, V0, 2);
            Addr((l as u32).wrapping_add(chan::ALLOCATED)).sb(c, 2);
            abi::ret(c);
            return;
        }
    }
    Addr((ch as u32).wrapping_add(chan::ALLOCATED)).sb(c, 0);
    c.cycles += 6;
    let state = addiu(reg(c, T3), -0x5718);
    set_reg(c, V1, state);
    Addr((ch as u32).wrapping_add(chan::GROUP)).sb(c, 0);
    Addr((ch as u32).wrapping_add(chan::LEADER)).sb(c, 0);
    let tick = lw_at(c, state, snd_state::TICKS);
    set_reg(c, V0, tick);
    sw_at(c, ch, chan::RELEASE_TICK, tick);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UTK interpolation / RC->LPC / decode entry points (package E1).

/// `void SND_UtkInterpolate(float* p)` 0x3cd518: one interpolation pass of
/// the UTK synthesis filter. Starting at `p`, 54 output samples are written
/// at `p + 8*i`, each the weighted sum of the five inputs around it with the
/// coefficients 0.5953, -0.0456 and 0.0180 (in f7, f6, f5). `v0` counts down
/// from 106 by 2 per iteration and is negative on return.
pub fn snd_utk_interpolate(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    c.cycles += 10;
    // The coefficient literals are materialised through `at` into f5..f7.
    c.g[AT][0] = 0x3f18_ee49;
    c.f[7] = c.g[AT][0] as u32;
    c.g[V0][0] = 0x6a;
    c.g[AT][0] = 0xffff_ffff_bdea_aefb;
    c.f[6] = c.g[AT][0] as u32;
    c.g[AT][0] = 0x3c93_b945;
    c.f[5] = c.g[AT][0] as u32;
    loop {
        c.cycles += 18;
        let a0 = c.g[A0][0] as u32;
        c.lwc1(2, a0.wrapping_add(4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(2));
        c.lwc1(0, a0.wrapping_sub(4));
        c.lwc1(3, a0.wrapping_sub(0xc));
        c.lwc1(4, a0.wrapping_add(0xc));
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[2])).0; // add_s f0, f0, f2
        c.lwc1(1, a0.wrapping_sub(0x14));
        c.f[3] = (FpuF32(c.f[3]) + FpuF32(c.f[4])).0; // add_s f3, f3, f4
        c.lwc1(2, a0.wrapping_add(0x14));
        c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[7])).0; // mul_s f0, f0, f7
        c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[2])).0; // add_s f1, f1, f2
        c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[6])).0; // mul_s f3, f3, f6
        c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[5])).0; // mul_s f1, f1, f5
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[3])).0; // add_s f0, f0, f3
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0; // add_s f0, f0, f1
        c.swc1(0, a0);
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx(a0.wrapping_add(8));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_d540);
        }
    }
}

/// `void SND_UtkRcToLpc(float* rc, float* lpc)` 0x3cd590: convert the 11
/// reflection coefficients at `rc` to LPC coefficients at `lpc` with the
/// Levinson-Durbin recursion (the intermediate working set lives in the
/// frame at `sp..sp+0x60`). No calls; the recursion loops carry the usual
/// back-edge event checks.
pub fn snd_utk_rc_to_lpc(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    c.cycles += 6;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[A3][0] = 0xa;
    c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2c));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    // Copy rc[10..=0] down to sp+0x04..=sp+0x2c.
    loop {
        c.cycles += 7;
        c.lwc1(0, c.g[V0][0] as u32);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        c.swc1(0, c.g[V1][0] as u32);
        let more = (c.g[A3][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_d5a8);
        }
    }
    c.cycles += 5;
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = c.g[AT][0] as u32;
    c.g[T0][0] = 0;
    c.g[T3][0] = c.g[T2][0];
    c.swc1(0, c.g[SP][0] as u32);
    let a0 = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let sp = c.g[SP][0] as u32;
    loop {
        c.cycles += 10;
        c.lwc1(0, a0.wrapping_add(0x2c));
        c.g[A3][0] = 0xa;
        c.lwc1(1, sp.wrapping_add(0x2c));
        c.g[V1][0] = sx((c.g[T0][0] as u32) << 2);
        c.f[0] = (-FpuF32(c.f[0])).0; // neg_s f0, f0
        c.g[T1][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
        c.g[V0][0] = sx(sp.wrapping_add(0x28));
        c.g[A2][0] = sx(a0.wrapping_add(0x28));
        c.f[3] = (FpuF32(c.f[0]) * FpuF32(c.f[1])).0; // mul_s f3, f0, f1
        loop {
            c.cycles += 11;
            c.lwc1(0, c.g[A2][0] as u32);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
            c.lwc1(2, c.g[V0][0] as u32);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(4));
            c.f[1] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0; // mul_s f1, f0, f2
            c.f[3] = (FpuF32(c.f[3]) - FpuF32(c.f[1])).0; // sub_s f3, f3, f1
            c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[3])).0; // mul_s f0, f0, f3
            c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[2])).0; // add_s f0, f0, f2
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(4));
            let more = (c.g[A3][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_d600);
            }
        }
        // Tail 0x3cd670: one more reflection step.
        c.cycles += 4;
        c.swc1(3, sp);
        c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let small = (c.g[T0][0] as i64) <= 0;
        c.swc1(3, c.g[V0][0] as u32);
        if !small {
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
            c.g[A3][0] = c.g[T0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A2][0] = c.g[A1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
            loop {
                c.cycles += 8;
                c.lwc1(0, c.g[A2][0] as u32);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
                c.lwc1(1, c.g[V0][0] as u32);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
                c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[1])).0; // mul_s f0, f0, f1
                let more = c.g[A3][0] != 0;
                c.f[3] = (FpuF32(c.f[3]) - FpuF32(c.f[0])).0; // sub_s f3, f3, f0
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_d650);
                }
            }
        }
        c.cycles += 5;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(a1));
        c.g[T0][0] = c.g[T1][0];
        c.g[V0][0] = ((c.g[T0][0] as i64) < 12i64) as u64;
        let more = c.g[V0][0] != 0;
        c.swc1(3, c.g[V1][0] as u32);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_d5d8);
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp.wrapping_add(0x60));
    abi::ret(c);
}

/// `int SND_DecUtk_DecodeMem(UtkDecoder* d, int samples, int, int)` 0x3c9360:
/// decode up to `d->remaining - d->pos` samples into the buffer at `d->buf`
/// (plus the cursor at `sp`), then zero the leftover; returns 1, or -1 when
/// nothing remains to decode.
pub fn snd_dec_utk_decode_mem(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        let d = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = d;
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        let v0 = Addr((d as u32).wrapping_add(0x20)).lw(c);
        c.g[V0][0] = v0;
        let v1 = Addr((d as u32).wrapping_add(0x1c)).lw(c);
        c.g[V1][0] = v1;
        let less = (v0 < v1) as u64;
        c.g[V0][0] = less;
        let t = less != 0;
        Addr(frame.at(c, 0)).sw(c, c.g[A3][0]);
        if !t {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 14;
            c.g[A0][0] = Addr((d as u32).wrapping_add(0x28)).lw(c);
            c.g[A1][0] = sx(frame.at(c, 0));
            c.g[A2][0] = c.g[S1][0];
            callee::SND_UTK_DECODE.call(c, 0x003c_93a4)?;
            c.g[A2][0] = c.g[V0][0];
            let a0 = Addr(frame.at(c, 0)).lw(c);
            c.g[A0][0] = a0;
            let v0 = Addr((d as u32).wrapping_add(0x20)).lw(c);
            c.g[V0][0] = v0;
            c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
            c.g[A0][0] = sx((a0 as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = ((c.g[A2][0] as i64) < (c.g[S1][0] as i64)) as u64;
            c.g[V0][0] = sx((v0 as u32).wrapping_add(c.g[A2][0] as u32));
            Addr((d as u32).wrapping_add(0x20)).sw(c, c.g[V0][0]);
            let t = c.g[A1][0] == 0;
            Addr(frame.at(c, 0)).sw(c, c.g[A0][0]);
            if !t {
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                c.g[A1][0] = 0;
                c.g[A2][0] = sx((c.g[S1][0] as u32) << 2);
                call_ptr(c, 0x0041_6210, 0x003c_93dc)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{covered, machine, ptr, FAKE, LOG_RESULTS};
    use crate::difftest::{check, check_lohi, check_paths, fill, require_paths, Rng, DATA, GAME_GP};
    use ssx3_ee::{s13, s14};

    const SEEDS: u32 = 150;
    /// The test channel table.
    const CHANNELS: u32 = DATA + 0x10_0000;

    /// [`reg64`] with the value computed first.
    macro_rules! r64 {
        ($r:expr, $v:expr) => {{
            let v = $v;
            reg64($r, v)
        }};
    }

    /// A whole register: mostly a sign-extended word, sometimes garbage above.
    fn reg64(r: &mut Rng, v: u32) -> u64 {
        if r.next_u32() % 4 == 0 {
            (r.next_u32() as u64) << 32 | v as u64
        } else {
            sx(v)
        }
    }

    /// A channel table of `n` channels with random groups, flags and
    /// handles (handle = `serial << 8 | index`); returns `n`.
    fn channels(c: &mut Cpu, r: &mut Rng, n: u32) -> u32 {
        c.bus.write32(snd::CHANNELS.0, CHANNELS);
        c.bus.write16(snd::CHANNEL_COUNT.0, n as u16);
        for i in 0..n {
            let ch = CHANNELS + i * snd::CHANNEL_SIZE;
            fill(c, r, ch, snd::CHANNEL_SIZE / 4);
            c.bus.write8(ch + 0x26, (r.next_u32() % 3) as u8);
            let alloc = if r.next_u32() % 4 != 0 { 1 } else { r.next_u32() as u8 % 3 };
            c.bus.write8(ch + 0x69, alloc);
            let h = if r.next_u32() % 5 != 0 { (r.next_u32() << 8 | i) & 0x7fff_ffff } else { r.next_u32() };
            c.bus.write32(ch, h);
            // No voices (StopVoices / ApplyPitch loops are tested elsewhere).
            c.bus.write8(ch + 0x23, 0);
        }
        n
    }

    /// A handle: mostly the handle of one of the channels.
    fn handle(c: &mut Cpu, r: &mut Rng, n: u32) -> u32 {
        if n > 0 && r.next_u32() % 5 != 0 {
            c.bus.read32(CHANNELS + (r.next_u32() % n) * snd::CHANNEL_SIZE)
        } else {
            r.next_u32()
        }
    }

    #[test]
    fn is_handle_done() {
        check(s13::f_003b8ab8, snd_is_handle_done, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 1, 0);
            let n = 1 + r.next_u32() % 8;
            let n = channels(c, &mut r, n);
            let h = handle(c, &mut r, n);
            c.g[A0][0] = r64!(&mut r, h);
        });
    }

    fn stop_setup(c: &mut Cpu, r: &mut Rng) {
        let n = r.next_u32() % 16;
        let n = channels(c, r, n);
        let h = handle(c, r, n);
        c.g[A0][0] = r64!(r, h);
    }

    driven!(gen_stop, s13::f_003b89e0);
    driven!(hand_stop, snd_stop);

    #[test]
    fn stop() {
        check_lohi(s13::f_003b89e0, snd_stop, 150, &|c, seed| {
            let mut r = machine(c, seed, 2, 0);
            c.pc = stop::START;
            stop_setup(c, &mut r);
        });
        check_events(gen_stop, hand_stop, 150, &|c, seed| {
            let mut r = machine(c, seed, 3, 0);
            stop_setup(c, &mut r);
            schedule(c, &mut r, seed, stop::START)
        });
    }

    #[test]
    fn sif_cmd_handler() {
        covered(s13::f_003c1578, snd_sif_cmd_handler, 200, &|c, seed| {
            let mut r = machine(c, seed, 4, 1);
            let pkt = ptr(&mut r);
            fill(c, &mut r, pkt, 8);
            let cmd = match r.next_u32() % 6 {
                0 | 1 => 0,
                2 => 1,
                3 => 2,
                4 => 3 + r.next_u32() % 4,
                _ => r.next_u32(),
            };
            c.bus.write32(pkt + 0x10, cmd);
            // SND_SifDmaComplete's state: no pending transfers.
            c.bus.write32(pkt + 0xc, r.next_u32() % 4);
            c.g[A0][0] = r64!(&mut r, pkt);
        });
    }

    #[test]
    fn save_vu0_context() {
        check_paths(s13::f_003c0858, snd_save_vu0_context, SEEDS, &[40], &|c, seed| {
            let mut r = machine(c, seed, 5, 0);
            for reg in 16..19 {
                c.bus.plat.write_ctrl(reg, r.next_u32());
            }
            c.g[A0][0] = r64!(&mut r, ptr(&mut r));
        });
    }

    fn dsp_chain(c: &mut Cpu, r: &mut Rng) {
        let head = ptr(r);
        // A longer chain so the sorted-insert walk reaches the loop's event
        // back edge in most seeds (the event-coverage assertion needs it).
        let n = 5 + r.next_u32() % 8;
        let mut prio = (r.next_u32() % 4) as u16;
        let mut prev = 0;
        let nodes: Vec<u32> = (0..n).map(|i| DATA + 0x20_0000 + i * 0x40).collect();
        c.bus.write32(head, nodes.first().copied().unwrap_or(0));
        for (i, &nd) in nodes.iter().enumerate() {
            fill(c, r, nd, 0x10);
            c.bus.write16(nd + 0x18, prio);
            prio += (r.next_u32() % 3) as u16;
            c.bus.write32(nd + 0x10, prev);
            c.bus.write32(nd + 0x8, nodes.get(i + 1).copied().unwrap_or(0));
            prev = nd;
        }
        let node = DATA + 0x30_0000;
        fill(c, r, node, 0x10);
        c.bus.write16(node + 0x18, (r.next_u32() % 14) as u16);
        c.g[A0][0] = r64!(r, head);
        c.g[A1][0] = r64!(r, node);
    }

    driven!(gen_insert, s14::f_003c8f48);
    driven!(hand_insert, snd_dsp_insert_sorted);

    #[test]
    fn dsp_insert_sorted() {
        check(s14::f_003c8f48, snd_dsp_insert_sorted, 100, &|c, seed| {
            let mut r = machine(c, seed, 6, 0);
            dsp_chain(c, &mut r);
        });
        check_events(gen_insert, hand_insert, 100, &|c, seed| {
            let mut r = machine(c, seed, 7, 0);
            dsp_chain(c, &mut r);
            schedule(c, &mut r, seed, 0x003c_8f48)
        });
    }

    fn bit_reader(c: &mut Cpu, r: &mut Rng) {
        crate::difftest::elf_image(c);
        let br = ptr(r);
        let input = DATA + 0x20_0000;
        fill(c, r, input, 4);
        c.bus.write32(br, input + r.next_u32() % 8);
        c.bus.write32(br + 4, r.next_u32());
        c.bus.write32(br + 8, 8 + r.next_u32() % 25);
        c.g[A0][0] = r64!(r, br);
        c.g[A1][0] = r64!(r, r.next_u32() % 17);
    }

    #[test]
    fn utk_read_and_skip_bits() {
        check_paths(s14::f_003cd1f8, snd_utk_read_bits, SEEDS, &[17, 26], &|c, seed| {
            let mut r = machine(c, seed, 8, 0);
            bit_reader(c, &mut r);
        });
        check_paths(s14::f_003cd260, snd_utk_skip_bits, SEEDS, &[11, 20], &|c, seed| {
            let mut r = machine(c, seed, 9, 0);
            bit_reader(c, &mut r);
        });
    }

    #[test]
    fn utk_synthesize() {
        check(s14::f_003cd690, snd_utk_synthesize, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 10, 0);
            let s = ptr(&mut r);
            for i in 0..0x400 {
                c.bus.write32(s + 4 * i, r.f32(1.0));
            }
            c.g[A0][0] = r64!(&mut r, s);
            c.g[A1][0] = r64!(&mut r, r.next_u32() % 64);
            c.g[A2][0] = sx(r.next_u32() % 5);
        });
    }

    #[test]
    fn utk_interpolate() {
        check(s14::f_003cd518, snd_utk_interpolate, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 19, 0);
            let a0 = DATA + 0x4000;
            for i in 0..0x100 {
                c.bus.write32(a0 - 0x20 + i * 4, r.edge_f32());
            }
            c.g[A0][0] = r64!(&mut r, a0);
        });
    }

    #[test]
    fn utk_rc_to_lpc() {
        check(s14::f_003cd590, snd_utk_rc_to_lpc, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 22, 0);
            let a0 = DATA + 0x2000;
            for i in 0..0x40 {
                c.bus.write32(a0 + i * 4, r.edge_f32());
            }
            let a1 = DATA + 0x3000;
            for i in 0..0x80 {
                c.bus.write32(a1 + i * 4, r.edge_f32());
            }
            c.g[A0][0] = r64!(&mut r, a0);
            c.g[A1][0] = r64!(&mut r, a1);
        });
    }

    #[test]
    fn dec_utk_decode_mem() {
        check(s14::f_003c9360, snd_dec_utk_decode_mem, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 21, 0);
            let d = DATA + 0x1000;
            fill(c, &mut r, d, 0x30);
            let out = DATA + 0x8000;
            fill(c, &mut r, out, 0x40);
            // `d->pos < d->remaining` makes it decode; otherwise it returns -1.
            let less = seed % 4 != 0;
            c.bus.write32(d + 0x1c, if less { 0x1000 } else { 0 });
            c.bus.write32(d + 0x20, if less { 0 } else { 0x2000 });
            c.bus.write32(d + 0x28, out);
            c.g[A0][0] = r64!(&mut r, d);
            c.g[A1][0] = r64!(&mut r, r.next_u32() % 0x100);
            c.g[A2][0] = r64!(&mut r, r.next_u32());
            c.g[A3][0] = r64!(&mut r, r.next_u32());
        });
    }

    // --- allocator ---------------------------------------------------------
    /// A heap descriptor with `k` blocks (sorted, with random gaps) at a
    /// table that is sometimes only 4-byte aligned; returns the block
    /// offsets.
    fn heap_setup(c: &mut Cpu, r: &mut Rng) -> (u32, Vec<u32>) {
        let h = DATA + 0x100;
        let base = DATA + 0x10_0000;
        let table = DATA + 0x2000 + 4 * (r.next_u32() % 2);
        c.bus.write32(snd::STATE.0 + heap::PTR, h);
        let k = r.next_u32() % 7;
        let mut offs = Vec::new();
        let mut at = if r.next_u32() % 3 == 0 { 0 } else { 16 * (r.next_u32() % 8) };
        for i in 0..k {
            let size = 16 * (1 + r.next_u32() % 8);
            let e = table.wrapping_sub(8 * i);
            c.bus.write32(e, at);
            c.bus.write32(e + 4, size);
            offs.push(at);
            at += size + if r.next_u32() % 2 == 0 { 0 } else { 16 * (r.next_u32() % 6) };
        }
        let lowest = if r.next_u32() % 20 == 0 { r.next_u32() % 3 } else { (k as u32).wrapping_neg() };
        c.bus.write32(h + heap::BASE, base);
        c.bus.write32(h + heap::BLOCKS, table);
        c.bus.write32(h + heap::LIMIT, at + 16 * (r.next_u32() % 12));
        c.bus.write32(h + heap::LOW_WATER, r.next_u32() % 0x100);
        c.bus.write32(h + heap::LOWEST, lowest);
        (base, offs)
    }

    fn mem_free_setup(c: &mut Cpu, r: &mut Rng) {
        let (base, offs) = heap_setup(c, r);
        let p = if !offs.is_empty() && r.next_u32() % 4 != 0 {
            base + offs[(r.next_u32() as usize) % offs.len()]
        } else {
            base + 16 * (r.next_u32() % 32)
        };
        c.g[A0][0] = r64!(r, p);
    }

    driven!(gen_mem_free, s13::f_003b5e98);
    driven!(hand_mem_free, snd_mem_free);

    #[test]
    fn mem_free() {
        check(s13::f_003b5e98, snd_mem_free, 100, &|c, seed| {
            let mut r = machine(c, seed, 11, 0);
            mem_free_setup(c, &mut r);
        });
        check_events(gen_mem_free, hand_mem_free, 100, &|c, seed| {
            let mut r = machine(c, seed, 12, 0);
            mem_free_setup(c, &mut r);
            schedule(c, &mut r, seed, 0x003b_5e98)
        });
    }

    fn mem_alloc_setup(c: &mut Cpu, r: &mut Rng) {
        heap_setup(c, r);
        let size = match r.next_u32() % 4 {
            0 => r.next_u32() % 0x40,
            1 | 2 => r.next_u32() % 0x100,
            _ => r.next_u32(),
        };
        c.g[A0][0] = r64!(r, size);
    }

    driven!(gen_mem_alloc, s13::f_003b5c60);
    driven!(hand_mem_alloc, snd_mem_alloc);

    #[test]
    fn mem_alloc() {
        check(s13::f_003b5c60, snd_mem_alloc, 100, &|c, seed| {
            let mut r = machine(c, seed, 13, 0);
            mem_alloc_setup(c, &mut r);
        });
        check_events(gen_mem_alloc, hand_mem_alloc, 100, &|c, seed| {
            let mut r = machine(c, seed, 14, 0);
            mem_alloc_setup(c, &mut r);
            schedule(c, &mut r, seed, 0x003b_5c60)
        });
    }

    fn mem_copy_setup(c: &mut Cpu, r: &mut Rng) {
        let src = DATA + 0x1000 + r.next_u32() % 64;
        let dst = DATA + 0x8000 + r.next_u32() % 64;
        let dst = if r.next_u32() % 3 == 0 { dst & !0xf } else { dst };
        let src = if r.next_u32() % 2 == 0 { (src & !0xf) | (dst & 0xf) } else { src };
        fill(c, r, DATA + 0x1000, 0x200);
        let n = match r.next_u32() % 5 {
            0 => r.next_u32() % 20,
            1 => (r.next_u32() % 8).wrapping_sub(4),
            _ => r.next_u32() % 700,
        };
        c.g[A0][0] = r64!(r, dst);
        c.g[A1][0] = r64!(r, src);
        c.g[A2][0] = r64!(r, n);
    }

    driven!(gen_mem_copy, s13::f_003c51e8);
    driven!(hand_mem_copy, snd_mem_copy);

    #[test]
    fn mem_copy() {
        check(s13::f_003c51e8, snd_mem_copy, 100, &|c, seed| {
            let mut r = machine(c, seed, 15, 0);
            mem_copy_setup(c, &mut r);
        });
        check_events(gen_mem_copy, hand_mem_copy, 100, &|c, seed| {
            let mut r = machine(c, seed, 16, 0);
            mem_copy_setup(c, &mut r);
            schedule(c, &mut r, seed, 0x003c_51e8)
        });
    }

    // --- channels ----------------------------------------------------------

    /// Channels in one or two groups with leaders and parked leaders.
    fn release_setup(c: &mut Cpu, r: &mut Rng) {
        let n = 1 + r.next_u32() % 10;
        let n = channels(c, r, n);
        c.bus.write32(snd::STATE.0 + snd_state::TICKS, r.next_u32());
        let leader = r.next_u32() % n;
        for i in 0..n {
            let ch = CHANNELS + i * snd::CHANNEL_SIZE;
            c.bus.write8(ch + chan::GROUP, if r.next_u32() % 4 == 0 { 0 } else { 1 + r.next_u32() % 2 } as u8);
            c.bus.write8(ch + chan::LEADER, (i == leader || r.next_u32() % 8 == 0) as u8);
            let a = match r.next_u32() % 4 {
                0 => 0,
                1 => 2,
                _ => 1,
            };
            c.bus.write8(ch + chan::ALLOCATED, if i == leader && r.next_u32() % 2 == 0 { 2 } else { a });
        }
        let i = if r.next_u32() % 3 == 0 { leader } else { r.next_u32() % n };
        c.g[A0][0] = r64!(r, i);
    }

    driven!(gen_release, s13::f_003ba550);
    driven!(hand_release, snd_channel_release);

    #[test]
    fn channel_release() {
        check_lohi(s13::f_003ba550, snd_channel_release, 100, &|c, seed| {
            let mut r = machine(c, seed, 17, 0);
            release_setup(c, &mut r);
        });
        check_events(gen_release, hand_release, 100, &|c, seed| {
            let mut r = machine(c, seed, 18, 0);
            release_setup(c, &mut r);
            schedule(c, &mut r, seed, 0x003b_a550)
        });
    }
}
