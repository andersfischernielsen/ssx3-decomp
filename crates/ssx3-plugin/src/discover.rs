//! Finding plugin folders and the disc images they need.

use std::path::{Path, PathBuf};

use ssx3_core::iso::{Disc, SECTOR};

use crate::image;
use crate::manifest::{self, DiscInput, Manifest};

/// `SLUS_207.72`, `slus-20772` and `SLUS-207.72` are the same serial.
pub fn normalize_serial(s: &str) -> String {
    s.trim()
        .chars()
        .filter(|&c| c != '.')
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_uppercase()
            }
        })
        .collect()
}

/// The boot serial of an image, from `SYSTEM.CNF` (`BOOT2 = cdrom0:\SLUS_207.72;1`).
pub fn boot_serial(disc: &mut Disc) -> Option<String> {
    let e = disc.find("SYSTEM.CNF")?;
    let data = disc.read_vec(e.sector, e.size.div_ceil(SECTOR as u32).max(1));
    let text = String::from_utf8_lossy(&data[..(e.size as usize).min(data.len())]).into_owned();
    let line = text
        .lines()
        .find(|l| l.trim_start().to_ascii_uppercase().starts_with("BOOT"))?;
    let path = line.split_once('=')?.1;
    let file = path.rsplit(['\\', '/', ':']).next()?;
    let name = file.split(';').next()?;
    (!name.is_empty()).then(|| normalize_serial(name))
}

/// The boot serial of the image at `path`, for the formats the plugin layer
/// reads itself (`.iso`, raw `.bin`: see [`crate::image`]).
pub fn disc_serial(path: &Path) -> Option<String> {
    boot_serial(&mut image::open(path)?)
}

/// The boot serial a plugin's own `detect` command prints for `path`.
fn detected_serial(detect: &[String], dir: &Path, path: &Path) -> Option<String> {
    let out = std::process::Command::new(detect.first()?)
        .args(&detect[1..])
        .arg(path)
        .current_dir(dir)
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let first = String::from_utf8_lossy(&out.stdout)
        .lines()
        .next()?
        .trim()
        .to_string();
    (!first.is_empty()).then(|| normalize_serial(&first))
}

/// An image in `dirs` whose boot serial is one of `serials`, for the formats
/// the plugin layer reads itself (the SSX 3 image).
pub fn find_disc_by_serial(dirs: &[PathBuf], serials: &[String]) -> Option<PathBuf> {
    find(dirs, serials, &[], None)
}

/// The image a plugin's `inputs.disc` asks for: the built-in readers for
/// `.iso` and `.bin`, and the plugin's `detect` command (run in `plugin_dir`)
/// for any other file with one of its `extensions`.
pub fn find_disc(dirs: &[PathBuf], input: &DiscInput, plugin_dir: &Path) -> Option<PathBuf> {
    find(
        dirs,
        &input.serials,
        &input.extensions,
        input.detect.as_deref().map(|d| (d, plugin_dir)),
    )
}

fn find(
    dirs: &[PathBuf],
    serials: &[String],
    extra_extensions: &[String],
    detect: Option<(&[String], &Path)>,
) -> Option<PathBuf> {
    let want: Vec<String> = serials.iter().map(|s| normalize_serial(s)).collect();
    let is_candidate = |p: &Path| {
        p.extension().and_then(|e| e.to_str()).is_some_and(|e| {
            ["iso", "bin", "img"]
                .iter()
                .copied()
                .chain(extra_extensions.iter().map(String::as_str))
                .any(|x| e.eq_ignore_ascii_case(x))
        })
    };
    for dir in dirs {
        let Ok(rd) = std::fs::read_dir(dir) else {
            continue;
        };
        let mut files: Vec<PathBuf> = rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| is_candidate(p))
            .collect();
        files.sort();
        for f in files {
            let serial = disc_serial(&f).or_else(|| {
                detect.and_then(|(cmd, plugin_dir)| detected_serial(cmd, plugin_dir, &f))
            });
            if serial.is_some_and(|s| want.contains(&s)) {
                return Some(f);
            }
        }
    }
    None
}

/// Where plugins and discs are looked for.
#[derive(Debug, Clone)]
pub struct Roots {
    /// Folders whose sub-folders are plugins (each with a `plugin.toml`).
    pub plugin_dirs: Vec<PathBuf>,
    /// Folders with disc images.
    pub disc_dirs: Vec<PathBuf>,
    /// Where derived content goes: `<cache>/plugins/<id>/`.
    pub cache: PathBuf,
}

/// Which plugins may load (`SSX3_PLUGINS`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    All,
    None,
    Only(Vec<String>),
}

impl Filter {
    /// Unset or empty: all. `0` or `off`: none. Else a comma-separated list.
    pub fn from_setting(v: Option<&str>) -> Filter {
        match v.map(str::trim) {
            None | Some("") => Filter::All,
            Some("0") | Some("off") | Some("none") => Filter::None,
            Some(list) => Filter::Only(
                list.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect(),
            ),
        }
    }

    pub fn allows(&self, id: &str) -> bool {
        match self {
            Filter::All => true,
            Filter::None => false,
            Filter::Only(ids) => ids.iter().any(|i| i == id),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    /// Its content is there: it loads.
    Ready,
    /// Its content is made from a disc that is there: run its `prepare`.
    NeedsPrepare,
    /// What it needs is not there (in words).
    MissingInput(String),
    /// The manifest is wrong (all problems).
    Invalid(Vec<String>),
    /// Turned off by `SSX3_PLUGINS`.
    Disabled,
}

#[derive(Debug, Clone)]
pub struct Found {
    pub dir: PathBuf,
    /// The manifest's id (the folder's name when the manifest is unusable).
    pub id: String,
    pub manifest: Option<Manifest>,
    /// Where its files are: the folder's `content`, or `<cache>/plugins/<id>`
    /// for content made by `prepare`.
    pub content: PathBuf,
    /// The disc image its `prepare` reads.
    pub disc: Option<PathBuf>,
    pub status: Status,
}

impl Found {
    /// Files that replace or extend the game disc's, by path.
    pub fn overlay(&self) -> PathBuf {
        self.content.join("overlay")
    }
}

fn content_ready(m: &Manifest, content: &Path) -> bool {
    if m.prepare.is_some() {
        content.join("ready").is_file()
    } else {
        content.is_dir()
    }
}

fn inspect(dir: &Path, roots: &Roots, filter: &Filter) -> Found {
    let folder = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let invalid = |errors: Vec<String>| Found {
        dir: dir.to_path_buf(),
        id: folder.clone(),
        manifest: None,
        content: dir.join("content"),
        disc: None,
        status: Status::Invalid(errors),
    };
    let text = match std::fs::read_to_string(dir.join("plugin.toml")) {
        Ok(t) => t,
        Err(e) => return invalid(vec![format!("plugin.toml: {e}")]),
    };
    let m = match manifest::parse(&text) {
        Ok(m) => m,
        Err(e) => return invalid(e),
    };
    let id = m.plugin.id.clone();
    let content = if m.prepare.is_some() {
        roots.cache.join("plugins").join(&id)
    } else {
        dir.join("content")
    };
    let disc = m
        .inputs
        .disc
        .as_ref()
        .and_then(|d| find_disc(&roots.disc_dirs, d, dir));
    let status = if !filter.allows(&id) {
        Status::Disabled
    } else if content_ready(&m, &content) {
        Status::Ready
    } else if m.prepare.is_some() {
        match (&m.inputs.disc, &disc) {
            (Some(d), None) => Status::MissingInput(format!(
                "a disc image with serial {} in one of: {}",
                d.serials.join(" or "),
                roots
                    .disc_dirs
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            _ => Status::NeedsPrepare,
        }
    } else {
        Status::MissingInput(format!("its content folder {}", content.display()))
    };
    Found {
        dir: dir.to_path_buf(),
        id,
        manifest: Some(m),
        content,
        disc,
        status,
    }
}

/// Every plugin folder under `roots.plugin_dirs`, in id order. A second plugin
/// with an id already taken is invalid.
pub fn discover(roots: &Roots, filter: &Filter) -> Vec<Found> {
    let mut found: Vec<Found> = Vec::new();
    for root in &roots.plugin_dirs {
        let Ok(rd) = std::fs::read_dir(root) else {
            continue;
        };
        let mut dirs: Vec<PathBuf> = rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.join("plugin.toml").is_file())
            .collect();
        dirs.sort();
        for dir in dirs {
            let mut f = inspect(&dir, roots, filter);
            if let Some(first) = found.iter().find(|o| o.id == f.id) {
                f.status = Status::Invalid(vec![format!(
                    "id {:?} is already used by {}",
                    f.id,
                    first.dir.display()
                )]);
                f.manifest = None;
            }
            found.push(f);
        }
    }
    found.sort_by(|a, b| a.id.cmp(&b.id).then(a.dir.cmp(&b.dir)));
    found
}

#[cfg(test)]
pub(crate) mod testutil {
    use std::path::{Path, PathBuf};

    /// A scratch folder that is removed at the end of the test.
    pub struct Tmp(pub PathBuf);

    impl Tmp {
        pub fn new(name: &str) -> Tmp {
            let p = std::env::temp_dir()
                .join(format!("ssx3-plugin-test-{}-{name}", std::process::id()));
            let _ = std::fs::remove_dir_all(&p);
            std::fs::create_dir_all(&p).unwrap();
            Tmp(p)
        }
        pub fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for Tmp {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// A tiny disc image whose `SYSTEM.CNF` boots `boot` (a file name such as
    /// `SLUS_203.26`): plain 2048-byte sectors, or raw MODE2/2352.
    pub fn fake_disc(boot: &str, raw: bool) -> Vec<u8> {
        let (stride, head) = if raw { (2352, 24) } else { (2048, 0) };
        let mut img = vec![0u8; stride * 21];
        let mut put = |sector: usize, off: usize, bytes: &[u8]| {
            let at = sector * stride + head + off;
            img[at..at + bytes.len()].copy_from_slice(bytes);
        };
        let cnf = format!("BOOT2 = cdrom0:\\{boot};1\nVER = 1.00\nVMODE = NTSC\n");
        // Primary volume descriptor (sector 16) with its root directory record.
        put(16, 0, &[1]);
        put(16, 1, b"CD001");
        let mut root = [0u8; 34];
        root[0] = 34;
        root[2..6].copy_from_slice(&18u32.to_le_bytes());
        root[10..14].copy_from_slice(&2048u32.to_le_bytes());
        root[25] = 2;
        root[32] = 1;
        put(16, 156, &root);
        // The root directory (sector 18): one file, SYSTEM.CNF;1 at sector 19.
        let name = b"SYSTEM.CNF;1";
        let mut rec = vec![0u8; 33 + name.len() + 1];
        rec[0] = rec.len() as u8;
        rec[2..6].copy_from_slice(&19u32.to_le_bytes());
        rec[10..14].copy_from_slice(&(cnf.len() as u32).to_le_bytes());
        rec[32] = name.len() as u8;
        rec[33..33 + name.len()].copy_from_slice(name);
        put(18, 0, &rec);
        put(19, 0, cnf.as_bytes());
        img
    }
}

#[cfg(test)]
mod tests {
    use super::testutil::*;
    use super::*;

    #[test]
    fn serials_are_normalised() {
        assert_eq!(normalize_serial("SLUS_207.72"), "SLUS-20772");
        assert_eq!(normalize_serial("slus-20326"), "SLUS-20326");
        assert_eq!(normalize_serial(" SLES_500.30 "), "SLES-50030");
    }

    #[test]
    fn the_serial_of_plain_and_raw_images() {
        let t = Tmp::new("serial");
        for (name, boot, raw) in [
            ("a.iso", "SLUS_212.78", false),
            ("b.bin", "SLES_500.30", true),
        ] {
            std::fs::write(t.path().join(name), fake_disc(boot, raw)).unwrap();
        }
        assert_eq!(
            disc_serial(&t.path().join("a.iso")).as_deref(),
            Some("SLUS-21278")
        );
        assert_eq!(
            disc_serial(&t.path().join("b.bin")).as_deref(),
            Some("SLES-50030")
        );
        let dirs = [t.path().to_path_buf()];
        assert_eq!(
            find_disc_by_serial(&dirs, &["sles-50030".to_string()]),
            Some(t.path().join("b.bin"))
        );
        assert_eq!(
            find_disc_by_serial(&dirs, &["SLUS-99999".to_string()]),
            None
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_plugin_reads_the_formats_the_engine_does_not() {
        let t = Tmp::new("detect");
        // A format nothing here reads; the plugin's detector knows it (here:
        // the serial is the file's first line).
        std::fs::write(t.path().join("game.chd"), "SLUS-12345\nrest of the image\n").unwrap();
        std::fs::write(t.path().join("other.chd"), "SLUS-99999\n").unwrap();
        std::fs::write(t.path().join("broken.chd"), "").unwrap();
        let dirs = [t.path().to_path_buf()];
        let input = |serial: &str, detect: bool, ext: bool| DiscInput {
            serials: vec![serial.to_string()],
            extensions: if ext { vec!["chd".to_string()] } else { vec![] },
            detect: detect.then(|| {
                vec![
                    "sh".to_string(),
                    "-c".to_string(),
                    "head -1 \"$0\"".to_string(),
                ]
            }),
        };
        let here = t.path();
        assert_eq!(
            find_disc(&dirs, &input("SLUS-12345", true, true), here),
            Some(t.path().join("game.chd"))
        );
        assert_eq!(
            find_disc(&dirs, &input("slus_123.45", true, true), here),
            Some(t.path().join("game.chd"))
        );
        assert_eq!(
            find_disc(&dirs, &input("SLUS-00000", true, true), here),
            None
        );
        // Without the command, or without the extension, the file is not looked at.
        assert_eq!(
            find_disc(&dirs, &input("SLUS-12345", false, true), here),
            None
        );
        assert_eq!(
            find_disc(&dirs, &input("SLUS-12345", true, false), here),
            None
        );
    }

    #[test]
    fn the_filter_reads_the_setting() {
        assert_eq!(Filter::from_setting(None), Filter::All);
        assert_eq!(Filter::from_setting(Some("")), Filter::All);
        assert_eq!(Filter::from_setting(Some("0")), Filter::None);
        let only = Filter::from_setting(Some("a, b"));
        assert!(only.allows("a") && only.allows("b") && !only.allows("c"));
    }

    fn manifest(id: &str, prepare: bool) -> String {
        let prep = if prepare {
            "[prepare]\ncommand = [\"true\"]\n"
        } else {
            ""
        };
        format!(
            "[plugin]\nid = \"{id}\"\napi = \"0.1\"\n[inputs]\ndisc = {{ serials = [\"SLUS-20326\"] }}\n{prep}"
        )
    }

    fn roots(t: &Tmp) -> Roots {
        Roots {
            plugin_dirs: vec![t.path().join("plugins")],
            disc_dirs: vec![t.path().join("discs")],
            cache: t.path().join("cache"),
        }
    }

    #[test]
    fn a_plugin_goes_from_missing_input_to_prepared() {
        let t = Tmp::new("states");
        let dir = t.path().join("plugins/tricky");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("plugin.toml"), manifest("tricky", true)).unwrap();
        std::fs::create_dir_all(t.path().join("discs")).unwrap();
        let r = roots(&t);

        let f = discover(&r, &Filter::All);
        assert!(
            matches!(f[0].status, Status::MissingInput(_)),
            "{:?}",
            f[0].status
        );

        std::fs::write(
            t.path().join("discs/tricky.iso"),
            fake_disc("SLUS_203.26", false),
        )
        .unwrap();
        let f = discover(&r, &Filter::All);
        assert_eq!(f[0].status, Status::NeedsPrepare);
        assert_eq!(f[0].disc, Some(t.path().join("discs/tricky.iso")));

        // Prepared: no disc needed any more.
        std::fs::remove_file(t.path().join("discs/tricky.iso")).unwrap();
        std::fs::create_dir_all(r.cache.join("plugins/tricky")).unwrap();
        std::fs::write(r.cache.join("plugins/tricky/ready"), "").unwrap();
        let f = discover(&r, &Filter::All);
        assert_eq!(f[0].status, Status::Ready);
        assert_eq!(f[0].overlay(), r.cache.join("plugins/tricky/overlay"));

        assert_eq!(discover(&r, &Filter::None)[0].status, Status::Disabled);
    }

    #[test]
    fn shipped_content_needs_only_its_folder() {
        let t = Tmp::new("shipped");
        let dir = t.path().join("plugins/pack");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("plugin.toml"), manifest("pack", false)).unwrap();
        let r = roots(&t);
        assert!(matches!(
            discover(&r, &Filter::All)[0].status,
            Status::MissingInput(_)
        ));
        std::fs::create_dir_all(dir.join("content")).unwrap();
        assert_eq!(discover(&r, &Filter::All)[0].status, Status::Ready);
    }

    #[test]
    fn bad_manifests_and_duplicate_ids_are_invalid() {
        let t = Tmp::new("invalid");
        for (folder, text) in [
            ("a", manifest("same", false)),
            ("b", manifest("same", false)),
            ("c", "not toml [".to_string()),
        ] {
            let d = t.path().join("plugins").join(folder);
            std::fs::create_dir_all(&d).unwrap();
            std::fs::write(d.join("plugin.toml"), text).unwrap();
        }
        let f = discover(&roots(&t), &Filter::All);
        let by_dir = |n: &str| f.iter().find(|x| x.dir.ends_with(n)).unwrap();
        assert!(matches!(by_dir("a").status, Status::MissingInput(_)));
        assert!(matches!(&by_dir("b").status, Status::Invalid(e) if e[0].contains("already used")));
        assert!(matches!(by_dir("c").status, Status::Invalid(_)));
        assert_eq!(by_dir("c").id, "c");
    }
}
