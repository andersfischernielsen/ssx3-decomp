//! Staging module oc1492 (wave wave275): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1492`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_0258, lbl_3a0258_scale_store, "lbl_003A0258"),
    (0x003a_019c, lbl_3a019c_store_f1_a3_44, "lbl_003A019C"),
    (0x003a_01a4, lbl_3a01a4_store_f1_a3_48, "lbl_003A01A4"),
    (0x003a_0268, lbl_3a0268_store_f1_a3_60, "lbl_003A0268"),
    (0x003a_3968, lbl_3a3968_pack_field, "lbl_003A3968"),
    (0x003e_3208, ea_file_platform_open, "EAFile_PlatformOpen"),
    (
        0x0016_c518,
        camera_trigger_man_read_trigger_action,
        "CameraTriggerMan_ReadTriggerAction",
    ),
    (0x0031_75a0, app_load_progress_update, "App_LoadProgressUpdate"),
];

/// `lbl_003A0258` (0x3a0258): `f0 = f1 * *(float*)(gp - 0x24fc)`, stored at
/// `a3 + 0x28`, then the shared loop at 0x3a0274 (its tail call).
pub fn lbl_3a0258_scale_store(c: &mut Cpu) {
    c.cycles += 4;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x24fc));
    c.mul_s(0, 1, 0);
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x28));
    c.pc = 0x003a_0274;
    ee::s13::f_003a0274(c);
}

/// `lbl_003A019C` (0x3a019c): stores f1 at `a3 + 0x44`, then tail-calls the
/// loop at 0x3a0274.
pub fn lbl_3a019c_store_f1_a3_44(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0x44));
    c.pc = 0x003a_0274;
    ee::s13::f_003a0274(c);
}

/// `lbl_003A01A4` (0x3a01a4): stores f1 at `a3 + 0x48`, then tail-calls the
/// loop at 0x3a0274.
pub fn lbl_3a01a4_store_f1_a3_48(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0x48));
    c.pc = 0x003a_0274;
    ee::s13::f_003a0274(c);
}

/// `lbl_003A0268` (0x3a0268): stores f1 at `a3 + 0x60`, then tail-calls the
/// loop at 0x3a0274.
pub fn lbl_3a0268_store_f1_a3_60(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0x60));
    c.pc = 0x003a_0274;
    ee::s13::f_003a0274(c);
}

/// `lbl_003A3968` (0x3a3968): converts f1 to an integer, keeps its low 6 bits
/// at bits 13..18 of the word at `t0 + 0x14` (the other bits kept), then
/// tail-calls 0x3a39e0.
pub fn lbl_3a3968_pack_field(c: &mut Cpu) {
    c.cycles += 11;
    c.cvt_w_s(0, 1);
    c.mfc1(2, 0);
    let field = (c.g[T0][0] as u32).wrapping_add(0x14);
    c.g[A0][0] = c.lw(field);
    // All bits except 13..18 (0xfffffffffff80000 | 0x1fff).
    c.g[V1][0] = !0x0007_ffff;
    c.g[V1][0] = c.g[V1][0] | 0x1fff;
    c.g[V0][0] = c.g[V0][0] & 0x3f;
    c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 13);
    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
    c.sw(field, c.g[A0][0]);
    c.pc = 0x003a_39e0;
    ee::s13::f_003a39e0(c);
}

/// `EAFile_PlatformOpen(const char *path, int mode, u32 *handle)` (0x3e3208):
/// resolves the path to a device (kind 1 = the CD image, found through
/// `EAFile_CdLookup`, its handle tagged with device 1 in the top byte;
/// kind 2 = host, opened with `sceOpen` with the mode the flags select).
/// Returns 1 when a handle was stored, else 0. Other kinds return 0 without
/// writing `*handle`; a failed `sceOpen` stores 0.
pub fn ea_file_platform_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x140);
        frame.save(c, S0, 0x130);
        frame.save(c, S1, 0x120);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x110);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = sx(frame.at(c, 0x100));
        Callee {
            addr: 0x003e_3098,
            f: ee::s14::f_003e3098,
        }
        .call(c, 0x003e_322c)?;
        let kind_at = frame.at(c, 0x100);
        c.g[V1][0] = c.lw(kind_at);
        c.g[A0][0] = 0x1;
        c.g[V0][0] = 0x2;
        let out = c.g[S0][0] as u32;
        'out: {
            'dc: {
                'd8: {
                    'cc: {
                        if c.g[V1][0] == c.g[A0][0] {
                            // Kind 1: the CD image.
                            c.cycles += 6;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A2][0] = c.g[S0][0];
                            Callee {
                                addr: 0x003e_4508,
                                f: ee::s14::f_003e4508,
                            }
                            .call(c, 0x003e_324c)?;
                            let not_found = c.g[V0][0] == 0;
                            c.g[A0][0] = 0x0100_0000;
                            if not_found {
                                break 'dc;
                            }
                            c.cycles += 3;
                            c.g[V1][0] = c.lw(out);
                            c.g[V0][0] = 0x1;
                            break 'cc;
                        }
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = 0;
                            break 'out;
                        }
                        // Kind 2: the host file, mode from the low flag bits.
                        'ac: {
                            c.cycles += 3;
                            c.g[A1][0] = c.g[S1][0] & 0x7;
                            if c.g[A1][0] == c.g[A0][0] {
                                c.g[A1][0] = 0x1;
                                break 'ac;
                            }
                            c.cycles += 2;
                            if c.g[A1][0] == 0 {
                                c.g[A1][0] = 0x103;
                                break 'ac;
                            }
                            'a4: {
                                'a9c: {
                                    'a94: {
                                        c.cycles += 2;
                                        let t = c.g[A1][0] == c.g[V1][0];
                                        c.g[V0][0] = 0x6;
                                        if t {
                                            break 'a94;
                                        }
                                        c.cycles += 2;
                                        let t = c.g[A1][0] == c.g[V0][0];
                                        c.g[V1][0] = u64::MAX;
                                        if t {
                                            break 'a9c;
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = 0;
                                        break 'a4;
                                    }
                                    c.cycles += 2;
                                    c.g[A1][0] = 0x203;
                                    break 'ac;
                                }
                                c.cycles += 2;
                                c.g[A1][0] = 0x603;
                                break 'ac;
                            }
                            // Unknown flags: the kind is stored, result 0.
                            c.cycles += 2;
                            c.sw(out, c.g[V1][0]);
                            break 'out;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x0042_7d60, 0x003e_32b4)?;
                        c.g[A0][0] = c.g[V0][0];
                        let failed = (c.g[A0][0] as i64) < 0;
                        c.sw(out, c.g[A0][0]);
                        if failed {
                            break 'd8;
                        }
                        c.cycles += 3;
                        c.g[V1][0] = c.lw(kind_at);
                        c.g[V0][0] = 0x1;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 24);
                    }
                    // Handle found (kind 1 or 2): tag it with the device.
                    c.cycles += 3;
                    c.g[V1][0] = c.g[A0][0] | c.g[V1][0];
                    c.sw(out, c.g[V1][0]);
                    break 'out;
                }
                // sceOpen failed.
                c.cycles += 1;
                c.sw(out, 0);
            }
            // Not found.
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x130);
        frame.restore(c, S1, 0x120);
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    })
}

/// `CameraTriggerMan_ReadTriggerAction(cursor, actionOut, cookie)` (0x16c518):
/// reads one action (`CameraTrigger_ReadAction`); result 6 if none. The
/// action's type word then picks the bound-object reader (type 1, at
/// `action + 8`, its result returned) or the spline reader (type 2, at
/// `action + 8`, result 0). Other types return 0.
pub fn camera_trigger_man_read_trigger_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        'a: {
            'b: {
                'c: {
                    Callee {
                        addr: 0x0017_12b0,
                        f: ee::s02::f_001712b0,
                    }
                    .call(c, 0x0016_c53c)?;
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                        break 'c;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0x6;
                    break 'b;
                }
                c.cycles += 4;
                c.g[V0][0] = 0x1;
                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                'd: {
                    let not_bound = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2;
                    if not_bound {
                        break 'd;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.g[S2][0];
                    Callee {
                        addr: 0x0016_c4f0,
                        f: ee::s02::f_0016c4f0,
                    }
                    .call(c, 0x0016_c56c)?;
                    frame.restore(c, S0, 0x30);
                    break 'a;
                }
                // Type 2 (spline), or another type: v0 still holds 2 here.
                c.cycles += 2;
                let other = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if other {
                    break 'b;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                Callee {
                    addr: 0x0017_1d58,
                    f: ee::s02::f_00171d58,
                }
                .call(c, 0x0016_c588)?;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `App_LoadProgressUpdate(LoadTimer *t)` (0x3175a0): `t[0] = t[5] + t[6] *
/// expf((gp.int_1c - (int)t[4]) * gp.float_14 * t[7])`, with the offsets of
/// the generated body (the Ghidra pseudo-C agrees on the fields).
pub fn app_load_progress_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x1c));
        c.mtc1(2, 12);
        c.cvt_s_w(12, 12);
        c.mul_s(12, 12, 1);
        c.mul_s(12, 12, 0);
        call_ptr(c, 0x0040_d758, 0x0031_75dc)?;
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x18));
        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x14));
        c.mul_s(1, 1, 0);
        frame.restore_ra(c, 0);
        c.add_s(2, 2, 1);
        let progress = c.g[S0][0] as u32;
        c.swc1(2, progress);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The tail-call leaves: the float they store, the object in a3, and a
    /// loop bound of 0 so the shared loop at 0x3a0274 returns at once.
    fn store_leaf_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[RA][0] = sx(crate::difftest::RA);
        c.g[A3][0] = sx(DATA + 0x100);
        c.g[A1][0] = 0;
        c.f[0] = r.edge_f32();
        c.f[1] = r.edge_f32();
        c.bus.write32(GAME_GP - 0x24fc, r.edge_f32());
    }

    #[test]
    fn lbl_3a0258_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| store_leaf_setup(c, seed, 0x3a_0258);
        check(
            ee::s13::f_003a0258__generated,
            lbl_3a0258_scale_store,
            64,
            setup,
        );
    }

    #[test]
    fn lbl_3a019c_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| store_leaf_setup(c, seed, 0x3a_019c);
        check(
            ee::s13::f_003a019c__generated,
            lbl_3a019c_store_f1_a3_44,
            64,
            setup,
        );
    }

    #[test]
    fn lbl_3a01a4_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| store_leaf_setup(c, seed, 0x3a_01a4);
        check(
            ee::s13::f_003a01a4__generated,
            lbl_3a01a4_store_f1_a3_48,
            64,
            setup,
        );
    }

    #[test]
    fn lbl_3a0268_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| store_leaf_setup(c, seed, 0x3a_0268);
        check(
            ee::s13::f_003a0268__generated,
            lbl_3a0268_store_f1_a3_60,
            64,
            setup,
        );
    }

    /// The packed field: `t0 + 0x14` holds a word, f1 is any float (its
    /// integer part lands in bits 13..18), and the loop bound is 0.
    #[test]
    fn lbl_3a3968_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x3a_3968, 0);
            c.g[RA][0] = sx(crate::difftest::RA);
            c.g[T0][0] = sx(DATA + 0x100);
            c.g[A1][0] = 0;
            c.g[A2][0] = sx(DATA + 0x200);
            c.bus.write32(DATA + 0x114, r.next_u32());
            c.f[1] = r.edge_f32();
        };
        check(
            ee::s13::f_003a3968__generated,
            lbl_3a3968_pack_field,
            64,
            setup,
        );
    }

    // EAFile_PlatformOpen fixture: the three callees are gated fakes in
    // their override slots (the gate word lives in this machine's RAM).
    const PO_GATE: u32 = DATA + 0x1_0000;
    const PO_KIND: u32 = PO_GATE + 4;
    const PO_FOUND: u32 = PO_GATE + 8;
    const PO_CDVAL: u32 = PO_GATE + 0xc;
    const PO_OPEN: u32 = PO_GATE + 0x10;
    const PO_MAGIC: u32 = 0x1492_0a0a;

    static OC1492_PO_PREV_RESOLVE: AtomicUsize = AtomicUsize::new(0);
    static OC1492_PO_PREV_CD: AtomicUsize = AtomicUsize::new(0);
    static OC1492_PO_PREV_OPEN: AtomicUsize = AtomicUsize::new(0);

    /// `EAFile_ResolvePath`: writes the kind through the kind pointer (a2).
    fn oc1492_po_resolve(c: &mut Cpu) {
        if c.bus.read32(PO_GATE) != PO_MAGIC {
            return fakes::fall_back(
                c,
                &OC1492_PO_PREV_RESOLVE,
                ee::s14::f_003e3098__generated,
            );
        }
        let kind = c.bus.read32(PO_KIND);
        let kind_ptr = c.g[A2][0] as u32;
        c.bus.write32(kind_ptr, kind);
        c.pc = c.g[RA][0] as u32;
    }

    /// `EAFile_CdLookup`: result in v0, the handle through the out pointer (a2).
    fn oc1492_po_cd_lookup(c: &mut Cpu) {
        if c.bus.read32(PO_GATE) != PO_MAGIC {
            return fakes::fall_back(c, &OC1492_PO_PREV_CD, ee::s14::f_003e4508__generated);
        }
        let found = c.bus.read32(PO_FOUND);
        let handle = c.bus.read32(PO_CDVAL);
        let out = c.g[A2][0] as u32;
        c.bus.write32(out, handle);
        c.g[V0][0] = found as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// `sceOpen` through the function table: the forced result in v0.
    fn oc1492_po_open(c: &mut Cpu) {
        if c.bus.read32(PO_GATE) != PO_MAGIC {
            return fakes::fall_back(c, &OC1492_PO_PREV_OPEN, ee::s15::f_00427d60__generated);
        }
        let ret = c.bus.read32(PO_OPEN);
        c.g[V0][0] = sx(ret);
        c.pc = c.g[RA][0] as u32;
    }

    fn platform_open_install() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E3098),
            oc1492_po_resolve,
            &OC1492_PO_PREV_RESOLVE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E4508),
            oc1492_po_cd_lookup,
            &OC1492_PO_PREV_CD,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00427D60),
            oc1492_po_open,
            &OC1492_PO_PREV_OPEN,
        );
    }

    /// Kind 1/2/other, every flag value, found or not, sceOpen failing or not.
    #[test]
    fn ea_file_platform_open_matches() {
        platform_open_install();
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3e_3208, 0);
            c.bus.write32(PO_GATE, PO_MAGIC);
            c.bus.write32(PO_KIND, [1, 2, 0, 3][(seed % 4) as usize]);
            c.bus.write32(PO_FOUND, (seed / 32) % 2);
            c.bus.write32(PO_CDVAL, 0x0040_0000 + seed);
            c.bus
                .write32(PO_OPEN, if seed % 5 == 0 { u32::MAX } else { 0x40 + seed });
            c.bus.write32(DATA + 0x500, 0x5a5a_0000 + seed);
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = ((seed / 4) % 8) as u64;
            c.g[A2][0] = sx(DATA + 0x500);
        };
        check(
            ee::s14::f_003e3208__generated,
            ea_file_platform_open,
            64,
            setup,
        );
    }

    // CameraTriggerMan_ReadTriggerAction fixture: the three callees are gated
    // fakes; the action's type word decides the path.
    const RT_GATE: u32 = DATA + 0x1_2000;
    const RT_ACTION: u32 = RT_GATE + 4;
    const RT_BOUND: u32 = RT_GATE + 8;
    const RT_MAGIC: u32 = 0x1492_0b0b;

    static OC1492_RT_PREV_ACTION: AtomicUsize = AtomicUsize::new(0);
    static OC1492_RT_PREV_BOUND: AtomicUsize = AtomicUsize::new(0);
    static OC1492_RT_PREV_SPLINE: AtomicUsize = AtomicUsize::new(0);

    /// `CameraTrigger_ReadAction`: the forced result in v0.
    fn oc1492_rt_action(c: &mut Cpu) {
        if c.bus.read32(RT_GATE) != RT_MAGIC {
            return fakes::fall_back(c, &OC1492_RT_PREV_ACTION, ee::s02::f_001712b0__generated);
        }
        let v = c.bus.read32(RT_ACTION);
        c.g[V0][0] = v as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// `CameraTriggerMan_ReadBoundObjAction`: the forced result in v0.
    fn oc1492_rt_bound(c: &mut Cpu) {
        if c.bus.read32(RT_GATE) != RT_MAGIC {
            return fakes::fall_back(c, &OC1492_RT_PREV_BOUND, ee::s02::f_0016c4f0__generated);
        }
        let v = c.bus.read32(RT_BOUND);
        c.g[V0][0] = v as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// `CameraTrigger_ReadSpline`: nothing to write in this fixture.
    fn oc1492_rt_spline(c: &mut Cpu) {
        if c.bus.read32(RT_GATE) != RT_MAGIC {
            return fakes::fall_back(c, &OC1492_RT_PREV_SPLINE, ee::s02::f_00171d58__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    #[test]
    fn camera_trigger_man_read_trigger_action_matches() {
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_001712B0),
            oc1492_rt_action,
            &OC1492_RT_PREV_ACTION,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C4F0),
            oc1492_rt_bound,
            &OC1492_RT_PREV_BOUND,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00171D58),
            oc1492_rt_spline,
            &OC1492_RT_PREV_SPLINE,
        );
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x16_c518, 0);
            c.bus.write32(RT_GATE, RT_MAGIC);
            c.bus.write32(RT_ACTION, if seed % 3 == 0 { 0 } else { 1 });
            c.bus.write32(RT_BOUND, 0x7000 + seed);
            // *actionOut = the action; the action's type word picks the path.
            c.bus.write32(DATA + 0x100, DATA + 0x200);
            c.bus.write32(DATA + 0x200, [1, 2, 3][((seed / 3) % 3) as usize]);
            c.g[A0][0] = sx(DATA + 0x600);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = seed as u64;
        };
        check(
            ee::s02::f_0016c518__generated,
            camera_trigger_man_read_trigger_action,
            64,
            setup,
        );
    }

    /// The progress value from the timer's float fields and the global int.
    #[test]
    fn app_load_progress_update_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x3175_a0, 0);
            c.g[A0][0] = sx(DATA + 0x100);
            c.bus.write32(GAME_GP + 0x2a74, DATA + 0x400);
            c.bus.write32(DATA + 0x400 + 0x1c, r.next_u32());
            c.bus.write32(DATA + 0x400 + 0x14, r.edge_f32());
            c.bus.write32(DATA + 0x100 + 0x10, r.next_u32());
            c.bus.write32(DATA + 0x100 + 0x14, r.edge_f32());
            c.bus.write32(DATA + 0x100 + 0x18, r.edge_f32());
            c.bus.write32(DATA + 0x100 + 0x1c, r.edge_f32());
        };
        check(
            ee::s10::f_003175a0__generated,
            app_load_progress_update,
            64,
            setup,
        );
    }
}
