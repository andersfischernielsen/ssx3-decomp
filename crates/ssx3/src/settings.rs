//! Settings: what the `SSX3_*` environment variables choose, kept in
//! `saves/ssx3-settings.cfg` (`SSX3_SETTINGS=<path>` elsewhere) and edited in
//! the settings overlay (F10 in the window).
//!
//! The file holds `KEY = value` lines with the environment variables' names.
//! At startup it is read into the process environment for every key the
//! environment does not set itself, so each part of the program keeps
//! reading its variable. Settings the window applies while running (field
//! of view, aspect, resolution, interpolation, fullscreen) are read through
//! [`get`], which sees changes made in the overlay.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::RwLock;

/// One setting: its variable, label, the values offered (`""` = the
/// default, variable unset) and whether a change applies at once.
pub struct Item {
    pub key: &'static str,
    pub label: &'static str,
    pub choices: &'static [(&'static str, &'static str)],
    pub live: bool,
}

pub const ITEMS: &[Item] = &[
    Item { key: "SSX3_TICK_HZ", label: "Game rate", choices: &[("", "60 Hz"), ("120", "120 Hz (exp.)"), ("144", "144 Hz (exp.)"), ("240", "240 Hz (exp.)")], live: false },
    Item { key: "SSX3_FPS", label: "Display rate", choices: &[("", "Auto (display)"), ("60", "60 Hz"), ("120", "120 Hz"), ("180", "180 Hz"), ("240", "240 Hz")], live: true },
    Item { key: "SSX3_FOV", label: "Field of view", choices: &[("0.9", "-10%"), ("", "Standard"), ("1.1", "+10%"), ("1.2", "+20%"), ("1.3", "+30%"), ("1.4", "+40%"), ("1.5", "+50%")], live: true },
    Item { key: "SSX3_ASPECT", label: "Aspect ratio", choices: &[("", "Fill window"), ("16:9", "16:9"), ("4:3", "4:3 (original)")], live: true },
    Item { key: "SSX3_RESOLUTION", label: "Resolution", choices: &[("", "100%"), ("75", "75%"), ("50", "50%")], live: true },
    Item { key: "SSX3_ANISO", label: "Anisotropic filter", choices: &[("1", "Off"), ("2", "2x"), ("4", "4x"), ("8", "8x"), ("", "16x")], live: true },
    Item { key: "SSX3_FULLSCREEN", label: "Fullscreen", choices: &[("", "Off"), ("1", "On")], live: true },
    Item { key: "SSX3_RENDER", label: "Renderer", choices: &[("", "Native (wgpu)"), ("emu", "Emulated GS")], live: false },
    Item { key: "SSX3_SND", label: "Sound", choices: &[("", "Native mixer"), ("iop", "Emulated IOP")], live: false },
    Item { key: "SSX3_IOP_SIO2", label: "Pads & memory card", choices: &[("", "Native"), ("1", "Emulated IOP")], live: false },
    Item { key: "SSX3_NATIVE_FILES", label: "Game files", choices: &[("", "Native"), ("0", "Emulated CD")], live: false },
    Item { key: "SSX3_SKIP_MOVIES", label: "Intro movies", choices: &[("", "Play"), ("1", "Skip")], live: false },
];

struct State {
    values: BTreeMap<String, String>,
    /// Keys the environment set when the program started (they win over the
    /// file at the next start too).
    from_env: Vec<String>,
}

static STATE: RwLock<Option<State>> = RwLock::new(None);

fn path() -> PathBuf {
    std::env::var_os("SSX3_SETTINGS").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("saves/ssx3-settings.cfg"))
}

fn parse(text: &str) -> BTreeMap<String, String> {
    text.lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.starts_with('#') {
                return None;
            }
            let (k, v) = l.split_once('=')?;
            Some((k.trim().to_string(), v.trim().to_string()))
        })
        .filter(|(k, _)| ITEMS.iter().any(|i| i.key == k))
        .collect()
}

/// Read the settings file and put its values into the environment where
/// the environment has none. Call first thing in `main`, before any thread
/// starts.
pub fn load() {
    let mut values = std::fs::read_to_string(path()).map(|t| parse(&t)).unwrap_or_default();
    let mut from_env = Vec::new();
    for item in ITEMS {
        match std::env::var(item.key) {
            Ok(v) => {
                values.insert(item.key.to_string(), v);
                from_env.push(item.key.to_string());
            }
            Err(_) => {
                if let Some(v) = values.get(item.key).filter(|v| !v.is_empty()) {
                    // SAFETY (edition 2021: safe): no other thread runs yet.
                    std::env::set_var(item.key, v);
                }
            }
        }
    }
    *STATE.write().unwrap() = Some(State { values, from_env });
}

/// A setting's current value (`None`: the default).
pub fn get(key: &str) -> Option<String> {
    match STATE.read().unwrap().as_ref() {
        Some(s) => s.values.get(key).filter(|v| !v.is_empty()).cloned(),
        None => std::env::var(key).ok().filter(|v| !v.is_empty()),
    }
}

/// Whether the environment set `key` at startup.
pub fn from_env(key: &str) -> bool {
    STATE.read().unwrap().as_ref().is_some_and(|s| s.from_env.iter().any(|k| k == key))
}

/// Change a setting and save the file.
pub fn set(key: &str, value: &str) {
    let mut g = STATE.write().unwrap();
    let s = g.get_or_insert_with(|| State { values: BTreeMap::new(), from_env: Vec::new() });
    s.values.insert(key.to_string(), value.to_string());
    let mut text = String::from("# SSX 3 settings (edited by the settings overlay, F10). The SSX3_* environment\n# variables of the same names override these.\n");
    for item in ITEMS {
        if let Some(v) = s.values.get(item.key).filter(|v| !v.is_empty()) {
            text += &format!("{} = {v}\n", item.key);
        }
    }
    let p = path();
    if let Some(dir) = p.parent().filter(|d| !d.as_os_str().is_empty()) {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Err(e) = std::fs::write(&p, text) {
        eprintln!("[settings] cannot write {}: {e}", p.display());
    }
}

/// The index of a setting's current value among its choices.
pub fn choice(item: &Item) -> usize {
    let v = get(item.key).unwrap_or_default();
    item.choices.iter().position(|(c, _)| *c == v).unwrap_or_else(|| item.choices.iter().position(|(c, _)| c.is_empty()).unwrap_or(0))
}

/// Step a setting to the next (`+1`) or previous (`-1`) choice; returns its
/// new value.
pub fn step(item: &Item, dir: i32) -> String {
    let n = item.choices.len() as i32;
    let i = (choice(item) as i32 + dir).rem_euclid(n) as usize;
    let v = item.choices[i].0;
    set(item.key, v);
    v.to_string()
}

// ------------------------------------------------------------------ overlay

/// The overlay panel: rows of the settings plus "Close", `sel` highlighted.
/// RGBA8 words (`r | g << 8 | b << 16 | a << 24`), 1 pixel per font pixel
/// (the presenter scales it up).
pub fn draw(sel: usize, restart_pending: bool) -> (u32, u32, Vec<u32>) {
    const W: u32 = 320;
    const ROW: u32 = 12;
    let rows = ITEMS.len() as u32 + 1;
    let h = 28 + rows * ROW + 26;
    let mut img = vec![rgba(10, 20, 40, 225); (W * h) as usize];
    let fill = |img: &mut Vec<u32>, x0: u32, y0: u32, w: u32, hh: u32, c: u32| {
        for y in y0..(y0 + hh).min(h) {
            for x in x0..(x0 + w).min(W) {
                img[(y * W + x) as usize] = c;
            }
        }
    };
    let orange = rgba(255, 120, 20, 255);
    let white = rgba(235, 240, 250, 255);
    let grey = rgba(150, 165, 190, 255);
    // Frame and title bar.
    fill(&mut img, 0, 0, W, 2, orange);
    fill(&mut img, 0, h - 2, W, 2, orange);
    text(&mut img, W, 10, 9, "SETTINGS", orange);
    text(&mut img, W, W - 10 - 8 * 7, 9, "F10/ESC", grey);
    let mut y = 28;
    for (i, item) in ITEMS.iter().enumerate() {
        let selected = i == sel;
        if selected {
            fill(&mut img, 4, y - 2, W - 8, ROW, rgba(255, 120, 20, 90));
        }
        let c = if selected { white } else { grey };
        let label = if item.live { item.label.to_string() } else { format!("{}*", item.label) };
        text(&mut img, W, 10, y, &label, c);
        let mut value = item.choices[choice(item)].1.to_string();
        if from_env(item.key) {
            value += " (env)";
        }
        let vx = W - 10 - 8 * value.len() as u32;
        text(&mut img, W, vx, y, &value, if selected { orange } else { white });
        if selected {
            text(&mut img, W, vx - 12, y, "<", orange);
        }
        y += ROW;
    }
    let close = ITEMS.len() == sel;
    if close {
        fill(&mut img, 4, y - 2, W - 8, ROW, rgba(255, 120, 20, 90));
    }
    text(&mut img, W, 10, y, "Close", if close { white } else { grey });
    let note = if restart_pending { "* changed: applies after a restart" } else { "* applies after a restart" };
    text(&mut img, W, 10, h - 16, note, if restart_pending { orange } else { grey });
    (W, h, img)
}

fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    r as u32 | (g as u32) << 8 | (b as u32) << 16 | (a as u32) << 24
}

fn text(img: &mut [u32], w: u32, x: u32, y: u32, s: &str, c: u32) {
    use font8x8::UnicodeFonts;
    for (k, ch) in s.chars().enumerate() {
        let Some(glyph) = font8x8::BASIC_FONTS.get(ch) else { continue };
        for (gy, row) in glyph.iter().enumerate() {
            for gx in 0..8 {
                if row & (1 << gx) != 0 {
                    let (px, py) = (x + k as u32 * 8 + gx, y + gy as u32);
                    if px < w {
                        if let Some(p) = img.get_mut((py * w + px) as usize) {
                            *p = c;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_lines() {
        let m = parse("# c\nSSX3_FOV = 1.2\nSSX3_BOGUS=1\n SSX3_FPS=120 \n");
        assert_eq!(m.get("SSX3_FOV").map(String::as_str), Some("1.2"));
        assert_eq!(m.get("SSX3_FPS").map(String::as_str), Some("120"));
        assert!(!m.contains_key("SSX3_BOGUS"));
    }

    #[test]
    fn every_item_has_a_default() {
        for i in ITEMS {
            assert!(i.choices.iter().any(|(v, _)| v.is_empty()), "{}", i.key);
        }
    }
}
