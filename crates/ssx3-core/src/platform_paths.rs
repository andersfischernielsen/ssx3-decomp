//! Default game-data and save locations. A frontend may set separate roots
//! once, before opening files. Per-file overrides (`crate::config`: the
//! environment, then the front end's settings) still win.
//! Without roots, a standalone build keeps its saves in the user's data
//! folder, and other callers retain their checkout-relative defaults.
//! Paths are joined without probing or canonicalizing: relative paths use the
//! current working directory; unavailable roots do not select another location.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Roots {
    /// Game data and replacement textures; may be read-only.
    pub data: PathBuf,
    /// Memory card, settings and texture dumps; must be writable when used.
    pub saves: PathBuf,
}

static ROOTS: OnceLock<Roots> = OnceLock::new();

/// Set process-lifetime roots before standalone setup or any file consumer.
/// Readers do not wait for initialization; the frontend must order startup.
/// A second call returns the rejected roots, leaving the first selection intact.
pub fn set(roots: Roots) -> Result<(), Roots> {
    ROOTS.set(roots)
}

fn resolve(
    override_path: Option<PathBuf>,
    root: Option<&Path>,
    name: &str,
    default: PathBuf,
) -> PathBuf {
    override_path.unwrap_or_else(|| root.map_or(default, |r| r.join(name)))
}

/// A data path without an environment override.
pub fn data_path(name: &str, default: PathBuf) -> PathBuf {
    resolve(None, ROOTS.get().map(|r| r.data.as_path()), name, default)
}

/// An OS-string override, else a rooted name, else the desktop default.
pub fn data_file(key: &str, name: &str, default: PathBuf) -> PathBuf {
    resolve(
        crate::config::var_os(key).map(PathBuf::from),
        ROOTS.get().map(|r| r.data.as_path()),
        name,
        default,
    )
}

/// Like `data_file`, but ignores non-UTF-8 overrides.
pub fn data_file_utf8(key: &str, name: &str, default: PathBuf) -> PathBuf {
    resolve(
        crate::config::var(key).map(PathBuf::from),
        ROOTS.get().map(|r| r.data.as_path()),
        name,
        default,
    )
}

/// A save file, resolved when asked: the `key` override, else `name` in
/// [`save_dir`], else `default`. `name` has no `saves/` prefix.
pub fn save_file(key: &str, name: &str, default: PathBuf) -> PathBuf {
    resolve(
        crate::config::var_os(key).map(PathBuf::from),
        save_dir().as_deref(),
        name,
        default,
    )
}

/// `SSX3_MEMCARD` accepts UTF-8; empty and `none` select an empty slot.
/// Overrides are returned verbatim, including those sentinels.
pub fn memory_card() -> PathBuf {
    resolve(
        crate::config::var("SSX3_MEMCARD").map(PathBuf::from),
        save_dir().as_deref(),
        "ssx3-mc0.ps2",
        PathBuf::from("saves/ssx3-mc0.ps2"),
    )
}

/// An explicit pack selection is authoritative, even if it cannot be opened.
/// With no selection, `Pack::locate` retains its executable/bundle search.
pub fn pack_override() -> Option<PathBuf> {
    crate::config::var_os("SSX3_PACK")
        .map(PathBuf::from)
        .or_else(|| ROOTS.get().map(|r| r.data.join("ssx3.pack")))
}

/// The default dump directory: mutable output belongs with saves.
pub fn texture_dump() -> PathBuf {
    resolve(
        None,
        ROOTS.get().map(|r| r.saves.as_path()),
        "textures/dump",
        PathBuf::from("textures/dump"),
    )
}

pub fn texture_replacements() -> PathBuf {
    data_file_utf8(
        "SSX3_TEX_REPLACE",
        "textures/replace",
        PathBuf::from("textures/replace"),
    )
}

static STANDALONE: AtomicBool = AtomicBool::new(false);

/// A standalone build (a pack in use): without a save root, saves go to the
/// user's data folder ([`save_dir`]).
pub fn set_standalone() {
    STANDALONE.store(true, Ordering::Relaxed);
}

/// Whether [`set_standalone`] was called.
pub fn is_standalone() -> bool {
    STANDALONE.load(Ordering::Relaxed)
}

/// Where the memory card and settings go when no override names them,
/// looked up when asked: the save root, else for a standalone build the
/// user's data folder. None (the checkout's `saves/`) when neither is set
/// or the user's data folder cannot be found.
pub fn save_dir() -> Option<PathBuf> {
    match ROOTS.get() {
        Some(roots) => Some(roots.saves.clone()),
        None if is_standalone() => user_data_dir(),
        None => None,
    }
}

/// The OS family a per-user folder depends on.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Platform {
    Windows,
    Mac,
    AppleMobile,
    Android,
    Unix,
}

impl Platform {
    pub(crate) fn current() -> Self {
        if cfg!(target_os = "windows") {
            Self::Windows
        } else if cfg!(target_os = "macos") {
            Self::Mac
        } else if cfg!(target_vendor = "apple") {
            Self::AppleMobile
        } else if cfg!(target_os = "android") {
            Self::Android
        } else {
            Self::Unix
        }
    }
}

/// The user's data folder on `platform`; an empty or relative variable counts as unset.
fn user_data_dir_for(platform: Platform, env: impl Fn(&str) -> Option<PathBuf>) -> Option<PathBuf> {
    let usable = |key: &str| env(key).filter(|p| p.has_root());
    match platform {
        Platform::Windows => usable("APPDATA").map(|dir| dir.join("SSX3")),
        Platform::Mac => usable("HOME").map(|home| home.join("Library/Application Support/SSX3")),
        Platform::AppleMobile => usable("HOME").map(|home| home.join("Documents/saves")),
        Platform::Android => None,
        Platform::Unix => usable("XDG_DATA_HOME")
            .or_else(|| usable("HOME").map(|home| home.join(".local/share")))
            .map(|dir| dir.join("ssx3")),
    }
}

fn user_data_dir() -> Option<PathBuf> {
    user_data_dir_for(Platform::current(), |key| {
        std::env::var_os(key).map(PathBuf::from)
    })
}

static CACHE: OnceLock<PathBuf> = OnceLock::new();

/// Set the cache root once, before any consumer; a second call returns the
/// rejected root.
pub fn set_cache(root: PathBuf) -> Result<(), PathBuf> {
    CACHE.set(root)
}

/// The cache folder, if there is one: the front end's root, else the
/// platform's. None keeps nothing.
pub fn cache_dir() -> Option<PathBuf> {
    cache_dir_from(CACHE.get().map(PathBuf::as_path), platform_cache_dir)
}

fn cache_dir_from(
    cache: Option<&Path>,
    platform: impl FnOnce() -> Option<PathBuf>,
) -> Option<PathBuf> {
    cache.map(Path::to_path_buf).or_else(platform)
}

/// The platform's cache folder on `platform` (`Library/Caches/SSX3` on Apple
/// phones and tablets too); an empty or relative variable counts as unset.
/// None on Android, where only the front end knows it ([`set_cache`]).
fn platform_cache_dir_for(
    platform: Platform,
    env: impl Fn(&str) -> Option<PathBuf>,
) -> Option<PathBuf> {
    let usable = |key: &str| env(key).filter(|p| p.has_root());
    match platform {
        Platform::Windows => usable("LOCALAPPDATA").map(|dir| dir.join("SSX3")),
        Platform::Mac | Platform::AppleMobile => {
            usable("HOME").map(|home| home.join("Library/Caches/SSX3"))
        }
        Platform::Android => None,
        Platform::Unix => usable("XDG_CACHE_HOME")
            .or_else(|| usable("HOME").map(|home| home.join(".cache")))
            .map(|dir| dir.join("ssx3")),
    }
}

fn platform_cache_dir() -> Option<PathBuf> {
    platform_cache_dir_for(Platform::current(), |key| {
        std::env::var_os(key).map(PathBuf::from)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct Layout(PathBuf);
    impl Layout {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let p = std::env::temp_dir().join(format!(
                "ssx3-paths-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(&p).unwrap();
            Self(p)
        }
    }
    impl Drop for Layout {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }

    #[test]
    fn checkout_defaults_keep_their_original_bases() {
        let repo = Path::new("checkout");
        for (name, default) in [
            ("SSX 3 (USA).iso", repo.join("SSX 3 (USA).iso")),
            ("orig/SLUS_207.72", repo.join("orig/SLUS_207.72")),
            ("ssx3-mc0.ps2", PathBuf::from("saves/ssx3-mc0.ps2")),
            (
                "ssx3-settings.cfg",
                PathBuf::from("saves/ssx3-settings.cfg"),
            ),
            ("textures/replace", PathBuf::from("textures/replace")),
            ("textures/dump", PathBuf::from("textures/dump")),
        ] {
            assert_eq!(resolve(None, None, name, default.clone()), default);
        }
    }

    #[test]
    fn bundled_save_default_and_explicit_root() {
        for dir in [
            "APPDATA/SSX3",
            "Library/Application Support/SSX3",
            ".local/share/ssx3",
            "xdg/ssx3",
        ] {
            let dir = Path::new(dir);
            let default = dir.join("ssx3-mc0.ps2");
            assert_eq!(
                resolve(None, None, "ssx3-mc0.ps2", default.clone()),
                default
            );
            let saves = Path::new("app-saves");
            assert_eq!(
                resolve(None, Some(saves), "ssx3-mc0.ps2", default),
                saves.join("ssx3-mc0.ps2")
            );
        }
    }

    #[test]
    fn user_data_folders() {
        use Platform::*;
        let p = PathBuf::from;
        let all = |k: &str| match k {
            "APPDATA" => Some(p("/appdata")),
            "HOME" => Some(p("/home")),
            "XDG_DATA_HOME" => Some(p("/xdg")),
            _ => None,
        };
        let home_only = |k: &str| (k == "HOME").then(|| p("/home"));
        assert_eq!(user_data_dir_for(Windows, &all), Some(p("/appdata/SSX3")));
        assert_eq!(
            user_data_dir_for(Mac, &all),
            Some(p("/home/Library/Application Support/SSX3"))
        );
        assert_eq!(
            user_data_dir_for(AppleMobile, &all),
            Some(p("/home/Documents/saves"))
        );
        assert_eq!(user_data_dir_for(Unix, &all), Some(p("/xdg/ssx3")));
        assert_eq!(
            user_data_dir_for(Unix, &home_only),
            Some(p("/home/.local/share/ssx3"))
        );
        assert_eq!(user_data_dir_for(Android, &all), None);
        // Empty and relative values count as unset.
        for bad in ["", "relative"] {
            let odd = |k: &str| match k {
                "XDG_DATA_HOME" | "APPDATA" => Some(p(bad)),
                "HOME" => Some(p("/home")),
                _ => None,
            };
            assert_eq!(
                user_data_dir_for(Unix, &odd),
                Some(p("/home/.local/share/ssx3"))
            );
            assert_eq!(user_data_dir_for(Windows, &odd), None);
            let only = |_: &str| Some(p(bad));
            for platform in [Windows, Mac, AppleMobile, Unix] {
                assert_eq!(
                    user_data_dir_for(platform, &only),
                    None,
                    "{platform:?} {bad:?}"
                );
            }
        }
        for platform in [Windows, Mac, AppleMobile, Unix, Android] {
            assert_eq!(
                user_data_dir_for(platform, &|_: &str| None),
                None,
                "{platform:?}"
            );
        }
    }

    #[test]
    fn platform_cache_folders() {
        use Platform::*;
        let p = PathBuf::from;
        let all = |k: &str| match k {
            "LOCALAPPDATA" => Some(p("/local")),
            "HOME" => Some(p("/home")),
            "XDG_CACHE_HOME" => Some(p("/xdg")),
            _ => None,
        };
        let home_only = |k: &str| (k == "HOME").then(|| p("/home"));
        assert_eq!(
            platform_cache_dir_for(Windows, &all),
            Some(p("/local/SSX3"))
        );
        assert_eq!(
            platform_cache_dir_for(Mac, &all),
            Some(p("/home/Library/Caches/SSX3"))
        );
        assert_eq!(
            platform_cache_dir_for(AppleMobile, &all),
            Some(p("/home/Library/Caches/SSX3"))
        );
        assert_eq!(platform_cache_dir_for(Unix, &all), Some(p("/xdg/ssx3")));
        assert_eq!(
            platform_cache_dir_for(Unix, &home_only),
            Some(p("/home/.cache/ssx3"))
        );
        assert_eq!(platform_cache_dir_for(Android, &all), None);
        // A relative cache variable counts as unset (the home fallback wins).
        let odd = |k: &str| match k {
            "XDG_CACHE_HOME" => Some(p("relative")),
            "HOME" => Some(p("/home")),
            _ => None,
        };
        assert_eq!(
            platform_cache_dir_for(Unix, &odd),
            Some(p("/home/.cache/ssx3"))
        );
    }

    #[test]
    fn cache_dir_precedence() {
        use Platform::*;
        let p = PathBuf::from;
        let env = |k: &str| match k {
            "LOCALAPPDATA" => Some(p("/local")),
            "HOME" => Some(p("/home")),
            "XDG_CACHE_HOME" => Some(p("/xdg")),
            _ => None,
        };
        let from = |cache: Option<&Path>, platform: Platform| {
            cache_dir_from(cache, || platform_cache_dir_for(platform, &env))
        };
        // The front end's root first; else the platform's (Android: none).
        assert_eq!(from(Some(Path::new("c")), Unix), Some(p("c")));
        assert_eq!(from(None, Unix), Some(p("/xdg/ssx3")));
        assert_eq!(from(None, Android), None);
    }

    #[test]
    fn file_override_wins_without_becoming_root_relative() {
        for value in ["", "none", "relative/file", "/absolute/file"] {
            let value = PathBuf::from(value);
            assert_eq!(
                resolve(
                    Some(value.clone()),
                    Some(Path::new("app")),
                    "file",
                    PathBuf::from("default")
                ),
                value
            );
        }
    }

    #[test]
    fn unavailable_root_does_not_fall_back_or_create_anything() {
        let layout = Layout::new();
        let default = layout.0.join("existing-card");
        std::fs::write(&default, b"existing save").unwrap();
        let missing = layout.0.join("unavailable");
        let selected = resolve(None, Some(&missing), "ssx3-mc0.ps2", default.clone());
        assert_eq!(selected, missing.join("ssx3-mc0.ps2"));
        assert!(std::fs::read(selected).is_err());
        assert!(!missing.exists());
        assert_eq!(std::fs::read(default).unwrap(), b"existing save");
    }

    /// A test variable, with a non-UTF-8 byte appended in `non-utf8` mode.
    fn test_var(value: &str, mode: &str) -> std::ffi::OsString {
        #[cfg(unix)]
        if mode == "non-utf8" {
            use std::os::unix::ffi::OsStringExt;
            let mut bytes = value.as_bytes().to_vec();
            bytes.push(0xff);
            return std::ffi::OsString::from_vec(bytes);
        }
        let _ = mode;
        value.into()
    }

    #[test]
    fn standalone_saves_through_the_globals() {
        const CHILD: &str = "SSX3_PATHS_TEST_STANDALONE_CHILD";
        let key = "SSX3_PLATFORM_PATHS_TEST_SAVE";
        let Some(mode) = std::env::var_os(CHILD) else {
            // The flag and the roots are process-wide.
            for mode in ["utf8", "non-utf8"] {
                let output = std::process::Command::new(std::env::current_exe().unwrap())
                    .args([
                        "--exact",
                        "platform_paths::tests::standalone_saves_through_the_globals",
                        "--test-threads=1",
                    ])
                    .env(CHILD, mode)
                    .env("APPDATA", test_var("/sa-appdata", mode))
                    .env("HOME", test_var("/sa-home", mode))
                    .env("XDG_DATA_HOME", test_var("/sa-xdg", mode))
                    .env_remove("SSX3_MEMCARD")
                    .env_remove(key)
                    .output()
                    .unwrap();
                assert!(
                    output.status.success()
                        && String::from_utf8_lossy(&output.stdout).contains("ok. 1 passed;"),
                    "{mode}: {}\n{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            return;
        };
        let mode = mode.to_str().unwrap();
        let card = PathBuf::from("saves/ssx3-mc0.ps2");
        let settings = PathBuf::from("saves/ssx3-settings.cfg");
        let settings_file = || save_file(key, "ssx3-settings.cfg", settings.clone());
        // A checkout: no roots, no pack.
        assert_eq!(save_dir(), None);
        assert_eq!(memory_card(), card);
        assert_eq!(settings_file(), settings);
        // A desktop pack: the user's data folder, from the variables as given.
        let var = |value| PathBuf::from(test_var(value, mode));
        let user = match Platform::current() {
            Platform::Windows => Some(var("/sa-appdata").join("SSX3")),
            Platform::Mac => Some(var("/sa-home").join("Library/Application Support/SSX3")),
            Platform::AppleMobile => Some(var("/sa-home").join("Documents/saves")),
            Platform::Android => None,
            Platform::Unix => Some(var("/sa-xdg").join("ssx3")),
        };
        set_standalone();
        assert!(is_standalone());
        assert_eq!(save_dir(), user);
        let in_user =
            |name, default: &PathBuf| user.as_ref().map_or(default.clone(), |d| d.join(name));
        assert_eq!(memory_card(), in_user("ssx3-mc0.ps2", &card));
        assert_eq!(settings_file(), in_user("ssx3-settings.cfg", &settings));
        // A save root set later wins; an override wins over both.
        let saves = PathBuf::from("/front-end-saves");
        set(Roots {
            data: PathBuf::from("/front-end-data"),
            saves: saves.clone(),
        })
        .unwrap();
        assert_eq!(save_dir(), Some(saves.clone()));
        assert_eq!(memory_card(), saves.join("ssx3-mc0.ps2"));
        assert_eq!(settings_file(), saves.join("ssx3-settings.cfg"));
        std::env::set_var("SSX3_MEMCARD", "/explicit/card");
        std::env::set_var(key, "/explicit/settings");
        assert_eq!(memory_card(), Path::new("/explicit/card"));
        assert_eq!(settings_file(), Path::new("/explicit/settings"));
    }

    #[test]
    fn standalone_save_dir_without_a_usable_folder() {
        const CHILD: &str = "SSX3_PATHS_TEST_NO_FOLDER_CHILD";
        if std::env::var_os(CHILD).is_none() {
            // The flag is process-wide: check it in a child.
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "platform_paths::tests::standalone_save_dir_without_a_usable_folder",
                    "--test-threads=1",
                ])
                .env(CHILD, "1")
                .env("APPDATA", "")
                .env("HOME", "")
                .env("XDG_DATA_HOME", "relative")
                .output()
                .unwrap();
            assert!(
                output.status.success()
                    && String::from_utf8_lossy(&output.stdout).contains("ok. 1 passed;"),
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        // An empty `HOME`/`APPDATA` and a relative `XDG_DATA_HOME` count as
        // unset on every OS, so a standalone build falls back to `./saves`.
        set_standalone();
        assert!(is_standalone());
        assert_eq!(save_dir(), None);
    }

    #[test]
    fn cache_through_the_globals() {
        const CHILD: &str = "SSX3_PATHS_TEST_CACHE_CHILD";
        if std::env::var_os(CHILD).is_none() {
            // The flag and the roots are process-wide.
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "platform_paths::tests::cache_through_the_globals",
                    "--test-threads=1",
                ])
                .env(CHILD, "1")
                .env("HOME", "/standalone-home")
                .env("XDG_CACHE_HOME", "/standalone-xdg")
                .env("LOCALAPPDATA", "/standalone-local")
                .output()
                .unwrap();
            assert!(
                output.status.success()
                    && String::from_utf8_lossy(&output.stdout).contains("ok. 1 passed;"),
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        // The platform's folder, else the front end's.
        assert_eq!(cache_dir(), platform_cache_dir());
        set_cache(PathBuf::from("/front-end-cache")).unwrap();
        assert_eq!(cache_dir(), Some(PathBuf::from("/front-end-cache")));
    }

    #[test]
    fn frontend_roots_and_environment_overrides() {
        const CHILD: &str = "SSX3_PATHS_TEST_CHILD";
        if std::env::var_os(CHILD).is_none() {
            // OnceLock and environment changes must not leak into other tests.
            for mode in ["utf8", "non-utf8"] {
                let output = std::process::Command::new(std::env::current_exe().unwrap())
                    .args([
                        "--exact",
                        "platform_paths::tests::frontend_roots_and_environment_overrides",
                        "--test-threads=1",
                    ])
                    .env(CHILD, mode)
                    .env_remove("SSX3_PACK")
                    .env_remove("SSX3_MEMCARD")
                    .env_remove("SSX3_TEX_REPLACE")
                    .output()
                    .unwrap();
                assert!(
                    output.status.success()
                        && String::from_utf8_lossy(&output.stdout).contains("ok. 1 passed;"),
                    "{mode}: {}\n{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            assert!(ROOTS.get().is_none());
            return;
        }
        let layout = Layout::new();
        let default = layout.0.join("desktop");
        let key = "SSX3_PLATFORM_PATHS_TEST_FILE";
        std::env::remove_var(key);
        assert_eq!(data_file(key, "file", default.clone()), default);
        let mut base = layout.0.clone();
        #[cfg(unix)]
        if std::env::var(CHILD).unwrap() == "non-utf8" {
            use std::os::unix::ffi::OsStringExt;
            base.push(std::ffi::OsString::from_vec(vec![0xff]));
        }
        let data = base.join("app-data");
        let saves = base.join("app-saves");
        set(Roots {
            data: data.clone(),
            saves: saves.clone(),
        })
        .unwrap();
        let rejected = set(Roots {
            data: default.clone(),
            saves: default.clone(),
        })
        .unwrap_err();
        assert_eq!(rejected.data, default);
        assert_eq!(rejected.saves, default);
        assert_eq!(data_file(key, "file", default.clone()), data.join("file"));
        assert_eq!(save_file(key, "file", default.clone()), saves.join("file"));
        assert_eq!(save_dir(), Some(saves.clone()));
        let cache = base.join("app-cache");
        // Saves are not a cache fallback; a cache root set later wins.
        assert_eq!(cache_dir(), platform_cache_dir());
        set_cache(cache.clone()).unwrap();
        assert_eq!(set_cache(default.clone()).unwrap_err(), default);
        assert_eq!(cache_dir(), Some(cache.clone()));
        assert_eq!(texture_dump(), saves.join("textures/dump"));
        assert_eq!(texture_replacements(), data.join("textures/replace"));
        assert_eq!(memory_card(), saves.join("ssx3-mc0.ps2"));
        assert_eq!(
            data_path("orig/SLUS_207.72", default.clone()),
            data.join("orig/SLUS_207.72")
        );
        for value in ["", "none", "relative/card", "/absolute/card"] {
            std::env::set_var("SSX3_MEMCARD", value);
            assert_eq!(memory_card(), Path::new(value));
        }
        std::env::remove_var("SSX3_MEMCARD");
        for value in ["", "0", "relative/textures", "/absolute/textures"] {
            std::env::set_var("SSX3_TEX_REPLACE", value);
            assert_eq!(texture_replacements(), Path::new(value));
        }
        std::env::remove_var("SSX3_TEX_REPLACE");
        for value in ["", "none", "relative/file", "/absolute/file"] {
            std::env::set_var(key, value);
            assert_eq!(data_file(key, "file", default.clone()), Path::new(value));
            assert_eq!(
                data_file_utf8(key, "file", default.clone()),
                Path::new(value)
            );
            assert_eq!(save_file(key, "file", default.clone()), Path::new(value));
        }
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;
            let value = std::ffi::OsString::from_vec(vec![0xff]);
            std::env::set_var(key, &value);
            std::env::set_var("SSX3_MEMCARD", &value);
            std::env::set_var("SSX3_TEX_REPLACE", &value);
            assert_eq!(memory_card(), saves.join("ssx3-mc0.ps2"));
            assert_eq!(texture_replacements(), data.join("textures/replace"));
            assert_eq!(
                data_file(key, "file", default.clone()),
                PathBuf::from(&value)
            );
            assert_eq!(
                save_file(key, "file", default.clone()),
                PathBuf::from(&value)
            );
            assert_eq!(
                data_file_utf8(key, "file", default.clone()),
                data.join("file")
            );
        }
        std::env::remove_var(key);
        std::env::set_var("SSX3_PACK", &default);
        assert_eq!(pack_override(), Some(default.clone()));
        assert!(crate::pack::Pack::locate().is_none());
        std::env::remove_var("SSX3_PACK");
        // No pack at the selected root: do not search executable/bundle paths.
        assert_eq!(pack_override(), Some(data.join("ssx3.pack")));
        assert!(crate::pack::Pack::locate().is_none());
        // A valid synthetic pack opens only at the explicitly selected root.
        std::fs::create_dir_all(&data).unwrap();
        let mut header = Vec::from(*crate::pack::MAGIC);
        header.extend_from_slice(&1u32.to_le_bytes());
        header.extend_from_slice(&0u32.to_le_bytes());
        std::fs::write(data.join("ssx3.pack"), header).unwrap();
        assert_eq!(
            crate::pack::Pack::locate(),
            Some((data.join("ssx3.pack"), 0))
        );
        assert!(data.is_dir());
        assert!(!saves.exists());
    }

    #[test]
    fn relative_roots_and_symlinks_are_not_canonicalized() {
        let relative = Path::new("../app/./saves");
        assert_eq!(
            resolve(None, Some(relative), "card", PathBuf::from("default")),
            relative.join("card")
        );
        #[cfg(unix)]
        {
            let layout = Layout::new();
            let target = layout.0.join("storage");
            let link = layout.0.join("link");
            std::fs::create_dir(&target).unwrap();
            std::os::unix::fs::symlink(&target, &link).unwrap();
            let selected = resolve(None, Some(&link), "card", PathBuf::from("default"));
            assert_eq!(selected, link.join("card"));
            std::fs::write(target.join("card"), b"save").unwrap();
            assert_eq!(std::fs::read(selected).unwrap(), b"save");
        }
    }
}
