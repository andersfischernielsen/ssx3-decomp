//! `plugin.toml`: what a plugin folder says about itself.
//!
//! ```toml
//! [plugin]
//! id = "tricky"
//! api = "0.1"
//!
//! [inputs]
//! disc = { serials = ["SLUS-20326"] }
//!
//! [prepare]
//! command = ["python3", "prepare/prepare.py"]
//!
//! [[peak]]
//! id = "tricky"
//! name = "Peak Tricky"
//!
//! [[event]]
//! id = "garibaldi-race"
//! peak = "tricky"
//! kind = "race"
//! template = "peak1-race1"
//! world = "garibaldi"
//! ```
//!
//! Ids in a manifest are local to the plugin; the engine names them
//! `<plugin>/<id>` (see [`crate::catalog`]).

use std::collections::HashSet;

use serde::Deserialize;

use crate::API_VERSION;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub plugin: Info,
    #[serde(default)]
    pub inputs: Inputs,
    /// How the content is made from the user's disc (absent: the folder
    /// ships its content).
    pub prepare: Option<Prepare>,
    /// Rust code that is built into the front end (absent: data only).
    pub code: Option<Code>,
    #[serde(default, rename = "peak")]
    pub peaks: Vec<Peak>,
    #[serde(default, rename = "event")]
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Info {
    pub id: String,
    /// The plugin API version it was written for, `major.minor`.
    pub api: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Inputs {
    /// The disc image the content comes from.
    pub disc: Option<DiscInput>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiscInput {
    /// Boot serials that are accepted, such as `SLUS-20326` (any region of the
    /// game will do if listed).
    pub serials: Vec<String>,
    /// More file extensions to try (lower case, no dot) besides `iso`, `bin`
    /// and `img`.
    #[serde(default)]
    pub extensions: Vec<String>,
    /// For image formats the engine does not read (a multi-track `.cue`,
    /// `.chd`, `.nrg`, ...): a program and arguments, run in the plugin folder
    /// with an image's path appended. It prints the image's boot serial on its
    /// first line and exits 0, or exits non-zero when the file is not a disc
    /// it understands. Keep it quick: it is tried on every candidate file.
    pub detect: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Prepare {
    /// Program and arguments, run in the plugin folder (no shell). It reads
    /// `SSX3_PLUGIN_DISC`, `SSX3_BASE_DISC` and writes `SSX3_PLUGIN_OUT`, then
    /// creates `ready` there.
    pub command: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Code {
    /// The Cargo package of the plugin's Rust code. It exports
    /// `pub fn plugin() -> Box<dyn ssx3_plugin::Plugin>`.
    pub package: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Peak {
    pub id: String,
    pub name: String,
    /// The peak's picture, relative to the content folder.
    pub picture: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Race,
    Freestyle,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub id: String,
    pub name: Option<String>,
    /// Locale key of the event's name.
    pub name_key: Option<String>,
    /// The peak (local id) the event is listed under.
    pub peak: String,
    pub kind: Kind,
    /// The native event whose rules and flow the event takes.
    pub template: String,
    /// The location name the game loads: `data/worlds/<world>.big`, served
    /// from the plugin's overlay.
    pub world: String,
}

/// Parse and check a manifest; the errors are all of its problems.
pub fn parse(text: &str) -> Result<Manifest, Vec<String>> {
    let manifest: Manifest = toml::from_str(text).map_err(|e| vec![format!("plugin.toml: {e}")])?;
    let errors = manifest.problems();
    if errors.is_empty() {
        Ok(manifest)
    } else {
        Err(errors)
    }
}

/// `major.minor`.
pub fn parse_api(s: &str) -> Option<(u32, u32)> {
    let (a, b) = s.split_once('.')?;
    Some((a.parse().ok()?, b.parse().ok()?))
}

/// Before 1.0 a minor version is a breaking change: the plugin must name the
/// exact version of the engine.
pub fn api_compatible(plugin: (u32, u32)) -> bool {
    plugin == API_VERSION
}

fn id_ok(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 32
        && !s.starts_with('-')
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// A location name becomes a file name in a guest buffer: plain, and short.
fn world_ok(s: &str) -> bool {
    !s.is_empty() && s.len() <= 16 && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

impl Manifest {
    /// Every way the manifest cannot be used, in words for its author.
    pub fn problems(&self) -> Vec<String> {
        let mut e = Vec::new();
        if !id_ok(&self.plugin.id) {
            e.push(format!(
                "plugin id {:?}: lower-case letters, digits and '-', at most 32",
                self.plugin.id
            ));
        }
        match parse_api(&self.plugin.api) {
            Some(v) if api_compatible(v) => {}
            Some((a, b)) => e.push(format!(
                "plugin api {a}.{b}: this engine is {}.{} (before 1.0 the version must match)",
                API_VERSION.0, API_VERSION.1
            )),
            None => e.push(format!(
                "plugin api {:?}: expected major.minor",
                self.plugin.api
            )),
        }
        if let Some(d) = &self.inputs.disc {
            if d.serials.is_empty() {
                e.push("inputs.disc.serials: at least one serial".to_string());
            }
            if d.detect.as_ref().is_some_and(|c| c.is_empty()) {
                e.push("inputs.disc.detect: a program".to_string());
            }
            for x in &d.extensions {
                if x.is_empty()
                    || x.starts_with('.')
                    || x.chars().any(|c| !c.is_ascii_alphanumeric())
                {
                    e.push(format!(
                        "inputs.disc.extensions {x:?}: letters and digits, no dot"
                    ));
                }
            }
        }
        if self.prepare.as_ref().is_some_and(|p| p.command.is_empty()) {
            e.push("prepare.command: a program".to_string());
        }
        let mut peaks = HashSet::new();
        for p in &self.peaks {
            if !id_ok(&p.id) {
                e.push(format!(
                    "peak id {:?}: lower-case letters, digits and '-'",
                    p.id
                ));
            }
            if !peaks.insert(p.id.as_str()) {
                e.push(format!("peak {:?}: listed twice", p.id));
            }
        }
        let mut events = HashSet::new();
        for ev in &self.events {
            if !id_ok(&ev.id) {
                e.push(format!(
                    "event id {:?}: lower-case letters, digits and '-'",
                    ev.id
                ));
            }
            if !events.insert(ev.id.as_str()) {
                e.push(format!("event {:?}: listed twice", ev.id));
            }
            if !peaks.contains(ev.peak.as_str()) {
                e.push(format!(
                    "event {:?}: peak {:?} is not listed",
                    ev.id, ev.peak
                ));
            }
            if ev.template.is_empty() {
                e.push(format!("event {:?}: no template event", ev.id));
            }
            if !world_ok(&ev.world) {
                e.push(format!(
                    "event {:?}: world {:?} (letters, digits and '_', at most 16)",
                    ev.id, ev.world
                ));
            }
        }
        e
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD: &str = r#"
[plugin]
id = "tricky"
api = "0.1"

[inputs]
disc = { serials = ["SLUS-20326"] }

[prepare]
command = ["python3", "prepare/prepare.py"]

[[peak]]
id = "tricky"
name = "Peak Tricky"

[[event]]
id = "garibaldi-race"
peak = "tricky"
kind = "race"
template = "peak1-race1"
world = "garibaldi"
"#;

    #[test]
    fn a_good_manifest_parses() {
        let m = parse(GOOD).unwrap();
        assert_eq!(m.plugin.id, "tricky");
        assert_eq!(m.peaks.len(), 1);
        assert_eq!(m.events[0].kind, Kind::Race);
        assert_eq!(m.inputs.disc.unwrap().serials, ["SLUS-20326"]);
    }

    #[test]
    fn problems_name_what_is_wrong() {
        let bad = GOOD
            .replace("id = \"tricky\"\napi", "id = \"Tricky!\"\napi")
            .replace("0.1", "0.9")
            .replace("peak = \"tricky\"", "peak = \"nowhere\"")
            .replace("world = \"garibaldi\"", "world = \"../x\"");
        let e = parse(&bad).unwrap_err().join("\n");
        assert!(e.contains("plugin id"), "{e}");
        assert!(e.contains("plugin api 0.9"), "{e}");
        assert!(e.contains("peak \"nowhere\" is not listed"), "{e}");
        assert!(e.contains("world \"../x\""), "{e}");
    }

    #[test]
    fn the_disc_input_can_name_its_own_detector_and_extensions() {
        let ok = GOOD.replace(
            "disc = { serials = [\"SLUS-20326\"] }",
            "disc = { serials = [\"SLUS-20326\"], extensions = [\"chd\"], detect = [\"python3\", \"prepare/detect.py\"] }",
        );
        let d = parse(&ok).unwrap().inputs.disc.unwrap();
        assert_eq!(d.extensions, ["chd"]);
        assert_eq!(d.detect.unwrap()[1], "prepare/detect.py");
        let bad = GOOD.replace(
            "disc = { serials = [\"SLUS-20326\"] }",
            "disc = { serials = [\"SLUS-20326\"], extensions = [\".chd\"], detect = [] }",
        );
        let e = parse(&bad).unwrap_err().join("\n");
        assert!(e.contains("inputs.disc.detect"), "{e}");
        assert!(e.contains("no dot"), "{e}");
    }

    #[test]
    fn duplicates_and_unknown_fields_are_errors() {
        let dup = format!("{GOOD}\n[[peak]]\nid = \"tricky\"\nname = \"Again\"\n");
        assert!(parse(&dup).unwrap_err().join("").contains("listed twice"));
        let unknown = GOOD.replace("kind = \"race\"", "kind = \"race\"\nspeed = 3");
        assert!(parse(&unknown).is_err());
    }
}
