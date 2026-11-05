//! Visible bootstrap failure. No guest, renderer worker, card or audio starts.
use super::super::present::Presenter;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

struct NeedsData {
    image: Vec<u32>,
    window: Option<Arc<Window>>,
    presenter: Option<Presenter>,
}

pub fn run(event_loop: EventLoop<()>, message: &str) {
    eprintln!("[NeedsData] {message}");
    let text = format!(
        "{message}\n\nSee docs/android.md for pack provisioning.\nNo game or audio has started."
    );
    let mut app = NeedsData {
        image: draw(&text),
        window: None,
        presenter: None,
    };
    event_loop.set_control_flow(ControlFlow::Wait);
    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("NeedsData event loop: {e}");
    }
}

impl ApplicationHandler for NeedsData {
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if super::lifecycle::destroyed() {
            event_loop.exit();
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("SSX 3 - Game data needed"))
                .expect("window"),
        );
        if window.inner_size().width > 0 && window.inner_size().height > 0 {
            self.presenter = Some(pollster::block_on(Presenter::new(window.clone(), None)));
        }
        window.request_redraw();
        self.window = Some(window);
    }
    fn suspended(&mut self, _: &ActiveEventLoop) {
        self.presenter = None;
        self.window = None;
    }
    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                super::lifecycle::java(super::lifecycle::JavaAction::Finish)
            }
            WindowEvent::KeyboardInput { event, .. }
                if event.state.is_pressed()
                    && event.physical_key
                        == winit::keyboard::PhysicalKey::Code(
                            winit::keyboard::KeyCode::BrowserBack,
                        ) =>
            {
                super::lifecycle::java(super::lifecycle::JavaAction::Finish);
            }
            WindowEvent::Resized(size) => {
                if size.width == 0 || size.height == 0 {
                    self.presenter = None;
                } else if let Some(p) = self.presenter.as_mut() {
                    p.resize(size.width, size.height);
                } else if let Some(window) = &self.window {
                    self.presenter = Some(pollster::block_on(Presenter::new(window.clone(), None)));
                }
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if super::lifecycle::rebuild_needed() {
                    self.presenter = None;
                    if let Some(window) = &self.window {
                        let size = window.inner_size();
                        if size.width > 0 && size.height > 0 {
                            self.presenter =
                                Some(pollster::block_on(Presenter::new(window.clone(), None)));
                            super::lifecycle::rebuilt();
                        }
                    }
                }
                if let Some(p) = self.presenter.as_mut() {
                    let _ = p.render(800, 448, &self.image);
                }
            }
            _ => {}
        }
    }
}

fn draw(message: &str) -> Vec<u32> {
    use font8x8::UnicodeFonts;
    let mut image = vec![0xff28_140a; 800 * 448];
    let text = format!("GAME DATA NEEDED\n\n{message}");
    let (mut x, mut y) = (16, 16);
    for c in text.chars() {
        if c == '\n' || x + 8 > 784 {
            x = 16;
            y += 14;
            if c == '\n' {
                continue;
            }
        }
        if y + 8 >= 448 {
            break;
        }
        if let Some(glyph) = font8x8::BASIC_FONTS.get(c) {
            for (gy, row) in glyph.iter().enumerate() {
                for gx in 0..8 {
                    if row & (1 << gx) != 0 {
                        image[(y + gy) * 800 + x + gx] = 0xfffa_f0eb;
                    }
                }
            }
        }
        x += 8;
    }
    image
}
