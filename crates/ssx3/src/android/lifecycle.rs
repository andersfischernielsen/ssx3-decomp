use super::policy::Policy;
use jni::{jni_sig, jni_str, objects::JObject, JavaVM};
use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    Mutex,
};
use std::time::{Duration, Instant};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoopProxy};
use winit::platform::android::activity::AndroidApp;

static REBUILD: AtomicBool = AtomicBool::new(false);
// Set by Java when a signal drops, so a loss and return between two polls
// still pauses (PAUSE) or restarts the audio request (MUTE).
static PAUSE: AtomicBool = AtomicBool::new(false);
static MUTE: AtomicBool = AtomicBool::new(false);
static JAVA: AtomicU32 = AtomicU32::new(0);
static WAKE: Mutex<Option<Box<dyn Fn() + Send>>> = Mutex::new(None);
thread_local! { static ACTIVITY: std::cell::RefCell<Option<AndroidApp>> = const { std::cell::RefCell::new(None) }; }

// Java sends this before entering NativeActivity.onDestroy, which waits
// for android_main. No JNI callback waits on native work or audio.
#[no_mangle]
pub extern "system" fn Java_org_ssx3_android_AndroidActivity_nativeState(
    _: *mut jni::sys::JNIEnv,
    _: jni::sys::jobject,
    bits: i32,
) {
    JAVA.store(bits as u32, Ordering::Release);
    if bits & 15 != 7 {
        MUTE.store(true, Ordering::Release);
        ssx3_iop::audio::android_lifecycle::silence();
    }
    if bits & 11 != 3 {
        PAUSE.store(true, Ordering::Release);
    }
    if let Some(wake) = WAKE.lock().unwrap().as_ref() {
        wake();
    }
}

pub fn install(app: AndroidApp, proxy: EventLoopProxy<()>) {
    // A raw AndroidAppWaker wake is ignored by winit without queued work.
    // Queue a real user event so focus gain can leave ControlFlow::Wait.
    *WAKE.lock().unwrap() = Some(Box::new(move || {
        let _ = proxy.send_event(());
    }));
    ACTIVITY.with(|a| *a.borrow_mut() = Some(app));
}
pub fn surface_lost() {
    REBUILD.store(true, Ordering::Release);
}
pub fn rebuild_needed() -> bool {
    REBUILD.load(Ordering::Acquire)
}
pub fn rebuilt() {
    REBUILD.store(false, Ordering::Release);
}
pub fn destroyed() -> bool {
    JAVA.load(Ordering::Acquire) & 8 != 0
}
#[derive(Debug, Clone, Copy)]
pub enum JavaAction {
    EnableAudio,
    Playing,
    Paused,
    Finish,
}
pub fn java(method: JavaAction) {
    ACTIVITY.with(|a| {
        let a = a.borrow();
        let Some(app) = a.as_ref() else {
            return;
        };
        let result = (|| -> jni::errors::Result<()> {
            let vm = JavaVM::singleton()?;
            let ptr: jni::sys::jobject = app.activity_as_ptr().cast();
            vm.attach_current_thread(|env| {
                let object = unsafe { env.as_cast_raw::<JObject>(&ptr)? };
                match method {
                    JavaAction::EnableAudio => {
                        env.call_method(&*object, jni_str!("enableAudio"), jni_sig!("()V"), &[])?;
                    }
                    JavaAction::Playing => {
                        env.call_method(&*object, jni_str!("playing"), jni_sig!("()V"), &[])?;
                    }
                    JavaAction::Paused => {
                        env.call_method(&*object, jni_str!("paused"), jni_sig!("()V"), &[])?;
                    }
                    JavaAction::Finish => {
                        env.call_method(&*object, jni_str!("finish"), jni_sig!("()V"), &[])?;
                    }
                }
                Ok(())
            })
        })();
        if let Err(e) = result {
            eprintln!("[android] {method:?}: {e}");
        }
    });
}

#[derive(Default)]
pub struct Lifecycle {
    policy: Policy,
    active: bool,
    audio: bool,
    stopping: Option<Instant>,
    confirmed: bool,
    heard: bool,
    first_poll: bool,
}
impl Lifecycle {
    pub fn event(app: &mut super::super::App, event: &WindowEvent) {
        match event {
            WindowEvent::Focused(value) => app.android.policy.native_focus = *value,
            WindowEvent::Resized(size) => {
                app.android.policy.surface = size.width > 0 && size.height > 0;
                if !app.android.policy.surface {
                    app.presenter = None;
                }
            }
            WindowEvent::KeyboardInput { event, .. } if event.state.is_pressed() => {
                if event.physical_key
                    == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::BrowserBack)
                {
                    java(JavaAction::Finish);
                }
            }
            _ => {}
        }
    }
    pub fn suspend(app: &mut super::super::App) {
        app.android.policy.surface = false;
        app.android.policy.native_focus = false;
        Self::neutralize(app);
        ssx3_iop::audio::android_lifecycle::request(false);
        app.android.audio = false;
        app.android.active = false;
        app.presenter = None; // surface must go before the native window
        app.window = None;
    }
    pub fn neutralize(app: &mut super::super::App) {
        app.keys = 0;
        let sif = &mut ssx3_ps2::m!(app.cpu).sif;
        sif.pad_buttons = [0; 2];
        sif.pad_sticks = [[0x80; 4]; 2];
        app.pacer.reset();
    }
    // Audio never gates play: a missing, slow or failed stream only
    // leaves the game silent.
    pub fn ready(app: &super::super::App) -> bool {
        app.android.confirmed
            && app.android.active
            && !REBUILD.load(Ordering::Acquire)
            && app.android.policy.playing(JAVA.load(Ordering::Acquire))
            && !PAUSE.load(Ordering::Acquire)
    }
    pub fn poll(app: &mut super::super::App, event_loop: &ActiveEventLoop) -> bool {
        use ssx3_iop::audio::android_lifecycle as audio;
        if !app.android.first_poll {
            app.android.first_poll = true;
            eprintln!("[android] first lifecycle poll");
        }
        if destroyed() && app.android.stopping.is_none() {
            Self::neutralize(app);
            app.presenter = None;
            app.window = None;
            app.android.stopping = Some(Instant::now());
            audio::shutdown();
        }
        if let Some(since) = app.android.stopping {
            if audio::settled() || since.elapsed() >= Duration::from_secs(1) {
                event_loop.exit();
            }
            event_loop.set_control_flow(ControlFlow::WaitUntil(
                Instant::now() + Duration::from_millis(10),
            ));
            return false;
        }
        if PAUSE.swap(false, Ordering::AcqRel) {
            Self::neutralize(app);
            app.android.active = false;
        }
        if MUTE.swap(false, Ordering::AcqRel) && app.android.audio {
            audio::request(false);
            app.android.audio = false;
            eprintln!(
                "[android] audio=false frame={}",
                ssx3_ps2::m!(app.cpu).sched.frames
            );
        }
        if rebuild_needed() {
            app.presenter = None;
        }
        if let Some(window) = &app.window {
            let size = window.inner_size();
            app.android.policy.surface = size.width > 0 && size.height > 0;
            if app.android.policy.surface && app.presenter.is_none() {
                app.presenter = Some(pollster::block_on(super::super::Presenter::new(
                    window.clone(),
                    app.gpu_ctx,
                )));
                app.follow_window_size(size.width, size.height);
                rebuilt();
            }
        }
        let bits = JAVA.load(Ordering::Acquire);
        let playing = app.android.policy.playing(bits);
        if playing != app.android.active {
            Self::neutralize(app);
            app.android.active = playing;
        }
        let audible = app.android.policy.audible(bits);
        if audible != app.android.audio {
            app.android.audio = audible;
            app.android.heard = false;
            audio::request(audible);
            eprintln!(
                "[android] audio={} frame={}",
                audible,
                ssx3_ps2::m!(app.cpu).sched.frames
            );
        }
        let ready = app.android.active;
        if ready != app.android.confirmed {
            app.android.confirmed = ready;
            java(if ready {
                JavaAction::Playing
            } else {
                JavaAction::Paused
            });
            eprintln!(
                "[android] playback={} frame={}",
                ready,
                ssx3_ps2::m!(app.cpu).sched.frames
            );
        }
        if audible && !app.android.heard && audio::callbacks_started() {
            app.android.heard = true;
            eprintln!(
                "[android] audio callbacks resumed frame={}",
                ssx3_ps2::m!(app.cpu).sched.frames
            );
        }
        event_loop.set_control_flow(if ready {
            ControlFlow::Poll
        } else {
            ControlFlow::Wait
        });
        ready
    }
}

pub fn refresh_rate() -> Option<f64> {
    ACTIVITY.with(|a| a.borrow().as_ref().and_then(super::display::refresh_rate))
}
