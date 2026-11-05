//! Staging wave 207 (oc255): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002e_f038, world_painter_get_surface08, "WorldPainter_GetSurface08"),
    (0x0022_24b8, luno_vm_call, "LunoVM_Call"),
    (0x0038_25f8, ps2_graphics_man_on_vblank_start, "PS2GraphicsMan_OnVBlankStart"),
    (0x0010_86b8, rider_probe_rail, "Rider_ProbeRail"),
    (0x0036_c740, ps2_graphics_man_reset_bloom_params, "PS2GraphicsMan_ResetBloomParams"),
    (0x002e_ef18, world_painter_get_light_glow30, "WorldPainter_GetLightGlow30"),
    (0x002e_e8a0, world_painter_get_weather80, "WorldPainter_GetWeather80"),
    (0x0020_cba0, game_ui_is_pause_menu_open, "GameUI_IsPauseMenuOpen"),
    (0x0039_0c20, vfx_render_toggles_update, "VfxRenderToggles_Update"),
    (0x0035_2810, mesh_anim_render, "MeshAnim_Render"),
];

// ---------------------------------------------------------------------------
// WorldPainter_GetSurface08 (0x2ef038)

/// The world-painter view table (`D_004FA370`), 0xf0 bytes per entry.
const SURFACE08_VIEWS: u32 = 0x004f_a370;
/// `sRSMEntry`: the surface holder (`sRSMObj**`).
const SURFACE08_HOLDER: u32 = 0x18;
/// `sRSMEntry`: the surface blend (the returned value is scaled by `1 - it`).
const SURFACE08_BLEND: u32 = 0x24;
/// The vtable slot (36): `{delta: i16, fn}` at +0x120.
const SURFACE08_SLOT: u32 = 0x120;

/// `float WorldPainter_GetSurface08(int i)` (0x2ef038): the surface value of
/// world-painter view `i`: call vtable slot 36 of the object behind
/// `views[i].holder`, which returns a pointer to a float, and return that
/// float times `1 - views[i].blend`. Keeps the entry pointer in s0 across
/// the call (saved with `sq`).
pub fn world_painter_get_surface08(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0xf0;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 0x50_0000;
        c.mult(0, S0, A0, V1); // s0 = i * 0xf0
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(SURFACE08_VIEWS);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(SURFACE08_VIEWS));
        let entry = r32(c, S0);
        let holder = Addr(entry).off(SURFACE08_HOLDER).lw(c);
        c.g[V1][0] = holder;
        let obj = Addr(holder as u32).lw(c);
        c.g[A2][0] = obj;
        let vt = Addr(obj as u32).off(4).lw(c);
        c.g[A1][0] = vt;
        let delta = Addr(vt as u32).off(SURFACE08_SLOT).lh(c);
        c.g[A0][0] = delta;
        let method = Addr(vt as u32).off(SURFACE08_SLOT + 4).lw(c);
        c.g[V0][0] = method;
        c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
        call_ptr(c, method as u32, 0x002e_f074)?;
        // value * (1 - blend), with the original's register use.
        let entry = r32(c, S0);
        let blend = Addr(entry).off(SURFACE08_BLEND).read_fpu(c);
        c.f[2] = blend.0;
        c.g[AT][0] = 0x3f80_0000;
        let value = Addr(r32(c, V0)).read_fpu(c);
        c.f[1] = value.0;
        let keep = FpuF32::ONE - blend;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        abi::set_fret(c, value * keep);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// LunoVM_Call (0x2224b8)

/// `operator new(size, name, flags, align)` (through the function table).
const LUNO_CALL_NEW_NAMED: u32 = 0x0031_7e30;
/// The allocation name (`D_00479780`).
const LUNO_CALL_NEW_NAME: u32 = 0x0047_9780;
/// The allocation flags.
const LUNO_CALL_NEW_FLAGS: u64 = 0x2000_0000;
/// The VM's default second register (`D_004C9098`, a `cLunoValue`).
const LUNO_CALL_DEFAULT_VALUE: u32 = 0x004c_9098;
/// `cLunoValue`: 12 bytes, `{value (8 bytes, a table pointer for type 4), type}`.
const LUNO_VALUE_SIZE: u64 = 0xc;
const LUNO_VALUE_TYPE: u32 = 0x8;
/// Value types: a table reference (counted) and an 8-byte pair.
const LUNO_TYPE_TABLE: u64 = 4;
const LUNO_TYPE_PAIR: u64 = 5;

const LUNO_CALL_TABLE_DTOR: Callee = Callee {
    addr: 0x0022_4df0,
    f: ee::s05::cLunoTable_dtor,
};
const LUNO_CALL_TABLE_ASSIGN: Callee = Callee {
    addr: 0x0022_5068,
    f: ee::s05::cLunoTable_Assign,
};
const LUNO_CALL_EXECUTE: Callee = Callee {
    addr: 0x0022_27d0,
    f: ee::s05::LunoVM_Execute,
};

/// `void LunoVM_Call(cLunoValue** out, LunoState* state, cLunoValue* table,
/// void* ctx)` (0x2224b8): run a Luno script function. Allocates the
/// register file (`n` values, `n` read from the script's function record at
/// `*state + *(*state + 4) + 8`, a 16-byte count header in front, every
/// value cleared), stores it in `*out`, sets register 0 to a reference to
/// `table` and register 1 to a copy of the VM default value (`D_004C9098`:
/// a table reference is counted, a type-5 pair copied as 8 bytes, anything
/// else as one word), each first releasing an old table reference, then
/// runs `LunoVM_Execute(out, state, ctx)`.
pub fn luno_vm_call(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x90);
        c.g[V1][0] = LUNO_VALUE_SIZE;
        for (reg, off) in [
            (S1, 0x70),
            (S3, 0x50),
            (S4, 0x40),
            (S5, 0x30),
            (S6, 0x20),
            (S7, 0x10),
            (S0, 0x80),
            (S2, 0x60),
        ] {
            frame.save(c, reg, off);
        }
        c.g[S3][0] = c.g[A2][0]; // table
        c.g[S4][0] = c.g[A0][0]; // out
        c.g[S6][0] = c.g[A1][0]; // state
        c.g[S7][0] = c.g[A3][0]; // ctx
        c.g[A2][0] = LUNO_CALL_NEW_FLAGS;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(LUNO_CALL_NEW_NAME);
        c.g[A3][0] = 0;
        c.g[S5][0] = 0x4d_0000;
        // n = the register count of the function record.
        let script = Addr(r32(c, S6)).lw(c);
        let code_off = Addr(script as u32).off(4).lw(c);
        c.g[A0][0] = code_off;
        let record = (script as u32).wrapping_add(code_off as u32);
        c.g[V0][0] = sx(record);
        c.g[S0][0] = Addr(record).off(8).lw(c);
        c.mult(0, V1, S0, V1); // v1 = n * 12
        c.g[A0][0] = sx(r32(c, V1).wrapping_add(0x10));
        call_ptr(c, LUNO_CALL_NEW_NAMED, 0x0022_2524)?;

        // The count header, then clear every value.
        let block = r32(c, V0);
        c.g[S1][0] = sx(block.wrapping_add(0x10));
        let w_s0 = c.g[S0][0];
        Addr(block).sw(c, w_s0);
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
        c.g[V0][0] = c.g[S1][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
            loop {
                c.cycles += 7;
                let value = r32(c, V0);
                Addr(value).off(LUNO_VALUE_TYPE).sw(c, 0);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                Addr(value).sw(c, 0);
                let more = c.g[V1][0] != c.g[A0][0];
                c.g[V0][0] = sx(value.wrapping_add(LUNO_VALUE_SIZE as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_2540);
                }
            }
        }

        // Register 0: release an old table reference, then reference `table`.
        c.cycles += 4;
        let regs = r32(c, S1);
        c.g[V0][0] = Addr(regs).off(LUNO_VALUE_TYPE).lw(c);
        c.g[S2][0] = LUNO_TYPE_TABLE;
        let w_s1 = c.g[S1][0];
        Addr(r32(c, S4)).sw(c, w_s1);
        if c.g[V0][0] == LUNO_TYPE_TABLE {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            LUNO_CALL_TABLE_DTOR.call(c, 0x0022_2578)?;
        }
        c.cycles += 11;
        c.g[A1][0] = c.g[S3][0];
        let regs = r32(c, S1);
        Addr(regs).off(LUNO_VALUE_TYPE).sw(c, 0);
        Addr(regs).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        LUNO_CALL_TABLE_ASSIGN.call(c, 0x0022_258c)?;
        c.g[S3][0] = sx(LUNO_CALL_DEFAULT_VALUE);
        c.g[S0][0] = Addr(r32(c, S4)).lw(c);
        let w_s2 = c.g[S2][0];
        Addr(r32(c, S1)).off(LUNO_VALUE_TYPE).sw(c, w_s2);

        // Register 1: release an old table reference.
        let regs = r32(c, S0);
        c.g[V0][0] = Addr(regs).off(LUNO_VALUE_SIZE as u32 + LUNO_VALUE_TYPE).lw(c);
        c.g[S1][0] = sx(regs.wrapping_add(LUNO_VALUE_SIZE as u32));
        if c.g[V0][0] == c.g[S2][0] {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            LUNO_CALL_TABLE_DTOR.call(c, 0x0022_25b0)?;
        }

        // Register 1 = a copy of the default value.
        c.cycles += 5;
        let reg1 = r32(c, S1);
        Addr(reg1).off(LUNO_VALUE_TYPE).sw(c, 0);
        let regs = r32(c, S0);
        Addr(regs).off(0xc).sw(c, 0);
        let default = Addr(LUNO_CALL_DEFAULT_VALUE);
        c.g[V1][0] = default.off(LUNO_VALUE_TYPE).lw(c);
        c.g[V0][0] = LUNO_TYPE_PAIR;
        if c.g[V1][0] == c.g[S2][0] {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S3][0];
            LUNO_CALL_TABLE_ASSIGN.call(c, 0x0022_25d0)?;
        } else {
            c.cycles += 2;
            let pair = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = default.lw(c);
            let regs = r32(c, S0);
            if pair {
                // `ldl/ldr` + `sdl/sdr`: an unaligned 8-byte copy.
                c.cycles += 7;
                c.g[A1][0] = sx(LUNO_CALL_DEFAULT_VALUE);
                c.g[V0][0] = default.ld(c);
                let lo = default.lw(c);
                let hi = default.off(4).lw(c);
                Addr(regs).off(0xc).sw(c, lo);
                Addr(regs).off(0x10).sw(c, hi);
            } else {
                c.cycles += 2;
                let w_v0 = c.g[V0][0];
                Addr(regs).off(0xc).sw(c, w_v0);
            }
        }
        c.g[V0][0] = default.off(LUNO_VALUE_TYPE).lw(c);

        c.cycles += 16;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S6][0];
        c.g[A2][0] = c.g[S7][0];
        let w_v0 = c.g[V0][0];
        Addr(r32(c, S1)).off(LUNO_VALUE_TYPE).sw(c, w_v0);
        LUNO_CALL_EXECUTE.call(c, 0x0022_2618)?;
        for (reg, off) in [
            (S0, 0x80),
            (S1, 0x70),
            (S2, 0x60),
            (S3, 0x50),
            (S4, 0x40),
            (S5, 0x30),
            (S6, 0x20),
            (S7, 0x10),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_OnVBlankStart (0x3825f8)

/// PS2GraphicsMan: the swap state (4 = waiting for the vblank count, 5 = go).
const VBLANK_SWAP_STATE: u32 = 0x5a8c;
/// PS2GraphicsMan: the number of vblanks to wait before signalling.
const VBLANK_WAIT_COUNT: u32 = 0x5ab8;
/// PS2GraphicsMan: vblanks counted so far.
const VBLANK_COUNTER: u32 = 0x5abc;
/// PS2GraphicsMan: the semaphore the render thread waits on.
const VBLANK_SEMA: u32 = 0x5ac8;
/// `iSignalSema(sema)` (interrupt-context syscall wrapper).
const VBLANK_I_SIGNAL_SEMA: u32 = 0x0042_3dd0;

/// `void PS2GraphicsMan_OnVBlankStart(PS2GraphicsMan* self)` (0x3825f8): the
/// vblank-start handler. Counts the vblank; when a swap is waiting (state 4)
/// and the counter has reached the wait count, moves the state to 5, resets
/// the counter and signals the render thread's semaphore.
pub fn ps2_graphics_man_on_vblank_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        let gfx = Addr(r32(c, A1));
        frame.save_ra(c, 0);
        c.g[V0][0] = 4;
        let count = gfx.off(VBLANK_COUNTER).lw(c);
        c.g[A0][0] = gfx.off(VBLANK_SWAP_STATE).lw(c);
        c.g[V1][0] = sx((count as u32).wrapping_add(1));
        let w_v1 = c.g[V1][0];
        gfx.off(VBLANK_COUNTER).sw(c, w_v1);
        if c.g[A0][0] == c.g[V0][0] {
            c.cycles += 4;
            let wait = gfx.off(VBLANK_WAIT_COUNT).lw(c);
            let early = (c.g[V1][0] as i64) < (wait as i64);
            c.g[V0][0] = early as u64;
            frame.restore_ra(c, 0);
            if early {
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 5;
            c.g[V0][0] = 5;
            c.g[A0][0] = gfx.off(VBLANK_SEMA).lw(c);
            gfx.off(VBLANK_SWAP_STATE).sw(c, 5);
            gfx.off(VBLANK_COUNTER).sw(c, 0);
            call_ptr(c, VBLANK_I_SIGNAL_SEMA, 0x0038_2640)?;
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_ProbeRail (0x1086b8)

/// Rider: the path (`+0x2c`: nodes of 0x20 bytes, position then rotation).
const PROBE_RAIL_PATH: u32 = 0x780;
/// Rider: the current path node index.
const PROBE_RAIL_NODE: u32 = 0x8a0;
/// Rider: the probe offset added to the node position (`vec4`).
const PROBE_RAIL_OFFSET: u32 = 0x9d0;
/// Rider: the blend between the near and far probe radii (0..1).
const PROBE_RAIL_BLEND: u32 = 0x25c;
/// Rider: nonzero picks the short far radius (50 instead of 100).
const PROBE_RAIL_SHORT: u32 = 0x330;
/// Rider: the overlap list of objects near the rider.
const PROBE_RAIL_OVERLAP: u32 = 0x860;
/// `gp-0x7e3c`: 0.001f, the "lines are parallel" threshold.
const PROBE_RAIL_PARALLEL_GP: u32 = 0x7e3c;
/// `OverlapList_FindNearestRail(list, pos, out, mask, radius)`.
const PROBE_RAIL_FIND_NEAREST: u32 = 0x0033_4680;

/// The VU0 dot-product idiom of this function: `vf4 = vf[a] * vf[b]`, then
/// the w lane accumulates y, z and w through `vf6.w = 2` (vf0.w + vf0.w).
fn probe_rail_dot(c: &mut Cpu, a: u8, b: u8) {
    let v = c.vu0();
    v.add(0b1000, 6, 0, Src::Bc(0, W));
    v.mul(0b1111, 4, a, Src::Vec(b));
    v.adda(0b1000, 4, Src::Bc(4, Y));
    v.madda(0b1000, 6, Src::Bc(4, Z));
    v.madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `vf5 = vf3 (+|-) vf4` with vf3/vf4 loaded from `a`/`b`, through the
/// stack temporary `tmp` (`sqc2`, then `lq`/`sq` via r2) into `dst`.
fn probe_rail_vop(c: &mut Cpu, a: u32, b: u32, sub: bool, tmp: u32, dst: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    if sub {
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    } else {
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    }
    c.sqc2(5, tmp);
    c.lq(2, tmp);
    c.sq(2, dst);
}

/// `vf5 = vf4 * vf3.x` (vf4 loaded from `v`, vf3.x = the float in f[fs]
/// moved through GPR `rt`), stored to `tmp` and copied to `dst`.
fn probe_rail_scale(c: &mut Cpu, fs: usize, rt: usize, v: u32, tmp: u32, dst: u32) {
    c.mfc1(rt, fs);
    c.lqc2(4, v);
    c.qmtc2(rt, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, tmp);
    c.lq(2, tmp);
    c.sq(2, dst);
}

/// `|vec at addr| <= near radius (f20)`: the dot idiom on itself, `sqrt`
/// into Q, Q read back through v1 into f0, then `c.le.s f0, f20`.
fn probe_rail_within_near(c: &mut Cpu, addr: u32) -> bool {
    c.lqc2(3, addr);
    probe_rail_dot(c, 3, 3);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
    c.mtc1(3, 0);
    c.c_le_s(0, 20);
    c.fcc()
}

/// `int Rider_ProbeRail(Rider* self, const sVec4* target, const sVec4* dir)`
/// (0x1086b8): whether a rail runs close to the rider's probe point. The
/// probe is the current path node's position plus the probe offset, looking
/// along the node rotation's forward axis; the near radius is
/// `lerp(30, 50, blend)` and the far one `lerp(100 or 50, 170, blend)`.
/// The point on the probe segment (clamped to `±(far - near)`) closest to
/// `target` within the near radius is a hit; otherwise, unless `dir` is
/// (nearly) parallel to the probe, the point closest to the line through
/// `target` along `dir` is asked for the nearest rail in the overlap list
/// (`OverlapList_FindNearestRail`, radius 300), and that rail must lie within
/// the near radius. Returns 1 or 0.
pub fn rider_probe_rail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0x110);
        c.g[T0][0] = c.g[A0][0];
        let me = r32(c, T0);
        frame.save_fpr(c, 20, 0x100);
        frame.save_ra(c, 0xf0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 9); // 1.0
        // node = &path->nodes[index]
        c.g[A0][0] = Addr(me).off(PROBE_RAIL_PATH).lw(c);
        c.g[V0][0] = Addr(me).off(PROBE_RAIL_NODE).lw(c);
        c.g[V1][0] = Addr(r32(c, A0)).off(0x2c).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 5);
        c.g[AT][0] = 0x4248_0000;
        c.mtc1(AT, 10); // 50.0
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        let node = r32(c, V1);
        c.g[AT][0] = 0x41f0_0000;
        c.mtc1(AT, 8); // 30.0
        // The forward axis of the node rotation (quaternion x,y,z,w at
        // +0x10): (1 - 2(y^2 + z^2), 2(xy + wz), 2(xz - wy)), w = 0.
        c.lwc1(4, node + 0x14);
        c.g[A0][0] = c.g[SP][0];
        let fwd = sp(c);
        c.lwc1(5, node + 0x18);
        c.lwc1(0, node + 0x10);
        c.mul_s(1, 4, 4);
        c.lwc1(3, node + 0x1c);
        c.mul_s(7, 5, 5);
        c.mul_s(2, 0, 5);
        Addr(fwd).off(0xc).sw(c, 0);
        c.mul_s(6, 3, 4);
        c.add_s(1, 1, 7);
        c.mul_s(0, 0, 4);
        c.mul_s(3, 3, 5);
        c.sub_s(2, 2, 6);
        c.add_s(1, 1, 1);
        c.add_s(0, 0, 3);
        c.add_s(2, 2, 2);
        c.sub_s(1, 9, 1);
        c.add_s(0, 0, 0);
        c.swc1(2, fwd + 8);
        c.swc1(1, fwd);
        c.swc1(0, fwd + 4);
        // pos (sp+0x10) = node position + probe offset (through sp+0x20).
        probe_rail_vop(c, node, me + PROBE_RAIL_OFFSET, false, fwd + 0x20, fwd + 0x10);
        let pos = fwd + 0x10;
        // Radii: f20 = near = 50 t + 30 (1 - t); f0 = far.
        c.lwc1(2, me + PROBE_RAIL_BLEND);
        c.sub_s(9, 9, 2);
        c.g[V1][0] = Addr(me).off(PROBE_RAIL_SHORT).lw(c);
        c.mul_s(0, 2, 10);
        c.mul_s(8, 9, 8);
        c.add_s(20, 0, 8);
        if c.g[V1][0] == 0 {
            c.cycles += 8;
            c.g[AT][0] = 0x432a_0000;
            c.mtc1(AT, 1); // 170.0
            c.g[AT][0] = 0x42c8_0000;
            c.mtc1(AT, 0); // 100.0
            c.mul_s(1, 2, 1);
            c.mul_s(0, 9, 0);
            c.add_s(0, 1, 0);
        } else {
            c.cycles += 5;
            c.g[AT][0] = 0x432a_0000;
            c.mtc1(AT, 0); // 170.0
            c.mul_s(1, 9, 10); // (1 - t) * 50
            c.mul_s(0, 2, 0);
            c.add_s(0, 0, 1);
        }

        // Clamp the target's projection on the probe to [near - far, far - near].
        c.cycles += 22;
        let target = r32(c, A1);
        probe_rail_vop(c, target, pos, true, fwd + 0x30, fwd + 0x20);
        c.sub_s(4, 20, 0); // lo
        c.sub_s(8, 0, 20); // hi
        c.mov_s(1, 4);
        c.lqc2(3, fwd);
        c.lqc2(5, fwd + 0x20);
        probe_rail_dot(c, 3, 5);
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.c_le_s(4, 0);
        if c.fcc() {
            c.min_s(1, 0, 8);
        }

        // The closest point on the probe: within the near radius is a hit.
        c.cycles += 33;
        probe_rail_scale(c, 1, 3, fwd, fwd + 0x50, fwd + 0x40);
        probe_rail_vop(c, pos, fwd + 0x40, false, fwd + 0x50, fwd + 0x30);
        probe_rail_vop(c, fwd + 0x30, target, true, fwd + 0x50, fwd + 0x20);
        let near = probe_rail_within_near(c, fwd + 0x20);
        c.g[V0][0] = 1;
        if !near {
            // den = 1 - (dir . fwd)^2; parallel lines miss.
            c.cycles += 18;
            let dir = r32(c, A2);
            c.lqc2(3, dir);
            c.lqc2(5, fwd);
            probe_rail_dot(c, 3, 5);
            c.qmfc2(2, 4);
            c.mtc1(2, 3); // dd
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2);
            c.mul_s(0, 3, 3);
            c.lwc1(1, r32(c, GP).wrapping_sub(PROBE_RAIL_PARALLEL_GP));
            c.sub_s(2, 2, 0);
            c.c_lt_s(2, 1);
            let parallel = c.fcc();
            c.g[V0][0] = 0;
            if !parallel {
                // u = ((pos - target) / den) . (dir * dd - fwd), clamped.
                c.cycles += 42;
                probe_rail_vop(c, pos, target, true, fwd + 0x50, fwd + 0x40);
                c.mov_s(1, 4);
                c.mfc1(3, 2);
                c.qmtc2(3, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, fwd + 0x40);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 4, 4, Src::Q);
                c.sqc2(4, fwd + 0x50);
                c.lq(2, fwd + 0x50);
                c.sq(2, fwd + 0x30);
                probe_rail_scale(c, 3, 3, dir, fwd + 0x60, fwd + 0x50);
                probe_rail_vop(c, fwd + 0x50, fwd, true, fwd + 0x60, fwd + 0x40);
                c.lqc2(3, fwd + 0x30);
                c.lqc2(5, fwd + 0x40);
                probe_rail_dot(c, 3, 5);
                c.qmfc2(3, 4);
                c.mtc1(3, 0);
                c.c_le_s(4, 0);
                if c.fcc() {
                    c.min_s(1, 0, 8);
                }

                // p (sp+0x40) = pos + fwd * u; ask the overlap list.
                c.cycles += 22;
                c.mfc1(2, 1);
                c.lqc2(4, fwd);
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, fwd + 0x60);
                c.g[AT][0] = 0x4396_0000;
                c.mtc1(AT, 12); // radius 300.0
                c.lq(2, fwd + 0x60);
                c.g[A1][0] = sx(fwd + 0x40);
                c.g[A2][0] = sx(fwd + 0x70);
                c.g[A3][0] = 1;
                c.sq(2, fwd + 0x50);
                probe_rail_vop(c, pos, fwd + 0x50, false, fwd + 0x60, fwd + 0x40);
                c.g[A0][0] = Addr(me).off(PROBE_RAIL_OVERLAP).lw(c);
                call_ptr(c, PROBE_RAIL_FIND_NEAREST, 0x0010_89d0)?;
                let found = c.g[V0][0] != 0;
                c.g[V0][0] = 0;
                if found {
                    // The rail point must lie within the near radius of p.
                    c.cycles += 22;
                    let sp_now = sp(c);
                    c.lqc2(3, sp_now + 0x40);
                    c.lqc2(4, sp_now + 0x70);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp_now + 0x60);
                    c.g[A0][0] = 1;
                    c.lq(2, sp_now + 0x60);
                    c.sq(2, sp_now + 0x20);
                    c.sq(2, sp_now + 0x50);
                    if !probe_rail_within_near(c, sp_now + 0x20) {
                        c.g[A0][0] = 0;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[A0][0];
                }
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0xf0);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_ResetBloomParams (0x36c740)

/// PS2GraphicsMan: the bloom parameter sets (0x1c bytes each, 7 floats).
const BLOOM_PARAMS: u32 = 0x6cd4;
const BLOOM_PARAMS_SIZE: u64 = 0x1c;
/// The defaults (`D_004A43D4..E4`, `D_004A43F0/F4`) as `gp` offsets, in the
/// order of the 7 fields.
const BLOOM_DEFAULTS_GP: [u32; 7] = [0x12e4, 0x12e8, 0x12ec, 0x12f0, 0x12f4, 0x1300, 0x1304];

/// `void PS2GraphicsMan_ResetBloomParams(PS2GraphicsMan* self, int idx)`
/// (0x36c740): copy the default bloom parameters into set `idx`. Leaves
/// LO/HI = `idx * 0x1c`, a1 = the set's offset in the object, a0 = the
/// set, f0/f1 the last two defaults copied.
pub fn ps2_graphics_man_reset_bloom_params(c: &mut Cpu) {
    c.cycles += 19;
    c.g[V0][0] = BLOOM_PARAMS_SIZE;
    c.mult(0, A1, A1, V0); // a1 = idx * 0x1c
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(BLOOM_PARAMS));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    let set = r32(c, A0);
    let gp = r32(c, GP);
    for (i, off) in BLOOM_DEFAULTS_GP.into_iter().enumerate() {
        // Alternates f1, f0 as the original does.
        let f = if i % 2 == 0 { 1 } else { 0 };
        c.lwc1(f, gp.wrapping_add(off));
        c.swc1(f, set.wrapping_add(4 * i as u32));
    }
    ret(c);
}

// ---------------------------------------------------------------------------
// WorldPainter_GetLightGlow30 (0x2eef18), WorldPainter_GetWeather80 (0x2ee8a0)

/// `float WorldPainter_GetLightGlow30(int i)` (0x2eef18): the light-glow
/// value of world-painter view `i`: vtable slot 18 (`+0x90`) of the object
/// behind `views[i] + 0x8`, returning the float it points at.
pub fn world_painter_get_light_glow30(c: &mut Cpu) {
    crate::game::visualfx::renderstateman::world_painter_virtual_getter(
        c,
        0x8,
        0x90,
        0x002e_ef50,
    );
}

/// `float WorldPainter_GetWeather80(int i)` (0x2ee8a0): the weather value of
/// world-painter view `i`: vtable slot 58 (`+0x1d0`) of the object behind
/// `views[i] + 0x20`, returning the float it points at.
pub fn world_painter_get_weather80(c: &mut Cpu) {
    crate::game::visualfx::renderstateman::world_painter_virtual_getter(
        c,
        0x20,
        0x1d0,
        0x002e_e8d8,
    );
}

// ---------------------------------------------------------------------------
// GameUI_IsPauseMenuOpen (0x20cba0)

/// `gp-0x848`: the world (`D_004A28A8`).
const PAUSE_WORLD_GP: u32 = 0x848;
/// World: the game UI; GameUI +0x48: the UI root, whose state manager is at +0x18.
const PAUSE_WORLD_UI: u32 = 0x84;
const PAUSE_UI_ROOT: u32 = 0x48;
const PAUSE_ROOT_STATE_MGR: u32 = 0x18;
/// The pause menu state's name (`D_0046F818`).
const PAUSE_STATE_NAME: u32 = 0x0046_f818;
/// `GetHashValue32(const char*)`.
const PAUSE_HASH: u32 = 0x0031_7670;
/// `UIStateMgr_FindState(mgr, id)`.
const PAUSE_FIND_STATE: u32 = 0x0039_f9d8;

/// `bool GameUI_IsPauseMenuOpen(void)` (0x20cba0): whether the UI state
/// manager holds the pause menu state (found by the hash of its name).
pub fn game_ui_is_pause_menu_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(PAUSE_WORLD_GP)).lw(c);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        c.g[A0][0] = sx(PAUSE_STATE_NAME);
        c.g[V0][0] = Addr(r32(c, V1)).off(PAUSE_WORLD_UI).lw(c);
        let root = Addr(r32(c, V0)).off(PAUSE_UI_ROOT).lw(c);
        c.g[S0][0] = sx((root as u32).wrapping_add(PAUSE_ROOT_STATE_MGR));
        call_ptr(c, PAUSE_HASH, 0x0020_cbc8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, PAUSE_FIND_STATE, 0x0020_cbd4)?;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = (c.g[V0][0] != 0) as u64;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// VfxRenderToggles_Update (0x390c20)

/// `gp+0x2a90`: the graphics manager (`D_004A5B80`).
const TOGGLES_GFX_GP: u32 = 0x2a90;
/// Graphics manager: the vtable pointer of the render-state interface.
const TOGGLES_GFX_VTABLE: u32 = 0x10d8;
/// Vtable slot 16 (`{delta: i16, fn}` at +0x80): set the render toggles.
const TOGGLES_SLOT: u32 = 0x80;
/// `gp+0x13d0`: the toggle flags (`D_004A44C0`).
const TOGGLES_FLAGS_GP: u32 = 0x13d0;
/// `gp+0x13d8`: a one-shot "changed" latch (`D_004A44C8`), cleared here.
const TOGGLES_LATCH_GP: u32 = 0x13d8;

/// `void VfxRenderToggles_Update(void)` (0x390c20): pass the render toggle
/// flags to the graphics manager (vtable slot 16), then clear the one-shot
/// latch if it is set.
pub fn vfx_render_toggles_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        let gp = r32(c, GP);
        c.g[A2][0] = Addr(gp.wrapping_add(TOGGLES_GFX_GP)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(gp.wrapping_add(TOGGLES_FLAGS_GP)).lw(c);
        let gfx = r32(c, A2);
        c.g[V0][0] = Addr(gfx).off(TOGGLES_GFX_VTABLE).lw(c);
        let vt = r32(c, V0);
        c.g[A0][0] = Addr(vt).off(TOGGLES_SLOT).lh(c);
        c.g[V1][0] = Addr(vt).off(TOGGLES_SLOT + 4).lw(c);
        let method = r32(c, V1);
        c.g[A0][0] = sx(gfx.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0039_0c44)?;
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(TOGGLES_LATCH_GP)).lw(c);
        if c.g[V0][0] != 0 {
            Addr(gp.wrapping_add(TOGGLES_LATCH_GP)).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MeshAnim_Render (0x352810)

/// MeshAnim: the model.
const MESH_ANIM_MODEL: u32 = 0x18;
/// MeshAnim: the fade (nothing is drawn at or below 0).
const MESH_ANIM_FADE: u32 = 0x1c;
/// MeshAnim: the draw alpha limit (the alpha passed is `min(fade, this)`
/// when this is >= 0 and >= fade, else 1.0).
const MESH_ANIM_ALPHA: u32 = 0x24;
/// MeshAnim: the bone rotations (quaternions, 16 bytes each).
const MESH_ANIM_ROTATIONS: u32 = 0x30;
/// MeshAnim: the bone translations (16 bytes each).
const MESH_ANIM_TRANSLATIONS: u32 = 0x3b0;
/// Model: the skeleton (`+4`: bone count, `+0xc`: the part count list).
const MESH_ANIM_SKELETON: u32 = 0x80;
/// Model: the part list (`+4`: part pointers; part `+0`: i16 object id).
const MESH_ANIM_PARTS: u32 = 0x94;
/// `gp+0x16c8`: the object visibility table holder.
const MESH_ANIM_VIS_GP: u32 = 0x16c8;
/// `gp+0x2a90`: the graphics manager.
const MESH_ANIM_GFX_GP: u32 = 0x2a90;
/// Graphics manager: render-state vtable; slot `+0x2f8`: draw skinned mesh.
const MESH_ANIM_GFX_VTABLE: u32 = 0x10d8;
const MESH_ANIM_DRAW_SLOT: u32 = 0x2f8;

/// The visibility test of one part (registers as the original: a2 the
/// packed id, a1 the id, t0 the visibility tables, t1 = !0xff, t8 = 0xff).
/// Leaves v0 = 1 when the part's object is visible.
fn mesh_anim_part_visible(c: &mut Cpu) {
    c.cycles += 11;
    let part = Addr(r32(c, A3)).lw(c);
    c.g[V0][0] = part;
    c.g[A2][0] &= c.g[T6][0];
    c.g[A2][0] |= c.g[T7][0];
    c.g[V1][0] = Addr(part as u32).lh(c);
    c.g[A0][0] = c.g[A2][0] & 0xff;
    c.g[V1][0] = sx(r32(c, V1) << 8);
    c.g[A0][0] |= c.g[V1][0];
    c.g[A2][0] = c.g[A0][0];
    c.g[A0][0] = c.g[A2][0] & 0xff;
    let other_table = c.g[A0][0] != c.g[T8][0];
    c.g[A1][0] = sx(r32(c, A2) >> 8);
    let t0 = r32(c, T0);
    // The object record (v1), or invisible.
    let have = if other_table {
        // (A table index other than 0xff: never taken, the low byte is
        // always 0xff here.)
        c.cycles += 5;
        c.g[V1][0] = Addr(t0).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(8).lw(c);
        let inside = c.g[A0][0] < c.g[V0][0];
        c.g[V0][0] = sx(r32(c, A0) << 2);
        if !inside {
            c.cycles += 1;
            c.g[V0][0] = 0;
            false
        } else {
            c.cycles += 5;
            c.g[V1][0] = Addr(t0).off(8).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            let some = c.g[V1][0] != 0;
            c.g[V0][0] = 0;
            some
        }
    } else {
        c.cycles += 2;
        c.g[V1][0] = Addr(t0).off(4).lw(c);
        true
    };
    if have {
        c.cycles += 4;
        let rec = r32(c, V1);
        c.g[V0][0] = Addr(rec).off(0x50).lw(c);
        if c.g[V0][0] < c.g[A1][0] {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V0][0] = Addr(rec).off(0x4c).lw(c);
            c.g[V1][0] = sx(r32(c, A1) << 2);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            c.g[A0][0] &= c.g[T1][0];
            c.g[V0][0] = (c.g[A0][0] != 0) as u64;
        }
    }
}

/// One bone of the matrix loop: the 3x3 rotation of the quaternion at
/// `a0 - 0x380` (VU0 macro code, kept op for op) with the translation at
/// `a0` as the fourth row, written to the stack at v1 (0x40 bytes).
fn mesh_anim_bone_matrix(c: &mut Cpu) {
    let a0 = r32(c, A0);
    let out = r32(c, V1);
    c.lqc2(4, a0.wrapping_sub(MESH_ANIM_TRANSLATIONS - MESH_ANIM_ROTATIONS));
    let v = c.vu0();
    v.add(0b1110, 1, 0, Src::Bc(0, W));
    v.add(0b1110, 5, 4, Src::Vec(4));
    v.sub(0b0001, 10, 10, Src::Vec(10));
    v.sub(0b0001, 11, 11, Src::Vec(11));
    v.sub(0b0001, 12, 12, Src::Vec(12));
    v.mul(0b1110, 6, 5, Src::Vec(4));
    v.mul(0b1110, 7, 5, Src::Bc(4, W));
    v.opmula(5, 4);
    v.madd(0b1110, 8, 0, Src::Vec(0));
    v.suba(0b1000, 1, Src::Bc(6, Y));
    v.msub(0b1000, 10, 1, Src::Bc(6, Z));
    v.suba(0b0100, 1, Src::Bc(6, Z));
    v.msub(0b0100, 11, 1, Src::Bc(6, X));
    v.suba(0b0010, 1, Src::Bc(6, X));
    v.msub(0b0010, 12, 1, Src::Bc(6, Y));
    v.adda(0b0100, 0, Src::Bc(8, Z));
    v.madd(0b0100, 10, 1, Src::Bc(7, Z));
    v.adda(0b0010, 0, Src::Bc(8, X));
    v.madd(0b0010, 11, 1, Src::Bc(7, X));
    v.adda(0b0100, 0, Src::Bc(8, X));
    v.msub(0b0100, 12, 1, Src::Bc(7, X));
    v.adda(0b0010, 0, Src::Bc(8, Y));
    v.msub(0b0010, 10, 1, Src::Bc(7, Y));
    v.adda(0b1000, 0, Src::Bc(8, Z));
    v.msub(0b1000, 11, 1, Src::Bc(7, Z));
    v.adda(0b1000, 0, Src::Bc(8, Y));
    v.madd(0b1000, 12, 1, Src::Bc(7, Y));
    c.sqc2(0, out + 0x30);
    c.sqc2(10, out);
    c.sqc2(11, out + 0x10);
    c.sqc2(12, out + 0x20);
    c.lq(2, a0);
    c.sq(2, out + 0x30);
}

/// `void MeshAnim_Render(MeshAnim* self)` (0x352810): draw an animated mesh
/// instance. Nothing when its fade is <= 0 or when any of the model's parts
/// belongs to an object that is not visible this frame. Otherwise builds
/// one 4x4 matrix per bone on the stack (rotation from the bone quaternion,
/// translation row) and calls the graphics manager's skinned-mesh draw
/// (slot `+0x2f8`) with the model, its part list, the matrices, the alpha
/// (f12) and the fixed draw parameters (a3 = 0x4ff1a0, t1 = 0, t2 = 0x420).
pub fn mesh_anim_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x610);
        c.g[T4][0] = c.g[A0][0];
        let me = r32(c, T4);
        frame.save_ra(c, 0x600);
        c.mtc1(0, 1);
        c.lwc1(0, me + MESH_ANIM_FADE);
        c.c_le_s(0, 1);
        let faded = c.fcc();
        frame.restore_ra(c, 0x600);
        if faded {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }

        // Every part's object must be visible.
        c.cycles += 7;
        c.g[V0][0] = Addr(me).off(MESH_ANIM_MODEL).lw(c);
        c.g[T2][0] = 0;
        let model = r32(c, V0);
        c.g[V1][0] = Addr(model).off(MESH_ANIM_SKELETON).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
        c.g[T3][0] = Addr(r32(c, A0)).lw(c);
        c.g[T5][0] = c.g[V0][0];
        if (c.g[T3][0] as i64) > 0 {
            c.cycles += 10;
            c.g[V0][0] = Addr(r32(c, GP).wrapping_add(MESH_ANIM_VIS_GP)).lw(c);
            c.g[T1][0] = 0xffff_ffff_ffff_0000;
            c.g[V1][0] = Addr(r32(c, T5)).off(MESH_ANIM_PARTS).lw(c);
            c.g[T8][0] = 0xff;
            c.g[T0][0] = Addr(r32(c, V0)).lw(c);
            c.g[T7][0] = 0xff;
            c.g[A3][0] = sx(r32(c, V1).wrapping_add(4));
            c.g[T6][0] = 0xffff_ffff_ffff_ff00;
            c.g[T1][0] |= 0xff00;
            loop {
                mesh_anim_part_visible(c);
                c.cycles += 3;
                c.g[V0][0] ^= 1;
                let hidden = c.g[V0][0] != 0;
                c.g[T2][0] = sx(r32(c, T2).wrapping_add(1));
                if hidden {
                    c.cycles += 1;
                    frame.restore_ra(c, 0x600);
                    c.cycles += 2;
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 3;
                let more = (c.g[T2][0] as i64) < (c.g[T3][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_2878);
                }
            }
        }

        // The bone matrices.
        c.cycles += 7;
        c.g[V0][0] = 0x17;
        c.g[V1][0] = u64::MAX;
        c.g[A2][0] = Addr(r32(c, T5)).off(MESH_ANIM_SKELETON).lw(c);
        c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
        c.g[A1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[A0][0] = sx(me.wrapping_add(MESH_ANIM_TRANSLATIONS));
            c.g[V1][0] = c.g[SP][0];
            loop {
                c.cycles += 40;
                mesh_anim_bone_matrix(c);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                let more = (c.g[A1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x40));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_2940);
                }
            }
        }

        // The alpha (f12): 1.0 unless 0 <= limit, then the fade if it is
        // within the limit.
        c.cycles += 8;
        c.lwc1(1, me + MESH_ANIM_ALPHA);
        c.mtc1(0, 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 12);
        c.c_le_s(0, 1);
        let limited = c.fcc();
        c.g[T3][0] = Addr(r32(c, GP).wrapping_add(MESH_ANIM_GFX_GP)).lw(c);
        if limited {
            c.cycles += 5;
            c.lwc1(0, me + MESH_ANIM_FADE);
            c.c_le_s(0, 1);
            if c.fcc() {
                c.mov_s(12, 0);
            } else {
                c.cycles += 1;
                c.g[T3][0] = Addr(r32(c, GP).wrapping_add(MESH_ANIM_GFX_GP)).lw(c);
            }
        }

        c.cycles += 12;
        c.g[A3][0] = sx(0x0050_0000u32.wrapping_sub(0xe60));
        c.g[A1][0] = Addr(me).off(MESH_ANIM_MODEL).lw(c);
        let gfx = r32(c, T3);
        c.g[V1][0] = Addr(gfx).off(MESH_ANIM_GFX_VTABLE).lw(c);
        c.g[T0][0] = c.g[SP][0];
        c.g[A2][0] = Addr(r32(c, A1)).off(MESH_ANIM_PARTS).lw(c);
        c.g[T1][0] = 0;
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(MESH_ANIM_DRAW_SLOT).lh(c);
        c.g[T2][0] = 0x420;
        c.g[V0][0] = Addr(vt).off(MESH_ANIM_DRAW_SLOT + 4).lw(c);
        let method = r32(c, V0);
        c.g[A0][0] = sx(gfx.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0035_2a48)?;
        c.cycles += 1;
        frame.restore_ra(c, 0x600);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::DATA;
    use crate::game::visualfx::renderstateman::tests::table_getter_setup;

    /// Views 0..3 with a fake slot-36 method and a random blend.
    #[test]
    fn world_painter_get_surface08_matches() {
        fakes::covered(
            ee::s09::f_002ef038__generated,
            world_painter_get_surface08,
            200,
            &|c, seed| {
                table_getter_setup(c, seed, 0x2ef0_3800, SURFACE08_HOLDER, SURFACE08_SLOT);
                let i = c.g[A0][0] as u32;
                let entry = SURFACE08_VIEWS + i * 0xf0;
                let blend = [0.0f32, 0.25, 1.0, -3.5][(seed / 4 % 4) as usize];
                c.bus.write32(entry + SURFACE08_BLEND, blend.to_bits());
                c.pc = 0x002e_f038;
            },
        );
    }

    /// The 16-pool-node classes of `pool_allocator` (block >= 16) filled so
    /// a value read past the register file looks like a counted table
    /// reference (type 4 at +4/+8, a table pointer at +0/+0xc of each 16
    /// bytes), which makes the "release the old reference" branches reachable.
    fn luno_call_fill_pools(c: &mut Cpu, table: u32) {
        let mut nodes = DATA + 0x24_000;
        for block in [8u32, 0x10, 0x20, 0x40, 0x80, 0x100, 0x200] {
            if block >= 0x10 {
                for n in 0..16 {
                    let node = nodes + block * n;
                    for off in (4..block).step_by(4) {
                        let v = match off % 16 {
                            4 | 8 => 4,
                            _ => table,
                        };
                        c.bus.write32(node + off, v);
                    }
                }
            }
            nodes += block * 16;
        }
    }

    /// A script whose function record has `n` registers and no code (the
    /// interpreter returns at once and frees the register file), the
    /// allocator fixtures, and the default value of each type.
    #[test]
    fn luno_vm_call_matches() {
        use crate::difftest::check;
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        check(ee::s05::f_002224b8__generated, luno_vm_call, 120, &|c, seed| {
            let mut r = alloc_machine(c, seed, 0x2224_b8);
            let state = DATA + 0x1000;
            let script = DATA + 0x1100;
            let out = DATA + 0x1200;
            let table = DATA + 0x1800; // refcount at +8
            let table2 = DATA + 0x1900;
            let gc_table = DATA + 0x1a00; // what the pool fill points at
            c.bus.write32(table + 8, 100);
            c.bus.write32(table2 + 8, 100);
            c.bus.write32(gc_table + 8, 100);
            if seed % 2 == 0 {
                luno_call_fill_pools(c, gc_table);
            }
            // state = {script, function 0}; script+4 = code offset 0x10, the
            // record at script+0x10: {_, start pc 0, n registers, 0 locals}.
            c.bus.write32(state, script);
            c.bus.write32(state + 4, 0);
            c.bus.write32(script + 4, 0x10);
            c.bus.write32(script + 0x14, 0);
            c.bus.write32(script + 0x18, seed % 4); // n = 0..3
            c.bus.write32(script + 0x1c, 0);
            // the table argument: a cLunoValue whose word 0 is the table.
            let arg = DATA + 0x1300;
            c.bus.write32(arg, table);
            // the default value: a table reference, a pair, or a plain word.
            let d = LUNO_CALL_DEFAULT_VALUE;
            let (word0, ty) = match seed / 4 % 3 {
                0 => (table2, 4),
                1 => (r.next_u32(), 5),
                _ => (r.next_u32(), 2),
            };
            c.bus.write32(d, word0);
            c.bus.write32(d + 4, r.next_u32());
            c.bus.write32(d + 8, ty);
            c.g[A0][0] = sx(out);
            c.g[A1][0] = sx(state);
            c.g[A2][0] = sx(arg);
            c.g[A3][0] = sx(DATA + 0x1400);
            c.pc = 0x0022_24b8;
        });
    }

    /// Swap state 4 or not, the counter below/at/above the wait count; the
    /// `iSignalSema` syscall goes to the fakes (and unwinds on some seeds).
    #[test]
    fn ps2_graphics_man_on_vblank_start_matches() {
        fakes::covered(
            ee::s12::f_003825f8__generated,
            ps2_graphics_man_on_vblank_start,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3825_f8, 1);
                let gfx = DATA + 0x100;
                let state = if seed % 4 == 3 { r.next_u32() % 8 } else { 4 };
                let count = r.next_u32() % 6;
                let wait = match seed / 4 % 3 {
                    0 => count + 1, // reached exactly
                    1 => r.next_u32() % 4,
                    _ => count + 3, // still waiting
                };
                c.bus.write32(gfx + VBLANK_SWAP_STATE, state);
                c.bus.write32(gfx + VBLANK_COUNTER, count);
                c.bus.write32(gfx + VBLANK_WAIT_COUNT, wait);
                c.bus.write32(gfx + VBLANK_SEMA, r.next_u32() % 64);
                c.g[A0][0] = sx(gfx);
                c.pc = 0x0038_25f8;
            },
        );
    }

    /// The rider on a path node (identity or random rotation), the probe
    /// offset, blend and short-radius flag; a target near the probe (a hit),
    /// at mid range or far; a direction parallel to the probe (identity
    /// rotation, dir = +x) or random; the overlap list with one plain rail
    /// whose mask passes or not, or empty. The overlap query is real (the
    /// same bounded data as `rider_find_rail_matches`).
    #[test]
    fn rider_probe_rail_matches() {
        use crate::difftest::{check, elf_image, put_vec};
        check(ee::s00::f_001086b8__generated, rider_probe_rail, 300, &|c, seed| {
            elf_image(c);
            let mut r = fakes::machine(c, seed, 0x1086_b8, 0);
            let me = DATA + 0x1000;
            let path = DATA + 0x3000;
            let nodes = DATA + 0x3100;
            let list = DATA + 0x4000;
            let rail = DATA + 0x5000;
            let rail_flags = DATA + 0x5200;
            let target = DATA + 0x7000;
            let dir = DATA + 0x7100;
            c.bus.write32(me + PROBE_RAIL_PATH, path);
            c.bus.write32(path + 0x2c, nodes);
            let node = r.next_u32() % 4;
            c.bus.write32(me + PROBE_RAIL_NODE, node);
            let identity = seed % 5 == 0;
            let mut node_pos = [0f32; 3];
            for i in 0..4 {
                let p = [r.f32(100.0), r.f32(100.0), r.f32(100.0)];
                if i == node {
                    node_pos = p.map(f32::from_bits);
                }
                put_vec(c, nodes + 0x20 * i, [p[0], p[1], p[2], 0]);
                let q = if identity {
                    [0, 0, 0, 1.0f32.to_bits()]
                } else {
                    [r.f32(0.7), r.f32(0.7), r.f32(0.7), r.f32(1.0)]
                };
                put_vec(c, nodes + 0x20 * i + 0x10, q);
            }
            let off = [r.f32(5.0), r.f32(5.0), r.f32(5.0)];
            put_vec(c, me + PROBE_RAIL_OFFSET, [off[0], off[1], off[2], 0]);
            c.bus.write32(me + PROBE_RAIL_BLEND, (f32::from_bits(r.f32(0.5)) + 0.5).to_bits());
            c.bus.write32(me + PROBE_RAIL_SHORT, r.next_u32() & 1);
            // The target: near the probe point, mid range, or far.
            let spread = [8.0f32, 60.0, 400.0][(seed / 5 % 3) as usize];
            let t: Vec<u32> = (0..3)
                .map(|i| {
                    let d = f32::from_bits(r.f32(spread));
                    (node_pos[i] + f32::from_bits(off[i]) + d).to_bits()
                })
                .collect();
            put_vec(c, target, [t[0], t[1], t[2], 0]);
            let d = if identity && seed % 2 == 0 {
                [1.0f32.to_bits(), 0, 0, 0]
            } else {
                [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]
            };
            put_vec(c, dir, d);
            // The overlap list: one plain rail (type 1) or none.
            c.bus.write32(me + PROBE_RAIL_OVERLAP, list);
            c.bus.write32(list + 0x210, (seed % 7 != 0) as u32);
            c.bus.write32(list + 0x214, rail);
            c.bus.write32(rail + 8, 1);
            c.bus.write32(rail + 0x68, rail_flags);
            c.bus.write32(rail_flags + 0x1c, (seed % 11 != 5) as u32);
            c.bus.write32(rail_flags + 0x28, r.next_u32());
            for off in [0x6c, 0x70, 0x74] {
                c.bus.write32(rail + off, (-1.0e4f32).to_bits());
            }
            for off in [0x78, 0x7c, 0x80] {
                c.bus.write32(rail + off, 1.0e4f32.to_bits());
            }
            for i in 0..4 {
                let w = if i == 3 { 1.0f32.to_bits() } else { 0 };
                let base = if i == 3 { node_pos } else { [0.0; 3] };
                let v: Vec<u32> = (0..3)
                    .map(|k| (base[k] + f32::from_bits(r.f32(50.0))).to_bits())
                    .collect();
                put_vec(c, rail + 0x10 + 0x10 * i, [v[0], v[1], v[2], w]);
            }
            c.g[A0][0] = sx(me);
            c.g[A1][0] = sx(target);
            c.g[A2][0] = sx(dir);
            c.pc = 0x0010_86b8;
        });
    }

    /// Random defaults in `.sdata`, set index 0..7 on a random object.
    #[test]
    fn ps2_graphics_man_reset_bloom_params_matches() {
        use crate::difftest::{check_lohi, scramble, Rng, GAME_GP};
        check_lohi(
            ee::s12::f_0036c740__generated,
            ps2_graphics_man_reset_bloom_params,
            200,
            &|c, seed| {
                let mut r = Rng(0x36_c740 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                for off in BLOOM_DEFAULTS_GP {
                    c.bus.write32(GAME_GP + off, r.edge_f32());
                }
                c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                c.g[A1][0] = (seed % 8) as u64;
                c.pc = 0x0036_c740;
            },
        );
    }

    /// Views 0..3, a fake slot-18 method behind `views[i] + 0x8`.
    #[test]
    fn world_painter_get_light_glow30_matches() {
        fakes::covered(
            ee::s09::f_002eef18__generated,
            world_painter_get_light_glow30,
            200,
            &|c, seed| {
                table_getter_setup(c, seed, 0x2eef_1800, 0x8, 0x90);
                c.pc = 0x002e_ef18;
            },
        );
    }

    /// Views 0..3, a fake slot-58 method behind `views[i] + 0x20`.
    #[test]
    fn world_painter_get_weather80_matches() {
        fakes::covered(
            ee::s09::f_002ee8a0__generated,
            world_painter_get_weather80,
            200,
            &|c, seed| {
                table_getter_setup(c, seed, 0x2ee8_a000, 0x20, 0x1d0);
                c.pc = 0x002e_e8a0;
            },
        );
    }

    /// The world -> game UI -> root -> state manager chain and a manager
    /// whose two lists (background, stack) hold 0..3 states; one of them is
    /// the pause menu (id = the hash of its name) on some seeds. The hash and
    /// the search are real.
    #[test]
    fn game_ui_is_pause_menu_open_matches() {
        use crate::difftest::{check, elf_image, GAME_GP};
        check(ee::s05::f_0020cba0__generated, game_ui_is_pause_menu_open, 200, &|c, seed| {
            elf_image(c);
            let mut r = fakes::machine(c, seed, 0x20_cba0, 0);
            let world = DATA + 0x100;
            let ui = DATA + 0x200;
            let root = DATA + 0x300;
            let mgr = root + PAUSE_ROOT_STATE_MGR;
            c.bus.write32(GAME_GP - PAUSE_WORLD_GP, world);
            c.bus.write32(world + PAUSE_WORLD_UI, ui);
            c.bus.write32(ui + PAUSE_UI_ROOT, root);
            let name: Vec<u8> = (0..64)
                .map(|i| c.bus.read8(PAUSE_STATE_NAME + i))
                .take_while(|&b| b != 0)
                .collect();
            let pause = crate::game::hashvalue::str_hash(&name);
            let open = seed % 3 == 0;
            // Background list at mgr+0, stack at mgr+0x1c (anchor link at +4),
            // states 0x40 apart, each list ending at a self-linked sentinel.
            for (k, (list, base)) in [(mgr, DATA + 0x1000), (mgr + 0x1c, DATA + 0x2000)]
                .into_iter()
                .enumerate()
            {
                let n = r.next_u32() % 4;
                let mut prev = list;
                for i in 0..n {
                    let s = base + 0x40 * i;
                    let id = if open && k as u32 == seed / 3 % 2 && i + 1 == n {
                        pause
                    } else {
                        r.next_u32() % 6
                    };
                    c.bus.write32(s + 0xc, id);
                    c.bus.write32(prev + 4, s);
                    c.bus.write32(s, prev);
                    prev = s;
                }
                let sentinel = base + 0x800;
                c.bus.write32(prev + 4, sentinel);
                c.bus.write32(sentinel, prev);
                c.bus.write32(sentinel + 4, sentinel);
            }
            c.pc = 0x0020_cba0;
        });
    }

    /// The graphics manager with a fake slot-16 method, random flags, the
    /// latch set or clear.
    #[test]
    fn vfx_render_toggles_update_matches() {
        use crate::difftest::GAME_GP;
        fakes::covered(ee::s12::f_00390c20__generated, vfx_render_toggles_update, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x39_0c20, 1);
            let gfx = fakes::ptr(&mut r);
            let vt = fakes::vtable(c, &mut r, &[TOGGLES_SLOT]);
            c.bus.write32(GAME_GP + TOGGLES_GFX_GP, gfx);
            c.bus.write32(gfx + TOGGLES_GFX_VTABLE, vt);
            c.bus.write32(GAME_GP + TOGGLES_FLAGS_GP, r.next_u32());
            let latch = if seed % 2 == 0 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(GAME_GP + TOGGLES_LATCH_GP, latch);
            c.pc = 0x0039_0c20;
        });
    }

    /// A mesh instance with fade <= 0 (nothing drawn) or > 0; 0..4 parts
    /// whose objects are visible (flag word with bits above the low byte),
    /// hidden or out of the table's range; 0..4 bones with random
    /// quaternions and translations; the alpha limit negative, below or
    /// above the fade. The draw method is a fake (unwinding on some seeds).
    #[test]
    fn mesh_anim_render_matches() {
        use crate::difftest::{put_vec, GAME_GP};
        fakes::covered(ee::s11::f_00352810__generated, mesh_anim_render, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x35_2810, 1);
            let me = DATA + 0x1000;
            let model = DATA + 0x2000;
            let skeleton = DATA + 0x2100;
            let counts = DATA + 0x2200;
            let parts = DATA + 0x2300;
            let vis_holder = DATA + 0x2500;
            let vis = DATA + 0x2600;
            let rec = DATA + 0x2700;
            let flags = DATA + 0x2800;
            let fade = match seed % 9 {
                0 => 0.0f32,
                1 => -0.5,
                _ => f32::from_bits(r.f32(1.0)).abs() + 0.01,
            };
            c.bus.write32(me + MESH_ANIM_FADE, fade.to_bits());
            let alpha = match seed / 9 % 3 {
                0 => -1.0f32,
                1 => fade * 0.5,
                _ => fade + 1.0,
            };
            c.bus.write32(me + MESH_ANIM_ALPHA, alpha.to_bits());
            c.bus.write32(me + MESH_ANIM_MODEL, model);
            c.bus.write32(model + MESH_ANIM_SKELETON, skeleton);
            c.bus.write32(skeleton + 0xc, counts);
            let n_parts = r.next_u32() % 5;
            c.bus.write32(counts, n_parts);
            c.bus.write32(model + MESH_ANIM_PARTS, parts);
            // Visibility: object ids 0..7 against a table of 6.
            c.bus.write32(GAME_GP + MESH_ANIM_VIS_GP, vis_holder);
            c.bus.write32(vis_holder, vis);
            c.bus.write32(vis + 4, rec);
            c.bus.write32(rec + 0x50, 6);
            c.bus.write32(rec + 0x4c, flags);
            let all_visible = seed % 3 != 0;
            for i in 0..8 {
                let w = if all_visible || r.next_u32() % 4 != 0 {
                    (r.next_u32() | 0x100) & !0xff | (r.next_u32() & 0xff)
                } else {
                    r.next_u32() & 0xff
                };
                c.bus.write32(flags + 4 * i, w);
            }
            for i in 0..n_parts {
                let part = DATA + 0x2400 + 0x10 * i;
                c.bus.write32(parts + 4 + 4 * i, part);
                let id = if all_visible { r.next_u32() % 7 } else { r.next_u32() % 9 };
                c.bus.write16(part, id as u16);
            }
            // Bones.
            let n_bones = r.next_u32() % 5;
            c.bus.write32(skeleton + 4, n_bones);
            for i in 0..4 {
                put_vec(
                    c,
                    me + MESH_ANIM_ROTATIONS + 0x10 * i,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)],
                );
                put_vec(
                    c,
                    me + MESH_ANIM_TRANSLATIONS + 0x10 * i,
                    [r.f32(100.0), r.f32(100.0), r.f32(100.0), 1.0f32.to_bits()],
                );
            }
            // The graphics manager with a fake draw method.
            let gfx = DATA + 0x8000;
            let vt = DATA + 0x9000;
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + MESH_ANIM_DRAW_SLOT, delta as u16);
            c.bus.write32(vt + MESH_ANIM_DRAW_SLOT + 4, fakes::FAKE);
            c.bus.write32(GAME_GP + MESH_ANIM_GFX_GP, gfx);
            c.bus.write32(gfx + MESH_ANIM_GFX_VTABLE, vt);
            c.g[A0][0] = sx(me);
            c.pc = 0x0035_2810;
        });
    }
}
