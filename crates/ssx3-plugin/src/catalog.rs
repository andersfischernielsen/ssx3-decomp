//! The extra peaks and events plugins add, as data. Ids are `<plugin>/<id>`,
//! so plugins cannot clash; the game's own fixed tables are never indexed by a
//! plugin's id (the engine maps ids to slots).

use std::path::PathBuf;

use crate::discover::{Found, Status};
use crate::manifest::Kind;

#[derive(Debug, Clone, PartialEq)]
pub struct Peak {
    /// `<plugin>/<id>`.
    pub id: String,
    pub plugin: String,
    pub name: String,
    /// The picture, in the plugin's content folder.
    pub picture: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    /// `<plugin>/<id>`.
    pub id: String,
    pub plugin: String,
    pub name: Option<String>,
    pub name_key: Option<String>,
    /// The peak's full id.
    pub peak: String,
    pub kind: Kind,
    pub template: String,
    /// The location name the game loads (`data/worlds/<world>.big`).
    pub world: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Catalog {
    pub peaks: Vec<Peak>,
    pub events: Vec<Event>,
    /// What was left out and why (a world another plugin already uses): the
    /// engine logs these at boot.
    pub problems: Vec<String>,
}

impl Catalog {
    /// The peaks and events of the plugins that are ready, in plugin id order.
    pub fn from_found(found: &[Found]) -> Catalog {
        let mut c = Catalog::default();
        // The world name space is global (`data/worlds/<world>.big`): the first
        // plugin to use a name keeps it, a later plugin's events with it are
        // left out (two plugins could not both be served from the overlay).
        let mut worlds: std::collections::HashMap<String, String> = Default::default();
        for f in found.iter().filter(|f| f.status == Status::Ready) {
            let Some(m) = &f.manifest else { continue };
            let plugin = m.plugin.id.clone();
            let global = |id: &str| format!("{plugin}/{id}");
            for p in &m.peaks {
                c.peaks.push(Peak {
                    id: global(&p.id),
                    plugin: plugin.clone(),
                    name: p.name.clone(),
                    picture: p.picture.as_ref().map(|pic| f.content.join(pic)),
                });
            }
            for e in &m.events {
                let key = e.world.to_ascii_uppercase();
                match worlds.get(&key) {
                    Some(owner) if *owner != plugin => {
                        c.problems.push(format!(
                            "plugin {plugin}: event {:?} left out: world {:?} is already used by plugin {owner}",
                            e.id, e.world
                        ));
                        continue;
                    }
                    Some(_) => {}
                    None => {
                        worlds.insert(key, plugin.clone());
                    }
                }
                c.events.push(Event {
                    id: global(&e.id),
                    plugin: plugin.clone(),
                    name: e.name.clone(),
                    name_key: e.name_key.clone(),
                    peak: global(&e.peak),
                    kind: e.kind,
                    template: e.template.clone(),
                    world: e.world.clone(),
                });
            }
        }
        c
    }

    pub fn event(&self, id: &str) -> Option<&Event> {
        self.events.iter().find(|e| e.id == id)
    }

    /// The events listed under a peak.
    pub fn events_of<'a>(&'a self, peak: &'a str) -> impl Iterator<Item = &'a Event> + 'a {
        self.events.iter().filter(move |e| e.peak == peak)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{self, Manifest};

    fn found(id: &str, world: &str) -> Found {
        let text = format!(
            "[plugin]\nid = \"{id}\"\napi = \"0.1\"\n[[peak]]\nid = \"p\"\nname = \"P\"\n\
             [[event]]\nid = \"e\"\npeak = \"p\"\nkind = \"race\"\ntemplate = \"ARA1\"\nworld = \"{world}\"\n"
        );
        let m: Manifest = manifest::parse(&text).unwrap();
        Found {
            id: id.to_string(),
            dir: PathBuf::from(id),
            content: PathBuf::from(id).join("content"),
            manifest: Some(m),
            disc: None,
            status: Status::Ready,
        }
    }

    #[test]
    fn a_world_name_belongs_to_the_first_plugin_that_uses_it() {
        let c = Catalog::from_found(&[found("a", "Alps"), found("b", "ALPS"), found("c", "other")]);
        let ids: Vec<&str> = c.events.iter().map(|e| e.id.as_str()).collect();
        assert_eq!(ids, ["a/e", "c/e"]);
        assert_eq!(c.problems.len(), 1);
        assert!(c.problems[0].contains("plugin b") && c.problems[0].contains("plugin a"));
    }
}
