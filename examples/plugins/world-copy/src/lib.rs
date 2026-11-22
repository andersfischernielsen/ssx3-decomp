//! Spike for `docs/plans/plugins.md` (S0). `cGame_loadTrack` (0x22f6b0) builds
//! `data/worlds/<location>` from a location name in the race record: `BAM` in
//! retail. With `SSX3_WORLD_COPY=1` this hook turns the name into `BAX`, so the
//! game opens `DATA/WORLDS/BAX.BIG`, which the plugin's overlay serves (an
//! exact copy of BAM.BIG, entries renamed). If the event then runs, a world
//! archive can be chosen per event.

use ssx3_game::call::{call_ptr, guest};
use ssx3_plugin::reg::*;
use ssx3_plugin::{Cpu, HookError, Hooks, Plugin, Prev};

/// `cGame_loadTrack`.
const LOAD_WORLD: u32 = 0x0022_f6b0;
/// The calls the game itself makes to find the race record.
const GET_LIBRARY: u32 = 0x0014_dc80;
const GET_INTERFACE: u32 = 0x0014_dd58;
const GET_RECORD: u32 = 0x0014_4bc0;

static PREV: Prev = Prev::new();

fn load_world(c: &mut Cpu) {
    if std::env::var_os("SSX3_WORLD_COPY").is_some() {
        rename_location(c);
    }
    PREV.call(c);
}

/// Rewrite the location name in the race record, in place (`BAM` to `BAX`: the
/// same length).
fn rename_location(c: &mut Cpu) {
    // The function's own arguments, kept across the calls below.
    let (a0, ra) = (c.g[A0][0], c.g[RA][0]);
    guest(c, |c| {
        // The calls come back to the function's own entry, so `pc` is what
        // the function expects when the hook runs it.
        call_ptr(c, GET_LIBRARY, LOAD_WORLD)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, GET_INTERFACE, LOAD_WORLD)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, GET_RECORD, LOAD_WORLD)?;
        let gp = c.g[GP][0] as u32;
        let offset = if c.bus.read32(gp.wrapping_sub(0x1f38)) == 0 { 0x34 } else { 0x44 };
        let at = (c.g[V0][0] as u32).wrapping_add(offset);
        let name = c.bus.read_cstr(at);
        eprintln!("[world-copy] location {name:?} at {at:#x}");
        if name.eq_ignore_ascii_case("BAM") {
            for (i, b) in b"BAX\0".iter().enumerate() {
                c.bus.write8(at + i as u32, *b);
            }
            eprintln!("[world-copy] location is now BAX");
        }
        Ok(())
    });
    c.g[A0][0] = a0;
    c.g[RA][0] = ra;
    c.pc = LOAD_WORLD;
}

struct WorldCopy;

impl Plugin for WorldCopy {
    fn id(&self) -> &str {
        "world-copy"
    }

    fn install(&self, hooks: &mut Hooks<'_>) -> Result<(), HookError> {
        hooks.chain(LOAD_WORLD, load_world, &PREV)
    }
}

pub fn plugin() -> Box<dyn Plugin> {
    Box::new(WorldCopy)
}
