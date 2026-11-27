//! `src/render/bezierman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::fe::debugfe::at as at_2;
pub(crate) use crate::game::fe::festateload::at;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_b190, ps2_bezier_man_v22, "PS2BezierMan_v22"),
    (0x0038_b338, ps2bezier_man_v21, "PS2BezierMan_v21"),
    (
        0x0038_ca08,
        ps2bezier_man_test_patch_bounds,
        "PS2BezierMan_TestPatchBounds",
    ),
    (0x0038_d638, ps2_bezier_man_call_v17, "PS2BezierMan_CallV17"),
    (
        0x0038_c788,
        ps2_bezier_man_build_patch_packet,
        "PS2BezierMan_BuildPatchPacket",
    ),
    (
        0x0038_d968,
        ps2_bezier_man_render_patch_fx,
        "PS2BezierMan_RenderPatchFX",
    ),
    (0x0038_aef8, ps2_bezier_man_create, "PS2BezierMan_Create"),
    (0x0038_af30, cps2_bezier_man_init, "PS2BezierMan_Init"),
    (0x0038_b158, ps2_bezier_man_v04, "PS2BezierMan_v04"),
    (
        0x0038_b0f8,
        ps2_bezier_man_free_caches_2,
        "PS2BezierMan_FreeCaches",
    ),
    (
        0x0038_ca70,
        ps2_bezier_man_emit_patch_type1,
        "PS2BezierMan_EmitPatchType1",
    ),
    (
        0x0038_ce20,
        ps2_bezier_man_emit_patch_type2,
        "PS2BezierMan_EmitPatchType2",
    ),
    (
        0x0038_d168,
        ps2_bezier_man_emit_patch_matrix_pass,
        "PS2BezierMan_EmitPatchMatrixPass",
    ),
    (
        0x0038_d660,
        ps2_bezier_man_free_caches_stub,
        "PS2BezierMan_FreeCachesStub",
    ),
    (
        0x0038_d448,
        ps2_bezier_man_emit_patch_projector_passes,
        "PS2BezierMan_EmitPatchProjectorPasses",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `PS2GraphicsMan_CullAgainstLights`.
    pub const CULL_AGAINST_LIGHTS: Callee = Callee {
        addr: 0x0037_df88,
        f: ee::s12::PS2GraphicsMan_CullAgainstLights,
    };

    /// `float RaceProgress_ProjectRider(int i)`.
    pub const PROJECT_RIDER: Callee = Callee {
        addr: 0x0021_0820,
        f: ee::s05::RaceProgress_ProjectRider,
    };

    // Cross-shard calls the generator emitted as `call_indirect`.
    pub const RIDER_GET_DIST_TO_FINISH: u32 = 0x0011_3128;

    pub const RIDER_SPHERE_TREE_CTOR: u32 = 0x0032_9910;

    pub const RIDER_SPHERE_TREE_GET_BOUNDS: u32 = 0x0032_9dc8;

    pub const SND_LOCK: u32 = 0x003b_58a0;

    pub const SND_UNLOCK: u32 = 0x003b_58d8;

    pub const SND_STREAM_POSITION: u32 = 0x003b_6670;

    pub const SND_SUB_TIME: u32 = 0x003b_8ad8;

    pub const SND_SUB_STATUS: u32 = 0x003b_86c0;

    pub const SND_STREAM_INFO: u32 = 0x003b_7b48;

    pub const SND_SUB_STATE: u32 = 0x003b_7c40;

    pub const SND_STATUS: u32 = 0x003b_7f18;
}

/// `gp`-relative globals used here.
pub(crate) mod g {}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// A float's bit pattern (test fixtures).
#[inline(always)]
#[cfg(test)]
pub(crate) fn f(x: f32) -> u32 {
    x.to_bits()
}

// ---------------------------------------------------------------------------
// PS2BezierMan_v21.

/// `void PS2BezierMan_v21(PS2BezierMan* self, void* a, void* b, void* c)`
/// (0x38b338): the manager's entry, which forwards to its renderer (vtable
/// slot 22, a `{short delta; void (*fn)();}` entry at `*(self+4)+0xb0`)
/// unless the global at `0x4a44d4` disables it.
pub fn ps2bezier_man_v21(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw(gp(c, 0x13e4));
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[T2][0] = c.g[A0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 6;
            let vt = c.lw(self_.wrapping_add(4));
            c.g[V0][0] = vt;
            c.g[T1][0] = 0;
            c.g[A0][0] = Addr((vt as u32).wrapping_add(0xb0)).lh(c);
            let func = c.lw((vt as u32).wrapping_add(0xb4));
            c.g[V1][0] = func;
            c.g[A0][0] = sx(self_.wrapping_add(r32(c, A0)));
            call_ptr(c, func as u32, 0x0038_b364)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2BezierMan_TestPatchBounds.

/// `void PS2BezierMan_TestPatchBounds(PS2BezierMan* self, Patch* patch)`
/// (0x38ca08): the patch's two corner points (+0x158 and +0x164, w = 1) are
/// handed to the graphics manager at `*(0x4a5b80)`'s light cull.
pub fn ps2bezier_man_test_patch_bounds(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let sp = sp(c);
        let self_ = r32(c, A1);
        c.g[V0][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = c.g[SP][0]; // a = first point
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 5); // f5 = 1.0
        c.g[A2][0] = sx(sp.wrapping_add(0x10)); // b = second point
        c.lwc1(6, self_.wrapping_add(0x16c));
        c.lwc1(4, self_.wrapping_add(0x158));
        c.lwc1(0, self_.wrapping_add(0x15c));
        c.lwc1(1, self_.wrapping_add(0x160));
        c.lwc1(2, self_.wrapping_add(0x164));
        c.lwc1(3, self_.wrapping_add(0x168));
        c.g[A0][0] = c.lw(gp(c, 0x2a90));
        c.swc1(4, sp);
        c.swc1(0, sp.wrapping_add(4));
        c.swc1(1, sp.wrapping_add(8));
        c.swc1(2, sp.wrapping_add(0x10));
        c.swc1(3, sp.wrapping_add(0x14));
        c.swc1(6, sp.wrapping_add(0x18));
        c.swc1(5, sp.wrapping_add(0x1c));
        c.g[RA][0] = sx(0x0038_ca64);
        c.swc1(5, sp.wrapping_add(0xc));
        callee::CULL_AGAINST_LIGHTS.call(c, 0x0038_ca64)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2BezierMan_CallV17(PS2BezierMan* self)` (0x38d638): call slot 0x11
/// (`+0x88` delta, `+0x8c` method) of the object at `self->+0x4`.
pub fn ps2_bezier_man_call_v17(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(abi::arg(c, 0), 4)).lw(c);
        let obj = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(at(obj, 0x88)).lh(c);
        c.g[A1][0] = Addr(at(obj, 0x8c)).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0038_d654)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void PS2BezierMan_v22(PS2BezierMan* self, void* obj, u8* types, void*
/// extra)` (0x38B190): queue a bezier patch. Four equal type bytes go to the
/// per-size list for that size (4, 6 or 8); a mixed patch goes to the mix list
/// with kind 2 when any type is 8, else 0.
pub fn ps2_bezier_man_v22(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = c.lbu(ea(c.g[A2][0], 0));
    c.g[V0][0] = c.lbu(ea(c.g[A2][0], 2));
    let mut uniform = c.g[A3][0] == c.g[V0][0];
    if uniform {
        c.cycles += 4;
        c.g[V1][0] = c.lbu(ea(c.g[A2][0], 1));
        c.g[V0][0] = c.lbu(ea(c.g[A2][0], 3));
        uniform = c.g[V1][0] == c.g[V0][0];
        if uniform {
            c.cycles += 2;
            uniform = c.g[A3][0] == c.g[V1][0];
            if !uniform {
                c.g[A3][0] = c.lw(ea(c.g[A0][0], 0x4d84));
            }
        } else {
            c.g[A3][0] = c.lw(ea(c.g[A0][0], 0x4d84));
        }
    } else {
        c.g[A3][0] = c.lw(ea(c.g[A0][0], 0x4d84));
    }
    if uniform {
        // One list per patch size: 4 (+0x458, 900 entries), 6 (+0x3c9c,
        // 220) and 8 (+0x4a60, 50); the entry is 16 bytes at the list base.
        c.cycles += 4;
        c.g[A2][0] = c.g[A3][0];
        c.g[V0][0] = 6;
        let is_six = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[A2][0] as i64) < 7) as u64;
        if is_six {
            c.cycles += 4;
            c.g[V1][0] = c.lw(ea(c.g[A0][0], 0x3c9c));
            c.g[V0][0] = ((c.g[V1][0] as i64) < 220) as u64;
            let full = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
            if full {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            c.cycles += 11;
            c.g[V1][0] = sx(ea(c.g[V1][0], 1));
            c.g[V0][0] = sx(ea(c.g[V0][0], 0x3ca0));
            c.sw(ea(c.g[A0][0], 0x3c9c), c.g[V1][0]);
            c.g[V0][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
            c.g[V1][0] = 1;
            c.sw(ea(c.g[V0][0], 4), c.g[T0][0]);
            c.sw(ea(c.g[V0][0], 8), c.g[V1][0]);
            c.sw(ea(c.g[V0][0], 0), c.g[A1][0]);
            c.g[V1][0] = c.lhu(ea(c.g[A1][0], 0x1a8));
            c.sh(ea(c.g[V0][0], 0xc), c.g[V1][0]);
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        let at_least_seven = c.g[V0][0] == 0;
        c.g[V0][0] = 4;
        if at_least_seven {
            c.cycles += 3;
            c.g[V0][0] = 8;
            if c.g[A2][0] == c.g[V0][0] {
                c.g[V1][0] = c.lw(ea(c.g[A0][0], 0x4a60));
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 50) as u64;
                let full = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
                if full {
                    c.cycles += 2;
                    abi::ret(c);
                    return;
                }
                c.cycles += 11;
                c.g[V1][0] = sx(ea(c.g[V1][0], 1));
                c.g[V0][0] = sx(ea(c.g[V0][0], 0x4a64));
                c.sw(ea(c.g[A0][0], 0x4a60), c.g[V1][0]);
                c.g[V0][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
                c.g[V1][0] = 2;
                c.sw(ea(c.g[V0][0], 4), c.g[T0][0]);
                c.sw(ea(c.g[V0][0], 8), c.g[V1][0]);
                c.sw(ea(c.g[V0][0], 0), c.g[A1][0]);
                c.g[V1][0] = c.lhu(ea(c.g[A1][0], 0x1aa));
                c.sh(ea(c.g[V0][0], 0xc), c.g[V1][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        if c.g[A2][0] == c.g[V0][0] {
            c.g[V1][0] = c.lw(ea(c.g[A0][0], 0x458));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 900) as u64;
            let full = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
            if full {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            c.cycles += 10;
            c.g[V1][0] = sx(ea(c.g[V1][0], 1));
            c.g[V0][0] = sx(ea(c.g[V0][0], 0x45c));
            c.sw(ea(c.g[A0][0], 0x458), c.g[V1][0]);
            c.g[V0][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
            c.sw(ea(c.g[V0][0], 4), c.g[T0][0]);
            c.sw(ea(c.g[V0][0], 0), c.g[A1][0]);
            c.sw(ea(c.g[V0][0], 8), 0);
            c.g[V1][0] = c.lhu(ea(c.g[A1][0], 0x1a6));
            c.sh(ea(c.g[V0][0], 0xc), c.g[V1][0]);
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    // Mixed patch: capacity 75, kind 2 when any type is 8.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A3][0] as i64) < 75) as u64;
    let full = c.g[V0][0] == 0;
    c.g[V0][0] = 0x14;
    if full {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    let kind;
    c.cycles += 11;
    c.g[V1][0] = sx(ea(c.g[A3][0], 1));
    // `mult a3, v0` (the generator routes the LO product through v0).
    c.mult(0, V0, A3, V0);
    c.sw(ea(c.g[A0][0], 0x4d84), c.g[V1][0]);
    c.g[A3][0] = 8;
    c.g[V0][0] = sx(ea(c.g[V0][0], 0x4d88));
    c.g[V1][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
    c.sw(ea(c.g[V1][0], 4), c.g[T0][0]);
    c.sw(ea(c.g[V1][0], 0), c.g[A1][0]);
    c.g[V0][0] = c.lbu(ea(c.g[A2][0], 0));
    let is_eight0 = c.g[V0][0] == c.g[A3][0];
    c.g[A0][0] = sx(ea(c.g[V1][0], 8));
    if is_eight0 {
        c.cycles += 1;
        kind = 2;
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lbu(ea(c.g[A2][0], 1));
        let is_eight1 = c.g[V0][0] == c.g[A3][0];
        c.g[V0][0] = 2;
        if is_eight1 {
            kind = 2;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lbu(ea(c.g[A2][0], 2));
            let is_eight2 = c.g[V0][0] == c.g[A3][0];
            c.g[V0][0] = 2;
            if is_eight2 {
                kind = 2;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(ea(c.g[A2][0], 3));
                let not_eight3 = c.g[V0][0] != c.g[A3][0];
                c.g[V0][0] = 0;
                if not_eight3 {
                    kind = 0;
                } else {
                    c.cycles += 1;
                    kind = 2;
                }
            }
        }
    }
    c.g[V0][0] = kind;
    c.cycles += 5;
    c.sw(ea(c.g[A0][0], 0), c.g[V0][0]);
    c.g[T0][0] = sx(ea(c.g[V1][0], 0x10));
    c.g[A3][0] = 0;
    c.g[V0][0] = c.lhu(ea(c.g[A1][0], 0x1aa));
    c.sh(ea(c.g[V1][0], 0xc), c.g[V0][0]);
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx(ea(c.g[A2][0], c.g[A3][0] as u32));
        c.g[A0][0] = sx(ea(c.g[T0][0], c.g[A3][0] as u32));
        c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
        c.g[A3][0] = sx(ea(c.g[A3][0], 1));
        c.g[V0][0] = ((c.g[A3][0] as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.sb(ea(c.g[A0][0], 0), c.g[V1][0]);
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_b310);
        }
    }
}

/// `void PS2BezierMan_FreeCaches(PS2BezierMan* man)` (0x38b0f8): free the
/// renderer's three cache blocks.
pub fn ps2_bezier_man_free_caches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let man = c.g[A0][0];
        c.g[S0][0] = man;
        call_ptr(c, 0x0038_d660, 0x0038_b10c)?;
        c.g[A0][0] = Addr(at_2(c, S0, 0x440)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b120)?;
        }
        c.g[A0][0] = Addr(at_2(c, S0, 0x444)).lw(c);
        'second: {
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.g[A0][0] = Addr(at_2(c, S0, 0x448)).lw(c);
                break 'second;
            }
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b134)?;
            c.g[A0][0] = Addr(at_2(c, S0, 0x448)).lw(c);
        }
        'third: {
            c.cycles += 2;
            let none = c.g[A0][0] == 0;
            frame.restore(c, S0, 0x10);
            if none {
                break 'third;
            }
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b148)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------- PS2BezierMan_RenderPatchFX

/// `void PS2BezierMan_RenderPatchFX(PS2BezierMan* bm, int pass)` (0x38d968):
/// the glitter pass over the terrain patches. Unless FX are off (0x4a44e0),
/// pushes a copy of the graphics manager's render state (stack top at
/// +0xe84, 0x14-byte entries), selects the current FX texture
/// (`texIds[texSel]`, +0xf50 / +0x64) and sets the FX blend/test flags,
/// refreshes the view matrices if stale (+0x6b90), then for every type-8
/// patch (count +0x4a60, 0x10-byte entries at +0x4a64) and every mixed patch
/// with a positive kind (count +0x4d84, 0x14-byte entries at +0x4d88, kind
/// at +8) uploads the patch (0x1b0 bytes) to the scratchpad and draws its FX
/// (`PS2BezierMan_DrawPatchFX(bm, uploaded, entry, pass)`); finally pops the
/// render state.
pub fn ps2_bezier_man_render_patch_fx(c: &mut Cpu) {
    /// Non-zero: patch FX are switched off (gp+0x13f0).
    const FX_OFF: u32 = 0x13f0;
    /// The graphics manager (gp+0x2a90).
    const GFX_MAN: u32 = 0x2a90;
    /// Graphics manager fields.
    const STATE_TOP: u32 = 0xe84;
    const TEX_SEL: u32 = 0x64;
    const TEX_IDS: u32 = 0xf50;
    const VIEW_VALID: u32 = 0x6b90;
    /// Render-state entry size and fields.
    const STATE_SIZE: u32 = 0x14;
    /// Bezier manager fields.
    const PATCH8_COUNT: u32 = 0x4a60;
    const PATCH8: u32 = 0x4a64;
    const MIX_COUNT: u32 = 0x4d84;
    const MIX: u32 = 0x4d88;
    const MIX_KIND: u32 = 0x8;
    /// Bytes of a patch uploaded to the scratchpad.
    const PATCH_BYTES: u64 = 0x1b0;
    const UPDATE_VIEW_MATRICES: Callee = Callee {
        addr: 0x0037_d968,
        f: ee::s12::PS2GraphicsMan_UpdateViewMatrices,
    };
    const SPR_STREAM_UPLOAD: Callee = Callee {
        addr: 0x0038_f708,
        f: ee::s12::SprStream_Upload,
    };
    const DRAW_PATCH_FX: Callee = Callee {
        addr: 0x0038_d690,
        f: ee::s12::PS2BezierMan_DrawPatchFX,
    };

    /// `base = gfx->top; base->[off] = (base->[off] & mask) | set` with the
    /// pointer in `base`, the word in v0.
    fn update_top(c: &mut Cpu, gfx: u32, base: usize, off: u32, mask: u64, set: u64) {
        c.g[base][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
        let field = (c.g[base][0] as u32).wrapping_add(off);
        c.g[V0][0] = Addr(field).lw(c);
        c.g[V0][0] = (c.g[V0][0] & mask) | set;
        c.sw(field, c.g[V0][0]);
    }

    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = Addr(gp.wrapping_add(FX_OFF)).lw(c);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 68;
            let gfx = Addr(gp.wrapping_add(GFX_MAN)).lw(c);
            c.g[S2][0] = gfx;
            let gfx = gfx as u32;
            c.g[T3][0] = !0x7c;
            c.g[T5][0] = !0x3e0;
            c.g[T0][0] = 0xffff_ffff_fe7f_0000;
            // Push: copy the top entry (8 + 8 + 4 bytes) one entry up.
            let top = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.g[V0][0] = top;
            let top = top as u32;
            c.g[T0][0] |= 0xffff;
            c.g[T6][0] = 0x80_0000;
            c.g[T1][0] = 0xffff_ffff_ffcf_0000;
            c.ldl(V1, top.wrapping_add(0x7));
            c.ldr(V1, top);
            c.ldl(A0, top.wrapping_add(0xf));
            c.ldr(A0, top.wrapping_add(0x8));
            c.g[A1][0] = Addr(top.wrapping_add(0x10)).lw(c);
            c.sdl(V1, top.wrapping_add(0x1b));
            c.sdr(V1, top.wrapping_add(0x14));
            c.sdl(A0, top.wrapping_add(0x23));
            c.sdr(A0, top.wrapping_add(0x1c));
            c.sw(top.wrapping_add(0x24), c.g[A1][0]);
            c.g[T1][0] |= 0xffff;
            c.g[T7][0] = 0x30_0000;
            c.g[T2][0] = 0xffff_ffff_fff0_0000;
            c.g[A3][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.g[T2][0] |= 0xfff;
            c.g[V0][0] = Addr(gfx.wrapping_add(TEX_SEL)).lw(c);
            c.g[A2][0] = 0x5_0000;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(STATE_SIZE));
            c.g[A2][0] |= 0xc000;
            c.sw(gfx.wrapping_add(STATE_TOP), c.g[V1][0]);
            c.g[V0][0] = sx(gfx.wrapping_add((c.g[V0][0] as u32) << 2));
            c.g[A1][0] = 0xffff_ffff_ffbf_0000;
            // The FX texture (halfword at the new top + 0x10).
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(TEX_IDS)).lw(c);
            c.g[A1][0] |= 0xffff;
            c.g[T4][0] = 0x40_0000;
            c.sh((c.g[A3][0] as u32).wrapping_add(0x24), c.g[A0][0]);
            // The FX blend / test flags.
            let (t3, t5, t0, t6, t1, t7, t2, a2, a1, t4) = (
                c.g[T3][0], c.g[T5][0], c.g[T0][0], c.g[T6][0], c.g[T1][0], c.g[T7][0], c.g[T2][0],
                c.g[A2][0], c.g[A1][0], c.g[T4][0],
            );
            update_top(c, gfx, V1, 0x4, t3, 0x54);
            update_top(c, gfx, A0, 0x8, t5, 0x80);
            update_top(c, gfx, V1, 0x4, t0, t6);
            update_top(c, gfx, A0, 0x4, t1, t7);
            update_top(c, gfx, V1, 0x4, t2, a2);
            update_top(c, gfx, A0, 0x4, a1, t4);
            c.g[V1][0] = Addr(gfx.wrapping_add(VIEW_VALID)).lw(c);
            if c.g[V1][0] == 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S2][0];
                UPDATE_VIEW_MATRICES.call(c, 0x0038_daac)?;
            }
            c.g[A0][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);

            c.cycles += 25;
            c.g[V1][0] = !0x3c0;
            c.g[A1][0] = 0xffff_ffff_c000_0000;
            c.g[A3][0] = !0x3;
            let top = c.g[A0][0] as u32;
            c.g[V0][0] = Addr(top).lw(c);
            c.g[A2][0] = 0xffff_ffff_e000_03ff;
            c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | 0x100;
            c.sw(top, c.g[V0][0]);
            let (a1, a3, a2) = (c.g[A1][0], c.g[A3][0], c.g[A2][0]);
            update_top(c, gfx, V1, 0xc, a1, 0);
            update_top(c, gfx, A1, 0x4, a3, 0x2);
            update_top(c, gfx, A0, 0x8, a2, 0);

            // Type-8 patches.
            let bm = c.g[S3][0] as u32;
            c.g[V1][0] = Addr(bm.wrapping_add(PATCH8_COUNT)).lw(c);
            c.g[S1][0] = 0;
            if (c.g[V1][0] as i64) > 0 {
                c.cycles += 1;
                c.g[S0][0] = sx(bm.wrapping_add(PATCH8));
                loop {
                    c.cycles += 14;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[A1][0] = PATCH_BYTES;
                    c.g[A2][0] = 0x2;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    SPR_STREAM_UPLOAD.call(c, 0x0038_db2c)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A3][0] = c.g[S4][0];
                    DRAW_PATCH_FX.call(c, 0x0038_db40)?;
                    c.g[V0][0] = Addr(bm.wrapping_add(PATCH8_COUNT)).lw(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0038_db18);
                    }
                }
            }

            // Mixed patches with a positive kind.
            c.cycles += 3;
            c.g[V0][0] = Addr(bm.wrapping_add(MIX_COUNT)).lw(c);
            c.g[S1][0] = 0;
            if (c.g[V0][0] as i64) > 0 {
                c.cycles += 1;
                c.g[S0][0] = sx(bm.wrapping_add(MIX));
                loop {
                    c.cycles += 3;
                    let entry = c.g[S0][0] as u32;
                    c.g[V0][0] = Addr(entry.wrapping_add(MIX_KIND)).lw(c);
                    c.g[A1][0] = PATCH_BYTES;
                    if (c.g[V0][0] as i64) > 0 {
                        c.cycles += 8;
                        c.g[A0][0] = Addr(entry).lw(c);
                        c.g[A2][0] = 0x2;
                        SPR_STREAM_UPLOAD.call(c, 0x0038_db78)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A2][0] = c.g[S0][0];
                        c.g[A3][0] = c.g[S4][0];
                        DRAW_PATCH_FX.call(c, 0x0038_db8c)?;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(bm.wrapping_add(MIX_COUNT)).lw(c);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0038_db60);
                    }
                }
            }

            // Pop the render state.
            c.cycles += 3;
            c.g[V0][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(STATE_SIZE));
            c.sw(gfx.wrapping_add(STATE_TOP), c.g[V0][0]);
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------ PS2BezierMan_BuildPatchPacket

/// `void PS2BezierMan_BuildPatchPacket(PS2BezierMan* bm, int type, int idx)`
/// (0x38c788): write the VIF packet that uploads patch cache slot `idx` of
/// lod `type` (0..2: an n x n grid with n = 2 * type + 4) to VU1: a header
/// (STCYCL / unpack setup constants), then three UNPACK transfers of the
/// slot's control points (+0x58/+0x5c/+0x60, 0xc0/0x1b0/0x300 bytes per
/// slot), normals (+0x64.., 0x80/0x120/0x200) and colours (+0x70..), and a
/// closing DMA ret. The packet lives at `(bm->packets[type] (+0x44c) +
/// idx * 0x100) | 0x30000000` (uncached-accelerated). Other types use null
/// sources. Leaf.
pub fn ps2_bezier_man_build_patch_packet(c: &mut Cpu) {
    /// Source tables per lod.
    const POINTS: u32 = 0x58;
    const NORMALS: u32 = 0x64;
    const COLOURS: u32 = 0x70;
    const PACKETS: u32 = 0x44c;

    c.cycles += 5;
    let bm = c.g[A0][0];
    c.g[A3][0] = bm;
    let bm = bm as u32;
    let ty = c.g[A1][0];
    c.g[V1][0] = sx((ty as u32) << 1);
    c.g[V0][0] = 1;
    c.g[A0][0] = sx(r32(c, V1).wrapping_add(4)); // n
    let field = |t: u32| bm.wrapping_add(t);

    // Control points -> t2.
    let lod = if ty == 1 {
        Some((1u32, 0x1b0u64))
    } else {
        c.cycles += 3;
        c.g[V0][0] = ((ty as i64) < 2) as u64;
        c.g[V0][0] = 2;
        c.cycles += 2;
        c.g[T2][0] = 0;
        if (ty as i64) < 2 {
            if ty == 0 {
                Some((0, 0xc0))
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                None
            }
        } else if ty == 2 {
            Some((2, 0x300))
        } else {
            c.cycles += 2;
            c.g[V0][0] = 1;
            None
        }
    };
    if let Some((t, stride)) = lod {
        c.cycles += if t == 2 { 2 } else { 3 };
        c.g[V0][0] = stride;
        c.g[V1][0] = Addr(field(POINTS + 4 * t)).lw(c);
        c.cycles += 3;
        c.mult(0, V0, A2, V0);
        c.g[T2][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[V0][0] = 1;
    }

    // Normals -> t3; colours -> t8 (same shape, different tables).
    for (dst, table) in [(T3, NORMALS), (T8, COLOURS)] {
        c.cycles += 2;
        c.g[V0][0] = ((ty as i64) < 2) as u64;
        let found = if ty == 1 {
            c.cycles += 5;
            c.g[V0][0] = 0x120;
            c.g[V1][0] = Addr(field(table + 4)).lw(c);
            c.mult(0, V0, A2, V0);
            c.g[dst][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            true
        } else {
            c.cycles += 2;
            let low = c.g[V0][0] != 0;
            c.g[V0][0] = 2;
            c.cycles += 2;
            c.g[dst][0] = 0;
            let (hit, k, shift) = if low {
                (ty == 0, 0, 7)
            } else {
                (ty == 2, 2, 9)
            };
            if hit {
                c.cycles += if k == 0 { 4 } else { 3 };
                c.g[V1][0] = Addr(field(table + 4 * k)).lw(c);
                c.g[V0][0] = sx(r32(c, A2) << shift);
                c.g[dst][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            } else {
                c.cycles += 2;
            }
            hit
        };
        if found {
            c.cycles += 1;
        }
        if dst == T3 {
            c.g[V0][0] = 1;
        } else {
            c.mult(0, A0, A0, A0); // n * n
        }
    }

    // The packet.
    c.cycles += 82;
    c.g[V0][0] = sx(bm.wrapping_add(r32(c, A1) << 2));
    c.g[T1][0] = 0x3000_0000;
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(PACKETS)).lw(c);
    c.g[A2][0] = sx(r32(c, A2) << 8);
    c.g[A1][0] = 0x7800_0000;
    c.g[A3][0] = 0x7400_0000;
    let nn = r32(c, A0);
    c.g[V0][0] = sx(nn << 1);
    c.g[T0][0] = sx(((nn as i32) >> 1) as u32);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(nn));
    c.g[T2][0] <<= 32;
    c.g[V0][0] = sx(((r32(c, V0) as i32) >> 2) as u32);
    c.g[A0][0] = sx(nn << 16);
    c.g[V0][0] |= c.g[T1][0];
    c.g[T0][0] |= c.g[T1][0];
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A2)));
    c.g[T2][0] |= c.g[V0][0];
    c.g[A1][0] |= 0x8000;
    c.g[A3][0] |= 0x8040;
    c.g[V1][0] |= c.g[T1][0];
    c.g[A1][0] |= c.g[A0][0];
    c.g[A3][0] |= c.g[A0][0];
    c.g[V0][0] = 0x1000_0003;
    c.g[T3][0] <<= 32;
    let p = r32(c, V1);
    c.sd(p, c.g[V0][0]);
    c.g[T3][0] |= c.g[T0][0];
    c.g[T6][0] = 0x1000_0001;
    c.g[T7][0] = 0x5050_5050_2000_0000;
    c.g[A1][0] <<= 32;
    c.g[A3][0] <<= 32;
    c.g[T1][0] = 0x3000_0000_0500_0000;
    c.g[T4][0] = 0x3f80_0000_3f80_0000;
    c.g[T5][0] = 0x4040_4040_2000_0000;
    c.g[A2][0] = 0x7400_8080;
    c.g[V0][0] = c.g[T8][0] << 32;
    c.g[A0][0] |= c.g[A2][0];
    c.g[T0][0] |= c.g[V0][0];
    c.g[A0][0] <<= 32;
    c.g[V0][0] = 0x6000_0000;
    // Header and the points / normals transfers.
    let words: [(u32, u64); 15] = [
        (0x08, 0),
        (0x10, c.g[T1][0]),
        (0x20, c.g[T4][0]),
        (0x28, c.g[T5][0]),
        (0x18, 0),
        (0x38, c.g[A1][0]),
        (0x40, c.g[T2][0]),
        (0x68, c.g[A3][0]),
        (0x70, c.g[T3][0]),
        (0x30, 0),
        (0x48, 0),
        (0x50, c.g[T6][0]),
        (0x58, 0),
        (0x60, c.g[T7][0]),
        (0x78, 0),
    ];
    for &(off, v) in &words {
        c.sd(p.wrapping_add(off), v);
    }
    // The colours transfer.
    let p = p.wrapping_add(0x80);
    c.g[V1][0] = sx(p);
    c.sd(p, c.g[T6][0]);
    c.sd(p.wrapping_add(0x8), 0);
    c.sd(p.wrapping_add(0x10), c.g[T7][0]);
    c.sd(p.wrapping_add(0x18), c.g[A0][0]);
    c.sd(p.wrapping_add(0x20), c.g[T0][0]);
    c.sd(p.wrapping_add(0x28), 0);
    // DMA ret.
    let p = p.wrapping_add(0x30);
    c.g[V1][0] = sx(p);
    c.sd(p, c.g[V0][0]);
    c.sd(p.wrapping_add(0x8), 0);
    ret(c);
}

/// `void cPS2BezierMan::Init(cPS2BezierMan* self)` (0x38af30).
///
/// Initialises the patch cache (`BezierPatchCache_Init(self + 0x10, 900, 220,
/// 140, 0, 146, 93)`), allocates the three VIF caches (0x38400, 0xdc00,
/// 0x8c00 bytes), sets the render flags and LOD distances, calls the base
/// init (`0x38d638`), and fills the 6x6 (step 0.2) and 8x8 (step 1/7)
/// grid UV tables at 0x5049c0 (`D_005049C0`).
pub fn cps2_bezier_man_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 58;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[T0][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[T1][0] = 0x92;
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[T2][0] = 0x5d;
        c.g[A1][0] = 0x384;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 0xdc;
        c.g[A3][0] = 0x8c;
        call_ptr(c, 0x00372b78, 0x0038af60)?;
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = 0x30000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2eb0));
        c.g[A0][0] = c.g[A0][0] | 0x8400;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x0038af7c)?;
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = 0xdc00;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2ec0));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x440), c.g[V0][0]);
        call_ptr(c, 0x00317e30, 0x0038af98)?;
        c.g[A1][0] = 0x490000;
        c.g[A2][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2ed0));
        c.g[A0][0] = 0x8c00;
        c.g[A3][0] = 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x444), c.g[V0][0]);
        call_ptr(c, 0x00317e30, 0x0038afb4)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x440));
        c.g[V1][0] = 0x1;
        c.g[AT][0] = 0x41000000;
        c.mtc1(1, 2);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x444));
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x26cc));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x26c8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x354), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x44c), c.g[A2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x454), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x350), c.g[V1][0]);
        c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x358));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x448), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x344), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x348), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x34c), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x450), c.g[A1][0]);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x33c));
        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x340));
        call_ptr(c, 0x0038d638, 0x0038b008)?;
        c.g[V0][0] = 0x500000;
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x26c4));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4a00));
        c.g[V1][0] = 0u64;
        'l_38b018: loop {
            c.cycles += 8;
            c.mtc1(3, 0);
            c.cvt_s_w(0, 0);
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = 0u64;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.mul_s(1, 0, 2);
            'l_38b038: loop {
                c.cycles += 13;
                c.mtc1(4, 0);
                c.cvt_s_w(0, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < 6i64) as u64;
                c.mul_s(0, 0, 2);
                c.swc1(0, c.g[SP][0] as u32);
                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(7, c.g[SP][0] as u32);
                c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.sdr(7, c.g[V1][0] as u32);
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0038b038);
                    }
                    continue 'l_38b038;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[A1][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 6i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x500000;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0038b018);
                        }
                        continue 'l_38b018;
                    } else {
                        c.cycles += 3;
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x26c0));
                        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4b80));
                        c.g[V1][0] = 0u64;
                        'l_38b088: loop {
                            c.cycles += 8;
                            c.mtc1(3, 0);
                            c.cvt_s_w(0, 0);
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.g[A0][0] = 0u64;
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.mul_s(1, 0, 2);
                            'l_38b0a8: loop {
                                c.cycles += 13;
                                c.mtc1(4, 0);
                                c.cvt_s_w(0, 0);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[A0][0] as i64) < 8i64) as u64;
                                c.mul_s(0, 0, 2);
                                c.swc1(0, c.g[SP][0] as u32);
                                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                                c.ldr(7, c.g[SP][0] as u32);
                                c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                c.sdr(7, c.g[V1][0] as u32);
                                let t = c.g[V0][0] != 0u64;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0038b0a8);
                                    }
                                    continue 'l_38b0a8;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.g[A1][0];
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 8i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0038b088);
                                        }
                                        continue 'l_38b088;
                                    } else {
                                        c.cycles += 3;
                                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                                        let t = c.g[RA][0] as u32;
                                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.pc = t;
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

/// `PS2BezierMan* PS2BezierMan_Create(void)` (0x38aef8): allocates 0x9c90
/// bytes (`EA_MemAlloc` named, name at 0x492ea0) and runs the constructor.
pub fn ps2_bezier_man_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_add(0x2ea0));
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x9c90;
        call_ptr(c, 0x0031_7d70, 0x0038_af18)?;
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x0038_ae60,
            f: ee::s12::PS2BezierMan_ctor,
        }
        .call(c, 0x0038_af20)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Offset of the embedded `BezierPatchCache` in `cPS2BezierMan`.
pub(crate) const PS2_BEZIER_MAN_V04_CACHE: u32 = 0x10;

/// `void PS2BezierMan_v04(cPS2BezierMan* man)` (0x38b158, `cPS2BezierMan_Reset`):
/// resets the patch cache embedded at `man + 0x10`.
pub fn ps2_bezier_man_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(PS2_BEZIER_MAN_V04_CACHE));
        call_ptr(c, 0x0037_39d0, 0x0038_b168)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2BezierMan_EmitPatchType1(PS2BezierMan* m, int kind, int sub,
/// Patch* patch, uint32* handle)` (0x38ca70): copy the patch record into the
/// scratchpad (`SprStream_Upload`), decide from its flags and the render
/// toggles (gp+0x13e8/0x13ec) whether it is drawn at once (patch
/// header packet, ring entry, reflection and projector passes) or deferred to
/// the manager's defer list (+0x5364).
pub fn ps2_bezier_man_emit_patch_type1(c: &mut Cpu) {
    guest(c, |c| {
        'b_38cb18: {
            c.cycles += 41;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.g[V1][0] = 0xc;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S1][0] = c.g[A2][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S6][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.mult(0, 3, 17, 3);
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S0][0] = sx((c.g[S6][0] as u32) << 2);
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = 0x1b0;
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S7][0] = c.g[T0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[FP][0] = c.g[A3][0];
            c.g[S5][0] = 0u64;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw(c.g[FP][0] as u32);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[SP][0] as u32, c.g[A2][0]);
            c.g[RA][0] = sx(0x0038cae0);
            c.g[A2][0] = 0x2;
            Callee {
                addr: 0x0038f708,
                f: ee::s12::SprStream_Upload,
            }
            .call(c, 0x0038cae0)?;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[S4][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
            c.g[S1][0] = sx((c.g[S1][0] as u32) << 8);
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x44c));
            c.g[V1][0] = sx((c.g[S6][0] as u32) << 1);
            c.g[A0][0] = 0x600000;
            c.g[S3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
            let t = c.g[V0][0] == 0u64;
            c.g[T3][0] = 0u64;
            if t {
                break 'b_38cb18;
            } else {
                c.cycles += 1;
                c.g[T3][0] = (0u64 < c.g[V1][0]) as u64;
                break 'b_38cb18;
            }
        }
        'b_38cb2c: {
            c.cycles += 2;
            let t = c.g[T3][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x13e8));
            if t {
                break 'b_38cb2c;
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[S5][0] = (0u64 < c.g[T3][0]) as u64;
                    break 'b_38cb2c;
                } else {
                    c.cycles += 1;
                    c.g[T3][0] = 0u64;
                    break 'b_38cb2c;
                }
            }
        }
        'b_38cb50: {
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            let t = c.g[S6][0] != c.g[V0][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
            if t {
                break 'b_38cb50;
            } else {
                c.cycles += 6;
                c.sq(11, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0038cb48);
                c.g[A1][0] = c.g[S4][0];
                Callee {
                    addr: 0x0038ca08,
                    f: ee::s12::PS2BezierMan_TestPatchBounds,
                }
                .call(c, 0x0038cb48)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.lq(11, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_38cb50;
            }
        }
        'b_38cb6c: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
            let t = c.g[S1][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x13ec));
            if t {
                break 'b_38cb6c;
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[S5][0] = c.g[S5][0] | c.g[S1][0];
                    break 'b_38cb6c;
                } else {
                    c.cycles += 1;
                    c.g[S1][0] = 0u64;
                    break 'b_38cb6c;
                }
            }
        }
        'b_38cc04: {
            'b_38cc00: {
                'b_38cbf8: {
                    'b_38cbf4: {
                        'b_38cbe0: {
                            'b_38cbc8: {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                                c.g[A1][0] = 0x180000;
                                c.g[A0][0] = 0x100000;
                                c.g[V1][0] = c.g[V0][0] & c.g[A1][0];
                                if c.g[V1][0] == c.g[A0][0] {
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                                    break 'b_38cbc8;
                                } else {
                                    'b_38cbb0: {
                                        'b_38cba0: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V0][0] = 0x80000;
                                            if t {
                                                break 'b_38cba0;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == c.g[V0][0];
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                                                if t {
                                                    break 'b_38cbb0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V1][0] = 0xfffffffffffffff3;
                                                    if t {
                                                        break 'b_38cbf8;
                                                    } else {
                                                        break 'b_38cba0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[A1][0] {
                                            c.g[V1][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                                            break 'b_38cbe0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                                            if t {
                                                break 'b_38cbf4;
                                            } else {
                                                break 'b_38cbb0;
                                            }
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[V1][0] = 0xfffffffffffffff3;
                                    c.g[S5][0] = 0x1;
                                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                    let t = true;
                                    c.g[V0][0] = c.g[V0][0] | 0x8;
                                    if t {
                                        break 'b_38cc00;
                                    } else {
                                        break 'b_38cbc8;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[V1][0] = 0xfffffffffffffff3;
                            c.g[S5][0] = 0x1;
                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            let t = true;
                            c.g[V0][0] = c.g[V0][0] | 0x4;
                            if t {
                                break 'b_38cc00;
                            } else {
                                break 'b_38cbe0;
                            }
                        }
                        c.cycles += 5;
                        c.g[S5][0] = 0x1;
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = c.g[V0][0] | 0xc;
                        let t = true;
                        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                        if t {
                            break 'b_38cc04;
                        } else {
                            break 'b_38cbf4;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = 0xfffffffffffffff3;
                    break 'b_38cbf8;
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                break 'b_38cc00;
            }
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
            break 'b_38cc04;
        }
        'b_38cdf0: {
            'b_38cdec: {
                'b_38cd88: {
                    c.cycles += 2;
                    let t = c.g[S5][0] == 0u64;
                    c.g[A2][0] = c.g[S0][0] << 32;
                    if t {
                        break 'b_38cd88;
                    } else {
                        'b_38cd30: {
                            c.cycles += 41;
                            c.g[A0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x1a0));
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                            c.g[V1][0] = 0x50000000;
                            c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
                            c.g[T4][0] = 0x30000000;
                            c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                            c.g[T0][0] = 0x10000000;
                            c.g[T0][0] = c.g[T0][0] | 0x1;
                            c.g[V0][0] = 0x450000;
                            c.g[A1][0] = sx((c.g[S6][0] as u32) << 2);
                            c.g[A0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x1a2));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4630));
                            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[T1][0] = 0x14000000;
                            c.g[A3][0] = 0xfff0000;
                            c.sh((c.g[V1][0] as u32).wrapping_add(0x12), c.g[A0][0]);
                            c.g[A3][0] = c.g[A3][0] | 0xffff;
                            c.g[T2][0] = c.lw(c.g[S7][0] as u32);
                            c.g[A0][0] = c.g[T2][0] | c.g[T4][0];
                            c.sd(c.g[A0][0] as u32, c.g[A2][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x18), 0u64);
                            c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
                            c.g[S0][0] = c.lw(c.g[A1][0] as u32);
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x20), 0u64);
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                            c.g[V0][0] = c.g[V0][0] << 32;
                            c.sd((c.g[A0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
                            c.g[T1][0] = c.g[A0][0] & c.g[A3][0];
                            c.sw(c.g[S7][0] as u32, c.g[T1][0]);
                            c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18f0));
                            c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
                            if t {
                                break 'b_38cd30;
                            } else {
                                c.cycles += 32;
                                c.g[A1][0] = 0x60000;
                                c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = c.g[A1][0] | 0x8000;
                                c.g[V1][0] = 0xffffffffffffffe0;
                                c.g[A1][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
                                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                                c.g[A2][0] = c.g[A2][0] | c.g[T4][0];
                                c.g[V0][0] = c.g[V0][0] & 0x1f;
                                c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                                c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                                c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                                c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                                c.ldr(3, c.g[T0][0] as u32);
                                c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
                                c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
                                c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                                c.sdr(3, c.g[V0][0] as u32);
                                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
                                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T2][0]);
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                                c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                                c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
                                break 'b_38cd30;
                            }
                        }
                        'b_38cd60: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw(c.g[S7][0] as u32);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                            let t = c.g[T3][0] == 0u64;
                            c.sw(c.g[S7][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_38cd60;
                            } else {
                                c.cycles += 8;
                                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                                c.g[A2][0] = c.g[S4][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A3][0] = c.g[S6][0];
                                c.g[T0][0] = c.g[S0][0];
                                c.g[T1][0] = c.g[FP][0];
                                c.g[RA][0] = sx(0x0038cd60);
                                c.g[T2][0] = c.g[S7][0];
                                Callee {
                                    addr: 0x0038d168,
                                    f: ee::s12::PS2BezierMan_EmitPatchMatrixPass,
                                }
                                .call(c, 0x0038cd60)?;
                                break 'b_38cd60;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S1][0] == 0u64;
                        c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        if t {
                            break 'b_38cdec;
                        } else {
                            c.cycles += 8;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A1][0] = c.g[S6][0];
                            c.g[A2][0] = c.g[S0][0];
                            c.g[A3][0] = c.g[FP][0];
                            c.g[RA][0] = sx(0x0038cd80);
                            c.g[T1][0] = c.g[S7][0];
                            Callee {
                                addr: 0x0038d448,
                                f: ee::s12::PS2BezierMan_EmitPatchProjectorPasses,
                            }
                            .call(c, 0x0038cd80)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                            if t {
                                break 'b_38cdf0;
                            } else {
                                break 'b_38cd88;
                            }
                        }
                    }
                }
                c.cycles += 25;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5364));
                c.g[A0][0] = c.lhu((c.g[S4][0] as u32).wrapping_add(0x1a0));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sh((c.g[V0][0] as u32).wrapping_add(0x5368), c.g[A0][0]);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5364));
                c.g[A0][0] = c.lhu((c.g[S4][0] as u32).wrapping_add(0x1a2));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sh((c.g[V1][0] as u32).wrapping_add(0x536a), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5364));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x536c), c.g[S6][0]);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5364));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x5370), c.g[S0][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5364));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5374), c.g[FP][0]);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5364));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x5364), c.g[V1][0]);
                break 'b_38cdec;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            break 'b_38cdf0;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

/// `void PS2BezierMan_EmitPatchType2(BezMan* self, int sub, Patch* patch, int* ext, uint* handle)`
/// (0x0038ce20): emits one 4x4-bezier ("type 2") patch into the GIF packet
/// list at `*handle`. Uploads the patch record to the scratchpad, tests its
/// bounds, sets the draw-state texture ids and the 0x180000 reflection/shadow
/// bits, writes the DMA packet (optional `ext` tag, then the program call),
/// queues a ring entry, then runs the matrix pass (reflection flag) and the
/// projector passes (shadow flag).
pub fn ps2_bezier_man_emit_patch_type2(c: &mut Cpu) {
    guest(c, |c| {
        'b_38ceb4: {
            c.cycles += 35;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.g[V0][0] = 0xc;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S0][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.mult(0, 2, 16, 2);
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A1][0] = 0x1b0;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = c.g[T0][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S5][0] = c.g[A0][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S6][0] = c.g[A2][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[FP][0] = c.g[A3][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A2][0] = 0x2;
            c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.lw(c.g[S6][0] as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[RA][0] = sx(0x0038ce84);
            c.sw(c.g[SP][0] as u32, c.g[V1][0]);
            Callee {
                addr: 0x0038f708,
                f: ee::s12::SprStream_Upload,
            }
            .call(c, 0x0038ce84)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x454));
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = 0x600000;
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 8);
            c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
            c.g[S2][0] = c.g[A0][0] & c.g[V0][0];
            c.g[S2][0] = (0u64 < c.g[S2][0]) as u64;
            let t = c.g[S2][0] == 0u64;
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            if t {
                break 'b_38ceb4;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x13e8));
                if c.g[V0][0] != 0 {
                    c.g[S2][0] = 0u64;
                }
                break 'b_38ceb4;
            }
        }
        'b_38cf74: {
            'b_38cf70: {
                'b_38cf68: {
                    'b_38cf64: {
                        'b_38cf54: {
                            'b_38cf40: {
                                c.cycles += 19;
                                c.g[A0][0] = c.g[S5][0];
                                c.g[RA][0] = sx(0x0038cec0);
                                c.g[A1][0] = c.g[S3][0];
                                Callee {
                                    addr: 0x0038ca08,
                                    f: ee::s12::PS2BezierMan_TestPatchBounds,
                                }
                                .call(c, 0x0038cec0)?;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[A2][0] = 0x180000;
                                c.g[A1][0] = 0x100000;
                                c.g[A0][0] = c.lh((c.g[S3][0] as u32).wrapping_add(0x1a0));
                                c.g[S7][0] = (0u64 < c.g[V0][0]) as u64;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x13ec));
                                c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                                if c.g[V0][0] != 0 {
                                    c.g[S7][0] = 0u64;
                                }
                                c.g[A0][0] = c.lh((c.g[S3][0] as u32).wrapping_add(0x1a2));
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                                c.sh((c.g[V0][0] as u32).wrapping_add(0x12), c.g[A0][0]);
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                                c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
                                if c.g[V1][0] == c.g[A1][0] {
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                                    break 'b_38cf40;
                                } else {
                                    'b_38cf2c: {
                                        'b_38cf1c: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V0][0] = 0x80000;
                                            if t {
                                                break 'b_38cf1c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == c.g[V0][0];
                                                c.g[A0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                                                if t {
                                                    break 'b_38cf2c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V1][0] = 0xfffffffffffffff3;
                                                    if t {
                                                        break 'b_38cf68;
                                                    } else {
                                                        break 'b_38cf1c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[A2][0] {
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                                            break 'b_38cf54;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                                            if t {
                                                break 'b_38cf64;
                                            } else {
                                                break 'b_38cf2c;
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[V1][0] = 0xfffffffffffffff3;
                                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                    let t = true;
                                    c.g[V0][0] = c.g[V0][0] | 0x8;
                                    if t {
                                        break 'b_38cf70;
                                    } else {
                                        break 'b_38cf40;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[V1][0] = 0xfffffffffffffff3;
                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            let t = true;
                            c.g[V0][0] = c.g[V0][0] | 0x4;
                            if t {
                                break 'b_38cf70;
                            } else {
                                break 'b_38cf54;
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = c.g[V0][0] | 0xc;
                        let t = true;
                        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                        if t {
                            break 'b_38cf74;
                        } else {
                            break 'b_38cf64;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = 0xfffffffffffffff3;
                    break 'b_38cf68;
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                break 'b_38cf70;
            }
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
            break 'b_38cf74;
        }
        'b_38cfe8: {
            c.cycles += 4;
            c.g[T2][0] = c.lw(c.g[S4][0] as u32);
            c.g[T3][0] = 0x30000000;
            let t = c.g[FP][0] == 0u64;
            c.g[T1][0] = c.g[T2][0] | c.g[T3][0];
            if t {
                break 'b_38cfe8;
            } else {
                c.cycles += 25;
                c.g[V0][0] = 0x10000000;
                c.g[V0][0] = c.g[V0][0] | 0x2;
                c.sd((c.g[T1][0] as u32).wrapping_add(0x8), 0u64);
                c.sd(c.g[T1][0] as u32, c.g[V0][0]);
                c.g[A0][0] = 0x6e010000;
                c.g[A0][0] = c.g[A0][0] | 0x80d0;
                c.g[V0][0] = 0x6c100000;
                c.g[V0][0] = c.g[V0][0] | 0x80c0;
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[A1][0] = 0x30000000;
                c.g[A1][0] = c.g[A1][0] | 0x10;
                c.g[V1][0] = c.lw(c.g[FP][0] as u32);
                c.sd((c.g[T1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                c.g[V1][0] = c.g[V1][0] << 32;
                c.sd((c.g[T1][0] as u32).wrapping_add(0x18), 0u64);
                c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                c.sd((c.g[T1][0] as u32).wrapping_add(0x20), 0u64);
                c.sd((c.g[T1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                c.sd((c.g[T1][0] as u32).wrapping_add(0x38), 0u64);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                c.sd((c.g[T1][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                break 'b_38cfe8;
            }
        }
        'b_38d0e4: {
            c.cycles += 31;
            c.g[V0][0] = c.g[S0][0] << 32;
            c.g[V1][0] = 0x50000000;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[A0][0] = 0x10000000;
            c.g[A0][0] = c.g[A0][0] | 0x1;
            c.sd((c.g[T1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            c.g[V1][0] = 0x500000;
            c.sd(c.g[T1][0] as u32, c.g[V0][0]);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4960));
            c.sd((c.g[T1][0] as u32).wrapping_add(0x8), 0u64);
            c.g[A1][0] = 0x14000000;
            c.sd((c.g[T1][0] as u32).wrapping_add(0x18), 0u64);
            c.g[A0][0] = 0xfff0000;
            c.g[A0][0] = c.g[A0][0] | 0xffff;
            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
            c.sd((c.g[T1][0] as u32).wrapping_add(0x20), 0u64);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
            c.g[V1][0] = c.g[V1][0] << 32;
            c.sd((c.g[T1][0] as u32).wrapping_add(0x28), c.g[V1][0]);
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x30));
            c.g[T1][0] = c.g[T1][0] & c.g[A0][0];
            c.sw(c.g[S4][0] as u32, c.g[T1][0]);
            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
            c.g[V0][0] = c.lw(c.g[A3][0] as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
            if t {
                break 'b_38d0e4;
            } else {
                c.cycles += 32;
                c.g[A1][0] = 0x60000;
                c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[A1][0] | 0x8000;
                c.g[V1][0] = 0xffffffffffffffe0;
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                c.g[A2][0] = c.g[A2][0] | c.g[T3][0];
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.ldr(3, c.g[T0][0] as u32);
                c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
                c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.sdr(3, c.g[V0][0] as u32);
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T2][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
                break 'b_38d0e4;
            }
        }
        'b_38d114: {
            c.cycles += 4;
            c.g[V0][0] = c.lw(c.g[S4][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            let t = c.g[S2][0] == 0u64;
            c.sw(c.g[S4][0] as u32, c.g[V0][0]);
            if t {
                break 'b_38d114;
            } else {
                c.cycles += 8;
                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                c.g[A2][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S5][0];
                c.g[A3][0] = 0x2;
                c.g[T0][0] = c.g[S0][0];
                c.g[T1][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x0038d114);
                c.g[T2][0] = c.g[S4][0];
                Callee {
                    addr: 0x0038d168,
                    f: ee::s12::PS2BezierMan_EmitPatchMatrixPass,
                }
                .call(c, 0x0038d114)?;
                break 'b_38d114;
            }
        }
        'b_38d134: {
            c.cycles += 2;
            let t = c.g[S7][0] == 0u64;
            c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            if t {
                break 'b_38d134;
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S6][0];
                c.g[T1][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x0038d134);
                c.g[A1][0] = 0x2;
                Callee {
                    addr: 0x0038d448,
                    f: ee::s12::PS2BezierMan_EmitPatchProjectorPasses,
                }
                .call(c, 0x0038d134)?;
                break 'b_38d134;
            }
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void PS2BezierMan_EmitPatchMatrixPass(BezMan* self, void* cacheEntry, Patch* obj,
/// int type, void* progs, Patch* prog, uint* handle)` (0x0038d168): pushes a copy
/// of the current draw state, picks the vector data address for the patch type
/// (0: 0xc0 stride, 1: 432, 2: 0x300), sets texture ids and bit groups, writes the
/// GIF packet with the matrix copy and program call, queues a ring entry and pops
/// the draw state again. A leaf: no guest calls.
pub fn ps2_bezier_man_emit_patch_matrix_pass(c: &mut Cpu) {
    'b_38d224: {
        'b_38d218: {
            'b_38d210: {
                'b_38d204: {
                    c.cycles += 25;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                    c.g[T4][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                    c.sq(16, c.g[SP][0] as u32);
                    c.g[T5][0] = c.g[A0][0];
                    c.g[T3][0] = c.g[A3][0];
                    c.g[S0][0] = c.g[T0][0];
                    c.g[V0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
                    c.g[T9][0] = c.g[T1][0];
                    c.g[T6][0] = c.g[T2][0];
                    c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.ldr(3, c.g[V0][0] as u32);
                    c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x1b));
                    c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x14));
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x23));
                    c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x1c));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[A3][0]);
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[T4][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
                    let t = c.g[T3][0] == c.g[V1][0];
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    if t {
                        break 'b_38d204;
                    } else {
                        'b_38d1f8: {
                            'b_38d1e8: {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[T3][0] as i64) < 2i64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_38d1e8;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[T3][0] == 0u64;
                                    c.g[A3][0] = 0u64;
                                    if t {
                                        break 'b_38d1f8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
                                        if t {
                                            break 'b_38d224;
                                        } else {
                                            break 'b_38d1e8;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[T3][0] == c.g[V0][0];
                            c.g[A3][0] = 0u64;
                            if t {
                                break 'b_38d210;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
                                if t {
                                    break 'b_38d224;
                                } else {
                                    break 'b_38d1f8;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0xc0;
                        let t = true;
                        c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x1c4));
                        if t {
                            break 'b_38d218;
                        } else {
                            break 'b_38d204;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = 0x1b0;
                let t = true;
                c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x1c8));
                if t {
                    break 'b_38d218;
                } else {
                    break 'b_38d210;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x300;
            c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x1cc));
            break 'b_38d218;
        }
        c.cycles += 3;
        c.mult(0, 2, 4, 2);
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
        break 'b_38d224;
    }
    'b_38d238: {
        c.cycles += 5;
        c.g[V0][0] = 0xffffffffffffffff;
        c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[V1][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x1a4));
        if (c.g[V1][0] as i64) < 0 {
            c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x10a8));
            break 'b_38d238;
        } else {
            break 'b_38d238;
        }
    }
    'b_38d288: {
        'b_38d278: {
            c.cycles += 11;
            c.g[V0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
            c.g[A1][0] = 0x400000;
            c.sh((c.g[V0][0] as u32).wrapping_add(0x12), c.g[V1][0]);
            c.g[A0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.g[V0][0] = c.g[V0][0] | 0x30;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
            c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
            let t = c.g[V1][0] == 0u64;
            c.g[V1][0] = 0xfffffffffffff07f;
            if t {
                break 'b_38d278;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                let t = true;
                c.g[V0][0] = c.g[V0][0] | 0x880;
                if t {
                    break 'b_38d288;
                } else {
                    break 'b_38d278;
                }
            }
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x100;
        break 'b_38d288;
    }
    'b_38d424: {
        c.cycles += 70;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[T3][0] as u32) << 1);
        c.g[A2][0] = 0x78000000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[T8][0] = c.lw(c.g[T6][0] as u32);
        c.mult(0, 3, 3, 3);
        c.g[T7][0] = 0x30000000;
        c.g[A2][0] = c.g[A2][0] | 0x8040;
        c.g[A3][0] = c.g[A3][0] << 32;
        c.g[A0][0] = c.g[T8][0] | c.g[T7][0];
        c.g[T1][0] = 0x10000000;
        c.g[T1][0] = c.g[T1][0] | 0x1;
        c.g[T2][0] = 0x40400000;
        c.g[T2][0] = c.g[T2][0] | 0x4040;
        c.g[T2][0] = c.g[T2][0] << 16;
        c.g[T2][0] = c.g[T2][0] | 0x2000;
        c.g[T2][0] = c.g[T2][0] << 16;
        c.g[T3][0] = 0x10000000;
        c.g[T3][0] = c.g[T3][0] | 0x6;
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 1);
        c.g[A1][0] = sx((c.g[V1][0] as u32) << 16);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = c.g[A1][0] | c.g[A2][0];
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[A1][0] = c.g[A1][0] << 32;
        c.g[V0][0] = c.g[V0][0] | c.g[T7][0];
        c.g[T0][0] = 0x20000000;
        c.g[A3][0] = c.g[V0][0] | c.g[A3][0];
        c.g[V1][0] = 0x6c040000;
        c.g[V1][0] = c.g[V1][0] | 0x8080;
        c.g[V1][0] = c.g[V1][0] << 32;
        c.sd(c.g[A0][0] as u32, c.g[T1][0]);
        c.g[A2][0] = sx((c.g[T5][0] as u32).wrapping_add(0x360));
        c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T2][0]);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
        c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x20), c.g[A3][0]);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x30), c.g[T3][0]);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x40), c.g[T0][0]);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x48), c.g[V1][0]);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x28), 0u64);
        c.sd((c.g[A0][0] as u32).wrapping_add(0x38), 0u64);
        c.lqc2(1, c.g[A2][0] as u32);
        c.lqc2(2, (c.g[A2][0] as u32).wrapping_add(0x10));
        c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[A2][0] as u32).wrapping_add(0x30));
        c.sqc2(1, c.g[V0][0] as u32);
        c.sqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
        c.sqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
        c.sqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x4));
        c.g[A2][0] = 0x14000000;
        c.sd((c.g[A0][0] as u32).wrapping_add(0x90), 0u64);
        c.g[A1][0] = 0xfff0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[A1][0] = c.g[A1][0] | 0xffff;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
        c.g[V0][0] = c.g[V0][0] << 32;
        c.sd((c.g[A0][0] as u32).wrapping_add(0x98), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa0));
        c.g[T1][0] = c.g[A0][0] & c.g[A1][0];
        c.sw(c.g[T6][0] as u32, c.g[T1][0]);
        c.g[A3][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x18f0));
        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[T0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
        if t {
            break 'b_38d424;
        } else {
            c.cycles += 33;
            c.g[A1][0] = 0x60000;
            c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[A1][0] | 0x8000;
            c.g[V1][0] = 0xffffffffffffffe0;
            c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
            c.g[A2][0] = c.g[A2][0] | c.g[T7][0];
            c.g[V0][0] = c.g[V0][0] & 0x1f;
            c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
            c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[V1][0] = c.lw(c.g[A3][0] as u32);
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.sw(c.g[A3][0] as u32, c.g[V1][0]);
            c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
            c.ldr(3, c.g[T0][0] as u32);
            c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
            c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
            c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
            c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
            c.sdr(3, c.g[V0][0] as u32);
            c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
            c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
            c.g[V1][0] = 0x1;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T8][0]);
            c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), c.g[V1][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
            c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
            break 'b_38d424;
        }
    }
    c.cycles += 9;
    c.g[V0][0] = c.lw(c.g[T6][0] as u32);
    c.lq(16, c.g[SP][0] as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
    c.sw(c.g[T6][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0xe84));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x14));
    c.sw((c.g[T4][0] as u32).wrapping_add(0xe84), c.g[V1][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t;
    return;
}

/// `s0 + off` for the Bezier manager (`s0` is `this`).
pub(crate) fn bezier_s0(c: &Cpu, off: u32) -> u32 {
    (c.g[S0][0] as u32).wrapping_add(off)
}

/// `void PS2BezierMan_FreeCaches(PS2BezierMan* self)` (0x38b0f8,
/// cPS2BezierMan_Purge): runs the stub at 0x38d660, then frees the blocks at
/// +0x440, +0x444 and +0x448 through EA_MemFree (0x317e98) where they are set.
pub fn ps2_bezier_man_free_caches_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0038_d660,
            f: ee::s12::f_0038d660,
        }
        .call(c, 0x0038_b10c)?;
        c.g[A0][0] = Addr(bezier_s0(c, 0x440)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = Addr(bezier_s0(c, 0x444)).lw(c);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b120)?;
            c.g[A0][0] = Addr(bezier_s0(c, 0x444)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[A0][0] = Addr(bezier_s0(c, 0x448)).lw(c);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b134)?;
            c.g[A0][0] = Addr(bezier_s0(c, 0x448)).lw(c);
        }
        c.cycles += 2;
        let free_last = c.g[A0][0] != 0;
        frame.restore(c, S0, 0x10);
        if free_last {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b148)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2BezierMan_FreeCachesStub(void)` (0x38d660): the caches are not
/// kept any more, so there is nothing to free; it only returns.
pub fn ps2_bezier_man_free_caches_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void PS2BezierMan_EmitPatchProjectorPasses(self, int kind, void *tbl,
/// sPatch *patch, unsigned lights, unsigned *handle)` (0x38d448): for every light
/// `k < gm->numLights` whose bit is set in `lights` (`t0`), emits one projector
/// pass for the patch: sets the render state's texture (`gm+0xe84` top state,
/// halfword `+0x12` from the light's texture at `gm+0x13f0+0xa0k+0x94`) and
/// its type bits to 3, then writes a 0x70-byte DMA record at `*handle` (uncached
/// alias): the light's 4x4 projection matrix (VU0 `lqc2/sqc2` from
/// `gm+0x1440+0xa0k`) and the VU1 start address taken from the program table
/// `tbl + 8*patch[1] + 0x30`; the record is then also queued in the draw ring
/// (`gm+0x18f0`, up to 2600 entries) by copying the 0x14 bytes of the state's
/// top state. `a2` tbl, `a3` patch, `t0` lights, `t1` handle. A leaf: no calls.
/// Kept as the generated block structure so every register and cycle matches.
#[allow(unreachable_code, unused_assignments, unused_variables)]
pub fn ps2_bezier_man_emit_patch_projector_passes(c: &mut Cpu) {
    'b_38d60c: {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.g[T3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[T5][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S2][0] = c.g[T0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(30, c.g[SP][0] as u32);
        c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x13ec));
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        if t {
            break 'b_38d60c;
        } else {
            c.cycles += 18;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
            c.g[T0][0] = 0xfff0000;
            c.g[T9][0] = 0x60000;
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[FP][0] = 0xffffffffffffffcf;
            c.g[S7][0] = 0x10000000;
            c.g[S7][0] = c.g[S7][0] | 0x6;
            c.g[S6][0] = 0x20000000;
            c.g[S5][0] = 0x6c040000;
            c.g[S5][0] = c.g[S5][0] | 0x8080;
            c.g[S5][0] = c.g[S5][0] << 32;
            c.g[S4][0] = 0x14000000;
            c.g[T0][0] = c.g[T0][0] | 0xffff;
            c.g[T9][0] = c.g[T9][0] | 0x8000;
            c.g[S3][0] = 0xffffffffffffffe0;
            c.g[T8][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1440));
            c.g[T7][0] = sx((c.g[T3][0] as u32).wrapping_add(0x13f0));
            'l_38d4d0: loop {
                'b_38d5f4: {
                    c.cycles += 4;
                    c.g[V0][0] =
                        sx(((c.g[S2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = 0x30000000;
                    if t {
                        break 'b_38d5f4;
                    } else {
                        'b_38d5e8: {
                            c.cycles += 37;
                            c.g[A0][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x94));
                            c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0xe84));
                            c.g[T6][0] = c.lw(c.g[T1][0] as u32);
                            c.sh((c.g[V0][0] as u32).wrapping_add(0x12), c.g[A0][0]);
                            c.g[V1][0] = c.g[T6][0] | c.g[A2][0];
                            c.g[A0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0xe84));
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                            c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                            c.g[V0][0] = c.g[V0][0] | 0x30;
                            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                            c.sd(c.g[V1][0] as u32, c.g[S7][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[S6][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[S5][0]);
                            c.lqc2(1, c.g[T8][0] as u32);
                            c.lqc2(2, (c.g[T8][0] as u32).wrapping_add(0x10));
                            c.lqc2(3, (c.g[T8][0] as u32).wrapping_add(0x20));
                            c.lqc2(4, (c.g[T8][0] as u32).wrapping_add(0x30));
                            c.sqc2(1, c.g[A1][0] as u32);
                            c.sqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                            c.sqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
                            c.sqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x60), 0u64);
                            c.g[S0][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                            c.g[V0][0] = c.g[V0][0] | c.g[S4][0];
                            c.g[V0][0] = c.g[V0][0] << 32;
                            c.sd((c.g[V1][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x70));
                            c.g[T4][0] = c.g[V1][0] & c.g[T0][0];
                            c.sw(c.g[T1][0] as u32, c.g[T4][0]);
                            c.g[A3][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x18f0));
                            c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[T2][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0xe84));
                            if t {
                                break 'b_38d5e8;
                            } else {
                                c.cycles += 29;
                                c.g[A1][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add(c.g[T9][0] as u32));
                                c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
                                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                                c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                                c.g[A0][0] = c.g[A0][0] | c.g[A2][0];
                                c.g[V1][0] = c.g[V1][0] & 0x1f;
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.sw((c.g[T2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                                c.ldl(3, (c.g[T2][0] as u32).wrapping_add(0x7));
                                c.ldr(3, c.g[T2][0] as u32);
                                c.ldl(4, (c.g[T2][0] as u32).wrapping_add(0xf));
                                c.ldr(4, (c.g[T2][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10));
                                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                                c.sdr(3, c.g[V0][0] as u32);
                                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
                                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                                c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), c.g[S0][0]);
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T6][0]);
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                                c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
                                break 'b_38d5e8;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(c.g[T1][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.sw(c.g[T1][0] as u32, c.g[V0][0]);
                        break 'b_38d5f4;
                    }
                }
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x13ec));
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x1));
                c.g[T8][0] = sx((c.g[T8][0] as u32).wrapping_add(0xa0));
                c.g[V0][0] = (c.g[T5][0] < c.g[V0][0]) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0xa0));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0038d4d0);
                    }
                    continue 'l_38d4d0;
                } else {
                    break 'b_38d60c;
                }
            }
        }
    }
    c.cycles += 11;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(30, c.g[SP][0] as u32);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::fe::ovstatepause::tests::oc778_install_fakes;
    pub(crate) use crate::game::render::lightman::tests::oc1065_install_fakes;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::be::beintoption::tests::slot;
    pub(crate) use crate::game::be::beintoption::tests::VT;
    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::object::railmodifier::tests::h;

    use ssx3_ee::s12;

    // PS2BezierMan_v21 -----------------------------------------------------

    pub(crate) const BEZIER: u32 = DATA;

    #[test]
    fn ps2bezier_man_v21_matches() {
        covered(
            ssx3_ee::s12::f_0038b338__generated,
            ps2bezier_man_v21,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38b338, 1);
                c.g[A0][0] = BEZIER as u64;
                c.bus
                    .write32(GAME_GP + 0x13e4, if seed % 3 == 0 { 0 } else { 1 });
                let vt = vtable(c, &mut r, &[0xb0]);
                c.bus.write32(BEZIER + 4, vt);
            },
        );
    }

    // PS2BezierMan_TestPatchBounds ----------------------------------------

    pub(crate) const PATCH: u32 = DATA + 0x2000;

    pub(crate) const GMAN: u32 = DATA + 0x4000;

    pub(crate) const LIGHT_VT: u32 = DATA + 0x5000;

    #[test]
    fn ps2bezier_man_test_patch_bounds_matches() {
        covered(
            ssx3_ee::s12::f_0038ca08__generated,
            ps2bezier_man_test_patch_bounds,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38ca08, 1);
                c.g[A1][0] = PATCH as u64;
                c.bus.write32(GAME_GP + 0x2a90, GMAN);
                for k in 0..7 {
                    c.bus.write32(PATCH + 0x158 + 4 * k, r.edge_f32());
                }
                // The manager's light list: empty half the time, else one item
                // whose method is a fake.
                let count = (seed % 2) as u32;
                c.bus.write32(GMAN + 0x13ec, count);
                if count != 0 {
                    c.bus.write32(GMAN + 0x10d8, LIGHT_VT);
                    c.bus.write16(LIGHT_VT + 0x2e8, 0);
                    c.bus.write32(LIGHT_VT + 0x2ec, FAKE);
                }
            },
        );
    }

    #[test]
    fn ps2_bezier_man_call_v17_matches() {
        covered(
            s12::f_0038d638__generated,
            ps2_bezier_man_call_v17,
            120,
            &|c, seed| {
                machine(c, seed, 0x38d638, 1);
                clear(c, OBJ, 0x40);
                clear(c, VT, 0x200);
                slot(c, VT, 0x88);
                c.bus.write32(OBJ + 4, VT);
                c.g[A0][0] = sx(OBJ);
                let _ = seed;
            },
        );
    }

    pub(crate) fn b(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    #[test]
    fn ps2_bezier_man_v22_matches() {
        check(
            ee::s12::f_0038b190__generated,
            ps2_bezier_man_v22,
            100,
            &|c, seed| {
                machine(c, seed, 0x0038_b190, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.g[T0][0] = (DATA + 0x3000) as u64;
                let types: [u8; 4] = match seed % 6 {
                    0 => [4, 4, 4, 4],
                    1 => [6, 6, 6, 6],
                    2 => [8, 8, 8, 8],
                    3 => [1, 2, 8, 3],
                    4 => [1, 2, 3, 4],
                    _ => [2, 2, 2, 2],
                };
                for (i, t) in types.into_iter().enumerate() {
                    b(c, DATA + 0x2000 + i as u32, t);
                }
                h(c, DATA + 0x1000 + 0x1a6, 0x11);
                h(c, DATA + 0x1000 + 0x1a8, 0x22);
                h(c, DATA + 0x1000 + 0x1aa, 0x33);
                match seed % 4 {
                    1 => w(c, DATA + 0x3c9c, 220), // 6-list full
                    2 => {
                        w(c, DATA + 0x458, 900); // 4-list full
                        w(c, DATA + 0x4d84, 75); // mix list full
                    }
                    3 => {
                        w(c, DATA + 0x4a60, 50); // 8-list full
                        w(c, DATA + 0x3c9c, 220);
                    }
                    _ => {}
                }
            },
        );
    }

    use crate::difftest::fill;

    /// `PS2BezierMan_RenderPatchFX` with its real callees: FX off, stale view
    /// matrices (the real `UpdateViewMatrices` on random data), 0..3 type-8
    /// patches and 0..3 mixed patches of either kind sign. The patches'
    /// flag word (+0xc) has bit 23 clear, so `DrawPatchFX` returns at once
    /// after each scratchpad upload.
    #[test]
    fn ps2_bezier_man_render_patch_fx_matches() {
        use crate::difftest::GAME_GP;
        check(
            ee::s12::f_0038d968__generated,
            ps2_bezier_man_render_patch_fx,
            120,
            &|c, seed| {
                let mut r = Rng(0x0038_d968 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0038_d968;
                let gfx = DATA;
                let top = DATA + 0x1000 + 4 * (seed % 4);
                let bm = DATA + 0x10000;
                let objs = DATA + 0x20000;
                let scratch = DATA + 0x30000;
                c.bus.write32(GAME_GP + 0x13f0, (seed % 7 == 0) as u32);
                c.bus.write32(GAME_GP + 0x2a90, gfx);
                c.bus.write32(GAME_GP + 0x2a94, scratch);
                // View data for UpdateViewMatrices.
                for k in 0..0x40 {
                    c.bus.write32(gfx + 4 * k, r.f32(2.0));
                }
                // Render-state stack: top entry plus room for the push.
                c.bus.write32(gfx + 0xe84, top);
                fill(c, &mut r, top, 10);
                let sel = r.next_u32() % 4;
                c.bus.write32(gfx + 0x64, sel);
                fill(c, &mut r, gfx + 0xf50, 4);
                c.bus.write32(
                    gfx + 0x6b90,
                    if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
                );
                // Patches.
                let n8 = seed % 4;
                let nmix = (seed / 4) % 4;
                c.bus.write32(bm + 0x4a60, n8);
                c.bus.write32(bm + 0x4d84, nmix);
                for i in 0..4 {
                    let obj = objs + 0x200 * i;
                    c.bus.write32(obj + 0xc, r.next_u32() & !0x80_0000);
                    c.bus.write32(bm + 0x4a64 + 0x10 * i, obj);
                    c.bus.write32(bm + 0x4d88 + 0x14 * i, obj + 0x1000);
                    c.bus.write32(obj + 0x100c, r.next_u32() & !0x80_0000);
                    let kind = match r.next_u32() % 3 {
                        0 => 0,
                        1 => (r.next_u32() % 4) as i32 + 1,
                        _ => -(1 + (r.next_u32() % 4) as i32),
                    };
                    c.bus.write32(bm + 0x4d88 + 0x14 * i + 8, kind as u32);
                }
                c.g[A0][0] = bm as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    /// `PS2BezierMan_BuildPatchPacket`: lod types 0, 1, 2 and out of range
    /// (negative, 3+), random slot indices and table bases.
    #[test]
    fn ps2_bezier_man_build_patch_packet_matches() {
        check(
            ee::s12::f_0038c788__generated,
            ps2_bezier_man_build_patch_packet,
            200,
            &|c, seed| {
                let mut r = Rng(0x0038_c788 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0038_c788;
                let bm = DATA;
                fill(c, &mut r, bm + 0x58, 9);
                for t in 0..3 {
                    c.bus
                        .write32(bm + 0x44c + 4 * t, DATA + 0x10000 + 0x4000 * t);
                }
                let ty: i64 = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => -1 - (r.next_u32() % 4) as i64,
                    4 => 3 + (r.next_u32() % 4) as i64,
                    _ => (seed % 3) as i64,
                };
                c.g[A0][0] = bm as u64;
                c.g[A1][0] = ty as u64;
                c.g[A2][0] = (r.next_u32() % 32) as u64;
                // Out-of-range types read their packet pointer from the
                // tables around +0x44c; keep it inside the test area.
                for t in -5i32..8 {
                    if !(0..3).contains(&t) {
                        let a = (bm as i32 + 0x44c + 4 * t) as u32;
                        c.bus.write32(a, DATA + 0x20000 + 0x800 * (t + 5) as u32);
                    }
                }
            },
        );
    }

    use crate::difftest::fakes;
    pub(crate) use crate::game::fe::ovstatepause::tests::oc778_arm;

    use std::sync::atomic::AtomicUsize;

    /// `PS2BezierMan_Init`: the real `BezierPatchCache_Init` and the grid
    /// loops over random FPU constants, with the unwinds of its calls.
    #[test]
    fn cps2_bezier_man_init_matches() {
        oc778_install_fakes();
        fakes::covered(
            ee::s12::f_0038af30__generated,
            cps2_bezier_man_init,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x778c, 16);
                oc778_arm(c);
                c.g[A0][0] = sx(DATA + 0x1_0000);
                for off in [0x26ccu32, 0x26c8, 0x26c4, 0x26c0] {
                    let v = r.f32(10.0);
                    c.bus.write32(GAME_GP - off, v);
                }
            },
        );
    }

    /// Gate word of the allocator fake (in each machine's own RAM).
    pub(crate) const OC795_CTRL: u32 = DATA + 0x20_0010;

    pub(crate) const OC795_MAGIC: u32 = 0x0795_f4c3;

    pub(crate) static OC795_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake of the allocator (`0x317d70`): logs the call like
    /// `fakes::fake_call`, may unwind, returns a pointer into the data area.
    pub(crate) fn oc795_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(OC795_CTRL) != OC795_MAGIC {
            return fakes::fall_back(c, &OC795_PREV_ALLOC, ee::s10::f_00317d70__generated);
        }
        fakes::fake_call(c, 0x0031_7d70);
    }

    pub(crate) fn oc795_hook() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc795_fake_alloc,
            &OC795_PREV_ALLOC,
        );
    }

    /// `PS2BezierMan_Create`: the allocator is faked (and unwinds on some
    /// seeds); the constructor runs for real on the fixture.
    #[test]
    fn ps2_bezier_man_create_matches() {
        oc795_hook();
        fakes::covered(
            ee::s12::f_0038aef8__generated,
            ps2_bezier_man_create,
            30,
            &|c, seed| {
                let _ = machine(c, seed, 0x7950_0002, 1);
                c.bus.write32(OC795_CTRL, OC795_MAGIC);
            },
        );
    }

    /// `PS2BezierMan_v04`: resets the cache at +0x10; pool counts of 0..3 with
    /// valid array pointers so the real `BezierPatchCache_Reset` and `MEM_fill`
    /// run on short paths.
    #[test]
    fn ps2_bezier_man_v04_matches() {
        check(
            ssx3_ee::s12::f_0038b158__generated,
            ps2_bezier_man_v04,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x799a, 0);
                c.pc = 0x0038_b158;
                let man = DATA + 0x1_0000;
                let cache = man + PS2_BEZIER_MAN_V04_CACHE;
                c.g[A0][0] = sx(man);
                for k in 0..3u32 {
                    let (n, nb) = (r.next_u32() % 4, r.next_u32() % 4);
                    c.bus.write32(cache + 4 * k, n);
                    c.bus.write32(cache + 0x1a8 + 4 * k, nb);
                    c.bus
                        .write32(cache + 0xc + 4 * k, DATA + 0x2_0000 + 0x400 * k);
                    c.bus
                        .write32(cache + 0x18 + 4 * k, DATA + 0x3_0000 + 0x400 * k);
                    c.bus
                        .write32(cache + 0x30 + 4 * k, DATA + 0x4_0000 + 0x400 * k);
                    c.bus
                        .write32(cache + 0x1cc + 4 * k, DATA + 0x5_0000 + 0x400 * k);
                }
            },
        );
    }

    use crate::difftest;

    pub(crate) use crate::game::render::lightman::tests::OC1065_CTRL;
    pub(crate) use crate::game::render::lightman::tests::OC1065_MAGIC;

    pub(crate) use crate::game::render::ps2graphicsman::tests::oc1071_hook;
    pub(crate) use crate::game::render::ps2graphicsman::tests::OC1071_GATE;
    pub(crate) use crate::game::render::ps2graphicsman::tests::OC1071_MAGIC;
    use std::ptr::addr_of_mut;

    // ------------------------------------------------ PS2BezierMan_EmitPatchType1

    pub(crate) const OC1064_CTRL: u32 = DATA + 0xf_0000;

    pub(crate) const OC1064_MAGIC: u32 = 0x1064_c0de;

    /// Fake `k` (0 = TestPatchBounds, 1 = EmitPatchMatrixPass, 2 =
    /// EmitPatchProjectorPasses): gate, call counter and forced result of
    /// call `n` live in the machine's RAM; the call logs its arguments.
    pub(crate) fn oc1064_gate(k: u32) -> u32 {
        OC1064_CTRL + 0x400 * k
    }

    pub(crate) fn oc1064_fake_call(c: &mut Cpu, k: u32) {
        let g = oc1064_gate(k);
        let n = c.bus.read32(g + 4);
        c.bus.write32(g + 4, n + 1);
        let args = [A0, A1, A2, A3, T0, T1, T2];
        for (i, reg) in args.into_iter().enumerate() {
            let v = c.g[reg][0] as u32;
            c.bus.write32(g + 0x200 + 0x20 * (n % 8) + 4 * i as u32, v);
        }
        let v = c.bus.read32(g + 0x100 + 4 * (n % 8));
        c.g[V0][0] = sx(v);
        c.cycles += 5;
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) static OC1064_PREV_BOUNDS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1064_PREV_MATRIX: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1064_PREV_PROJECTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1064_fake_bounds(c: &mut Cpu) {
        if c.bus.read32(oc1064_gate(0)) != OC1064_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &OC1064_PREV_BOUNDS,
                ee::s12::f_0038ca08__generated,
            );
        }
        oc1064_fake_call(c, 0)
    }

    pub(crate) fn oc1064_fake_matrix(c: &mut Cpu) {
        if c.bus.read32(oc1064_gate(1)) != OC1064_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &OC1064_PREV_MATRIX,
                ee::s12::f_0038d168__generated,
            );
        }
        oc1064_fake_call(c, 1)
    }

    pub(crate) fn oc1064_fake_projector(c: &mut Cpu) {
        if c.bus.read32(oc1064_gate(2)) != OC1064_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                &OC1064_PREV_PROJECTOR,
                ee::s12::f_0038d448__generated,
            );
        }
        oc1064_fake_call(c, 2)
    }

    pub(crate) fn oc1064_hook_all() {
        use crate::difftest::fakes::hook;
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038CA08),
            oc1064_fake_bounds,
            &OC1064_PREV_BOUNDS,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038D168),
            oc1064_fake_matrix,
            &OC1064_PREV_MATRIX,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038D448),
            oc1064_fake_projector,
            &OC1064_PREV_PROJECTOR,
        );
    }

    /// `PS2BezierMan_EmitPatchType1`: the patch record is uploaded to the
    /// scratchpad by the real `SprStream_Upload`; its flags pick the four
    /// render-state cases (and the reflection/shadow passes, whose callees
    /// are gated fakes with random results), the gp toggles disable the
    /// passes, the ring (count below or at its 2600 limit) takes an entry;
    /// patches that are not drawn go to the defer list.
    #[test]
    fn ps2_bezier_man_emit_patch_type1_matches() {
        const MAN: u32 = DATA;
        const ARGS: u32 = DATA + 0x2_0000;
        const SRC: u32 = DATA + 0x2_1000;
        const HANDLE: u32 = DATA + 0x2_2000;
        const GM: u32 = DATA + 0x3_0000;
        const TOP: u32 = DATA + 0x3_4000;
        const TABLE: u32 = DATA + 0x3_8000;
        const RING: u32 = DATA + 0x4_0000;
        oc1064_hook_all();
        check(
            ee::s12::f_0038ca70__generated,
            ps2_bezier_man_emit_patch_type1,
            300,
            &|c, seed| {
                let mut r = Rng(0x0038_ca70 ^ (seed as u64) << 11);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let kind = r.next_u32() % 4;
                for (k, on) in [(0, true), (1, seed % 4 != 0), (2, seed % 3 != 0)] {
                    let g = oc1064_gate(k);
                    c.bus.write32(g, if on { OC1064_MAGIC } else { 0 });
                    c.bus.write32(g + 4, 0);
                    for n in 0..8 {
                        c.bus.write32(g + 0x100 + 4 * n, r.next_u32() % 3);
                    }
                }
                // Patch record in RAM; the scratchpad copy lands at gp+0x2a94.
                fill(c, &mut r, SRC, 0x6c);
                let flags = [0, 0x80000, 0x100000, 0x180000][(r.next_u32() % 4) as usize]
                    | [0, 0x200000, 0x400000, 0x600000][(r.next_u32() % 4) as usize];
                c.bus.write32(SRC + 0xc, flags);
                c.bus
                    .write32(GAME_GP + 0x2a94, 0x7000_0000 + (r.next_u32() % 0x30) * 0x10);
                c.bus.write32(ARGS, SRC);
                c.bus.write32(ARGS + 4, r.next_u32() % 4);
                // The manager: cache entries, texture bases, defer list.
                fill(c, &mut r, MAN + 0x28, 8);
                fill(c, &mut r, MAN + 0x44c, 8);
                c.bus.write32(MAN + 0x5364, r.next_u32() % 8);
                // Render state ring of the graphics manager.
                c.bus.write32(GAME_GP + 0x2a90, GM);
                c.bus.write32(GM + 0xe84, TOP);
                c.bus.write32(GM + 0x18f0, RING);
                fill(c, &mut r, TOP, 8);
                c.bus.write32(
                    RING,
                    if seed % 7 == 3 {
                        2600
                    } else {
                        r.next_u32() % 4
                    },
                );
                c.bus.write32(RING + 0x69cc0, r.next_u32());
                c.bus.write32(RING + 0x69cc4, r.next_u32());
                // Toggles and the per-kind tables.
                c.bus
                    .write32(GAME_GP + 0x13e8, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(GAME_GP + 0x13ec, if seed % 4 == 1 { 0 } else { 1 });
                for k in 0..4 {
                    c.bus.write32(0x0044_b9d0 + 4 * k, TABLE + 0x100 * k);
                    fill(c, &mut r, TABLE + 0x100 * k, 0x40);
                }
                c.bus
                    .write32(HANDLE, DATA + 0x3_c000 + (r.next_u32() & 0xff0));
                c.g[A0][0] = sx(MAN);
                c.g[A1][0] = kind as u64;
                c.g[A2][0] = (r.next_u32() % 4) as u64;
                c.g[A3][0] = sx(ARGS);
                c.g[T0][0] = sx(HANDLE);
            },
        );
    }

    /// Bezier manager (`self`), graphics manager (`gp+0x2a90`), draw state stack
    /// (`gfx+0xe84`), ring (`gfx+0x18f0`), patch (`sPatch`), its copy, the
    /// handle word, the packet buffer and the program table.
    pub(crate) const OC1065_SELF: u32 = DATA + 0x100;

    pub(crate) const OC1065_GFX: u32 = DATA + 0x2000;

    pub(crate) const OC1065_DSTATE: u32 = DATA + 0x3000;

    pub(crate) const OC1065_PATCH: u32 = DATA + 0x400;

    pub(crate) const OC1065_HANDLE: u32 = DATA + 0x500;

    pub(crate) const OC1065_EXT: u32 = DATA + 0x520;

    pub(crate) const OC1065_COPY: u32 = DATA + 0x8000;

    pub(crate) const OC1065_RING: u32 = DATA + 0x10_000;

    pub(crate) const OC1065_CACHE: u32 = DATA + 0x2_0000;

    pub(crate) const OC1065_PROGS: u32 = DATA + 0x3_0000;

    pub(crate) const OC1065_PACKETS: u32 = DATA + 0x4_0000;

    /// The structures both bezier emitters read: the graphics manager with its
    /// draw state stack and ring, the patch and its copy (flags chosen by
    /// `seed`), the handle word and the program table.
    pub(crate) fn oc1065_bezier_world(c: &mut Cpu, r: &mut difftest::Rng, seed: u32) {
        difftest::elf_image(c);
        let gp = difftest::GAME_GP;
        difftest::fill(c, r, OC1065_SELF, 0x180);
        difftest::fill(c, r, OC1065_GFX, 0x80);
        difftest::fill(c, r, OC1065_DSTATE, 0x20);
        difftest::fill(c, r, OC1065_PATCH, 0x10);
        difftest::fill(c, r, OC1065_COPY, 0x80);
        difftest::fill(c, r, OC1065_EXT, 4);
        c.bus.write32(gp + 0x2a90, OC1065_GFX);
        c.bus.write32(OC1065_GFX + 0xe84, OC1065_DSTATE);
        c.bus.write32(OC1065_GFX + 0x18f0, OC1065_RING);
        c.bus.write32(OC1065_GFX + 0x10a8, r.next_u32() & 0x7fff);
        // The ring: a count below or at the 2600-entry limit, and its two keys.
        let count = if seed % 5 == 0 {
            2600 + r.next_u32() % 50
        } else {
            r.next_u32() % 2600
        };
        c.bus.write32(OC1065_RING, count);
        c.bus.write32(OC1065_RING + 0x69cc0, r.next_u32());
        c.bus.write32(OC1065_RING + 0x69cc4, r.next_u32());
        // gp flags: projector / reflection kill switches.
        c.bus.write32(gp + 0x13e8, (seed / 3 % 2) * r.next_u32());
        c.bus.write32(gp + 0x13ec, (seed / 7 % 2) * r.next_u32());
        // The patch copy: the 0x180000 mode bits and the reflection bits.
        let mode = [0u32, 0x80000, 0x100000, 0x180000][(seed / 2 % 4) as usize];
        let refl = [0u32, 0x200000, 0x400000, 0x600000][(seed / 5 % 4) as usize];
        c.bus
            .write32(OC1065_COPY + 0xc, mode | refl | (r.next_u32() & 0x7fff));
        // The patch: object, program index.
        c.bus.write32(OC1065_PATCH, OC1065_COPY);
        c.bus.write32(OC1065_PATCH + 4, r.next_u32() % 8);
        c.bus.write32(OC1065_PATCH + 8, r.next_u32() % 16);
        // The manager: pointers the pass adds offsets to.
        c.bus.write32(OC1065_SELF + 0x30, OC1065_CACHE);
        c.bus.write32(OC1065_SELF + 0x454, OC1065_PROGS);
        // The handle word: the next packet position (28-bit, 16-byte aligned).
        c.bus.write32(
            OC1065_HANDLE,
            (OC1065_PACKETS + (r.next_u32() % 0x80) * 16) & 0x0fff_ffff,
        );
    }

    /// `PS2BezierMan_EmitPatchType2` against the generated body: all four
    /// 0x180000 mode cases, with and without the `ext` tag and the reflection
    /// and projector passes, ring full or not, callees unwinding.
    #[test]
    fn ps2_bezier_man_emit_patch_type2_matches() {
        oc1065_install_fakes();
        fakes::covered(
            ee::s12::f_0038ce20__generated,
            ps2_bezier_man_emit_patch_type2,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1065_0001, 3);
                oc1065_bezier_world(c, &mut r, seed);
                c.bus.write32(OC1065_CTRL, OC1065_MAGIC);
                c.bus.write32(OC1065_CTRL + 4, OC1065_COPY);
                c.bus
                    .write32(OC1065_CTRL + 8, [0, 1, 7][(seed / 3 % 3) as usize]);
                c.g[A0][0] = sx(OC1065_SELF);
                c.g[A1][0] = sx(r.next_u32() % 4);
                c.g[A2][0] = sx(OC1065_PATCH);
                c.g[A3][0] = if seed % 2 == 0 { 0 } else { sx(OC1065_EXT) };
                c.g[T0][0] = sx(OC1065_HANDLE);
            },
        );
    }

    /// `PS2BezierMan_EmitPatchMatrixPass` against the generated body: patch
    /// types 0, 1, 2 and the default (3, -1, 5), both bit-group cases, negative
    /// and positive texture ids, ring entries recorded or skipped.
    #[test]
    fn ps2_bezier_man_emit_patch_matrix_pass_matches() {
        difftest::check_lohi(
            ee::s12::f_0038d168__generated,
            ps2_bezier_man_emit_patch_matrix_pass,
            240,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1065_0002, 0);
                oc1065_bezier_world(c, &mut r, seed);
                let ty = [0u32, 1, 2, 3, u32::MAX, 5][(seed % 6) as usize];
                c.g[A0][0] = sx(OC1065_SELF);
                c.g[A1][0] = sx(OC1065_CACHE);
                c.g[A2][0] = sx(OC1065_COPY);
                c.g[A3][0] = sx(ty);
                c.g[T0][0] = sx(OC1065_PROGS);
                c.g[T1][0] = sx(OC1065_PATCH);
                c.g[T2][0] = sx(OC1065_HANDLE);
            },
        );
    }

    #[test]
    fn ps2_bezier_man_free_caches_matches() {
        oc1071_hook();
        check(
            ee::s12::f_0038b0f8__generated,
            ps2_bezier_man_free_caches_2,
            200,
            &|c, seed| {
                let mut r = Rng(0x0038_b0f8 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC1071_GATE, OC1071_MAGIC);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                // Each of the three blocks is set or clear, by the seed's bits.
                for (i, off) in [0x440u32, 0x444, 0x448].iter().enumerate() {
                    let block = if (seed >> i) & 1 == 1 {
                        DATA + 0x5000 + i as u32 * 0x100
                    } else {
                        0
                    };
                    c.bus.write32(this + off, block);
                }
            },
        );
    }

    #[test]
    fn ps2_bezier_man_free_caches_stub_matches() {
        check(
            ssx3_ee::s12::f_0038d660__generated,
            ps2_bezier_man_free_caches_stub,
            200,
            &|c, seed| {
                let mut r = Rng(0x38d6_6000 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    /// `PS2BezierMan_EmitPatchProjectorPasses`: 0..4 lights with random bit
    /// masks, the program table entry flag 0..3, the draw ring below and at its
    /// 2600-entry limit (no ring entry), and an empty light count.
    #[test]
    fn ps2_bezier_man_emit_patch_projector_passes_matches() {
        const GM: u32 = DATA + 0x3_0000;
        const TOP: u32 = DATA + 0x3_4000;
        const TABLE: u32 = DATA + 0x3_8000;
        const ENTRY: u32 = DATA + 0x3_9000;
        const HANDLE: u32 = DATA + 0x2_2000;
        const RING: u32 = DATA + 0x4_0000;
        check(
            ee::s12::f_0038d448__generated,
            ps2_bezier_man_emit_patch_projector_passes,
            300,
            &|c, seed| {
                let mut r = Rng(0x0038_d448 ^ (seed as u64) << 11);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP + 0x2a90, GM);
                fill(c, &mut r, GM + 0x13ec, 0xc0);
                c.bus.write32(
                    GM + 0x13ec,
                    if seed % 9 == 4 { 0 } else { r.next_u32() % 5 },
                );
                c.bus.write32(GM + 0xe84, TOP);
                c.bus.write32(GM + 0x18f0, RING);
                fill(c, &mut r, TOP, 8);
                c.bus.write32(
                    RING,
                    if seed % 7 == 3 {
                        2600
                    } else {
                        r.next_u32() % 4
                    },
                );
                c.bus.write32(RING + 0x69cc0, r.next_u32());
                c.bus.write32(RING + 0x69cc4, r.next_u32());
                fill(c, &mut r, TABLE, 0x80);
                c.bus.write32(ENTRY + 4, r.next_u32() % 4);
                c.bus
                    .write32(HANDLE, DATA + 0x3_c000 + (r.next_u32() & 0xff0));
                c.g[A2][0] = TABLE as u64;
                c.g[A3][0] = ENTRY as u64;
                c.g[T0][0] = (if seed % 5 == 0 { 0xf } else { r.next_u32() }) as u64;
                c.g[T1][0] = HANDLE as u64;
            },
        );
    }
}
