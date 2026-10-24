//! Tests that code generation folds syscalls into native calls and uses direct
//! function calls (never an address dispatcher).

use std::collections::BTreeSet;
use std::path::Path;

use ssx3_recomp::codegen;
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

#[test]
fn syscalls_become_native_sdk_calls() {
    let Some(p) = open_program() else { return };
    let entry = p.function_by_name("entry").unwrap();
    let src = codegen::emit_function(&p, entry, &BTreeSet::new());

    // `li v1,0x3c; syscall` -> init_main_thread; `li v1,0x3d` -> init_heap.
    assert!(
        src.contains("crate::sdk::init_main_thread(c, a0, a1, a2, a3);"),
        "expected a native InitMainThread call:\n{src}"
    );
    assert!(
        src.contains("crate::sdk::init_heap(c, a0, a1);"),
        "expected a native InitHeap call"
    );
    // No runtime syscall-number dispatcher on the folded path.
    assert!(
        !src.contains("c.dynamic_syscall"),
        "constant syscalls must not fall back"
    );
}

#[test]
fn direct_calls_are_direct_name_calls() {
    let Some(p) = open_program() else { return };
    let entry = p.function_by_name("entry").unwrap();

    // entry calls 0x0042c300. With that function translated, the emitted code
    // must be a direct call, not a dispatch on the target address.
    let mut translated = BTreeSet::new();
    translated.insert(0x0042_c300u32);
    let src = codegen::emit_function(&p, entry, &translated);
    assert!(
        src.contains("f_0042c300(c);"),
        "expected a direct call to f_0042c300:\n{src}"
    );
    assert!(
        !src.contains("dispatch("),
        "no address dispatcher should appear"
    );
    assert!(!src.contains("generated::"));
}

#[test]
fn entry_codegen_has_no_unimplemented_instructions() {
    let Some(p) = open_program() else { return };
    let entry = p.function_by_name("entry").unwrap();
    let src = codegen::emit_function(&p, entry, &BTreeSet::new());
    assert!(
        !src.contains("unimplemented("),
        "entry should lower completely:\n{src}"
    );
}

#[test]
fn untranslated_callee_is_a_graceful_stop() {
    let Some(p) = open_program() else { return };
    let entry = p.function_by_name("entry").unwrap();
    let src = codegen::emit_function(&p, entry, &BTreeSet::new());
    assert!(
        src.contains("c.untranslated_call("),
        "an untranslated callee should stop cleanly, not be missed"
    );
}

#[test]
fn every_emitted_function_is_syntactically_balanced() {
    // A cheap guard against emitting broken Rust: braces must balance.
    let Some(p) = open_program() else { return };
    let translated: BTreeSet<u32> = index()
        .functions()
        .iter()
        .take(64)
        .map(|f| f.address)
        .collect();
    for f in index().functions().iter().take(64) {
        let src = codegen::emit_function(&p, f, &translated);
        let open = src.matches('{').count();
        let close = src.matches('}').count();
        assert_eq!(
            open, close,
            "unbalanced braces in {} @ {:08x}",
            f.name, f.address
        );
    }
}

#[test]
fn some_functions_emit_structured_control_flow() {
    // Functions without loops are emitted as structured Rust (no `match pc`).
    let Some(p) = open_program() else { return };
    let sample: Vec<_> = index().functions().iter().take(300).collect();
    let translated: BTreeSet<u32> = sample.iter().map(|f| f.address).collect();
    let mut structured = 0;
    for f in &sample {
        let src = codegen::emit_function(&p, f, &translated);
        if !src.contains("match pc") {
            assert!(!src.contains("pc = "), "structured code must not use pc");
            structured += 1;
        }
    }
    eprintln!("structured {structured}/{}", sample.len());
    assert!(
        structured * 100 >= sample.len() * 10,
        "structured coverage regressed: {structured}/{}",
        sample.len()
    );
}

#[test]
fn structured_function_has_no_dispatcher_or_pc() {
    // A function known to structure (no loops): FUN_00424020.
    let Some(p) = open_program() else { return };
    let f = index().by_address(0x0042_4020).unwrap();
    let translated: BTreeSet<u32> = [0x0042_4020u32].into_iter().collect();
    let src = codegen::emit_function(&p, f, &translated);
    assert!(
        !src.contains("match pc"),
        "expected structured output:\n{src}"
    );
    assert!(!src.contains("pc = "), "structured output must not use pc");
}
