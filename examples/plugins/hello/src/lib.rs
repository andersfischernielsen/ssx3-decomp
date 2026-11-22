//! The example plugin: wraps `CameraBoundObj_GetRotX` (0x176448, returns
//! `this + 0xc`) and counts the calls. A real plugin would change what the
//! function returns, or do something before or after it.

use std::sync::atomic::{AtomicUsize, Ordering};

use ssx3_plugin::{Cpu, HookError, Hooks, Plugin, Prev};

/// `CameraBoundObj_GetRotX`: a ported leaf, so it has an override slot.
pub const GET_ROT_X: u32 = 0x0017_6448;

static PREV: Prev = Prev::new();

/// How often the game called the wrapped function.
pub static CALLS: AtomicUsize = AtomicUsize::new(0);

fn wrap(c: &mut Cpu) {
    CALLS.fetch_add(1, Ordering::Relaxed);
    PREV.call(c);
}

struct Hello;

impl Plugin for Hello {
    fn id(&self) -> &str {
        "hello"
    }

    fn install(&self, hooks: &mut Hooks<'_>) -> Result<(), HookError> {
        hooks.chain(GET_ROT_X, wrap, &PREV)
    }
}

/// What `ssx3 plugins build` links in.
pub fn plugin() -> Box<dyn Plugin> {
    Box::new(Hello)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ssx3_plugin::hooks::{GuestFn, Owners};
    use ssx3_plugin::Slots;

    use super::*;

    #[derive(Default)]
    struct Table(HashMap<u32, GuestFn>);

    impl Slots for Table {
        fn get(&self, addr: u32) -> Option<GuestFn> {
            self.0.get(&addr).copied()
        }
        fn set(&mut self, addr: u32, f: GuestFn) -> bool {
            self.0.insert(addr, f).is_some()
        }
    }

    fn original(_: &mut Cpu) {}

    #[test]
    fn it_wraps_the_function_it_names() {
        let mut slots = Table::default();
        slots.0.insert(GET_ROT_X, original);
        let mut owners = Owners::default();
        let mut hooks = Hooks::new(&mut slots, &mut owners, "hello");
        plugin().install(&mut hooks).unwrap();
        assert_eq!(hooks.installed(), 1);
        assert_eq!(
            slots.get(GET_ROT_X).map(|f| f as usize),
            Some(wrap as GuestFn as usize)
        );
        assert_eq!(
            PREV.get().map(|f| f as usize),
            Some(original as GuestFn as usize)
        );
    }

    #[test]
    fn the_manifest_is_valid() {
        let text =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/plugin.toml")).unwrap();
        let m = ssx3_plugin::manifest::parse(&text).unwrap();
        assert_eq!(m.plugin.id, plugin().id());
        assert_eq!(m.code.unwrap().package, "ssx3-plugin-hello");
    }
}
