//! Staging wave 211 (oc299): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

// ---------------------------------------------------------------------------
// cDynamicSprayFX_UpdateEmitterEnables (0x2e2550)

/// `cDynamicSprayFX` -> its rider (`*self`).
const SPRAY_RIDER: u32 = 0x0;
/// `cDynamicSprayFX` -> its emitter block (the enable flags live inside it).
const SPRAY_EMITTERS: u32 = 0x28;
/// `cDynamicSprayFX` -> the "spray on" flag of the current frame.
const SPRAY_ACTIVE: u32 = 0x124;
/// `cDynamicSprayFX` -> copy of the rider's spray mode.
const SPRAY_MODE: u32 = 0x128;
/// Rider -> spray mode (0 = full, 1 = reduced, else board-only).
const RIDER_SPRAY_MODE: u32 = 0x898;
/// Rider -> forced-spray flag (keeps emitter 0x9b4 on in mode 0).
const RIDER_FORCE_SPRAY: u32 = 0xac4;

/// Store `v` into the enable word at `off` of the emitter block, reloading
/// the block pointer into `reg` first as the compiled code does.
#[inline(always)]
fn spray_enable_store(c: &mut Cpu, fx: Addr, reg: usize, off: u32, v: u64) {
    c.g[reg][0] = fx.off(SPRAY_EMITTERS).lw(c);
    let block = c.g[reg][0] as u32;
    Addr(block.wrapping_add(off)).sw(c, v);
}

/// Store `fx->active` into the enable word at `off`, loading the block into
/// `blk` and the flag into `val` in the order given by `flag_first`.
#[inline(always)]
fn spray_enable_copy_active(c: &mut Cpu, fx: Addr, blk: usize, val: usize, off: u32, flag_first: bool) {
    if flag_first {
        c.g[val][0] = fx.off(SPRAY_ACTIVE).lw(c);
        c.g[blk][0] = fx.off(SPRAY_EMITTERS).lw(c);
    } else {
        c.g[blk][0] = fx.off(SPRAY_EMITTERS).lw(c);
        c.g[val][0] = fx.off(SPRAY_ACTIVE).lw(c);
    }
    let block = c.g[blk][0] as u32;
    let v = c.g[val][0];
    Addr(block.wrapping_add(off)).sw(c, v);
}

/// `void cDynamicSprayFX::UpdateEmitterEnables()` (0x2e2550): copies the
/// rider's spray mode to `self+0x128` and switches the board-spray emitters
/// on/off by mode: mode 0 enables 0xbc4/0xdd4/0x174, 0x9b4 if active or
/// forced, and the rest follow `active`; mode 1 is the same with 0x9b4 and
/// 0x1404 off; any other mode keeps only 0xbc4 (and 0xdd4 = active).
pub fn dynamic_spray_fx_update_emitter_enables(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] = c.g[A0][0];
    let fx = Addr(c.g[A2][0] as u32);
    c.g[V0][0] = fx.off(SPRAY_RIDER).lw(c);
    let rider = c.g[V0][0] as u32;
    c.g[A0][0] = Addr(rider.wrapping_add(RIDER_SPRAY_MODE)).lw(c);
    let mode = c.g[A0][0];
    fx.off(SPRAY_MODE).sw(c, mode);
    if mode == 0 {
        // Mode 0: everything follows `active`; 0x9b4 also when forced.
        c.cycles += 10;
        c.g[A0][0] = 1;
        spray_enable_store(c, fx, V1, 0xbc4, 1);
        spray_enable_store(c, fx, V0, 0xdd4, 1);
        spray_enable_store(c, fx, V1, 0x174, 1);
        c.g[V0][0] = fx.off(SPRAY_ACTIVE).lw(c);
        c.g[A1][0] = 0;
        let mut on = c.g[V0][0] != 0;
        if !on {
            c.cycles += 4;
            c.g[V0][0] = fx.off(SPRAY_RIDER).lw(c);
            let rider = c.g[V0][0] as u32;
            c.g[V1][0] = Addr(rider.wrapping_add(RIDER_FORCE_SPRAY)).lw(c);
            on = c.g[V1][0] != 0;
        }
        if on {
            c.cycles += 2;
            c.g[A1][0] = 1;
        }
        c.g[V0][0] = fx.off(SPRAY_EMITTERS).lw(c);
        c.cycles += 20;
        let block = c.g[V0][0] as u32;
        let a1 = c.g[A1][0];
        Addr(block.wrapping_add(0x9b4)).sw(c, a1);
        spray_enable_copy_active(c, fx, V1, V0, 0x384, false);
        spray_enable_copy_active(c, fx, A0, V0, 0x594, false);
        spray_enable_copy_active(c, fx, V1, V0, 0x7a4, false);
        spray_enable_copy_active(c, fx, A0, V0, 0xfe4, false);
        spray_enable_copy_active(c, fx, V1, V0, 0x11f4, true);
        spray_enable_copy_active(c, fx, V0, A0, 0x1404, true);
        ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 1;
    if mode == 1 {
        // Mode 1: like mode 0, but 0x9b4 and 0x1404 off.
        c.cycles += 23;
        spray_enable_store(c, fx, V0, 0xbc4, mode);
        spray_enable_store(c, fx, V1, 0xdd4, mode);
        spray_enable_store(c, fx, V0, 0x174, mode);
        spray_enable_copy_active(c, fx, V1, V0, 0x384, false);
        spray_enable_copy_active(c, fx, A0, V0, 0x594, false);
        spray_enable_copy_active(c, fx, V1, V0, 0x7a4, false);
        spray_enable_copy_active(c, fx, A0, V0, 0xfe4, true);
        spray_enable_copy_active(c, fx, A1, V0, 0x11f4, true);
        spray_enable_store(c, fx, V1, 0x9b4, 0);
    } else {
        // Any other mode: only 0xbc4 on, 0xdd4 = active, the rest off.
        c.cycles += 2;
        c.cycles += 18;
        spray_enable_store(c, fx, V1, 0xbc4, 1);
        spray_enable_copy_active(c, fx, A0, V0, 0xdd4, true);
        spray_enable_store(c, fx, V1, 0x384, 0);
        spray_enable_store(c, fx, V0, 0x174, 0);
        spray_enable_store(c, fx, V1, 0x594, 0);
        spray_enable_store(c, fx, V0, 0x7a4, 0);
        spray_enable_store(c, fx, V1, 0xfe4, 0);
        spray_enable_store(c, fx, V0, 0x9b4, 0);
        spray_enable_store(c, fx, V1, 0x11f4, 0);
    }
    c.cycles += 3;
    spray_enable_store(c, fx, V0, 0x1404, 0);
    ret(c);
}

// ---------------------------------------------------------------------------
// BankMonitorInstance_UpdateDistance (0x2abd48)

/// Listener object -> its position-source vtable (slot 0x30/0x34: distance²).
const LISTENER_VTABLE: u32 = 0x8d8;
/// BankMonitorInstance -> distance to the listener.
const BANK_MONITOR_DISTANCE: u32 = 0x5c;

/// `void BankMonitorInstance::UpdateDistance(Listener* l)` (0x2abd48): asks
/// the listener's virtual (vtable slot 0x30/0x34, `this` adjusted by the
/// slot's delta) for the squared distance to this instance (with a 0x20-byte
/// scratch buffer on the stack) and stores its square root at `self+0x5c`.
pub fn bank_monitor_instance_update_distance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[A2][0] = c.g[SP][0];
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        let listener = c.g[A3][0] as u32;
        c.g[V1][0] = Addr(listener.wrapping_add(LISTENER_VTABLE)).lw(c);
        let vt = Addr(c.g[V1][0] as u32);
        c.g[A0][0] = vt.off(0x30).lh(c);
        c.g[V0][0] = vt.off(0x34).lw(c);
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx(listener.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x002a_bd78)?;
        let dist = crate::ps2f::FpuF32(c.f[0]).sqrt();
        c.f[0] = dist.0;
        frame.restore_ra(c, 0x20);
        let this = c.g[S0][0] as u32;
        Addr(this.wrapping_add(BANK_MONITOR_DISTANCE)).write_fpu(c, dist);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderMetrix_BankCombo (0x117718)

/// RiderMetrix -> tricks in the running combo.
const METRIX_COMBO_COUNT: u32 = 0x9c;
/// RiderMetrix -> points of the running combo.
const METRIX_COMBO_POINTS: u32 = 0xa0;
/// RiderMetrix -> tricks banked over all combos.
const METRIX_BANKED_TRICKS: u32 = 0x134;
/// RiderMetrix -> longest combo (tricks).
const METRIX_BEST_COMBO_COUNT: u32 = 0x184;
/// RiderMetrix -> best combo (points).
const METRIX_BEST_COMBO_POINTS: u32 = 0x188;
/// RiderMetrix -> points banked over all combos.
const METRIX_BANKED_POINTS: u32 = 0x198;
/// The event settings (`mode` byte +0x48, `online` byte +0x49).
const BANK_COMBO_EVENT_SETTINGS: u32 = 0x0053_5bc8;

const BANK_COMBO_SHOW_HUD_NUMBER: Callee = Callee {
    addr: 0x0011_7b88,
    f: ee::s00::RiderMetrix_ShowHUDNumber,
};
const BANK_COMBO_AWARD_POINTS: Callee = Callee {
    addr: 0x0011_9ef8,
    f: ee::s00::RiderMetrix_AwardPoints,
};
const BANK_COMBO_RESET_COMBO: Callee = Callee {
    addr: 0x0011_75f8,
    f: ee::s00::RiderMetrix_ResetCombo,
};

/// `void RiderMetrix::BankCombo()` (0x117718): a combo of two or more
/// tricks with positive points is banked: points and trick count are added
/// to the totals, the best-combo records updated, and the back end's
/// interface 0 is touched; then either the combo number (HUD id 0x26, 1.5 s,
/// with the trick count) and the points (id 0x18, gp-0x7b10 s) are shown,
/// or, offline in mode 4, `AwardPoints(2, min(points / 500, 20))`. The combo
/// is always reset (`ResetCombo`). Frame 0x20 (s0).
pub fn rider_metrix_bank_combo(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let m = Addr(c.g[S0][0] as u32);
        c.g[A2][0] = m.off(METRIX_COMBO_COUNT).lw(c);
        c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
        'bank: {
            if c.g[V0][0] != 0 {
                break 'bank;
            }
            c.cycles += 3;
            c.g[A1][0] = m.off(METRIX_COMBO_POINTS).lw(c);
            if (c.g[A1][0] as i64) <= 0 {
                break 'bank;
            }
            // Bank the combo into the totals and records.
            c.cycles += 9;
            c.g[V0][0] = m.off(METRIX_BANKED_POINTS).lw(c);
            c.g[V1][0] = m.off(METRIX_BANKED_TRICKS).lw(c);
            c.g[A0][0] = m.off(METRIX_BEST_COMBO_POINTS).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            let total = c.g[V0][0];
            m.off(METRIX_BANKED_POINTS).sw(c, total);
            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let tricks = c.g[V1][0];
            m.off(METRIX_BANKED_TRICKS).sw(c, tricks);
            if c.g[A0][0] != 0 {
                c.cycles += 1;
                let pts = c.g[A1][0];
                m.off(METRIX_BEST_COMBO_POINTS).sw(c, pts);
            }
            c.cycles += 5;
            c.g[V1][0] = m.off(METRIX_COMBO_COUNT).lw(c);
            c.g[V0][0] = m.off(METRIX_BEST_COMBO_COUNT).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                let count = c.g[V1][0];
                m.off(METRIX_BEST_COMBO_COUNT).sw(c, count);
            }

            // cBE_getBE()->GetInterface(0), result unused.
            c.cycles += 11;
            call_ptr(c, 0x0014_dc80, 0x0011_7788)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_dd58, 0x0011_7794)?;

            // Offline in mode 4?
            c.g[V1][0] = sx(BANK_COMBO_EVENT_SETTINGS);
            c.g[A0][0] = 4;
            c.g[V0][0] = Addr(BANK_COMBO_EVENT_SETTINGS + 0x48).lb(c);
            c.g[A1][0] = 0;
            if c.g[V0][0] == c.g[A0][0] {
                c.cycles += 2;
                c.g[V0][0] = Addr(BANK_COMBO_EVENT_SETTINGS + 0x49).lb(c);
                c.g[A1][0] = (c.g[V0][0] < 1) as u64;
            }
            c.cycles += 2;
            let s0 = c.g[S0][0] as u32;
            c.g[A2][0] = Addr(s0.wrapping_add(METRIX_COMBO_POINTS)).lw(c);
            if c.g[A1][0] == 0 {
                // Show the combo count and the points on the HUD.
                c.cycles += 14;
                c.g[A0][0] = c.g[S0][0];
                c.g[AT][0] = 0x3fc0_0000;
                c.f[12] = 0x3fc0_0000;
                c.g[A1][0] = 0x26;
                c.g[A3][0] = Addr(s0.wrapping_add(METRIX_COMBO_COUNT)).lw(c);
                BANK_COMBO_SHOW_HUD_NUMBER.call(c, 0x0011_77d4)?;
                let s0 = c.g[S0][0] as u32;
                c.g[A2][0] = Addr(s0.wrapping_add(METRIX_COMBO_POINTS)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                let gp = c.g[GP][0] as u32;
                c.f[12] = Addr(gp.wrapping_sub(0x7b10)).read_u32(c);
                c.g[A1][0] = 0x18;
                c.g[A3][0] = 0;
                BANK_COMBO_SHOW_HUD_NUMBER.call(c, 0x0011_77ec)?;
            } else {
                // Award min(points / 500, 20) as an offline bonus.
                c.cycles += 3;
                c.g[V0][0] = 0x1f4;
                c.cycles += 8;
                c.g[V1][0] = 0x14;
                c.div(0, A2, V0);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 2;
                c.g[A2][0] = c.lo[0];
                c.g[V0][0] = ((c.g[A2][0] as i64) < 21) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = c.g[V1][0];
                }
                BANK_COMBO_AWARD_POINTS.call(c, 0x0011_7820)?;
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        BANK_COMBO_RESET_COMBO.call(c, 0x0011_7828)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObjFactory_AnimObject (0x2fbcb8)

/// gp-relative guard word of the static default anim parameters.
const WOFA_DEFAULTS_READY_GP: u32 = 0x24e8;
/// The static default parameter block (11 words: id -1, 1, 0, -1.0, -1.0,
/// 30.0, 0, -1.0, 0, 0, tag byte at +0x28).
const WOFA_DEFAULTS: u32 = 0x004f_b498;
/// Bytes copied from the defaults into the local block.
const WOFA_BLOCK_BYTES: u64 = 0x2c;
/// Parameter kinds by slot (`D_00445F48`): kind 2 is a float slot.
const WOFA_PARAM_KINDS: u32 = 0x0044_5f48;
/// gp-relative: the world-object directory and the world.
const WOFA_DIRECTORY_GP: u32 = 0x16c8;
const WOFA_WORLD_GP: u32 = 0xce8;
/// World -> its default object (used for id -1).
const WOFA_WORLD_DEFAULT_OBJECT: u32 = 0x290;
/// Object -> its active node.
const WOFA_OBJECT_NODE: u32 = 0xc;
/// The factory's result value (12 bytes, returned by copy).
const WOFA_RESULT: u32 = 0x004c_9098;

const WOFA_ATTACH_MODIFIER: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};
const WOFA_MEM_ALLOC_NAMED: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};

/// `Result WorldObjFactory_AnimObject(Param* params, int count)` (0x2fbcb8;
/// a0 = hidden result slot, a1 = count, a2 = params): fills an 11-word anim
/// parameter block from the (lazily initialised) defaults and the
/// `{slot, value, -, kind}` parameters (an int for a float slot is
/// converted), looks the object id up (-1: the world's default object), and
/// unless the object already has an active node accepting the modifier
/// (`AttachModifier(node, 1, block)`), allocates and constructs an
/// `AnimObject` (0x6c bytes) for it. Returns the factory's result value.
/// Frame 0xa0 (s0-s4).
pub fn world_obj_factory_anim_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        let gp = Addr(c.g[GP][0] as u32);
        c.g[V0][0] = gp.off(WOFA_DEFAULTS_READY_GP).lw(c);
        frame.save(c, S3, 0x60);
        frame.save(c, S0, 0x90);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x80);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x50);
        c.g[A2][0] = 0x50_0000;
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0x40);
        let defaults = Addr(WOFA_DEFAULTS);
        if !ready {
            // First use: initialise the static defaults (guarded local).
            c.cycles += 22;
            c.g[V0][0] = sx(WOFA_DEFAULTS);
            let tag = defaults.off(0x28).lbu(c);
            c.g[A1][0] = tag;
            c.g[V1][0] = u64::MAX;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.f[0] = 0xbf80_0000; // -1.0
            c.g[A0][0] = 1;
            c.g[AT][0] = 0x41f0_0000;
            c.f[1] = 0x41f0_0000; // 30.0
            defaults.sw(c, u64::MAX);
            defaults.off(0x28).sb(c, tag);
            defaults.off(0x14).write_u32(c, 0x41f0_0000);
            defaults.off(0x1c).write_u32(c, 0xbf80_0000);
            gp.off(WOFA_DEFAULTS_READY_GP).sw(c, 1);
            defaults.off(0x4).sw(c, 1);
            defaults.off(0x8).sw(c, 0);
            defaults.off(0xc).write_u32(c, 0xbf80_0000);
            defaults.off(0x10).write_u32(c, 0xbf80_0000);
            defaults.off(0x18).sw(c, 0);
            defaults.off(0x20).sw(c, 0);
            defaults.off(0x24).sw(c, 0);
            Addr(sp(c)).sb(c, tag);
        }
        // The parameter block at 0x10(sp), copied from the defaults.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        let block = frame.at(c, 0x10);
        c.g[S1][0] = sx(block);
        c.g[A1][0] = sx(WOFA_DEFAULTS);
        c.g[A2][0] = WOFA_BLOCK_BYTES;
        Addr(block).sw(c, u64::MAX);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x4d_0000;
        call_ptr(c, 0x003e_6574, 0x002f_bd68)?; // EA_MemCopy
        c.g[A2][0] = c.g[S3][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(WOFA_PARAM_KINDS);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                let param = Addr(c.g[A2][0] as u32);
                let slot4 = (param.lw(c) as u32) << 2;
                c.g[V1][0] = param.off(0xc).lw(c);
                let dest = Addr(slot4.wrapping_add(c.g[S1][0] as u32));
                c.g[A0][0] = sx(dest.0);
                c.g[V0][0] = sx(slot4.wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                if c.g[V1][0] != c.g[V0][0] && c.g[V0][0] == c.g[A3][0] {
                    // An int given for a float slot.
                    c.cycles += 2 + 4;
                    let value = param.off(4).read_u32(c);
                    let converted = crate::ps2f::FpuF32::from_i32(value as i32);
                    c.f[0] = converted.0;
                    dest.write_fpu(c, converted);
                } else {
                    c.cycles += if c.g[V1][0] == c.g[V0][0] { 1 } else { 2 + 1 };
                    c.g[V0][0] = param.off(4).lw(c);
                    let value = c.g[V0][0];
                    dest.sw(c, value);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if c.g[A1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_bd80);
                }
            }
        }
        // Look the object up.
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        let gp = Addr(c.g[GP][0] as u32);
        c.g[A1][0] = gp.off(WOFA_DIRECTORY_GP).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = gp.off(WOFA_WORLD_GP).lw(c);
            let world = c.g[V0][0] as u32;
            c.g[S0][0] = Addr(world.wrapping_add(WOFA_WORLD_DEFAULT_OBJECT)).lw(c);
        } else {
            c.cycles += 8;
            let id = c.g[A2][0] as u32;
            c.g[V0][0] = sx((id & 0xff) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx(id >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !found {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A3][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A3][0] = 0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A3][0];
        }
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(WOFA_OBJECT_NODE)).lw(c);
            c.g[A1][0] = 1;
            let mut create = true;
            if c.g[A0][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                WOFA_ATTACH_MODIFIER.call(c, 0x002f_be50)?;
                create = c.g[V0][0] == 0;
            }
            if create {
                c.cycles += 12;
                c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x6958));
                c.g[A0][0] = 0x6c;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                WOFA_MEM_ALLOC_NAMED.call(c, 0x002f_be70)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                c.g[A3][0] = c.g[S0][0];
                c.g[T0][0] = c.g[S1][0];
                c.g[A2][0] = 1;
                call_ptr(c, 0x0034_1aa0, 0x002f_be88)?; // AnimObject_ctor
            }
        }
        // Return the factory's result value by copy.
        c.cycles += 16;
        let result = Addr((c.g[S4][0] as u32).wrapping_sub(0x4d_0000 - WOFA_RESULT));
        c.g[A1][0] = sx(result.0);
        let head = result.read_u64_unaligned(c);
        c.g[V0][0] = head;
        c.g[V1][0] = result.off(8).lw(c);
        let out = Addr(c.g[S2][0] as u32);
        out.write_u64_unaligned(c, head);
        let tail = c.g[V1][0];
        out.off(8).sw(c, tail);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AnimSeqList_RemoveEventTrigger (0x314718)

/// AnimSeqList -> first sequence.
const SEQ_LIST_HEAD: u32 = 0x4;
/// AnimSequence -> next sequence in the list.
const SEQ_NEXT: u32 = 0xc8;

const SEQ_REMOVE_EVENT_TRIGGER: Callee = Callee {
    addr: 0x0031_3ba8,
    f: ee::s10::AnimSequence_RemoveEventTrigger,
};

/// `void AnimSeqList::RemoveEventTrigger(int id)` (0x314718): removes the
/// event trigger `id` from every sequence of the list
/// (`AnimSequence_RemoveEventTrigger(seq, id)`, following `seq+0xc8`).
/// Frame 0x30 (s0 = sequence, s1 = id).
pub fn anim_seq_list_remove_event_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        let list = c.g[A0][0] as u32;
        c.g[S0][0] = Addr(list.wrapping_add(SEQ_LIST_HEAD)).lw(c);
        c.g[S1][0] = c.g[A1][0];
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                SEQ_REMOVE_EVENT_TRIGGER.call(c, 0x0031_4740)?;
                let seq = c.g[S0][0] as u32;
                c.g[S0][0] = Addr(seq.wrapping_add(SEQ_NEXT)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_4738);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAirState_PlayLandAnim2 (0x13a968)

/// gp-relative literals: the hard-landing speed (-2222.22), pi and the
/// medium-landing speed (-1250.0).
const LAND2_HARD_SPEED_GP: u32 = 0x719c;
const LAND2_PI_GP: u32 = 0x7198;
const LAND2_MEDIUM_SPEED_GP: u32 = 0x7194;
/// Air state -> its rider.
const LAND2_RIDER: u32 = 0x4;
/// Rider -> its anim controller.
const LAND2_RIDER_ANIM: u32 = 0x784;
/// Rider -> the landing spin angle (radians).
const LAND2_RIDER_SPIN: u32 = 0x2dc;
/// `AnimController_PlayAnim` (through the function table).
const LAND2_PLAY_ANIM: u32 = 0x0031_28e8;

/// `void RiderAirState::PlayLandAnim2(float vy)` (0x13a968): a very hard
/// landing (`vy < -2222.2`) plays anim 0x3f and clears the spin angle;
/// otherwise an over-rotated spin plays 0x42 (< -pi) or 0x43 (> pi), and
/// a straight landing 0x3e (`vy < -1250`) or 0x3d. Each through
/// `AnimController_PlayAnim(anim, id, 0, -1.0)`. Frame 0x20 (s0).
pub fn rider_air_state_play_land_anim2(c: &mut Cpu) {
    use crate::abi::set_fcc;
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 7;
        c.f[0] = land2_gp_lit(c, LAND2_HARD_SPEED_GP);
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let hard = FpuF32(c.f[12]).lt(FpuF32(c.f[0]));
        set_fcc(c, hard);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        if hard {
            c.cycles += 10;
            let state = c.g[S0][0] as u32;
            c.g[V0][0] = Addr(state.wrapping_add(LAND2_RIDER)).lw(c);
            c.g[A1][0] = 0x3f;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.f[12] = 0xbf80_0000;
            c.g[A2][0] = 0;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(LAND2_RIDER_ANIM)).lw(c);
            call_ptr(c, LAND2_PLAY_ANIM, 0x0013_a9a0)?;
            let state = c.g[S0][0] as u32;
            c.g[V1][0] = Addr(state.wrapping_add(LAND2_RIDER)).lw(c);
            Addr((c.g[V1][0] as u32).wrapping_add(LAND2_RIDER_SPIN)).sw(c, 0);
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            // Pick the anim id; `None` is the plain landing 0x3d, which
            // loads the controller after the decision (0x13aa1c).
            c.cycles += 8;
            c.f[2] = land2_gp_lit(c, LAND2_PI_GP);
            let state = c.g[S0][0] as u32;
            c.g[A0][0] = Addr(state.wrapping_add(LAND2_RIDER)).lw(c);
            let pi = FpuF32(c.f[2]);
            c.f[0] = pi.0 ^ 0x8000_0000;
            let rider = c.g[A0][0] as u32;
            c.f[1] = Addr(rider.wrapping_add(LAND2_RIDER_SPIN)).read_u32(c);
            let spin = FpuF32(c.f[1]);
            let under = spin.lt(FpuF32(c.f[0]));
            set_fcc(c, under);
            c.g[A1][0] = 0x42;
            let chosen = if under {
                c.cycles += 2;
                true
            } else {
                c.cycles += 4;
                let over = pi.lt(spin);
                set_fcc(c, over);
                c.g[A1][0] = 0x43;
                if over {
                    c.cycles += 2;
                    true
                } else {
                    c.cycles += 5;
                    c.f[0] = land2_gp_lit(c, LAND2_MEDIUM_SPEED_GP);
                    let medium = FpuF32(c.f[12]).lt(FpuF32(c.f[0]));
                    set_fcc(c, medium);
                    c.g[A1][0] = 0x3e;
                    if medium {
                        c.cycles += 1;
                    }
                    medium
                }
            };
            if chosen {
                let rider = c.g[A0][0] as u32;
                c.g[A0][0] = Addr(rider.wrapping_add(LAND2_RIDER_ANIM)).lw(c);
                c.cycles += 6;
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.f[12] = 0xbf80_0000;
                c.g[A2][0] = 0;
                call_ptr(c, LAND2_PLAY_ANIM, 0x0013_aa14)?;
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 6;
                let rider = c.g[A0][0] as u32;
                c.g[A0][0] = Addr(rider.wrapping_add(LAND2_RIDER_ANIM)).lw(c);
                c.g[A1][0] = 0x3d;
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.f[12] = 0xbf80_0000;
                c.g[A2][0] = 0;
                call_ptr(c, LAND2_PLAY_ANIM, 0x0013_aa34)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `lwc1` of the gp-relative literal at `gp - off`.
#[inline(always)]
fn land2_gp_lit(c: &mut Cpu, off: u32) -> u32 {
    let gp = c.g[GP][0] as u32;
    Addr(gp.wrapping_sub(off)).read_u32(c)
}

// ---------------------------------------------------------------------------
// Emitter_SetDescVelocityCurve (0x36d318)

/// Emitter desc -> lifetime (float).
const EMITTER_LIFETIME: u32 = 0x4c;
/// Emitter desc -> curve base `a - 1.5 * (c + d)` (vec4).
const EMITTER_CURVE_BASE: u32 = 0xf0;
/// Emitter desc -> curve control points `c` and `d` (vec4 each).
const EMITTER_CURVE_C: u32 = 0x100;
const EMITTER_CURVE_D: u32 = 0x110;
/// Emitter desc -> linear velocity rate `(b - a) / (t * lifetime)` (vec4).
const EMITTER_CURVE_RATE: u32 = 0x120;

/// `void Emitter_SetDescVelocityCurve(desc, const Vec4* a, const Vec4* b,
/// const Vec4* c (a3), const Vec4* d (t0), float t)` (0x36d318): sets the
/// velocity curve of a particle emitter description: base
/// `a - 1.5 * (c + d)`, rate `(b - a) / (t * lifetime)`, and the control
/// points `c`, `d`. VU0 macro ops through 16-byte stack temporaries, as the
/// original (frame 0x40); one 47-cycle block.
pub fn emitter_set_desc_velocity_curve(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    use ssx3_ee_rt::vu0::{Src, X};
    c.cycles += 47;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
    let tmp = sp(c);
    let desc = c.g[A0][0] as u32;
    let a = c.g[A1][0] as u32;
    let b = c.g[A2][0] as u32;
    let cp = c.g[A3][0] as u32;
    let dp = c.g[T0][0] as u32;
    c.g[AT][0] = 0x3fc0_0000;
    c.f[0] = 0x3fc0_0000; // 1.5
    // sum = c + d
    c.lqc2(3, cp);
    c.lqc2(4, dp);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, tmp.wrapping_add(0x30));
    c.lq(V0, tmp.wrapping_add(0x30));
    c.sq(V0, tmp.wrapping_add(0x20));
    // scaled = sum * 1.5
    c.mfc1(V1, 0);
    c.lqc2(4, tmp.wrapping_add(0x20));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, tmp.wrapping_add(0x30));
    c.lq(V0, tmp.wrapping_add(0x30));
    c.sq(V0, tmp.wrapping_add(0x10));
    // base = a - scaled
    c.lqc2(3, a);
    c.lqc2(4, tmp.wrapping_add(0x10));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, tmp.wrapping_add(0x30));
    c.lq(V0, tmp.wrapping_add(0x30));
    c.sq(V0, desc.wrapping_add(EMITTER_CURVE_BASE));
    c.sq(V0, tmp);
    // delta = b - a; rate = delta / (t * lifetime)
    c.lqc2(3, b);
    c.lqc2(4, a);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, tmp.wrapping_add(0x20));
    c.f[0] = Addr(desc.wrapping_add(EMITTER_LIFETIME)).read_u32(c);
    c.lq(V0, tmp.wrapping_add(0x20));
    c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[0])).0;
    c.sq(V0, tmp.wrapping_add(0x10));
    c.mfc1(V0, 12);
    c.qmtc2(V0, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, tmp.wrapping_add(0x10));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, tmp.wrapping_add(0x20));
    c.lq(V1, tmp.wrapping_add(0x20));
    c.sq(V1, desc.wrapping_add(EMITTER_CURVE_RATE));
    c.sq(V1, tmp);
    // The control points.
    c.lq(V0, cp);
    c.sq(V0, desc.wrapping_add(EMITTER_CURVE_C));
    c.lq(V1, dp);
    c.sq(V1, desc.wrapping_add(EMITTER_CURVE_D));
    c.g[SP][0] = sx(tmp.wrapping_add(0x40));
    ret(c);
}

// ---------------------------------------------------------------------------
// SND_EeVoiceSetPitch (0x3c8a88)

/// The sound driver's state block (`D_00515B40`).
const PITCH_SND_STATE: u32 = 0x0051_5b40;
/// State -> the voice entries (0x60 bytes each).
const PITCH_STATE_VOICES: u32 = 0x1dc;
/// State -> output sample rate code (>= 50: high-quality resampling).
const PITCH_STATE_RATE_CODE: u32 = 0xe;
/// Resampler interpolation setting byte (`D_0050AA62`).
const PITCH_RESAMPLE_MODE: u32 = 0x0050_aa62;
/// Bytes per voice entry.
const PITCH_VOICE_SIZE: u64 = 0x60;
/// Voice -> its DSP chain (sorted by priority).
const PITCH_VOICE_DSP_CHAIN: u32 = 0x40;
/// Voice -> its resampler DSP (0x38 bytes; priority half +0x18).
const PITCH_VOICE_RESAMPLER: u32 = 0x50;
/// Highest pitch ratio (2.0 in 16.16).
const PITCH_MAX: u64 = 0x2_0000;

const PITCH_RESAMPLE_INIT: Callee = Callee {
    addr: 0x003c_bb28,
    f: ee::s14::SND_DspResample_Init,
};
const PITCH_DSP_INSERT_SORTED: Callee = Callee {
    addr: 0x003c_8f48,
    f: ee::s14::SND_DspInsertSorted,
};
const PITCH_RESAMPLE_SET_RATE: Callee = Callee {
    addr: 0x003c_b538,
    f: ee::s14::SND_DspResample_SetRate,
};

/// `void SND_EeVoiceSetPitch(int voice, int pitch)` (0x3c8a88): sets the
/// voice's resampling rate to `min(pitch, 0x20000)`, first creating its
/// resampler DSP if it has none (`SND_MemAlloc(0x38)`, priority 0xa0,
/// `SND_DspResample_Init(dsp, mode, rate code >= 50)`, inserted into the
/// voice's DSP chain). Frame 0x40 (s0 = voice entry, s1 = rate, s2 =
/// driver state); the entry offset uses `mult`.
pub fn snd_ee_voice_set_pitch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = PITCH_VOICE_SIZE;
        frame.save(c, S0, 0x30);
        c.mult(0, A0, A0, V0);
        frame.save(c, S1, 0x20);
        c.g[V1][0] = 0x51_0000;
        frame.save(c, S2, 0x10);
        c.g[V0][0] = PITCH_MAX;
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(PITCH_SND_STATE);
        c.g[S1][0] = c.g[V0][0];
        c.g[A2][0] = Addr(PITCH_SND_STATE + PITCH_STATE_VOICES).lw(c);
        c.g[V0][0] = ((PITCH_MAX as i64) < (c.g[A1][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[S1][0] = c.g[A1][0];
        }
        c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let voice = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(voice.wrapping_add(PITCH_VOICE_RESAMPLER)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[A0][0] = c.g[V0][0];
        } else {
            // Create the resampler and insert it into the DSP chain.
            c.cycles += 20;
            c.g[A0][0] = 0x38;
            call_ptr(c, 0x003b_5c60, 0x003c_8ad8)?; // SND_MemAlloc
            let voice = Addr(c.g[S0][0] as u32);
            let dsp = c.g[V0][0];
            voice.off(PITCH_VOICE_RESAMPLER).sw(c, dsp);
            c.g[A0][0] = 0xa0;
            c.g[A3][0] = 0x51_0000;
            let state = c.g[S2][0] as u32;
            c.g[A2][0] = Addr(state.wrapping_add(PITCH_STATE_RATE_CODE)).lbu(c);
            Addr((dsp as u32).wrapping_add(4)).sw(c, 0);
            c.g[A2][0] = (c.g[A2][0] < 0x32) as u64;
            c.g[V1][0] = voice.off(PITCH_VOICE_RESAMPLER).lw(c);
            c.g[A2][0] ^= 1;
            let prio = c.g[A0][0];
            Addr((c.g[V1][0] as u32).wrapping_add(0x18)).sh(c, prio);
            c.g[V0][0] = voice.off(PITCH_VOICE_RESAMPLER).lw(c);
            Addr((c.g[V0][0] as u32).wrapping_add(0x1a)).sb(c, 0);
            c.g[A1][0] = Addr(PITCH_RESAMPLE_MODE).lbu(c);
            c.g[A0][0] = voice.off(PITCH_VOICE_RESAMPLER).lw(c);
            PITCH_RESAMPLE_INIT.call(c, 0x003c_8b10)?;
            let voice = Addr(c.g[S0][0] as u32);
            c.g[A1][0] = voice.off(PITCH_VOICE_RESAMPLER).lw(c);
            c.g[A0][0] = sx(voice.0.wrapping_add(PITCH_VOICE_DSP_CHAIN));
            PITCH_DSP_INSERT_SORTED.call(c, 0x003c_8b1c)?;
            let voice = Addr(c.g[S0][0] as u32);
            c.g[A0][0] = voice.off(PITCH_VOICE_RESAMPLER).lw(c);
        }
        c.cycles += 8;
        c.g[A1][0] = c.g[S1][0];
        PITCH_RESAMPLE_SET_RATE.call(c, 0x003c_8b28)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// GameStateRestartRace_SaveState (0x235f48)

/// `GameState_SaveState` (same shard; through the override slot).
const RESTART_RACE_BASE_SAVE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};

/// `void GameStateRestartRace_SaveState(GameStateRestartRace* self,
/// Stream* stream)` (0x235f48): save the base game state, then the three
/// restart words at `+0x10`, `+0x14`, `+0x18` (4 bytes each, through the
/// stream's write method, vtable slot 1). Frame 0x30 (s0 = stream,
/// s1 = self); one 33-cycle block.
pub fn game_state_restart_race_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0]; // stream
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0]; // self
        RESTART_RACE_BASE_SAVE.call(c, 0x0023_5f64)?;
        for (field, ret_addr) in [
            (0x10u32, 0x0023_5f80u32),
            (0x14, 0x0023_5f9c),
            (0x18, 0x0023_5fb8),
        ] {
            let stream = r32(c, S0);
            c.g[V1][0] = Addr(stream).lw(c);
            let vt = r32(c, V1);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(field));
            c.g[A2][0] = 4;
            c.g[A0][0] = Addr(vt.wrapping_add(0x8)).lh(c);
            c.g[V0][0] = Addr(vt.wrapping_add(0xc)).lw(c);
            let write = r32(c, V0);
            c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, write, ret_addr)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002e_2550, dynamic_spray_fx_update_emitter_enables, "cDynamicSprayFX_UpdateEmitterEnables"),
    (0x002a_bd48, bank_monitor_instance_update_distance, "BankMonitorInstance_UpdateDistance"),
    (0x0011_7718, rider_metrix_bank_combo, "RiderMetrix_BankCombo"),
    (0x002f_bcb8, world_obj_factory_anim_object, "WorldObjFactory_AnimObject"),
    (0x0031_4718, anim_seq_list_remove_event_trigger, "AnimSeqList_RemoveEventTrigger"),
    (0x0013_a968, rider_air_state_play_land_anim2, "RiderAirState_PlayLandAnim2"),
    (0x0036_d318, emitter_set_desc_velocity_curve, "Emitter_SetDescVelocityCurve"),
    (0x003c_8a88, snd_ee_voice_set_pitch, "SND_EeVoiceSetPitch"),
    (0x0023_5f48, game_state_restart_race_save_state, "GameStateRestartRace_SaveState"),
];

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::{check, fill, scramble, Rng, DATA};

    /// cDynamicSprayFX_UpdateEmitterEnables: modes 0, 1 and other; mode 0
    /// with active on, active off + forced, and neither.
    #[test]
    fn dynamic_spray_fx_update_emitter_enables_matches() {
        check(
            ee::s09::f_002e2550__generated,
            dynamic_spray_fx_update_emitter_enables,
            300,
            &|c, seed| {
                let mut r = Rng(0x2e25_5000 ^ seed as u64);
                scramble(c, &mut r);
                let fx = DATA;
                let rider = DATA + 0x1000;
                let block = DATA + 0x4000;
                fill(c, &mut r, block, 0x1500 / 4);
                c.bus.write32(fx + SPRAY_RIDER, rider);
                c.bus.write32(fx + SPRAY_EMITTERS, block);
                let mode = match seed % 4 {
                    0 | 1 => 0,
                    2 => 1,
                    _ => r.next_u32() | 2,
                };
                c.bus.write32(rider + RIDER_SPRAY_MODE, mode);
                let active = if seed % 3 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(fx + SPRAY_ACTIVE, active);
                let forced = if (seed / 4) % 2 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(rider + RIDER_FORCE_SPRAY, forced);
                c.g[A0][0] = sx(fx);
                c.pc = 0x002e_2550;
            },
        );
    }

    /// BankMonitorInstance_UpdateDistance: the virtual is a fake that
    /// returns a random f0 (sqrt of its bits) and sometimes unwinds.
    #[test]
    fn bank_monitor_instance_update_distance_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        covered(
            ee::s08::f_002abd48__generated,
            bank_monitor_instance_update_distance,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2abd48, 1);
                let this = DATA;
                let listener = DATA + 0x1000;
                let vt = DATA + 0x3000;
                fill(c, &mut r, this, 0x20);
                c.bus.write32(listener + LISTENER_VTABLE, vt);
                c.bus.write16(vt + 0x30, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x34, FAKE);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(listener);
                c.pc = 0x002a_bd48;
            },
        );
    }

    /// RiderMetrix_BankCombo: short combos, no points, banked combos with
    /// and without new records, online/offline in mode 4 (AwardPoints on
    /// its early-out), the HUD calls on an empty message table, and the
    /// back end's interface query as a fake that may unwind.
    #[test]
    fn rider_metrix_bank_combo_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::GAME_GP;
        covered(
            ee::s00::f_00117718__generated,
            rider_metrix_bank_combo,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x117718, 1);
                // Back end: library exists, interface 0 -> fake method.
                let lib = DATA + 0x20_0000;
                let vt = DATA + 0x20_1000;
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x21_0000);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, lib);
                c.bus.write32(lib + 4, vt);
                c.bus.write16(vt + 8, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0xc, FAKE);
                // World: AwardPoints takes its early-out (rank word 5).
                let world = DATA + 0x22_0000;
                let game = world + 0x1000;
                let rank = world + 0x2000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0x28, rank);
                c.bus.write32(rank, 5);
                c.bus.write32(GAME_GP - 0x7b10, 0.7f32.to_bits());
                // Event settings: mode 4 online/offline, or another mode.
                let mode = if seed % 3 == 0 { 4 } else { (r.next_u32() % 6) as u8 };
                c.bus.write8(BANK_COMBO_EVENT_SETTINGS + 0x48, mode);
                c.bus.write8(BANK_COMBO_EVENT_SETTINGS + 0x49, (seed / 3 % 2) as u8);
                // The metrix record; empty HUD message table.
                let m = DATA;
                fill(c, &mut r, m, 0x200 / 4);
                c.bus.write32(m + 0x1b0, 0);
                let count = match seed % 8 {
                    0 => r.next_u32() % 2,
                    1 => 0u32.wrapping_sub(r.next_u32() % 4),
                    _ => 2 + r.next_u32() % 40,
                };
                c.bus.write32(m + METRIX_COMBO_COUNT, count);
                let points = match seed % 7 {
                    0 => 0u32.wrapping_sub(r.next_u32() % 100),
                    1 => 0,
                    2 => 20_000 + r.next_u32() % 50_000,
                    _ => r.next_u32() % 12_000,
                };
                c.bus.write32(m + METRIX_COMBO_POINTS, points);
                c.bus.write32(m + METRIX_BEST_COMBO_POINTS, r.next_u32() % 15_000);
                c.bus.write32(m + METRIX_BEST_COMBO_COUNT, r.next_u32() % 40);
                c.g[A0][0] = sx(m);
                c.pc = 0x0011_7718;
            },
        );
    }

    /// WorldObjFactory_AnimObject: the static defaults initialised or not;
    /// -1..3 parameters with random slots/kinds (ints into float slots get
    /// converted; slot 0 is the object id); the id is -1 (the world's
    /// default object, sometimes none) or a section/object pair whose
    /// section or entry may be empty. Found objects have no node (an
    /// `AnimObject` is allocated from the pool allocator and constructed for
    /// real; node flag 0x100 keeps WorldObj_ctor away from the focus
    /// manager) or a node whose modifier check is a fake returning non-zero.
    /// Not reached: a node whose modifier check returns 0.
    #[test]
    fn world_obj_factory_anim_object_matches() {
        use crate::difftest::GAME_GP;
        use crate::difftest::fakes::FAKE;
        use crate::game::fe::feasyncfile::tests::alloc_base_3;
        const RESULT_SLOT: u32 = DATA + 0x3000;
        const PARAMS: u32 = DATA + 0x1000;
        const DIR: u32 = DATA + 0x20_0000;
        const SECTIONS: u32 = DIR + 0x100;
        const TABLE: u32 = DIR + 0x200;
        const SECTION: u32 = DIR + 0x300;
        const ENTRIES: u32 = DIR + 0x400;
        const WORLD: u32 = DIR + 0xa00;
        const NODE: u32 = DIR + 0xc00;
        const VT: u32 = DIR + 0xd00;
        const PROCS: u32 = DIR + 0x1000;
        const TAILS: u32 = DIR + 0x1800;
        const OBJS: u32 = DIR + 0x2000;
        check(
            ee::s10::f_002fbcb8__generated,
            world_obj_factory_anim_object,
            300,
            &|c, seed| {
                let mut r = alloc_base_3(c, seed, 0x2f_bcb8, 1);
                c.pc = 0x002f_bcb8;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // Static defaults.
                let ready = seed % 4 != 0;
                w(c, GAME_GP + WOFA_DEFAULTS_READY_GP, ready as u32);
                fill(c, &mut r, WOFA_DEFAULTS, 11);
                if ready && seed % 3 == 0 {
                    w(c, WOFA_DEFAULTS, u32::MAX);
                }
                for i in 0..11 {
                    w(c, WOFA_PARAM_KINDS + 4 * i, r.next_u32() % 4);
                }
                // Directory: sections 0..2 present, 3 empty; entries 0..2 objects, 3 empty.
                w(c, GAME_GP + WOFA_DIRECTORY_GP, DIR);
                w(c, DIR, SECTIONS);
                w(c, SECTIONS + 8, TABLE);
                for i in 0..4 {
                    w(c, TABLE + 4 * i, if i == 3 { 0 } else { SECTION });
                    let obj = OBJS + 0x100 * i;
                    w(
                        c,
                        ENTRIES + 4 * i,
                        if i == 3 {
                            r.next_u32() & 0xff
                        } else {
                            (obj >> 2) << 8 | (r.next_u32() & 0xff)
                        },
                    );
                    w(c, obj + 8, 0x100);
                    w(
                        c,
                        obj + WOFA_OBJECT_NODE,
                        if (seed / 4) % 3 == 0 { NODE } else { 0 },
                    );
                }
                w(c, SECTION + 0x1c, ENTRIES);
                w(c, GAME_GP + WOFA_WORLD_GP, WORLD);
                w(
                    c,
                    WORLD + WOFA_WORLD_DEFAULT_OBJECT,
                    if seed % 7 == 0 { 0 } else { OBJS + 0x100 },
                );
                // The node's modifier check: a fake (non-zero result).
                w(c, NODE + 0xc, VT);
                c.bus.write16(VT + 0x50, 0);
                w(c, VT + 0x54, FAKE);
                // Process list for the new object (list 1).
                w(c, GAME_GP + 0x2898 + 4, PROCS);
                w(c, PROCS + 0x44, r.next_u32());
                w(c, PROCS + 0x44 + 8, TAILS);
                w(c, PROCS + 0x44 + 0x28, TAILS + 0x40);
                // The result value.
                fill(c, &mut r, WOFA_RESULT, 3);
                // Parameters.
                let count: u32 = if seed % 11 == 0 { u32::MAX } else { seed % 4 };
                for i in 0..3 {
                    let p = PARAMS + 0x10 * i;
                    let slot = r.next_u32() % 11;
                    let value = if slot == 0 {
                        match r.next_u32() % 4 {
                            0 => u32::MAX,
                            _ => (r.next_u32() % 4) | (r.next_u32() % 4) << 8,
                        }
                    } else {
                        r.next_u32()
                    };
                    w(c, p, slot);
                    w(c, p + 4, value);
                    w(c, p + 8, r.next_u32());
                    w(c, p + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(RESULT_SLOT + 4 * (seed % 2));
                c.g[A1][0] = sx(count);
                c.g[A2][0] = sx(PARAMS);
            },
        );
    }

    /// AnimSeqList_RemoveEventTrigger: empty lists and lists of 1-4
    /// sequences, each with no triggers or a short chain of triggers whose
    /// ids never match (the real callee walks them and frees nothing).
    #[test]
    fn anim_seq_list_remove_event_trigger_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s10::f_00314718__generated,
            anim_seq_list_remove_event_trigger,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_4718, 0);
                let list = DATA + 0x20_0000;
                let seqs = list + 0x100;
                let id = r.next_u32() % 4;
                let n = seed % 5;
                c.bus.write32(list + SEQ_LIST_HEAD, if n == 0 { 0 } else { seqs });
                for i in 0..n {
                    let seq = seqs + 0x200 * i;
                    let next = if i + 1 == n { 0 } else { seq + 0x200 };
                    c.bus.write32(seq + SEQ_NEXT, next);
                    // Triggers {id, flag, -, next}, never the removed id.
                    let trig = seq + 0x100;
                    let count = r.next_u32() % 3;
                    c.bus.write32(seq + 0xac, if count == 0 { 0 } else { trig });
                    for t in 0..count {
                        let tr = trig + 0x20 * t;
                        let tid = id + 1 + r.next_u32() % 8;
                        c.bus.write32(tr, tid);
                        c.bus.write32(tr + 4, r.next_u32() % 2);
                        c.bus.write32(tr + 0xc, if t + 1 == count { 0 } else { tr + 0x20 });
                    }
                }
                c.g[A0][0] = sx(list);
                c.g[A1][0] = id as u64;
                c.pc = 0x0031_4718;
            },
        );
    }

    /// RiderAirState_PlayLandAnim2: vertical speeds around the hard and
    /// medium thresholds and random, spin angles below -pi, above pi and in
    /// between; the real AnimController_PlayAnim on the controller fixture
    /// of the RiderAirState_PlayLandAnim test.
    #[test]
    fn rider_air_state_play_land_anim2_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::GAME_GP;
        use crate::game::ai::ai::tests::{slot, LOAD_B0};
        check(
            ee::s01::f_0013a968__generated,
            rider_air_state_play_land_anim2,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x13a968, 0);
                let state = DATA;
                let rider = DATA + 0x1000;
                let anim = DATA + 0x4000;
                let vt = DATA + 0x5000;
                c.bus.write32(GAME_GP - LAND2_HARD_SPEED_GP, (-2222.2222f32).to_bits());
                c.bus.write32(GAME_GP - LAND2_PI_GP, std::f32::consts::PI.to_bits());
                c.bus.write32(GAME_GP - LAND2_MEDIUM_SPEED_GP, (-1250.0f32).to_bits());
                c.f[12] = match seed % 4 {
                    0 => (-2222.3f32 + (seed % 5) as f32).to_bits(),
                    1 => (-1251.0f32 + (seed % 3) as f32).to_bits(),
                    2 => r.f32(3000.0),
                    _ => r.edge_f32(),
                };
                c.bus.write32(state + LAND2_RIDER, rider);
                c.bus.write32(rider + LAND2_RIDER_ANIM, anim);
                c.bus.write32(anim + 0x58, vt);
                slot(c, vt, 0x20, 0, LOAD_B0);
                c.bus.write32(anim + 0xb0, 0x207);
                let spin = match (seed / 4) % 4 {
                    0 => -3.5f32 - (seed % 7) as f32,
                    1 => 3.5 + (seed % 7) as f32,
                    2 => ((seed % 61) as f32 - 30.0) / 10.0,
                    _ => f32::from_bits(r.edge_f32()),
                };
                c.bus.write32(rider + LAND2_RIDER_SPIN, spin.to_bits());
                c.g[A0][0] = sx(state);
                c.pc = 0x0013_a968;
            },
        );
    }

    /// Emitter_SetDescVelocityCurve: random vectors (sometimes edge
    /// floats), lifetimes and t, including zero (VU0 division by zero).
    #[test]
    fn emitter_set_desc_velocity_curve_matches() {
        check(
            ee::s12::f_0036d318__generated,
            emitter_set_desc_velocity_curve,
            300,
            &|c, seed| {
                let mut r = Rng(0x36d3_1800 ^ seed as u64);
                scramble(c, &mut r);
                let desc = DATA;
                let vecs = DATA + 0x1000;
                fill(c, &mut r, desc, 0x130 / 4);
                for i in 0..16 {
                    let v = if seed % 5 == 0 { r.edge_f32() } else { r.f32(100.0) };
                    c.bus.write32(vecs + 4 * i, v);
                }
                c.bus.write32(
                    desc + EMITTER_LIFETIME,
                    match seed % 7 {
                        0 => 0,
                        1 => r.edge_f32(),
                        _ => r.f32(10.0),
                    },
                );
                c.f[12] = if seed % 11 == 0 { 0 } else { r.f32(4.0) };
                c.g[A0][0] = sx(desc);
                c.g[A1][0] = sx(vecs);
                c.g[A2][0] = sx(vecs + 0x10);
                c.g[A3][0] = sx(vecs + 0x20);
                c.g[T0][0] = sx(vecs + 0x30);
                c.pc = 0x0036_d318;
            },
        );
    }

    /// SND_EeVoiceSetPitch: pitches below, at and above 0x20000 (and
    /// negative), a voice that already has a resampler, or none (the real
    /// SND_MemAlloc allocates from an empty driver heap at DATA+0x20_0000),
    /// inserted into an empty DSP chain or one of 1-3 nodes with random
    /// priorities; both rate-code sides and several resampler modes.
    #[test]
    fn snd_ee_voice_set_pitch_matches() {
        use crate::difftest::check_lohi;
        use crate::difftest::fakes::machine;
        check_lohi(ee::s14::f_003c8a88__generated, snd_ee_voice_set_pitch, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3c_8a88, 0);
            // The voice entries.
            let entries = DATA + 0x3000;
            fill(c, &mut r, entries, 0x60 * 8 / 4);
            c.bus.write32(PITCH_SND_STATE + PITCH_STATE_VOICES, entries);
            let voice_index = r.next_u32() % 8;
            let voice = entries + 0x60 * voice_index;
            let existing = DATA + 0x6000;
            c.bus.write32(
                voice + PITCH_VOICE_RESAMPLER,
                if seed % 3 == 0 { existing } else { 0 },
            );
            // The DSP chain: empty or 1-3 nodes {next +8, priority +0x18}.
            let nodes = DATA + 0x7000;
            let n = seed / 3 % 4;
            c.bus.write32(voice + PITCH_VOICE_DSP_CHAIN, if n == 0 { 0 } else { nodes });
            for i in 0..n {
                let node = nodes + 0x40 * i;
                c.bus.write32(node + 8, if i + 1 == n { 0 } else { node + 0x40 });
                c.bus.write16(node + 0x18, (0x40 + 0x30 * i + r.next_u32() % 0x40) as u16);
            }
            // The driver heap (`STATE + 0x234`): empty, room for the block.
            let heap = DATA + 0x100;
            c.bus.write32(PITCH_SND_STATE + 0x234, heap);
            c.bus.write32(heap, DATA + 0x20_0000); // base
            c.bus.write32(heap + 0x4, DATA + 0x2000); // blocks
            c.bus.write32(heap + 0xc, 0x8_0000); // limit
            c.bus.write32(heap + 0x10, 0x100); // low water
            c.bus.write32(heap + 0x14, 0); // lowest: empty
            c.bus.write8(PITCH_SND_STATE + PITCH_STATE_RATE_CODE, (r.next_u32() % 100) as u8);
            c.bus.write8(PITCH_RESAMPLE_MODE, (r.next_u32() % 3) as u8);
            c.g[A0][0] = voice_index as u64;
            c.g[A1][0] = match seed % 5 {
                0 => PITCH_MAX,
                1 => PITCH_MAX + 1 + (r.next_u32() % 0x1_0000) as u64,
                2 => sx(0u32.wrapping_sub(r.next_u32() % 0x100)),
                _ => (r.next_u32() % 0x2_0000) as u64,
            };
            c.pc = 0x003c_8a88;
        });
    }

    /// GameStateRestartRace_SaveState: the stream's write method (and the
    /// base save's) is a fake that may unwind at any of the four calls;
    /// `self` sometimes carries upper-half garbage.
    #[test]
    fn game_state_restart_race_save_state_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        const ME: u32 = DATA;
        const STREAM: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        covered(
            ee::s06::f_00235f48__generated,
            game_state_restart_race_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23_5f48, 4);
                c.pc = 0x0023_5f48;
                c.bus.write32(STREAM, VT);
                c.bus.write16(VT + 8, (r.next_u32() % 0x20) as u16 & !3);
                c.bus.write32(VT + 0xc, FAKE);
                c.g[A0][0] = if seed % 4 == 0 {
                    (r.next_u32() as u64) << 32 | ME as u64
                } else {
                    sx(ME)
                };
                c.g[A1][0] = sx(STREAM);
            },
        );
    }
}
