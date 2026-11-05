use winit::event_loop::EventLoop;
use winit::platform::android::{activity::AndroidApp, EventLoopBuilderExtAndroid};

#[no_mangle]
fn android_main(app: AndroidApp) {
    let result = super::bootstrap::initialize(&app);
    let mut builder = EventLoop::builder();
    builder.with_android_app(app);
    let event_loop = builder.build().expect("Android event loop");
    match result {
        Ok(()) => super::super::run(Some(event_loop)),
        Err(message) => super::needs_data::run(event_loop, &message),
    }
}
