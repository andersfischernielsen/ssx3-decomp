//! Plugin peaks and events on the Select Peak screen (`docs/plans/plugin-peaks.md`).
//!
//! The plugins' catalog (`plugin.toml`: `[[peak]]`, `[[event]]`) becomes a
//! [`MenuPlan`] at boot: which rows the screen gets, what they are called in the
//! screen's data, and which event each row starts. The plan is pure data
//! (unit-tested here); the screen itself is patched by [`super::lui_patch`] and
//! the rows are driven by the hooks further down.
//!
//! Nothing here runs without a plugin that has a peak with an event: the plan
//! is empty, no file is generated and no hook is installed.

use std::collections::HashMap;
use std::path::PathBuf;

use ssx3_plugin::catalog::Catalog;
use ssx3_plugin::manifest::Kind;

use crate::game::hashvalue::str_hash;

/// Peaks that fit on the screen next to the three of the game (the list has no
/// scrolling: it is as tall as its rows).
pub const MAX_PEAKS: usize = 4;
/// Rows in one event list (22 lines each).
pub const MAX_ROWS: usize = 12;
/// Longest name shown, in ASCII characters.
pub const MAX_NAME: usize = 31;

/// What a row says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Label {
    /// A key of the game's locale table (`name_key`): hashed.
    Key(u32),
    /// Plain ASCII text.
    Ascii(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventPlan {
    /// `<plugin>/<id>`.
    pub id: String,
    pub label: Label,
    /// The native event whose rules it takes (`SSX3_EVENT=list`).
    pub template: String,
    /// The location name loaded for it (`data/worlds/<world>.big`).
    pub world: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PeakPlan {
    /// Position among the plugin peaks (0 based).
    pub index: usize,
    /// `<plugin>/<id>`.
    pub id: String,
    pub plugin: String,
    pub name: String,
    pub picture: Option<PathBuf>,
    pub race: Vec<EventPlan>,
    pub freestyle: Vec<EventPlan>,
}

/// A kind of event list: the goal tab of a peak (0 race, 1 freestyle).
pub const RACE: usize = 0;
pub const FREESTYLE: usize = 1;

impl PeakPlan {
    /// The events of a tab (0 race, 1 freestyle).
    pub fn events(&self, tab: usize) -> &[EventPlan] {
        if tab == FREESTYLE {
            &self.freestyle
        } else {
            &self.race
        }
    }

    /// The tabs that have events, in screen order.
    pub fn tabs(&self) -> Vec<usize> {
        [RACE, FREESTYLE]
            .into_iter()
            .filter(|t| !self.events(*t).is_empty())
            .collect()
    }
}

/// What a row of the patched screen is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Widget {
    /// A plugin peak's row in the peak list.
    Peak { peak: usize },
    /// The Race / Freestyle row of a peak's goal list.
    Goal { peak: usize, tab: usize },
    /// An event row.
    Event { peak: usize, tab: usize, row: usize },
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MenuPlan {
    pub peaks: Vec<PeakPlan>,
    /// What was dropped or changed, for the log.
    pub warnings: Vec<String>,
    by_hash: HashMap<u32, Widget>,
}

/// The widget names in the screen's data. Index based and ASCII, so plugins
/// cannot clash with each other or with the game.
pub mod names {
    pub fn peak(p: usize) -> String {
        format!("PlugPeak{p}")
    }
    pub fn goals(p: usize) -> String {
        format!("PlugGoals{p}")
    }
    pub fn goal(p: usize, tab: usize) -> String {
        format!("PlugGoal{p}{}", if tab == 0 { "Race" } else { "Free" })
    }
    pub fn locs(p: usize, tab: usize) -> String {
        format!("Plug{p}{}Locs", if tab == 0 { "Race" } else { "Free" })
    }
    pub fn event(p: usize, tab: usize, row: usize) -> String {
        format!("Plug{p}{}Loc{row}", if tab == 0 { "Race" } else { "Free" })
    }
}

/// Printable ASCII only, at most [`MAX_NAME`] characters: the text goes to the
/// game's ASCII string setter and must not carry anything it could mistake.
pub fn clean_name(s: &str) -> String {
    s.chars()
        .map(|ch| if (' '..='~').contains(&ch) { ch } else { '?' })
        .take(MAX_NAME)
        .collect::<String>()
        .trim()
        .to_string()
}

impl MenuPlan {
    /// Build the plan from the plugins' catalog: catalog order (plugin id, then
    /// manifest order), at most [`MAX_PEAKS`] peaks with at most [`MAX_ROWS`]
    /// events per list; a peak without events is not listed.
    pub fn build(catalog: &Catalog) -> MenuPlan {
        let mut plan = MenuPlan::default();
        for peak in &catalog.peaks {
            let mut race = Vec::new();
            let mut freestyle = Vec::new();
            for e in catalog.events_of(&peak.id) {
                let list = match e.kind {
                    Kind::Race => &mut race,
                    Kind::Freestyle => &mut freestyle,
                };
                if list.len() == MAX_ROWS {
                    plan.warnings.push(format!(
                        "plugin {}: event {} left out: a list holds {MAX_ROWS} events",
                        e.plugin, e.id
                    ));
                    continue;
                }
                let local = e.id.split_once('/').map_or(e.id.as_str(), |(_, l)| l);
                let label = match (&e.name_key, &e.name) {
                    (Some(k), _) => Label::Key(str_hash(k.as_bytes())),
                    (None, Some(n)) => Label::Ascii(clean_name(n)),
                    (None, None) => Label::Ascii(clean_name(local)),
                };
                list.push(EventPlan {
                    id: e.id.clone(),
                    label,
                    template: e.template.clone(),
                    world: e.world.clone(),
                });
            }
            if race.is_empty() && freestyle.is_empty() {
                plan.warnings.push(format!(
                    "plugin {}: peak {} has no events and is not listed",
                    peak.plugin, peak.id
                ));
                continue;
            }
            if plan.peaks.len() == MAX_PEAKS {
                plan.warnings.push(format!(
                    "plugin {}: peak {} left out: the screen holds {MAX_PEAKS} plugin peaks",
                    peak.plugin, peak.id
                ));
                continue;
            }
            plan.peaks.push(PeakPlan {
                index: plan.peaks.len(),
                id: peak.id.clone(),
                plugin: peak.plugin.clone(),
                name: clean_name(&peak.name),
                picture: peak.picture.clone(),
                race,
                freestyle,
            });
        }
        plan.index_widgets();
        plan
    }

    fn index_widgets(&mut self) {
        let mut map = HashMap::new();
        for p in &self.peaks {
            map.insert(
                str_hash(names::peak(p.index).as_bytes()),
                Widget::Peak { peak: p.index },
            );
            for tab in p.tabs() {
                map.insert(
                    str_hash(names::goal(p.index, tab).as_bytes()),
                    Widget::Goal {
                        peak: p.index,
                        tab,
                    },
                );
                for row in 0..p.events(tab).len() {
                    map.insert(
                        str_hash(names::event(p.index, tab, row).as_bytes()),
                        Widget::Event {
                            peak: p.index,
                            tab,
                            row,
                        },
                    );
                }
            }
        }
        self.by_hash = map;
    }

    pub fn is_empty(&self) -> bool {
        self.peaks.is_empty()
    }

    /// The row a widget name hash (`widget + 0x38`) is, if it is ours.
    pub fn widget(&self, name_hash: u32) -> Option<Widget> {
        self.by_hash.get(&name_hash).copied()
    }

    /// The event a row starts.
    pub fn event(&self, peak: usize, tab: usize, row: usize) -> Option<&EventPlan> {
        self.peaks.get(peak)?.events(tab).get(row)
    }
}

#[cfg(test)]
mod tests {
    use ssx3_plugin::catalog::{Event, Peak};

    use super::*;

    fn peak(plugin: &str, id: &str, name: &str) -> Peak {
        Peak {
            id: format!("{plugin}/{id}"),
            plugin: plugin.to_string(),
            name: name.to_string(),
            picture: None,
        }
    }

    fn event(plugin: &str, id: &str, peak: &str, kind: Kind) -> Event {
        Event {
            id: format!("{plugin}/{id}"),
            plugin: plugin.to_string(),
            name: None,
            name_key: None,
            peak: format!("{plugin}/{peak}"),
            kind,
            template: "ARA1".to_string(),
            world: format!("w{id}"),
        }
    }

    #[test]
    fn an_empty_catalog_gives_an_empty_plan() {
        let p = MenuPlan::build(&Catalog::default());
        assert!(p.is_empty() && p.warnings.is_empty());
        assert_eq!(p.widget(str_hash(b"PlugPeak0")), None);
    }

    #[test]
    fn peaks_keep_catalog_order_and_two_plugins_coexist() {
        let cat = Catalog {
            peaks: vec![peak("a", "one", "One"), peak("a", "two", "Two"), peak("b", "x", "X")],
            events: vec![
                event("a", "r1", "one", Kind::Race),
                event("a", "f1", "one", Kind::Freestyle),
                event("a", "r2", "one", Kind::Race),
                event("a", "r3", "two", Kind::Race),
                event("b", "r4", "x", Kind::Race),
            ],
            problems: vec![],
        };
        let p = MenuPlan::build(&cat);
        assert_eq!(p.peaks.len(), 3);
        assert_eq!(
            p.peaks.iter().map(|x| x.id.as_str()).collect::<Vec<_>>(),
            ["a/one", "a/two", "b/x"]
        );
        assert_eq!(p.peaks[0].race.len(), 2);
        assert_eq!(p.peaks[0].freestyle.len(), 1);
        assert_eq!(p.peaks[0].tabs(), [RACE, FREESTYLE]);
        assert_eq!(p.peaks[1].tabs(), [RACE]);
        assert_eq!(p.peaks[2].index, 2);
        // The widget map covers exactly the rows of the plan.
        assert_eq!(
            p.widget(str_hash(names::peak(2).as_bytes())),
            Some(Widget::Peak { peak: 2 })
        );
        assert_eq!(
            p.widget(str_hash(names::event(0, 0, 1).as_bytes())),
            Some(Widget::Event {
                peak: 0,
                tab: 0,
                row: 1
            })
        );
        assert_eq!(
            p.widget(str_hash(names::goal(0, 1).as_bytes())),
            Some(Widget::Goal { peak: 0, tab: 1 })
        );
        // No freestyle goal row for a peak without freestyle events.
        assert_eq!(p.widget(str_hash(names::goal(1, 1).as_bytes())), None);
        assert_eq!(p.event(0, 0, 1).unwrap().id, "a/r2");
        assert_eq!(p.event(0, 1, 0).unwrap().id, "a/f1");
        assert!(p.event(0, 1, 1).is_none() && p.event(9, 0, 0).is_none());
    }

    #[test]
    fn widget_names_do_not_collide() {
        let mut seen = HashMap::new();
        for p in 0..MAX_PEAKS {
            let mut all = vec![names::peak(p), names::goals(p)];
            for tab in [RACE, FREESTYLE] {
                all.push(names::goal(p, tab));
                all.push(names::locs(p, tab));
                all.extend((0..MAX_ROWS).map(|r| names::event(p, tab, r)));
            }
            for n in all {
                assert_eq!(seen.insert(str_hash(n.as_bytes()), n.clone()), None, "{n}");
            }
        }
    }

    #[test]
    fn caps_empty_peaks_and_names() {
        let mut cat = Catalog::default();
        cat.peaks.push(peak("a", "empty", "Empty"));
        for i in 0..6 {
            cat.peaks.push(peak("a", &format!("p{i}"), &"N".repeat(80)));
            for j in 0..14 {
                let mut e = event("a", &format!("e{i}-{j}"), &format!("p{i}"), Kind::Race);
                e.name = Some(format!("Name\u{e9}\u{0}{j}"));
                cat.events.push(e);
            }
        }
        let p = MenuPlan::build(&cat);
        assert_eq!(p.peaks.len(), MAX_PEAKS);
        assert!(p.peaks.iter().all(|x| x.race.len() == MAX_ROWS));
        assert!(p.peaks.iter().all(|x| x.name.len() == MAX_NAME));
        // 1 empty peak, 4 peaks x 2 surplus events, 2 surplus peaks x 2 surplus events (peaks past the
        // cap still count their rows first), 2 surplus peaks.
        assert!(p.warnings.iter().any(|w| w.contains("a/empty has no events")), "{:?}", p.warnings);
        assert!(p.warnings.iter().any(|w| w.contains("a/p4 left out")), "{:?}", p.warnings);
        assert!(p.warnings.iter().any(|w| w.contains("a/e0-12 left out")), "{:?}", p.warnings);
        // Non-ASCII and control characters become '?'.
        assert_eq!(
            p.peaks[0].race[0].label,
            Label::Ascii("Name??0".to_string())
        );
    }

    #[test]
    fn labels_prefer_the_locale_key_then_the_name_then_the_id() {
        let mut cat = Catalog {
            peaks: vec![peak("a", "p", "P")],
            events: vec![
                event("a", "keyed", "p", Kind::Race),
                event("a", "named", "p", Kind::Race),
                event("a", "plain", "p", Kind::Race),
            ],
            problems: vec![],
        };
        cat.events[0].name_key = Some("kT_240SelectPeak".to_string());
        cat.events[0].name = Some("ignored".to_string());
        cat.events[1].name = Some("Named".to_string());
        let p = MenuPlan::build(&cat);
        let l: Vec<_> = p.peaks[0].race.iter().map(|e| e.label.clone()).collect();
        assert_eq!(l[0], Label::Key(str_hash(b"kT_240SelectPeak")));
        assert_eq!(l[1], Label::Ascii("Named".to_string()));
        assert_eq!(l[2], Label::Ascii("plain".to_string()));
    }
}
