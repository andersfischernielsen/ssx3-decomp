//! Native Luno adapters for imported Tricky reset and teleport commands.
//!
//! These use unused builtin IDs, not new VM instructions. Rider placement,
//! state transitions, camera and projection stay in the SSX 3 engine.

use crate::call::{call_ptr, Flow};
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::Cpu;

pub(crate) fn rider(c: &mut Cpu) -> Option<(u32, i32, u32)> {
    let manager = c.bus.read32((c.g[GP][0] as u32).wrapping_add(0xce8));
    if manager == 0 { return None }
    let index = c.bus.read32(manager) as i32;
    let app = c.bus.read32(0x004a_28a8);
    if app == 0 || index < 0 { return None }
    let module = c.bus.read32(app + 0x84);
    if module == 0 { return None }
    let ai = c.bus.read32(module + 0xc);
    if ai == 0 || index as u32 >= c.bus.read32(ai + 0x78) { return None }
    let address = c.bus.read32(ai + 0x28 + index as u32 * 4);
    (address != 0).then_some((manager, index, address))
}

fn tracing() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("SSX3_OBJECT_TRACE").is_some())
}

/// Source recovery requests respect the current path's NoReset regions.
/// Tricky's rider entry118f10/119be0/119c90 checks196858 before scheduling
/// recovery. Native ForceReset deliberately skips that policy, so keep it
/// at this imported-script boundary. Ranges are the native Path event list;
/// runtime13 is stored101 after the Tricky104 conversion.
fn scripted_reset_allowed(c: &mut Cpu, rider: u32) -> bool {
    if f32::from_bits(c.bus.read32(rider + 0x470)) >= 0. { return false }
    let owner = c.bus.read32(rider + 0x77c);
    if owner != 0 && c.bus.read32(owner + 0xde0) == 3 { return false }
    let path = c.bus.read32(rider + 0xab8);
    if path == 0 { return true }
    let count = c.bus.read32(path);
    let events = c.bus.read32(path + 4);
    let distance = f32::from_bits(c.bus.read32(rider + 0x4c4));
    for index in 0..count {
        let event = events + index * 16;
        if c.bus.read32(event) == 13 {
            let start = f32::from_bits(c.bus.read32(event + 8));
            let end = f32::from_bits(c.bus.read32(event + 12));
            if start <= distance && distance <= end { return false }
        }
    }
    true
}

pub(crate) fn resident_instance(c: &mut Cpu, rid: u32) -> Option<u32> {
    let world = c.bus.read32((c.g[GP][0] as u32).wrapping_add(0x16c8));
    if world == 0 { return None }
    let cache = c.bus.read32(world);
    if cache == 0 { return None }
    let banks = c.bus.read32(cache + 8);
    if banks == 0 { return None }
    let bank = c.bus.read32(banks + (rid & 255) * 4);
    if bank == 0 { return None }
    let entries = c.bus.read32(bank + 0x1c);
    if entries == 0 || rid >> 8 >= c.bus.read32(bank + 0x20) { return None }
    let reference = c.bus.read32(entries + (rid >> 8) * 4);
    let instance = (reference >> 8) << 2;
    (reference & 255 != 0 && instance != 0
        && instance != (c.g[GP][0] as u32).wrapping_add(0x16e0)).then_some(instance)
}

/// Tricky uses local +Y for arrival and a three-metre six-rider spread.
/// Native Rider_PlaceAtMatrix uses local +X: build its temporary frame rather
/// than changing the shared exit marker or turning the rider sideways.
fn teleport_frame(c: &mut Cpu, index: i32, matrix: u32) -> bool {
    let arguments = c.g[A2][0] as u32;
    if c.g[A1][0] != 1 || arguments == 0 || c.bus.read32(arguments) != 0 { return false }
    let rid = c.bus.read32(arguments + 4);
    let Some(instance) = resident_instance(c, rid) else { return false };
    let source: [[f32; 4]; 4] = std::array::from_fn(|row| {
        std::array::from_fn(|col| f32::from_bits(c.bus.read32(instance + 0x10 + row as u32 * 16 + col as u32 * 4)))
    });
    let offsets = [(-1., 0.), (-1., 1.), (0., 1.), (1., 1.), (1., 0.), (0., 0.)];
    let (x, y) = offsets.get(index as usize).copied().unwrap_or((0., 0.));
    let mut frame = [source[1], source[0].map(|value| -value), source[2], source[3]];
    for axis in 0..3 {
        frame[3][axis] += 300. * (x * source[0][axis] + y * source[1][axis]);
    }
    for (row, values) in frame.iter().enumerate() {
        for (col, value) in values.iter().enumerate() {
            c.bus.write32(matrix + row as u32 * 16 + col as u32 * 4, value.to_bits());
        }
    }
    true
}

/// Handle an imported builtin at the regular native VM call boundary.
/// Each adapter makes at most one guest call to that same return site. Its
/// output is initialized first, so the VM's existing unwind continuation is
/// also valid if the native action yields.
pub fn builtin(c: &mut Cpu, id: u32, back: u32) -> Option<Flow> {
    if !(111..=135).contains(&id) {
        return None;
    }
    let out = c.g[A0][0] as u32;
    for offset in [0, 4, 8] { c.bus.write32(out + offset, 0); }
    // These slots belong to imported scripts even while a course retires.
    // Retail has no defined builtins at these IDs. Do not fall through to
    // a null target or adjacent data if a late callback sees no active course.
    if crate::custom_tracks::active().is_none() { return Some(Ok(())) }
    if id == 135 { crate::custom_movies::register(c); return Some(Ok(())); }
    if id == 134 { crate::custom_crowds::register(c); return Some(Ok(())); }
    if id == 133 { return Some(crate::custom_animation_debounce::begin(c, back)); }
    if id == 132 { return Some(crate::custom_force_overlap::begin(c, back)); }
    if id == 131 { return Some(crate::custom_spline_movers::begin(c, back)); }
    if id == 130 { return Some(crate::custom_flags::begin(c, back)); }
    if id == 129 { crate::custom_fences::register(c); return Some(Ok(())); }
    if id == 128 { return Some(crate::custom_fences::begin(c, back)); }
    if id == 127 { return Some(crate::custom_collision_emitters::begin(c, back)); }
    if id == 124 { crate::custom_cracked_glass::contact(c); return Some(Ok(())); }
    if id == 125 { crate::custom_cracked_glass::attach(c); return Some(Ok(())); }
    if id == 126 { crate::custom_cracked_glass::message(c); return Some(Ok(())); }
    if id == 123 {
        crate::custom_initial_debounce::register(c);
        return Some(Ok(()));
    }
    if id == 122 {
        return Some(crate::custom_roller_runtime::begin(c, back));
    }
    if id == 121 {
        crate::custom_script_materials::register(c);
        return Some(Ok(()));
    }
    if id == 120 {
        return Some(crate::custom_script_predicates::begin(c, back));
    }
    if id == 115 {
        crate::custom_script_queue::schedule(c);
        return Some(Ok(()));
    }
    if id == 116 {
        let allowed = crate::custom_script_queue::can_start(c);
        c.bus.write32(out, allowed as u32);
        c.bus.write32(out + 8, 1);
        return Some(Ok(()));
    }
    if id == 117 {
        crate::custom_script_queue::detach(c);
        return Some(Ok(()));
    }
    if id == 118 {
        crate::custom_script_queue::restore_scope(c);
        return Some(Ok(()));
    }
    if id == 119 {
        crate::custom_rail_rules::builtin(c);
        return Some(Ok(()));
    }
    let Some((manager, index, address)) = rider(c) else { return Some(Ok(())) };
    Some(match id {
        111 => {
            if !scripted_reset_allowed(c, address) { return Some(Ok(())) }
            if tracing() {
                static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
                if COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 32 {
                    let instance = c.bus.read32(manager + 0x290);
                    let rid = if instance == 0 { 0 } else { c.bus.read32(instance + 0x78) };
                    eprintln!("[tricky] native reset from instance {rid:08x}, rider {index} ({address:08x})");
                }
            }
            c.g[A0][0] = sx(address);
            call_ptr(c, 0x0012_35f8, back)
        }
        112 => {
            let matrix = (c.g[SP][0] as u32) + 0x10;
            if !teleport_frame(c, index, matrix) { return Some(Ok(())) }
            c.g[A0][0] = sx(address);
            c.g[A1][0] = sx(matrix);
            call_ptr(c, 0x0012_34d0, back)
        }
        113 => {
            let cursor = c.bus.read32(address + 0xac0);
            if cursor != 0 { c.bus.write32(cursor + 0x14, u32::MAX); }
            c.g[A0][0] = sx(address);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0011_27f0, back)
        }
        114 => {
            let path = c.bus.read32(address + 0xab4);
            if path == 0 { return Some(Ok(())) }
            let scratch = c.g[SP][0] as u32;
            c.g[A0][0] = sx(path);
            c.g[A1][0] = sx(address + 0x110);
            c.g[A2][0] = sx(scratch + 0x10); // temporary nearest vec4
            c.g[A3][0] = sx(scratch + 0x20); // temporary off-path distance
            c.g[T0][0] = sx(c.bus.read32(address + 0xac0));
            c.g[T1][0] = 1;
            call_ptr(c, 0x0026_a638, back)
        }
        _ => unreachable!(),
    })
}

/// Complete pure-memory work after a native call, including resumed calls.
/// Priming both distance fields prevents a teleport from collecting the time
/// gates/splits along the skipped section of the old path.
pub fn after_builtin(c: &mut Cpu, id: u32, back: u32) -> Flow {
    if id == 132 { return crate::custom_force_overlap::after(c, back) }
    if id == 131 { return crate::custom_spline_movers::after(c, back) }
    if id == 128 { return crate::custom_fences::after(c, back) }
    if id == 127 { return crate::custom_collision_emitters::after(c) }
    if id == 122 { return crate::custom_roller_runtime::after(c, back) }
    if id == 120 && crate::custom_tracks::active().is_some() {
        crate::custom_script_predicates::finish(c);
        return Ok(());
    }
    if !matches!(id, 112 | 114) || crate::custom_tracks::active().is_none() { return Ok(()) }
    let Some((_, index, address)) = rider(c) else { return Ok(()) };
    if id == 114 {
        let path = c.bus.read32(address + 0xab4);
        if path == 0 { return Ok(()) }
        let remaining = FpuF32(c.bus.read32(path + 0x38));
        let distance = (remaining - FpuF32(c.f[0])).0;
        c.bus.write32(address + 0x4d0, distance);
        c.bus.write32(address + 0x4d4, distance);
        c.bus.write32(address + 0x59c, 0);
        crate::custom_course_rules::relocated(address);
        if tracing() {
            eprintln!("[tricky] teleport path primed for rider {index}: {path:08x}, distance {}", f32::from_bits(distance));
        }
    } else if tracing() {
        let position: [f32; 3] = std::array::from_fn(|i| f32::from_bits(c.bus.read32(address + 0x110 + i as u32 * 4)));
        eprintln!("[tricky] native teleport rider {index}: {position:?}");
    }
    Ok(())
}
