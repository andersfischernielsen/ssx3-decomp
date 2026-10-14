//! Debug statistics over GIF packets (enabled with `SSX3_GIF_STATS`): counts
//! vertices, ADC (no-kick) vertices and primitives per path, to diagnose
//! culling/clipping problems in geometry produced by VU1 microcode.

use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::sync::OnceLock;

use crate::Qword;

pub static VERTS: AtomicU64 = AtomicU64::new(0);
pub static ADC: AtomicU64 = AtomicU64::new(0);
pub static FAR: AtomicU64 = AtomicU64::new(0);
pub static PACKETS: AtomicU64 = AtomicU64::new(0);

pub fn enabled() -> bool {
    static E: OnceLock<bool> = OnceLock::new();
    *E.get_or_init(|| std::env::var("SSX3_GIF_STATS").is_ok())
}

/// Scan one GIF packet (PACKED XYZ registers only).
pub fn scan(p: &[Qword]) {
    PACKETS.fetch_add(1, Relaxed);
    let mut i = 0;
    while i < p.len() {
        let tag = p[i];
        i += 1;
        let nloop = (tag & 0x7fff) as usize;
        let eop = (tag >> 15) & 1 != 0;
        let flg = ((tag >> 58) & 3) as u32;
        let nreg = match ((tag >> 60) & 15) as usize {
            0 => 16,
            n => n,
        };
        let regs = (tag >> 64) as u64;
        match flg {
            0 => {
                for _ in 0..nloop {
                    for r in 0..nreg {
                        let Some(&q) = p.get(i) else { return };
                        i += 1;
                        let id = (regs >> (4 * r)) & 15;
                        if id == 4 || id == 5 || id == 0xc || id == 0xd {
                            VERTS.fetch_add(1, Relaxed);
                            if (q >> 111) & 1 != 0 {
                                ADC.fetch_add(1, Relaxed);
                            }
                            let x = (q & 0xffff) as i32 - 0x8000;
                            let y = ((q >> 32) & 0xffff) as i32 - 0x8000;
                            if x.abs() > 0x6000 || y.abs() > 0x6000 {
                                FAR.fetch_add(1, Relaxed);
                            }
                        }
                    }
                }
            }
            1 => i += (nloop * nreg).div_ceil(2),
            _ => i += nloop,
        }
        if eop {
            break;
        }
    }
}

/// Take and reset the counters: (packets, vertices, adc, far).
pub fn take() -> (u64, u64, u64, u64) {
    (PACKETS.swap(0, Relaxed), VERTS.swap(0, Relaxed), ADC.swap(0, Relaxed), FAR.swap(0, Relaxed))
}
