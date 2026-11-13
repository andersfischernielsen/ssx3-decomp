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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

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
}
