//! `src/bx/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod appman;
pub mod bigfile;
pub mod bxstring;
pub mod bxstring2;
pub mod bxstringctor;
pub mod cubicspline;
pub mod execman;
pub mod memblockman;
pub mod ps2main;
pub mod seg_1ba100;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(appman::OVERRIDES);
    v.extend_from_slice(bigfile::OVERRIDES);
    v.extend_from_slice(bxstring::OVERRIDES);
    v.extend_from_slice(bxstring2::OVERRIDES);
    v.extend_from_slice(bxstringctor::OVERRIDES);
    v.extend_from_slice(cubicspline::OVERRIDES);
    v.extend_from_slice(execman::OVERRIDES);
    v.extend_from_slice(memblockman::OVERRIDES);
    v.extend_from_slice(ps2main::OVERRIDES);
    v.extend_from_slice(seg_1ba100::OVERRIDES);
    v
}
