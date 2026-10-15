//! # ssx3-ps2
//!
//! The PlayStation 2 platform layer for the SSX 3 recompilation: everything
//! that is hardware or firmware rather than game code. Game code (EE and VU
//! microcode) is translated to Rust by `ssx3-recomp`. Generated EE code is
//! compiled against the small core crate `ssx3-ee-rt` ([`Cpu`], instruction
//! semantics, RAM fast paths); this crate plugs into it as the
//! [`Platform`](ssx3_ee_rt::Platform) behind memory-mapped I/O, VU0, syscalls
//! and events ([`platform`]), so changing it does not recompile generated
//! code. See `docs/investigation/crate-split.md` and
//! `docs/plans/playable-push.md`.

pub use ssx3_ee_rt::{bus, cpu, fpu, mmi};
// First, so its `m!`/`mr!` macros are in scope in every module below.
#[macro_use]
pub mod platform;
pub mod elf;
pub mod gifstats;
pub mod gs;
pub mod hw;
pub mod iop_bridge;
pub mod ipu;
pub mod iso;
pub mod kernel;
pub mod machine;
pub mod pad;
pub mod sif;
pub mod threads;
pub mod vif;
/// The Vector Units live in their own crate (build times); see `ssx3-vu`.
pub use ssx3_vu::vu;

pub use platform::{new_cpu, Machine};
pub use ssx3_ee_rt::{Cpu, Qword};

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
