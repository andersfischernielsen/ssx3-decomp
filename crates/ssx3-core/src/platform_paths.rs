//! Default game-data and save locations. A frontend may set separate roots
//! once, before opening files. Per-file environment overrides still win.
//! Without roots, callers retain their checkout-relative or bundled defaults.
//! Paths are joined without probing or canonicalizing: relative paths use the
//! current working directory; unavailable roots do not select another location.

use std::path::{Path, PathBuf};
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
        std::env::var_os(key).map(PathBuf::from),
        ROOTS.get().map(|r| r.data.as_path()),
        name,
        default,
    )
}

/// Like `data_file`, but ignores non-UTF-8 overrides.
pub fn data_file_utf8(key: &str, name: &str, default: PathBuf) -> PathBuf {
    resolve(
        std::env::var(key).ok().map(PathBuf::from),
        ROOTS.get().map(|r| r.data.as_path()),
        name,
        default,
    )
}

/// A save file. `name` is relative to the save root, without a `saves/` prefix.
pub fn save_file(key: &str, name: &str, default: PathBuf) -> PathBuf {
    resolve(
        std::env::var_os(key).map(PathBuf::from),
        ROOTS.get().map(|r| r.saves.as_path()),
        name,
        default,
    )
}

/// `SSX3_MEMCARD` accepts UTF-8; empty and `none` select an empty slot.
/// Overrides are returned verbatim, including those sentinels.
pub fn memory_card() -> PathBuf {
    resolve(
        std::env::var("SSX3_MEMCARD").ok().map(PathBuf::from),
        ROOTS.get().map(|r| r.saves.as_path()),
        "ssx3-mc0.ps2",
        PathBuf::from("saves/ssx3-mc0.ps2"),
    )
}

/// An explicit pack selection is authoritative, even if it cannot be opened.
/// With no selection, `Pack::locate` retains its executable/bundle search.
pub fn pack_override() -> Option<PathBuf> {
    std::env::var_os("SSX3_PACK")
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

/// The explicit save root, else the frontend's platform-specific default.
pub fn standalone_saves(default: PathBuf) -> PathBuf {
    ROOTS.get().map_or(default, |r| r.saves.clone())
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
                    output.status.success(),
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
        assert_eq!(standalone_saves(default.clone()), saves);
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
