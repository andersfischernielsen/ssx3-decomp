//! `src/object/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod animnode;
pub mod bucketman;
pub mod deadfadenode;
pub mod debouncenode;
pub mod effectlink;
pub mod flagset;
pub mod flexbridgenode;
pub mod flexrailnode;
pub mod floatingnode;
pub mod instanceman;
pub mod instancenode;
pub mod modifierblock;
pub mod movenode;
pub mod multiparticle;
pub mod objnode;
pub mod railmodifier;
pub mod spline3d;
pub mod splinemodifier;
pub mod spring;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(animnode::OVERRIDES);
    v.extend_from_slice(bucketman::OVERRIDES);
    v.extend_from_slice(deadfadenode::OVERRIDES);
    v.extend_from_slice(debouncenode::OVERRIDES);
    v.extend_from_slice(effectlink::OVERRIDES);
    v.extend_from_slice(flagset::OVERRIDES);
    v.extend_from_slice(flexbridgenode::OVERRIDES);
    v.extend_from_slice(flexrailnode::OVERRIDES);
    v.extend_from_slice(floatingnode::OVERRIDES);
    v.extend_from_slice(instanceman::OVERRIDES);
    v.extend_from_slice(instancenode::OVERRIDES);
    v.extend_from_slice(modifierblock::OVERRIDES);
    v.extend_from_slice(movenode::OVERRIDES);
    v.extend_from_slice(multiparticle::OVERRIDES);
    v.extend_from_slice(objnode::OVERRIDES);
    v.extend_from_slice(railmodifier::OVERRIDES);
    v.extend_from_slice(spline3d::OVERRIDES);
    v.extend_from_slice(splinemodifier::OVERRIDES);
    v.extend_from_slice(spring::OVERRIDES);
    v
}
