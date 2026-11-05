//! Observe the actual Activity display; do not request a mode or resolution.
use jni::{jni_sig, jni_str, objects::JObject, JavaVM};
use winit::platform::android::activity::AndroidApp;
pub fn refresh_rate(app: &AndroidApp) -> Option<f64> {
    let vm = JavaVM::singleton().ok()?;
    let ptr: jni::sys::jobject = app.activity_as_ptr().cast();
    let hz = vm
        .attach_current_thread(|env| -> jni::errors::Result<f32> {
            let activity = unsafe { env.as_cast_raw::<JObject>(&ptr)? };
            let display = env
                .call_method(
                    &*activity,
                    jni_str!("getDisplay"),
                    jni_sig!("()Landroid/view/Display;"),
                    &[],
                )?
                .l()?;
            env.call_method(&display, jni_str!("getRefreshRate"), jni_sig!("()F"), &[])?
                .f()
        })
        .ok()?;
    (hz.is_finite() && hz > 0.0).then_some(f64::from(hz))
}
