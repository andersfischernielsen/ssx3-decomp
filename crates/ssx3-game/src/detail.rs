//! Detail distance (settings "Detail distance", `SSX3_DRAW_DIST`): how far
//! out the game keeps its detailed geometry.
//!
//! - Terrain: not here. `GameModule_DrawBezierPatch` (0x22c410) picks each
//!   patch edge's grid size from camera distance through a 4-entry table at
//!   0x4a2a10 (4, 4, 6, 8 points), but finer grids further out break the
//!   terrain (measured: large untextured areas): the renderer's terrain
//!   detail option refines the grids instead.
//! - Riders: `Rider_ComputeLOD` (0x121f30) scales the rider's view depth by
//!   the constant at 0x49b82c (0.01, loaded only there) and switches models
//!   at 6 and 10. Dividing the constant by the factor keeps the detailed
//!   model that much further away.
//!
//! At factor 1 nothing is written: the game is the original.

use std::sync::atomic::{AtomicU32, Ordering};

use ssx3_ee_rt::Cpu;

/// The terrain LOD table (grid sizes by LOD index; left alone, see above).
pub const TERRAIN_LOD_ORIGINAL: [u8; 4] = [4, 4, 6, 8];
/// The rider LOD distance scale (`f32`).
pub const RIDER_LOD_SCALE: u32 = 0x0049_b82c;
pub const RIDER_LOD_SCALE_ORIGINAL: f32 = 0.01;

/// The configured factor ×100 (100: original).
static FACTOR: AtomicU32 = AtomicU32::new(100);
/// The factor last written (0: none).
static APPLIED: AtomicU32 = AtomicU32::new(0);

/// Set the detail distance factor (1.0 = original, up to 4).
pub fn set(factor: f32) {
    FACTOR.store(
        (factor.clamp(1.0, 4.0) * 100.0).round() as u32,
        Ordering::Relaxed,
    );
}

/// At every VSync: write the tables when the factor changed.
pub fn on_vblank(c: &mut Cpu) {
    let want = FACTOR.load(Ordering::Relaxed);
    let done = APPLIED.load(Ordering::Relaxed);
    if done == want || (done == 0 && want == 100) {
        return;
    }
    let f = want as f32 / 100.0;
    c.bus
        .write32(RIDER_LOD_SCALE, (RIDER_LOD_SCALE_ORIGINAL / f).to_bits());
    APPLIED.store(want, Ordering::Relaxed);
    eprintln!(
        "[detail] distance x{f}: rider LOD scale {}",
        RIDER_LOD_SCALE_ORIGINAL / f
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_bits() {
        // The executable's values (0x4a2a10 holds 04 04 06 08, 0x49b82c 0.01).
        assert_eq!(u32::from_le_bytes(TERRAIN_LOD_ORIGINAL), 0x0806_0404);
        assert_eq!(RIDER_LOD_SCALE_ORIGINAL.to_bits(), 0x3c23_d70a);
    }
}
