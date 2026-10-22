//! `src/be/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod be;
pub mod bedbprofile;
pub mod bedbreward;
pub mod beintaggression;
pub mod beintbagt;
pub mod beintecon;
pub mod beintmission;
pub mod beintnetwork;
pub mod beintnewrace;
pub mod beintoption;
pub mod beintplayer;
pub mod beintreward;
pub mod beintsavefile;
pub mod beintscore;
pub mod beintstat;
pub mod belibrary;
pub mod beprepoststate;
pub mod bestate;
pub mod bestate2;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(be::OVERRIDES);
    v.extend_from_slice(bedbprofile::OVERRIDES);
    v.extend_from_slice(bedbreward::OVERRIDES);
    v.extend_from_slice(beintaggression::OVERRIDES);
    v.extend_from_slice(beintbagt::OVERRIDES);
    v.extend_from_slice(beintecon::OVERRIDES);
    v.extend_from_slice(beintmission::OVERRIDES);
    v.extend_from_slice(beintnetwork::OVERRIDES);
    v.extend_from_slice(beintnewrace::OVERRIDES);
    v.extend_from_slice(beintoption::OVERRIDES);
    v.extend_from_slice(beintplayer::OVERRIDES);
    v.extend_from_slice(beintreward::OVERRIDES);
    v.extend_from_slice(beintsavefile::OVERRIDES);
    v.extend_from_slice(beintscore::OVERRIDES);
    v.extend_from_slice(beintstat::OVERRIDES);
    v.extend_from_slice(belibrary::OVERRIDES);
    v.extend_from_slice(beprepoststate::OVERRIDES);
    v.extend_from_slice(bestate::OVERRIDES);
    v.extend_from_slice(bestate2::OVERRIDES);
    v
}
