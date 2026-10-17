//! The browser build's VU helper (`crates/ssx3-web/www/vuworker.js`): the
//! renderer's chunks of VU1 calls run here, in parallel, as they run on
//! threads natively. Jobs and results are bytes (`ssx3_render::vujob`).
#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;

/// Run one encoded job; returns the encoded result.
#[wasm_bindgen]
pub fn run_job(job: &[u8]) -> Vec<u8> {
    ssx3_render::vujob::run(job)
}
