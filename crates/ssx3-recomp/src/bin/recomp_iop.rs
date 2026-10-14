//! Translate the IOP sound modules (LIBSD.IRX, SNDDRV.IRX) from the disc
//! image into the crate `ssx3-iop-gen` under `generated/iop/modules/`.
//!
//! ```sh
//! cargo run --release -p ssx3-recomp --bin recomp-iop [-- path/to.iso]
//! ```
//!
//! `recomp-ee` runs this step too. Output is not checked in; `report.txt` next to
//! the crate lists the modules, imports, and suspicious load-delay uses.

use std::path::{Path, PathBuf};

fn main() {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize().unwrap();
    let iso = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .or_else(|| std::env::var("SSX3_ISO").ok().map(PathBuf::from))
        .unwrap_or_else(|| root.join("SSX 3 (USA).iso"));
    match ssx3_recomp::iopgen::generate_from_disc(&iso, &root.join("generated/iop/modules")) {
        Ok(o) => {
            for l in o.report.lines().filter(|l| !l.starts_with("load-delay")) {
                println!("{l}");
            }
            println!("wrote generated/iop/modules ({} bytes of Rust)", o.code.len());
        }
        Err(e) => {
            eprintln!("recomp-iop: {e}");
            std::process::exit(1);
        }
    }
}
