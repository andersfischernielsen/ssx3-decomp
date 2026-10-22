//! `src/worldpainter/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod quadtree;
pub mod worldpainterman;
pub mod worldpaintermanquery;
pub mod worldpainterquery;
pub mod worldpainterqueryctor;
pub mod wpfile;
pub mod wpigd;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(quadtree::OVERRIDES);
    v.extend_from_slice(worldpainterman::OVERRIDES);
    v.extend_from_slice(worldpaintermanquery::OVERRIDES);
    v.extend_from_slice(worldpainterquery::OVERRIDES);
    v.extend_from_slice(worldpainterqueryctor::OVERRIDES);
    v.extend_from_slice(wpfile::OVERRIDES);
    v.extend_from_slice(wpigd::OVERRIDES);
    v
}
