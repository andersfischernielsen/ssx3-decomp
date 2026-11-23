//! Plugins in the game (`ssx3-plugin`, `docs/plans/plugins.md`): the override
//! slots their hooks go in, and the peaks and events they add.

use std::sync::OnceLock;

use ssx3_ee_rt::funcs::GuestFn;
use ssx3_plugin::catalog::Catalog;
use ssx3_plugin::Slots;

/// The generated code's override slots, for plugin hooks: every ported
/// function, and the addresses of `hookable.txt`.
pub struct GameSlots;

impl Slots for GameSlots {
    fn get(&self, addr: u32) -> Option<GuestFn> {
        ssx3_ee::get_override(addr)
    }

    fn set(&mut self, addr: u32, f: GuestFn) -> bool {
        ssx3_ee::set_override(addr, f)
    }
}

static CATALOG: OnceLock<Catalog> = OnceLock::new();

/// The peaks and events the loaded plugins add. Set once, at boot, by the
/// engine; the first call wins.
pub fn set_catalog(c: Catalog) {
    let _ = CATALOG.set(c);
}

static OVERLAYS: OnceLock<Vec<std::path::PathBuf>> = OnceLock::new();

/// The overlay folders of the loaded plugins (`<content>/overlay`), in load
/// order. Set once, at boot, by the engine; the first call wins.
pub fn set_overlays(v: Vec<std::path::PathBuf>) {
    let _ = OVERLAYS.set(v);
}

/// The overlay folders of the loaded plugins; a plugin finds its derived
/// content (the importer's files) next to its own overlay folder.
pub fn overlays() -> &'static [std::path::PathBuf] {
    OVERLAYS.get().map(Vec::as_slice).unwrap_or(&[])
}

/// What a plugin hears about its events: [`add_event_listener`].
#[derive(Clone, Copy)]
pub struct EventListener {
    /// A plugin event was picked (Select Peak screen): its world and course
    /// name are set, the load comes next. `back` is the return address for
    /// guest calls, as in the game's own hooks.
    pub started: fn(&mut ssx3_ee_rt::Cpu, &ssx3_plugin::catalog::Event, u32) -> crate::call::Flow,
    /// The plugin event is over (the front end is back, or another event
    /// starts): the next world and names are the game's own.
    pub ended: fn(&mut ssx3_ee_rt::Cpu),
}

static LISTENERS: std::sync::Mutex<Vec<EventListener>> = std::sync::Mutex::new(Vec::new());

/// Be told when an event of the catalog is started from the menu and when it
/// ends (`SSX3_EVENT=<plugin>/<event>` boots do not call `started`: the
/// plugin knows its event from the variable). A plugin calls this from
/// `Plugin::install`.
pub fn add_event_listener(l: EventListener) {
    LISTENERS.lock().unwrap().push(l);
}

pub(crate) fn event_started(c: &mut ssx3_ee_rt::Cpu, event: &str, back: u32) -> crate::call::Flow {
    let Some(e) = catalog().event(event) else {
        return Ok(());
    };
    notify_started(c, e, back)
}

fn notify_started(
    c: &mut ssx3_ee_rt::Cpu,
    e: &ssx3_plugin::catalog::Event,
    back: u32,
) -> crate::call::Flow {
    let all: Vec<EventListener> = LISTENERS.lock().unwrap().clone();
    for l in all {
        (l.started)(c, e, back)?;
    }
    Ok(())
}

pub(crate) fn event_ended(c: &mut ssx3_ee_rt::Cpu) {
    let all: Vec<EventListener> = LISTENERS.lock().unwrap().clone();
    for l in all {
        (l.ended)(c);
    }
}

/// The plugins' catalog (empty without plugins).
pub fn catalog() -> &'static Catalog {
    CATALOG.get_or_init(Catalog::default)
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use ssx3_ee_rt::cpu::reg::{A0, V0};
    use ssx3_ee_rt::Cpu;
    use ssx3_plugin::hooks::{HookError, Hooks, Owners, Prev};

    use super::*;
    use crate::difftest::fresh;

    /// `CameraBoundObj_GetRotX` (0x176448): returns `this + 0xc`.
    const GET_ROT_X: u32 = 0x0017_6448;

    static PREV: Prev = Prev::new();
    static CALLS: AtomicUsize = AtomicUsize::new(0);

    thread_local! {
        /// The hook changes the result only on the thread of the test that set
        /// this: it stays in the process-wide slot, invisible to other tests.
        static ACTIVE: Cell<bool> = const { Cell::new(false) };
    }

    fn add_four(c: &mut Cpu) {
        PREV.call(c);
        if ACTIVE.get() {
            CALLS.fetch_add(1, Ordering::Relaxed);
            c.g[V0][0] = c.g[V0][0].wrapping_add(4);
        }
    }

    /// Run the function at `addr` the way a guest call does: through its slot.
    fn call(addr: u32, this: u32) -> u64 {
        let mut c = fresh();
        c.g[A0][0] = this as u64;
        let f = ssx3_ee_rt::funcs::lookup(addr).expect("function table");
        f(&mut c);
        c.g[V0][0]
    }

    #[test]
    fn a_plugin_hook_wraps_a_ported_function_through_its_real_slot() {
        let mut slots = GameSlots;
        let mut owners = Owners::default();
        assert_eq!(call(GET_ROT_X, 0x1000), 0x100c);
        Hooks::new(&mut slots, &mut owners, "test")
            .chain(GET_ROT_X, add_four, &PREV)
            .unwrap();
        ACTIVE.set(true);
        assert_eq!(
            call(GET_ROT_X, 0x1000),
            0x1010,
            "the hook ran after the port"
        );
        assert_eq!(CALLS.load(Ordering::Relaxed), 1);
        ACTIVE.set(false);
        assert_eq!(
            call(GET_ROT_X, 0x1000),
            0x100c,
            "transparent when not active"
        );
    }

    thread_local! {
        static HEARD: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
    }

    fn heard_started(
        _c: &mut Cpu,
        e: &ssx3_plugin::catalog::Event,
        back: u32,
    ) -> crate::call::Flow {
        HEARD.with(|h| h.borrow_mut().push(format!("start {} {back:#x}", e.id)));
        Ok(())
    }

    fn heard_ended(_c: &mut Cpu) {
        HEARD.with(|h| h.borrow_mut().push("end".into()));
    }

    #[test]
    fn event_listeners_hear_the_start_and_the_end() {
        add_event_listener(EventListener {
            started: heard_started,
            ended: heard_ended,
        });
        let event = ssx3_plugin::catalog::Event {
            id: "p/e".into(),
            plugin: "p".into(),
            name: Some("E".into()),
            name_key: None,
            peak: "p/k".into(),
            kind: ssx3_plugin::manifest::Kind::Race,
            template: "ARA1".into(),
            world: "W".into(),
        };
        let mut c = fresh();
        notify_started(&mut c, &event, 0x1234).unwrap();
        event_ended(&mut c);
        HEARD.with(|h| assert_eq!(*h.borrow(), ["start p/e 0x1234", "end"]));
    }

    #[test]
    fn hookable_txt_gives_an_unported_function_a_slot() {
        // 0x282a80 (ScriptEngine_FindCommandHandler) is listed in hookable.txt
        // and not ported; the game's entry point has no slot.
        let mut slots = GameSlots;
        assert!(slots.get(0x0028_2a80).is_some());
        assert!(slots.get(0x0010_0008).is_none());
        static P: Prev = Prev::new();
        let mut owners = Owners::default();
        let mut h = Hooks::new(&mut slots, &mut owners, "test");
        assert_eq!(
            h.chain(0x0010_0008, add_four, &P),
            Err(HookError::NoSlot(0x0010_0008))
        );
        // The slot starts out holding the generated body, which a hook can wrap.
        assert!(ssx3_ee::generated_body(0x0028_2a80).is_some());
    }
}
