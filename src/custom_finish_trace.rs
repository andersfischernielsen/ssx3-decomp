//! Bounded, change-only observations of the native finish transition.
//!
//! Reads RAM without changing game state, timing, registers or UI ownership.

use ssx3_ee_rt::cpu::reg::GP;
use ssx3_ee_rt::Cpu;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::{LazyLock, Mutex};

#[derive(Debug, Eq, PartialEq)]
struct Rider {
    address: u32,
    finished: Option<bool>,
    dnf: Option<u32>,
}

#[derive(Debug, Eq, PartialEq)]
struct Snapshot {
    mode: Option<u8>,
    single_event: Option<u8>,
    module_gate: Option<u32>,
    manager: Option<u32>,
    handler: Option<u32>,
    heat: Option<u32>,
    wait_for_results: Option<u32>,
    results_ready: Option<u32>,
    humans: Option<u32>,
    riders: Vec<Rider>,
    finish: u32,
    next_state: Option<u32>,
    phase: Option<u32>,
    delay_armed: Option<bool>,
    detail: u32,
}

#[derive(Default)]
struct Trace {
    course: Option<&'static str>,
    run: u64,
    lines: u8,
    last: HashMap<&'static str, Snapshot>,
    file: Option<File>,
    opened: bool,
}

static TRACE: LazyLock<Mutex<Trace>> = LazyLock::new(|| Mutex::new(Trace::default()));

fn physical(at: u32, size: u32) -> Option<u32> {
    let at = at & 0x1fff_ffff;
    (at >= 0x10000 && at.checked_add(size).is_some_and(|end| end <= 0x0200_0000))
        .then_some(at)
}

fn word(c: &mut Cpu, at: u32) -> Option<u32> {
    physical(at, 4).map(|at| c.bus.read32(at))
}

fn field(c: &mut Cpu, base: Option<u32>, offset: u32) -> Option<u32> {
    // Validate the owner too: a null/invalid pointer must not become a valid
    // absolute address merely because the field offset is added to it.
    let base = physical(base?, offset.checked_add(4)?)?;
    word(c, base.checked_add(offset)?)
}

fn byte(c: &mut Cpu, at: u32) -> Option<u8> {
    physical(at, 1).map(|at| c.bus.read8(at))
}

impl Trace {
    fn reset_run(&mut self) {
        self.run = self.run.wrapping_add(1);
        self.lines = 0;
        self.last.clear();
    }

    fn write(&mut self, line: &str) {
        if !self.opened {
            self.opened = true;
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../.cache/diagnostics");
            let stamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |time| time.as_secs());
            let path = root.join(format!("finish-{}-{stamp}.log", std::process::id()));
            if std::fs::create_dir_all(root).is_ok() {
                self.file = File::create(&path).ok();
                if self.file.is_some() {
                    eprintln!("[tricky finish] observations: {}", path.display());
                }
            }
        }
        if let Some(file) = &mut self.file {
            let _ = writeln!(file, "{line}");
            // A UI stall or process exit must not lose the last stage.
            let _ = file.flush();
        }
    }
}

pub fn restart() {
    TRACE.lock().unwrap().reset_run();
}

/// At most 64 changed records per course/run. The countdown's positive value
/// and rider finish timers are deliberately absent from the comparison key.
/// Ordinary frame updates therefore do not create per-frame disk output.
pub fn observe(c: &mut Cpu, stage: &'static str, finish: u32, detail: u32) {
    let Some(track) = crate::custom_tracks::active() else { return };
    let mut trace = TRACE.lock().unwrap();
    if trace.course != Some(track.id) {
        trace.course = Some(track.id);
        trace.reset_run();
    }
    if trace.lines >= 64 { return }

    let app = word(c, (c.g[GP][0] as u32).wrapping_sub(0x848));
    let module = field(c, app, 0x84);
    let manager = field(c, app, 0xc0);
    let ai = field(c, module, 0xc);
    let gate = field(c, module, 0x28);
    let humans = field(c, ai, 0x7c);
    let mut riders = Vec::new();
    for index in 0..humans.unwrap_or(0).min(6) {
        let player = field(c, ai, 0x40 + index * 4);
        let address = field(c, player, 0x18).unwrap_or(0);
        riders.push(Rider {
            address,
            finished: field(c, Some(address), 0x470)
                .map(|bits| f32::from_bits(bits) >= 0.),
            dnf: field(c, Some(address), 0x480),
        });
    }
    // A finish-state object is supplied explicitly. Other entry points do
    // not infer it by scanning memory or mistaking another state for it.
    let finish_base = (finish != 0).then_some(finish);
    let delay = field(c, finish_base, 0x14);
    let snapshot = Snapshot {
        mode: byte(c, 0x0053_5c10),
        single_event: byte(c, 0x0053_5c11),
        module_gate: field(c, gate, 0),
        manager,
        handler: field(c, manager, 4),
        heat: field(c, manager, 0),
        wait_for_results: field(c, manager, 0x98),
        results_ready: field(c, manager, 0x9c),
        humans,
        riders,
        finish,
        next_state: field(c, finish_base, 0),
        phase: field(c, finish_base, 0x10),
        delay_armed: delay.map(|delay| delay != 0),
        detail,
    };
    if trace.last.get(stage) == Some(&snapshot) { return }
    trace.lines += 1;
    let line = format!(
        "[tricky finish v1] course={} run={} stage={stage} cycles={} delay={delay:?} {snapshot:?}",
        track.id, trace.run, c.cycles,
    );
    trace.write(&line);
    trace.last.insert(stage, snapshot);
}
