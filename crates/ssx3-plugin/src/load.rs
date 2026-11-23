//! Loading plugins at boot: discover, check, collect what they add.

use std::path::PathBuf;

use crate::catalog::Catalog;
use crate::discover::{discover, Filter, Found, Roots, Status};
use crate::hooks::{Hooks, Owners, Slots};
use crate::plugin::Plugin;

/// What the engine gets from the plugins.
pub struct Loaded {
    /// Every plugin folder found, with its status (after the checks below).
    pub found: Vec<Found>,
    /// The extra peaks and events of the plugins that loaded.
    pub catalog: Catalog,
    /// Overlay directories of the plugins that loaded, in load order.
    pub overlays: Vec<PathBuf>,
    /// Plugins that loaded (folder and/or code), by id.
    pub active: Vec<String>,
    /// One line per plugin, for the log.
    pub log: Vec<String>,
}

/// Phase 1: discover and check the plugins; collect their overlays and the
/// catalog. Touches no override slot, so it can run before the game's own
/// installers (overlays must exist before the file layer starts). `code` are
/// the plugins linked into this front end: a manifest that declares Rust code
/// needs it there.
pub fn resolve(roots: &Roots, filter: &Filter, code: &[Box<dyn Plugin>]) -> Loaded {
    let mut found = discover(roots, filter);
    let mut log = Vec::new();
    let has_code = |id: &str| code.iter().any(|p| p.id() == id);

    // A plugin whose manifest declares code needs that code in this build.
    for f in &mut found {
        if f.status != Status::Ready {
            continue;
        }
        let wants = f.manifest.as_ref().is_some_and(|m| m.code.is_some());
        if wants && !has_code(&f.id) {
            f.status = Status::MissingInput(
                "its Rust code, which this build does not include (run `ssx3 plugins build`)"
                    .to_string(),
            );
        }
    }

    let mut active = Vec::new();
    let mut overlays = Vec::new();
    for f in &found {
        let line = match &f.status {
            Status::Ready => {
                active.push(f.id.clone());
                let overlay = f.overlay();
                if overlay.is_dir() {
                    overlays.push(overlay);
                }
                let n = f.manifest.as_ref().map_or(0, |m| m.events.len());
                format!("plugin {}: ready ({n} events)", f.id)
            }
            Status::NeedsPrepare => format!(
                "plugin {}: not prepared yet (run `ssx3 plugins prepare {}`)",
                f.id, f.id
            ),
            Status::MissingInput(what) => format!("plugin {}: needs {what}", f.id),
            Status::Invalid(errors) => {
                format!("plugin {}: ignored, {}", f.dir.display(), errors.join("; "))
            }
            Status::Disabled => format!("plugin {}: off (SSX3_PLUGINS)", f.id),
        };
        log.push(line);
    }

    let catalog = Catalog::from_found(
        &found
            .iter()
            .filter(|f| f.status == Status::Ready)
            .cloned()
            .collect::<Vec<_>>(),
    );
    log.extend(catalog.problems.iter().cloned());
    Loaded {
        found,
        catalog,
        overlays,
        active,
        log,
    }
}

/// Phase 2: run the code plugins' `install`. Call it after every installer of
/// the game itself (the file layer, the draw capture, the debug hooks): a hook
/// wraps what is in the slot, so it must come last or a later installer would
/// replace it. Plugins with a folder install only when it is ready.
pub fn install_hooks(
    loaded: &mut Loaded,
    filter: &Filter,
    code: &[Box<dyn Plugin>],
    slots: &mut dyn Slots,
) {
    let mut owners = Owners::default();
    for p in code {
        let id = p.id().to_string();
        let folder = loaded.found.iter().find(|f| f.id == id);
        let allowed = filter.allows(&id) && folder.is_none_or(|f| f.status == Status::Ready);
        if !allowed {
            continue;
        }
        let standalone = folder.is_none();
        let mut hooks = Hooks::new(slots, &mut owners, &id);
        match p.install(&mut hooks) {
            Ok(()) => {
                loaded
                    .log
                    .push(format!("plugin {id}: {} hook(s)", hooks.installed()));
                if standalone {
                    loaded.active.push(id);
                }
            }
            Err(e) => loaded.log.push(format!("plugin {id}: hook failed: {e}")),
        }
    }
}

/// Both phases at once, for callers that have no installers in between.
pub fn load(
    roots: &Roots,
    filter: &Filter,
    code: &[Box<dyn Plugin>],
    slots: &mut dyn Slots,
) -> Loaded {
    let mut loaded = resolve(roots, filter, code);
    install_hooks(&mut loaded, filter, code, slots);
    loaded
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::discover::testutil::Tmp;
    use crate::hooks::{GuestFn, HookError, Prev};

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

    fn original(_: &mut ssx3_ee_rt::Cpu) {}
    fn hooked(_: &mut ssx3_ee_rt::Cpu) {}
    static PREV: Prev = Prev::new();

    struct Hello(&'static str, u32);

    impl Plugin for Hello {
        fn id(&self) -> &str {
            self.0
        }
        fn install(&self, hooks: &mut Hooks<'_>) -> Result<(), HookError> {
            hooks.chain(self.1, hooked, &PREV)
        }
    }

    fn plugin(t: &Tmp, id: &str, extra: &str, content: bool) {
        let d = t.path().join("plugins").join(id);
        std::fs::create_dir_all(d.join("content/overlay")).unwrap();
        if !content {
            std::fs::remove_dir_all(d.join("content")).unwrap();
        }
        std::fs::write(
            d.join("plugin.toml"),
            format!(
                "[plugin]\nid = \"{id}\"\napi = \"0.1\"\n{extra}\n\
                 [[peak]]\nid = \"main\"\nname = \"Peak {id}\"\n\
                 [[event]]\nid = \"one\"\npeak = \"main\"\nkind = \"race\"\ntemplate = \"t\"\nworld = \"w\"\n"
            ),
        )
        .unwrap();
    }

    fn roots(t: &Tmp) -> Roots {
        Roots {
            plugin_dirs: vec![t.path().join("plugins")],
            disc_dirs: vec![],
            cache: t.path().join("cache"),
        }
    }

    #[test]
    fn ready_plugins_add_their_peaks_events_and_overlays() {
        let t = Tmp::new("load");
        plugin(&t, "alpha", "", true);
        plugin(&t, "beta", "", false);
        let mut slots = Table::default();
        let l = load(&roots(&t), &Filter::All, &[], &mut slots);
        assert_eq!(l.active, ["alpha"]);
        assert_eq!(l.catalog.peaks.len(), 1);
        assert_eq!(l.catalog.peaks[0].id, "alpha/main");
        assert_eq!(l.catalog.events[0].id, "alpha/one");
        assert_eq!(l.catalog.events[0].peak, "alpha/main");
        assert_eq!(l.catalog.events_of("alpha/main").count(), 1);
        assert_eq!(l.overlays, [t.path().join("plugins/alpha/content/overlay")]);
        assert!(
            l.log.iter().any(|s| s.contains("beta: needs")),
            "{:?}",
            l.log
        );
    }

    #[test]
    fn a_code_plugin_hooks_only_when_its_folder_is_ready() {
        let t = Tmp::new("code");
        plugin(&t, "alpha", "[code]\npackage = \"alpha-code\"\n", true);
        plugin(&t, "beta", "[code]\npackage = \"beta-code\"\n", false);
        let mut slots = Table::default();
        slots.0.insert(0x100, original);
        slots.0.insert(0x200, original);
        let code: Vec<Box<dyn Plugin>> = vec![
            Box::new(Hello("alpha", 0x100)),
            Box::new(Hello("beta", 0x200)),
            Box::new(Hello("loose", 0x300)),
        ];
        let l = load(&roots(&t), &Filter::All, &code, &mut slots);
        assert_eq!(
            slots.get(0x100).map(|f| f as usize),
            Some(hooked as GuestFn as usize)
        );
        assert_eq!(
            slots.get(0x200).map(|f| f as usize),
            Some(original as GuestFn as usize)
        );
        assert!(
            l.log.iter().any(|s| s.contains("alpha: 1 hook")),
            "{:?}",
            l.log
        );
        assert!(
            l.log
                .iter()
                .any(|s| s.contains("loose: hook failed") && s.contains("no override slot")),
            "{:?}",
            l.log
        );
        assert_eq!(l.active, ["alpha"]);
    }

    #[test]
    fn declared_code_that_is_not_linked_keeps_the_plugin_out() {
        let t = Tmp::new("nocode");
        plugin(&t, "alpha", "[code]\npackage = \"alpha-code\"\n", true);
        let l = load(&roots(&t), &Filter::All, &[], &mut Table::default());
        assert!(l.active.is_empty());
        assert!(l.catalog.events.is_empty());
        assert!(l.overlays.is_empty());
        assert!(l.log[0].contains("ssx3 plugins build"), "{:?}", l.log);
    }

    #[test]
    fn resolve_touches_no_slot_and_install_hooks_comes_after() {
        let t = Tmp::new("phases");
        plugin(&t, "alpha", "[code]\npackage = \"alpha-code\"\n", true);
        let mut slots = Table::default();
        slots.0.insert(0x100, original);
        let code: Vec<Box<dyn Plugin>> = vec![Box::new(Hello("alpha", 0x100))];
        let mut loaded = resolve(&roots(&t), &Filter::All, &code);
        assert_eq!(loaded.active, ["alpha"]);
        assert_eq!(
            loaded.overlays.len(),
            1,
            "overlays are known before any hook"
        );
        assert_eq!(
            slots.get(0x100).map(|f| f as usize),
            Some(original as GuestFn as usize)
        );
        // The game's own installers run here and replace slots...
        fn builtin(_: &mut ssx3_ee_rt::Cpu) {}
        slots.set(0x100, builtin);
        // ...and the plugin's hook, installed last, wraps what they left.
        install_hooks(&mut loaded, &Filter::All, &code, &mut slots);
        assert_eq!(
            slots.get(0x100).map(|f| f as usize),
            Some(hooked as GuestFn as usize)
        );
        assert_eq!(
            PREV.get().map(|f| f as usize),
            Some(builtin as GuestFn as usize)
        );
    }

    #[test]
    fn the_filter_turns_plugins_off() {
        let t = Tmp::new("filter");
        plugin(&t, "alpha", "", true);
        let l = load(&roots(&t), &Filter::None, &[], &mut Table::default());
        assert!(l.active.is_empty() && l.catalog.events.is_empty());
        assert!(l.log[0].contains("off"), "{:?}", l.log);
    }
}
