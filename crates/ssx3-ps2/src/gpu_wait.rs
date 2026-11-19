//! Every place the CPU waits for the GPU goes through [`wait`], which counts
//! the calls and the time spent per [`Site`].
//!
//! wgpu holds the device's resource lock for the whole of a device wait, and
//! a surface present needs that lock exclusively: with a window, a present on
//! another thread then waits until the GPU is done, and every wgpu call
//! queued behind it waits too, so the CPU and the GPU no longer overlap.
//! Headless runs never present and do not show it, but their counts do:
//! a site that runs every frame is one a window would stall on.

use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

macro_rules! sites {
    ($($site:ident $name:literal $device:literal,)*) => {
        /// A place that waits for the GPU.
        #[derive(Clone, Copy)]
        pub enum Site { $($site,)* }
        impl Site {
            const ALL: &[Site] = &[$(Site::$site,)*];
            fn name(self) -> &'static str {
                match self { $(Site::$site => $name,)* }
            }
            /// A wait on the device itself, not on a thread that waits for it.
            fn device(self) -> bool {
                match self { $(Site::$site => $device,)* }
            }
        }
    };
}

sites! {
    GfxDepthSync "gfx-depth-sync" true,
    GfxProbe "gfx-probe" true,
    GfxReadTexture "gfx-read-texture" true,
    GfxTakeDepth "gfx-take-depth" false,
    GfxFlush "gfx-flush" false,
    GfxRenderQueue "gfx-render-queue" false,
    GsProf "gs-prof" true,
    GsInflightCap "gs-inflight-cap" true,
    GsLatentIdx "gs-latent-idx" true,
    GsLatentIdle "gs-latent-idle" true,
    GsWaitIdle "gs-wait-idle" true,
    GsReadStaging "gs-read-staging" true,
    GsReadTargets "gs-read-targets" true,
    GsHiresSnapshot "gs-hires-snapshot" true,
    GsSubmitInflight "gs-submit-inflight" true,
    GsSubmitExit "gs-submit-exit" true,
    GsDrain "gs-drain" false,
    ContextCreate "context-create" true,
    PresenterNew "presenter-new" true,
    PresentConfigure "present-configure" true,
    PresentErrorScope "present-error-scope" true,
}

const SITES: usize = Site::ALL.len();
static CALLS: [AtomicU64; SITES] = [const { AtomicU64::new(0) }; SITES];
static WAIT_NS: [AtomicU64; SITES] = [const { AtomicU64::new(0) }; SITES];

/// Run `f`, a wait for the GPU, counted as `site`.
pub fn wait<R>(site: Site, f: impl FnOnce() -> R) -> R {
    CALLS[site as usize].fetch_add(1, Relaxed);
    let start = web_time::Instant::now();
    let r = f();
    WAIT_NS[site as usize].fetch_add(start.elapsed().as_nanos() as u64, Relaxed);
    r
}

/// Start the counts over (a benchmark's window).
pub fn reset() {
    for (calls, ns) in CALLS.iter().zip(&WAIT_NS) {
        calls.store(0, Relaxed);
        ns.store(0, Relaxed);
    }
}

/// Calls at the sites that wait on the device itself.
pub fn device_waits() -> u64 {
    (Site::ALL.iter().filter(|s| s.device()))
        .map(|s| CALLS[*s as usize].load(Relaxed))
        .sum()
}

/// Per frame over `frames`: calls and wait time of each site that was called.
pub fn report(frames: f64) -> String {
    let sites: Vec<String> = Site::ALL
        .iter()
        .filter(|s| CALLS[**s as usize].load(Relaxed) > 0)
        .map(|s| {
            format!(
                "{} {:.3} x {:.2} ms{}",
                s.name(),
                CALLS[*s as usize].load(Relaxed) as f64 / frames,
                WAIT_NS[*s as usize].load(Relaxed) as f64 / 1e6 / frames,
                if s.device() { "" } else { " (thread)" },
            )
        })
        .collect();
    if sites.is_empty() {
        "none".into()
    } else {
        sites.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_unique() {
        for (i, a) in Site::ALL.iter().enumerate() {
            for b in &Site::ALL[i + 1..] {
                assert_ne!(a.name(), b.name());
            }
        }
    }
}
