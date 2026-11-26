//! Staging module oc885 (wave wave252): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc885`.

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
    (
        0x001b_6b28,
        cfe_state_auth_dnas_error_dtor,
        "cFEStateAuthDNASError_dtor",
    ),
    (
        0x001b_7088,
        cfe_state_net_account_mem_init,
        "cFEStateNetAccountMem_Init",
    ),
];

/// `NetFEState::~NetFEState` (0x1a85d0), the base destructor the DNAS error
/// state tail-calls.
const AUTH_DNAS_DTOR_BASE: Callee = Callee {
    addr: 0x001a_85d0,
    f: ee::s03::f_001a85d0,
};

/// `FEScreenBlank_Init` (0x1abfa8), the overridden screen initialiser.
const NET_ACCOUNT_MEM_INIT_BLANK: Callee = Callee {
    addr: 0x001a_bfa8,
    f: ee::s03::f_001abfa8,
};

/// `cFEStateAuthDNASError::~cFEStateAuthDNASError(this)` (0x1b6b28): stores
/// the state's method table word (0x4688e0) at `this + 8`, then tail-calls the
/// base `NetFEState` destructor with `this` still in `a0`.
pub fn cfe_state_auth_dnas_error_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x7720));
        c.g[V1][0] = c.g[A0][0];
        Addr(c.g[V1][0] as u32).off(0x8).sw(c, c.g[V0][0]);
        AUTH_DNAS_DTOR_BASE.call(c, 0x001b_6b44)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetAccountMem_Init(void* self)` (0x1b7088): a one-call
/// wrapper that runs `FEScreenBlank_Init` with `a0` unchanged.
pub fn cfe_state_net_account_mem_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        NET_ACCOUNT_MEM_INIT_BLANK.call(c, 0x001b_7098)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    use crate::game::fe::feasyncfile::tests::{be_library_4, pool, world};

    #[test]
    fn cfe_state_auth_dnas_error_dtor_matches() {
        check(
            ee::s03::f_001b6b28__generated,
            cfe_state_auth_dnas_error_dtor,
            6,
            &|c, seed| {
                machine(c, seed, 0x001b_6b28, 2);
                be_library_4(c);
                world(c);
                pool(c);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x8, DATA + 0x2000 + seed * 0x10);
                c.pc = 0x001b_6b28;
            },
        );
    }

    #[test]
    fn cfe_state_net_account_mem_init_matches() {
        check(
            ee::s03::f_001b7088__generated,
            cfe_state_net_account_mem_init,
            6,
            &|c, seed| {
                machine(c, seed, 0x001b_7088, 3);
                be_library_4(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, DATA + 0x1000);
                c.pc = 0x001b_7088;
            },
        );
    }
}
