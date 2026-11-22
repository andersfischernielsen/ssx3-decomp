//! Native CrowdAudio_GetInstanceReaction (0x2a77c8), retained uninstalled.
//!
//! Generated EE remains authoritative. The user disabled automated tests,
//! replays and gates; this translation has no test or installed override.

use crate::{abi, guest::Addr};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const INSTANCE_COUNT: u32 = 0x14;
const INSTANCES: u32 = 0x18;
const INSTANCE_BYTES: u32 = 0x190;
const REACTION: u32 = 0x18c;

/// `int CrowdAudio_GetInstanceReaction(CrowdAudio* audio, int index)`.
/// Return the selected 0x190-byte audio instance's signed reaction, or zero
/// when its signed index is not below the instance count. Retail has no
/// separate negative-index guard; callers must provide a valid index.
pub fn crowd_audio_get_instance_reaction(c: &mut Cpu) {
    c.cycles += 4;
    let audio = Addr(abi::arg(c, 0));
    c.g[V0][0] = audio.off(INSTANCE_COUNT).lw(c);
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let present = c.g[V0][0] != 0;
    c.g[V1][0] = INSTANCE_BYTES as u64;
    if present {
        c.cycles += 5;
        c.g[V0][0] = audio.off(INSTANCES).lw(c);
        c.mult(0, V1, A1, V1);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(REACTION).lw(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}
