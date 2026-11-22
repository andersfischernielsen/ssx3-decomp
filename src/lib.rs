//! SSX Tricky as a plugin of ssx3-decomp (`docs/plugins.md` of the engine).
//!
//! The courses themselves are made from your Tricky disc by `prepare/`; this
//! crate holds the game code they need: hand-written replacements of the
//! functions that the Tricky data and rules change.

use ssx3_ee_rt::funcs::GuestFn;
use ssx3_plugin::{Cpu, HookError, Hooks, Plugin};

pub mod custom_tracks;
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
mod custom_rail_diagnostics;
mod custom_tricky_rail_probe;
mod custom_tricky_recovery;
mod custom_snap_ai;
mod custom_finish_trace;
mod custom_heap_diagnostics;
mod custom_audio_banks;
mod custom_contact_trace;
pub mod custom_rail_rules;
pub mod custom_script_manager_update;
pub mod custom_anim_object_factory;
pub mod custom_anim_object_ctor;
pub mod custom_race_path_select;
pub mod custom_race_distance;
pub mod custom_base_path_read;
pub mod bonus_menu_mode;
pub mod custom_path_reader;
pub mod custom_peak_widgets;
pub mod custom_ui_objects;
pub mod custom_ui_screen;
pub mod custom_instance_dma;
pub mod custom_collision_leaves;
pub mod custom_collision_resolve;
pub mod custom_world_resolve;
pub mod custom_static_objects;
pub mod custom_static_object_ctor;
pub mod custom_surface_defaults;
pub mod custom_attach_modifier;
pub mod custom_sphere_reader;
pub mod custom_worldscript;
pub mod custom_results;
pub mod custom_finish_dispatch;
pub mod custom_placement;
pub mod custom_nash_insert;
pub mod custom_resident_check;
pub mod custom_prerace_enter;
pub mod custom_prerace_update;
pub mod custom_reset_enter;
pub mod custom_path_markers;
pub mod custom_script_prepare;
pub mod custom_finish_update;
pub mod custom_restart_enter;
pub mod custom_results_enter;
pub mod custom_quit_load_screen;
pub mod custom_finish_script_queue;
pub mod custom_race_cutscene_queue;
pub mod custom_resource_alloc;
pub mod custom_timer_dispatch;
pub mod custom_timer_init;
pub mod custom_timer_add;
pub mod custom_ui_texture_bank;
pub mod custom_model_relocate;
pub mod custom_model_parts;
pub mod custom_menu_panels;
pub mod custom_model_dma_nodes;
pub mod custom_load_shape_texture;
pub mod custom_anger_render;
pub mod custom_fx_texture_load;
pub mod custom_texture_load;
pub mod custom_temp_texture;
pub mod custom_luno_execute;
pub mod custom_sky_render;
pub mod custom_loading_texture;
pub mod custom_peak_refresh;
pub mod custom_async_data;
pub mod custom_event_menu_update;
pub mod custom_menu_selection;
pub mod custom_text_id;
pub mod custom_camera_stream;
pub mod custom_section_alloc;
pub mod custom_text_id_string;

pub mod core_boot;
mod core_object;
mod core_race;
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
        bonus_menu_mode::OVERRIDES,
        custom_rail_diagnostics::OBSERVERS,
        custom_sky_render::OVERRIDES,
        custom_loading_texture::OVERRIDES,
        custom_peak_refresh::OVERRIDES,
        custom_async_data::OVERRIDES,
        custom_event_menu_update::OVERRIDES,
        custom_menu_selection::OVERRIDES,
        custom_text_id::OVERRIDES,
        custom_race_path_select::OVERRIDES,
        custom_race_distance::OVERRIDES,
        custom_base_path_read::OVERRIDES,
        custom_camera_stream::OVERRIDES,
        custom_section_alloc::OVERRIDES,
        custom_text_id_string::OVERRIDES,
        custom_path_reader::OVERRIDES,
        custom_peak_widgets::OVERRIDES,
        custom_ui_objects::OVERRIDES,
        custom_ui_screen::OVERRIDES,
        custom_instance_dma::OVERRIDES,
        custom_collision_leaves::OVERRIDES,
        custom_collision_resolve::OVERRIDES,
        custom_world_resolve::OVERRIDES,
        custom_static_objects::OVERRIDES,
        custom_anim_object_factory::OVERRIDES,
        custom_anim_object_ctor::OVERRIDES,
        custom_script_manager_update::OVERRIDES,
        custom_static_object_ctor::OVERRIDES,
        custom_surface_defaults::OVERRIDES,
        custom_attach_modifier::OVERRIDES,
        custom_sphere_reader::OVERRIDES,
        custom_worldscript::OVERRIDES,
        custom_results::OVERRIDES,
        custom_finish_dispatch::OVERRIDES,
        custom_tricky_rail_probe::OVERRIDES,
        custom_snap_ai::OVERRIDES,
        custom_placement::OVERRIDES,
        custom_nash_insert::OVERRIDES,
        custom_resident_check::OVERRIDES,
        custom_prerace_enter::OVERRIDES,
        custom_prerace_update::OVERRIDES,
        custom_reset_enter::OVERRIDES,
        custom_path_markers::OVERRIDES,
        custom_script_prepare::OVERRIDES,
        custom_finish_update::OVERRIDES,
        custom_restart_enter::OVERRIDES,
        custom_results_enter::OVERRIDES,
        custom_quit_load_screen::OVERRIDES,
        custom_finish_script_queue::OVERRIDES,
        custom_race_cutscene_queue::OVERRIDES,
        custom_resource_alloc::OVERRIDES,
        custom_timer_dispatch::OVERRIDES,
        custom_timer_init::OVERRIDES,
        custom_timer_add::OVERRIDES,
        custom_ui_texture_bank::OVERRIDES,
        custom_model_relocate::OVERRIDES,
        custom_model_parts::OVERRIDES,
        custom_menu_panels::OVERRIDES,
        custom_model_dma_nodes::OVERRIDES,
        custom_load_shape_texture::OVERRIDES,
        custom_anger_render::OVERRIDES,
        custom_fx_texture_load::OVERRIDES,
        custom_texture_load::OVERRIDES,
        custom_temp_texture::OVERRIDES,
        custom_luno_execute::OVERRIDES,
    ]
}

/// The hooks that run the function they wrap.
fn prev_of(addr: u32) -> Option<&'static ssx3_plugin::Prev> {
    match addr {
        0x0010_6848 => Some(&custom_rail_diagnostics::PREV),
        0x0010_86b8 => Some(&custom_tricky_rail_probe::PREV),
        0x0011_2d58 => Some(&custom_snap_ai::PREV),
        0x0020_a8f8 => Some(&custom_results_enter::PREV_OPEN),
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
        let mut missing = Vec::new();
        for chain in all_chains() {
            if hooks.has_slot(chain.addr) {
                hooks.chain(chain.addr, chain.f, chain.prev)?;
            } else {
                missing.push(format!("{:#010x} {}", chain.addr, chain.name));
            }
        }
        for table in tables() {
            for &(addr, f, name) in table {
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
