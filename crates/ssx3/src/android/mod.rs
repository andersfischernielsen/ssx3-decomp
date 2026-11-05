//! Android application boundary. The native game lives in its own process.
mod bootstrap;
mod display;
mod entry;
mod lifecycle;
mod needs_data;
mod policy;
pub use lifecycle::{refresh_rate, surface_lost, Lifecycle};
