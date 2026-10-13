//! SSX 3 (USA) native entry point.
//!
//! Brings up the machine model and a `wgpu` window that presents the Graphics
//! Synthesizer VRAM. Recompiled EE code is wired in on top of this.

use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use ssx3_core::{memory_map, Memory};
use ssx3_gs::{Gs, GsRenderer};

/// How many redraws to perform before exiting, when set via
/// `SSX3_EXIT_AFTER_FRAMES`. Useful for smoke-testing in CI.
fn exit_after_frames() -> Option<u64> {
    std::env::var("SSX3_EXIT_AFTER_FRAMES")
        .ok()
        .and_then(|s| s.parse().ok())
}

struct App {
    gs: Gs,
    _mem: Memory,
    window: Option<Arc<Window>>,
    renderer: Option<GsRenderer>,
    frames: u64,
    exit_after: Option<u64>,
}

impl App {
    fn new() -> Self {
        Self {
            gs: Gs::new(),
            _mem: Memory::new(),
            window: None,
            renderer: None,
            frames: 0,
            exit_after: exit_after_frames(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("SSX 3 (USA) — GS VRAM")
            .with_inner_size(winit::dpi::LogicalSize::new(768.0, 768.0));
        let window = Arc::new(
            event_loop
                .create_window(attrs)
                .expect("failed to create window"),
        );
        let renderer = pollster::block_on(GsRenderer::new(window.clone()));
        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.renderer.as_mut() {
                    if let Err(err) = renderer.render(&self.gs) {
                        log::debug!("frame skipped: {err:?}");
                    }
                }
                self.frames += 1;
                if let Some(limit) = self.exit_after {
                    if self.frames >= limit {
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

fn main() {
    env_logger::init();

    let mem = Memory::new();
    println!("SSX 3 (USA) — SLUS_207.72");
    println!(
        "  image base : {:#010x}   entry: {:#010x}",
        memory_map::IMAGE_BASE,
        memory_map::ENTRY_POINT
    );
    println!("  main RAM   : {} MiB", mem.ram.len() / (1024 * 1024));
    println!("  scratchpad : {} KiB", mem.scratchpad.len() / 1024);
    drop(mem);

    let event_loop = EventLoop::new().expect("event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new();
    if let Err(err) = event_loop.run_app(&mut app) {
        eprintln!("event loop error: {err}");
        std::process::exit(1);
    }
    println!("presented {} frame(s)", app.frames);
}
