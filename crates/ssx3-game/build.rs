//! Fails the build when the generated EE code is older than `overrides.txt`.
//!
//! A function listed in `overrides.txt` needs a slot in the generated
//! `set_override` table; without one the game panics before its first frame,
//! so a pull that adds an override to an already-generated checkout would
//! otherwise build and then crash. Extra slots are harmless. The no-disc stubs
//! have no table at all and are skipped; any other tree without one is an error.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Path;

/// How many missing functions to name before summarising the rest.
const NAME_LIMIT: usize = 10;

/// What `tools/setup.py` writes for a no-disc stub (`is_set_up` checks it too).
const STUB_MARKER: &str = "FnEntry; 0]";

/// A generated EE form: the Cargo feature selecting it, its tree and how to
/// regenerate it.
struct Form {
    feature_env: &'static str,
    tree: &'static str,
    regenerate: &'static str,
}

const FORMS: [Form; 2] = [
    Form {
        feature_env: "CARGO_FEATURE_EE_THREADS",
        tree: "generated/ee",
        regenerate: "python3 tools/setup.py (or cargo run --release -p ssx3-recomp --bin recomp-ee)",
    },
    Form {
        feature_env: "CARGO_FEATURE_EE_UNWIND",
        tree: "generated/ee-unwind",
        regenerate: "cargo run --release -p ssx3-recomp --bin recomp-ee -- --unwind --out generated/ee-unwind --package-prefix ssx3-ee-unwind",
    },
];

/// The entries of `overrides.txt`, by address.
fn overrides(text: &str) -> BTreeMap<u32, &str> {
    text.lines()
        .filter_map(|line| {
            let mut words = line.split('#').next()?.split_whitespace();
            let address = u32::from_str_radix(words.next()?.trim_start_matches("0x"), 16).ok()?;
            Some((address, words.next().unwrap_or("")))
        })
        .collect()
}

/// The addresses of the `set_override` match arms (lines like
/// `        0x0011e098 => unsafe {`), or `None` without the function. The scan
/// runs to the end of the file: no other generated match arm has `unsafe {`.
fn generated_slots(lib: &str) -> Option<BTreeSet<u32>> {
    let (_, table) = lib.split_once("fn set_override(")?;
    Some(
        table
            .lines()
            .filter_map(|line| {
                let (address, rest) = line.trim_start().split_once(" => ")?;
                if !rest.starts_with("unsafe {") {
                    return None;
                }
                u32::from_str_radix(address.strip_prefix("0x")?, 16).ok()
            })
            .collect(),
    )
}

/// The error for `form`, if any override has no generated slot.
fn stale_message(
    form: &Form,
    overrides: &BTreeMap<u32, &str>,
    slots: &BTreeSet<u32>,
) -> Option<String> {
    let missing: Vec<String> = overrides
        .iter()
        .filter(|(address, _)| !slots.contains(address))
        .map(|(address, name)| {
            if name.is_empty() {
                format!("{address:#010x}")
            } else {
                format!("{name} ({address:#010x})")
            }
        })
        .collect();
    if missing.is_empty() {
        return None;
    }
    let mut names = missing[..missing.len().min(NAME_LIMIT)].join(", ");
    if missing.len() > NAME_LIMIT {
        names += &format!(" and {} more", missing.len() - NAME_LIMIT);
    }
    Some(format!(
        "{} is older than crates/ssx3-game/overrides.txt: no override slot for {names}; run {}",
        form.tree, form.regenerate
    ))
}

fn main() {
    let package = Path::new(env!("CARGO_MANIFEST_DIR"));
    let overrides_path = package.join("overrides.txt");
    println!("cargo:rerun-if-changed=overrides.txt");
    let overrides_text = fs::read_to_string(&overrides_path).expect("read overrides.txt");
    let overrides = overrides(&overrides_text);
    let root = package.join("../..");
    for form in FORMS
        .iter()
        .filter(|form| env::var_os(form.feature_env).is_some())
    {
        println!("cargo:rerun-if-changed=../../{}/game/src/lib.rs", form.tree);
        let path = root.join(form.tree).join("game/src/lib.rs");
        let lib = fs::read_to_string(&path).unwrap_or_default();
        if lib.contains(STUB_MARKER) {
            continue;
        }
        let Some(slots) = generated_slots(&lib) else {
            eprintln!(
                "error: {} has no set_override table; run {}",
                form.tree, form.regenerate
            );
            std::process::exit(1);
        };
        if let Some(message) = stale_message(form, &overrides, &slots) {
            eprintln!("error: {message}");
            std::process::exit(1);
        }
    }
}
