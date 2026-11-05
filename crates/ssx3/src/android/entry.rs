use winit::event_loop::EventLoop;
use winit::platform::android::{activity::AndroidApp, EventLoopBuilderExtAndroid};

#[no_mangle]
fn android_main(app: AndroidApp) {
    super::lifecycle::install(app.clone());
    let result = super::bootstrap::initialize(&app);
    let mut builder = EventLoop::builder();
    builder.with_android_app(app);
    let event_loop = builder.build().expect("Android event loop");
    match result {
        Ok(()) => {
            super::lifecycle::java(super::lifecycle::JavaAction::EnableAudio);
            super::super::run(Some(event_loop));
        }
        Err(message) => super::needs_data::run(event_loop, &message),
    }
    // Guest globals and worker ownership are process-lifetime. End only
    // this dedicated game process; the Java launcher waits for its death.
    extern "C" {
        fn _exit(status: std::ffi::c_int) -> !;
    }
    unsafe { _exit(0) }
}
