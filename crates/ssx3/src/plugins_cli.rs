//! `ssx3 plugins list | prepare [id] | build`: plugin folders next to the game
//! (`docs/plans/plugins.md`).

use std::path::PathBuf;

use ssx3_engine::plugins::{filter, roots};
use ssx3_engine::ssx3_plugin::discover::{discover, find_disc_by_serial, Found, Status};
use ssx3_engine::ssx3_plugin::{prepare, workspace, Plugin};

const USAGE: &str = "usage: ssx3 plugins list | prepare [id] | build\n\
  list       the plugin folders found and what is missing for each\n\
  prepare    make a plugin's content from your disc image (all that need it, or one)\n\
  build      build a front end with the Rust code of the plugins that have it\n\
Folders: plugins/ next to the game data, or SSX3_PLUGIN_DIR; disc images: SSX3_DISC_DIR,\n\
next to the SSX 3 image, or the data folder. SSX3_PLUGINS=0 turns plugins off.";

pub fn run(args: &[String], linked: &[Box<dyn Plugin>]) -> i32 {
    match args.first().map(String::as_str) {
        Some("list") => list(linked),
        Some("prepare") => prepare_cmd(args.get(1).map(String::as_str)),
        Some("build") => build(),
        _ => {
            eprintln!("{USAGE}");
            2
        }
    }
}

fn describe(f: &Found) -> String {
    match &f.status {
        Status::Ready => {
            let n = f.manifest.as_ref().map_or(0, |m| m.events.len());
            format!("ready, {n} events")
        }
        Status::NeedsPrepare => "needs `ssx3 plugins prepare`".to_string(),
        Status::MissingInput(what) => format!("needs {what}"),
        Status::Invalid(errors) => format!("invalid: {}", errors.join("; ")),
        Status::Disabled => "off (SSX3_PLUGINS)".to_string(),
    }
}

fn list(linked: &[Box<dyn Plugin>]) -> i32 {
    let r = roots();
    let found = discover(&r, &filter());
    if found.is_empty() {
        println!(
            "no plugins in {}",
            r.plugin_dirs
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        return 0;
    }
    for f in &found {
        let code = f.manifest.as_ref().is_some_and(|m| m.code.is_some());
        let in_build = linked.iter().any(|p| p.id() == f.id);
        println!(
            "{:<16} {}{}  [{}]",
            f.id,
            describe(f),
            match (code, in_build) {
                (true, true) => " (Rust code, in this build)",
                (true, false) => " (Rust code, NOT in this build: `ssx3 plugins build`)",
                _ => "",
            },
            f.dir.display()
        );
    }
    0
}

fn base_disc() -> Option<PathBuf> {
    find_disc_by_serial(&roots().disc_dirs, &["SLUS-20772".to_string()])
}

fn prepare_cmd(only: Option<&str>) -> i32 {
    let r = roots();
    let found = discover(&r, &filter());
    let todo: Vec<&Found> = found
        .iter()
        .filter(|f| only.is_none_or(|id| f.id == id))
        .collect();
    if todo.is_empty() {
        eprintln!("no such plugin: {}", only.unwrap_or(""));
        return 1;
    }
    let base = base_disc();
    let mut failed = false;
    for f in todo {
        match &f.status {
            Status::NeedsPrepare => {
                println!(
                    "plugin {}: preparing from {}",
                    f.id,
                    f.disc
                        .as_ref()
                        .map_or("?".into(), |d| d.display().to_string())
                );
                match prepare::prepare(f, base.as_deref()) {
                    Ok(()) => println!("plugin {}: ready", f.id),
                    Err(e) => {
                        eprintln!("{e}");
                        failed = true;
                    }
                }
            }
            _ => println!("plugin {}: {}", f.id, describe(f)),
        }
    }
    i32::from(failed)
}

fn build() -> i32 {
    let found = discover(&roots(), &filter());
    let repo = ssx3_engine::repo_root();
    let out = repo.join("target/ssx3-custom");
    match workspace::build(&found, &repo, &out) {
        Ok(Some(bin)) => {
            println!("built {}", bin.display());
            0
        }
        Ok(None) => {
            println!("no plugin has Rust code: nothing to build (the stock ssx3 runs them all)");
            0
        }
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}
