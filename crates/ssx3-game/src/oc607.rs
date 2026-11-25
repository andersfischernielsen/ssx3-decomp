//! Staging module oc607 (wave l592b.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc607`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0036_3c20,
    defer_man_emit_state_change,
    "DeferMan_EmitStateChange",
)];

/// The callees of the state emitter, each reached through its override slot.
const CHG_MATERIAL_SETUP: Callee = Callee {
    addr: 0x0036_4cd0,
    f: ee::s12::DeferMan_EmitMaterialSetup,
};
const CHG_SCISSOR: Callee = Callee {
    addr: 0x0036_23d8,
    f: ee::s12::GsPkt_AddScissor,
};
const CHG_ZBUF: Callee = Callee {
    addr: 0x0036_2660,
    f: ee::s12::GsPkt_AddZbuf,
};
const CHG_TEST: Callee = Callee {
    addr: 0x0036_26d8,
    f: ee::s12::GsPkt_AddTest,
};
const CHG_CLAMP: Callee = Callee {
    addr: 0x0036_25c0,
    f: ee::s12::GsPkt_AddClamp,
};
const CHG_ALPHA: Callee = Callee {
    addr: 0x0036_2478,
    f: ee::s12::GsPkt_AddAlpha,
};
const CHG_FINISH: Callee = Callee {
    addr: 0x0036_27a8,
    f: ee::s12::GsPkt_Finish,
};

/// `void DeferMan_EmitStateChange(DeferMan* self, u32* state, int layer,
/// void** pp)` (0x363c20): writes the packets that take the GS state from the
/// last emitted one (the context block at `self + 0x68000`) to `state`. The
/// material setup comes first (mode 0 when the material changed, mode 1 when
/// only the layer changed, mode 2 when the key word changed), then one
/// scissor, zbuffer, test, clamp and alpha packet for each field that
/// differs, then the finish packet. `state` becomes the last state.
pub fn defer_man_emit_state_change(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x80);
        c.cycles += 32;
        c.g[V1][0] = 0x1000_0001;
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S0, 0x70);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S2, 0x50);
        c.g[A3][0] = 0x8000u64 << 45;
        frame.save_ra(c, 0x10);

        // The packet header: the GIF tag 0x10000001 at the write pointer, two
        // zero quadwords, the A+D tag 0x1000000000000000 at +0x18, and the
        // write pointer moved by 0x20.
        c.g[V0][0] = c.lw(r32(c, S5));
        c.sd(r32(c, V0), c.g[V1][0]);
        c.g[V0][0] = c.lw(r32(c, S5));
        c.sd(r32(c, V0).wrapping_add(0x8), 0);
        c.g[V1][0] = c.lw(r32(c, S5));
        c.sd(r32(c, V1).wrapping_add(0x10), 0);
        c.g[A0][0] = c.lw(r32(c, S5));
        c.sd(r32(c, A0).wrapping_add(0x18), c.g[A3][0]);
        c.g[V0][0] = c.lw(r32(c, S5));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x20));
        c.sw(r32(c, S5), c.g[V0][0]);

        material_section(c)?;
        scissor_section(c)?;
        zbuf_section(c)?;
        test_section(c)?;
        clamp_wrap_s(c)?;
        alpha_wrap_7c(c)?;
        clamp_wrap_t(c)?;
        alpha_wrap_f80(c)?;

        // The last state becomes `state` (a 128-bit unaligned copy of its
        // first quadword, then the words at +0x10), then the finish packet.
        c.cycles += 25;
        c.g[A1][0] = c.g[S5][0];
        c.g[V0][0] = c.g[V0][0] | 0x8000;
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        c.ldl(V1, r32(c, S3).wrapping_add(0x7));
        c.ldr(V1, r32(c, S3));
        c.ldl(A3, r32(c, S3).wrapping_add(0xf));
        c.ldr(A3, r32(c, S3).wrapping_add(0x8));
        c.g[T0][0] = c.lw(r32(c, S3).wrapping_add(0x10));
        c.sdl(V1, r32(c, V0).wrapping_add(0x1caf));
        c.sdr(V1, r32(c, V0).wrapping_add(0x1ca8));
        c.sdl(A3, r32(c, V0).wrapping_add(0x1cb7));
        c.sdr(A3, r32(c, V0).wrapping_add(0x1cb0));
        c.sw(r32(c, V0).wrapping_add(0x1cb8), c.g[T0][0]);
        c.g[A2][0] = 0x1;
        CHG_FINISH.call(c, 0x0036_402c)?;

        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The material block: the material index is bits 6..9 of the state's first
/// word. Nothing happens for material 0. Otherwise the last material key
/// (`+0x69cbc`) and layer (`+0x69cc0`) decide which of the three setup modes
/// runs.
fn material_section(c: &mut Cpu) -> Flow {
    'b_363d7c: {
        c.g[V1][0] = c.lw(r32(c, S3));
        c.g[V1][0] = c.g[V1][0] & 0x3c0;
        c.g[A2][0] = sx(r32(c, V1) >> 6);
        let t = c.g[A2][0] == 0u64;
        c.g[V1][0] = 0x6_0000;
        if t {
            break 'b_363d7c;
        }
        'b_363d44: {
            'b_363d04: {
                c.cycles += 10;
                c.g[A0][0] = 0x3fff_0000;
                c.g[V1][0] = c.g[V1][0] | 0x9ca8;
                c.g[A1][0] = c.lw(r32(c, S3).wrapping_add(0xc));
                c.g[V1][0] = sx(r32(c, S4).wrapping_add(r32(c, V1)));
                c.g[A0][0] = c.g[A0][0] | 0xffff;
                c.g[V0][0] = c.lw(r32(c, V1));
                c.g[V0][0] = c.g[V0][0] & 0x3c0;
                c.g[V0][0] = sx(r32(c, V0) >> 6);
                let t = c.g[V0][0] == c.g[A2][0];
                c.g[S0][0] = c.g[A1][0] & c.g[A0][0];
                if t {
                    break 'b_363d04;
                }
                // Mode 0: the material changed.
                c.cycles += 15;
                c.g[V0][0] = 0x6_0000;
                c.g[A0][0] = c.g[S4][0];
                c.g[V0][0] = c.g[V0][0] | 0x8000;
                c.g[A2][0] = 0u64;
                c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                c.g[A3][0] = c.g[S0][0];
                c.sw(r32(c, V0).wrapping_add(0x1cbc), c.g[S0][0]);
                c.g[T0][0] = c.g[S5][0];
                c.sw(r32(c, V0).wrapping_add(0x1cc0), c.g[S1][0]);
                c.g[A1][0] = c.lw(r32(c, S3));
                c.g[A1][0] = c.g[A1][0] & 0x3c0;
                c.g[A1][0] = sx(r32(c, A1) >> 6);
                CHG_MATERIAL_SETUP.call(c, 0x0036_3cfc)?;
                c.g[V0][0] = 0x6_0000;
                break 'b_363d44;
            }
            'b_363d40: {
                // Mode 1: the material kept, the layer changed.
                c.cycles += 6;
                c.g[V0][0] = 0x6_0000;
                c.g[V0][0] = c.g[V0][0] | 0x8000;
                c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1cc0));
                let t = c.g[S1][0] == c.g[V1][0];
                c.g[A0][0] = c.g[S4][0];
                if t {
                    break 'b_363d40;
                }
                c.cycles += 9;
                c.sw(r32(c, V0).wrapping_add(0x1cbc), c.g[S0][0]);
                c.sw(r32(c, V0).wrapping_add(0x1cc0), c.g[S1][0]);
                c.g[A2][0] = 0x1;
                c.g[A3][0] = c.g[S0][0];
                c.g[T0][0] = c.g[S5][0];
                c.g[A1][0] = c.lw(r32(c, S3));
                c.g[A1][0] = c.g[A1][0] & 0x3c0;
                c.g[A1][0] = sx(r32(c, A1) >> 6);
                CHG_MATERIAL_SETUP.call(c, 0x0036_3d40)?;
                break 'b_363d40;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x6_0000;
        }
        // Mode 2: the key word changed.
        c.cycles += 5;
        c.g[V0][0] = c.g[V0][0] | 0x8000;
        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1cbc));
        let t = c.g[V1][0] == c.g[S0][0];
        c.g[A3][0] = c.g[S0][0];
        if t {
            break 'b_363d7c;
        }
        c.cycles += 9;
        c.sw(r32(c, V0).wrapping_add(0x1cc0), c.g[S1][0]);
        c.sw(r32(c, V0).wrapping_add(0x1cbc), c.g[S0][0]);
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = 0x2;
        c.g[T0][0] = c.g[S5][0];
        c.g[A1][0] = c.lw(r32(c, S3));
        c.g[A1][0] = c.g[A1][0] & 0x3c0;
        c.g[A1][0] = sx(r32(c, A1) >> 6);
        CHG_MATERIAL_SETUP.call(c, 0x0036_3d7c)?;
    }
    Ok(())
}

/// The scissor packet: the state's third word (its low five bits) indexes
/// the scissor rectangle table at `self + 0x68000 + 8 * index`, used when the
/// index differs from the last one (`+0x69cc4`).
fn scissor_section(c: &mut Cpu) -> Flow {
    'b_363de8: {
        c.cycles += 13;
        c.g[A0][0] = c.lw(r32(c, S5));
        c.g[V0][0] = 0x6_0000;
        c.sw(r32(c, SP), 0);
        c.g[V0][0] = c.g[V0][0] | 0x8000;
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(0x30));
        c.sw(r32(c, SP).wrapping_add(0x8), c.g[A0][0]);
        c.sw(r32(c, SP).wrapping_add(0x4), c.g[V1][0]);
        c.g[A1][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x8));
        c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x1cc4));
        c.g[V0][0] = c.g[V0][0] & 0x1f;
        if c.g[V0][0] == c.g[V1][0] {
            c.g[V0][0] = 0x6_0000;
            break 'b_363de8;
        }
        'b_363de4: {
            c.cycles += 2;
            let t = (c.g[V0][0] as i64) < 0;
            c.sw(r32(c, A1).wrapping_add(0x1cc4), c.g[V0][0]);
            if t {
                break 'b_363de4;
            }
            c.cycles += 11;
            c.g[V0][0] = sx(r32(c, V0) << 3);
            c.g[A0][0] = c.g[SP][0];
            c.g[T1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
            c.g[V1][0] = c.g[T1][0];
            c.g[T0][0] = c.lhu(r32(c, T1).wrapping_add(0x2eda));
            c.g[A3][0] = c.g[V1][0];
            c.g[V0][0] = c.g[V1][0];
            c.g[A2][0] = c.lhu(r32(c, A3).wrapping_add(0x2ed8));
            c.g[A1][0] = c.lhu(r32(c, V1).wrapping_add(0x2ed4));
            c.g[A3][0] = c.lhu(r32(c, V0).wrapping_add(0x2ed6));
            CHG_SCISSOR.call(c, 0x0036_3de4)?;
        }
        c.cycles += 1;
        c.g[V0][0] = 0x6_0000;
    }
    Ok(())
}

/// The zbuffer packet, when the zbuffer-enable bit (0x400000 of word 1)
/// changed: two `GsPkt_AddZbuf` calls, for context 0 and then 1, from the
/// zbuffer object at `*(gp + 0x2a90)`.
fn zbuf_section(c: &mut Cpu) -> Flow {
    'b_363e44: {
        c.cycles += 9;
        c.g[V1][0] = c.lw(r32(c, S3).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] | 0x9ca8;
        c.g[S1][0] = 0x40_0000;
        c.g[S2][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.g[V1][0] & c.g[S1][0];
        c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
        let t = c.g[V0][0] == c.g[V1][0];
        c.g[A3][0] = sx(r32(c, V1) >> 22);
        if t {
            break 'b_363e44;
        }
        c.cycles += 14;
        c.g[S0][0] = c.lw(r32(c, GP).wrapping_add(0x2a90));
        c.g[A0][0] = c.g[SP][0];
        c.g[T0][0] = 0u64;
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x5a80));
        c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(0x5a40));
        CHG_ZBUF.call(c, 0x0036_3e24)?;
        c.g[A3][0] = c.lw(r32(c, S3).wrapping_add(0x4));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(0x5a40));
        c.g[T0][0] = 0x1;
        c.g[A3][0] = c.g[A3][0] & c.g[S1][0];
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x5a80));
        c.g[A3][0] = sx(r32(c, A3) >> 22);
        CHG_ZBUF.call(c, 0x0036_3e44)?;
    }
    Ok(())
}

/// The test packets (alpha test and depth test fields of word 1, the two
/// contexts), when the bits 0x1800000, 0x300000 or 0xff000 of word 1 differ
/// from the last state. Nothing is written when none differ.
fn test_section(c: &mut Cpu) -> Flow {
    'b_363ef0: {
        'b_363e90: {
            c.cycles += 7;
            c.g[A1][0] = c.lw(r32(c, S2).wrapping_add(0x4));
            c.g[V0][0] = 0x180_0000;
            c.g[A0][0] = c.lw(r32(c, S3).wrapping_add(0x4));
            c.g[V1][0] = c.g[A1][0] & c.g[V0][0];
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A2][0] = c.lw(r32(c, S3).wrapping_add(0x4));
                break 'b_363e90;
            }
            c.cycles += 5;
            c.g[V0][0] = 0x30_0000;
            c.g[V1][0] = c.g[A0][0] & c.g[V0][0];
            c.g[V0][0] = c.g[A1][0] & c.g[V0][0];
            if c.g[V0][0] != c.g[V1][0] {
                c.g[A2][0] = c.lw(r32(c, S3).wrapping_add(0x4));
                break 'b_363e90;
            }
            c.cycles += 6;
            c.g[V0][0] = 0xf0000;
            c.g[V0][0] = c.g[V0][0] | 0xf000;
            c.g[V1][0] = c.g[A0][0] & c.g[V0][0];
            c.g[V0][0] = c.g[A1][0] & c.g[V0][0];
            let t = c.g[V0][0] == c.g[V1][0];
            c.g[V0][0] = 0x6_0000;
            if t {
                break 'b_363ef0;
            }
            c.cycles += 1;
            c.g[A2][0] = c.lw(r32(c, S3).wrapping_add(0x4));
        }
        c.cycles += 24;
        c.g[S0][0] = 0xf0000;
        c.g[S1][0] = 0x180_0000;
        c.g[S0][0] = c.g[S0][0] | 0xf000;
        c.g[A1][0] = c.g[A2][0] & c.g[S1][0];
        c.g[A3][0] = c.g[A2][0] & c.g[S0][0];
        c.g[S2][0] = 0x30_0000;
        c.g[A1][0] = sx(r32(c, A1) >> 23);
        c.g[A2][0] = c.g[A2][0] & c.g[S2][0];
        c.g[A3][0] = sx(r32(c, A3) >> 12);
        c.g[A2][0] = sx(r32(c, A2) >> 20);
        c.g[A0][0] = c.g[SP][0];
        c.g[T0][0] = 0u64;
        CHG_TEST.call(c, 0x0036_3ec4)?;
        c.g[A2][0] = c.lw(r32(c, S3).wrapping_add(0x4));
        c.g[A0][0] = c.g[SP][0];
        c.g[T0][0] = 0x1;
        c.g[S0][0] = c.g[A2][0] & c.g[S0][0];
        c.g[S1][0] = c.g[A2][0] & c.g[S1][0];
        c.g[A2][0] = c.g[A2][0] & c.g[S2][0];
        c.g[A1][0] = sx(r32(c, S1) >> 23);
        c.g[A2][0] = sx(r32(c, A2) >> 20);
        c.g[A3][0] = sx(r32(c, S0) >> 12);
        CHG_TEST.call(c, 0x0036_3eec)?;
        c.g[V0][0] = 0x6_0000;
    }
    Ok(())
}

/// The first wrap-mode clamp packet (word 0, bits 2..3), when it differs.
fn clamp_wrap_s(c: &mut Cpu) -> Flow {
    'b_363f1c: {
        c.cycles += 8;
        c.g[V1][0] = c.lw(r32(c, S3));
        c.g[V0][0] = c.g[V0][0] | 0x9ca8;
        c.g[S0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.g[V1][0] & 0xc;
        c.g[V0][0] = c.lw(r32(c, S0));
        c.g[V0][0] = c.g[V0][0] & 0xc;
        let t = c.g[V0][0] == c.g[V1][0];
        c.g[A1][0] = sx(r32(c, V1) >> 2);
        if t {
            break 'b_363f1c;
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 0u64;
        CHG_CLAMP.call(c, 0x0036_3f1c)?;
    }
    Ok(())
}

/// The first alpha packet (word 1 bits 2..6 and word 0 bits 12..19): the
/// first check compares the alpha bits; the second skips the packet when the
/// alpha value also matches.
fn alpha_wrap_7c(c: &mut Cpu) -> Flow {
    'b_363f70: {
        'b_363f54: {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x4));
            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x4));
            c.g[V0][0] = c.g[V0][0] & 0x7c;
            c.g[V1][0] = c.g[V1][0] & 0x7c;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[A1][0] = sx(r32(c, V0) >> 2);
            if t {
                break 'b_363f54;
            }
            c.cycles += 8;
            c.g[V1][0] = c.lw(r32(c, S0));
            c.g[V0][0] = c.lw(r32(c, S3));
            c.g[V1][0] = sx(r32(c, V1) >> 12);
            c.g[V0][0] = sx(r32(c, V0) >> 12);
            c.g[V1][0] = c.g[V1][0] & 0xff;
            c.g[V0][0] = c.g[V0][0] & 0xff;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x6_0000;
            if t {
                break 'b_363f70;
            }
        }
        c.cycles += 7;
        c.g[A2][0] = c.lw(r32(c, S3));
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = 0u64;
        c.g[A2][0] = sx(r32(c, A2) >> 12);
        c.g[A2][0] = c.g[A2][0] & 0xff;
        CHG_ALPHA.call(c, 0x0036_3f6c)?;
        c.g[V0][0] = 0x6_0000;
    }
    Ok(())
}

/// The second wrap-mode clamp packet (word 0, bits 4..5), when it differs.
fn clamp_wrap_t(c: &mut Cpu) -> Flow {
    'b_363f9c: {
        c.cycles += 8;
        c.g[V1][0] = c.lw(r32(c, S3));
        c.g[V0][0] = c.g[V0][0] | 0x9ca8;
        c.g[S0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.g[V1][0] & 0x30;
        c.g[V0][0] = c.lw(r32(c, S0));
        c.g[V0][0] = c.g[V0][0] & 0x30;
        let t = c.g[V0][0] == c.g[V1][0];
        c.g[A1][0] = sx(r32(c, V1) >> 4);
        if t {
            break 'b_363f9c;
        }
        c.cycles += 3;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 0x1;
        CHG_CLAMP.call(c, 0x0036_3f9c)?;
    }
    Ok(())
}

/// The second alpha packet (word 1 bits 7..11 and word 0 bits 20..27), with
/// the same two checks as `alpha_wrap_7c`.
fn alpha_wrap_f80(c: &mut Cpu) -> Flow {
    'b_363fec: {
        'b_363fd4: {
            c.cycles += 7;
            c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x4));
            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x4));
            c.g[V0][0] = c.g[V0][0] & 0xf80;
            c.g[A2][0] = c.lw(r32(c, S3));
            c.g[V1][0] = c.g[V1][0] & 0xf80;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[A1][0] = sx(r32(c, V0) >> 7);
            if t {
                break 'b_363fd4;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lw(r32(c, S0));
            c.g[V1][0] = sx(r32(c, A2) >> 20);
            c.g[V1][0] = c.g[V1][0] & 0xff;
            c.g[V0][0] = sx(r32(c, V0) >> 20);
            c.g[V0][0] = c.g[V0][0] & 0xff;
            let t = c.g[V0][0] == c.g[V1][0];
            c.g[V0][0] = 0x6_0000;
            if t {
                break 'b_363fec;
            }
        }
        c.cycles += 6;
        c.g[A2][0] = sx(r32(c, A2) >> 20);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[A2][0] & 0xff;
        c.g[A3][0] = 0x1;
        CHG_ALPHA.call(c, 0x0036_3fe8)?;
        c.g[V0][0] = 0x6_0000;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, put_words, DATA, GAME_GP};
    use crate::game::fe::fepopup::tests::base_14;

    /// The render state (`this`): its context block is at `this + 0x68000`.
    const THIS: u32 = DATA;
    /// The three words of the state (param 2).
    const STATE: u32 = DATA + 0x7_0000;
    /// The write pointer slot (`pp`) and the packet buffer it points at.
    const PP: u32 = DATA + 0x7_1000;
    const PKT: u32 = DATA + 0x7_2000;
    /// The zbuffer object that `*(gp + 0x2a90)` points at.
    const ZBUF_OBJ: u32 = DATA + 0x7_3000;
    /// The jump table of `DeferMan_EmitMaterialSetup` (0x492040).
    const MATERIAL_TABLE: [u32; 9] = [
        0x365e20, 0x364d3c, 0x364e38, 0x365190, 0x365314, 0x365624, 0x3658d4, 0x365d7c, 0x365e20,
    ];

    /// Random render state, last-state words equal to the new ones in some
    /// fields and random in others, every material index 0..=8 (0 takes the
    /// early exit), every layer, and the three setup modes.
    fn setup(c: &mut Cpu, seed: u32) {
        let mut r = base_14(c, seed, 0x363c20, 0);
        for (i, t) in MATERIAL_TABLE.iter().enumerate() {
            c.bus.write32(0x0049_2040 + 4 * i as u32, *t);
        }
        for i in 0..0x400u32 {
            c.bus.write32(THIS + 0x69_cc0 + 4 * i, r.next_u32());
        }
        for i in 0..0x80u32 {
            c.bus
                .write32(THIS + 0x68000 + 0x2ec0 + 4 * i, r.next_u32() & 0xffff);
        }
        c.bus.write32(GAME_GP + 0x2a90, ZBUF_OBJ);

        let mat = seed % 9;
        let mut state = [0u32; 5];
        for w in state.iter_mut() {
            *w = r.next_u32();
        }
        state[0] = (state[0] & !0x3c0) | (mat << 6);
        put_words(c, STATE, &state);
        for k in 0..5u32 {
            let v = if (seed >> k) & 1 == 0 {
                state[k as usize]
            } else {
                r.next_u32()
            };
            c.bus.write32(THIS + 0x69ca8 + 4 * k, v);
        }
        let layer = (seed / 2) % 3;
        c.bus.write32(
            THIS + 0x69cbc,
            if seed % 3 == 0 {
                state[3] & 0x3fff_ffff
            } else {
                r.next_u32()
            },
        );
        c.bus.write32(
            THIS + 0x69cc0,
            if seed % 4 < 2 {
                layer
            } else {
                r.next_u32() % 3
            },
        );
        c.bus.write32(
            THIS + 0x69cc4,
            if seed % 5 < 2 {
                state[2] & 0x1f
            } else {
                r.next_u32() % 32
            },
        );
        c.bus.write32(PP, PKT);
        c.g[A0][0] = sx(THIS);
        c.g[A1][0] = sx(STATE);
        c.g[A2][0] = layer as u64;
        c.g[A3][0] = sx(PP);
        c.pc = 0x0036_3c20;
    }

    #[test]
    fn defer_man_emit_state_change_matches() {
        check(
            ee::s12::f_00363c20__generated,
            defer_man_emit_state_change,
            200,
            &setup,
        );
    }
}
