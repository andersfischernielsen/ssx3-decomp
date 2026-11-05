//! `src/util/objectinterface.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::worldpainter::quadtree::create_with;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002d_1ce0,
        world_get_instances_man,
        "World_GetInstancesMan",
    ),
    (0x002d_2058, world_is_occluded, "World_IsOccluded"),
    (0x002d_2100, ssx_stats_man_create, "SSXStatsMan_Create"),
    (
        0x002d_21b0,
        stats_process_update_stub,
        "StatsProcess_UpdateStub",
    ),
    (
        0x002d_21b8,
        stats_process_render_stub,
        "StatsProcess_RenderStub",
    ),
    (0x002d_21c0, ssx_stats_man_ctor, "SSXStatsMan_ctor"),
    (
        0x002d_1d10,
        halo_modifier_draw_quad_2,
        "HaloModifier_DrawQuad",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::worldpainter::quadtree::callee::AI_VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::CONSOLE_CONFIG_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EMITTER_RENDER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_ON_VIF1_DMA_END;
    pub(crate) use crate::game::worldpainter::quadtree::callee::INTC_REMOVE_HANDLER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_SET_STATE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MENU_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MOD_OBJ_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::PS2_SOUND_MANAGER_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SNDSTRM_GET_REQUEST_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_FADE_OUT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_GET_STREAM_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_POST_EVENT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SPEECH_INSTANCE_STOP_STREAM;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STATS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STRCPY;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_SET_FILE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_STOP;
    pub(crate) use crate::game::worldpainter::quadtree::callee::VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_GET_SECTION_SCRIPT_FUNC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_OBJ_CTOR;
}

/// `void* World_GetInstancesMan(void)` (0x2d1ce0): follow the world singleton
/// to the instance manager at +0x88.
pub fn world_get_instances_man(c: &mut Cpu) {
    c.cycles += 4;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = world;
    let man = Addr(world as u32).off(0x84).lw(c);
    c.g[V1][0] = man;
    c.g[V0][0] = Addr(man as u32).off(0x88).lw(c);
    abi::ret(c);
}

/// `SSXStatsMan* SSXStatsMan_ctor(SSXStatsMan* self)` (0x2d21c0): publish the
/// singleton pointer at `gp + 0x910`.
pub fn ssx_stats_man_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr((c.g[GP][0] as u32).wrapping_add(0x910)).sw(c, c.g[A0][0]);
    abi::ret(c);
}

/// A `gp`-relative global address.
#[inline(always)]
pub(crate) fn gp_at(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `int World_IsOccluded(sQ128* p)` (0x2d2058): over the occlusion manager
/// `*(world + 0x84) -> +0x8c`, for each instance whose bit is set in the
/// candidate mask (`+0x110`, `count` at `+0x10c`), run the VU0 clip-test
/// microprogram on the instance's six quadwords at `+0x50..+0x90` and the
/// point `p`; the microprogram reports "occluded" in `vi1 == 1`. 1 when any
/// instance occludes the point, else 0.
pub fn world_is_occluded(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = Addr(gp_at(c, -0x848)).lw(c);
    c.g[A1][0] = 0;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
    c.g[A3][0] = Addr(c.g[V1][0] as u32).off(0x8c).lw(c);
    let inst = c.g[A3][0] as u32;
    c.g[V0][0] = Addr(inst).off(0x10c).lw(c);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[T1][0] = Addr(inst).off(0x110).lw(c);
    if !empty {
        c.cycles += 5;
        c.g[V0][0] = 0;
        c.g[T2][0] = 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x828));
        c.g[A2][0] = sx(inst.wrapping_add(0x8));
        // The microprogram address, as VI27 expects it (byte address >> 3).
        c.g[T0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
        loop {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[T2][0] as u32) << ((c.g[A1][0] as u32) & 31));
            c.g[V0][0] = c.g[T1][0] & c.g[V0][0];
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr(inst).off(0x10c).lw(c);
            } else {
                c.cycles += 15;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                let m = c.g[V0][0] as u32;
                c.lqc2(1, m.wrapping_add(0x50));
                c.lqc2(2, m.wrapping_add(0x60));
                c.lqc2(3, m.wrapping_add(0x70));
                c.lqc2(4, m.wrapping_add(0x80));
                c.lqc2(5, m.wrapping_add(0x90));
                c.lqc2(6, c.g[A0][0] as u32);
                c.ctc2(8, 27);
                c.vu0().vnop();
                c.vu0().vnop();
                c.vcallmsr();
                c.cfc2(3, 1);
                c.g[V1][0] ^= 0x1;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = Addr(inst).off(0x10c).lw(c);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    abi::ret(c);
                    return;
                }
            }
            c.cycles += 4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_2088);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Stats and visual effects constructors.

/// `void* SSXStatsMan_Create()` (0x2d2100): allocate the one-byte stats
/// manager singleton and construct it.
pub fn ssx_stats_man_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 1, 0x0048_6758, 0, 0x002d_2120, |c, ret| {
            callee::STATS_MAN_CTOR.call(c, ret)
        })
    })
}

/// `void StatsProcess_RenderStub(void)` (0x2d21b8): stats render hook, not
/// installed.
pub fn stats_process_render_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void StatsProcess_UpdateStub(void)` (0x2d21b0): stats update hook, not
/// installed.
pub fn stats_process_update_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

pub(crate) mod halo {

    /// Render context: current packet write pointer.
    pub const PACKET: u32 = 0xe84;

    /// Render context / world: vtable.
    pub const VTABLE: u32 = 0x10d8;

    /// Render context: per-blend-mode halfwords (`(mode + 0x25) * 4 + 0xf50`).
    pub const MODE_WORDS: u32 = 0xf50;

    /// vtable slots {delta (i16), fn}: world "skip draw" test, quad draw
    /// with UV scale, quad draw.
    pub const SLOT_SKIP: u32 = 0x2d8;

    pub const SLOT_DRAW_SCALED: u32 = 0x278;

    pub const SLOT_DRAW: u32 = 0x270;

    /// The GS register-set template copied into the packet (0x14 bytes).
    pub const TEMPLATE: u32 = 0x0050_1420;
}

/// `void HaloModifier_DrawQuad(const Vec4* pos, const Vec4* colour, int mode,
/// float size /* f12 */, float stretch /* f13 */)` (0x2d1d10): draw one halo
/// sprite unless the world's skip test (vtable slot 0x2d8) says no. Pushes
/// a 0x14-byte register template onto the render packet, patches its
/// fields (alpha/blend setup, the mode's halfword), then draws: a plain
/// unit quad when `stretch` is 0, else a quad stretched by `stretch *
/// k` (k at gp-0x3f98), and for modes 1 and 4 a second, mirrored one
/// (`-stretch * k`). Pops the template again.
pub fn halo_modifier_draw_quad(c: &mut Cpu) {
    use halo::*;
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 28;
        let frame = Frame::open(c, 0x120);
        let world = Addr(gp.wrapping_sub(0x854)).lw(c);
        c.g[A3][0] = world;
        frame.save(c, S0, 0xe0);
        frame.save(c, S2, 0xc0);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0xb0);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 23, 0x108);
        c.g[S3][0] = c.g[A2][0];
        frame.save_fpr(c, 22, 0x100);
        let stretch = abi::farg(c, 1);
        c.f[23] = stretch.0;
        frame.save(c, S1, 0xd0);
        c.f[22] = c.f[12]; // size
        frame.save(c, S4, 0xa0);
        c.g[A1][0] = c.g[S2][0];
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 24, 0x110);
        frame.save_fpr(c, 21, 0xf8);
        frame.save_fpr(c, 20, 0xf0);
        // world->ShouldSkipHalo(pos)
        let vt = Addr((world as u32).wrapping_add(VTABLE)).lw(c);
        c.g[V1][0] = vt;
        let delta = Addr((vt as u32).wrapping_add(SLOT_SKIP)).lh(c);
        c.g[A0][0] = delta;
        let f = Addr((vt as u32).wrapping_add(SLOT_SKIP + 4)).lw(c);
        c.g[V0][0] = f;
        c.g[A0][0] = sx((world as u32).wrapping_add(delta as u32));
        call_ptr(c, f as u32, 0x002d_1d78)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0xe0);
        } else {
            halo_push_and_draw(c, frame, gp, stretch)?;
            // Pop the template.
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
            let ctx = r32(c, S1);
            Addr(ctx.wrapping_add(PACKET)).sw(c, c.g[V0][0]);
            frame.restore(c, S0, 0xe0);
        }
        c.cycles += 14;
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 24, 0x110);
        frame.restore_fpr(c, 23, 0x108);
        frame.restore_fpr(c, 22, 0x100);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// The body of `HaloModifier_DrawQuad` after the skip test: push and patch
/// the template, draw. Leaves the packet pointer in v0.
pub(crate) fn halo_push_and_draw(c: &mut Cpu, frame: Frame, gp: u32, stretch: FpuF32) -> Flow {
    use halo::*;
    c.cycles += 98;
    // The colour onto the stack (w, x, y, z order of the loads).
    let colour = Addr(r32(c, S0));
    c.f[3] = colour.off(8).read_fpu(c).0;
    c.g[A0][0] = 0x50_0000;
    c.f[0] = colour.off(0xc).read_fpu(c).0;
    c.g[A1][0] = 0xffff_ffff_ffbf_ffff;
    c.f[1] = colour.read_fpu(c).0;
    c.f[2] = colour.off(4).read_fpu(c).0;
    c.g[T4][0] = 0x40_0000;
    c.swc1(0, frame.at(c, 0));
    c.g[T5][0] = 0xffff_ffff_ffff_fffc;
    c.swc1(1, frame.at(c, 4));
    c.g[A3][0] = 0xffff_ffff_e000_03ff;
    c.swc1(2, frame.at(c, 8));
    c.swc1(3, frame.at(c, 0xc));
    c.g[T6][0] = 0xffff_ffff_ffff_ff83;
    let ctx = Addr(gp.wrapping_add(0x2a90)).lw(c);
    c.g[S1][0] = ctx;
    let ctx = Addr(ctx as u32);
    c.g[T7][0] = 0xffff_ffff_ffff_fc1f;
    c.g[S0][0] = 0x80_0000;
    let packet = ctx.off(PACKET).lw(c);
    c.g[V0][0] = packet;
    c.g[T0][0] = 0xffff_ffff_fe7f_ffff;
    // Duplicate the last 0x14 bytes of the packet.
    let p = Addr(packet as u32);
    c.g[V1][0] = p.read_u64_unaligned(c);
    c.g[A2][0] = p.off(8).read_u64_unaligned(c);
    c.g[T2][0] = p.off(0x10).lw(c);
    p.off(0x14).write_u64_unaligned(c, c.g[V1][0]);
    p.off(0x1c).write_u64_unaligned(c, c.g[A2][0]);
    p.off(0x24).sw(c, c.g[T2][0]);
    c.g[T1][0] = 0xffff_ffff_ffcf_ffff;
    c.g[T3][0] = 0xffff_ffff_fff0_0fff;
    c.g[T2][0] = 0x1_4000;
    // Advance the packet and copy the template over the new entry.
    let packet = ctx.off(PACKET).lw(c);
    c.g[V1][0] = packet;
    let p = Addr(packet as u32);
    c.f[21] = 0;
    c.g[V0][0] = sx(p.0.wrapping_add(0x14));
    let mode_word = ctx.0.wrapping_add(r32(c, S3).wrapping_add(0x25) << 2);
    ctx.off(PACKET).sw(c, c.g[V0][0]);
    c.g[A2][0] = sx(mode_word);
    let template = Addr(TEMPLATE);
    c.g[S4][0] = sx(TEMPLATE);
    c.g[S5][0] = template.read_u64_unaligned(c);
    c.g[S6][0] = template.off(8).read_u64_unaligned(c);
    c.g[V0][0] = template.off(0x10).lw(c);
    p.off(0x14).write_u64_unaligned(c, c.g[S5][0]);
    p.off(0x1c).write_u64_unaligned(c, c.g[S6][0]);
    p.off(0x24).sw(c, c.g[V0][0]);
    let plain = stretch.eq_s(FpuF32::ZERO);
    abi::set_fcc(c, plain);
    // Patch the new entry's words: (pointer register, offset, and, or).
    const EDITS: [(usize, u32, u64, u64); 8] = [
        (A0, 0x4, 0xffff_ffff_ffbf_ffff, 0x40_0000),
        (V1, 0x4, 0xffff_ffff_ffff_fffc, 0),
        (A0, 0x8, 0xffff_ffff_e000_03ff, 0),
        (V1, 0x4, 0xffff_ffff_ffff_ff83, 0x1c),
        (A0, 0x8, 0xffff_ffff_ffff_fc1f, 0x100),
        (A1, 0x4, 0xffff_ffff_fe7f_ffff, 0x80_0000),
        (V1, 0x4, 0xffff_ffff_ffcf_ffff, 0),
        (A0, 0x4, 0xffff_ffff_fff0_0fff, 0x1_4000),
    ];
    for &(reg, off, and, or) in EDITS.iter() {
        let entry = ctx.off(PACKET).lw(c);
        c.g[reg][0] = entry;
        let at = Addr((entry as u32).wrapping_add(off));
        c.g[V0][0] = (at.lw(c) & and) | or;
        at.sw(c, c.g[V0][0]);
    }
    c.g[V1][0] = Addr(mode_word.wrapping_add(MODE_WORDS)).lw(c);
    c.g[V0][0] = ctx.off(PACKET).lw(c);
    Addr(r32(c, V0).wrapping_add(0x10)).sh(c, c.g[V1][0]);

    if plain {
        // A unit quad: size x size, uv (0, 0)-(1, 1).
        c.cycles += 19;
        let vt = ctx.off(VTABLE).lw(c);
        c.g[V1][0] = vt;
        c.g[A1][0] = c.g[S2][0];
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = FpuF32::ONE.0;
        c.g[A2][0] = sx(frame.at(c, 0x10));
        let delta = Addr((vt as u32).wrapping_add(SLOT_DRAW)).lh(c);
        c.g[A3][0] = sx(frame.at(c, 0x20));
        c.swc1(22, frame.at(c, 0x14));
        c.g[T0][0] = sx(frame.at(c, 0x60));
        c.swc1(21, frame.at(c, 0x24));
        c.g[A0][0] = sx(ctx.0.wrapping_add(delta as u32));
        c.swc1(0, frame.at(c, 0x64));
        c.swc1(22, frame.at(c, 0x10));
        c.swc1(21, frame.at(c, 0x20));
        c.swc1(0, frame.at(c, 0x60));
        let f = Addr((vt as u32).wrapping_add(SLOT_DRAW + 4)).lw(c);
        c.g[V0][0] = f;
        c.g[T1][0] = c.g[SP][0];
        call_ptr(c, f as u32, 0x002d_2010)?;
        c.g[V0][0] = ctx.off(PACKET).lw(c);
        return Ok(());
    }
    // Stretched quad, scale stretch * k.
    c.cycles += 25;
    let vt = ctx.off(VTABLE).lw(c);
    c.g[V1][0] = vt;
    c.g[S0][0] = sx(frame.at(c, 0x10));
    c.g[AT][0] = 0x3f80_0000;
    let one = FpuF32::ONE;
    c.f[20] = one.0;
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = Addr((vt as u32).wrapping_add(SLOT_DRAW_SCALED)).lh(c);
    c.g[A2][0] = c.g[S0][0];
    let k = Addr(gp.wrapping_sub(0x3f98)).read_fpu(c);
    c.f[24] = k.0;
    c.g[A3][0] = sx(frame.at(c, 0x20));
    c.swc1(22, frame.at(c, 0x10));
    c.g[A0][0] = sx(ctx.0.wrapping_add(r32(c, A0)));
    c.swc1(22, frame.at(c, 0x14));
    c.f[12] = (stretch * k).0;
    c.swc1(21, frame.at(c, 0x20));
    c.g[T0][0] = sx(frame.at(c, 0x30));
    c.swc1(21, frame.at(c, 0x24));
    c.g[T1][0] = c.g[SP][0];
    c.swc1(20, frame.at(c, 0x30));
    c.swc1(20, frame.at(c, 0x34));
    let f = Addr((vt as u32).wrapping_add(SLOT_DRAW_SCALED + 4)).lw(c);
    c.g[V0][0] = f;
    c.g[T2][0] = 1;
    call_ptr(c, f as u32, 0x002d_1f60)?;
    c.g[V1][0] = 1;
    c.g[V0][0] = 4;
    let mut mirrored = c.g[S3][0] == 1;
    if !mirrored {
        c.cycles += 2;
        mirrored = c.g[S3][0] == 4;
    }
    if mirrored {
        // The mirrored half: scale -stretch * k.
        c.cycles += 21;
        let vt = ctx.off(VTABLE).lw(c);
        c.g[V1][0] = vt;
        let neg = -stretch;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = Addr((vt as u32).wrapping_add(SLOT_DRAW_SCALED)).lh(c);
        c.g[A3][0] = sx(frame.at(c, 0x40));
        c.swc1(22, frame.at(c, 0x14));
        c.f[12] = (neg * k).0;
        c.swc1(21, frame.at(c, 0x44));
        c.g[A0][0] = sx(ctx.0.wrapping_add(r32(c, A0)));
        c.swc1(20, frame.at(c, 0x54));
        c.g[T0][0] = sx(frame.at(c, 0x50));
        c.swc1(22, frame.at(c, 0x10));
        c.g[T1][0] = c.g[SP][0];
        c.swc1(21, frame.at(c, 0x40));
        c.swc1(20, frame.at(c, 0x50));
        let f = Addr((vt as u32).wrapping_add(SLOT_DRAW_SCALED + 4)).lw(c);
        c.g[V0][0] = f;
        c.g[T2][0] = 1;
        call_ptr(c, f as u32, 0x002d_1fc0)?;
    }
    c.g[V0][0] = ctx.off(PACKET).lw(c);
    Ok(())
}

// ---------------------------------------------------- HaloModifier_DrawQuad

/// `void HaloModifier_DrawQuad(const Vec4* pos, void* tex, int mode, float
/// size, float spin)` (0x2d1d10): draw a camera-facing halo sprite. Unless
/// the world's visibility test (virtual `+0x2d8` of `world+0x10d8`) rejects
/// `pos`, append a 0x14-byte GS packet to the render context's packet
/// (`ctx+0xe84`): the 0x14 bytes before the cursor are copied after it,
/// the GS state template at 0x501420 after that, the header's fields
/// patched (additive alpha blend, primitive 0x1c, ...) and the texture word
/// `ctx[0xf50 + 4 (mode + 0x25)]` stored at +0x10. Then the quad: with no
/// spin (`spin == 0`) one call to the drawer's sprite method (`+0x270`),
/// else its rotated-quad method (`+0x278`) with angle `spin * k`, and for
/// modes 1 and 4 a second, mirrored one (`-spin * k`). The cursor then
/// steps back by 0x14.
pub fn halo_modifier_draw_quad_2(c: &mut Cpu) {
    /// `gp-0x854`: the world; `gp+0x2a90`: the render context.
    const WORLD_GP: u32 = 0x854;
    const CTX_GP: u32 = 0x2a90;
    /// The render context's packet cursor, drawer and texture words.
    const CTX_PACKET: u32 = 0xe84;
    const CTX_DRAWER: u32 = 0x10d8;
    const CTX_TEXTURES: u32 = 0xf50;
    /// The GS state template copied into each halo packet.
    const STATE_TEMPLATE: u32 = 0x0050_1420;
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x120);
        c.g[A3][0] = c.lw(r32(c, GP).wrapping_sub(WORLD_GP));
        frame.save(c, S0, 0xe0);
        frame.save(c, S2, 0xc0);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0xb0);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 23, 0x108);
        c.g[S3][0] = c.g[A2][0];
        frame.save_fpr(c, 22, 0x100);
        c.f[23] = c.f[13];
        frame.save(c, S1, 0xd0);
        c.f[22] = c.f[12];
        frame.save(c, S4, 0xa0);
        c.g[A1][0] = c.g[S2][0];
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 24, 0x110);
        frame.save_fpr(c, 21, 0xf8);
        frame.save_fpr(c, 20, 0xf0);
        // world->drawer->IsCulled(pos) (virtual +0x2d8).
        c.g[V1][0] = c.lw(r32(c, A3).wrapping_add(CTX_DRAWER));
        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x2d8));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x2dc));
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002d_1d78)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0xe0);
        } else {
            draw_halo_packet(c)?;
            frame.restore(c, S0, 0xe0);
        }
        c.cycles += 14;
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 24, 0x110);
        frame.restore_fpr(c, 23, 0x108);
        frame.restore_fpr(c, 22, 0x100);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    });

    /// The visible case (0x2d1d80..0x2d2020): build the packet, draw.
    fn draw_halo_packet(c: &mut Cpu) -> crate::call::Flow {
        c.cycles += 98;
        // The quad's colour (tex[0..4] rotated: w, x, y, z) -> sp+0.
        let tex = r32(c, S0);
        let sp0 = sp(c);
        c.lwc1(3, tex.wrapping_add(0x8));
        c.g[A0][0] = 0x50_0000;
        c.lwc1(0, tex.wrapping_add(0xc));
        c.g[A1][0] = 0xffff_ffff_ffbf_ffff;
        c.lwc1(1, tex);
        c.lwc1(2, tex.wrapping_add(0x4));
        c.g[T4][0] = 0x40_0000;
        c.swc1(0, sp0);
        c.g[T5][0] = 0xffff_ffff_ffff_fffc;
        c.swc1(1, sp0 + 0x4);
        c.g[A3][0] = 0xffff_ffff_e000_03ff;
        c.swc1(2, sp0 + 0x8);
        c.swc1(3, sp0 + 0xc);
        c.g[T6][0] = 0xffff_ffff_ffff_ff83;
        let ctx = c.lw(r32(c, GP).wrapping_add(CTX_GP));
        c.g[S1][0] = ctx;
        let ctx = ctx as u32;
        c.g[T7][0] = 0xffff_ffff_ffff_fc1f;
        c.g[S0][0] = 0x80_0000;
        // Repeat the previous 0x14-byte packet after the cursor.
        let cur = c.lw(ctx.wrapping_add(CTX_PACKET));
        c.g[V0][0] = cur;
        let cur = cur as u32;
        c.g[T0][0] = 0xffff_ffff_fe7f_ffff;
        c.g[T1][0] = 0xffff_ffff_ffcf_0000;
        c.g[T3][0] = 0xffff_ffff_fff0_0000;
        c.ldl(V1, cur.wrapping_add(0x7));
        c.ldr(V1, cur);
        c.ldl(A2, cur.wrapping_add(0xf));
        c.ldr(A2, cur.wrapping_add(0x8));
        c.g[T2][0] = c.lw(cur.wrapping_add(0x10));
        c.sdl(V1, cur.wrapping_add(0x1b));
        c.sdr(V1, cur.wrapping_add(0x14));
        c.sdl(A2, cur.wrapping_add(0x23));
        c.sdr(A2, cur.wrapping_add(0x1c));
        c.sw(cur.wrapping_add(0x24), c.g[T2][0]);
        c.g[T1][0] |= 0xffff;
        c.g[T3][0] |= 0xfff;
        c.g[T2][0] = 0x1_4000;
        // Advance the cursor; copy the state template into the new packet.
        let cur = c.lw(ctx.wrapping_add(CTX_PACKET));
        c.g[V1][0] = cur;
        let cur = cur as u32;
        c.g[A2][0] = sx(r32(c, S3).wrapping_add(0x25));
        c.f[21] = 0;
        c.g[V0][0] = sx(cur.wrapping_add(0x14));
        c.g[A2][0] = sx(r32(c, A2) << 2);
        c.sw(ctx.wrapping_add(CTX_PACKET), c.g[V0][0]);
        c.g[A2][0] = sx(ctx.wrapping_add(r32(c, A2)));
        c.g[S4][0] = sx(STATE_TEMPLATE);
        c.ldl(S5, STATE_TEMPLATE + 0x7);
        c.ldr(S5, STATE_TEMPLATE);
        c.ldl(S6, STATE_TEMPLATE + 0xf);
        c.ldr(S6, STATE_TEMPLATE + 0x8);
        c.g[V0][0] = c.lw(STATE_TEMPLATE + 0x10);
        c.sdl(S5, cur.wrapping_add(0x1b));
        c.sdr(S5, cur.wrapping_add(0x14));
        c.sdl(S6, cur.wrapping_add(0x23));
        c.sdr(S6, cur.wrapping_add(0x1c));
        c.sw(cur.wrapping_add(0x24), c.g[V0][0]);
        // spin == 0 ?
        c.c_eq_s(23, 21);
        // Patch the header words of the packet.
        let patch = |c: &mut Cpu, p: usize, off: u32, and: usize, or: Option<u64>| {
            c.g[p][0] = c.lw(ctx.wrapping_add(CTX_PACKET));
            let at = r32(c, p).wrapping_add(off);
            c.g[V0][0] = c.lw(at);
            c.g[V0][0] &= c.g[and][0];
            if let Some(v) = or {
                c.g[V0][0] |= v;
            }
            c.sw(at, c.g[V0][0]);
        };
        let (t4, s0, t2) = (c.g[T4][0], c.g[S0][0], c.g[T2][0]);
        patch(c, A0, 0x4, A1, Some(t4));
        patch(c, V1, 0x4, T5, None);
        patch(c, A0, 0x8, A3, None);
        patch(c, V1, 0x4, T6, Some(0x1c));
        patch(c, A0, 0x8, T7, Some(0x100));
        patch(c, A1, 0x4, T0, Some(s0));
        patch(c, V1, 0x4, T1, None);
        patch(c, A0, 0x4, T3, Some(t2));
        // The texture word at +0x10.
        c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(CTX_TEXTURES));
        c.g[V0][0] = c.lw(ctx.wrapping_add(CTX_PACKET));
        let no_spin = c.fcc();
        c.sh(r32(c, V0).wrapping_add(0x10), c.g[V1][0]);

        if no_spin {
            // drawer->Sprite(pos, &{size, size}, &{0, 0}, &{1, 1}, &colour).
            c.cycles += 19;
            let sp0 = sp(c);
            c.g[V1][0] = c.lw(ctx.wrapping_add(CTX_DRAWER));
            c.g[A1][0] = c.g[S2][0];
            c.g[AT][0] = 0x3f80_0000;
            c.f[0] = 0x3f80_0000;
            c.g[A2][0] = sx(sp0 + 0x10);
            c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x270));
            c.g[A3][0] = sx(sp0 + 0x20);
            c.swc1(22, sp0 + 0x14);
            c.g[T0][0] = sx(sp0 + 0x60);
            c.swc1(21, sp0 + 0x24);
            c.g[A0][0] = sx(ctx.wrapping_add(r32(c, A0)));
            c.swc1(0, sp0 + 0x64);
            c.swc1(22, sp0 + 0x10);
            c.swc1(21, sp0 + 0x20);
            c.swc1(0, sp0 + 0x60);
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x274));
            let target = r32(c, V0);
            c.g[T1][0] = c.g[SP][0];
            call_ptr(c, target, 0x002d_2010)?;
        } else {
            // drawer->RotatedSprite(pos, &{size, size}, &{0, 0}, &{1, 1},
            // &colour, 1, spin * k).
            c.cycles += 25;
            let sp0 = sp(c);
            c.g[V1][0] = c.lw(ctx.wrapping_add(CTX_DRAWER));
            c.g[S0][0] = sx(sp0 + 0x10);
            c.g[AT][0] = 0x3f80_0000;
            c.f[20] = 0x3f80_0000;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x278));
            c.g[A2][0] = c.g[S0][0];
            c.lwc1(24, r32(c, GP).wrapping_sub(0x3f98));
            c.g[A3][0] = sx(sp0 + 0x20);
            c.swc1(22, sp0 + 0x10);
            c.g[A0][0] = sx(ctx.wrapping_add(r32(c, A0)));
            c.swc1(22, sp0 + 0x14);
            c.mul_s(12, 23, 24);
            c.swc1(21, sp0 + 0x20);
            c.g[T0][0] = sx(sp0 + 0x30);
            c.swc1(21, sp0 + 0x24);
            c.g[T1][0] = c.g[SP][0];
            c.swc1(20, sp0 + 0x30);
            c.swc1(20, sp0 + 0x34);
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x27c));
            let target = r32(c, V0);
            c.g[T2][0] = 1;
            call_ptr(c, target, 0x002d_1f60)?;
            c.g[V1][0] = 1;
            c.g[V0][0] = 4;
            let mut mirrored = c.g[S3][0] == 1;
            if !mirrored {
                c.cycles += 2;
                mirrored = c.g[S3][0] == 4;
            }
            if mirrored {
                // The mirrored quad (angle -spin * k).
                c.cycles += 21;
                let sp0 = sp(c);
                c.g[V1][0] = c.lw(ctx.wrapping_add(CTX_DRAWER));
                c.f[12] = c.f[23] ^ 0x8000_0000;
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x278));
                c.g[A3][0] = sx(sp0 + 0x40);
                c.swc1(22, sp0 + 0x14);
                c.mul_s(12, 12, 24);
                c.swc1(21, sp0 + 0x44);
                c.g[A0][0] = sx(ctx.wrapping_add(r32(c, A0)));
                c.swc1(20, sp0 + 0x54);
                c.g[T0][0] = sx(sp0 + 0x50);
                c.swc1(22, sp0 + 0x10);
                c.g[T1][0] = c.g[SP][0];
                c.swc1(21, sp0 + 0x40);
                c.swc1(20, sp0 + 0x50);
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x27c));
                let target = r32(c, V0);
                c.g[T2][0] = 1;
                call_ptr(c, target, 0x002d_1fc0)?;
            }
        }
        // Step the cursor back over the drawn packet.
        c.g[V0][0] = c.lw(ctx.wrapping_add(CTX_PACKET));
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
        c.sw(ctx.wrapping_add(CTX_PACKET), c.g[V0][0]);
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::object::debouncenode::tests::base;
    pub(crate) use crate::game::object::debouncenode::tests::SEEDS;
    pub(crate) use crate::game::worldpainter::quadtree::tests::alloc_machine;
    pub(crate) use crate::game::worldpainter::quadtree::tests::heap_init;
    pub(crate) use crate::game::worldpainter::quadtree::tests::pool_allocator;
    pub(crate) use crate::game::worldpainter::quadtree::tests::run;
    pub(crate) use crate::game::worldpainter::quadtree::tests::MEM_HEAP;

    #[test]
    fn world_get_instances_man_matches() {
        check(
            ssx3_ee::s09::f_002d1ce0,
            world_get_instances_man,
            50,
            &|c, seed| {
                let mut r = Rng(0x1ce0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = DATA + 0x100;
                let man = DATA + 0x200;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, man);
                c.bus.write32(man + 0x88, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_stats_man_ctor_matches() {
        check(
            ssx3_ee::s09::f_002d21c0,
            ssx_stats_man_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0x21c0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
            },
        );
    }

    /// The VU0 microprogram at 0x828 the occlusion test runs: `vi1 = vi14`.
    /// Hand-written microcode has no translation: use the reference
    /// interpreter.
    pub(crate) fn put_occlusion_microprogram(c: &mut Cpu) {
        ssx3_ps2::platform::machine_of(&mut c.bus.plat)
            .hw
            .vu0
            .fallback = Some(ssx3_ps2::vu::reference::run_program);
        let at = 0x1100_0000 + 0x828;
        c.bus.write32(at, 0x8000_7070); // IADD vi1, vi14, vi0
        c.bus.write32(at + 4, 0x4000_02ff); // NOP, E
    }

    #[test]
    fn world_is_occluded_matches() {
        check(
            ee::s09::f_002d2058__generated,
            world_is_occluded,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x2d20);
                put_occlusion_microprogram(c);
                // vi14 (the object's clip word the microprogram copies to vi1):
                // 1 means "occluded".
                c.g[14][0] = (seed % 3) as u64;
                c.ctc2(14, 14);
                let (world, mgr, inst, pts) = (DATA, DATA + 0x100, DATA + 0x200, DATA + 0x3000);
                c.bus.write32((GAME_GP as u32).wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, mgr);
                c.bus.write32(mgr + 0x8c, inst);
                let count = (seed % 4) as u32;
                c.bus.write32(inst + 0x10c, count);
                c.bus.write32(
                    inst + 0x110,
                    if count == 0 {
                        0
                    } else {
                        (r.next_u32() | 1) & 0xf
                    },
                );
                for i in 0..3u32 {
                    let m = DATA + 0x1000 + 0x100 * i;
                    c.bus.write32(inst + 8 + 8 * i, m);
                    for q in 0..5u32 {
                        put_vec(
                            c,
                            m + 0x50 + 0x10 * q,
                            [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                        );
                    }
                }
                put_vec(
                    c,
                    pts,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                c.g[A0][0] = sx(pts);
            },
        );
    }

    #[test]
    fn ssx_stats_man_create_matches() {
        check(
            ee::s09::f_002d2100__generated,
            ssx_stats_man_create,
            60,
            &|c, seed| {
                alloc_machine(c, seed, 0x2d21_00);
            },
        );
    }

    #[test]
    fn stats_process_render_stub_matches() {
        leaf(
            ssx3_ee::s09::f_002d21b8__generated,
            stats_process_render_stub,
            7,
        );
    }

    #[test]
    fn stats_process_update_stub_matches() {
        leaf(
            ssx3_ee::s09::f_002d21b0__generated,
            stats_process_update_stub,
            8,
        );
    }
    use super::*;

    // ---------------------------------------------------- HaloModifier_DrawQuad

    /// `HaloModifier_DrawQuad`: culled or visible (a real `return 0` method
    /// or a fake), no spin / spin, modes 0..5 (1 and 4 draw the mirrored
    /// quad), a packet cursor at various alignments (the unaligned copies).
    /// The drawer's methods are fakes that log their arguments.
    #[test]
    fn halo_modifier_draw_quad_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        const RETURN_ZERO: u32 = 0x003a_5ab0;
        check(
            ee::s09::f_002d1d10__generated,
            halo_modifier_draw_quad_2,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d1d, 3);
                c.pc = 0x002d_1d10;
                let (world, wvt, ctx, cvt, tex, pos) = (
                    DATA + 0x10000,
                    DATA + 0x14000,
                    DATA + 0x18000,
                    DATA + 0x1c000,
                    DATA + 0x100,
                    DATA + 0x200,
                );
                let cur = DATA + 0x2000 + 4 * (seed % 4);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), world);
                c.bus.write32(world + 0x10d8, wvt);
                c.bus.write16(wvt + 0x2d8, (r.next_u32() & 0x7c) as u16);
                c.bus
                    .write32(wvt + 0x2dc, if seed % 5 == 0 { FAKE } else { RETURN_ZERO });
                c.bus.write32(GAME_GP + 0x2a90, ctx);
                fill(c, &mut r, ctx + 0xf50, 0x30);
                c.bus.write32(ctx + 0xe84, cur);
                c.bus.write32(ctx + 0x10d8, cvt);
                for (k, off) in [0x270u32, 0x278].into_iter().enumerate() {
                    c.bus.write16(cvt + off, (r.next_u32() & 0x7c) as u16);
                    c.bus.write32(cvt + off + 4, FAKE + 0x10 * (k as u32 + 1));
                }
                fill(c, &mut r, cur - 0x20, 0x20);
                fill(c, &mut r, 0x0050_1420, 6);
                fill(c, &mut r, tex, 4);
                fill(c, &mut r, pos, 4);
                c.bus.write32(GAME_GP.wrapping_sub(0x3f98), r.f32(1.0));
                c.g[A0][0] = sx(pos);
                c.g[A1][0] = sx(tex);
                c.g[A2][0] = sx((seed / 2) % 6);
                c.f[12] = r.f32(50.0);
                c.f[13] = if seed % 3 == 0 { 0 } else { r.f32(6.0) };
            },
        );
    }
}
