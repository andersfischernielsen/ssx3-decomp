//! Pure lifecycle policy: surface, native focus and Java focus are independent.
//! Audio is optional: losing it silences the game but never pauses play.
use std::time::{Duration, Instant};

// Java bits: 1 foreground, 2 window focus, 4 audio focus, 8 destroyed.
#[derive(Default)]
pub struct Policy {
    pub native_focus: bool,
    pub surface: bool,
}
impl Policy {
    pub fn playing(&self, java: u32) -> bool {
        self.surface && self.native_focus && java & 11 == 3
    }
    pub fn audible(&self, java: u32) -> bool {
        self.playing(java) && java & 4 != 0
    }
}

// A failed stream is reopened after a backoff that doubles from half a
// second to eight seconds. A slow or stuck operation is left to finish:
// the owner handles one command at a time, and play does not wait for it.
#[derive(Default)]
pub struct AudioRetry {
    failures: u32,
    due: Option<Instant>,
}
impl AudioRetry {
    /// True when a reopen should be requested now.
    pub fn poll(&mut self, now: Instant, settled: bool, failed: bool, heard: bool) -> bool {
        if !settled {
            return false;
        }
        if !failed {
            self.due = None;
            if heard {
                self.failures = 0;
            }
            return false;
        }
        let failures = &mut self.failures;
        let due = *self.due.get_or_insert_with(|| {
            let delay = Duration::from_millis(500) * (1 << (*failures).min(4));
            *failures += 1;
            now + delay
        });
        if now < due {
            return false;
        }
        self.due = None;
        true
    }
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

// NativeActivity keys are consumed by winit before Java's Back callback.
// Android Back has a logical name but no mapped physical key in winit.
pub fn handle_back(
    key: &winit::keyboard::Key,
    state: winit::event::ElementState,
    repeat: bool,
    background: impl FnOnce(),
) -> bool {
    if *key != winit::keyboard::Key::Named(winit::keyboard::NamedKey::BrowserBack) {
        return false;
    }
    if state.is_pressed() && !repeat {
        background();
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::{
        event::ElementState,
        keyboard::{Key, NamedKey},
    };

    #[test]
    fn audio_focus_loss_of_any_kind_keeps_play_and_only_silences() {
        let p = Policy {
            native_focus: true,
            surface: true,
        };
        // Transient, duck and permanent loss clear only the audio bit.
        assert!(p.playing(7) && p.audible(7));
        assert!(p.playing(3) && !p.audible(3));
        // Gain needs no key: the audio bit alone brings sound back.
        assert!(p.playing(7) && p.audible(7));
    }

    #[test]
    fn slow_or_stuck_audio_never_requests_a_reopen() {
        let t = Instant::now();
        let mut audio = AudioRetry::default();
        for s in [0, 2, 5, 60] {
            let now = t + Duration::from_secs(s);
            assert!(!audio.poll(now, false, false, false));
            assert!(!audio.poll(now, false, true, false));
        }
        assert!(!audio.poll(t + Duration::from_secs(61), true, false, true));
    }

    #[test]
    fn stream_errors_reopen_with_a_capped_backoff_and_no_key() {
        let mut now = Instant::now();
        let mut audio = AudioRetry::default();
        for delay in [500, 1000, 2000, 4000, 8000, 8000] {
            assert!(!audio.poll(now, true, true, false)); // schedules
            let due = now + Duration::from_millis(delay);
            assert!(!audio.poll(due - Duration::from_millis(1), true, true, false));
            assert!(audio.poll(due, true, true, false));
            // The reopen is pending, then fails again.
            assert!(!audio.poll(due, false, false, false));
            now = due;
        }
        // A reopen that plays resets the backoff.
        assert!(!audio.poll(now, true, false, true));
        assert!(!audio.poll(now, true, true, false));
        assert!(audio.poll(now + Duration::from_millis(500), true, true, false));
    }

    #[test]
    fn a_stream_that_opens_but_never_plays_keeps_backing_off() {
        let t = Instant::now();
        let mut audio = AudioRetry::default();
        assert!(!audio.poll(t, true, true, false));
        assert!(audio.poll(t + Duration::from_millis(500), true, true, false));
        assert!(!audio.poll(t + Duration::from_millis(600), true, false, false));
        let t = t + Duration::from_millis(700);
        assert!(!audio.poll(t, true, true, false));
        assert!(audio.poll(t + Duration::from_secs(1), true, true, false));
        audio.reset();
        assert!(!audio.poll(t, true, true, false));
        assert!(audio.poll(t + Duration::from_millis(500), true, true, false));
    }

    #[test]
    fn logical_android_back_backgrounds_once_and_consumes_release_and_repeat() {
        let key = Key::Named(NamedKey::BrowserBack);
        let mut calls = 0;
        for (state, repeat) in [
            (ElementState::Pressed, false),
            (ElementState::Pressed, true),
            (ElementState::Released, false),
        ] {
            assert!(handle_back(&key, state, repeat, || calls += 1));
        }
        assert_eq!(calls, 1);
    }

    #[test]
    fn other_keys_remain_available_to_the_game() {
        for key in [Key::Named(NamedKey::Escape), Key::Character("x".into())] {
            assert!(!handle_back(&key, ElementState::Pressed, false, || {
                panic!("non-Back key requested backgrounding")
            }));
        }
    }

    #[test]
    fn every_window_loss_pauses_until_all_signals_return() {
        for native_focus in [false, true] {
            for surface in [false, true] {
                for java in 0..16 {
                    let p = Policy {
                        native_focus,
                        surface,
                    };
                    let play = native_focus && surface && java & 11 == 3;
                    assert_eq!(p.playing(java), play);
                    assert_eq!(p.audible(java), play && java & 4 != 0);
                }
            }
        }
    }
    #[test]
    fn zero_extent_then_valid_extent_recovers_without_focus_fabrication() {
        let mut p = Policy {
            native_focus: true,
            surface: true,
        };
        assert!(p.playing(7));
        p.surface = false;
        assert!(!p.playing(7));
        p.native_focus = false;
        p.surface = true;
        assert!(!p.playing(7));
        p.native_focus = true;
        assert!(p.playing(7));
        assert!(!p.playing(15)); // destruction cannot resume
    }
    #[test]
    fn audio_focus_gain_cannot_revive_a_paused_game() {
        let mut p = Policy {
            native_focus: true,
            surface: true,
        };
        assert!(!p.playing(6) && !p.audible(6)); // Java foreground absent
        assert!(!p.playing(5) && !p.audible(5)); // Java window focus absent
        p.surface = false;
        assert!(!p.audible(7)); // gain cannot revive a missing surface
        p.surface = true;
        p.native_focus = false;
        assert!(!p.audible(7)); // Java gain never fabricates native focus
        p.native_focus = true;
        assert!(p.playing(7) && p.audible(7));
    }
}
