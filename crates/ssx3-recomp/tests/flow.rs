//! Tests for the recursive-descent disassembler against Ghidra's boundaries.

use std::collections::BTreeSet;
use std::path::Path;

use ssx3_recomp::boundaries::boundaries;
use ssx3_recomp::flow;
use ssx3_recomp::index;
use ssx3_recomp::program::Program;

fn open_program() -> Option<Program> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../orig/SLUS_207.72");
    if !path.exists() {
        eprintln!("skipping: {} not found", path.display());
        return None;
    }
    Some(Program::open(&path).expect("open SLUS_207.72"))
}

fn ours_within(f: &ssx3_recomp::Function, d: &flow::Disasm) -> BTreeSet<u32> {
    d.instructions
        .range(f.address..f.end)
        .map(|(a, _)| *a)
        .collect()
}

#[test]
fn entry_matches_ghidra_exactly() {
    let Some(p) = open_program() else { return };
    let f = p.function_by_name("entry").unwrap();
    let d = flow::disassemble(&p.elf, f.address, 4096);
    let runs = boundaries().runs(f.address).unwrap();
    let ours = ours_within(f, &d);
    let gh: BTreeSet<u32> = runs
        .iter()
        .flat_map(|&(lo, hi)| (lo..hi).step_by(4))
        .collect();
    assert_eq!(ours, gh, "entry boundaries differ from Ghidra");
}

#[test]
fn recursive_descent_does_not_walk_into_embedded_data() {
    // FUN_0042d470 contains a jump table at 0x42d584/0x42d5ec/.. that Ghidra does
    // not disassemble. Recursive descent must not reach it.
    let Some(p) = open_program() else { return };
    let f = index().by_address(0x0042_d470).unwrap();
    let d = flow::disassemble(&p.elf, f.address, (f.len() as usize / 4) + 64);
    let ours = ours_within(f, &d);
    for hole in [0x0042_d584u32, 0x0042_d5ec, 0x0042_d864] {
        assert!(
            !ours.contains(&hole),
            "{hole:08x} is data, not an instruction"
        );
    }
}

#[test]
fn recursive_descent_agrees_with_ghidra_broadly() {
    let Some(p) = open_program() else { return };
    let b = boundaries();
    let mut exact = 0usize;
    let mut total = 0usize;
    for f in p.functions() {
        let Some(runs) = b.runs(f.address) else {
            continue;
        };
        total += 1;
        let d = flow::disassemble(&p.elf, f.address, (f.len() as usize / 4) + 64);
        let ours = ours_within(f, &d);
        let gh: BTreeSet<u32> = runs
            .iter()
            .flat_map(|&(lo, hi)| (lo..hi).step_by(4))
            .collect();
        if ours == gh {
            exact += 1;
        }
    }
    // Ghidra additionally recovers code reached only via jump tables, which
    // recursive descent cannot follow, so this is a floor, not equality.
    assert!(
        exact * 100 >= total * 65,
        "exact match regressed: {exact}/{total}"
    );
}
