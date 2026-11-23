//! Patching `DATA/UI/FE.LUI` in memory: rows for the plugin peaks and events on
//! the Event Select screen (`docs/plans/plugin-peaks.md`, 3.3).
//!
//! `FE.LUI` is `IYF\0`, a screen table and the screens, each RefPack
//! compressed. The Event Select screen (id 0x5380) is a header of 20 bytes,
//! the widget records (class u16, size u16, name hash u32 at +4, ...), a
//! 16-byte header and the layout records (same name hash at +4, y s16 at +18,
//! width u16 at +34, height u16 at +38), and then the screen's script and value
//! data, which is left as it is. Plugin rows are clones of the game's own rows
//! (the first peak, the first goal row, the first row of the race and freestyle
//! lists), so they take their class, font and colours from the game.
//!
//! The patched file is made on demand from the user's own disc and kept in
//! memory (`native_files::register_generator`): nothing derived from the game
//! is stored.

use crate::game::fe::pluginmenu::{names, MenuPlan, FREESTYLE, RACE};
use crate::game::hashvalue::str_hash;

/// The Event Select screen.
pub const EVENT_SELECT_SCREEN: u32 = 0x5380;
/// Row distance in the peak list.
const PEAK_STEP: i16 = 25;
/// Row distance in the goal and event lists.
const ROW_STEP: i16 = 22;
/// Width of a plugin row (names are up to 31 characters).
const ROW_WIDTH: u16 = 270;
/// Largest screen accepted (the game's own are about 80 KB).
const MAX_SCREEN: usize = 8 << 20;

type R<T> = Result<T, String>;

fn u16_at(d: &[u8], p: usize) -> R<u16> {
    d.get(p..p + 2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .ok_or_else(|| format!("truncated at {p:#x}"))
}

fn u32_at(d: &[u8], p: usize) -> R<u32> {
    d.get(p..p + 4)
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .ok_or_else(|| format!("truncated at {p:#x}"))
}

fn put_u16(d: &mut [u8], p: usize, v: u16) {
    d[p..p + 2].copy_from_slice(&v.to_le_bytes());
}

fn put_u32(d: &mut [u8], p: usize, v: u32) {
    d[p..p + 4].copy_from_slice(&v.to_le_bytes());
}

// ------------------------------------------------------------------ RefPack

/// Decode an EA RefPack stream (strict bounds: nothing is read or written past
/// the declared size).
pub fn refpack_decode(data: &[u8]) -> R<Vec<u8>> {
    if data.len() < 5 || data[1] != 0xfb {
        return Err("not a RefPack stream".into());
    }
    let size_bytes = if data[0] & 0x80 != 0 { 4 } else { 3 };
    let mut pos = 2;
    let mut expected = 0usize;
    for _ in 0..size_bytes {
        expected = expected << 8 | *data.get(pos).ok_or("truncated RefPack size")? as usize;
        pos += 1;
    }
    if data[0] & 1 != 0 {
        pos += size_bytes;
    }
    if expected > MAX_SCREEN {
        return Err(format!("RefPack output of {expected} bytes is too large"));
    }
    let mut out: Vec<u8> = Vec::with_capacity(expected);
    let take = |pos: &mut usize, n: usize| -> R<&[u8]> {
        let s = data
            .get(*pos..*pos + n)
            .ok_or("truncated RefPack command")?;
        *pos += n;
        Ok(s)
    };
    while out.len() < expected {
        let control = take(&mut pos, 1)?[0] as usize;
        let literal: usize;
        let (mut distance, mut run) = (0usize, 0usize);
        if control < 0x80 {
            let b = take(&mut pos, 1)?;
            literal = control & 3;
            distance = ((control >> 5 & 3) << 8) + b[0] as usize + 1;
            run = (control >> 2 & 7) + 3;
        } else if control < 0xc0 {
            let b = take(&mut pos, 2)?;
            literal = (b[0] >> 6) as usize;
            distance = (((b[0] & 0x3f) as usize) << 8) + b[1] as usize + 1;
            run = (control & 0x3f) + 4;
        } else if control < 0xe0 {
            let b = take(&mut pos, 3)?;
            literal = control & 3;
            distance = ((control & 0x10) << 12) + ((b[0] as usize) << 8) + b[1] as usize + 1;
            run = ((control & 0x0c) << 6) + b[2] as usize + 5;
        } else if control < 0xfc {
            literal = ((control & 0x1f) + 1) * 4;
        } else {
            literal = control & 3;
        }
        if out.len() + literal + run > expected {
            return Err("RefPack command exceeds the output size".into());
        }
        out.extend_from_slice(take(&mut pos, literal)?);
        if run > 0 {
            if distance > out.len() {
                return Err("RefPack back reference out of range".into());
            }
            for _ in 0..run {
                out.push(out[out.len() - distance]);
            }
        }
        if control >= 0xfc {
            break;
        }
    }
    if out.len() != expected {
        return Err("RefPack stream ended early".into());
    }
    Ok(out)
}

/// A RefPack stream of literal commands only (no back references): small and
/// auditable, a little bigger than the game's own.
pub fn refpack_encode_literals(data: &[u8]) -> Vec<u8> {
    assert!(data.len() < 1 << 24, "one RefPack block holds 16 MB");
    let mut out = vec![0x10, 0xfb];
    out.extend_from_slice(&(data.len() as u32).to_be_bytes()[1..]);
    let mut pos = 0;
    while data.len() - pos >= 4 {
        let n = ((data.len() - pos) & !3).min(112);
        out.push(0xe0 + (n / 4 - 1) as u8);
        out.extend_from_slice(&data[pos..pos + n]);
        pos += n;
    }
    out.push(0xfc + (data.len() - pos) as u8);
    out.extend_from_slice(&data[pos..]);
    out
}

// ------------------------------------------------------------------- screen

/// A decoded screen: widget records and layout records as byte rows, so rows
/// can be cloned and inserted and the offsets follow.
#[derive(Debug, Clone, PartialEq)]
pub struct Screen {
    header: [u8; 20],
    widgets: Vec<Vec<u8>>,
    props_header: [u8; 16],
    props: Vec<Vec<u8>>,
    /// Everything after the layout records.
    tail: Vec<u8>,
}

fn rows(d: &[u8], mut pos: usize, count: usize) -> R<(Vec<Vec<u8>>, usize)> {
    let mut out = Vec::with_capacity(count.min(4096));
    for _ in 0..count {
        let size = u16_at(d, pos + 2)? as usize;
        if size < 8 || pos + size > d.len() {
            return Err(format!("record at {pos:#x} has size {size}"));
        }
        out.push(d[pos..pos + size].to_vec());
        pos += size;
    }
    Ok((out, pos))
}

fn name_of(row: &[u8]) -> u32 {
    u32::from_le_bytes([row[4], row[5], row[6], row[7]])
}

fn find(rows: &[Vec<u8>], name: u32) -> R<usize> {
    rows.iter()
        .position(|r| name_of(r) == name)
        .ok_or_else(|| format!("no record {name:#x}"))
}

impl Screen {
    pub fn parse(d: &[u8]) -> R<Screen> {
        if u32_at(d, 0)? != EVENT_SELECT_SCREEN {
            return Err("not the Event Select screen".into());
        }
        let second = u32_at(d, 8)? as usize;
        let count = u32_at(d, 16)? as usize;
        let (widgets, end) = rows(d, 20, count)?;
        if end != second {
            return Err("widget table does not end where the layout table starts".into());
        }
        let props_header: [u8; 16] = d
            .get(second..second + 16)
            .ok_or("truncated layout header")?
            .try_into()
            .unwrap();
        let pcount = u16_at(&props_header, 14)? as usize;
        let rel = u32_at(&props_header, 8)? as usize;
        let (props, pend) = rows(d, second + 16, pcount)?;
        if pend != second + 4 + rel {
            return Err("layout table size does not match its header".into());
        }
        Ok(Screen {
            header: d[..20].try_into().unwrap(),
            widgets,
            props_header,
            props,
            tail: d[pend..].to_vec(),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let wlen: usize = self.widgets.iter().map(Vec::len).sum();
        let plen: usize = self.props.iter().map(Vec::len).sum();
        let mut header = self.header;
        put_u32(&mut header, 8, (20 + wlen) as u32);
        put_u32(&mut header, 16, self.widgets.len() as u32);
        let mut ph = self.props_header;
        put_u16(&mut ph, 14, self.props.len() as u16);
        put_u32(&mut ph, 8, (12 + plen) as u32);
        let mut out = Vec::with_capacity(20 + wlen + 16 + plen + self.tail.len());
        out.extend_from_slice(&header);
        self.widgets.iter().for_each(|w| out.extend_from_slice(w));
        out.extend_from_slice(&ph);
        self.props.iter().for_each(|p| out.extend_from_slice(p));
        out.extend_from_slice(&self.tail);
        out
    }

    /// Number of widget records.
    pub fn widget_count(&self) -> usize {
        self.widgets.len()
    }

    /// Whether a widget record with this name exists.
    pub fn has_widget(&self, name: u32) -> bool {
        find(&self.widgets, name).is_ok()
    }

    /// The child hashes of a menu or group record.
    pub fn children(&self, name: u32) -> R<Vec<u32>> {
        let w = &self.widgets[find(&self.widgets, name)?];
        let n = u32_at(w, 32)? as usize;
        (0..n).map(|k| u32_at(w, 36 + 4 * k)).collect()
    }

    /// A layout record's (y, width, height).
    pub fn layout(&self, name: u32) -> R<(i16, u16, u16)> {
        let p = &self.props[find(&self.props, name)?];
        Ok((u16_at(p, 18)? as i16, u16_at(p, 34)?, u16_at(p, 38)?))
    }
}

fn set_name(row: &mut [u8], name: &str) {
    put_u32(row, 4, str_hash(name.as_bytes()));
}

fn set_y(row: &mut [u8], y: i16) {
    row[18..20].copy_from_slice(&y.to_le_bytes());
}

/// Clone a menu (`root`) whose first child is `template_child`, as a menu of
/// `children` (names), inserting the new rows and the menu before `parent` and
/// linking the menu into the parent group. Layout rows follow the game's: the
/// children one below the other, the menu as tall as its rows.
fn add_list(s: &mut Screen, root: u32, menu: &str, children: &[String], parent: u32) -> R<()> {
    let ri = find(&s.widgets, root)?;
    let template_child = u32_at(&s.widgets[ri], 36)?;
    let child_w = s.widgets[find(&s.widgets, template_child)?].clone();
    let child_p = s.props[find(&s.props, template_child)?].clone();
    let root_p = s.props[find(&s.props, root)?].clone();
    let mut new_rows = Vec::new();
    for (i, name) in children.iter().enumerate() {
        let mut w = child_w.clone();
        set_name(&mut w, name);
        new_rows.push(w);
        let mut p = child_p.clone();
        set_name(&mut p, name);
        set_y(&mut p, i as i16 * ROW_STEP);
        put_u16(&mut p, 34, ROW_WIDTH);
        s.props.push(p);
    }
    let mut m = s.widgets[ri][..36].to_vec();
    for name in children {
        m.extend_from_slice(&str_hash(name.as_bytes()).to_le_bytes());
    }
    let mlen = m.len() as u16;
    put_u16(&mut m, 2, mlen);
    set_name(&mut m, menu);
    put_u32(&mut m, 32, children.len() as u32);
    new_rows.push(m);
    let mut p = root_p;
    set_name(&mut p, menu);
    let h = u16_at(&p, 38)?.max(children.len() as u16 * ROW_STEP as u16);
    put_u16(&mut p, 38, h);
    s.props.push(p);

    // The container drives visibility and input: the new rows and menu are
    // created before it, and it lists the menu.
    let pi = find(&s.widgets, parent)?;
    if u16_at(&s.widgets[pi], 0)? != 0x10 {
        return Err("a panel container is not a group".into());
    }
    {
        let g = &mut s.widgets[pi];
        let n = u32_at(g, 32)?;
        put_u32(g, 32, n + 1);
        g.extend_from_slice(&str_hash(menu.as_bytes()).to_le_bytes());
        let len = g.len() as u16;
        put_u16(g, 2, len);
    }
    s.widgets.splice(pi..pi, new_rows);
    Ok(())
}

/// Add the plan's rows to the screen: a peak row per plugin peak after Peak1
/// (the Mountain row moves down), and per peak a goal list and an event list
/// per kind.
pub fn add_peak_rows(s: &mut Screen, plan: &MenuPlan) -> R<()> {
    if plan.is_empty() {
        return Ok(());
    }
    let h = |n: &str| str_hash(n.as_bytes());
    let (peak1, mountain, list) = (h("Peak1"), h("Mountain"), h("SP_PeakList"));
    let n = plan.peaks.len();

    // Peak rows: clones of Peak1, after it.
    let at = find(&s.widgets, peak1)?;
    let template = s.widgets[at].clone();
    let rows: Vec<Vec<u8>> = plan
        .peaks
        .iter()
        .map(|p| {
            let mut w = template.clone();
            set_name(&mut w, &names::peak(p.index));
            w
        })
        .collect();
    s.widgets.splice(at + 1..at + 1, rows);

    let li = find(&s.widgets, list)?;
    {
        let l = &mut s.widgets[li];
        if u16_at(l, 0)? != 0x13 {
            return Err("the peak list is not a menu".into());
        }
        let count = u32_at(l, 32)? as usize;
        let pos = (0..count)
            .find(|k| u32_at(l, 36 + 4 * k).ok() == Some(peak1))
            .ok_or("Peak1 is not in the peak list")?;
        let mut at = 36 + 4 * (pos + 1);
        for p in &plan.peaks {
            let hash = str_hash(names::peak(p.index).as_bytes());
            l.splice(at..at, hash.to_le_bytes());
            at += 4;
        }
        put_u32(l, 32, (count + n) as u32);
        let len = l.len() as u16;
        put_u16(l, 2, len);
    }

    // Layout: the new rows continue Peak1's spacing, Mountain follows them.
    let pi = find(&s.props, peak1)?;
    let (y1, _, _) = s.layout(peak1)?;
    let ptemplate = s.props[pi].clone();
    let prows: Vec<Vec<u8>> = plan
        .peaks
        .iter()
        .map(|p| {
            let mut r = ptemplate.clone();
            set_name(&mut r, &names::peak(p.index));
            set_y(&mut r, y1 + PEAK_STEP * (p.index as i16 + 1));
            r
        })
        .collect();
    s.props.splice(pi + 1..pi + 1, prows);
    let mi = find(&s.props, mountain)?;
    let (my, _, _) = s.layout(mountain)?;
    set_y(&mut s.props[mi], my + PEAK_STEP * n as i16);
    let lp = find(&s.props, list)?;
    let height = u16_at(&s.props[lp], 38)? + (PEAK_STEP as u16) * n as u16;
    put_u16(&mut s.props[lp], 38, height);

    // Goal and event lists.
    for p in &plan.peaks {
        let tabs = p.tabs();
        let goals: Vec<String> = tabs.iter().map(|t| names::goal(p.index, *t)).collect();
        add_list(
            s,
            h("SPG_Peak1Goals"),
            &names::goals(p.index),
            &goals,
            h("SelectPeakGoal"),
        )?;
        for tab in tabs {
            let rows: Vec<String> = (0..p.events(tab).len())
                .map(|r| names::event(p.index, tab, r))
                .collect();
            let root = if tab == FREESTYLE {
                h("Peak1FreestyleLocations")
            } else {
                debug_assert_eq!(tab, RACE);
                h("Peak1RaceLocations")
            };
            add_list(s, root, &names::locs(p.index, tab), &rows, h("SelectEvent"))?;
        }
    }
    Ok(())
}

// --------------------------------------------------------------------- file

/// Patch the plan's rows into the Event Select screen of a whole `FE.LUI`.
pub fn patch_lui(file: &[u8], plan: &MenuPlan) -> R<Vec<u8>> {
    if file.len() < 24 || &file[..4] != b"IYF\0" {
        return Err("not a LUI file".into());
    }
    let table = u32_at(file, 12)? as usize;
    let count = u32_at(file, table)? as usize;
    if count > 4096 {
        return Err("LUI screen table too large".into());
    }
    let entry = |i: usize| -> R<(u32, usize)> {
        Ok((
            u32_at(file, table + 4 + i * 8)?,
            u32_at(file, table + 8 + i * 8)? as usize,
        ))
    };
    let mut all = Vec::with_capacity(count);
    for i in 0..count {
        all.push(entry(i)?);
    }
    let index = all
        .iter()
        .position(|(n, _)| *n == EVENT_SELECT_SCREEN)
        .ok_or("the Event Select screen is not in this LUI")?;
    let begin = table + all[index].1;
    // The screen ends where the next thing in the file starts: another screen,
    // else the object table.
    let object_table = u32_at(file, 16)? as usize;
    let end = all
        .iter()
        .map(|(_, o)| table + o)
        .chain([object_table, u32_at(file, 20)? as usize])
        .filter(|o| *o > begin)
        .min()
        .ok_or("cannot find the end of the Event Select screen")?;
    let mut screen = Screen::parse(&refpack_decode(
        file.get(begin..end).ok_or("bad screen offset")?,
    )?)?;
    add_peak_rows(&mut screen, plan)?;
    let mut replacement = refpack_encode_literals(&screen.to_bytes());
    // The game's word loads align down: every following block keeps its
    // original alignment (16 bytes).
    let old = end - begin;
    replacement.resize(
        replacement.len() + (old as i64 - replacement.len() as i64).rem_euclid(16) as usize,
        0,
    );
    let delta = replacement.len() as i64 - old as i64;
    let mut out = Vec::with_capacity((file.len() as i64 + delta) as usize);
    out.extend_from_slice(&file[..begin]);
    out.extend_from_slice(&replacement);
    out.extend_from_slice(&file[end..]);
    let shift = |v: usize| (v as i64 + delta) as u32;
    put_u32(&mut out, 16, shift(object_table));
    put_u32(&mut out, 20, shift(u32_at(file, 20)? as usize));
    for (i, (_, off)) in all.iter().enumerate() {
        if table + off >= end {
            put_u32(&mut out, table + 8 + i * 8, shift(*off));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use ssx3_plugin::catalog::{Catalog, Event, Peak};
    use ssx3_plugin::manifest::Kind;

    use super::*;

    // ------------------------------------------------ synthetic screen

    fn widget(class: u16, name: &str, extra: &[u32]) -> Vec<u8> {
        let mut w = vec![0u8; 36];
        put_u16(&mut w, 0, class);
        set_name(&mut w, name);
        if class == 0x10 || class == 0x13 {
            put_u32(&mut w, 32, extra.len() as u32);
            extra
                .iter()
                .for_each(|e| w.extend_from_slice(&e.to_le_bytes()));
        } else {
            w.extend_from_slice(&0x1234_5678u32.to_le_bytes());
        }
        let len = w.len() as u16;
        put_u16(&mut w, 2, len);
        w
    }

    fn prop(name: &str, y: i16, w: u16, h: u16) -> Vec<u8> {
        let mut p = vec![0u8; 68];
        put_u16(&mut p, 2, 68);
        set_name(&mut p, name);
        set_y(&mut p, y);
        put_u16(&mut p, 34, w);
        put_u16(&mut p, 38, h);
        p
    }

    fn hs(names: &[&str]) -> Vec<u32> {
        names.iter().map(|n| str_hash(n.as_bytes())).collect()
    }

    /// The parts of the real screen the patch touches, in the real order.
    fn synthetic() -> Screen {
        let text = |n: &str| widget(0x17, n, &[]);
        let widgets = vec![
            text("Peak3"),
            text("Peak2"),
            text("Peak1"),
            text("Mountain"),
            widget(
                0x13,
                "SP_PeakList",
                &hs(&["Peak3", "Peak2", "Peak1", "Mountain"]),
            ),
            widget(0x10, "SelectPeak", &hs(&["SP_PeakList"])),
            text("Peak1RaceGoal"),
            text("Peak1FreeGoal"),
            widget(
                0x13,
                "SPG_Peak1Goals",
                &hs(&["Peak1RaceGoal", "Peak1FreeGoal"]),
            ),
            widget(0x10, "SelectPeakGoal", &hs(&["SPG_Peak1Goals"])),
            text("Peak1RaceLoc0"),
            text("Peak1RaceLoc1"),
            widget(
                0x13,
                "Peak1RaceLocations",
                &hs(&["Peak1RaceLoc0", "Peak1RaceLoc1"]),
            ),
            text("Peak1FreestyleLoc0"),
            widget(
                0x13,
                "Peak1FreestyleLocations",
                &hs(&["Peak1FreestyleLoc0"]),
            ),
            widget(
                0x10,
                "SelectEvent",
                &hs(&["Peak1RaceLocations", "Peak1FreestyleLocations"]),
            ),
        ];
        let props = vec![
            prop("Peak3", 0, 180, 20),
            prop("Peak2", 25, 180, 20),
            prop("Peak1", 50, 180, 20),
            prop("Mountain", 75, 180, 20),
            prop("SP_PeakList", 103, 200, 120),
            prop("Peak1RaceGoal", 0, 180, 20),
            prop("SPG_Peak1Goals", 103, 200, 120),
            prop("Peak1RaceLoc0", 0, 180, 20),
            prop("Peak1RaceLocations", 103, 200, 120),
            prop("Peak1FreestyleLoc0", 0, 180, 20),
            prop("Peak1FreestyleLocations", 103, 200, 120),
        ];
        let mut header = [0u8; 20];
        put_u32(&mut header, 0, EVENT_SELECT_SCREEN);
        let mut s = Screen {
            header,
            widgets,
            props_header: [0; 16],
            props,
            tail: (0..50).collect(),
        };
        s.props_header[0] = 0x33;
        s
    }

    fn catalog(peaks: &[(&str, &[Kind])]) -> Catalog {
        let mut c = Catalog::default();
        for (name, kinds) in peaks {
            c.peaks.push(Peak {
                id: format!("t/{name}"),
                plugin: "t".into(),
                name: name.to_string(),
                picture: None,
            });
            for (i, k) in kinds.iter().enumerate() {
                c.events.push(Event {
                    id: format!("t/{name}{i}"),
                    plugin: "t".into(),
                    name: Some(format!("{name} event {i}")),
                    name_key: None,
                    peak: format!("t/{name}"),
                    kind: *k,
                    template: "ARA1".into(),
                    world: format!("{name}{i}"),
                });
            }
        }
        c
    }

    #[test]
    fn refpack_literals_round_trip_and_the_decoder_reads_the_other_commands() {
        for len in [0usize, 1, 3, 4, 5, 111, 112, 113, 224, 1000, 70_000] {
            let data: Vec<u8> = (0..len).map(|i| (i * 7 + i / 5) as u8).collect();
            let enc = refpack_encode_literals(&data);
            assert_eq!(refpack_decode(&enc).unwrap(), data, "{len}");
        }
        // A copy command: control 0x01 = 1 literal byte, then 3 bytes from 1 back.
        let s = [0x10, 0xfb, 0, 0, 4, 0x01, 0x00, b'a', 0xfc];
        assert_eq!(refpack_decode(&s).unwrap(), b"aaaa");
        // Hostile streams are errors, not panics.
        assert!(refpack_decode(&[0x10, 0xfb, 0, 0, 9, 0x01]).is_err());
        assert!(refpack_decode(&[0x10, 0xfb, 0x7f, 0xff, 0xff, 0xfc]).is_err());
        assert!(refpack_decode(&[0x10, 0xfb, 0, 0, 4, 0x20, 0x05, 0xfc]).is_err());
        assert!(refpack_decode(b"junk").is_err());
    }

    #[test]
    fn a_screen_parses_and_serializes_back_to_the_same_bytes() {
        let s = synthetic();
        let bytes = s.to_bytes();
        let again = Screen::parse(&bytes).unwrap();
        assert_eq!(again.to_bytes(), bytes);
        assert_eq!(again.widget_count(), s.widget_count());
        // No plan: nothing changes.
        let mut t = again.clone();
        add_peak_rows(&mut t, &MenuPlan::default()).unwrap();
        assert_eq!(t.to_bytes(), bytes);
        // Damaged input is an error.
        assert!(Screen::parse(&bytes[..bytes.len() - 60]).is_err());
        assert!(Screen::parse(&bytes[..30]).is_err());
    }

    #[test]
    fn plugin_rows_are_cloned_into_the_screen() {
        let plan = MenuPlan::build(&catalog(&[
            ("alpha", &[Kind::Race, Kind::Race, Kind::Freestyle]),
            ("beta", &[Kind::Race]),
        ]));
        let mut s = Screen::parse(&synthetic().to_bytes()).unwrap();
        let before = s.widget_count();
        add_peak_rows(&mut s, &plan).unwrap();
        // 2 peak rows; alpha: goals (2 rows + menu), race list (2 + menu),
        // freestyle list (1 + menu); beta: goals (1 + menu), race list (1 + menu).
        assert_eq!(s.widget_count(), before + 2 + 3 + 3 + 2 + 2 + 2);
        let h = |n: &str| str_hash(n.as_bytes());
        // Peak list: the new rows follow Peak1, before the Mountain.
        assert_eq!(
            s.children(h("SP_PeakList")).unwrap(),
            hs(&[
                "Peak3",
                "Peak2",
                "Peak1",
                "PlugPeak0",
                "PlugPeak1",
                "Mountain"
            ])
        );
        assert_eq!(s.layout(h("PlugPeak0")).unwrap().0, 75);
        assert_eq!(s.layout(h("PlugPeak1")).unwrap().0, 100);
        assert_eq!(s.layout(h("Mountain")).unwrap().0, 125);
        assert_eq!(s.layout(h("SP_PeakList")).unwrap().2, 170);
        // Containers list the new menus; the rows come before their menu, the
        // menu before its container.
        assert_eq!(
            s.children(h("SelectPeakGoal")).unwrap(),
            hs(&["SPG_Peak1Goals", "PlugGoals0", "PlugGoals1"])
        );
        assert_eq!(
            s.children(h("SelectEvent")).unwrap(),
            hs(&[
                "Peak1RaceLocations",
                "Peak1FreestyleLocations",
                "Plug0RaceLocs",
                "Plug0FreeLocs",
                "Plug1RaceLocs"
            ])
        );
        assert_eq!(
            s.children(h("PlugGoals0")).unwrap(),
            hs(&["PlugGoal0Race", "PlugGoal0Free"])
        );
        assert_eq!(s.children(h("PlugGoals1")).unwrap(), hs(&["PlugGoal1Race"]));
        assert_eq!(
            s.children(h("Plug0RaceLocs")).unwrap(),
            hs(&["Plug0RaceLoc0", "Plug0RaceLoc1"])
        );
        // Created in order: every plugin child exists before its menu, every
        // menu before its container.
        let order: Vec<u32> = s.widgets.iter().map(|w| name_of(w)).collect();
        let pos = |n: u32| order.iter().position(|x| *x == n).unwrap();
        for menu in [
            "PlugGoals0",
            "Plug0RaceLocs",
            "Plug0FreeLocs",
            "Plug1RaceLocs",
        ] {
            for child in s.children(h(menu)).unwrap() {
                assert!(pos(child) < pos(h(menu)), "{menu}");
            }
        }
        for menu in [
            "PlugGoals0",
            "PlugGoals1",
            "Plug0RaceLocs",
            "Plug0FreeLocs",
            "Plug1RaceLocs",
        ] {
            let parent = if menu.starts_with("PlugGoals") {
                "SelectPeakGoal"
            } else {
                "SelectEvent"
            };
            assert!(pos(h(menu)) < pos(h(parent)), "{menu}");
        }
        // Rows are one below the other and wide enough for a name.
        assert_eq!(s.layout(h("Plug0RaceLoc1")).unwrap(), (22, 270, 20));
        assert_eq!(s.layout(h("Plug0RaceLocs")).unwrap().2, 120);
        // Every widget has a layout row and the counts agree after a round trip.
        assert_eq!(s.widget_count(), s.widgets.len());
        let bytes = s.to_bytes();
        let back = Screen::parse(&bytes).unwrap();
        assert_eq!(back.to_bytes(), bytes);
        assert_eq!(back.widget_count(), s.widget_count());
        // The tail (script data) is untouched.
        assert_eq!(
            &bytes[bytes.len() - 50..],
            &(0..50).collect::<Vec<u8>>()[..]
        );
        // The clone keeps the template's class and text hash.
        let w = &s.widgets[find(&s.widgets, h("PlugPeak0")).unwrap()];
        assert_eq!(u16_at(w, 0).unwrap(), 0x17);
        assert_eq!(u32_at(w, 36).unwrap(), 0x1234_5678);
    }

    #[test]
    fn a_tall_list_grows_its_menu() {
        let kinds = vec![Kind::Race; 9];
        let plan = MenuPlan::build(&catalog(&[("tall", &kinds)]));
        let mut s = synthetic();
        add_peak_rows(&mut s, &plan).unwrap();
        assert_eq!(s.layout(str_hash(b"Plug0RaceLocs")).unwrap().2, 9 * 22);
    }

    #[test]
    fn a_screen_missing_what_the_patch_needs_is_an_error() {
        let plan = MenuPlan::build(&catalog(&[("alpha", &[Kind::Race])]));
        let mut s = synthetic();
        s.widgets
            .remove(find(&s.widgets, str_hash(b"Peak1")).unwrap());
        assert!(add_peak_rows(&mut s, &plan).is_err());
    }

    /// A whole LUI file around a synthetic screen: patching grows the screen,
    /// keeps the following screens at 16-byte alignment and moves their offsets.
    fn lui(screen: &Screen) -> Vec<u8> {
        let comp = refpack_encode_literals(&screen.to_bytes());
        let other_before = vec![0xaa; 32];
        let other_after = vec![0xbb; 48];
        // header (24) | screen table | other_before | screen | other_after | object table | fonts
        let table = 24usize;
        let table_len = 4 + 3 * 8;
        let b0 = table + table_len;
        let mut file = vec![0u8; b0];
        file[..4].copy_from_slice(b"IYF\0");
        file.extend_from_slice(&other_before);
        let s_off = file.len();
        let mut comp = comp;
        while comp.len() % 16 != 0 {
            comp.push(0);
        }
        file.extend_from_slice(&comp);
        let a_off = file.len();
        file.extend_from_slice(&other_after);
        let obj = file.len();
        file.extend_from_slice(&[1; 16]);
        let font = file.len();
        file.extend_from_slice(&[2; 16]);
        put_u32(&mut file, 12, table as u32);
        put_u32(&mut file, 16, obj as u32);
        put_u32(&mut file, 20, font as u32);
        put_u32(&mut file, table, 3);
        for (i, (n, o)) in [(0x111u32, b0), (EVENT_SELECT_SCREEN, s_off), (0x222, a_off)]
            .into_iter()
            .enumerate()
        {
            put_u32(&mut file, table + 4 + i * 8, n);
            put_u32(&mut file, table + 8 + i * 8, (o - table) as u32);
        }
        file
    }

    #[test]
    fn a_lui_file_is_patched_with_its_offsets_and_alignment_kept() {
        let screen = synthetic();
        let file = lui(&screen);
        // No plan: the screen is the same, bytes after it too.
        let same = patch_lui(&file, &MenuPlan::default()).unwrap();
        let plan = MenuPlan::build(&catalog(&[("alpha", &[Kind::Race, Kind::Freestyle])]));
        let out = patch_lui(&file, &plan).unwrap();
        assert!(out.len() > file.len());
        assert_eq!(same.len() % 16, file.len() % 16);
        let delta = out.len() - file.len();
        assert_eq!(delta % 16, 0, "later blocks keep their alignment");
        let table = u32_at(&out, 12).unwrap() as usize;
        let (n0, o0) = (
            u32_at(&out, table + 4).unwrap(),
            u32_at(&out, table + 8).unwrap(),
        );
        let (n1, o1) = (
            u32_at(&out, table + 12).unwrap(),
            u32_at(&out, table + 16).unwrap(),
        );
        let (n2, o2) = (
            u32_at(&out, table + 20).unwrap(),
            u32_at(&out, table + 24).unwrap(),
        );
        assert_eq!((n0, n1, n2), (0x111, EVENT_SELECT_SCREEN, 0x222));
        assert_eq!(o0, u32_at(&file, table + 8).unwrap());
        assert_eq!(o1, u32_at(&file, table + 16).unwrap());
        assert_eq!(
            o2 as usize,
            u32_at(&file, table + 24).unwrap() as usize + delta
        );
        assert_eq!(
            u32_at(&out, 16).unwrap() as usize,
            u32_at(&file, 16).unwrap() as usize + delta
        );
        assert_eq!(
            u32_at(&out, 20).unwrap() as usize,
            u32_at(&file, 20).unwrap() as usize + delta
        );
        // The later screen's bytes moved unchanged; the patched screen decodes
        // and has the plugin rows.
        let a = table + o2 as usize;
        assert_eq!(&out[a..a + 48], &[0xbb; 48][..]);
        let patched =
            Screen::parse(&refpack_decode(&out[table + o1 as usize..a]).unwrap()).unwrap();
        assert!(patched.has_widget(str_hash(b"PlugPeak0")));
        // Not a LUI, or no such screen: errors.
        assert!(patch_lui(b"nope", &plan).is_err());
        let mut no_screen = file.clone();
        put_u32(&mut no_screen, table + 12, 0x999);
        assert!(patch_lui(&no_screen, &plan).is_err());
    }

    // ------------------------------------------- the real screen (needs the disc)

    fn real_fe_lui() -> Option<Vec<u8>> {
        real_lui("DATA/UI/FE.LUI")
    }

    fn real_lui(path: &str) -> Option<Vec<u8>> {
        let iso = ssx3_core::platform_paths::data_file(
            "SSX3_ISO",
            "SSX 3 (USA).iso",
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../SSX 3 (USA).iso"),
        );
        let mut disc = ssx3_core::iso::Disc::open(&iso).ok()?;
        let e = disc.find(path)?;
        let data = disc.read_vec(e.sector, e.size.div_ceil(2048));
        Some(data[..e.size as usize].to_vec())
    }

    #[test]
    fn the_real_event_select_screen_round_trips_and_takes_plugin_rows() {
        let Some(file) = real_fe_lui() else {
            eprintln!("skipped: no SSX 3 disc image");
            return;
        };
        let table = u32_at(&file, 12).unwrap() as usize;
        let count = u32_at(&file, table).unwrap() as usize;
        let (_, off) = (0..count)
            .map(|i| {
                (
                    u32_at(&file, table + 4 + i * 8).unwrap(),
                    u32_at(&file, table + 8 + i * 8).unwrap(),
                )
            })
            .find(|(n, _)| *n == EVENT_SELECT_SCREEN)
            .unwrap();
        let begin = table + off as usize;
        let end = (0..count)
            .map(|i| table + u32_at(&file, table + 8 + i * 8).unwrap() as usize)
            .filter(|o| *o > begin)
            .min()
            .unwrap();
        let raw = refpack_decode(&file[begin..end]).unwrap();
        // Parse and serialize: the very same bytes.
        let screen = Screen::parse(&raw).unwrap();
        assert_eq!(screen.widget_count(), 377);
        assert_eq!(screen.to_bytes(), raw);
        // The game's own rows the patch clones are all there.
        let h = |n: &str| str_hash(n.as_bytes());
        for n in [
            "Peak1",
            "Mountain",
            "SP_PeakList",
            "SPG_Peak1Goals",
            "Peak1RaceLocations",
            "Peak1FreestyleLocations",
            "SelectPeakGoal",
            "SelectEvent",
        ] {
            assert!(screen.has_widget(h(n)), "{n}");
        }
        assert_eq!(screen.layout(h("Mountain")).unwrap().0, 75);
        assert_eq!(screen.layout(h("SP_PeakList")).unwrap().2, 120);
        // No plan: the same screen.
        assert_eq!(refpack_decode(&refpack_encode_literals(&raw)).unwrap(), raw);
        // One plugin peak with 2 race and 1 freestyle events.
        let plan = MenuPlan::build(&catalog(&[(
            "alpha",
            &[Kind::Race, Kind::Race, Kind::Freestyle],
        )]));
        let out = patch_lui(&file, &plan).unwrap();
        assert_eq!((out.len() - file.len()) % 16, 0);
        let t2 = u32_at(&out, 12).unwrap() as usize;
        let b2 = t2
            + (0..count)
                .map(|i| {
                    (
                        u32_at(&out, t2 + 4 + i * 8).unwrap(),
                        u32_at(&out, t2 + 8 + i * 8).unwrap(),
                    )
                })
                .find(|(n, _)| *n == EVENT_SELECT_SCREEN)
                .unwrap()
                .1 as usize;
        let e2 = (0..count)
            .map(|i| t2 + u32_at(&out, t2 + 8 + i * 8).unwrap() as usize)
            .filter(|o| *o > b2)
            .min()
            .unwrap();
        let patched = Screen::parse(&refpack_decode(&out[b2..e2]).unwrap()).unwrap();
        // 1 peak row + goals (2 rows + menu) + race list (2 + menu) + freestyle (1 + menu).
        assert_eq!(patched.widget_count(), 377 + 1 + 3 + 3 + 2);
        assert_eq!(patched.layout(h("Mountain")).unwrap().0, 100);
        assert_eq!(patched.layout(h("SP_PeakList")).unwrap().2, 145);
        assert_eq!(
            patched.children(h("SP_PeakList")).unwrap(),
            vec![
                h("Peak3"),
                h("Peak2"),
                h("Peak1"),
                h("PlugPeak0"),
                h("Mountain")
            ]
        );
        // Everything after the screen is byte for byte what it was.
        assert_eq!(file[end..], out[e2..]);
    }

    /// The in-game screen (`OV.LUI`, shown after a race: "Next event") is
    /// patched like `FE.LUI`.
    #[test]
    fn the_in_game_lui_takes_plugin_rows_too() {
        let Some(file) = real_lui("DATA/UI/OV.LUI") else {
            eprintln!("skipped: no SSX 3 disc image");
            return;
        };
        let plan = MenuPlan::build(&catalog(&[("alpha", &[Kind::Race, Kind::Freestyle])]));
        let out = patch_lui(&file, &plan).unwrap();
        assert_eq!((out.len() - file.len()) % 16, 0);
        assert!(out.len() > file.len());
        assert_eq!(&out[..4], b"IYF\0");
    }
}
