//! Retail object-contact dispatch into the world's kind-2 scripts.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0030_a060, on_object_trigger, "WScriptMan_OnObjectTrigger")];

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `OnObjectTrigger(manager, trigger_b, trigger_a, rider_interface)`.
/// `trigger_a+0x50` supplies the contacted instance. Temporarily install
/// the rider and contact in the script manager, run instance script kind 2,
/// notify a newly attached WorldObj, then clear the transient contact.
pub fn on_object_trigger(c: &mut Cpu) {
    trace_contact(c);
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0x30);
        c.g[V1][0] = c.lw(r(c, S3));
        c.g[S4][0] = c.lw(r(c, S2));
        c.g[A0][0] = c.lh(at(c, V1, 0x38));
        c.g[V0][0] = c.lw(at(c, V1, 0x3c));
        let get_rider = r(c, V0);
        c.g[A0][0] = sx(r(c, S3).wrapping_add(r(c, A0)));
        call_ptr(c, get_rider, 0x0030_a0a4)?;
        c.sw(r(c, S2), c.g[V0][0]);
        c.g[V0][0] = c.lw(at(c, S1, 0x50));
        c.sw(at(c, S2, 0x294), c.g[S0][0]);
        c.sw(at(c, S2, 0x290), c.g[V0][0]);
        c.sw(at(c, S2, 0x298), c.g[S1][0]);
        c.g[V1][0] = c.lw(r(c, S3));
        c.g[A0][0] = c.lh(at(c, V1, 0x40));
        c.g[V0][0] = c.lw(at(c, V1, 0x44));
        let rider_state = r(c, V0);
        c.g[A0][0] = sx(r(c, S3).wrapping_add(r(c, A0)));
        call_ptr(c, rider_state, 0x0030_a0cc)?;
        let queue_event = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x848));

        'contact: {
            if queue_event {
                c.cycles += 5;
                c.g[A1][0] = 4;
                c.g[V0][0] = c.lw(at(c, A0, 0x84));
                c.g[V1][0] = c.lw(at(c, V0, 0x214));
                let unrestricted_mode = c.g[V1][0] == c.g[A1][0];
                c.g[V0][0] = sx(r(c, S3).wrapping_sub(0x6c0));
                if !unrestricted_mode {
                    c.cycles += 4;
                    if c.g[S3][0] == 0 {
                        c.g[V0][0] = 0;
                    }
                    c.g[V1][0] = c.lw(at(c, V0, 0x480));
                    if c.g[V1][0] != 0 {
                        trace_skip(c, "rider+480");
                        c.sw(r(c, S2), c.g[S4][0]);
                        break 'contact;
                    }
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw(at(c, S2, 0x290));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.lw(at(c, V0, 0x78));
                call_ptr(c, 0x0030_97e0, 0x0030_a108)?;
            }

            c.cycles += 8;
            c.g[A1][0] = c.lw(at(c, S2, 0x290));
            c.g[A2][0] = 2;
            c.g[A0][0] = c.lw(at(c, S2, 0x28c));
            call_ptr(c, 0x003a_6b78, 0x0030_a118)?;
            trace_script(c);
            c.g[A1][0] = sx(at(c, SP, 0x24));
            c.g[V1][0] = !c.g[V0][0];
            let no_script = c.g[V1][0] == 0;
            c.sw(at(c, SP, 0x24), c.g[V0][0]);
            let mut direct_call = false;
            if !no_script {
                c.cycles += 3;
                c.g[V0][0] = c.lw(at(c, S2, 0x2a4));
                let mut nested = c.g[V0][0] != 0;
                if !nested {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, S2, 0x2a8));
                    nested = c.g[V0][0] != 0;
                }
                if !nested {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0030_9c88, 0x0030_a148)?;
                    c.g[V0][0] = c.lw(at(c, S2, 0x290));
                    direct_call = true;
                } else {
                    c.cycles += 14;
                    c.g[A0][0] = c.lw(at(c, S2, 0x28c));
                    call_ptr(c, 0x003a_6d18, 0x0030_a158)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x0022_6600, 0x0030_a168)?;
                    c.g[A0][0] = sx(at(c, SP, 0x20));
                    c.sw(at(c, SP, 0x18), 0);
                    c.g[S3][0] = c.g[A0][0];
                    c.sw(at(c, SP, 0x10), 0);
                    call_ptr(c, 0x0022_24a8, 0x0030_a17c)?;
                    c.g[V0][0] = c.lw(at(c, S2, 0x2a8));
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = c.lw(at(c, S2, 0x2a4));
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw(at(c, V0, 0x3c));
                    }
                    c.cycles += 3;
                    c.g[S1][0] = c.lw(at(c, V0, 0x1c));
                    let have_table = c.g[S1][0] != 0;
                    c.g[V0][0] = sx(at(c, SP, 0x10));
                    if have_table {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[SP][0];
                        c.g[A3][0] = c.g[V0][0];
                        c.g[S0][0] = c.g[V0][0];
                        call_ptr(c, 0x0022_24b8, 0x0030_a1b0)?;
                        c.g[A0][0] = c.g[S0][0];
                    } else {
                        c.cycles += 18;
                        c.g[A1][0] = 0x0049_0000;
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A3][0] = 0;
                        c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x6728));
                        c.g[A0][0] = 4;
                        call_ptr(c, 0x0031_7d70, 0x0030_a1d0)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 8;
                        call_ptr(c, 0x0022_4da0, 0x0030_a1dc)?;
                        c.g[S1][0] = c.g[V0][0];
                        c.g[S0][0] = sx(at(c, SP, 0x10));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.g[SP][0];
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A3][0] = c.g[S0][0];
                        call_ptr(c, 0x0022_24b8, 0x0030_a1f8)?;
                        let no_table = c.g[S1][0] == 0;
                        c.g[A0][0] = c.g[S1][0];
                        if !no_table {
                            c.cycles += 2;
                            c.g[A1][0] = 3;
                            call_ptr(c, 0x0022_4df0, 0x0030_a208)?;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S0][0];
                    }
                    c.cycles += 2;
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0022_5b90, 0x0030_a214)?;
                }
            }
            if !direct_call {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c, S2, 0x290));
            }

            c.cycles += 3;
            c.g[A1][0] = c.lw(at(c, V0, 0xc));
            if c.g[A1][0] == 0 {
                c.sw(r(c, S2), c.g[S4][0]);
            } else {
                c.cycles += 8;
                c.g[V0][0] = c.lw(at(c, A1, 0xc));
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(AT, 12);
                c.g[A0][0] = c.lh(at(c, V0, 0x148));
                c.g[V1][0] = c.lw(at(c, V0, 0x14c));
                let after_trigger = r(c, V1);
                c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                call_ptr(c, after_trigger, 0x0030_a240)?;
                c.sw(r(c, S2), c.g[S4][0]);
            }
        }

        c.cycles += 11;
        c.sw(at(c, S2, 0x290), 0);
        c.sw(at(c, S2, 0x294), 0);
        c.sw(at(c, S2, 0x298), 0);
        for (register, offset) in [(S0, 0x80), (S1, 0x70), (S2, 0x60), (S3, 0x50), (S4, 0x40)] {
            frame.restore(c, register, offset);
        }
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });
}

fn trace_enabled() -> bool {
    crate::custom_tracks::active().is_some() && std::env::var_os("SSX3_OBJECT_TRACE").is_some()
}

fn trace_contact(c: &mut Cpu) {
    if !trace_enabled() {
        return;
    }
    let contact = r(c, A2);
    let instance = c.bus.read32(contact.wrapping_add(0x50));
    if instance == 0 {
        return;
    }
    let handle = c.bus.read32(instance.wrapping_add(0x78));
    if handle & 255 != 49 {
        return;
    }
    static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) >= 32 {
        return;
    }
    let property = c.bus.read32(instance.wrapping_add(0x88));
    let flags = c.bus.read32(instance.wrapping_add(8));
    let mode = c.bus.read32(property);
    let script = c.bus.read32(property.wrapping_add(8));
    eprintln!("[object trigger] instance={instance:08x} handle={handle:08x} flags={flags:08x} property={property:08x} mode={mode} binding={script:08x} rider={:08x}", r(c, A3));
}

fn trace_script(c: &mut Cpu) {
    if !trace_enabled() {
        return;
    }
    let instance = c.bus.read32(at(c, S2, 0x290));
    if instance == 0 {
        return;
    }
    let handle = c.bus.read32(instance.wrapping_add(0x78));
    if handle & 255 != 49 {
        return;
    }
    static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) >= 32 {
        return;
    }
    eprintln!(
        "[object script] handle={handle:08x} kind=2 function={:08x}",
        r(c, V0)
    );
}

fn trace_skip(c: &mut Cpu, reason: &str) {
    if !trace_enabled() {
        return;
    }
    let instance = c.bus.read32(at(c, S2, 0x290));
    if instance == 0 || c.bus.read32(instance.wrapping_add(0x78)) & 255 != 49 {
        return;
    }
    static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 32 {
        eprintln!("[object trigger skipped] {reason}");
    }
}
