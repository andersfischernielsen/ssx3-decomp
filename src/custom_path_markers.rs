//! Native WorldAIP marker loading and path-pointer relocation.

use crate::call::{guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0026_b880, read_markers, "PathData_ReadMarkers")];

const READ_INT: Callee = Callee {
    addr: 0x0026_ba68,
    f: ssx3_ee::s07::f_0026ba68,
};
const READ_VEC3: Callee = Callee {
    addr: 0x0026_ba48,
    f: ssx3_ee::s07::f_0026ba48,
};
const MARKER_BYTES: u32 = 0x28;
const TYPE_KEYS: u32 = 0x0044_5428;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// Convert the serialized type through the three native keys. On the USA
/// disc those keys are [-1, 0, 1], producing runtime types [0, 1, 2]. An
/// unknown key retains type zero, exactly as the original loader does.
fn relocate_type(c: &mut Cpu) {
    c.g[V0][0] = c.lw(r(c, S1).wrapping_add(4));
    c.g[T0][0] = sx(r(c, S3).wrapping_add(1));
    c.g[V0][0] = sx(r(c, S2).wrapping_add(r(c, V0)));
    c.sw(r(c, V0).wrapping_add(4), 0);
    c.g[V1][0] = c.lw(r(c, S6).wrapping_add(0x5428));
    c.g[V0][0] = c.lw(r(c, SP));
    let first_key = c.g[V1][0] == c.g[V0][0];
    c.g[A1][0] = 0;
    if first_key {
        c.cycles += 4;
        c.g[V0][0] = c.lw(r(c, S1).wrapping_add(4));
        c.g[V0][0] = sx(r(c, S2).wrapping_add(r(c, V0)));
        c.sw(r(c, V0).wrapping_add(4), 0);
        return;
    }

    c.cycles += 1;
    c.g[A1][0] = sx(r(c, A1).wrapping_add(1));
    loop {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A1][0] as i64) < 3) as u64;
        let exhausted = c.g[V0][0] == 0;
        c.g[V0][0] = sx(r(c, A1) << 2);
        if exhausted {
            return;
        }
        c.cycles += 5;
        c.g[A0][0] = c.lw(r(c, SP));
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S5)));
        c.g[V1][0] = c.lw(r(c, V0));
        if c.g[V1][0] == c.g[A0][0] {
            c.cycles += 4;
            c.mult(0, V0, S3, S4);
            c.g[V1][0] = c.lw(r(c, S1).wrapping_add(4));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
            c.sw(r(c, V0).wrapping_add(4), c.g[A1][0]);
            return;
        }
        c.g[A1][0] = sx(r(c, A1).wrapping_add(1));
        if event_due(c) {
            deliver_event_here(c, 0x0026_b994);
        }
    }
}

/// `int PathData_ReadMarkers(PathData* self, const void* source)` (0x26b880).
/// Bind the serialized 40-byte marker array in place. Each marker stores
/// id, authored type, position, direction, AIPath index and TrackPath index.
/// The two indices become native pointers at +0x24 and +0x20 respectively;
/// AIPath records are 0x40 bytes and TrackPath records are 0x3c bytes.
/// Returns the serialized byte count consumed, including the count word.
pub fn read_markers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x60);
        frame.save(c, S7, 0x20);
        frame.save(c, S2, 0x70);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S4, 0x50);
        c.g[S7][0] = c.g[S0][0];
        frame.save(c, S5, 0x40);
        c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
        frame.save(c, S6, 0x30);
        frame.save_ra(c, 0x10);
        c.g[S3][0] = 0;
        READ_INT.call(c, 0x0026_b8c4)?;
        c.g[V0][0] = c.lw(r(c, SP));
        c.sw(r(c, S1).wrapping_add(4), c.g[S0][0]);
        let nonempty = (c.g[V0][0] as i64) > 0;
        c.sw(r(c, S1), c.g[V0][0]);
        if nonempty {
            c.cycles += 5;
            c.g[S6][0] = 0x0044_0000;
            c.g[S4][0] = MARKER_BYTES as u64;
            c.g[S5][0] = sx(TYPE_KEYS);
            c.mult(0, S2, S3, S4);
            loop {
                c.cycles += 37;
                c.g[A0][0] = c.lw(r(c, S1).wrapping_add(4));
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
                c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S2)));
                READ_INT.call(c, 0x0026_b8fc)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S0][0];
                READ_INT.call(c, 0x0026_b908)?;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
                c.g[A0][0] = c.lw(r(c, S1).wrapping_add(4));
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx(r(c, S0).wrapping_add(0xc));
                c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S2)));
                c.g[A0][0] = sx(r(c, A0).wrapping_add(8));
                READ_VEC3.call(c, 0x0026_b924)?;
                c.g[A0][0] = c.lw(r(c, S1).wrapping_add(4));
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx(r(c, S0).wrapping_add(0xc));
                c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S2)));
                c.g[A0][0] = sx(r(c, A0).wrapping_add(0x14));
                READ_VEC3.call(c, 0x0026_b93c)?;
                c.g[A0][0] = sx(r(c, SP).wrapping_add(4));
                c.g[A1][0] = c.g[S0][0];
                READ_INT.call(c, 0x0026_b948)?;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = sx(r(c, SP).wrapping_add(8));
                READ_INT.call(c, 0x0026_b958)?;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(4));

                relocate_type(c);
                c.cycles += 21;
                c.mult(0, A2, S3, S4);
                c.g[V1][0] = c.lw(r(c, SP).wrapping_add(4));
                c.g[A1][0] = c.lw(r(c, S1).wrapping_add(4));
                c.g[A3][0] = 0x3c;
                c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0xc));
                c.g[V1][0] = sx(r(c, V1) << 6);
                c.g[S3][0] = c.g[T0][0];
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                c.g[A1][0] = sx(r(c, A2).wrapping_add(r(c, A1)));
                c.sw(r(c, A1).wrapping_add(0x24), c.g[V0][0]);
                c.g[A0][0] = c.lw(r(c, SP).wrapping_add(8));
                c.g[V0][0] = c.lw(r(c, S1).wrapping_add(4));
                c.mult(0, A0, A0, A3);
                c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0x14));
                c.g[A2][0] = sx(r(c, A2).wrapping_add(r(c, V0)));
                c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                c.sw(r(c, A2).wrapping_add(0x20), c.g[V1][0]);
                c.g[V0][0] = c.lw(r(c, S1));
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.mult(0, S2, S3, S4);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b8e8);
                }
            }
        }

        debug_start(c, r(c, S1));
        c.cycles += 12;
        c.g[V0][0] = sx(r(c, S0).wrapping_sub(r(c, S7)));
        frame.restore(c, S1, 0x80);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// Optional native start marker for focused, windowed development. The
/// serialized archives and normal start/recovery markers remain unchanged.
fn debug_start(c: &mut Cpu, paths: u32) {
    if crate::custom_tracks::active().is_none() { return }
    let Ok(value) = std::env::var("SSX3_DEBUG_START") else { return };
    let fields: Vec<_> = value.split(',').map(str::trim).collect();
    if fields.len() != 8 { return }
    let Some(values) = fields[..6].iter().map(|s| s.parse::<f32>().ok()).collect::<Option<Vec<_>>>() else { return };
    let (Ok(ai), Ok(race)) = (fields[6].parse::<u32>(), fields[7].parse::<u32>()) else { return };
    if !values.iter().all(|v| v.is_finite())
        || ai >= c.bus.read32(paths + 8) || race >= c.bus.read32(paths + 0x10) {
        return;
    }
    let length = values[3..].iter().map(|v| v * v).sum::<f32>().sqrt();
    if !length.is_finite() || length < 1e-6 { return }
    let count = c.bus.read32(paths);
    let entries = c.bus.read32(paths + 4);
    for i in 0..count {
        let marker = entries + i * MARKER_BYTES;
        if c.bus.read32(marker) != 0 || c.bus.read32(marker + 4) != 1 { continue }
        for axis in 0..3 {
            c.bus.write32(marker + 8 + axis * 4, values[axis as usize].to_bits());
            c.bus.write32(marker + 0x14 + axis * 4, (values[axis as usize + 3] / length).to_bits());
        }
        let race_path = c.bus.read32(paths + 0x14) + race * 0x3c;
        let ai_path = c.bus.read32(paths + 0xc) + ai * 0x40;
        c.bus.write32(marker + 0x20, race_path);
        c.bus.write32(marker + 0x24, ai_path);
        eprintln!("[tricky] debug start {:?}, AI path {ai}, race path {race}", &values[..3]);
        break;
    }
}
