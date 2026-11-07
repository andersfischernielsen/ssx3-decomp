//! Pure lifecycle policy: surface, native focus and Java focus are independent.
//! Audio is optional: losing it silences the game but never pauses play.

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
