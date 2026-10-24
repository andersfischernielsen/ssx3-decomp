//! `src/be/beprepoststate.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::bedbreward::POST_RACE_GP;
pub(crate) use crate::game::be::bedbreward::PRE_RACE_GP;
pub(crate) use crate::game::be::beintreward::alloc_named;
pub(crate) use crate::game::be::beintreward::EA_MEM_ALLOC_NAMED;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_6338,
        be_post_race_get_instance,
        "BEPostRace_GetInstance",
    ),
    (
        0x0015_6570,
        be_pre_race_get_instance,
        "BEPreRace_GetInstance",
    ),
];

/// `BEPostRace_GetInstance(int id)` (0x156338): as above with `gp - 0x1e9c`
/// and the "BEPostRace" name.
pub fn be_post_race_get_instance(c: &mut Cpu) {
    get_instance(
        c,
        POST_RACE_GP,
        0x0015_6368,
        8,
        0x0045_a8a8,
        &[(0, Arg::A0), (4, Arg::Const(0x0045_abb8))],
    );
}

/// `BEPreRace_GetInstance(int id)` (0x156570): as above with `gp - 0x1e98`
/// and the "BEPreRace" name.
pub fn be_pre_race_get_instance(c: &mut Cpu) {
    get_instance(
        c,
        PRE_RACE_GP,
        0x0015_65a0,
        8,
        0x0045_a8a8,
        &[(0, Arg::A0), (4, Arg::Const(0x0045_ac18))],
    );
}

/// What to store in an allocated singleton's word.
#[derive(Clone, Copy)]
pub(crate) enum Arg {
    /// The function's first argument (`a0`).
    A0,
    /// A constant word.
    Const(u32),
}

/// The shared `*_GetInstance` body: return the cached singleton, allocating
/// it (and storing the given words) when the global is still zero.
pub(crate) fn get_instance(
    c: &mut Cpu,
    gp_off: u32,
    ret: u32,
    size: u64,
    name: u32,
    init: &[(u32, Arg)],
) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        let global = (c.g[GP][0] as u32).wrapping_sub(gp_off);
        let inst = Addr(global).lw(c);
        c.g[V0][0] = inst;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let present = inst != 0;
        c.g[S0][0] = c.g[A0][0];
        if !present {
            c.cycles += 11;
            let block = alloc_named(c, size, name, ret)?;
            for &(off, value) in init {
                let v = match value {
                    Arg::A0 => c.g[S0][0],
                    Arg::Const(x) => x as u64,
                };
                c.g[V1][0] = v;
                Addr(block as u32 + off).sw(c, v);
            }
            Addr(global).sw(c, block);
        }
        c.cycles += 5;
        c.g[V0][0] = Addr(global).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
