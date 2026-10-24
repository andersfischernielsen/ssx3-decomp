//! The Windows executable's icon: `SSX3_ICON` names an `.ico` (made from the
//! game's logo by `tools/bundle.py`, which sets it); without it there is none.
//! The window's own icon is set at run time on every OS (`src/icon.rs`).

fn main() {
    println!("cargo:rerun-if-env-changed=SSX3_ICON");
    let Ok(ico) = std::env::var("SSX3_ICON") else { return };
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    println!("cargo:rerun-if-changed={ico}");
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let rc = out.join("ssx3.rc");
    std::fs::write(&rc, format!("1 ICON \"{}\"\n", ico.replace('\\', "/"))).unwrap();
    embed_resource::compile_for(&rc, ["ssx3"], embed_resource::NONE).manifest_required().unwrap();
}
