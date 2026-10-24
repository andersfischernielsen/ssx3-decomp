//! Phase 3 rendering, stage 2A check (`SSX3_VERIFY_PARTICLES=1`): the native
//! particle programs (`ssx3_render::mat4`, type 4, and `ssx3_render::mat5`,
//! type 5 snow) must produce exactly the XGKICK packets and the data memory
//! of the translated microprograms (`f55ee63416fe5dcf`, `e72540c9e7c434ee`).
//!
//! On every VIF1 microprogram start in either program, [`begin`] copies VU1
//! data memory; the translated program then runs as usual while its kicks
//! are collected ([`Pending::kick`]); [`end`] runs the native program on the
//! copy and compares the kick sequence and the whole data memory. Counts per
//! entry are printed at exit ([`summary`]), with the sprites drawn and, for
//! type 4, which header the call saw (qw4.x: the mode-1 screen scale 256, or
//! the mode-0 guard scale; phase3-render.md §1.5). The first mismatches are
//! printed with details. Emulation itself is unchanged.
//!
//! The native side starts every call from a fresh `VuState` on purpose: a
//! mismatch would show a VU register carried over between calls that the
//! state does not model.

use std::collections::BTreeMap;
use std::sync::Mutex;

use ssx3_render::{mat4, mat5};

use crate::vu::Vu;
use crate::Qword;

pub fn enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("SSX3_VERIFY_PARTICLES").is_some())
}

#[derive(Default, Clone, Copy)]
struct Counts {
    ok: u64,
    mismatch: u64,
    not_implemented: u64,
    /// Sprites in the translated program's kicks (sum of NLOOP).
    sprites: u64,
}

#[derive(Default)]
struct State {
    /// Keyed by (program type 4/5, MSCAL entry).
    counts: BTreeMap<(u8, u32), Counts>,
    /// Type 4: calls per header kind (see [`header_kind`]).
    headers: BTreeMap<(u32, u32, bool), u64>,
    reported: u32,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

/// One call in flight: the input copy and the translated program's kicks.
pub struct Pending {
    program: u8,
    entry: u32,
    top: u32,
    itop: u32,
    mem: Box<[u8; 16384]>,
    kicks: Vec<Vec<Qword>>,
}

impl Pending {
    pub fn kick(&mut self, p: &[Qword]) {
        self.kicks.push(p.to_vec());
    }
}

/// Before the translated program runs (VIF1 only).
pub fn begin(vu: &mut Vu, pc: u32, top: u32, itop: u32) -> Option<Pending> {
    if !enabled() {
        return None;
    }
    let program = match vu.segment_hash_pub(pc) {
        mat4::HASH => 4,
        mat5::HASH => 5,
        _ => return None,
    };
    Some(Pending {
        program,
        entry: pc,
        top,
        itop,
        mem: vu.data[..].try_into().map(Box::new).ok()?,
        kicks: Vec::new(),
    })
}

/// The type-4 header as (qw4.x, qw6.x as f32 bits, M affine): the mode-1
/// header has the screen scale (256) and M = worldClip (perspective: some
/// column has w != 0 besides the translation); the mode-0 header has the
/// guard scale (1024) and M = worldView (affine: w row = 0, 0, 0, 1).
fn header_kind(mem: &[u8; 16384]) -> (u32, u32, bool) {
    let f = |qw: usize, lane: usize| {
        u32::from_le_bytes(
            mem[qw * 16 + lane * 4..qw * 16 + lane * 4 + 4]
                .try_into()
                .unwrap(),
        )
    };
    let affine = f(0, 3) == 0 && f(1, 3) == 0 && f(2, 3) == 0 && f(3, 3) == 0x3f80_0000;
    (f(4, 0), f(6, 0), affine)
}

fn sprites(kicks: &[Vec<Qword>]) -> u64 {
    kicks
        .iter()
        .filter_map(|k| k.first())
        .map(|t| *t as u64 & 0x7fff)
        .sum()
}

/// After the translated program finished: run the native one and compare.
pub fn end(mut p: Pending, vu: &Vu) {
    let mut native: Vec<Vec<Qword>> = Vec::new();
    let header = header_kind(&p.mem);
    let implemented = if p.program == 4 {
        mat4::run(
            p.entry,
            p.top,
            p.itop,
            &mut p.mem,
            &mut mat4::VuState::default(),
            &mut |k| native.push(k.to_vec()),
        )
    } else {
        mat5::run(
            p.entry,
            p.top,
            p.itop,
            &mut p.mem,
            &mut mat5::VuState::default(),
            &mut |k| native.push(k.to_vec()),
        )
    };
    let mut g = STATE.lock().unwrap();
    let s = g.get_or_insert_with(State::default);
    if p.program == 4 {
        *s.headers.entry(header).or_default() += 1;
    }
    let c = s.counts.entry((p.program, p.entry)).or_default();
    c.sprites += sprites(&p.kicks);
    if !implemented {
        c.not_implemented += 1;
        return;
    }
    let kicks_ok = native == p.kicks;
    let mem_ok = p.mem[..] == vu.data[..];
    if kicks_ok && mem_ok {
        c.ok += 1;
        return;
    }
    c.mismatch += 1;
    if s.reported >= 10 {
        return;
    }
    s.reported += 1;
    let frame = crate::vu::capture::frame();
    eprintln!(
        "[particle-verify] mismatch #{}: type {} entry {:#06x} top {:#05x} itop {:#x} frame {frame}: kicks native {} translated {}",
        s.reported,
        p.program,
        p.entry,
        p.top,
        p.itop,
        native.len(),
        p.kicks.len()
    );
    if let Some(i) = (0..native.len().max(p.kicks.len())).find(|&i| native.get(i) != p.kicks.get(i))
    {
        let (a, b) = (native.get(i), p.kicks.get(i));
        eprintln!(
            "  kick {i}: native {} qw, translated {} qw",
            a.map_or(0, |k| k.len()),
            b.map_or(0, |k| k.len())
        );
        if let (Some(a), Some(b)) = (a, b) {
            for (j, (x, y)) in a
                .iter()
                .zip(b)
                .enumerate()
                .filter(|(_, (x, y))| x != y)
                .take(6)
            {
                eprintln!("    qw {j}: native {x:032x} translated {y:032x}");
            }
        }
    }
    let diff: Vec<usize> = (0..1024)
        .filter(|q| p.mem[q * 16..q * 16 + 16] != vu.data[q * 16..q * 16 + 16])
        .collect();
    if !diff.is_empty() {
        eprintln!(
            "  data memory: {} qwords differ, first at {:?}",
            diff.len(),
            &diff[..diff.len().min(8)]
        );
    }
}

/// Counts per entry, for the exit report (None when the check is off).
pub fn summary() -> Option<String> {
    let g = STATE.lock().unwrap();
    let s = g.as_ref()?;
    let mut out = String::from(
        "[particle-verify] native types 4/5 vs translated f55ee634 / e72540c9 (entry: ok, mismatch, not implemented, sprites):",
    );
    let mut total = Counts::default();
    for (&(program, entry), c) in &s.counts {
        out += &format!(
            "\n  type {program} {entry:#06x}: ok {} mismatch {} not-implemented {} sprites {}",
            c.ok, c.mismatch, c.not_implemented, c.sprites
        );
        total.ok += c.ok;
        total.mismatch += c.mismatch;
        total.not_implemented += c.not_implemented;
    }
    out += &format!(
        "\n  total: ok {} mismatch {} not-implemented {}",
        total.ok, total.mismatch, total.not_implemented
    );
    out += "\n  type 4 headers (qw4.x scale: 256 screen / 1024 guard; M affine = worldView, else worldClip):";
    let mut kinds: Vec<_> = s.headers.iter().collect();
    kinds.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
    let affine: u64 = kinds
        .iter()
        .filter(|((_, _, a), _)| *a)
        .map(|(_, n)| **n)
        .sum();
    out += &format!(" {} kinds, {affine} calls with affine M", kinds.len());
    for (&(x, p, a), n) in kinds.iter().take(8) {
        out += &format!(
            "\n    scale {} proj.x {} M {}: {n}",
            f32::from_bits(x),
            f32::from_bits(p),
            if a { "affine" } else { "perspective" }
        );
    }
    Some(out)
}
