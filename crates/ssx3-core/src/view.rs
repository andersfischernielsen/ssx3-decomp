//! How the 2D layer is placed in the window, shared between the renderer
//! (which knows the window's size) and the game code that lays out the
//! screens (the race HUD at the window's edges).

use std::sync::atomic::{AtomicU32, Ordering};

static HUD_SQUEEZE: AtomicU32 = AtomicU32::new(0x3f80_0000);

/// The renderer's squeeze of the 2D layer around the frame's centre (1: none;
/// below 1: the 4:3 layer is narrower than a wider window).
pub fn set_hud_squeeze(s: f32) {
    HUD_SQUEEZE.store(s.to_bits(), Ordering::Release);
}

/// The squeeze set with [`set_hud_squeeze`] (1 until a window sets it).
pub fn hud_squeeze() -> f32 {
    f32::from_bits(HUD_SQUEEZE.load(Ordering::Acquire))
}
