//! The process-wide wgpu device: the native renderer (ssx3-gfx), the window's
//! presenter and (in verification builds) the emulated GS's GPU backend all
//! use the same device, so frames never leave the GPU.

use std::sync::OnceLock;

/// The shared device and queue.
pub struct GpuContext {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

/// The shared device, created on first use; `None` if no adapter is usable.
pub fn context() -> Option<&'static GpuContext> {
    CONTEXT
        .get_or_init(|| {
            // The browser cannot block on the device: `init_context` first.
            if cfg!(target_arch = "wasm32") {
                return None;
            }
            context_or_log(pollster::block_on(create_context()))
        })
        .as_ref()
}

static CONTEXT: OnceLock<Option<GpuContext>> = OnceLock::new();

fn context_or_log(r: Result<GpuContext, String>) -> Option<GpuContext> {
    r.map_err(|e| eprintln!("[gs] GPU backend unavailable: {e}"))
        .ok()
}

/// Create the shared device without blocking (the browser's way; native
/// code may call `context` directly).
pub async fn init_context() -> Option<&'static GpuContext> {
    if CONTEXT.get().is_none() {
        let c = context_or_log(create_context().await);
        let _ = CONTEXT.set(c);
    }
    CONTEXT.get().and_then(Option::as_ref)
}

async fn create_context() -> Result<GpuContext, String> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            ..Default::default()
        })
        .await
        .map_err(|e| format!("request_adapter: {e}"))?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("ssx3-gs"),
            required_limits: adapter.limits(),
            required_features: (if std::env::var_os("SSX3_GS_PROFILE").is_some() {
                adapter.features() & wgpu::Features::TIMESTAMP_QUERY
            } else {
                wgpu::Features::empty()
            }) | if crate::gs::render_native_type3() {
                adapter.features() & wgpu::Features::DUAL_SOURCE_BLENDING
            } else {
                wgpu::Features::empty()
            } | adapter.features()
                & (wgpu::Features::DEPTH_CLIP_CONTROL | wgpu::Features::CLIP_DISTANCES),
            ..Default::default()
        })
        .await
        .map_err(|e| format!("request_device: {e}"))?;
    device.set_device_lost_callback(|reason, msg| {
        eprintln!("[gs] GPU device lost ({reason:?}): {msg}");
    });
    Ok(GpuContext {
        instance,
        adapter,
        device,
        queue,
    })
}
