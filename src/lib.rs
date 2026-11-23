//! SSX Tricky as a plugin of ssx3-decomp (`docs/plugins.md` of the engine).
//!
//! The courses themselves are made from your Tricky disc by `prepare/`; this
//! crate holds the game code they need: hand-written replacements of the
//! functions that the Tricky data and rules change.

use ssx3_ee_rt::funcs::GuestFn;
use ssx3_plugin::{Cpu, HookError, Hooks, Plugin};

pub mod custom_tracks;
mod custom_rail_diagnostics;
pub mod custom_results_enter;
pub mod custom_course_rules;
pub mod custom_script_effects;
pub mod custom_script_queue;
mod custom_script_predicates;
mod custom_script_materials;
mod custom_counter_trace;
mod custom_roller_runtime;
mod custom_roller_update;
mod custom_initial_debounce;
mod custom_animation_debounce;
mod custom_crowds;
mod custom_movies;
mod custom_cracked_glass;
mod custom_collision_emitters;
mod custom_fences;
mod custom_flags;
mod custom_spline_movers;
mod custom_force_overlap;
mod custom_tricky_rail_probe;
mod custom_tricky_recovery;
mod custom_snap_ai;
mod custom_finish_trace;
mod custom_audio_banks;
mod custom_contact_trace;
pub mod custom_rail_rules;
pub mod custom_script_manager_update;
pub mod custom_finish_update;
pub mod custom_luno_execute;

pub mod core_boot;
mod core_object;
pub mod core_race;
mod core_ui;

/// A hook that wraps the function in the slot (`Hooks::chain`): it runs the
/// previous function through `prev`.
pub struct Chain {
    pub addr: u32,
    pub f: GuestFn,
    pub prev: &'static ssx3_plugin::Prev,
    pub name: &'static str,
}

/// A direct call to the guest function at `A` through its slot (what the
/// fork's `ssx3_ee::sNN::f_X` paths did): the call stays valid when the
/// function is hooked.
pub fn thunk<const A: u32>(c: &mut Cpu) {
    c.pc = A;
    let f = ssx3_ee_rt::funcs::lookup(A).expect("thunk: no function at the address");
    f(c)
}

/// Every replacement this plugin installs: `(address, function, name)`.
fn tables() -> Vec<&'static [(u32, GuestFn, &'static str)]> {
    vec![
        core_object::OVERRIDES,
        core_race::OVERRIDES,
        core_ui::OVERRIDES,
        custom_script_manager_update::OVERRIDES,
        custom_tricky_rail_probe::OVERRIDES,
        custom_snap_ai::OVERRIDES,
        custom_finish_update::OVERRIDES,
        custom_luno_execute::OVERRIDES,
    ]
}

/// The hooks that run the function they wrap.
fn prev_of(addr: u32) -> Option<&'static ssx3_plugin::Prev> {
    match addr {
        0x0010_86b8 => Some(&custom_tricky_rail_probe::PREV),
        0x0011_2d58 => Some(&custom_snap_ai::PREV),
        _ => None,
    }
}

/// The wrapping hooks of the core-function conversions.
fn all_chains() -> Vec<&'static Chain> {
    let mut v: Vec<&'static Chain> = Vec::new();
    v.extend(core_boot::CHAINS);
    v.extend(core_object::CHAINS);
    v.extend(core_race::CHAINS);
    v.extend(core_ui::CHAINS);
    v
}

struct Tricky;

impl Plugin for Tricky {
    fn id(&self) -> &str {
        "tricky"
    }

    fn install(&self, hooks: &mut Hooks<'_>) -> Result<(), HookError> {
        core_boot::configure();
        ssx3_game::packetless::set_opaque_blend_hook(core_race::screen_blend);
        // Events picked on the Select Peak screen (built from plugin.toml by
        // the engine): the course becomes the active one.
        ssx3_game::plugin_host::add_event_listener(ssx3_game::plugin_host::EventListener {
            started: core_boot::event_started,
            ended: core_boot::event_ended,
        });
        let mut missing = Vec::new();
        // `SSX3_TRICKY_SKIP=0x3a7878,0x26a180`: leave these functions to the
        // engine (to find which hook changes a run).
        let skip: Vec<u32> = std::env::var("SSX3_TRICKY_SKIP")
            .unwrap_or_default()
            .split(',')
            .filter_map(|a| u32::from_str_radix(a.trim().trim_start_matches("0x"), 16).ok())
            .collect();
        for chain in all_chains() {
            if skip.contains(&chain.addr) {
                continue;
            }
            if hooks.has_slot(chain.addr) {
                hooks.chain(chain.addr, chain.f, chain.prev)?;
            } else {
                missing.push(format!("{:#010x} {}", chain.addr, chain.name));
            }
        }
        for table in tables() {
            for &(addr, f, name) in table {
                if skip.contains(&addr) {
                    continue;
                }
                if !hooks.has_slot(addr) {
                    missing.push(format!("{addr:#010x} {name}"));
                    continue;
                }
                match prev_of(addr) {
                    Some(prev) => hooks.chain(addr, f, prev)?,
                    None => hooks.replace(addr, f)?,
                }
            }
        }
        if !missing.is_empty() {
            eprintln!("[tricky] no override slot for: {}", missing.join(", "));
        }
        Ok(())
    }
}

/// What `ssx3 plugins build` links in.
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Tricky)
}
