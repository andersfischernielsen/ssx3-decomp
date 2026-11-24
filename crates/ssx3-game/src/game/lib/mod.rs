//! `src/lib/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod cdvd;
pub mod deci2;
pub mod dev;
pub mod dma;
pub mod eafile;
pub mod gcc;
pub mod ipu;
pub mod kernel;
pub mod libc;
pub mod mc;
pub mod mpeg;
pub mod pad;
pub mod scf;
pub mod sif;
pub mod snd;
pub mod usbkb;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(cdvd::OVERRIDES);
    v.extend_from_slice(deci2::OVERRIDES);
    v.extend_from_slice(dev::OVERRIDES);
    v.extend_from_slice(dma::OVERRIDES);
    v.extend_from_slice(eafile::OVERRIDES);
    v.extend_from_slice(gcc::OVERRIDES);
    v.extend_from_slice(ipu::OVERRIDES);
    v.extend_from_slice(kernel::OVERRIDES);
    v.extend_from_slice(libc::OVERRIDES);
    v.extend_from_slice(mc::OVERRIDES);
    v.extend_from_slice(mpeg::OVERRIDES);
    v.extend_from_slice(pad::OVERRIDES);
    v.extend_from_slice(scf::OVERRIDES);
    v.extend_from_slice(sif::OVERRIDES);
    v.extend_from_slice(snd::OVERRIDES);
    v.extend_from_slice(usbkb::OVERRIDES);
    v
}
