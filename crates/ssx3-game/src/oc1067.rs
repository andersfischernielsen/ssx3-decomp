//! Staging module oc1067 (wave oc1067): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1067`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003f_6960, voip_rpc_call, "VoipRpc_Call"),
    (0x003f_6a40, voip_rpc_init, "VoipRpc_Init"),
    (0x003f_6ab0, voip_rpc_disable, "VoipRpc_Disable"),
    (0x003f_6ab8, voip_rpc_start, "VoipRpc_Start"),
    (0x003f_6af0, voip_rpc_stop, "VoipRpc_Stop"),
    (0x003f_6b20, voip_rpc_enable_voice, "VoipRpc_EnableVoice"),
    (0x003f_6b50, voip_rpc_disable_voice, "VoipRpc_DisableVoice"),
    (0x003f_6b80, voip_rpc_is_active, "VoipRpc_IsActive"),
    (0x003f_6ba0, voip_rpc_get_status, "VoipRpc_GetStatus"),
    (0x003f_6c10, voip_rpc_set_params, "VoipRpc_SetParams"),
    (0x003f_6c40, voip_rpc_get_shared_status, "VoipRpc_GetSharedStatus"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub const VOIP_RPC_CALL: Callee = Callee {
        addr: 0x003f_6960,
        f: ee::s14::f_003f6960,
    };

    pub const VOIP_RPC_GET_SHARED_STATUS: Callee = Callee {
        addr: 0x003f_6c40,
        f: ee::s14::f_003f6c40,
    };
}

/// Cross-shard callees (reached through the function table, as generated).
pub(crate) mod table {
    pub const SCE_SIF_CHECK_STAT_RPC: u32 = 0x0042_6f08;
    pub const SCE_SIF_CALL_RPC: u32 = 0x0042_6d18;
    pub const SCE_SIF_BIND_RPC: u32 = 0x0042_6b48;
}

// ---------------------------------------------------------------------------
// The voice-chat IOP RPC client.

/// gp-relative byte: the client is bound and calls go out (`VoipRpc_Init`
/// sets it to 1, `VoipRpc_Disable` clears it; `VoipRpc_Call` does nothing
/// while it is 0).
pub(crate) const VOIP_RPC_ENABLED_GP: u32 = 0x2a24;
/// gp-relative word: the uncached address of the status block the IOP
/// module shares with the EE (`VoipRpc_Init` stores it).
pub(crate) const VOIP_RPC_SHARED_STATUS_GP: u32 = 0x2a28;
/// The high half (`lui 0x52`) the code builds the client's addresses from.
pub(crate) const VOIP_RPC_HI: u32 = 0x0052_0000;
/// The send buffer: the two argument words of a call (0x80 bytes sent).
pub(crate) const VOIP_RPC_SEND_BUF: u32 = 0x0052_6080;
/// The SIF RPC client (`sceSifClientData`) bound to the voice server.
pub(crate) const VOIP_RPC_CLIENT: u32 = 0x0052_6100;
/// The receive buffer the commands pass (the IOP writes its answer there).
pub(crate) const VOIP_RPC_RECV_BUF: u32 = 0x0052_6140;
/// The status block shared with the IOP module (its first word's bit 0:
/// voice active).
pub(crate) const VOIP_RPC_SHARED_STATUS: u32 = 0x0052_61c0;
/// Bytes sent from the send buffer, and the receive size the commands pass.
pub(crate) const VOIP_RPC_BUF_SIZE: u64 = 0x80;
/// The IOP voice module's RPC server id.
pub(crate) const VOIP_RPC_SERVER_ID: u64 = 0x237;
/// `addr & KSEG_ADDR_MASK | UNCACHED_SEGMENT`: an address's uncached mirror.
const KSEG_ADDR_MASK: u64 = 0x0fff_ffff;
const UNCACHED_SEGMENT: u64 = 0x2000_0000;
/// Shared status bit 0: voice is active.
const VOIP_SHARED_ACTIVE: u64 = 0x1;

/// RPC command numbers (`fno`).
pub(crate) const VOIP_RPC_CMD_INIT: u64 = 0;
pub(crate) const VOIP_RPC_CMD_START: u64 = 2;
pub(crate) const VOIP_RPC_CMD_STOP: u64 = 3;
pub(crate) const VOIP_RPC_CMD_ENABLE_VOICE: u64 = 4;
pub(crate) const VOIP_RPC_CMD_DISABLE_VOICE: u64 = 5;
pub(crate) const VOIP_RPC_CMD_SET_PARAMS: u64 = 6;
pub(crate) const VOIP_RPC_CMD_GET_STATUS: u64 = 7;

/// Loop heads of `VoipRpc_Call`: waiting for the previous call, retrying
/// `sceSifCallRpc`.
pub(crate) const VOIP_RPC_CALL_WAIT_HEAD: u32 = 0x003f_69c0;
pub(crate) const VOIP_RPC_CALL_RETRY_HEAD: u32 = 0x003f_69e0;

/// `int VoipRpc_Call(int fno, u32 arg0, u32 arg1, void* recv, int recv_size)`
/// (0x3f6960): while the client is enabled, wait until the previous call has
/// finished (`sceSifCheckStatRpc` > 0), put the two argument words in the
/// send buffer and issue `sceSifCallRpc(client, fno, 0, send, 0x80, recv,
/// recv_size, NULL, NULL)`, retrying while it fails. Returns its result (0);
/// disabled, it returns at once with v0 = 0x520000 left over.
pub fn voip_rpc_call(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = VOIP_RPC_HI as u64;
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x60);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S5, 0x50);
        c.g[S4][0] = c.g[T0][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S7, 0x30);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, FP, 0x20);
        c.g[S7][0] = sx(VOIP_RPC_SEND_BUF);
        let enabled_at = r32(c, GP).wrapping_add(VOIP_RPC_ENABLED_GP);
        c.g[V1][0] = Addr(enabled_at).lbu(c);
        c.g[FP][0] = c.g[V0][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save_ra(c, 0x10);
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[S1][0] = VOIP_RPC_HI as u64;
            c.g[S0][0] = c.g[S1][0];
            // Wait for the previous call to finish.
            loop {
                c.cycles += 4;
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(VOIP_RPC_CLIENT - VOIP_RPC_HI));
                call_ptr(c, table::SCE_SIF_CHECK_STAT_RPC, 0x003f_69c8)?;
                if (c.g[V0][0] as i64) <= 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, VOIP_RPC_CALL_WAIT_HEAD);
                }
            }
            c.cycles += 4;
            let send = r32(c, S7);
            let (arg0, arg1) = (c.g[S3][0], c.g[S2][0]);
            Addr(send.wrapping_add(4)).sw(c, arg1);
            Addr(send).sw(c, arg0);
            // The ninth argument (end_param), on the stack.
            Addr(sp(c)).sw(c, 0);
            loop {
                c.cycles += 11;
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(VOIP_RPC_CLIENT - VOIP_RPC_HI));
                c.g[A1][0] = c.g[S6][0];
                c.g[A2][0] = 0;
                c.g[A3][0] = sx(r32(c, FP).wrapping_add(VOIP_RPC_SEND_BUF - VOIP_RPC_HI));
                c.g[T0][0] = VOIP_RPC_BUF_SIZE;
                c.g[T1][0] = c.g[S5][0];
                c.g[T2][0] = c.g[S4][0];
                c.g[T3][0] = 0;
                call_ptr(c, table::SCE_SIF_CALL_RPC, 0x003f_6a04)?;
                if (c.g[V0][0] as i64) >= 0 {
                    break;
                }
                Addr(sp(c)).sw(c, 0);
                if event_due(c) {
                    deliver_event_here(c, VOIP_RPC_CALL_RETRY_HEAD);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void VoipRpc_Init(void)` (0x3f6a40): enable the client, bind it to the
/// IOP voice server (`sceSifBindRpc(client, 0x237, 0)`, result ignored),
/// store the shared status block's uncached address at `gp+0x2a28` and send
/// it to the IOP with command 0 (`VoipRpc_Call(0, status, 0, recv, 0x80)`).
pub fn voip_rpc_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(VOIP_RPC_CLIENT);
        c.g[A1][0] = VOIP_RPC_SERVER_ID;
        c.g[A2][0] = 0;
        let enabled_at = r32(c, GP).wrapping_add(VOIP_RPC_ENABLED_GP);
        let enabled = c.g[V0][0];
        Addr(enabled_at).sb(c, enabled);
        call_ptr(c, table::SCE_SIF_BIND_RPC, 0x003f_6a64)?;
        c.g[V0][0] = KSEG_ADDR_MASK;
        c.g[V1][0] = sx(VOIP_RPC_SHARED_STATUS) & c.g[V0][0];
        c.g[A0][0] = UNCACHED_SEGMENT;
        c.g[V1][0] |= c.g[A0][0];
        c.g[A3][0] = sx(VOIP_RPC_RECV_BUF);
        c.g[A1][0] = c.g[V1][0];
        let status_ptr_at = r32(c, GP).wrapping_add(VOIP_RPC_SHARED_STATUS_GP);
        let status = c.g[V1][0];
        Addr(status_ptr_at).sw(c, status);
        c.g[A0][0] = VOIP_RPC_CMD_INIT;
        c.g[A2][0] = 0;
        c.g[T0][0] = VOIP_RPC_BUF_SIZE;
        callee::VOIP_RPC_CALL.call(c, 0x003f_6aa0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VoipRpc_Disable(void)` (0x3f6ab0): clear the enabled flag, so
/// later `VoipRpc_Call`s do nothing.
pub fn voip_rpc_disable(c: &mut Cpu) {
    c.cycles += 2;
    let enabled_at = r32(c, GP).wrapping_add(VOIP_RPC_ENABLED_GP);
    Addr(enabled_at).sb(c, 0);
    ret(c);
}

/// The one-command wrappers' common tail: a 0x10 frame holding `ra`, then
/// `VoipRpc_Call(fno, a1, a2, recv, 0x80)` returning to `ret_to` (the caller
/// has charged its cycles and set a1/a2).
fn voip_rpc_send_command(c: &mut Cpu, fno: u64, ret_to: u32) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = sx(VOIP_RPC_RECV_BUF);
        c.g[A0][0] = fno;
        c.g[T0][0] = VOIP_RPC_BUF_SIZE;
        callee::VOIP_RPC_CALL.call(c, ret_to)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VoipRpc_Start(u32 arg, u32 lo, u32 hi)` (0x3f6ab8): command 2 with
/// the word `arg` and the word `hi << 16 | (lo & 0xffff)`.
pub fn voip_rpc_start(c: &mut Cpu) {
    c.cycles += 14;
    c.g[A1][0] &= 0xffff;
    c.g[A2][0] = sx(r32(c, A2) << 16);
    c.g[A2][0] |= c.g[A1][0];
    c.g[A1][0] = c.g[A0][0];
    voip_rpc_send_command(c, VOIP_RPC_CMD_START, 0x003f_6ae4);
}

/// `void VoipRpc_Stop(void)` (0x3f6af0): command 3, arguments 0.
pub fn voip_rpc_stop(c: &mut Cpu) {
    c.cycles += 12;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    voip_rpc_send_command(c, VOIP_RPC_CMD_STOP, 0x003f_6b14);
}

/// `void VoipRpc_EnableVoice(void)` (0x3f6b20): command 4, arguments 0.
pub fn voip_rpc_enable_voice(c: &mut Cpu) {
    c.cycles += 12;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    voip_rpc_send_command(c, VOIP_RPC_CMD_ENABLE_VOICE, 0x003f_6b44);
}

/// `void VoipRpc_DisableVoice(void)` (0x3f6b50): command 5, arguments 0.
pub fn voip_rpc_disable_voice(c: &mut Cpu) {
    c.cycles += 12;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    voip_rpc_send_command(c, VOIP_RPC_CMD_DISABLE_VOICE, 0x003f_6b74);
}

/// `int VoipRpc_IsActive(void)` (0x3f6b80): bit 0 of the shared status word
/// (`VoipRpc_GetSharedStatus() & 1`).
pub fn voip_rpc_is_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::VOIP_RPC_GET_SHARED_STATUS.call(c, 0x003f_6b90)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] &= VOIP_SHARED_ACTIVE;
        frame.close(c);
        Ok(())
    })
}

/// `void VoipRpc_GetStatus(u32* out0, u32* out1)` (0x3f6ba0): command 7
/// (arguments 0) into the receive buffer, then copy its first two words to
/// `*out0` and `*out1`.
pub fn voip_rpc_get_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0;
        frame.save(c, S1, 0x30);
        c.g[T0][0] = VOIP_RPC_BUF_SIZE;
        frame.save(c, S0, 0x40);
        c.g[S1][0] = VOIP_RPC_HI as u64;
        frame.save(c, S2, 0x20);
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(VOIP_RPC_RECV_BUF - VOIP_RPC_HI));
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = VOIP_RPC_CMD_GET_STATUS;
        c.g[A1][0] = 0;
        callee::VOIP_RPC_CALL.call(c, 0x003f_6be0)?;
        let first_at = r32(c, S1).wrapping_add(VOIP_RPC_RECV_BUF - VOIP_RPC_HI);
        c.g[V0][0] = Addr(first_at).lw(c);
        frame.restore(c, S1, 0x30);
        let (out0, first) = (r32(c, S2), c.g[V0][0]);
        Addr(out0).sw(c, first);
        frame.restore(c, S2, 0x20);
        let second_at = r32(c, S0).wrapping_add(4);
        c.g[V1][0] = Addr(second_at).lw(c);
        frame.restore(c, S0, 0x40);
        let (out1, second) = (r32(c, S3), c.g[V1][0]);
        Addr(out1).sw(c, second);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VoipRpc_SetParams(u32 arg0, u32 arg1)` (0x3f6c10): command 6 with
/// the two words.
pub fn voip_rpc_set_params(c: &mut Cpu) {
    c.cycles += 12;
    c.g[A2][0] = c.g[A1][0];
    c.g[A1][0] = c.g[A0][0];
    voip_rpc_send_command(c, VOIP_RPC_CMD_SET_PARAMS, 0x003f_6c34);
}

/// `u32 VoipRpc_GetSharedStatus(void)` (0x3f6c40): the first word of the
/// status block the IOP shares (read through the pointer at `gp+0x2a28`).
pub fn voip_rpc_get_shared_status(c: &mut Cpu) {
    c.cycles += 3;
    let ptr_at = r32(c, GP).wrapping_add(VOIP_RPC_SHARED_STATUS_GP);
    c.g[V1][0] = Addr(ptr_at).lw(c);
    let status_at = r32(c, V1);
    c.g[V0][0] = Addr(status_at).lw(c);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{covered, machine};
    use crate::difftest::{
        check, fill, fresh, run_generated, scramble, Rng, CYCLE_BUDGET, DATA, GAME_GP,
    };

    /// libkernel's SIF RPC packet pool: +0 the request counter, +4 the
    /// packets (0x40 bytes each), +8 their number.
    const VOIP_RPC_TEST_SIF_POOL: u32 = 0x0052_d680;
    /// The SIF command word `_sceSifSendCmd` reads.
    const VOIP_RPC_TEST_SIF_CMD_WORD: u32 = 0x0052_bce0;
    /// A packet's flags (bit 0: in use) and request id.
    const VOIP_RPC_TEST_PACKET_FLAGS: u32 = 0x10;
    const VOIP_RPC_TEST_PACKET_ID: u32 = 0x18;
    /// The client's current packet and request id.
    const VOIP_RPC_TEST_CLIENT_PACKET: u32 = 0x0;
    const VOIP_RPC_TEST_CLIENT_ID: u32 = 0x4;
    /// The previous call's packet (the client's, when there is one).
    const VOIP_RPC_TEST_PREV_PACKET: u32 = DATA + 0x400;
    /// The pool's two packets.
    const VOIP_RPC_TEST_PACKETS: u32 = DATA + 0x1000;
    /// The test's IOP side: +0 deliveries at the wait head before the
    /// pending call completes, +4 those seen; +8 deliveries at the retry
    /// head before a packet frees up, +0xc those seen.
    const VOIP_RPC_TEST_IOP: u32 = DATA + 0x700;

    /// What the IOP side looks like when `VoipRpc_Call` runs.
    #[derive(Clone, Copy)]
    struct VoipRpcIop {
        /// The enabled flag byte.
        enabled: u8,
        /// Wait-loop rounds before the previous call completes (0: none
        /// pending).
        waits: u32,
        /// With nothing pending, why `sceSifCheckStatRpc` says 0: no
        /// packet (0), another request's packet (1), its in-use bit clear (2).
        idle: u32,
        /// `sceSifCallRpc` failures (no free packet) before one frees up.
        retries: u32,
        /// The failing pool is empty (else both packets are in use).
        empty_pool: bool,
    }

    /// The client, its buffers and the SIF packet pool as `iop` says, all
    /// other words junk.
    fn voip_rpc_fixture(c: &mut Cpu, r: &mut Rng, iop: VoipRpcIop) {
        c.bus.write8(GAME_GP + VOIP_RPC_ENABLED_GP, iop.enabled);
        // Send buffer, client, receive buffer, shared status block.
        fill(c, r, VOIP_RPC_SEND_BUF, 0x60);
        fill(c, r, VOIP_RPC_TEST_SIF_CMD_WORD, 1);
        let prev = VOIP_RPC_TEST_PREV_PACKET;
        fill(c, r, prev, 0x10);
        let id = r.next_u32();
        c.bus.write32(prev + VOIP_RPC_TEST_PACKET_ID, id);
        let flags = r.next_u32();
        let (packet, client_id, flags) = if iop.waits > 0 {
            (prev, id, flags | 1)
        } else {
            match iop.idle {
                0 => (0, id, flags | 1),
                1 => (prev, id ^ 0x10, flags | 1),
                _ => (prev, id, flags & !1),
            }
        };
        c.bus.write32(VOIP_RPC_CLIENT + VOIP_RPC_TEST_CLIENT_PACKET, packet);
        c.bus.write32(VOIP_RPC_CLIENT + VOIP_RPC_TEST_CLIENT_ID, client_id);
        c.bus.write32(prev + VOIP_RPC_TEST_PACKET_FLAGS, flags);
        let count = if iop.retries > 0 && iop.empty_pool { 0 } else { 2 };
        let counter = r.next_u32() | 0x10;
        c.bus.write32(VOIP_RPC_TEST_SIF_POOL, counter);
        c.bus.write32(VOIP_RPC_TEST_SIF_POOL + 4, VOIP_RPC_TEST_PACKETS);
        c.bus.write32(VOIP_RPC_TEST_SIF_POOL + 8, count);
        fill(c, r, VOIP_RPC_TEST_PACKETS, 0x20);
        for i in 0..2u32 {
            let f = r.next_u32();
            let in_use = i == 1 || iop.retries > 0;
            let f = if in_use { f | 1 } else { f & !1 };
            c.bus
                .write32(VOIP_RPC_TEST_PACKETS + i * 0x40 + VOIP_RPC_TEST_PACKET_FLAGS, f);
        }
        let iop_words = [iop.waits, 0, iop.retries, 0];
        for (i, w) in iop_words.into_iter().enumerate() {
            c.bus.write32(VOIP_RPC_TEST_IOP + 4 * i as u32, w);
        }
    }

    /// The IOP's progress at an event: at the wait head the pending call
    /// completes (its packet's in-use bit clears) after the set number of
    /// deliveries; at the retry head a packet frees up (the pool's last;
    /// an empty pool gets one).
    fn voip_rpc_iop_step(c: &mut Cpu) {
        let (target, seen) = match c.pc {
            VOIP_RPC_CALL_WAIT_HEAD => (0, 4),
            VOIP_RPC_CALL_RETRY_HEAD => (8, 0xc),
            _ => return,
        };
        let n = c.bus.read32(VOIP_RPC_TEST_IOP + seen) + 1;
        c.bus.write32(VOIP_RPC_TEST_IOP + seen, n);
        if n < c.bus.read32(VOIP_RPC_TEST_IOP + target) {
            return;
        }
        let flags_at = if c.pc == VOIP_RPC_CALL_WAIT_HEAD {
            c.bus.read32(VOIP_RPC_CLIENT + VOIP_RPC_TEST_CLIENT_PACKET)
                + VOIP_RPC_TEST_PACKET_FLAGS
        } else {
            let mut count = c.bus.read32(VOIP_RPC_TEST_SIF_POOL + 8);
            if count == 0 {
                count = 1;
                c.bus.write32(VOIP_RPC_TEST_SIF_POOL + 8, count);
            }
            c.bus.read32(VOIP_RPC_TEST_SIF_POOL + 4)
                + (count - 1) * 0x40
                + VOIP_RPC_TEST_PACKET_FLAGS
        };
        let f = c.bus.read32(flags_at);
        c.bus.write32(flags_at, f & !1);
    }

    /// Event hook of the plain tests (`next_event` 0: every check delivers):
    /// the IOP steps, nothing else happens.
    fn voip_rpc_iop_event(c: &mut Cpu) -> bool {
        assert!(c.cycles < CYCLE_BUDGET, "runaway at pc {:#x}", c.pc);
        voip_rpc_iop_step(c);
        false
    }

    /// Event hook of the event tests: the IOP steps, then the machine's real
    /// event delivery.
    fn voip_rpc_iop_live_event(c: &mut Cpu) -> bool {
        assert!(c.cycles < CYCLE_BUDGET, "runaway at pc {:#x}", c.pc);
        voip_rpc_iop_step(c);
        (ssx3_ps2::platform::HOOKS.event)(c)
    }

    /// The arms of `VoipRpc_Call`: disabled (`seed % 4 == 0`), else
    /// 0-2 wait rounds (`seed / 4 % 3`) crossed with 0-2 call retries
    /// (`seed / 36 % 3`). `seed / 12 % 3` picks why nothing is pending
    /// (no wait rounds), `seed / 108 % 2` an empty or full pool for the
    /// retries and the receive buffer. The four syscalls of a successful
    /// `sceSifCallRpc` are fakes (some unwinding); everything else is real.
    #[test]
    fn voip_rpc_call_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x3f_6960, 4);
            c.pc = 0x003f_6960;
            let iop = VoipRpcIop {
                enabled: [0u8, 1, 0x80, 0xff][(seed % 4) as usize],
                waits: seed / 4 % 3,
                idle: seed / 12 % 3,
                retries: seed / 36 % 3,
                empty_pool: seed / 108 % 2 == 1,
            };
            voip_rpc_fixture(c, &mut r, iop);
            c.next_event = 0;
            c.hooks.event = voip_rpc_iop_event;
            let (recv, size) = if seed / 108 % 2 == 0 {
                (VOIP_RPC_RECV_BUF, 0x80)
            } else {
                (DATA + 0x800, 0x40)
            };
            fill(c, &mut r, DATA + 0x800, 0x10);
            c.g[A0][0] = sx(r.next_u32() % 16);
            c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.g[A3][0] = sx(recv);
            c.g[T0][0] = size;
        };
        covered(ee::s14::f_003f6960__generated, voip_rpc_call, 216, &setup);
        // Every arm ran: (enabled, wait rounds, retries) as the IOP side saw them.
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..216 {
            let mut c = fresh();
            setup(&mut *c, seed);
            run_generated(&mut c, ee::s14::f_003f6960__generated);
            let enabled = c.bus.read8(GAME_GP + VOIP_RPC_ENABLED_GP) != 0;
            let waits = c.bus.read32(VOIP_RPC_TEST_IOP + 4);
            let retries = c.bus.read32(VOIP_RPC_TEST_IOP + 0xc);
            seen.insert((enabled, waits, retries));
        }
        assert!(seen.contains(&(false, 0, 0)), "disabled arm not run: {seen:?}");
        for waits in 0..3 {
            for retries in 0..3 {
                assert!(
                    seen.contains(&(true, waits, retries)),
                    "arm (waits {waits}, retries {retries}) not run: {seen:?}"
                );
            }
        }
    }

    /// `VoipRpc_Call` with real events: `seed % 3` the event mode; with
    /// events on, `seed / 3 % 2` makes the first due check the wait head
    /// (one round pending) or the retry head (an empty pool, one retry).
    #[test]
    fn voip_rpc_call_events() {
        driven!(gen, ee::s14::f_003f6960__generated);
        driven!(hand, voip_rpc_call);
        check_events(gen, hand, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x3f_6961, 0);
            let (waits, retries) = match (seed % 3, seed / 3 % 2) {
                (0, _) => (0, 0),
                (_, 0) => (1, 0),
                _ => (0, 1),
            };
            let iop = VoipRpcIop {
                enabled: 1,
                waits,
                idle: seed / 6 % 3,
                retries,
                empty_pool: true,
            };
            voip_rpc_fixture(c, &mut r, iop);
            c.g[A0][0] = sx(r.next_u32() % 16);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = sx(VOIP_RPC_RECV_BUF);
            c.g[T0][0] = VOIP_RPC_BUF_SIZE;
            let mode = schedule(c, &mut r, seed, 0x003f_6960);
            c.hooks.event = voip_rpc_iop_live_event;
            mode
        });
    }

    /// `VoipRpc_Init`: the bind succeeds (`seed % 2 == 0`; then 1-2 wait
    /// rounds for it, `seed / 2 % 2`) or finds no packet (an empty or full
    /// pool, `seed / 2 % 2`; then 1-2 call retries, `seed / 4 % 2`). Bind
    /// and call syscalls are fakes, some unwinding.
    #[test]
    fn voip_rpc_init_matches() {
        covered(ee::s14::f_003f6a40__generated, voip_rpc_init, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x3f_6a40, 8);
            c.pc = 0x003f_6a40;
            let bind_ok = seed % 2 == 0;
            let iop = VoipRpcIop {
                enabled: r.next_u32() as u8,
                waits: if bind_ok { 1 + seed / 2 % 2 } else { 0 },
                idle: 0,
                retries: if bind_ok { 0 } else { 1 + seed / 4 % 2 },
                empty_pool: seed / 2 % 2 == 1,
            };
            voip_rpc_fixture(c, &mut r, iop);
            c.next_event = 0;
            c.hooks.event = voip_rpc_iop_event;
        });
    }

    #[test]
    fn voip_rpc_disable_matches() {
        check(ee::s14::f_003f6ab0__generated, voip_rpc_disable, 16, &|c, seed| {
            let mut r = Rng(0x3f_6ab0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x003f_6ab0;
            fill(c, &mut r, GAME_GP + (VOIP_RPC_ENABLED_GP & !3), 2);
        });
    }

    /// The command wrappers: `VoipRpc_Call` runs for real, enabled or not
    /// (`seed % 3 == 0`: not), with a wait round (`seed / 3 % 2`), a retry
    /// (`seed / 6 % 2`; empty or full pool, `seed / 12 % 2`), fake syscalls
    /// (some unwinding). `args` sets the wrapper's arguments (its own
    /// steering from `seed / 24 % 2`).
    fn voip_rpc_wrapper_check(
        gen: GuestFn,
        hand: GuestFn,
        entry: u32,
        args: fn(&mut Cpu, &mut Rng, u32),
    ) {
        covered(gen, hand, 48, &|c, seed| {
            let mut r = machine(c, seed, entry as u64, 4);
            c.pc = entry;
            let iop = VoipRpcIop {
                enabled: if seed % 3 == 0 { 0 } else { 1 },
                waits: seed / 3 % 2,
                idle: 0,
                retries: seed / 6 % 2,
                empty_pool: seed / 12 % 2 == 1,
            };
            voip_rpc_fixture(c, &mut r, iop);
            c.next_event = 0;
            c.hooks.event = voip_rpc_iop_event;
            args(c, &mut r, seed);
        });
    }

    /// No arguments read (the scrambled a0-t0 are overwritten).
    fn voip_rpc_no_args(_: &mut Cpu, _: &mut Rng, _: u32) {}

    /// `VoipRpc_Start`: `hi`'s bit 15 (the shifted word's sign) set or clear
    /// (`seed / 24 % 2`), junk upper halves everywhere.
    #[test]
    fn voip_rpc_start_matches() {
        fn args(c: &mut Cpu, r: &mut Rng, seed: u32) {
            c.g[A0][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            let hi = r.next_u32() & !0x8000 | (seed / 24 % 2) << 15;
            c.g[A2][0] = (r.next_u32() as u64) << 32 | hi as u64;
        }
        voip_rpc_wrapper_check(ee::s14::f_003f6ab8__generated, voip_rpc_start, 0x003f_6ab8, args);
    }

    #[test]
    fn voip_rpc_stop_matches() {
        voip_rpc_wrapper_check(
            ee::s14::f_003f6af0__generated,
            voip_rpc_stop,
            0x003f_6af0,
            voip_rpc_no_args,
        );
    }

    #[test]
    fn voip_rpc_enable_voice_matches() {
        voip_rpc_wrapper_check(
            ee::s14::f_003f6b20__generated,
            voip_rpc_enable_voice,
            0x003f_6b20,
            voip_rpc_no_args,
        );
    }

    #[test]
    fn voip_rpc_disable_voice_matches() {
        voip_rpc_wrapper_check(
            ee::s14::f_003f6b50__generated,
            voip_rpc_disable_voice,
            0x003f_6b50,
            voip_rpc_no_args,
        );
    }

    /// `VoipRpc_IsActive`: the status block in RAM or at its real address,
    /// bit 0 set or clear (`seed / 2 % 2`).
    #[test]
    fn voip_rpc_is_active_matches() {
        check(ee::s14::f_003f6b80__generated, voip_rpc_is_active, 32, &|c, seed| {
            let mut r = Rng(0x3f_6b80 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x003f_6b80;
            let status = if seed % 2 == 0 {
                DATA + 0x300
            } else {
                VOIP_RPC_SHARED_STATUS
            };
            c.bus.write32(GAME_GP + VOIP_RPC_SHARED_STATUS_GP, status);
            c.bus.write32(status, r.next_u32() & !1 | seed / 2 % 2);
        });
    }

    /// `VoipRpc_GetStatus`: two output words, or both pointers at one word
    /// (`seed / 24 % 2`: the second store wins), pre-filled.
    #[test]
    fn voip_rpc_get_status_matches() {
        fn args(c: &mut Cpu, r: &mut Rng, seed: u32) {
            let out0 = DATA + 0x600;
            let out1 = if seed / 24 % 2 == 0 { DATA + 0x608 } else { out0 };
            fill(c, r, out0, 4);
            c.g[A0][0] = sx(out0);
            c.g[A1][0] = sx(out1);
        }
        voip_rpc_wrapper_check(
            ee::s14::f_003f6ba0__generated,
            voip_rpc_get_status,
            0x003f_6ba0,
            args,
        );
    }

    #[test]
    fn voip_rpc_set_params_matches() {
        fn args(c: &mut Cpu, r: &mut Rng, _: u32) {
            c.g[A0][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        }
        voip_rpc_wrapper_check(
            ee::s14::f_003f6c10__generated,
            voip_rpc_set_params,
            0x003f_6c10,
            args,
        );
    }

    /// `VoipRpc_GetSharedStatus`: the status block in RAM or at its real
    /// address.
    #[test]
    fn voip_rpc_get_shared_status_matches() {
        check(ee::s14::f_003f6c40__generated, voip_rpc_get_shared_status, 16, &|c, seed| {
            let mut r = Rng(0x3f_6c40 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x003f_6c40;
            let status = if seed % 2 == 0 {
                DATA + 0x300
            } else {
                VOIP_RPC_SHARED_STATUS
            };
            c.bus.write32(GAME_GP + VOIP_RPC_SHARED_STATUS_GP, status);
            fill(c, &mut r, status, 1);
        });
    }
}
