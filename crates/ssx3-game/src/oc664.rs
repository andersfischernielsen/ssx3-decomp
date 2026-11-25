//! Staging module oc664 (wave g44.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc664`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_9270, anim_delta_ctor_2, "AnimDelta_ctor_2"),
];

/// `cMemMan_alloc` (0x317d70, shard s10): called through the function table.
const OC664_ALLOC: u32 = 0x0031_7d70;
/// The UV scroll and tex flip modifiers (shard s12): called through the table.
const OC664_UV_SCROLL_CTOR: u32 = 0x0035_f788;
const OC664_TEX_FLIP_CTOR: u32 = 0x0035_f1a8;

/// Constructors, appenders and helpers the bodies call by name (shard s11).
const MB_SPLINE_CTOR: Callee = Callee {
    addr: 0x0035_95d8,
    f: ee::s11::f_003595d8,
};
const MB_MULTI_SPLINE_CTOR: Callee = Callee {
    addr: 0x0035_a118,
    f: ee::s11::f_0035a118,
};
const MB_PARENT_CTOR: Callee = Callee {
    addr: 0x0035_7090,
    f: ee::s11::f_00357090,
};
const MB_POSITION_CTOR: Callee = Callee {
    addr: 0x0035_6f68,
    f: ee::s11::f_00356f68,
};
const MB_ROLLER_CTOR: Callee = Callee {
    addr: 0x0035_df70,
    f: ee::s11::f_0035df70,
};
const MB_MAGNET_CTOR: Callee = Callee {
    addr: 0x0035_7358,
    f: ee::s11::f_00357358,
};
const MB_AVA_SPLINE_CTOR: Callee = Callee {
    addr: 0x0035_7798,
    f: ee::s11::f_00357798,
};
const MB_BOOST_CTOR: Callee = Callee {
    addr: 0x0034_2808,
    f: ee::s11::f_00342808,
};
const MB_PARTICLE_CTOR: Callee = Callee {
    addr: 0x0034_59a8,
    f: ee::s11::f_003459a8,
};
const MB_DYNAMIC_PARTICLE_CTOR: Callee = Callee {
    addr: 0x0034_5d80,
    f: ee::s11::f_00345d80,
};
const MB_HALO_CTOR: Callee = Callee {
    addr: 0x0034_61c0,
    f: ee::s11::f_003461c0,
};
const MB_EFFECT_APPEND: Callee = Callee {
    addr: 0x0034_5720,
    f: ee::s11::f_00345720,
};
const MB_RAIL_CTOR: Callee = Callee {
    addr: 0x0035_ba88,
    f: ee::s11::f_0035ba88,
};
const MB_RAIL_APPEND: Callee = Callee {
    addr: 0x0035_b690,
    f: ee::s11::f_0035b690,
};
const MOBJ_WORLD_CTOR: Callee = Callee {
    addr: 0x0034_fb90,
    f: ee::s11::f_0034fb90,
};
const MOBJ_CREATE_BLOCK: Callee = Callee {
    addr: 0x0035_5468,
    f: ee::s11::f_00355468,
};
const MOBJ_MODIFIER_BLOCK_LOAD: Callee = Callee {
    addr: 0x0035_36d8,
    f: ee::s11::f_003536d8,
};
const MOBJ_SETUP_OVERLAP: Callee = Callee {
    addr: 0x0035_6780,
    f: ee::s11::f_00356780,
};
const ANIMD_OBJECT_CTOR: Callee = Callee {
    addr: 0x0034_1c80,
    f: ee::s11::f_00341c80,
};
const ANIMD_OBJECT_V15: Callee = Callee {
    addr: 0x0034_1d48,
    f: ee::s11::f_00341d48,
};

/// One arm of the `hdr.type` chain: compares V1 with `kind` (V0 holds it),
/// leaves `next` in V0, and on a match allocates `size` bytes named
/// `0x490000 - name` and constructs the modifier with `ctor`.
#[derive(Clone, Copy)]
struct Oc664RootArm {
    kind: u64,
    next: u64,
    entry: u64,
    size: u64,
    name: u32,
    alloc_back: u32,
    ctor: Callee,
    ctor_back: u32,
}

const OC664_ROOT_ARMS: [Oc664RootArm; 6] = [
    Oc664RootArm {
        kind: 1,
        next: 2,
        entry: 3,
        size: 0xf0,
        name: 0x1730,
        alloc_back: 0x0035_376c,
        ctor: MB_SPLINE_CTOR,
        ctor_back: 0x0035_3778,
    },
    Oc664RootArm {
        kind: 2,
        next: 4,
        entry: 2,
        size: 0x58,
        name: 0x1730,
        alloc_back: 0x0035_37a0,
        ctor: MB_MULTI_SPLINE_CTOR,
        ctor_back: 0x0035_37ac,
    },
    Oc664RootArm {
        kind: 4,
        next: 3,
        entry: 2,
        size: 0xb0,
        name: 0x1720,
        alloc_back: 0x0035_37d4,
        ctor: MB_PARENT_CTOR,
        ctor_back: 0x0035_37e0,
    },
    Oc664RootArm {
        kind: 3,
        next: 5,
        entry: 2,
        size: 0x90,
        name: 0x1710,
        alloc_back: 0x0035_3808,
        ctor: MB_POSITION_CTOR,
        ctor_back: 0x0035_3814,
    },
    Oc664RootArm {
        kind: 5,
        next: 6,
        entry: 2,
        size: 0x2d0,
        name: 0x16f8,
        alloc_back: 0x0035_383c,
        ctor: MB_ROLLER_CTOR,
        ctor_back: 0x0035_3848,
    },
    Oc664RootArm {
        kind: 6,
        next: 7,
        entry: 2,
        size: 0xb0,
        name: 0x16e8,
        alloc_back: 0x0035_3870,
        ctor: MB_MAGNET_CTOR,
        ctor_back: 0x0035_387c,
    },
];

/// `cMemMan_alloc(size, str, 0x20000000, 0)` where the name string sits at
/// `0x490000 - name_off`; returns to `back` with v0 = the block.
fn oc664_alloc(c: &mut Cpu, size: u64, name_off: u32, back: u32) -> Flow {
    c.g[A0][0] = size;
    c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(name_off));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, OC664_ALLOC, back)
}

/// `stream->read(a1, a2)` through the stream's vtable entry 2 (`{short delta;
/// fn}` at +0x10). The vtable pointer goes to register `vt`, the method to
/// `method`, and a0 = stream + delta; the caller sets a1 and a2 first. Which
/// registers hold them differs per call site in the generated code.
fn oc664_stream_read(c: &mut Cpu, stream: u32, vt: usize, method: usize, back: u32) -> Flow {
    c.g[vt][0] = Addr(stream).lw(c);
    let table = c.g[vt][0] as u32;
    c.g[A0][0] = Addr(table.wrapping_add(0x10)).lh(c);
    c.g[method][0] = Addr(table.wrapping_add(0x14)).lw(c);
    let target = c.g[method][0] as u32;
    c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, back)
}

/// One arm of the root-transform chain (`'b_353780` ... `'b_353884`). Returns
/// true when the type matched (the arm built its modifier).
fn oc664_root_arm(c: &mut Cpu, arm: Oc664RootArm) -> Result<bool, Unwound> {
    c.cycles += arm.entry;
    c.g[V0][0] = arm.kind;
    let mismatch = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = arm.next;
    if mismatch {
        return Ok(false);
    }
    c.cycles += 11;
    oc664_alloc(c, arm.size, arm.name, arm.alloc_back)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    arm.ctor.call(c, arm.ctor_back)?;
    let made = c.g[V0][0];
    Addr(c.g[S2][0] as u32).sw(c, made);
    Ok(true)
}

/// The root transform by `hdr.type` (V1), for `flags & 1`. Returns true when
/// the body goes on to its tail: a match, or type 7 is not taken and the tail
/// is skipped (false) when no arm matched.
fn oc664_root_transform(c: &mut Cpu) -> Result<bool, Unwound> {
    for arm in OC664_ROOT_ARMS {
        if oc664_root_arm(c, arm)? {
            return Ok(true);
        }
    }
    // Type 7 (`'b_353884`'s successor): the flags word is reloaded before the
    // branch, so the constructor call sees it in v0.
    let hdr = sp(c);
    c.cycles += 2;
    let mismatch = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = Addr(hdr).lw(c);
    if mismatch {
        return Ok(false);
    }
    c.cycles += 10;
    oc664_alloc(c, 0x90, 0x16d8, 0x0035_38a4)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    MB_AVA_SPLINE_CTOR.call(c, 0x0035_38b0)?;
    let made = c.g[V0][0];
    Addr(c.g[S2][0] as u32).sw(c, made);
    Ok(true)
}

/// One numA effect (`'b_353a1c`): the effect type is in v0 (a byte address
/// on entry); a1 leaves with the effect (or 0) for the link.
fn oc664_effect(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[V1][0] = 1;
    c.g[V0][0] = Addr(c.g[V0][0] as u32).lb(c);
    let mismatch = c.g[V0][0] != c.g[V1][0];
    c.g[A0][0] = 0x1f0;
    if !mismatch {
        c.cycles += 9;
        oc664_alloc(c, 0x1f0, 0x1688, 0x0035_39a8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        MB_PARTICLE_CTOR.call(c, 0x0035_39b4)?;
        c.g[A1][0] = c.g[V0][0];
        return Ok(());
    }
    c.cycles += 2;
    let mismatch = c.g[V0][0] != c.g[S7][0];
    c.g[A0][0] = 0x270;
    if !mismatch {
        c.cycles += 9;
        oc664_alloc(c, 0x270, 0x1670, 0x0035_39d4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        MB_DYNAMIC_PARTICLE_CTOR.call(c, 0x0035_39e0)?;
        c.g[A1][0] = c.g[V0][0];
        return Ok(());
    }
    c.cycles += 2;
    let mismatch = c.g[V0][0] != c.g[S5][0];
    c.g[V0][0] = 0;
    if !mismatch {
        c.cycles += 10;
        c.g[A0][0] = 0x40;
        oc664_alloc(c, 0x40, 0x1658, 0x0035_3a04)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        MB_HALO_CTOR.call(c, 0x0035_3a10)?;
        c.g[A1][0] = c.g[V0][0];
        return Ok(());
    }
    c.cycles += 1;
    c.g[A1][0] = c.g[V0][0];
    Ok(())
}

/// `void tModifierBlock_readFromReplayFrame(ModifierBlock *self, Stream *stream)`
/// (0x3536d8): reads the 52-byte block header (flags +0, transform type +4,
/// kind +8, numA +0xc, numB +0x10, effect types from +0x14), clears the block's
/// four slots, then builds what the flags ask for: the root transform
/// (`flags & 1`, by type), the UV scroll (2), the tex flip (4), the boost
/// (8, when kind is 1), the numA effects (linked at +0x10) and the numB rails
/// (linked at +0x1c). Frame 0xe0: saves ra, s0-s7 and fp.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn modifier_block_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0xe0);
        c.g[A2][0] = 0x34;
        frame.save(c, 18, 0xb0);
        frame.save(c, 17, 0xc0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 16, 0xd0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 19, 0xa0);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, 20, 0x90);
        frame.save(c, 21, 0x80);
        frame.save(c, 22, 0x70);
        frame.save(c, 23, 0x60);
        frame.save(c, 30, 0x50);
        frame.save_ra(c, 0x40);
        let block = c.g[S2][0] as u32;
        let stream = c.g[S1][0] as u32;
        for off in [0u32, 4, 8, 0xc] {
            Addr(block + off).sw(c, 0);
        }
        oc664_stream_read(c, stream, V0, V1, 0x0035_3738)?;

        // flags & 1: the root transform, by hdr.type (V1).
        let hdr = sp(c);
        c.g[V0][0] = Addr(hdr).lw(c);
        c.g[V0][0] &= 1;
        let no_root = c.g[V0][0] == 0;
        c.g[V1][0] = Addr(hdr + 4).lw(c);
        let mut tail = true;
        if !no_root {
            tail = oc664_root_transform(c)?;
        }
        if tail {
            c.cycles += 1;
        }
        c.g[V0][0] = Addr(hdr).lw(c);

        // flags & 2: UV scroll.
        c.cycles += 3;
        c.g[V0][0] &= 2;
        let no_uv = c.g[V0][0] == 0;
        c.g[A0][0] = 0x50;
        if !no_uv {
            c.cycles += 9;
            oc664_alloc(c, 0x50, 0x16c0, 0x0035_38d8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, OC664_UV_SCROLL_CTOR, 0x0035_38e4)?;
            let made = c.g[V0][0];
            Addr(block + 4).sw(c, made);
        }

        // flags & 4: tex flip.
        c.cycles += 4;
        c.g[V0][0] = Addr(hdr).lw(c);
        c.g[V0][0] &= 4;
        let no_flip = c.g[V0][0] == 0;
        c.g[A0][0] = 0xe8;
        if !no_flip {
            c.cycles += 9;
            oc664_alloc(c, 0xe8, 0x16a8, 0x0035_390c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, OC664_TEX_FLIP_CTOR, 0x0035_3918)?;
            let made = c.g[V0][0];
            Addr(block + 8).sw(c, made);
        }

        // flags & 8 and kind == 1: the boost (physics) modifier.
        c.cycles += 4;
        c.g[V0][0] = Addr(hdr).lw(c) & 8;
        let no_boost_flag = c.g[V0][0] == 0;
        c.g[V1][0] = Addr(hdr + 8).lw(c);
        let mut tail = true;
        if !no_boost_flag {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let not_boost = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = Addr(hdr + 0xc).lw(c);
            if not_boost {
                tail = false;
            } else {
                c.cycles += 10;
                oc664_alloc(c, 0x50, 0x1698, 0x0035_3950)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                MB_BOOST_CTOR.call(c, 0x0035_395c)?;
                let made = c.g[V0][0];
                Addr(block + 0xc).sw(c, made);
            }
        }
        if tail {
            c.cycles += 1;
        }
        c.g[V0][0] = Addr(hdr + 0xc).lw(c);

        // numA effects, each linked into the list at block+0x10.
        c.cycles += 2;
        let effects_wanted = (c.g[V0][0] as i64) > 0;
        c.g[S0][0] = 0;
        if effects_wanted {
            c.cycles += 7;
            c.g[S3][0] = sx(hdr.wrapping_add(0x14));
            c.g[FP][0] = 0x0049_0000;
            c.g[S7][0] = 2;
            c.g[S6][0] = 0x0049_0000;
            c.g[S5][0] = 3;
            c.g[S4][0] = 0x0049_0000;
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
            loop {
                oc664_effect(c)?;
                c.cycles += 7;
                c.g[A0][0] = sx(block.wrapping_add(0x10));
                MB_EFFECT_APPEND.call(c, 0x0035_3a24)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(hdr + 0xc).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_3988);
                }
            }
        }

        // numB rails, each built with a 0x90-byte name and linked at block+0x1c.
        c.cycles += 3;
        c.g[V0][0] = Addr(hdr + 0x10).lw(c);
        let rails_wanted = (c.g[V0][0] as i64) > 0;
        c.g[S0][0] = 0;
        if rails_wanted {
            c.cycles += 3;
            c.g[S3][0] = 0x0049_0000;
            c.g[A2][0] = 0x2000_0000;
            loop {
                c.cycles += 15;
                c.g[A3][0] = 0;
                c.g[A0][0] = 0x90;
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1648));
                call_ptr(c, OC664_ALLOC, 0x0035_3a60)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                MB_RAIL_CTOR.call(c, 0x0035_3a70)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = sx(block.wrapping_add(0x1c));
                MB_RAIL_APPEND.call(c, 0x0035_3a7c)?;
                c.g[V0][0] = Addr(hdr + 0x10).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = 0x2000_0000;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_3a50);
                }
            }
        }

        c.cycles += 12;
        frame.restore(c, 16, 0xd0);
        frame.restore(c, 17, 0xc0);
        frame.restore(c, 18, 0xb0);
        frame.restore(c, 19, 0xa0);
        frame.restore(c, 20, 0x90);
        frame.restore(c, 21, 0x80);
        frame.restore(c, 22, 0x70);
        frame.restore(c, 23, 0x60);
        frame.restore(c, 30, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `cMoveNode *cMoveNode_cMoveNode(cMoveNode *self, int bucket, Stream *stream)`
/// (0x3552d8): the instance node base, the move-node vtable, four stream reads
/// (debounce, fade, transfer hack, and the has-block flag at SP+0), then the
/// modifier block when the flag is set, the overlap system setup, and
/// `Flags |= 1`. Frame 0x40. Returns self. Each read uses its own vtable
/// registers, as the generated code does.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn mod_obj_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 42;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A2][0];
        MOBJ_WORLD_CTOR.call(c, 0x0035_52f4)?;

        let this = c.g[S0][0] as u32;
        let stream = c.g[S1][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        Addr(this + 0x1c).sw(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1028));
        c.g[A1][0] = sx(this.wrapping_add(0x20));
        let vtable = c.g[V1][0];
        Addr(this + 0xc).sw(c, vtable);
        c.g[A2][0] = 4;
        oc664_stream_read(c, stream, V0, V1, 0x0035_5320)?;

        c.g[A1][0] = sx(this.wrapping_add(0x24));
        c.g[A2][0] = 4;
        oc664_stream_read(c, stream, A3, V0, 0x0035_533c)?;

        c.g[A1][0] = sx(this.wrapping_add(0x28));
        c.g[A2][0] = 4;
        oc664_stream_read(c, stream, V1, V0, 0x0035_5358)?;

        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        oc664_stream_read(c, stream, A3, V1, 0x0035_5374)?;

        c.g[V0][0] = Addr(sp(c)).lw(c);
        let has_block = c.g[V0][0] != 0;
        if has_block {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            MOBJ_CREATE_BLOCK.call(c, 0x0035_5388)?;
            c.g[A0][0] = Addr(this + 0x1c).lw(c);
            c.g[A1][0] = c.g[S1][0];
            MOBJ_MODIFIER_BLOCK_LOAD.call(c, 0x0035_5394)?;
        }

        c.cycles += 11;
        c.g[A0][0] = c.g[S0][0];
        MOBJ_SETUP_OVERLAP.call(c, 0x0035_539c)?;
        c.g[V1][0] = Addr(this + 0x12).lhu(c);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0x20);
        c.g[V1][0] |= 1;
        frame.restore_ra(c, 0x10);
        let flags = c.g[V1][0];
        Addr(this + 0x12).sh(c, flags);
        frame.restore(c, 16, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `cAnimDeltaNode *cAnimDeltaNode_cAnimDeltaNode(self, int bucket, Stream *stream)`
/// (0x359270): the anim object base, the delta-node vtable, then the float
/// delta (+0x6c) read from the stream. Frame 0x30. Returns self.
pub fn anim_delta_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        ANIMD_OBJECT_CTOR.call(c, 0x0035_928c)?;

        let this = c.g[S0][0] as u32;
        let stream = c.g[S1][0] as u32;
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = sx(this.wrapping_add(0x6c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1590));
        c.g[A2][0] = 4;
        let vtable = c.g[V1][0];
        Addr(this + 0x3c).sw(c, vtable);
        oc664_stream_read(c, stream, V0, V1, 0x0035_92b4)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0x10);
        frame.restore(c, 16, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Flags path of the update: `flags |= 1`, then return 1 (the tail's `+1`).
fn oc664_set_flag_return(c: &mut Cpu, this: u32) -> u64 {
    c.cycles += 2;
    let flags = Addr(this + 0x42).lhu(c) | 1;
    Addr(this + 0x42).sh(c, flags);
    c.cycles += 1;
    1
}

/// The update of a node that is not complete (`AnimComplete == 0`): returns
/// v0 for the epilogue and charges the cycles of the path it takes.
fn oc664_anim_update(c: &mut Cpu, this: u32) -> Result<u64, Unwound> {
    // Frame-end marker is already stored. f1 = Delta, f2 = 0.0, fcc = 0 < Delta.
    c.cycles += 6;
    let delta = Addr(this + 0x6c).read_fpu(c);
    abi::set_fpr(c, 1, delta);
    abi::set_fpr(c, 2, FpuF32(0));
    let positive = abi::c_lt_s(c, FpuF32(0), delta);
    c.g[V0][0] = Addr(this + 0xc).lw(c);
    if !positive {
        // Idle node: the countdown steps down, then the flag path.
        c.cycles += 2;
        let delay = c.g[V0][0] as i64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if delay > 0 {
            c.cycles += 1;
            let left = c.g[V0][0];
            Addr(this + 0xc).sw(c, left);
        }
        c.cycles += 3;
        c.g[A2][0] = sx(this.wrapping_add(0x30));
        c.g[V0][0] = Addr(this + 8).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = Addr(this + 0x42).lhu(c);
            return Ok(oc664_set_flag_return(c, this));
        }
        // AnimComplete changed (unreachable here: nothing wrote it): the vtable call.
        c.cycles += 8;
        c.g[V1][0] = Addr(this + 0x3c).lw(c);
        c.g[A1][0] = 1;
        let table = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(table + 0x110).lh(c);
        c.g[V0][0] = Addr(table + 0x114).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_93a4)?;
        return Ok(0);
    }

    // Moving node: countdown >= 0 is required for an update.
    let delay = c.g[V0][0] as i64;
    c.cycles += 2;
    if delay < 0 {
        // Countdown negative: the flag path.
        return Ok(oc664_set_flag_return(c, this));
    }
    if delay == 0 {
        c.cycles += 2;
        if c.fcc() {
            // Delta still running: step it by the speed (never below zero).
            c.cycles += 7;
            let speed = Addr(this + 0x10).read_fpu(c);
            abi::set_fpr(c, 0, speed.abs());
            let stepped = FpuF32(c.f[1]) - FpuF32(c.f[0]);
            abi::set_fpr(c, 0, stepped);
            let below_zero = abi::c_lt_s(c, stepped, FpuF32(c.f[2]));
            Addr(this + 0x6c).write_fpu(c, stepped);
            if below_zero {
                c.cycles += 1;
                Addr(this + 0x6c).write_fpu(c, FpuF32(c.f[2]));
            }
        }
    }

    // The update: the object's per-frame method, then AnimActive from the new Delta.
    c.cycles += 8;
    c.g[A0][0] = c.g[S0][0];
    ANIMD_OBJECT_V15.call(c, 0x0035_934c)?;
    let after = Addr(this + 0x6c).read_fpu(c);
    abi::set_fpr(c, 1, after);
    abi::set_fpr(c, 0, FpuF32(0));
    let moving = abi::c_lt_s(c, FpuF32(0), after);
    c.g[V0][0] = 1;
    let one = c.g[V0][0];
    if moving {
        c.cycles += 2;
        Addr(this + 4).sw(c, one);
        return Ok(1);
    }
    c.cycles += 2;
    Addr(this + 4).sw(c, 0);
    c.cycles += 1;
    Ok(1)
}

/// `void cAnimDeltaNode_updateNode(cAnimDeltaNode *self)` (0x3592d0), returns
/// int: a complete node returns 1 at once; otherwise the frame ends are set to
/// 1e10, the countdown and delta step, and the object updates. Frame 0x20.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn anim_delta_v15(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        let no_frame = Addr((c.g[GP][0] as u32).wrapping_sub(0x2a6c)).read_fpu(c);
        abi::set_fpr(c, 0, no_frame);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        Addr(this + 0x28).write_fpu(c, no_frame);
        c.g[V0][0] = Addr(this + 8).lw(c);
        let live = c.g[V0][0] == 0;
        Addr(this + 0x24).write_fpu(c, no_frame);
        let v0 = if live {
            oc664_anim_update(c, this)?
        } else {
            c.cycles += 1;
            1
        };
        c.g[V0][0] = v0;
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake_call, fall_back, hook, machine, prev_cell, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word for the callee fakes (a magic word in the machine's RAM).
    const OC664_CTRL: u32 = DATA + 0x3f00;
    const OC664_MAGIC: u32 = 0x6f63_3634;
    /// The stream's read method: outside the function table, so it reaches
    /// the machine's miss hook (`oc664_miss`).
    const OC664_READ: u32 = FAKE + 0x800;
    /// The read cursor, the stream object (its vtable at +0) and its vtable.
    const OC664_CURSOR: u32 = DATA + 0x3f40;
    const OC664_STREAM: u32 = DATA + 0x3f80;
    const OC664_VT: u32 = DATA + 0x3fc0;
    /// The bytes the reads consume.
    const OC664_BYTES: u32 = DATA + 0x3400;
    /// The block the gated `addModifierBlock` fake hands to a move node.
    const OC664_BLOCK: u32 = DATA + 0x3800;
    /// The objects under test.
    const OC664_OBJ: u32 = DATA + 0x3000;
    const OC664_NODE: u32 = DATA + 0x3a00;
    const OC664_ANIM: u32 = DATA + 0x3c00;
    /// A vtable whose slot at +0x110 is a fake (the dead branch of the update).
    const OC664_VT2: u32 = DATA + 0x3d00;

    /// The stream's read: copies the next `a2` bytes of the stream to `a1`.
    /// Other misses are plain fake calls.
    fn oc664_miss(c: &mut Cpu, target: u32) {
        if target != OC664_READ {
            return fake_call(c, target);
        }
        let dst = c.g[A1][0] as u32;
        let len = c.g[A2][0] as u32;
        let at = c.bus.read32(OC664_CURSOR);
        for i in 0..len {
            let b = c.bus.read8(OC664_BYTES + at + i);
            c.bus.write8(dst + i, b);
        }
        c.bus.write32(OC664_CURSOR, at + len);
        c.pc = c.g[RA][0] as u32;
    }

    /// Gate on, the miss hook, and the stream object with its read vtable.
    fn oc664_gate(c: &mut Cpu) {
        c.hooks.miss = oc664_miss;
        c.bus.write32(OC664_CTRL, OC664_MAGIC);
        c.bus.write32(OC664_CURSOR, 0);
        c.bus.write32(OC664_STREAM, OC664_VT);
        c.bus.write16(OC664_VT + 0x10, 0);
        c.bus.write32(OC664_VT + 0x14, OC664_READ);
    }

    /// A 52-byte modifier-block header at `at`: flags (the low nibble of the
    /// seed), transform type (0..8), kind (1 or 2), numA and numB, effect types.
    fn oc664_header(c: &mut Cpu, at: u32, seed: u32, r: &mut Rng) {
        c.bus.write32(at, seed & 0xf);
        c.bus.write32(at + 4, (seed >> 4) % 9);
        c.bus.write32(at + 8, if seed % 5 == 0 { 2 } else { 1 });
        c.bus.write32(at + 0xc, r.next_u32() % 4);
        c.bus.write32(at + 0x10, r.next_u32() % 3);
        for i in 0..4 {
            c.bus.write8(at + 0x14 + i, (r.next_u32() % 4) as u8);
        }
    }

    /// Gated fakes for the callees that have an override slot: while the gate
    /// is set, a call is logged and returns; otherwise the body it replaced runs.
    macro_rules! oc664_callees {
        ($($fake:ident => $shard:ident :: $slot:ident, $gen:ident, $addr:expr;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC664_CTRL) != OC664_MAGIC {
                        return fall_back(c, prev_cell($fake), ee::$shard::$gen);
                    }
                    fake_call(c, $addr);
                }
            )*

            fn oc664_install_slots() {
                $(hook(addr_of_mut!(ee::$shard::$slot), $fake, prev_cell($fake));)*
            }
        };
    }

    oc664_callees! {
        oc664_fake_alloc => s10::OVERRIDE_00317D70, f_00317d70__generated, 0x0031_7d70;
        oc664_fake_spline => s11::OVERRIDE_003595D8, f_003595d8__generated, 0x0035_95d8;
        oc664_fake_multi_spline => s11::OVERRIDE_0035A118, f_0035a118__generated, 0x0035_a118;
        oc664_fake_parent => s11::OVERRIDE_00357090, f_00357090__generated, 0x0035_7090;
        oc664_fake_position => s11::OVERRIDE_00356F68, f_00356f68__generated, 0x0035_6f68;
        oc664_fake_magnet => s11::OVERRIDE_00357358, f_00357358__generated, 0x0035_7358;
        oc664_fake_ava => s11::OVERRIDE_00357798, f_00357798__generated, 0x0035_7798;
        oc664_fake_uv => s12::OVERRIDE_0035F788, f_0035f788__generated, 0x0035_f788;
        oc664_fake_tex_flip => s12::OVERRIDE_0035F1A8, f_0035f1a8__generated, 0x0035_f1a8;
        oc664_fake_boost => s11::OVERRIDE_00342808, f_00342808__generated, 0x0034_2808;
        oc664_fake_particle => s11::OVERRIDE_003459A8, f_003459a8__generated, 0x0034_59a8;
        oc664_fake_dyn_particle => s11::OVERRIDE_00345D80, f_00345d80__generated, 0x0034_5d80;
        oc664_fake_halo => s11::OVERRIDE_003461C0, f_003461c0__generated, 0x0034_61c0;
        oc664_fake_effect => s11::OVERRIDE_00345720, f_00345720__generated, 0x0034_5720;
        oc664_fake_rail_link => s11::OVERRIDE_0035B690, f_0035b690__generated, 0x0035_b690;
        oc664_fake_world => s11::OVERRIDE_0034FB90, f_0034fb90__generated, 0x0034_fb90;
        oc664_fake_setup_overlap => s11::OVERRIDE_00356780, f_00356780__generated, 0x0035_6780;
        oc664_fake_anim_ctor => s11::OVERRIDE_00341C80, f_00341c80__generated, 0x0034_1c80;
        oc664_fake_anim_v15 => s11::OVERRIDE_00341D48, f_00341d48__generated, 0x0034_1d48;
    }

    /// `addModifierBlock`: the node's block pointer (+0x1c) is set, then the call is logged.
    fn oc664_fake_block(c: &mut Cpu) {
        if c.bus.read32(OC664_CTRL) != OC664_MAGIC {
            return fall_back(c, prev_cell(oc664_fake_block), ee::s11::f_00355468__generated);
        }
        let node = c.g[A0][0] as u32;
        c.bus.write32(node + 0x1c, OC664_BLOCK);
        fake_call(c, 0x0035_5468);
    }

    fn oc664_install() {
        oc664_install_slots();
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_00355468),
            oc664_fake_block,
            prev_cell(oc664_fake_block),
        );
    }





    #[test]
    fn anim_delta_ctor_2_matches() {
        oc664_install();
        check(ee::s11::f_00359270__generated, anim_delta_ctor_2, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3592_70, 2);
            oc664_gate(c);
            c.bus.write32(OC664_BYTES, r.next_u32());
            c.g[A0][0] = OC664_ANIM as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = OC664_STREAM as u64;
        });
    }


}
