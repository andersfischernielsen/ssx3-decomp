//! Phase 3 rendering, stage 0b check (`SSX3_VERIFY_FRAME=1`): the typed
//! [`Frame`](ssx3_render::frame::Frame) built from the DeferMan buckets must
//! account for the published chains exactly.
//!
//! Hook: `PS2GraphicsMan_EndFrame` signals the kick semaphore (gm+0x5ac8)
//! right after publishing both phases' chain heads (gm+0x5aa8..) and before
//! `DeferMan_ResetFrame`, so buckets, states, MatrixRecords and the chain are
//! all intact. There the frame is built and checked:
//!
//! - (a) the stage-0a walker (`chain::walk` + `vif::Vif1`) over the
//!   published chains sees the same VU calls, in order, as the frame; each
//!   one belongs to exactly one draw (its payload, or its prelude for the
//!   type-6 lighting packet); none is left over;
//! - (b) the GS state registers each draw's RenderState predicts (through
//!   the simulated diff emission) equal the A+D writes the chain made before
//!   that draw's CALL;
//! - (c) the TEX0 of each bound texture equals the last TEX0 (TEX2 merged)
//!   written for that context before the CALL.
//!
//! Informational: draws whose payload-time GS state differs from the emitted
//! state (a previous payload wrote TEST/ZBUF/..), and the VU1 header matrix
//! (qw0-3) against the draw's MatrixRecord.

use std::sync::Mutex;

use ssx3_render::chain::{self, Mem};
use ssx3_render::frame::{self, reg_name, Carry, GsRegs, Material, Owner};
use ssx3_render::vif::{Event, Vif1};

use crate::cpu::Cpu;

#[derive(Default)]
struct Kind {
    frames: u64,
    draws: u64,
    calls: u64,
    directs: u64,
}

#[derive(Default)]
struct State {
    carry: Carry,
    started: bool,
    menu: Kind,
    race: Kind,
    // (a)
    calls_ok: u64,
    calls_bad: u64,
    prelude_calls: u64,
    leftover_calls: u64,
    issues: u64,
    // (b)
    regs_ok: u64,
    regs_bad: u64,
    regs_unknown: u64,
    clobbered_draws: u64,
    // (c)
    tex_ok: u64,
    tex_moved: u64,
    tex_bad: u64,
    // info
    clobbered: [u64; 10],
    hdr_predicted_stale: u64,
    split_quirk: u64,
    uncalled: u64,
    continuation: u64,
    setups: [u64; 3],
    markers: [u64; 6],
    payload_writers: std::collections::BTreeMap<(u32, u8, bool), u64>,
    // info
    hdr_draw: u64,
    hdr_stale: u64,
    hdr_other: u64,
    emitter_bad: u64,
    reported: u32,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

pub fn enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        let on = std::env::var_os("SSX3_VERIFY_FRAME").is_some();
        if on {
            frame::keep_payloads(true);
        }
        on
    })
}

struct EeMem<'a> {
    ram: &'a [u8],
    spr: &'a [u8],
}

impl Mem for EeMem<'_> {
    fn qword(&self, addr: u32) -> u128 {
        crate::hw::mem_q(self.ram, self.spr, addr, addr & 0x8000_0000 != 0)
    }
}

fn report(s: &mut State, msg: String) {
    if s.reported < 30 {
        s.reported += 1;
        eprintln!("[frame-verify] {msg}");
    }
}

/// `SignalSema(id)` from the EE: if it is the kick semaphore, check the
/// frame just published.
pub fn on_signal_sema(c: &Cpu, id: i32) {
    let mem = EeMem { ram: &c.bus.ram, spr: &c.bus.spr };
    let g = frame::word(&mem, frame::addr::GM_PTR);
    if g == 0 || frame::word(&mem, g + frame::gm::KICK_SEMA) as i32 != id {
        return;
    }
    let mut guard = STATE.lock().unwrap();
    let s = guard.get_or_insert_with(State::default);
    check(s, &mem, g);
}

fn check(s: &mut State, m: &EeMem, g: u32) {
    let b = frame::build(m, g, &s.carry);
    let fr = &b.frame;
    let draws: Vec<&frame::Draw> = fr.draws().collect();
    let race = draws.iter().any(|d| matches!(d.state.material, Material::Bezier1 | Material::Bezier2));
    let defer = fr.defer;

    // Emitter end state vs memory (+0x69cc0 / +0x69cbc); the first frame's
    // start values are unknown.
    let (mm, mp) = (frame::word(m, defer + frame::dm::CUR_MATRIX) as i32, frame::word(m, defer + frame::dm::EMIT_MATERIAL));
    if s.started && (b.emitter.matrix != mm || b.emitter.material_param != mp) {
        s.emitter_bad += 1;
        report(s, format!("emitter end matrix/param {}/{:#x}, memory {mm}/{mp:#x}", b.emitter.matrix, b.emitter.material_param));
    }

    for i in &b.issues {
        s.issues += 1;
        report(s, format!("structure: {i}"));
    }

    // (a) reference walk.
    let mut v = Vif1::new(s.carry.regs);
    for &head in &fr.vif_heads {
        let mut asr = [0u32; 2];
        chain::walk(m, head, false, true, 0, &mut asr, &mut |q| v.write(q));
    }
    let ref_calls: Vec<_> = v.events.iter().filter_map(|e| if let Event::Call(c) = e { Some(c) } else { None }).collect();
    let ref_directs = v.events.iter().filter(|e| matches!(e, Event::Direct(_))).count();
    let ours: Vec<_> = b.events.iter().filter_map(|(o, e)| if let Event::Call(c) = e { Some((*o, c)) } else { None }).collect();
    let our_directs = b.events.iter().filter(|(_, e)| matches!(e, Event::Direct(_))).count();
    if ref_calls.len() != ours.len() || ref_directs != our_directs {
        s.issues += 1;
        report(s, format!("walker calls/directs {}/{} vs frame {}/{}", ref_calls.len(), ref_directs, ours.len(), our_directs));
    }
    for (k, (o, c)) in ours.iter().enumerate() {
        let same = ref_calls.get(k).is_some_and(|r| *r == *c);
        let owned = match o {
            Owner::Payload(_) => true,
            Owner::Prelude(_) => {
                s.prelude_calls += 1;
                true
            }
            _ => false,
        };
        if !owned {
            s.leftover_calls += 1;
        }
        if same && owned {
            s.calls_ok += 1;
        } else {
            s.calls_bad += 1;
            report(s, format!("call {k} entry {:?}: same as walker {same}, owner {o:?}", c.entry));
        }
    }
    s.calls_bad += ref_calls.len().saturating_sub(ours.len()) as u64;

    // (b), (c) per draw. Draw indices in obs are emission order = draws order.
    let texman = frame::word(m, g + frame::gm::TEXMAN);
    let mut flagged = 0;
    for (i, d) in draws.iter().enumerate() {
        let o = &b.obs[i];
        if d.continuation {
            s.continuation += 1;
        }
        let ty = d.state.material.type_id();
        if d.header_matrix != d.matrix as i32 && matches!(ty, 1 | 2 | 3 | 6) {
            s.hdr_predicted_stale += 1;
        }
        s.setups[0] += d.setup.program as u64;
        s.setups[1] += d.setup.matrix as u64;
        s.setups[2] += d.setup.param as u64;
        if o.call_tag == 0 {
            s.uncalled += 1;
            continue;
        }
        if !o.ret_at_end {
            s.issues += 1;
            report(s, format!("draw {i}: RET not at bucket end {:#x}", d.packets.end));
        }
        let mut bad = Vec::new();
        for (k, &r) in GsRegs::REGS.iter().enumerate() {
            match (d.gs.0[k], o.emitted.0[k]) {
                (None, _) => s.regs_unknown += 1,
                (Some(p), Some(e)) if p == e => s.regs_ok += 1,
                (p, e) => {
                    s.regs_bad += 1;
                    bad.push(format!("{} predicted {p:x?} chain {e:x?}", reg_name(r)));
                }
            }
        }
        if !bad.is_empty() && flagged < 3 {
            flagged += 1;
            report(s, format!("draw {i} (sorted {}, state {:x?}): {}", d.sorted, d.state.raw, bad.join(", ")));
        }
        {
            // Which draws write state registers from their payload
            // (material type, register, value differs from the emitted one).
            let mut gr = ssx3_render::gif::GifReader::default();
            let mut seen = std::collections::BTreeSet::new();
            for p in &d.payload {
                if let ssx3_render::frame::Payload::Gif(q) = p {
                    gr.feed(q, &mut |r, v| {
                        if GsRegs::slot(r).is_some() {
                            seen.insert((r, d.gs.get(r) != Some(v)));
                        }
                    });
                }
            }
            for (r, differs) in seen {
                *s.payload_writers.entry((ty, r, differs)).or_default() += 1;
            }
        }
        if o.actual != o.emitted {
            s.clobbered_draws += 1;
            for k in 0..10 {
                if o.actual.0[k] != o.emitted.0[k] {
                    s.clobbered[k] += 1;
                }
            }
        }
        for ctx in 0..2 {
            let Some(t) = d.tex[ctx] else { continue };
            let ti = frame::word(m, texman + 8 + t as u32 * 4);
            let tex0 = frame::word(m, ti + 0x38) as u64 | (frame::word(m, ti + 0x3c) as u64) << 32;
            let tbp = frame::word(m, ti + 0x28) as i32;
            let cbp = frame::word(m, ti + 0x2c) as i32;
            let mut want = tex0 | tbp.max(0) as u64;
            if cbp >= 0 {
                want |= (cbp as u64) << 37;
            }
            // Streamed textures get VRAM per draw (LRU on the draw index) and
            // may be evicted later in the frame: TexInfo's TBP is then -1 or
            // elsewhere; a CLUT likewise (CBP -1 now). Everything else must
            // still match.
            const TBP0: u64 = 0x3fff;
            const CBP: u64 = 0x3fff << 37;
            let mask = !(TBP0 | if cbp < 0 { CBP } else { 0 });
            if o.tex0[ctx] == Some(want) {
                s.tex_ok += 1;
            } else if o.tex0[ctx].is_some_and(|c| c & mask == want & mask) {
                s.tex_moved += 1;
            } else {
                s.tex_bad += 1;
                report(s, format!("draw {i} ctx {ctx} tex {t:#x}: TexInfo predicts {want:#x} (tbp {tbp:#x} cbp {cbp:#x}), chain {:x?}", o.tex0[ctx]));
            }
        }
    }

    // Info: VU1 header qw0-3 vs MatrixRecord, from a shadow of what the VIF wrote.
    let mut shadow = vec![0u128; 1024];
    for (o, e) in &b.events {
        let Event::Call(c) = e else { continue };
        for (a, q) in c.writes() {
            shadow[*a as usize & 1023] = *q;
        }
        let Owner::Payload(i) = o else { continue };
        let d = draws[*i];
        if !matches!(d.state.material, Material::Bezier1 | Material::Bezier2 | Material::General | Material::Skinned) {
            continue;
        }
        let hdr = |mi: i32| fr.matrices.get(mi as usize).map(|r| r.header_qwords());
        let cur = &shadow[0..4];
        if hdr(d.matrix as i32).is_some_and(|h| h[..4] == *cur) {
            s.hdr_draw += 1;
        } else if hdr(d.header_matrix).is_some_and(|h| h[..4] == *cur) {
            s.hdr_stale += 1;
        } else {
            s.hdr_other += 1;
        }
    }

    // Layer split: a range-0 draw with layer >= 6 (the search never tests
    // a range's first bucket).
    let mut first_pass = true;
    for p in &fr.passes {
        use ssx3_render::frame::Pass;
        match p {
            Pass::Draws(ds) if first_pass => {
                first_pass = false;
                s.split_quirk += ds.iter().filter(|d| d.state.layer >= 6).count() as u64;
            }
            Pass::DepthFog { .. } => {
                first_pass = false;
                s.markers[0] += 1
            }
            Pass::PhaseBoundary => s.markers[1] += 1,
            Pass::Bloom { .. } => s.markers[2] += 1,
            Pass::ColorFilter { .. } => s.markers[3] += 1,
            Pass::PostChain(_) => s.markers[4] += 1,
            Pass::DefaultState(_) => s.markers[5] += 1,
            _ => {}
        }
    }

    // Debug: SSX3_VERIFY_FRAME_OUTLINE=race|menu prints the pass outline of
    // a few frames of that kind (every 200th).
    let want = std::env::var("SSX3_VERIFY_FRAME_OUTLINE").ok();
    let nth = if race { s.race.frames } else { s.menu.frames };
    if want.as_deref() == Some(if race { "race" } else { "menu" }) && nth % 200 == 100 {
        let outline: Vec<String> = fr
            .passes
            .iter()
            .map(|p| match p {
                ssx3_render::frame::Pass::Draws(d) => format!("Draws[{} layers {:?}]", d.len(), d.iter().map(|x| x.state.layer).collect::<std::collections::BTreeSet<_>>()),
                ssx3_render::frame::Pass::DefaultState(v) => format!("Default({})", v.len()),
                ssx3_render::frame::Pass::PostChain(v) => format!("Post({})", v.len()),
                ssx3_render::frame::Pass::DepthFog { view, .. } => format!("DepthFog({view})"),
                ssx3_render::frame::Pass::Bloom { view, .. } => format!("Bloom({view})"),
                other => format!("{other:?}"),
            })
            .collect();
        eprintln!("[frame-verify] outline: {}", outline.join(" "));
    }
    let k = if race { &mut s.race } else { &mut s.menu };
    k.frames += 1;
    k.draws += draws.len() as u64;
    k.calls += ours.len() as u64;
    k.directs += our_directs as u64;

    s.carry = Carry { regs: b.regs_end, matrix: b.emitter.matrix, material_param: b.emitter.material_param };
    s.started = true;
}

/// Summary line.
pub fn summary() -> Option<String> {
    let g = STATE.lock().unwrap();
    let s = g.as_ref()?;
    Some(format!(
        "[frame-verify] frames menu {} race {}; draws {}/{}; calls {}/{} directs {}/{} | (a) calls ok {} bad {} (prelude {}, leftover {}) structure issues {} | (b) regs ok {} bad {} unknown {} (payload-clobbered draws {}) | (c) tex ok {} ok-but-VRAM-reallocated {} bad {} | header qw0-3: draw matrix {} stale {} other {} (predicted stale {}); emitter mismatches {} | info: clobbered by payload {} ; payload state writers (type reg differs: draws) {} ; uncalled {} continuation {} setups program/matrix/param {:?}; range-0 layer>=6 draws {}; markers fog/phase/bloom/cf/postchain/default {:?}",
        s.menu.frames,
        s.race.frames,
        s.menu.draws,
        s.race.draws,
        s.menu.calls,
        s.race.calls,
        s.menu.directs,
        s.race.directs,
        s.calls_ok,
        s.calls_bad,
        s.prelude_calls,
        s.leftover_calls,
        s.issues,
        s.regs_ok,
        s.regs_bad,
        s.regs_unknown,
        s.clobbered_draws,
        s.tex_ok,
        s.tex_moved,
        s.tex_bad,
        s.hdr_draw,
        s.hdr_stale,
        s.hdr_other,
        s.hdr_predicted_stale,
        s.emitter_bad,
        GsRegs::REGS.iter().zip(s.clobbered).filter(|(_, n)| *n > 0).map(|(r, n)| format!("{} {n}", reg_name(*r))).collect::<Vec<_>>().join(", "),
        s.payload_writers.iter().map(|((t, r, dif), n)| format!("t{t} {} {dif}: {n}", reg_name(*r))).collect::<Vec<_>>().join(", "),
        s.uncalled,
        s.continuation,
        s.setups,
        s.split_quirk,
        s.markers
    ))
}
