//! How the game is placed in the window, shared between the renderer (which
//! knows the window's size), the frame sink (which writes the projection the
//! game renders with) and the game code that lays out the screens: the view's
//! aspect and field of view, the squeeze of the 2D layer, and which screen
//! the game is showing.

use std::sync::atomic::{AtomicU32, Ordering};

static ASPECT: AtomicU32 = AtomicU32::new(0);
static FOV: AtomicU32 = AtomicU32::new(0x3f80_0000);
static HUD_SQUEEZE: AtomicU32 = AtomicU32::new(0x3f80_0000);

/// The view the game is rendered for: `aspect` (width / height; 0: the game's
/// own) and the field-of-view multiplier `fov` (1: the game's).
pub fn set_view(aspect: f32, fov: f32) {
    ASPECT.store(aspect.to_bits(), Ordering::Release);
    FOV.store(fov.to_bits(), Ordering::Release);
}

/// The aspect set with [`set_view`] (0: the game's own).
pub fn aspect() -> f32 {
    f32::from_bits(ASPECT.load(Ordering::Acquire))
}

/// The field-of-view multiplier set with [`set_view`] (1: the game's).
pub fn fov() -> f32 {
    let f = f32::from_bits(FOV.load(Ordering::Acquire));
    if f > 0.0 {
        f
    } else {
        1.0
    }
}

/// The renderer's squeeze of the 2D layer around the frame's centre (1: none;
/// below 1: the 4:3 layer is narrower than a wider window).
pub fn set_hud_squeeze(s: f32) {
    HUD_SQUEEZE.store(s.to_bits(), Ordering::Release);
}

/// The squeeze set with [`set_hud_squeeze`] (1 until a window sets it).
pub fn hud_squeeze() -> f32 {
    f32::from_bits(HUD_SQUEEZE.load(Ordering::Acquire))
}

/// What the game is showing, as far as the view cares.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    /// Menus, the intro and loading screens: authored for 4:3.
    FrontEnd,
    /// A race on screen (with its HUD): the view follows the window.
    Race,
    /// The pause menu over a race: 2D, authored for 4:3.
    Paused,
}

/// The screen being shown, from the game's memory (`ram_u32` reads a word of
/// EE RAM, `gp` is the game's global pointer).
///
/// A race is on screen when the race world (`gp - 0x848`) is non-null, its game
/// module (+0x84) holds the AI race manager (+0xc) and the manager's first
/// word, the race state, is 2 or more (2 game init, 4 countdown, 5 racing /
/// free ride). It is paused when the audio system's pause flags (`gp - 0x6a0`)
/// have bit 0 set. The front end's menu flow word (`gp - 0xb5c`) does not
/// change between the front end and a race and is no use here.
pub fn screen(ram_u32: impl Fn(u32) -> u32, gp: u32) -> Screen {
    if gp == 0 {
        return Screen::FrontEnd;
    }
    let world = ram_u32(gp.wrapping_sub(0x848));
    if world == 0 {
        return Screen::FrontEnd;
    }
    let module = ram_u32(world.wrapping_add(0x84));
    if module == 0 {
        return Screen::FrontEnd;
    }
    let ai = ram_u32(module.wrapping_add(0xc));
    if ai == 0 || ram_u32(ai) < 2 {
        return Screen::FrontEnd;
    }
    if ram_u32(gp.wrapping_sub(0x6a0)) & 1 != 0 {
        Screen::Paused
    } else {
        Screen::Race
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_follows_the_race_chain_and_the_pause_flag() {
        let gp = 0x5000;
        let mut m = std::collections::HashMap::new();
        let ram = |m: &std::collections::HashMap<u32, u32>| {
            let m = m.clone();
            move |a: u32| m.get(&a).copied().unwrap_or(0)
        };
        assert_eq!(screen(ram(&m), 0), Screen::FrontEnd);
        assert_eq!(screen(ram(&m), gp), Screen::FrontEnd);
        m.insert(gp - 0x848, 0x100);
        m.insert(0x100 + 0x84, 0x200);
        m.insert(0x200 + 0xc, 0x300);
        m.insert(0x300, 1);
        assert_eq!(screen(ram(&m), gp), Screen::FrontEnd);
        m.insert(0x300, 5);
        assert_eq!(screen(ram(&m), gp), Screen::Race);
        m.insert(gp - 0x6a0, 0xffff_ffdb);
        assert_eq!(screen(ram(&m), gp), Screen::Paused);
        m.insert(gp - 0x6a0, 0xffff_ffda);
        assert_eq!(screen(ram(&m), gp), Screen::Race);
    }
}
