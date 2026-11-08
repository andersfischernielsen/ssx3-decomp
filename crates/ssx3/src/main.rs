//! SSX 3 (USA) — native recompilation, windowed front end.
//!
//! Runs the game (`ssx3-engine`) in a winit window: the engine's pacing,
//! the renderer's image presented with `wgpu`, keyboard and controller
//! input, the settings file and overlay (F10).
//!
//! Keys (pad 1): arrows = d-pad, X = Cross, Z = Square, S = Circle, A = Triangle,
//! Enter = Start, Right Shift = Select, Q/W = L1/R1, 1/2 = L2/R2.
//! Game controllers (via gilrs) map to pad 1 too, analog sticks included; a
//! second controller drives pad 2 when `SSX3_PAD2=1` plugs it in (Multi Play).

use std::sync::Arc;
use std::time::{Duration, Instant};

use ssx3_engine::{Engine, EngineConfig, Pace};

use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowId};

mod icon;
mod png;
mod present;
mod settings;

use present::Presenter;
use ssx3_ps2::Cpu;

struct App {
    /// The game: machine, renderer, pacing.
    engine: Engine,
    window: Option<Arc<Window>>,
    presenter: Option<Presenter>,
    frame: Vec<u32>,
    last: Instant,
    started: Instant,
    exit_after: Option<u64>,
    shots: u64,
    fps_mark: Instant,
    /// Pad 1 buttons held on the keyboard / on a game controller (libpad bits).
    keys: u16,
    gilrs: Option<gilrs::Gilrs>,
    /// Ticket of the previous frame's display request (MTGS): the window
    /// shows frame N-1 (or newer) while the GS thread finishes frame N.
    #[cfg(feature = "emu-gs")]
    pending_display: Option<u64>,
    /// The GS's wgpu device (GPU backend).
    gpu_ctx: Option<&'static ssx3_ps2::gpu::GpuContext>,
    /// The settings overlay (F10): open with this row selected; the game
    /// is paused meanwhile.
    menu: Option<usize>,
    /// The window is hidden (occluded) or the app suspended: paused too.
    occluded: bool,
    suspended: bool,
    /// A setting that applies after a restart was changed.
    restart_pending: bool,
    /// Game frames to run while the overlay is open: after a change that
    /// applies at once, so the image behind shows it.
    preview: u32,
}

/// libpad button bit for a controller button.
fn gamepad_bit(b: gilrs::Button) -> Option<u16> {
    use gilrs::Button::*;
    Some(match b {
        Select => 1 << 0,
        LeftThumb => 1 << 1,
        RightThumb => 1 << 2,
        Start => 1 << 3,
        DPadUp => 1 << 4,
        DPadRight => 1 << 5,
        DPadDown => 1 << 6,
        DPadLeft => 1 << 7,
        LeftTrigger2 => 1 << 8,
        RightTrigger2 => 1 << 9,
        LeftTrigger => 1 << 10,
        RightTrigger => 1 << 11,
        North => 1 << 12,
        East => 1 << 13,
        South => 1 << 14,
        West => 1 << 15,
        _ => return None,
    })
}

impl App {
    /// Merge keyboard and controller state into the pads: the keyboard and
    /// the first game controller drive pad 1, a second controller pad 2
    /// (plugged in with `SSX3_PAD2=1`).
    fn update_pad(&mut self) {
        let mut state = [(self.keys, [0x80u8; 4]), (0u16, [0x80u8; 4])];
        let mut open_menu = false;
        if let Some(g) = self.gilrs.as_mut() {
            while let Some(ev) = g.next_event() {
                match ev.event {
                    gilrs::EventType::Connected => {
                        let p = g.gamepad(ev.id);
                        eprintln!(
                            "[pad] controller connected: {} (vendor {:04x?}, product {:04x?}, mapping {:?})",
                            p.name(),
                            p.vendor_id(),
                            p.product_id(),
                            p.mapping_source()
                        );
                    }
                    gilrs::EventType::ButtonPressed(b, code)
                        if ssx3_core::config::var_os("SSX3_PAD_DEBUG").is_some() =>
                    {
                        eprintln!("[pad] pressed {b:?} (code {code:?})");
                    }
                    gilrs::EventType::AxisChanged(a, v, code)
                        if ssx3_core::config::var_os("SSX3_PAD_DEBUG").is_some()
                            && v.abs() > 0.5 =>
                    {
                        eprintln!("[pad] axis {a:?} {v:.2} (code {code:?})");
                    }
                    gilrs::EventType::Disconnected => {
                        eprintln!("[pad] controller disconnected: {}", g.gamepad(ev.id).name())
                    }
                    // The controller's home/guide button opens the settings.
                    gilrs::EventType::ButtonPressed(gilrs::Button::Mode, _) => open_menu = true,
                    _ => {}
                }
            }
            for (port, (_, pad)) in g.gamepads().take(2).enumerate() {
                let (buttons, sticks) = &mut state[port];
                for b in [
                    gilrs::Button::Select,
                    gilrs::Button::LeftThumb,
                    gilrs::Button::RightThumb,
                    gilrs::Button::Start,
                    gilrs::Button::DPadUp,
                    gilrs::Button::DPadRight,
                    gilrs::Button::DPadDown,
                    gilrs::Button::DPadLeft,
                    gilrs::Button::LeftTrigger2,
                    gilrs::Button::RightTrigger2,
                    gilrs::Button::LeftTrigger,
                    gilrs::Button::RightTrigger,
                    gilrs::Button::North,
                    gilrs::Button::East,
                    gilrs::Button::South,
                    gilrs::Button::West,
                ] {
                    if let Some(bit) = gamepad_bit(b) {
                        if pad.is_pressed(b) {
                            *buttons |= bit;
                        }
                    }
                }
                // PS2 sticks: 0 = left/up, 0xFF = right/down; gilrs Y is up-positive.
                let axis = |a: gilrs::Axis, flip: bool| {
                    let v = pad.value(a);
                    let v = if flip { -v } else { v };
                    ((v.clamp(-1.0, 1.0) * 127.5) + 127.5).round() as u8
                };
                *sticks = [
                    axis(gilrs::Axis::RightStickX, false),
                    axis(gilrs::Axis::RightStickY, true),
                    axis(gilrs::Axis::LeftStickX, false),
                    axis(gilrs::Axis::LeftStickY, true),
                ];
            }
        }
        if open_menu && self.menu.is_none() {
            self.toggle_menu();
        }
        for (port, (buttons, sticks)) in state.into_iter().enumerate() {
            self.engine.set_pad(port, buttons);
            self.engine.set_sticks(port, sticks);
        }
    }
}

fn pad_bit(k: KeyCode) -> Option<u16> {
    // libpad button bits (active-low in the wire format; we store active-high).
    Some(match k {
        KeyCode::ShiftRight => 1 << 0, // Select
        KeyCode::Enter => 1 << 3,      // Start
        KeyCode::ArrowUp => 1 << 4,
        KeyCode::ArrowRight => 1 << 5,
        KeyCode::ArrowDown => 1 << 6,
        KeyCode::ArrowLeft => 1 << 7,
        KeyCode::Digit1 => 1 << 8, // L2
        KeyCode::Digit2 => 1 << 9, // R2
        KeyCode::KeyQ => 1 << 10,  // L1
        KeyCode::KeyW => 1 << 11,  // R1
        KeyCode::KeyA => 1 << 12,  // Triangle
        KeyCode::KeyS => 1 << 13,  // Circle
        KeyCode::KeyX => 1 << 14,  // Cross
        KeyCode::KeyZ => 1 << 15,  // Square
        _ => return None,
    })
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            self.suspended = false;
            self.sync_pause();
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("SSX 3 (USA) — native recompilation")
            .with_inner_size(winit::dpi::LogicalSize::new(960.0, 672.0));
        // SSX3_FULLSCREEN=1: start in borderless fullscreen (F11 toggles).
        let attrs = if settings::get("SSX3_FULLSCREEN").is_some_and(|v| v != "0") {
            attrs.with_fullscreen(Some(Fullscreen::Borderless(None)))
        } else {
            attrs
        };
        // The SSX 3 logo from the disc (title bar and taskbar on Windows and
        // Linux; macOS takes the app bundle's icon).
        let attrs = match icon::from_disc(128)
            .and_then(|rgba| winit::window::Icon::from_rgba(rgba, 128, 128).ok())
        {
            Some(i) => attrs.with_window_icon(Some(i)),
            None => attrs,
        };
        let window = Arc::new(event_loop.create_window(attrs).expect("window"));
        // No mouse in the game (the overlay too is keys and pad only): hide
        // the pointer while it is over the window.
        window.set_cursor_visible(false);
        let gs = ssx3_ps2::gpu::context();
        self.gpu_ctx = gs;
        self.presenter = Some(pollster::block_on(Presenter::new(window.clone(), gs)));
        let size = window.inner_size();
        let display_hz = window
            .current_monitor()
            .and_then(|m| m.refresh_rate_millihertz())
            .map_or(60.0, |m| (m as f64 / 1000.0).clamp(24.0, 500.0));
        let locked = self.engine.set_display_hz(display_hz);
        eprintln!(
            "[window] display {:.0} Hz, game {} Hz{}",
            display_hz,
            self.engine.vsync_hz(),
            if locked {
                ", locked to the display"
            } else {
                ""
            }
        );
        self.window = Some(window);
        self.engine.set_surface_size(size.width, size.height);
    }

    /// Mobile and web platforms suspend the app (never called on desktops).
    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.suspended = true;
        self.sync_pause();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(p) = self.presenter.as_mut() {
                    p.resize(size.width, size.height);
                }
                self.engine.set_surface_size(size.width, size.height);
            }
            // Hidden (minimized, covered, another space): pause.
            WindowEvent::Occluded(o) => {
                self.occluded = o;
                self.sync_pause();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    let pressed = event.state == ElementState::Pressed;
                    if code == KeyCode::F10 && pressed && !event.repeat {
                        self.toggle_menu();
                        return;
                    }
                    if self.menu.is_some() {
                        if pressed {
                            match code {
                                KeyCode::ArrowUp => self.menu_input(MenuKey::Up),
                                KeyCode::ArrowDown => self.menu_input(MenuKey::Down),
                                KeyCode::ArrowLeft => self.menu_input(MenuKey::Left),
                                KeyCode::ArrowRight => self.menu_input(MenuKey::Right),
                                KeyCode::Enter | KeyCode::Space | KeyCode::KeyX => {
                                    self.menu_input(MenuKey::Select)
                                }
                                KeyCode::Escape | KeyCode::Backspace => {
                                    self.menu_input(MenuKey::Close)
                                }
                                _ => {}
                            }
                        }
                        return;
                    }
                    // F11: borderless fullscreen on and off.
                    if code == KeyCode::F11 && event.state == ElementState::Pressed && !event.repeat
                    {
                        if let Some(w) = self.window.as_ref() {
                            w.set_fullscreen(if w.fullscreen().is_some() {
                                None
                            } else {
                                Some(Fullscreen::Borderless(None))
                            });
                        }
                    }
                    if code == KeyCode::F12 && event.state == ElementState::Pressed {
                        // The image on screen: the native renderer's while it is active.
                        let (w, h) = match self.engine.read_image() {
                            Some((w, h, px)) => {
                                self.frame = px;
                                (w, h)
                            }
                            #[cfg(feature = "emu-gs")]
                            None => ssx3_ps2::m!(self.engine.cpu_mut())
                                .hw
                                .gs
                                .core()
                                .display(&mut self.frame),
                            #[cfg(not(feature = "emu-gs"))]
                            None => (0, 0),
                        };
                        let path = format!("/tmp/ssx3-shot-{}.png", self.shots);
                        self.shots += 1;
                        let _ = std::fs::write(&path, png::encode(w, h, &self.frame));
                        eprintln!("screenshot: {path}");
                    }
                    if let Some(bit) = pad_bit(code) {
                        if event.state == ElementState::Pressed {
                            self.keys |= bit;
                        } else {
                            self.keys &= !bit;
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                // The settings overlay pauses the game.
                if self.menu.is_some() {
                    self.poll_menu_pad();
                    // (Paused: the pacer starts over when the game resumes.)
                    if self.preview > 0 {
                        self.preview -= 1;
                        if !self.step(event_loop) {
                            return;
                        }
                    }
                    self.show();
                    return;
                }
                // Fixed-timestep loop: the VSyncs (game updates) the wall
                // clock asks for, then the newest image. A display slower
                // than the game rate gets several VSyncs per image, a faster
                // one waits for the next update.
                match self.engine.pace(self.started.elapsed().as_secs_f64()) {
                    // Show the last image again: the next call comes with
                    // the next display refresh.
                    Pace::Repeat => self.show(),
                    Pace::Wait(w) => std::thread::sleep(Duration::from_secs_f64(w.min(0.002))),
                    Pace::Paused => std::thread::sleep(Duration::from_millis(2)),
                    Pace::Run(n) => {
                        for _ in 0..n {
                            if !self.step(event_loop) {
                                return;
                            }
                        }
                        self.show();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(w) = self.window.as_ref() {
            w.request_redraw();
        }
    }
}

impl App {
    /// Run one game frame; false when the game halted.
    fn step(&mut self, event_loop: &ActiveEventLoop) -> bool {
        self.update_pad();
        let alive = self.engine.step();
        if self.engine.frames() % 60 == 0 {
            eprintln!(
                "[{:6.1}s] {}",
                self.started.elapsed().as_secs_f64(),
                self.engine.status()
            );
            {
                let c = self.engine.cpu_mut();
                let app = c.bus.read32(ssx3_game::tick::APPMAN_PTR);
                let updates = if app != 0 {
                    c.bus.read32(app + 0x1c)
                } else {
                    0
                };
                eprintln!(
                    "          game updates {updates}, audio gaps {:.2}s total, lead {} ms, repeated images {}, catch-up VSyncs {}",
                    ssx3_iop::audio::underrun_frames() as f64 / 48000.0,
                    ssx3_ps2::machine::audio_lead_frames() / 48,
                    self.engine.pacer().repeats,
                    self.engine.pacer().doubled
                );
            }
            let now = Instant::now();
            let fps = 60.0 / now.duration_since(self.fps_mark).as_secs_f64().max(1e-6);
            self.fps_mark = now;
            if let Some(w) = self.window.as_ref() {
                w.set_title(&format!(
                    "SSX 3 (USA) — native recompilation — {fps:.0} fps"
                ));
            }
        }
        if !alive {
            eprintln!(
                "halted: {:?}",
                ssx3_ps2::mr!(self.engine.cpu()).kernel.halted
            );
            event_loop.exit();
            return false;
        }
        if let Some(n) = self.exit_after {
            if self.engine.frames() >= n {
                event_loop.exit();
            }
        }
        true
    }

    /// Present the latest image.
    fn show(&mut self) {
        if let Some(p) = self.presenter.as_mut() {
            // Rendering at the window's size fills it exactly.
            let native_fill = self.engine.native_image() && self.engine.follows_surface();
            p.set_aspect(if native_fill {
                self.engine.aspect_box().unwrap_or(0.0)
            } else {
                display_aspect(self.engine.cpu())
            });
            match self.menu {
                Some(sel) => {
                    let (w, h, img) = settings::draw(sel, self.restart_pending);
                    p.set_overlay(Some((w, h, &img)));
                }
                None => p.set_overlay(None),
            }
        }
        let gfx = self.engine.renderer().cloned();
        let native = ssx3_ps2::frame_sink::native();
        #[cfg(feature = "emu-gs")]
        let gs = &mut ssx3_ps2::m!(self.engine.cpu_mut()).hw.gs;
        match self.presenter.as_mut() {
            // Native: the renderer's latest image (it lives on the
            // GS device: no copy).
            Some(p) if native && gfx.is_some() => {
                let g = gfx.as_ref().unwrap();
                if p.same_device(self.gpu_ctx) {
                    if let Some((view, _, _)) = g.present_view() {
                        let _ = p.render_view(&view);
                    }
                } else if let Some((w, h, px)) = g.read_image() {
                    self.frame = px.iter().map(|p| p | 0xFF00_0000).collect();
                    let _ = p.render(w, h, &self.frame);
                }
            }
            // GPU GS: the frame stays on the GPU. The display pass
            // is queued on the GS thread right behind this frame's
            // drawing; the previous request's frame (or a newer
            // one) is presented, so the guest never waits for the
            // GS thread to finish the frame it just produced.
            #[cfg(feature = "emu-gs")]
            Some(p) if p.same_device(self.gpu_ctx) => {
                let ticket = gs.request_display();
                let prev = self.pending_display.replace(ticket).unwrap_or(ticket);
                match gs.take_display(prev) {
                    Some(Some(frame)) => {
                        let _ = p.render_gpu(&frame);
                    }
                    Some(None) => {
                        let (w, h) = gs.core().display(&mut self.frame);
                        let _ = p.render(w, h, &self.frame);
                    }
                    None => {} // nothing new: keep the last image
                }
            }
            #[cfg(feature = "emu-gs")]
            Some(p) => {
                let (w, h) = gs.core().display(&mut self.frame);
                let _ = p.render(w, h, &self.frame);
            }
            _ => {}
        }
        self.last = Instant::now();
    }

    /// Pause while the overlay is open, the window hidden or the app
    /// suspended.
    fn sync_pause(&mut self) {
        if self.menu.is_some() || self.occluded || self.suspended {
            self.engine.pause();
        } else {
            self.engine.resume();
        }
    }

    fn toggle_menu(&mut self) {
        self.menu = match self.menu {
            Some(_) => None,
            None => {
                // (No key stays held in the game.)
                self.keys = 0;
                Some(0)
            }
        };
        self.sync_pause();
    }

    fn menu_input(&mut self, k: MenuKey) {
        let Some(sel) = self.menu else { return };
        let n = settings::ITEMS.len();
        match k {
            MenuKey::Up => self.menu = Some((sel + n) % (n + 1)),
            MenuKey::Down => self.menu = Some((sel + 1) % (n + 1)),
            MenuKey::Close => {
                self.menu = None;
                self.sync_pause();
            }
            MenuKey::Select if sel == n => {
                self.menu = None;
                self.sync_pause();
            }
            MenuKey::Left | MenuKey::Right | MenuKey::Select if sel < n => {
                let item = &settings::ITEMS[sel];
                settings::step(item, if k == MenuKey::Left { -1 } else { 1 });
                if item.live {
                    self.apply_setting(item.key);
                    self.preview = 3;
                } else {
                    self.restart_pending = true;
                }
            }
            _ => {}
        }
    }

    /// Apply a changed setting now.
    fn apply_setting(&mut self, key: &str) {
        match key {
            "SSX3_FULLSCREEN" => {
                if let Some(w) = self.window.as_ref() {
                    let on = settings::get("SSX3_FULLSCREEN").is_some_and(|v| v != "0");
                    w.set_fullscreen(on.then_some(Fullscreen::Borderless(None)));
                }
            }
            // Catch-up, the renderer's options and debug knobs, the view.
            k => self.engine.setting_changed(k),
        }
    }

    /// Controller navigation while the overlay is open.
    fn poll_menu_pad(&mut self) {
        let mut keys = Vec::new();
        if let Some(g) = self.gilrs.as_mut() {
            while let Some(ev) = g.next_event() {
                if let gilrs::EventType::ButtonPressed(b, _) = ev.event {
                    keys.push(match b {
                        gilrs::Button::DPadUp => MenuKey::Up,
                        gilrs::Button::DPadDown => MenuKey::Down,
                        gilrs::Button::DPadLeft => MenuKey::Left,
                        gilrs::Button::DPadRight => MenuKey::Right,
                        gilrs::Button::South => MenuKey::Select,
                        gilrs::Button::East | gilrs::Button::Mode | gilrs::Button::Start => {
                            MenuKey::Close
                        }
                        _ => continue,
                    });
                }
            }
        }
        for k in keys {
            self.menu_input(k);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MenuKey {
    Up,
    Down,
    Left,
    Right,
    Select,
    Close,
}

/// The aspect ratio to show the game at. SSX3_ASPECT=4:3 / 16:9 / stretch
/// overrides; by default it follows the game's own widescreen option
/// (options word at 0x535610, bits 20-21: off, 16:9 letterboxed in a 4:3
/// image, anamorphic), so anamorphic is shown at 16:9.
fn display_aspect(cpu: &Cpu) -> f32 {
    static FORCED: std::sync::OnceLock<Option<f32>> = std::sync::OnceLock::new();
    let forced = *FORCED.get_or_init(|| match ssx3_core::config::var("SSX3_ASPECT")?.as_str() {
        "stretch" => Some(0.0),
        "16:9" => Some(16.0 / 9.0),
        "4:3" => Some(4.0 / 3.0),
        v => {
            eprintln!("SSX3_ASPECT={v}: expected 4:3, 16:9 or stretch");
            None
        }
    });
    forced.unwrap_or(if (cpu.bus.ram_u32(0x0053_5610) >> 20) & 3 == 2 {
        16.0 / 9.0
    } else {
        4.0 / 3.0
    })
}

fn main() {
    // A standalone build finds its data first (it chooses the settings file).
    ssx3_engine::init_paths(None, None, true);
    // Then: the settings file fills in the environment.
    settings::load();
    env_logger::init();
    let engine = Engine::boot(EngineConfig {
        find_pack: true,
        ..EngineConfig::new(Box::new(settings::Desktop))
    });
    let event_loop = EventLoop::new().expect("event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App {
        engine,
        window: None,
        presenter: None,
        frame: Vec::new(),
        last: Instant::now(),
        started: Instant::now(),
        exit_after: ssx3_core::config::var("SSX3_EXIT_AFTER_FRAMES").and_then(|s| s.parse().ok()),
        shots: 0,
        fps_mark: Instant::now(),
        keys: 0,
        #[cfg(feature = "emu-gs")]
        pending_display: None,
        gpu_ctx: None,
        menu: None,
        occluded: false,
        suspended: false,
        restart_pending: false,
        preview: 0,
        gilrs: gilrs::Gilrs::new()
            .map(|g| {
                let pads: Vec<String> = g.gamepads().map(|(_, p)| p.name().to_string()).collect();
                // Controllers usually arrive a moment later, as Connected
                // events (logged in `update_pad`).
                if !pads.is_empty() {
                    eprintln!("[pad] controllers: {}", pads.join(", "));
                }
                g
            })
            .map_err(|e| eprintln!("no game controller support: {e}"))
            .ok(),
    };
    // SSX3_CATCHUP (on by default): game time and audio follow the wall
    // clock when the host is slower than the game rate.
    app.engine.setting_changed("SSX3_CATCHUP");
    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("event loop error: {e}");
        std::process::exit(1);
    }
}
