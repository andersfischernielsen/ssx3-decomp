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

#[cfg(test)]
mod tests {
    use super::*;

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
}
