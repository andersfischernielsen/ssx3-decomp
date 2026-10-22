//! The hand-written replacements, laid out like the matching C++
//! decompilation's `src/` tree (one module per `.cpp` unit;
//! `tools/reorg_modules.py`).
// Each unit imports the union of what its functions' batch modules used
// (`tools/reorg_modules.py`); unused ones are expected.
#![allow(unused_imports)]

use ssx3_ee_rt::funcs::GuestFn;

pub mod ai;
pub mod animation;
pub mod be;
pub mod bx;
pub mod camera;
pub mod dirtysock;
pub mod fe;
pub mod input;
pub mod intersect;
pub mod lib;
pub mod luno;
pub mod main;
pub mod mem;
pub mod movie;
pub mod object;
pub mod path;
pub mod render;
pub mod replay;
pub mod scripter;
pub mod seg;
pub mod sound;
pub mod ui;
pub mod util;
pub mod visualfx;
pub mod world;
pub mod worldpainter;
pub mod wscript;
pub mod md5;
pub mod misc;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend(ai::overrides());
    v.extend(animation::overrides());
    v.extend(be::overrides());
    v.extend(bx::overrides());
    v.extend(camera::overrides());
    v.extend(dirtysock::overrides());
    v.extend(fe::overrides());
    v.extend(input::overrides());
    v.extend(intersect::overrides());
    v.extend(lib::overrides());
    v.extend(luno::overrides());
    v.extend(main::overrides());
    v.extend(mem::overrides());
    v.extend(movie::overrides());
    v.extend(object::overrides());
    v.extend(path::overrides());
    v.extend(render::overrides());
    v.extend(replay::overrides());
    v.extend(scripter::overrides());
    v.extend(seg::overrides());
    v.extend(sound::overrides());
    v.extend(ui::overrides());
    v.extend(util::overrides());
    v.extend(visualfx::overrides());
    v.extend(world::overrides());
    v.extend(worldpainter::overrides());
    v.extend(wscript::overrides());
    v.extend_from_slice(md5::OVERRIDES);
    v.extend_from_slice(misc::OVERRIDES);
    v
}
