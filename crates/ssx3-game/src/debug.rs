//! Debug-only replacements, installed by environment switches (never by
//! default). They wrap the generated body to observe or force values.

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;

const LENSFX_QUERY: u32 = 0x002e_c478;

/// `SSX3_DEBUG_LENSFX_VIS=<f32>`: force the result of LensFX's occlusion
/// query (`LensFX_v04`: the visible fraction of a light from a Z-buffer
/// readback) to a fixed value, to see which glows depend on it.
pub fn install() {
    if let Some(v) = std::env::var("SSX3_DEBUG_LENSFX_VIS")
        .ok()
        .and_then(|v| v.parse::<f32>().ok())
    {
        FORCED.store(v.to_bits(), std::sync::atomic::Ordering::Relaxed);
        assert!(ssx3_ee::set_override(LENSFX_QUERY, lensfx_query_forced));
        eprintln!("[debug] LensFX visibility forced to {v}");
    }
}

static FORCED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

fn lensfx_query_forced(c: &mut Cpu) {
    let out = c.g[A2][0] as u32;
    ssx3_ee::s09::f_002ec478__generated(c);
    c.bus
        .write32(out, FORCED.load(std::sync::atomic::Ordering::Relaxed));
}
