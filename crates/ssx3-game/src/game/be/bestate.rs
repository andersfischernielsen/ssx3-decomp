//! `src/be/bestate.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::guest;
use crate::call::sp;
use ssx3_ee as ee;
pub(crate) use crate::game::be::beintreward::EA_MEM_ALLOC_NAMED;
pub(crate) use crate::game::be::bedbreward::PRE_RACE_GP;
pub(crate) use crate::game::be::bedbreward::RACE_GP;
pub(crate) use crate::game::be::bedbreward::POST_RACE_GP;
pub(crate) use crate::game::be::beintreward::REWARD_GP;
pub(crate) use crate::game::be::beprepoststate::Arg;
pub(crate) use crate::game::be::beprepoststate::get_instance;
pub(crate) use crate::game::be::beintreward::alloc_named;
pub(crate) use crate::game::sound::asyncsys::lw;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_3088, be_race_get_instance, "BERace_GetInstance"),
    (0x0015_30e0, be_race_get_interface, "BERace_GetInterface"),
];


/// `BERace_GetInstance(int id)` (0x153088): the `BERace` singleton, allocated
/// (with `id` in its first word and a name literal at +4) through
/// `EA_MemAllocNamed(8, "BERace", 0, 0)` on first use.
pub fn be_race_get_instance(c: &mut Cpu) {
    get_instance(
        c,
        RACE_GP,
        0x0015_30b8,
        8,
        0x004a_1230,
        &[(0, Arg::A0), (4, Arg::Const(0x0045_abe8))],
    );
}


/// The BE interface getters `BERace_GetInterface` dispatches to, in table
/// order: (the case block, its return address, the getter).
pub(crate) const BE_INTERFACE_GETTERS: [(u32, u32, Callee); 14] = [
    (0x0015_310c, 0x0015_3114, Callee { addr: 0x0014_48d8, f: ee::s01::f_001448d8 }),
    (0x0015_311c, 0x0015_3124, Callee { addr: 0x0014_5970, f: ee::s01::f_00145970 }),
    (0x0015_312c, 0x0015_3134, Callee { addr: 0x0014_edd8, f: ee::s01::f_0014edd8 }),
    (0x0015_313c, 0x0015_3144, Callee { addr: 0x0014_7f78, f: ee::s01::f_00147f78 }),
    (0x0015_314c, 0x0015_3154, Callee { addr: 0x0014_f250, f: ee::s01::f_0014f250 }),
    (0x0015_31dc, 0x0015_31e4, Callee { addr: 0x0015_2638, f: ee::s01::f_00152638 }),
    (0x0015_315c, 0x0015_3164, Callee { addr: 0x0014_f900, f: ee::s01::f_0014f900 }),
    (0x0015_316c, 0x0015_3174, Callee { addr: 0x0014_e0e0, f: ee::s01::f_0014e0e0 }),
    (0x0015_317c, 0x0015_3184, Callee { addr: 0x0015_4948, f: ee::s01::f_00154948 }),
    (0x0015_318c, 0x0015_3194, Callee { addr: 0x0014_abe0, f: ee::s01::f_0014abe0 }),
    (0x0015_319c, 0x0015_31a4, Callee { addr: 0x0015_32a0, f: ee::s01::f_001532a0 }),
    (0x0015_31ac, 0x0015_31b4, Callee { addr: 0x0015_08d8, f: ee::s01::f_001508d8 }),
    (0x0015_31bc, 0x0015_31c4, Callee { addr: 0x0015_5a50, f: ee::s01::f_00155a50 }),
    (0x0015_31cc, 0x0015_31d4, Callee { addr: 0x0015_69c0, f: ee::s01::f_001569c0 }),
];


/// `BERace_GetInterface`'s kind table (`.rodata` at 0x45a810; entry `kind`).
pub(crate) const BE_INTERFACE_TABLE: u32 = 0x0045_a810;


/// `ra = *(sp)` (the frame's saved return address).
#[inline(always)]
pub(crate) fn reload_ra(c: &mut Cpu) {
    let a = sp(c);
    c.g[RA][0] = c.ld(a);
}


// ---------------------------------------------------------------------------
// BERace_GetInterface.

/// The stages `BERace_GetInterface` can be entered (or resumed) at.
#[derive(Clone, Copy)]
pub(crate) enum Iface {
    /// 0x1530e0: the frame, the range check.
    Entry,
    /// 0x1530f0: read the kind's table entry into `a0`.
    Dispatch,
    /// One getter's call block (index into [`BE_INTERFACE_GETTERS`]).
    Getter(usize),
    /// 0x1531ec: unknown kind, return null.
    Empty,
    /// The table's first entry points back at the entry block.
    Restart,
    /// 0x1531f4: `jr ra` with `sp += 0x10`.
    Return,
}


/// `void* BERace_GetInterface(int kind)` (0x1530e0): the BE subsystem's
/// interface object for `kind` (0 race, 1 player, 2 char DB, 3 char, 4 option,
/// 5 save, 6 bag, 7 network, 8 score, 9 bolt, 10 mission, 11 econ, 12
/// aggression, 13 reward), or null for anything else. The table of case blocks
/// is at 0x45a810; each block calls that kind's singleton getter.
pub fn be_race_get_interface(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        let mut stage = match c.pc as u32 {
            0x0015_30e0 => Iface::Entry,
            0x0015_30f0 => Iface::Dispatch,
            0x0015_31ec => Iface::Empty,
            0x0015_31f4 => Iface::Return,
            pc => match BE_INTERFACE_GETTERS.iter().position(|g| g.0 == pc) {
                Some(k) => Iface::Getter(k),
                None => {
                    c.jump_out(pc, 0x0015_30e0, 0x0015_31fc);
                    return Ok(());
                }
            },
        };
        loop {
            match stage {
                Iface::Entry => {
                    c.cycles += 4;
                    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
                    c.g[V0][0] = (c.g[A1][0] < 0xeu64) as u64;
                    let in_range = c.g[V0][0] != 0;
                    frame.save_ra(c, 0);
                    stage = if in_range { Iface::Dispatch } else { Iface::Empty };
                }
                Iface::Dispatch => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x46_0000;
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                    // (0x460000 - 0x57f0 is the kind table at 0x45a810.)
                    c.g[V0][0] = sx(BE_INTERFACE_TABLE);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    // `v1` already holds table + kind * 4.
                    let slot = c.g[V1][0] as u32;
                    c.g[A0][0] = Addr(slot).lw(c);
                    let target = c.g[A0][0] as u32;
                    stage = match target {
                        0x0015_30e0 => Iface::Restart,
                        pc => match BE_INTERFACE_GETTERS.iter().position(|g| g.0 == pc) {
                            Some(k) => Iface::Getter(k),
                            None => {
                                c.jump_out(target, 0x0015_30e0, 0x0015_31fc);
                                return Ok(());
                            }
                        },
                    };
                }
                Iface::Getter(k) => {
                    c.cycles += 4;
                    let (_, ret, getter) = BE_INTERFACE_GETTERS[k];
                    getter.call(c, ret)?;
                    reload_ra(c);
                    stage = Iface::Return;
                }
                Iface::Empty => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    reload_ra(c);
                    stage = Iface::Return;
                }
                Iface::Restart => {
                    // Unreachable with the shipped table: a target equal to the
                    // entry loops the whole function (with its event check).
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_30e0);
                    }
                    stage = Iface::Entry;
                }
                Iface::Return => {
                    c.cycles += 2;
                    frame.close(c);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    pub(crate) use crate::game::be::beintnetwork::tests::MEM_MUTEX;
    pub(crate) use crate::game::be::beintnetwork::tests::alloc_setup;
    pub(crate) use crate::game::be::beprepoststate::be_pre_race_get_instance;
    pub(crate) use crate::game::be::beprepoststate::be_post_race_get_instance;
    pub(crate) use crate::game::be::beintreward::be_reward_int_get_instance;


    pub(crate) fn get_instance_test(generated: GuestFn, hand: GuestFn, gp_off: u32, salt: u64) {
        check(generated, hand, 60, &|c, seed| {
            let mut r = machine(c, seed, salt, 6);
            alloc_setup(c, &mut r);
            c.bus.write32(GAME_GP - gp_off, if seed % 2 == 0 { 0 } else { 0x0080_0000u32 + seed * 8 });
            c.g[A0][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        });
    }


    #[test]
    fn be_get_instances_match() {
        get_instance_test(ssx3_ee::s01::f_00153088, be_race_get_instance, RACE_GP, 11);
        get_instance_test(ssx3_ee::s01::f_00156338, be_post_race_get_instance, POST_RACE_GP, 12);
        get_instance_test(ssx3_ee::s01::f_00156570, be_pre_race_get_instance, PRE_RACE_GP, 13);
        get_instance_test(ssx3_ee::s01::f_001569c0, be_reward_int_get_instance, REWARD_GP, 14);
    }


    #[test]
    fn be_race_get_interface_matches() {
        check(ssx3_ee::s01::f_001530e0__generated, be_race_get_interface, 300, &|c, seed| {
            machine(c, seed, 0x1530_e0, 1);
            c.pc = 0x0015_30e0;
            // The kind table and the 14 singletons: with the instance globals
            // set, each getter returns its dummy at once.
            let mut table = [
                0x0015_310c, 0x0015_311c, 0x0015_312c, 0x0015_313c, 0x0015_314c, 0x0015_31dc,
                0x0015_315c, 0x0015_316c, 0x0015_317c, 0x0015_318c, 0x0015_319c, 0x0015_31ac,
                0x0015_31bc, 0x0015_31cc,
            ];
            let kind = if seed % 15 == 0 {
                14 + (seed / 15) % 3 // out of range: null, no table read
            } else {
                seed % 15
            };
            if seed % 32 == 0 {
                // A table slot that is not a case block: the original jumps out
                // of itself (the fake hook takes the call).
                table[kind as usize % 14] = 0xdead_beef;
            }
            put_words(c, BE_INTERFACE_TABLE, &table);
            let globals = [
                0x1f3c, 0x1f30, 0x1ee8, 0x1f2c, 0x1ee4, 0x1ed4, 0x1ee0, 0x1eec, 0x1ea8, 0x1f28,
                0x1eb8, 0x1edc, 0x1ea4, 0x1e8c,
            ];
            for (i, off) in globals.iter().enumerate() {
                c.bus.write32(GAME_GP.wrapping_sub(*off), DATA + 0x1000 + 0x100 * i as u32);
            }
            c.g[A1][0] = sx(kind);
        });
    }
}
