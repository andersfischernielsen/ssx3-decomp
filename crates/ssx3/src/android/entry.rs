use jni::{jni_sig, jni_str, objects::JObject, JavaVM};
use winit::event_loop::EventLoop;
use winit::platform::android::{activity::AndroidApp, EventLoopBuilderExtAndroid};

#[no_mangle]
fn android_main(app: AndroidApp) {
    // NativeActivity.onCreate cannot be skipped (its native handle is required).
    // Reject duplicates before touching ANY process-global guest/lifecycle state.
    // Returning lets stock android-activity finish just this rejected Activity.
    match owns_session(&app) {
        Ok(true) => {}
        result => {
            eprintln!("[android] rejected Activity session: {result:?}");
            return;
        }
    }
    let result = super::bootstrap::initialize(&app);
    let mut builder = EventLoop::builder();
    builder.with_android_app(app.clone());
    let event_loop = builder.build().expect("Android event loop");
    super::lifecycle::install(app, event_loop.create_proxy());
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

fn owns_session(app: &AndroidApp) -> jni::errors::Result<bool> {
    let ptr: jni::sys::jobject = app.activity_as_ptr().cast();
    JavaVM::singleton()?.attach_current_thread(|env| {
        let object = unsafe { env.as_cast_raw::<JObject>(&ptr)? };
        env.call_method(&*object, jni_str!("ownsSession"), jni_sig!("()Z"), &[])?
            .z()
    })
}
