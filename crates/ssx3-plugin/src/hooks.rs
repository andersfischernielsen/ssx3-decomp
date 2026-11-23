//! Hooks: a plugin's Rust code wrapping or replacing game functions at their
//! override slots.
//!
//! A hook that wraps keeps the body that was in the slot and runs it through
//! its [`Prev`]:
//!
//! ```ignore
//! static PREV: Prev = Prev::new();
//!
//! fn wipeout_hook(c: &mut Cpu) {
//!     // before
//!     PREV.call(c); // what was in the slot: the built-in port or the game's code
//!     // after
//! }
//!
//! fn install(&self, hooks: &mut Hooks) -> Result<(), HookError> {
//!     hooks.chain(0x0012_3456, wipeout_hook, &PREV)
//! }
//! ```
//!
//! One `Prev` per hook and address (a `static`): hooks of several plugins on
//! one function run in load order, each through its own `Prev`.
//!
//! Only functions with an override slot can be hooked: every ported function,
//! and the addresses of `crates/ssx3-game/hookable.txt`.

use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use ssx3_ee_rt::Cpu;

pub use ssx3_ee_rt::funcs::GuestFn;

/// The override slots of the game, as the engine exposes them.
pub trait Slots {
    /// The function in the slot at `addr`; `None` without a slot.
    fn get(&self, addr: u32) -> Option<GuestFn>;
    /// Put `f` in the slot; false without a slot. Single-threaded startup only.
    fn set(&mut self, addr: u32, f: GuestFn) -> bool;
}

/// What a hook wraps: the function that was in the slot.
pub struct Prev(AtomicUsize);

impl Prev {
    pub const fn new() -> Prev {
        Prev(AtomicUsize::new(0))
    }

    /// Run the wrapped function. Panics when the hook was never chained.
    pub fn call(&self, c: &mut Cpu) {
        let f = self
            .get()
            .expect("Prev::call before Hooks::chain: this hook was never installed");
        f(c)
    }

    /// The wrapped function, once chained.
    pub fn get(&self) -> Option<GuestFn> {
        match self.0.load(Ordering::Relaxed) {
            0 => None,
            // SAFETY: `set` stored a `GuestFn`.
            p => Some(unsafe { std::mem::transmute::<usize, GuestFn>(p) }),
        }
    }

    /// For the host's own wrappers (not plugins): record the function a
    /// wrapper replaced.
    pub fn set_for_host(&self, f: GuestFn) {
        self.set(f)
    }

    fn set(&self, f: GuestFn) {
        self.0.store(f as usize, Ordering::Relaxed);
    }
}

impl Default for Prev {
    fn default() -> Self {
        Prev::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookError {
    /// The function has no override slot.
    NoSlot(u32),
    /// Another plugin replaced the function (not wrapped it).
    Conflict { addr: u32, other: String },
}

impl fmt::Display for HookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HookError::NoSlot(a) => write!(
                f,
                "{a:#010x} has no override slot (list it in crates/ssx3-game/hookable.txt)"
            ),
            HookError::Conflict { addr, other } => write!(
                f,
                "{addr:#010x} was already replaced by plugin {other:?}: only wrapping (chain) can share a function"
            ),
        }
    }
}

impl std::error::Error for HookError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Chain,
    Replace,
}

/// Who touched which slot, across all plugins of one boot.
#[derive(Default)]
pub struct Owners {
    map: HashMap<u32, Vec<(String, Mode)>>,
}

impl Owners {
    /// The plugins that touched `addr`, in order.
    pub fn of(&self, addr: u32) -> Vec<&str> {
        self.map
            .get(&addr)
            .map(|v| v.iter().map(|(p, _)| p.as_str()).collect())
            .unwrap_or_default()
    }
}

/// What a plugin's `install` is given.
pub struct Hooks<'a> {
    slots: &'a mut dyn Slots,
    owners: &'a mut Owners,
    plugin: &'a str,
    installed: usize,
}

impl<'a> Hooks<'a> {
    pub fn new(slots: &'a mut dyn Slots, owners: &'a mut Owners, plugin: &'a str) -> Self {
        Hooks {
            slots,
            owners,
            plugin,
            installed: 0,
        }
    }

    pub fn has_slot(&self, addr: u32) -> bool {
        self.slots.get(addr).is_some()
    }

    /// How many hooks this plugin has installed so far.
    pub fn installed(&self) -> usize {
        self.installed
    }

    /// Put `f` in the slot of the function at `addr`; `prev` runs what was
    /// there.
    pub fn chain(&mut self, addr: u32, f: GuestFn, prev: &Prev) -> Result<(), HookError> {
        let was = self.slots.get(addr).ok_or(HookError::NoSlot(addr))?;
        prev.set(was);
        assert!(self.slots.set(addr, f), "{addr:#x}: slot vanished");
        self.touch(addr, Mode::Chain);
        Ok(())
    }

    /// Put `f` in the slot and drop what was there. An error when another
    /// plugin already hooked the function.
    pub fn replace(&mut self, addr: u32, f: GuestFn) -> Result<(), HookError> {
        if self.slots.get(addr).is_none() {
            return Err(HookError::NoSlot(addr));
        }
        if let Some((other, _)) = self
            .owners
            .map
            .get(&addr)
            .and_then(|v| v.iter().find(|(p, _)| p != self.plugin))
        {
            return Err(HookError::Conflict {
                addr,
                other: other.clone(),
            });
        }
        assert!(self.slots.set(addr, f), "{addr:#x}: slot vanished");
        self.touch(addr, Mode::Replace);
        Ok(())
    }

    fn touch(&mut self, addr: u32, mode: Mode) {
        self.owners
            .map
            .entry(addr)
            .or_default()
            .push((self.plugin.to_string(), mode));
        self.installed += 1;
    }
}

#[cfg(test)]
mod tests {
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
    fn hook_a(_: &mut Cpu) {}
    fn hook_b(_: &mut Cpu) {}
    fn hook_c(_: &mut Cpu) {}

    fn is(slot: Option<GuestFn>, f: GuestFn) -> bool {
        slot.map(|s| s as usize) == Some(f as usize)
    }

    #[test]
    fn chained_hooks_stack_and_each_keeps_what_it_wrapped() {
        static A: Prev = Prev::new();
        static B: Prev = Prev::new();
        let mut t = Table::default();
        t.0.insert(0x100, original);
        let mut owners = Owners::default();
        Hooks::new(&mut t, &mut owners, "one")
            .chain(0x100, hook_a, &A)
            .unwrap();
        Hooks::new(&mut t, &mut owners, "two")
            .chain(0x100, hook_b, &B)
            .unwrap();
        assert!(is(t.get(0x100), hook_b));
        assert!(is(B.get(), hook_a), "the second wraps the first");
        assert!(is(A.get(), original), "the first wraps the original");
        assert_eq!(owners.of(0x100), ["one", "two"]);
    }

    #[test]
    fn a_missing_slot_is_an_error_not_a_panic() {
        static P: Prev = Prev::new();
        let mut t = Table::default();
        let mut owners = Owners::default();
        let mut h = Hooks::new(&mut t, &mut owners, "one");
        assert_eq!(h.chain(0x200, hook_a, &P), Err(HookError::NoSlot(0x200)));
        assert_eq!(h.replace(0x200, hook_a), Err(HookError::NoSlot(0x200)));
        assert!(!h.has_slot(0x200));
        assert!(P.get().is_none());
    }

    #[test]
    fn replacing_what_another_plugin_hooked_is_a_conflict() {
        static P: Prev = Prev::new();
        let mut t = Table::default();
        t.0.insert(0x100, original);
        let mut owners = Owners::default();
        Hooks::new(&mut t, &mut owners, "one")
            .chain(0x100, hook_a, &P)
            .unwrap();
        let err = Hooks::new(&mut t, &mut owners, "two")
            .replace(0x100, hook_b)
            .unwrap_err();
        assert_eq!(
            err,
            HookError::Conflict {
                addr: 0x100,
                other: "one".into()
            }
        );
        assert!(is(t.get(0x100), hook_a), "the slot is untouched");
        // The plugin itself may replace what it hooked; wrapping a replacement is fine.
        Hooks::new(&mut t, &mut owners, "one")
            .replace(0x100, hook_c)
            .unwrap();
    }

    #[test]
    fn replace_then_chain_wraps_the_replacement() {
        static P: Prev = Prev::new();
        let mut t = Table::default();
        t.0.insert(0x100, original);
        let mut owners = Owners::default();
        Hooks::new(&mut t, &mut owners, "one")
            .replace(0x100, hook_a)
            .unwrap();
        Hooks::new(&mut t, &mut owners, "two")
            .chain(0x100, hook_b, &P)
            .unwrap();
        assert!(is(P.get(), hook_a));
    }
}
