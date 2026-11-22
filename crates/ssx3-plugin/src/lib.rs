//! Plugins for ssx3-decomp: extra content (peaks, events, courses) on top of
//! SSX 3, from folders next to the game. See `docs/plans/plugins.md`.

pub mod catalog;
pub mod discover;
pub mod hooks;
pub mod load;
pub mod manifest;
pub mod plugin;
pub mod prepare;

pub use hooks::{HookError, Hooks, Prev, Slots};
pub use load::{load, Loaded};
pub use plugin::Plugin;

/// The plugin API version, `major.minor`. Before 1.0 a plugin names the exact
/// version.
pub const API_VERSION: (u32, u32) = (0, 1);
