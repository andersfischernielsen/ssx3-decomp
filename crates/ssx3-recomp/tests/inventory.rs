//! Validates the embedded function inventory against known facts about
//! `SLUS_207.72`, and cross-checks it with `ssx3-core`'s memory map.

use ssx3_core::memory_map;
use ssx3_recomp::index;

#[test]
fn inventory_has_every_function_ghidra_found() {
    assert_eq!(index().len(), 11_844);
}

#[test]
fn inventory_is_sorted_and_unique() {
    assert!(index().is_sorted_unique());
}

#[test]
fn entry_point_is_present_and_agrees_with_memory_map() {
    let f = index()
        .by_address(memory_map::ENTRY_POINT)
        .expect("entry must be in the inventory");
    assert_eq!(f.name, "entry");
}

#[test]
fn every_function_lies_within_the_loaded_image() {
    let image = memory_map::SLUS_207_72;
    let lo = image.first().unwrap().start;
    let hi = image.last().unwrap().end;
    for f in index().functions() {
        assert!(
            (lo..=hi).contains(&f.address),
            "{} at {:08x} is outside the image {lo:08x}..={hi:08x}",
            f.name,
            f.address
        );
    }
}

#[test]
fn named_functions_include_the_sdk_entry_points() {
    // The emotion-engine extension recovered a number of SDK names.
    assert!(
        index().named_count() >= 60,
        "expected the SDK names to survive"
    );
    assert!(
        index().search("SetGsCrt").next().is_some(),
        "SetGsCrt should be named"
    );
}

#[test]
fn lookup_matches_a_linear_scan_for_every_function() {
    // Binary search must agree exactly with the exported order.
    for f in index().functions() {
        let found = index().by_address(f.address).unwrap();
        assert_eq!(found, f);
    }
}
