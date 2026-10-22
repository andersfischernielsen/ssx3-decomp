//! `src/lib/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod eafile;
pub mod gcc;
pub mod kernel;
pub mod libc;
pub mod sif;
pub mod snd;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(eafile::OVERRIDES);
    v.extend_from_slice(gcc::OVERRIDES);
    v.extend_from_slice(kernel::OVERRIDES);
    v.extend_from_slice(libc::OVERRIDES);
    v.extend_from_slice(sif::OVERRIDES);
    v.extend_from_slice(snd::OVERRIDES);
    v
}
