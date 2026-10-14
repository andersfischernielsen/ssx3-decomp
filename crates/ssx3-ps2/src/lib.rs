//! # ssx3-ps2
//!
//! The PlayStation 2 platform layer for the SSX 3 recompilation: everything
//! that is hardware or firmware rather than game code. Game code (EE and VU
//! microcode) is translated to Rust by `ssx3-recomp`; it runs against [`Cpu`]
//! and calls into this crate for instruction semantics, memory-mapped I/O,
//! the kernel, and the IOP services. See `docs/plans/playable-push.md`.

pub mod bus;
pub mod cop2;
pub mod cpu;
pub mod elf;
pub mod fpu;
pub mod gifstats;
pub mod gs;
pub mod hw;
pub mod iop_bridge;
pub mod ipu;
pub mod iso;
pub mod kernel;
pub mod machine;
pub mod mmi;
pub mod pad;
pub mod sif;
pub mod threads;
pub mod vif;
pub mod vu;

pub use cpu::Cpu;

/// One 128-bit quadword as moved by DMA, GIF and VIF.
pub type Qword = u128;

/// Debug: `SSX3_TRACE_PATH3=<from>:<to>` logs GIF DMA tags, VIF1
/// MSKPATH3/FLUSH*/DIRECT/MSCAL commands and GIF packets per path, in order,
/// for frames `from..=to` (`1` alone: always).
pub fn trace_path3() -> bool {
    static E: std::sync::OnceLock<Option<(u64, u64)>> = std::sync::OnceLock::new();
    let r = E.get_or_init(|| {
        let v = std::env::var("SSX3_TRACE_PATH3").ok()?;
        let mut it = v.split(':').map(|x| x.parse::<u64>().ok());
        match (it.next().flatten(), it.next().flatten()) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => Some((0, u64::MAX)),
        }
    });
    r.is_some_and(|(a, b)| (a..=b).contains(&vu::capture::frame()))
}
