//! Staging module oc350 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc350`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0012_8af0, ai_update, "AI_Update")];

// ------------------------------------------------------------------ AI_Update

/// `AI` fields read by `AI_Update`.
pub(crate) const AI_UPDATE_FRAME: u32 = 0x8;
pub(crate) const AI_UPDATE_HAVE_COMPUTERS: u32 = 0x14;
pub(crate) const AI_UPDATE_PLAYERS: u32 = 0x40;
pub(crate) const AI_UPDATE_COMPUTERS: u32 = 0x48;
pub(crate) const AI_UPDATE_OTHERS: u32 = 0x5c;
pub(crate) const AI_UPDATE_MISSIONS: u32 = 0x70;
pub(crate) const AI_UPDATE_NUM_PLAYERS: u32 = 0x7c;
pub(crate) const AI_UPDATE_NUM_COMPUTERS: u32 = 0x80;
pub(crate) const AI_UPDATE_NUM_OTHERS: u32 = 0x84;
pub(crate) const AI_UPDATE_NUM_MISSIONS: u32 = 0x88;
pub(crate) const AI_UPDATE_HAVE_OTHERS: u32 = 0x8c;
pub(crate) const AI_UPDATE_FOCUSMAN: u32 = 0xa4;
pub(crate) const AI_UPDATE_DANGERMAN: u32 = 0xa8;
/// A rider record's rider (`Rider_AF0`: `rec ? rec->rider : 0`).
pub(crate) const AI_UPDATE_REC_RIDER: u32 = 0x18;
/// A rider's model (the `m[]` array of sub-systems).
pub(crate) const AI_UPDATE_RIDER_MODEL: u32 = 0x77c;
/// `gp-0x848`: the world; `gp-0x6a0`: the debug flags (bit 0: pose only).
pub(crate) const AI_UPDATE_GP_WORLD: u32 = 0x848;
pub(crate) const AI_UPDATE_GP_FLAGS: u32 = 0x6a0;

const AIU_RACE_TIME_LIMIT: Callee = Callee {
    addr: 0x0012_bb20,
    f: ee::s00::AI_UpdateRaceTimeLimit,
};
const AIU_RACE_STATE: Callee = Callee {
    addr: 0x0011_3c20,
    f: ee::s00::AI_UpdateRaceState,
};
const AIU_RIDER_RELATIONS: Callee = Callee {
    addr: 0x0010_f560,
    f: ee::s00::AI_UpdateRiderRelations,
};
const AIU_FOCUS_BOX: Callee = Callee {
    addr: 0x0012_0e30,
    f: ee::s00::Rider_UpdateFocusBox,
};
const AIU_REFRESH_POSE: Callee = Callee {
    addr: 0x0012_0ed8,
    f: ee::s00::Rider_RefreshPose,
};
const AIU_FOCUSMAN_UPDATE: Callee = Callee {
    addr: 0x0010_1b60,
    f: ee::s00::FocusMan_Update,
};
const AIU_DANGERMAN_UPDATE: Callee = Callee {
    addr: 0x0010_13a8,
    f: ee::s00::DangerMan_Update,
};

/// One per-rider pass of `AI_Update`: call `target` for every collected
/// rider (`list` 0: `r[]` at `sp+0`) or for a sub-system of its model
/// (`list` 0x20: `m[]`, plus `field`).
#[derive(Clone, Copy)]
struct AiUpdatePass {
    /// `s0 = n` sits in the delay slot of the `n <= 0` test.
    early_count: bool,
    /// Cycles of the block that sets up the loop.
    prelude: u64,
    list: u32,
    field: Option<u32>,
    target: Target,
    ret: u32,
    head: u32,
}

const fn aiu_rider_pass(addr: u32, f: GuestFn, ret: u32, head: u32) -> AiUpdatePass {
    AiUpdatePass {
        early_count: false,
        prelude: 3,
        list: 0,
        field: None,
        target: Target::Direct(Callee { addr, f }),
        ret,
        head,
    }
}

const fn aiu_model_pass(
    early_count: bool,
    prelude: u64,
    field: u32,
    addr: u32,
    ret: u32,
    head: u32,
) -> AiUpdatePass {
    AiUpdatePass {
        early_count,
        prelude,
        list: 0x20,
        field: Some(field),
        target: Target::Table(addr),
        ret,
        head,
    }
}

/// The full update's passes, in order (after `m[]` is filled).
const AIU_PASSES: [AiUpdatePass; 24] = [
    aiu_rider_pass(
        0x0012_0f20,
        ee::s00::Rider_BeginFrame,
        0x0012_8d44,
        0x0012_8d38,
    ),
    aiu_rider_pass(
        0x0012_1068,
        ee::s00::Rider_ProcessControlInput,
        0x0012_8d6c,
        0x0012_8d60,
    ),
    aiu_rider_pass(
        0x0012_10b0,
        ee::s00::Rider_UpdateImpacts,
        0x0012_8d94,
        0x0012_8d88,
    ),
    aiu_rider_pass(
        0x0012_11f8,
        ee::s00::Rider_UpdateSlewedParams,
        0x0012_8dbc,
        0x0012_8db0,
    ),
    aiu_rider_pass(
        0x0012_16e0,
        ee::s00::Rider_UpdateMainState,
        0x0012_8de4,
        0x0012_8dd8,
    ),
    aiu_rider_pass(
        0x0012_1700,
        ee::s00::Rider_UpdateAnimation,
        0x0012_8e0c,
        0x0012_8e00,
    ),
    aiu_rider_pass(
        0x0012_1728,
        ee::s00::Rider_UpdatePose,
        0x0012_8e34,
        0x0012_8e28,
    ),
    aiu_rider_pass(
        0x0012_1750,
        ee::s00::Rider_PostUpdateMainState,
        0x0012_8e5c,
        0x0012_8e50,
    ),
    aiu_rider_pass(
        0x0012_17f8,
        ee::s00::Rider_UpdateModel,
        0x0012_8e84,
        0x0012_8e78,
    ),
    aiu_rider_pass(
        0x0012_1818,
        ee::s00::Rider_UpdateProgress,
        0x0012_8eac,
        0x0012_8ea0,
    ),
    aiu_rider_pass(
        0x0012_18d0,
        ee::s00::Rider_UpdateSurfaceFX,
        0x0012_8ed4,
        0x0012_8ec8,
    ),
    aiu_rider_pass(
        0x0012_1950,
        ee::s00::Rider_UpdateHeadingAndAudio,
        0x0012_8efc,
        0x0012_8ef0,
    ),
    aiu_model_pass(false, 3, 0x3b0, 0x002d_d0b8, 0x0012_8f28, 0x0012_8f18),
    aiu_model_pass(false, 4, 0x470, 0x002d_abc8, 0x0012_8f58, 0x0012_8f48),
    aiu_model_pass(false, 4, 0x520, 0x002e_8938, 0x0012_8f88, 0x0012_8f78),
    aiu_model_pass(false, 4, 0x610, 0x002e_66b8, 0x0012_8fb8, 0x0012_8fa8),
    aiu_model_pass(false, 4, 0x9c0, 0x002e_add0, 0x0012_8fe8, 0x0012_8fd8),
    aiu_model_pass(false, 4, 0xad0, 0x002e_f6d0, 0x0012_9018, 0x0012_9008),
    aiu_model_pass(false, 4, 0xaf0, 0x002d_4c08, 0x0012_9048, 0x0012_9038),
    aiu_model_pass(false, 4, 0xb00, 0x002e_39d8, 0x0012_9078, 0x0012_9068),
    aiu_model_pass(false, 4, 0xb40, 0x002d_f920, 0x0012_90a8, 0x0012_9098),
    aiu_model_pass(true, 2, 0xc70, 0x002f_1150, 0x0012_90d0, 0x0012_90c0),
    aiu_model_pass(true, 2, 0xd20, 0x002f_6518, 0x0012_90f8, 0x0012_90e8),
    AiUpdatePass {
        early_count: true,
        prelude: 2,
        list: 0,
        field: None,
        target: Target::Direct(Callee {
            addr: 0x0012_0e88,
            f: ee::s00::Rider_UpdateAttachedFX,
        }),
        ret: 0x0012_911c,
        head: 0x0012_9110,
    },
];

/// The "pose only" pass (debug flag bit 0).
const AIU_REFRESH_PASS: AiUpdatePass = AiUpdatePass {
    early_count: true,
    prelude: 2,
    list: 0,
    field: None,
    target: Target::Direct(AIU_REFRESH_POSE),
    ret: 0x0012_8ce4,
    head: 0x0012_8cd8,
};

/// `gp - off`.
fn aiu_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// One pass over the `s2` collected riders (`s0` counts down, `s1` walks the
/// list).
fn ai_update_pass(c: &mut Cpu, p: AiUpdatePass) -> Flow {
    c.cycles += 2;
    let none = (c.g[S2][0] as i64) <= 0;
    if p.early_count {
        c.g[S0][0] = c.g[S2][0];
    }
    if none {
        return Ok(());
    }
    c.cycles += p.prelude;
    c.g[S1][0] = if p.list == 0 {
        c.g[SP][0]
    } else {
        sx(sp(c).wrapping_add(p.list))
    };
    c.g[S0][0] = c.g[S2][0];
    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
    loop {
        c.cycles += if p.field.is_some() { 6 } else { 5 };
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        if let Some(off) = p.field {
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(off));
        }
        match p.target {
            Target::Direct(f) => f.call(c, p.ret)?,
            Target::Table(addr) => call_ptr(c, addr, p.ret)?,
        }
        if c.g[S0][0] == 0 {
            return Ok(());
        }
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        if event_due(c) {
            deliver_event_here(c, p.head);
        }
    }
}

/// Append the riders of the `a0` records at `ai+list` to `r[]` (`s2` is the
/// count so far): `r[s2++] = rec ? rec->rider : 0`.
fn ai_update_collect(c: &mut Cpu, prelude: u64, list: u32, head: u32) {
    let none = (c.g[A0][0] as i64) <= 0;
    c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
    if none {
        return;
    }
    c.cycles += prelude;
    c.g[V1][0] = sx(sp(c).wrapping_sub(4));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A2][0] = c.g[A0][0];
    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(list));
    loop {
        c.cycles += 5;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
        c.g[A0][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(AI_UPDATE_REC_RIDER).lw(c);
        }
        c.cycles += 4;
        Addr(c.g[V1][0] as u32).sw(c, c.g[A0][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        if c.g[A2][0] == 0 {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void AI_Update(AI *ai)` (0x128af0): the per-frame AI update. Updates
/// the race time limit, race state and rider relations; collects the riders
/// to update (in a mission with a live mission object, the first player and
/// the mission rider, with the focus box updated; otherwise all players,
/// computers and others); then either only refreshes their poses (debug
/// flag `gp-0x6a0` bit 0) or runs every rider stage and every model
/// sub-system for all of them, stage by stage; then updates the focus and
/// danger managers and counts the frame.
pub fn ai_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S3, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save_ra(c, 0x40);
        c.g[S2][0] = 0;
        AIU_RACE_TIME_LIMIT.call(c, 0x0012_8b14)?;
        c.g[A0][0] = c.g[S3][0];
        AIU_RACE_STATE.call(c, 0x0012_8b1c)?;
        c.g[A0][0] = c.g[S3][0];
        AIU_RIDER_RELATIONS.call(c, 0x0012_8b24)?;

        let ai = c.g[S3][0] as u32;
        c.g[V0][0] = Addr(ai).off(AI_UPDATE_NUM_MISSIONS).lw(c);
        let mut mission = false;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(ai).off(AI_UPDATE_NUM_PLAYERS).lw(c);
            let no_players = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = Addr(aiu_gp(c, AI_UPDATE_GP_WORLD)).lw(c);
            if no_players {
                c.cycles += 1;
                c.g[A0][0] = Addr(ai).off(AI_UPDATE_NUM_PLAYERS).lw(c);
            } else {
                mission = true;
            }
        } else {
            c.g[A0][0] = Addr(ai).off(AI_UPDATE_NUM_PLAYERS).lw(c);
        }

        if mission {
            // Mission: is the world's mission object live (`+0x70 == 0`)?
            c.cycles += 4;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x34).lw(c);
            let solo = if c.g[V0][0] == 0 {
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                false
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x70).lw(c);
                let busy = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                !busy
            };
            if solo {
                // r[0] = player 0; focus box of the mission rider.
                c.cycles += 4;
                c.g[V0][0] = Addr(ai).off(AI_UPDATE_PLAYERS).lw(c);
                c.g[S2][0] = 1;
                c.g[V1][0] = 0;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(AI_UPDATE_REC_RIDER).lw(c);
                }
                c.cycles += 4;
                c.g[V0][0] = Addr(ai).off(AI_UPDATE_MISSIONS).lw(c);
                c.g[A0][0] = 0;
                Addr(sp(c)).sw(c, c.g[V1][0]);
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(AI_UPDATE_REC_RIDER).lw(c);
                }
            } else {
                // r[0] = player 0, r[1] = mission rider; focus box of player 0.
                c.cycles += 4;
                c.g[V1][0] = Addr(ai).off(AI_UPDATE_PLAYERS).lw(c);
                c.g[V0][0] = sx(sp(c).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = 0;
                if c.g[V1][0] != 0 {
                    c.cycles += 1;
                    c.g[A1][0] = Addr(c.g[V1][0] as u32).off(AI_UPDATE_REC_RIDER).lw(c);
                }
                c.cycles += 5;
                c.g[A0][0] = Addr(ai).off(AI_UPDATE_MISSIONS).lw(c);
                c.g[S2][0] = 2;
                Addr(c.g[V0][0] as u32).sw(c, c.g[A1][0]);
                c.g[V0][0] = 0;
                if c.g[A0][0] != 0 {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(AI_UPDATE_REC_RIDER).lw(c);
                }
                c.cycles += 3;
                Addr(sp(c).wrapping_add(4)).sw(c, c.g[V0][0]);
                c.g[A0][0] = 0;
                if c.g[V1][0] != 0 {
                    c.cycles += 1;
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(AI_UPDATE_REC_RIDER).lw(c);
                }
            }
            c.cycles += 4;
            AIU_FOCUS_BOX.call(c, 0x0012_8bc8)?;
            c.g[V0][0] = Addr(aiu_gp(c, AI_UPDATE_GP_FLAGS)).lw(c);
        } else {
            let ai = c.g[S3][0] as u32;
            c.cycles += 2;
            ai_update_collect(c, 5, AI_UPDATE_PLAYERS, 0x0012_8bf0);
            c.cycles += 3;
            c.g[V0][0] = Addr(ai).off(AI_UPDATE_HAVE_COMPUTERS).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = Addr(ai).off(AI_UPDATE_NUM_COMPUTERS).lw(c);
                ai_update_collect(c, 4, AI_UPDATE_COMPUTERS, 0x0012_8c40);
                c.cycles += 1;
            }
            c.g[V0][0] = Addr(ai).off(AI_UPDATE_HAVE_OTHERS).lw(c);
            c.cycles += 2;
            let no_others = c.g[V0][0] == 0;
            c.g[V0][0] = Addr(aiu_gp(c, AI_UPDATE_GP_FLAGS)).lw(c);
            if !no_others {
                c.cycles += 3;
                c.g[A0][0] = Addr(ai).off(AI_UPDATE_NUM_OTHERS).lw(c);
                ai_update_collect(c, 4, AI_UPDATE_OTHERS, 0x0012_8c90);
                c.cycles += 1;
                c.g[V0][0] = Addr(aiu_gp(c, AI_UPDATE_GP_FLAGS)).lw(c);
            }
        }

        c.cycles += 3;
        c.g[V0][0] &= 1;
        if c.g[V0][0] != 0 {
            ai_update_pass(c, AIU_REFRESH_PASS)?;
            if (c.g[S2][0] as i64) > 0 {
                c.cycles += 2;
            }
        } else {
            // m[i] = r[i]->model.
            c.cycles += 2;
            if (c.g[S2][0] as i64) > 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[SP][0];
                c.g[S0][0] = c.g[S2][0];
                loop {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(AI_UPDATE_RIDER_MODEL).lw(c);
                    Addr(c.g[A0][0] as u32).off(0x20).sw(c, c.g[V0][0]);
                    let more = c.g[S0][0] != 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0012_8d08);
                    }
                }
            }
            for p in AIU_PASSES {
                ai_update_pass(c, p)?;
            }
        }

        c.cycles += 14;
        c.g[A0][0] = Addr(c.g[S3][0] as u32).off(AI_UPDATE_FOCUSMAN).lw(c);
        AIU_FOCUSMAN_UPDATE.call(c, 0x0012_912c)?;
        c.g[A0][0] = Addr(c.g[S3][0] as u32).off(AI_UPDATE_DANGERMAN).lw(c);
        AIU_DANGERMAN_UPDATE.call(c, 0x0012_9134)?;
        c.g[V0][0] = Addr(c.g[S3][0] as u32).off(AI_UPDATE_FRAME).lw(c);
        frame.restore(c, S0, 0x80);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        frame.restore(c, S1, 0x70);
        Addr(c.g[S3][0] as u32)
            .off(AI_UPDATE_FRAME)
            .sw(c, c.g[V0][0]);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake, fall_back, hook, machine};
    use crate::difftest::{fill, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // ------------------------------------------------------------ AI_Update

    /// Gate of the `AI_Update` test fakes (in each test machine's own RAM):
    /// the rest of the suite reaches the bodies the fakes chain to.
    const AI_UPDATE_CTRL: u32 = DATA + 0x0f_0350;
    const AI_UPDATE_MAGIC: u32 = 0x4149_5550;

    /// A gated fake for one callee of `AI_Update`: logs the call (registers
    /// at the call) and returns, or unwinds at the chosen call index.
    macro_rules! ai_update_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(AI_UPDATE_CTRL) != AI_UPDATE_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fake(c, $addr, ra);
            }
        };
    }

    ai_update_fake!(
        aiu_fake_bb20,
        AIU_PREV_BB20,
        0x0012_bb20,
        ee::s00::f_0012bb20__generated
    );
    ai_update_fake!(
        aiu_fake_3c20,
        AIU_PREV_3C20,
        0x0011_3c20,
        ee::s00::f_00113c20__generated
    );
    ai_update_fake!(
        aiu_fake_f560,
        AIU_PREV_F560,
        0x0010_f560,
        ee::s00::f_0010f560__generated
    );
    ai_update_fake!(
        aiu_fake_0e30,
        AIU_PREV_0E30,
        0x0012_0e30,
        ee::s00::f_00120e30__generated
    );
    ai_update_fake!(
        aiu_fake_0ed8,
        AIU_PREV_0ED8,
        0x0012_0ed8,
        ee::s00::f_00120ed8__generated
    );
    ai_update_fake!(
        aiu_fake_0f20,
        AIU_PREV_0F20,
        0x0012_0f20,
        ee::s00::f_00120f20__generated
    );
    ai_update_fake!(
        aiu_fake_1068,
        AIU_PREV_1068,
        0x0012_1068,
        ee::s00::f_00121068__generated
    );
    ai_update_fake!(
        aiu_fake_10b0,
        AIU_PREV_10B0,
        0x0012_10b0,
        ee::s00::f_001210b0__generated
    );
    ai_update_fake!(
        aiu_fake_11f8,
        AIU_PREV_11F8,
        0x0012_11f8,
        ee::s00::f_001211f8__generated
    );
    ai_update_fake!(
        aiu_fake_16e0,
        AIU_PREV_16E0,
        0x0012_16e0,
        ee::s00::f_001216e0__generated
    );
    ai_update_fake!(
        aiu_fake_1700,
        AIU_PREV_1700,
        0x0012_1700,
        ee::s00::f_00121700__generated
    );
    ai_update_fake!(
        aiu_fake_1728,
        AIU_PREV_1728,
        0x0012_1728,
        ee::s00::f_00121728__generated
    );
    ai_update_fake!(
        aiu_fake_1750,
        AIU_PREV_1750,
        0x0012_1750,
        ee::s00::f_00121750__generated
    );
    ai_update_fake!(
        aiu_fake_17f8,
        AIU_PREV_17F8,
        0x0012_17f8,
        ee::s00::f_001217f8__generated
    );
    ai_update_fake!(
        aiu_fake_1818,
        AIU_PREV_1818,
        0x0012_1818,
        ee::s00::f_00121818__generated
    );
    ai_update_fake!(
        aiu_fake_18d0,
        AIU_PREV_18D0,
        0x0012_18d0,
        ee::s00::f_001218d0__generated
    );
    ai_update_fake!(
        aiu_fake_1950,
        AIU_PREV_1950,
        0x0012_1950,
        ee::s00::f_00121950__generated
    );
    ai_update_fake!(
        aiu_fake_0e88,
        AIU_PREV_0E88,
        0x0012_0e88,
        ee::s00::f_00120e88__generated
    );
    ai_update_fake!(
        aiu_fake_1b60,
        AIU_PREV_1B60,
        0x0010_1b60,
        ee::s00::f_00101b60__generated
    );
    ai_update_fake!(
        aiu_fake_13a8,
        AIU_PREV_13A8,
        0x0010_13a8,
        ee::s00::f_001013a8__generated
    );
    ai_update_fake!(
        aiu_fake_d0b8,
        AIU_PREV_D0B8,
        0x002d_d0b8,
        ee::s09::f_002dd0b8__generated
    );
    ai_update_fake!(
        aiu_fake_abc8,
        AIU_PREV_ABC8,
        0x002d_abc8,
        ee::s09::f_002dabc8__generated
    );
    ai_update_fake!(
        aiu_fake_8938,
        AIU_PREV_8938,
        0x002e_8938,
        ee::s09::f_002e8938__generated
    );
    ai_update_fake!(
        aiu_fake_66b8,
        AIU_PREV_66B8,
        0x002e_66b8,
        ee::s09::f_002e66b8__generated
    );
    ai_update_fake!(
        aiu_fake_add0,
        AIU_PREV_ADD0,
        0x002e_add0,
        ee::s09::f_002eadd0__generated
    );
    ai_update_fake!(
        aiu_fake_f6d0,
        AIU_PREV_F6D0,
        0x002e_f6d0,
        ee::s09::f_002ef6d0__generated
    );
    ai_update_fake!(
        aiu_fake_4c08,
        AIU_PREV_4C08,
        0x002d_4c08,
        ee::s09::f_002d4c08__generated
    );
    ai_update_fake!(
        aiu_fake_39d8,
        AIU_PREV_39D8,
        0x002e_39d8,
        ee::s09::f_002e39d8__generated
    );
    ai_update_fake!(
        aiu_fake_f920,
        AIU_PREV_F920,
        0x002d_f920,
        ee::s09::f_002df920__generated
    );
    ai_update_fake!(
        aiu_fake_1150,
        AIU_PREV_1150,
        0x002f_1150,
        ee::s09::f_002f1150__generated
    );
    ai_update_fake!(
        aiu_fake_6518,
        AIU_PREV_6518,
        0x002f_6518,
        ee::s09::f_002f6518__generated
    );

    /// Hook every callee of `AI_Update` (once per process).
    fn ai_update_install_fakes() {
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0012BB20),
            aiu_fake_bb20,
            &AIU_PREV_BB20,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00113C20),
            aiu_fake_3c20,
            &AIU_PREV_3C20,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0010F560),
            aiu_fake_f560,
            &AIU_PREV_F560,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00120E30),
            aiu_fake_0e30,
            &AIU_PREV_0E30,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00120ED8),
            aiu_fake_0ed8,
            &AIU_PREV_0ED8,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00120F20),
            aiu_fake_0f20,
            &AIU_PREV_0F20,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00121068),
            aiu_fake_1068,
            &AIU_PREV_1068,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_001210B0),
            aiu_fake_10b0,
            &AIU_PREV_10B0,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_001211F8),
            aiu_fake_11f8,
            &AIU_PREV_11F8,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_001216E0),
            aiu_fake_16e0,
            &AIU_PREV_16E0,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00121700),
            aiu_fake_1700,
            &AIU_PREV_1700,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00121728),
            aiu_fake_1728,
            &AIU_PREV_1728,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00121750),
            aiu_fake_1750,
            &AIU_PREV_1750,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_001217F8),
            aiu_fake_17f8,
            &AIU_PREV_17F8,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00121818),
            aiu_fake_1818,
            &AIU_PREV_1818,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_001218D0),
            aiu_fake_18d0,
            &AIU_PREV_18D0,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00121950),
            aiu_fake_1950,
            &AIU_PREV_1950,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00120E88),
            aiu_fake_0e88,
            &AIU_PREV_0E88,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00101B60),
            aiu_fake_1b60,
            &AIU_PREV_1B60,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_001013A8),
            aiu_fake_13a8,
            &AIU_PREV_13A8,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002DD0B8),
            aiu_fake_d0b8,
            &AIU_PREV_D0B8,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002DABC8),
            aiu_fake_abc8,
            &AIU_PREV_ABC8,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E8938),
            aiu_fake_8938,
            &AIU_PREV_8938,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E66B8),
            aiu_fake_66b8,
            &AIU_PREV_66B8,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002EADD0),
            aiu_fake_add0,
            &AIU_PREV_ADD0,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002EF6D0),
            aiu_fake_f6d0,
            &AIU_PREV_F6D0,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D4C08),
            aiu_fake_4c08,
            &AIU_PREV_4C08,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E39D8),
            aiu_fake_39d8,
            &AIU_PREV_39D8,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002DF920),
            aiu_fake_f920,
            &AIU_PREV_F920,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002F1150),
            aiu_fake_1150,
            &AIU_PREV_1150,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002F6518),
            aiu_fake_6518,
            &AIU_PREV_6518,
        );
    }

    /// `AI_Update` with every callee faked: the mission paths (no mission
    /// object, a busy one, a live one; no players), the player / computer /
    /// other collection (empty, negative and null records, flags off), the
    /// pose-only flag and the full stage-by-stage update, and unwinds at
    /// every call index.
    #[test]
    fn ai_update_matches() {
        ai_update_install_fakes();
        covered(
            ee::s00::f_00128af0__generated,
            ai_update,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0012_8af0, 70);
                c.pc = 0x0012_8af0;
                c.bus.write32(AI_UPDATE_CTRL, AI_UPDATE_MAGIC);
                let ai = DATA + 0x1000;
                c.g[A0][0] = sx(ai);
                fill(c, &mut r, ai, 0x30);
                // Rider records (0x40..0x78): null, a record with no rider, or a
                // record with a rider (whose model is a random word).
                for i in 0..15u32 {
                    let slot = ai + AI_UPDATE_PLAYERS + 4 * i;
                    let rec = DATA + 0x2000 + 0x20 * i;
                    let rider = DATA + 0x10000 + 0x1000 * i;
                    match r.next_u32() % 6 {
                        0 => c.bus.write32(slot, 0),
                        1 => {
                            c.bus.write32(slot, rec);
                            c.bus.write32(rec + AI_UPDATE_REC_RIDER, 0);
                        }
                        _ => {
                            c.bus.write32(slot, rec);
                            c.bus.write32(rec + AI_UPDATE_REC_RIDER, rider);
                            let model = r.next_u32();
                            c.bus.write32(rider + AI_UPDATE_RIDER_MODEL, model);
                        }
                    }
                }
                let count = |r: &mut crate::difftest::Rng, max: u32| match r.next_u32() % 5 {
                    0 => 0,
                    1 => (-1i32 - (r.next_u32() % 3) as i32) as u32,
                    _ => 1 + r.next_u32() % max,
                };
                let players = count(&mut r, 2);
                let computers = count(&mut r, 3);
                let others = count(&mut r, 3);
                c.bus.write32(ai + AI_UPDATE_NUM_PLAYERS, players);
                c.bus.write32(ai + AI_UPDATE_NUM_COMPUTERS, computers);
                c.bus.write32(ai + AI_UPDATE_NUM_OTHERS, others);
                let flag = |r: &mut crate::difftest::Rng| {
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    }
                };
                let have_computers = flag(&mut r);
                let have_others = flag(&mut r);
                c.bus.write32(ai + AI_UPDATE_HAVE_COMPUTERS, have_computers);
                c.bus.write32(ai + AI_UPDATE_HAVE_OTHERS, have_others);
                let missions = match seed % 4 {
                    0 | 1 => 0,
                    2 => 1 + r.next_u32() % 2,
                    _ => (-1i32) as u32,
                };
                c.bus.write32(ai + AI_UPDATE_NUM_MISSIONS, missions);
                // The world's mission object: none, busy, live.
                let world = DATA + 0x800;
                let holder = DATA + 0x900;
                let object = DATA + 0xa00;
                c.bus.write32(GAME_GP - AI_UPDATE_GP_WORLD, world);
                c.bus.write32(world + 0x84, holder);
                let object_ptr = if r.next_u32() % 4 == 0 { 0 } else { object };
                c.bus.write32(holder + 0x34, object_ptr);
                let busy = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    r.next_u32() | 1
                };
                c.bus.write32(object + 0x70, busy);
                // Debug flags: bit 0 = pose only.
                let flags = (r.next_u32() & !1) | (seed / 4 % 3 == 0) as u32;
                c.bus.write32(GAME_GP - AI_UPDATE_GP_FLAGS, flags);
            },
        );
    }
}
