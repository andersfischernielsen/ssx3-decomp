//! Host tests of the real Android owner against a blocked fake backend.
use super::{Ring, RING_FRAMES};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Condvar, Mutex,
};
static OPEN: AtomicUsize = AtomicUsize::new(0);
static CLOSE: AtomicUsize = AtomicUsize::new(0);
static RELEASE: (Mutex<bool>, Condvar) = (Mutex::new(false), Condvar::new());
struct Stream;
impl Drop for Stream {
    fn drop(&mut self) {
        CLOSE.fetch_add(1, Ordering::SeqCst);
    }
}
fn open_stream(_: Ring) -> Result<Stream, String> {
    OPEN.fetch_add(1, Ordering::SeqCst);
    let (lock, signal) = &RELEASE;
    let mut released = lock.lock().unwrap();
    while !*released {
        released = signal.wait(released).unwrap();
    }
    Ok(Stream)
}
#[path = "android_lifecycle.rs"]
pub mod android_lifecycle;
#[test]
fn lazy_start_pause_during_open_resume_and_shutdown() {
    use android_lifecycle as a;
    let wait = |condition: &dyn Fn() -> bool| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
        while !condition() {
            assert!(
                std::time::Instant::now() < deadline,
                "fake owner failed to settle"
            );
            std::thread::yield_now();
        }
    };
    a::request(true);
    assert!(
        a::settled(),
        "lazy audio cannot prevent guest sound initialization"
    );
    let ring = a::start().unwrap();
    wait(&|| OPEN.load(Ordering::SeqCst) == 1);
    // open_stream is blocked. Dispatch must still return from pause.
    a::request(false);
    assert!(!a::settled());
    assert!(!a::accepts());
    ring.lock().unwrap().extend([1, 2, 3, 4]);
    *RELEASE.0.lock().unwrap() = true;
    RELEASE.1.notify_all();
    wait(&|| a::settled());
    assert!(!a::accepts());
    assert_eq!(CLOSE.load(Ordering::SeqCst), 1);
    assert!(ring.lock().unwrap().is_empty());
    a::request(true);
    wait(&|| a::settled());
    assert!(a::accepts());
    assert!(!a::callbacks_started());
    a::callback();
    assert!(a::callbacks_started());
    a::silence();
    assert!(!a::accepts());
    // A stream error only silences; the next request reopens the stream.
    a::request(true);
    wait(&|| a::settled());
    a::stream_error();
    assert!(a::failed() && !a::accepts());
    a::request(true);
    wait(&|| a::settled());
    assert!(!a::failed() && a::accepts());
    a::shutdown();
    wait(&|| a::settled());
    assert_eq!(CLOSE.load(Ordering::SeqCst), 4);
    assert!(!a::failed());
    assert!(!a::callbacks_started());
}
