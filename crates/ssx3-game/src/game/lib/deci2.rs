//! `src/lib/deci2` SCE debug-library wrappers.
//!
//! Kept as its own unit (rather than folded into `kernel.rs`, whose address
//! range surrounds it) because the nine calls form one family sharing a
//! single `Deci2Call` helper and one override table, distinct from the
//! kernel syscall stubs on either side.

use crate::call::{guest, sp, Callee, Frame};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_c8a0, sce_deci2_open, "sceDeci2Open"),
    (0x0042_c8e8, sce_deci2_close, "sceDeci2Close"),
    (0x0042_c910, sce_deci2_req_send, "sceDeci2ReqSend"),
    (0x0042_c940, sce_deci2_poll, "sceDeci2Poll"),
    (0x0042_c968, sce_deci2_ex_recv, "sceDeci2ExRecv"),
    (0x0042_c9a0, sce_deci2_ex_send, "sceDeci2ExSend"),
    (0x0042_ca08, sce_deci2_ex_lock, "sceDeci2ExLock"),
    (0x0042_ca30, sce_deci2_ex_un_lock, "sceDeci2ExUnLock"),
    (0x0042_ca58, deci2_kputs, "Deci2Kputs"),
];

// ---------------------------------------------------------------------------
// Shared direct-call targets (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// Direct-call target: f_004241d0 (0x0042_41d0).
    pub(crate) const DECI2_CALL: Callee = Callee {
        addr: 0x0042_41d0,
        f: ee::s15::f_004241d0,
    };
}

/// One-word Deci2 call: buffer `a0` at `sp` and invoke `Deci2Call` with `op`.
fn deci2_call1(c: &mut Cpu, op: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.sw(sp(c), c.g[A0][0]);
        c.g[A1][0] = sx(sp(c));
        frame.save_ra(c, 0x10);
        c.g[A0][0] = op;
        callee::DECI2_CALL.call(c, ret)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Three-word Deci2 call: buffer `a0`, `a1` and `a2 & 0xffff` at `sp`,
/// `sp + 4` and `sp + 8` and invoke `Deci2Call` with `op`.
fn deci2_call3(c: &mut Cpu, op: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        c.g[A2][0] = c.g[A2][0] & 0xffff;
        c.sw(sp(c), c.g[A0][0]);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx(sp(c));
        c.sw(sp(c).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = op;
        c.sw(sp(c).wrapping_add(0x8), c.g[A2][0]);
        callee::DECI2_CALL.call(c, ret)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// sceDeci2Open (0x42c8a0): open the Deci2 channel `a0 & 0xffff`, passing mode,
/// flags and options on the stack; invokes `Deci2Call` with op 1 and returns
/// its result.
pub fn sce_deci2_open(c: &mut Cpu) {
    const OPT_HI: u64 = 0x2000_0000;
    guest(c, |c| {
        c.cycles += 17;
        c.g[V0][0] = 0x530000;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = OPT_HI;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xcd8));
        c.g[A0][0] = c.g[A0][0] & 0xffff;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.sw(sp(c).wrapping_add(0x4), c.g[A1][0]);
        c.sw(sp(c), c.g[A0][0]);
        c.g[A1][0] = sx(sp(c));
        frame.save_ra(c, 0x10);
        c.g[A0][0] = 0x1;
        c.sw(sp(c).wrapping_add(0x8), c.g[A2][0]);
        c.sw(sp(c).wrapping_add(0xc), c.g[V0][0]);
        callee::DECI2_CALL.call(c, 0x0042_c8d8)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `sceDeci2Close` (0x42c8e8): close the Deci2 channel in `a0`.
pub fn sce_deci2_close(c: &mut Cpu) {
    deci2_call1(c, 0x2, 0x0042_c900);
}

/// `sceDeci2ReqSend` (0x42c910): request a Deci2 send, `a1` narrowed to a
/// signed byte first.
pub fn sce_deci2_req_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        c.sw(sp(c), c.g[A0][0]);
        c.sw(sp(c).wrapping_add(0x4), c.g[A1][0]);
        c.g[A0][0] = 0x3;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx(sp(c));
        callee::DECI2_CALL.call(c, 0x0042_c934)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `sceDeci2Poll` (0x42c940): poll the Deci2 channel; invokes `Deci2Call`
/// with op 4.
pub fn sce_deci2_poll(c: &mut Cpu) {
    deci2_call1(c, 0x4, 0x0042_c958);
}

/// `sceDeci2ExRecv` (0x42c968): receive a Deci2 exception-channel packet.
pub fn sce_deci2_ex_recv(c: &mut Cpu) {
    deci2_call3(c, 0xfffffffffffffffb, 0x0042_c990);
}

/// `sceDeci2ExSend` (0x42c9a0): send on the Deci2 exception channel;
/// invokes `Deci2Call` with op -6.
pub fn sce_deci2_ex_send(c: &mut Cpu) {
    deci2_call3(c, 0xfffffffffffffffa, 0x0042_c9c8);
}

/// `sceDeci2ExLock` (0x42ca08): lock the Deci2 exception channel.
pub fn sce_deci2_ex_lock(c: &mut Cpu) {
    deci2_call1(c, 0xfffffffffffffff8, 0x0042_ca20);
}

/// `sceDeci2ExUnLock` (0x42ca30): unlock the Deci2 exception channel.
pub fn sce_deci2_ex_un_lock(c: &mut Cpu) {
    deci2_call1(c, 0xfffffffffffffff7, 0x0042_ca48);
}

/// `Deci2Kputs` (0x42ca58): write a string to the Deci2 channel;
/// invokes `Deci2Call` with op 0x10.
pub fn deci2_kputs(c: &mut Cpu) {
    deci2_call1(c, 0x10, 0x0042_ca70);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    pub(crate) const SEEDS: u32 = 200;

    #[test]
    fn sce_deci2_open_matches() {
        check(
            ee::s15::f_0042c8a0__generated,
            sce_deci2_open,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121019, 0);
                c.pc = 0x0042_c8a0;
                c.g[A0][0] = (r.next_u32() & 0xffff) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn sce_deci2_close_matches() {
        check(
            ee::s15::f_0042c8e8__generated,
            sce_deci2_close,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121018, 0);
                c.pc = 0x0042_c8e8;
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
            },
        );
    }

    #[test]
    fn sce_deci2_req_send_matches() {
        check(
            ee::s15::f_0042c910__generated,
            sce_deci2_req_send,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121017, 0);
                c.pc = 0x0042_c910;
                c.g[A0][0] = (r.next_u32() & 0xffff) as u64;
                c.g[A1][0] = (r.next_u32() & 0xff) as u64;
            },
        );
    }

    #[test]
    fn sce_deci2_poll_matches() {
        check(
            ee::s15::f_0042c940__generated,
            sce_deci2_poll,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121016, 0);
                c.pc = 0x0042_c940;
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
            },
        );
    }

    #[test]
    fn sce_deci2_ex_recv_matches() {
        check(
            ee::s15::f_0042c968__generated,
            sce_deci2_ex_recv,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121015, 1);
                c.pc = 0x0042_c968;
                c.g[A0][0] = (r.next_u32() & 0xffff) as u64;
                c.g[A1][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn sce_deci2_ex_send_matches() {
        check(
            ee::s15::f_0042c9a0__generated,
            sce_deci2_ex_send,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121014, 1);
                c.pc = 0x0042_c9a0;
                c.g[A0][0] = (r.next_u32() & 0xffff) as u64;
                c.g[A1][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn sce_deci2_ex_lock_matches() {
        check(
            ee::s15::f_0042ca08__generated,
            sce_deci2_ex_lock,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121013, 0);
                c.pc = 0x0042_ca08;
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
            },
        );
    }

    #[test]
    fn sce_deci2_ex_un_lock_matches() {
        check(
            ee::s15::f_0042ca30__generated,
            sce_deci2_ex_un_lock,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121012, 0);
                c.pc = 0x0042_ca30;
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
            },
        );
    }

    #[test]
    fn deci2_kputs_matches() {
        check(
            ee::s15::f_0042ca58__generated,
            deci2_kputs,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 121011, 0);
                c.pc = 0x0042_ca58;
                c.g[A0][0] = (DATA + 4 + (r.next_u32() & 0xf_ff0)) as u64;
            },
        );
    }
}
