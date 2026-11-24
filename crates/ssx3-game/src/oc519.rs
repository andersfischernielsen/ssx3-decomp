//! Staging module oc519 (wave wave230): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc519`.

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
    0x0024_2eb8,
    ssxapp_static_init_destruct,
    "SSXApp_StaticInitDestruct",
)];

/// `void SSXApp_StaticInitDestruct(int init, int prio)` (0x242eb8; `prio` is
/// `a1`, `init` is `a0`). Does nothing unless `prio == 0xFFFF`. Then with
/// `init != 0` it builds the static tables: the sV4 vector tables at
/// `0x4c90a8..0x4c9428`, the slot table at `0x4a4f68..0x4a4fe0` (`gp+0x1e78..`,
/// with the byte and word globals next to it), the SSXApp object (cSSXApp at
/// `0x4c9428`, 0x226830) and `cRandom_init(GMrandom)` (0x317930). With
/// `init == 0` it destroys the SSXApp object: three sub-object destructors
/// (0x318260, `a1 = 2`, at `+0xcc`, `+0xc8`, `+0xc4`) and then the object's own
/// (0x316d28).
///
/// The straight-line bodies are the generated statements, transcribed in
/// order (every register and cycle the generated body leaves). The calls are
/// [`call_ptr`]s, so an unwinding callee returns at once, as the generated
/// `if c.pc != ret { return; }` does.
pub fn ssxapp_static_init_destruct(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0xffff;
        frame.save(c, 16, 0x90);
        frame.save(c, 17, 0x80);
        frame.save(c, 18, 0x70);
        frame.save(c, 19, 0x60);
        frame.save(c, 20, 0x50);
        frame.save(c, 21, 0x40);
        frame.save(c, 22, 0x30);
        frame.save(c, 23, 0x20);
        frame.save(c, 30, 0x10);
        frame.save_ra(c, 0);
        if c.g[A1][0] == c.g[V0][0] {
            c.cycles += 2;
            c.g[T5][0] = 0x4d0000;
            if c.g[A0][0] != 0 {
                ssxapp_init_static_tables(c)?;
                frame.restore(c, 16, 0x90);
                ssxapp_epilogue(c, frame);
                return Ok(());
            }
            ssxapp_destruct(c)?;
        }
        c.cycles += 1;
        frame.restore(c, 16, 0x90);
        ssxapp_epilogue(c, frame);
        Ok(())
    })
}

/// The epilogue of [`ssxapp_static_init_destruct`]: restore the saved
/// registers, pop the frame, return.
fn ssxapp_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, 17, 0x80);
    frame.restore(c, 18, 0x70);
    frame.restore(c, 19, 0x60);
    frame.restore(c, 20, 0x50);
    frame.restore(c, 21, 0x40);
    frame.restore(c, 22, 0x30);
    frame.restore(c, 23, 0x20);
    frame.restore(c, 30, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// The `init != 0` branch: the static tables and the SSXApp object.
fn ssxapp_init_static_tables(c: &mut Cpu) -> Flow {
    c.cycles += 566;
    c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x511c));
    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x5118));
    c.g[T6][0] = 0x4d0000;
    c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x5114));
    c.g[T7][0] = 0x4d0000;
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5110));
    c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x6f28));
    c.swc1(9, (c.g[T5][0] as u32).wrapping_sub(0x6f48));
    c.g[S1][0] = 0x4d0000;
    c.swc1(10, (c.g[T6][0] as u32).wrapping_sub(0x6f38));
    c.g[S2][0] = 0x4d0000;
    c.swc1(11, (c.g[T7][0] as u32).wrapping_sub(0x6f28));
    c.g[S3][0] = 0x4d0000;
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x510c));
    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6ef8));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5108));
    c.g[S5][0] = 0x4d0000;
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5104));
    c.g[S4][0] = 0x4d0000;
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc));
    c.g[T3][0] = 0x4d0000;
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5100));
    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x6f58));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x6f48));
    c.swc1(8, (c.g[S1][0] as u32).wrapping_sub(0x6f18));
    c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x6f38));
    c.swc1(4, (c.g[S2][0] as u32).wrapping_sub(0x6f08));
    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6f18));
    c.swc1(5, (c.g[S3][0] as u32).wrapping_sub(0x6ef8));
    c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6f08));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x50fc));
    c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6ee8));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x50f8));
    c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6ed8));
    c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
    c.g[T9][0] = 0x4d0000;
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 6);
    c.g[S1][0] = 0x4d0000;
    c.swc1(3, (c.g[S4][0] as u32).wrapping_sub(0x6ee8));
    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6e88));
    c.swc1(1, (c.g[T3][0] as u32).wrapping_sub(0x6ed8));
    c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_sub(0x6ec8));
    c.swc1(0, (c.g[S5][0] as u32).wrapping_sub(0x6f58));
    c.g[S6][0] = 0x4d0000;
    c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[S2][0] = 0x4d0000;
    c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6e78));
    c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x50f4));
    c.g[T7][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6e98));
    c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x50f0));
    c.g[S3][0] = 0x4d0000;
    c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
    c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6e68));
    c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x50ec));
    c.g[FP][0] = 0x4d0000;
    c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x50e8));
    c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6e58));
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.g[T8][0] = 0x4d0000;
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[FP][0] = 0x4d0000;
    c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = 0x4d0000;
    c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6e48));
    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
    c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_sub(0x6eb8));
    c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x50e4));
    c.g[V1][0] = 0x4d0000;
    c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x50e0));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6e08));
    c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
    c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6de8));
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x50dc));
    c.g[S7][0] = 0x4d0000;
    c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[FP][0] = 0x4d0000;
    c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[T6][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x6ea8));
    c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = 0x4d0000;
    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
    c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6df8));
    c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = 0x4d0000;
    c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
    c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6dd8));
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = 0x4d0000;
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = 0x4d0000;
    c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6db8));
    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
    c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6d88));
    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
    c.g[A3][0] = 0x4d0000;
    c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x50d8));
    c.g[A0][0] = 0x4d0000;
    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x50d4));
    c.g[A2][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6da8));
    c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
    c.g[FP][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6d68));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x50d0));
    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6d58));
    c.swc1(2, (c.g[T9][0] as u32).wrapping_sub(0x6ec8));
    c.g[A0][0] = 0x4d0000;
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x50cc));
    c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x50c8));
    c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0xc));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x50c4));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x50c0));
    c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
    c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
    c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
    c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[T8][0] as u32).wrapping_sub(0x6eb8));
    c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
    c.swc1(0, (c.g[S7][0] as u32).wrapping_sub(0x6ea8));
    c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x50bc));
    c.swc1(0, (c.g[S6][0] as u32).wrapping_sub(0x6e98));
    c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6e88));
    c.g[A0][0] = 0x4d0000;
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
    c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6cf8));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x50b8));
    c.g[A0][0] = 0x4d0000;
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x50b4));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6e78));
    c.g[A0][0] = 0x4d0000;
    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
    c.g[T4][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6cd8));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x50b0));
    c.g[A0][0] = 0x4d0000;
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x50ac));
    c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6cc8));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x50a8));
    c.g[A0][0] = 0x4d0000;
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x50a4));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6e68));
    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
    c.g[A0][0] = 0x4d0000;
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6e58));
    c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = 0x4d0000;
    c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
    c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
    c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
    c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0x4));
    c.g[T6][0] = 0xffffffffffff0000;
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[T6][0] = c.g[T6][0] | 0xff00;
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6c58));
    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = 0x4d0000;
    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
    c.g[T5][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6c38));
    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
    c.g[A0][0] = 0x4d0000;
    c.g[S2][0] = 0x4d0000;
    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6c68));
    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
    c.g[S2][0] = 0x4d0000;
    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
    c.g[T7][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6c48));
    c.g[S3][0] = 0x4d0000;
    c.g[S2][0] = 0x4d0000;
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x50a0));
    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6e18));
    c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x509c));
    c.swc1(6, (c.g[S2][0] as u32).wrapping_add(0xc));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
    c.swc1(1, (c.g[S3][0] as u32).wrapping_sub(0x6de8));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6e48));
    c.g[S3][0] = 0x4d0000;
    c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = 0x4d0000;
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6e38));
    c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6c28));
    c.g[A0][0] = 0x4d0000;
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5098));
    c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6c18));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5094));
    c.g[A0][0] = 0x4d0000;
    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x5090));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6e38));
    c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x508c));
    c.swc1(6, (c.g[A0][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = 0x4d0000;
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5088));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6e28));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6e28));
    c.swc1(6, (c.g[A0][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = 0x4d0000;
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6e18));
    c.g[A0][0] = 0x4d0000;
    c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x6e08));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = 0x4d0000;
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6bd8));
    c.g[V0][0] = 0x4d0000;
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5084));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6df8));
    c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = 0x4d0000;
    c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
    c.swc1(7, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.swc1(8, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
    c.g[V1][0] = 0x4d0000;
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5080));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6dd8));
    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
    c.g[V1][0] = 0x4d0000;
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6dc8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6e38));
    c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = 0x4d0000;
    c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6e28));
    c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = 0x4d0000;
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6e18));
    c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = 0x4d0000;
    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6dc8));
    c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = 0x4d0000;
    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
    c.swc1(1, (c.g[T1][0] as u32).wrapping_add(0xc));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x507c));
    c.g[V0][0] = 0x4d0000;
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6db8));
    c.swc1(4, (c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = 0x4d0000;
    c.swc1(5, (c.g[A1][0] as u32).wrapping_add(0x8));
    c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5078));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5074));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6da8));
    c.swc1(6, (c.g[A2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V1][0];
    c.swc1(7, (c.g[A2][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6d98));
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5070));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6d98));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = 0x4d0000;
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6d88));
    c.g[V0][0] = 0x4d0000;
    c.swc1(9, (c.g[T1][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = c.g[V0][0];
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6d78));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6d78));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = 0x4d0000;
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6d68));
    c.g[V1][0] = 0x4d0000;
    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = 0x4d0000;
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6d58));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6d98));
    c.swc1(10, (c.g[A3][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = 0x4d0000;
    c.swc1(11, (c.g[A3][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6d78));
    c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
    c.g[A3][0] = 0x4d0000;
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x506c));
    c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6d48));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6d48));
    c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = 0x4d0000;
    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6ce8));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = 0x4d0000;
    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
    c.g[FP][0] = 0x4d0000;
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0xc));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5068));
    c.g[A3][0] = c.g[V1][0];
    c.swc1(4, (c.g[FP][0] as u32).wrapping_sub(0x6d08));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6d38));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5064));
    c.g[FP][0] = 0x4d0000;
    c.swc1(3, (c.g[T0][0] as u32).wrapping_add(0xc));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5060));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
    c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = c.g[FP][0];
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x505c));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6d28));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6d38));
    c.swc1(5, (c.g[A3][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = 0x4d0000;
    c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
    c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x6d28));
    c.g[A3][0] = c.g[V1][0];
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6d18));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6d18));
    c.g[FP][0] = 0x4d0000;
    c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6d08));
    c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
    c.g[V0][0] = 0x4d0000;
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5058));
    c.g[V1][0] = 0x4d0000;
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5054));
    c.g[A3][0] = c.g[V1][0];
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6cf8));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6ce8));
    c.swc1(6, (c.g[T0][0] as u32).wrapping_add(0x4));
    c.g[FP][0] = 0x4d0000;
    c.swc1(7, (c.g[T0][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = 0x4d0000;
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5050));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6ce8));
    c.swc1(8, (c.g[A3][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = 0x4d0000;
    c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x504c));
    c.g[A3][0] = 0x4d0000;
    c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x6cd8));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6d38));
    c.swc1(9, (c.g[T4][0] as u32).wrapping_add(0x8));
    c.g[FP][0] = 0x4d0000;
    c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6d28));
    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x5048));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6cc8));
    c.swc1(10, (c.g[T2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = 0x4d0000;
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6cb8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6d18));
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = 0x4d0000;
    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6d08));
    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
    c.g[A3][0] = 0x4d0000;
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6ce8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
    c.g[FP][0] = 0x4d0000;
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6cb8));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
    c.g[V0][0] = 0x4d0000;
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6ca8));
    c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = 0x4d0000;
    c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = 0x4d0000;
    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5044));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0xc));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5040));
    c.g[V0][0] = 0x4d0000;
    c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0xc));
    c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0xc));
    c.g[FP][0] = c.g[A3][0];
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6ca8));
    c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6c98));
    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6c98));
    c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
    c.g[V1][0] = c.g[V0][0];
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6c88));
    c.g[A3][0] = 0x4d0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c88));
    c.g[FP][0] = c.g[A3][0];
    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6c78));
    c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6c78));
    c.g[V0][0] = 0x4d0000;
    c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
    c.g[V1][0] = 0x4d0000;
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x503c));
    c.g[A3][0] = 0x4d0000;
    c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6c68));
    c.g[FP][0] = 0x4d0000;
    c.swc1(3, (c.g[S1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = 0x4d0000;
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5038));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6cb8));
    c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x6c58));
    c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = 0x4d0000;
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5034));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6ca8));
    c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6c48));
    c.swc1(5, (c.g[T7][0] as u32).wrapping_add(0x8));
    c.g[A3][0] = 0x4d0000;
    c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6c98));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5030));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x502c));
    c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x6c38));
    c.swc1(6, (c.g[T5][0] as u32).wrapping_add(0x4));
    c.g[FP][0] = 0x4d0000;
    c.swc1(7, (c.g[T5][0] as u32).wrapping_add(0x8));
    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6c88));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.swc1(12, (c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = 0x4d0000;
    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c78));
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = 0xffffffffffffffff;
    c.sw((c.g[A3][0] as u32).wrapping_add(0x8), 0u64);
    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = 0x1;
    c.sw((c.g[FP][0] as u32).wrapping_add(0x8), 0u64);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.g[FP][0] = 0x4d0000;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6c08));
    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[S1][0] = 0x2;
    c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
    c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1e78), c.g[V1][0]);
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1e88), 0u64);
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1e90), c.g[A3][0]);
    c.g[V0][0] = c.g[V1][0] & 0xff;
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5028));
    c.g[V0][0] = c.g[V0][0] | c.g[T6][0];
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1e98), c.g[S1][0]);
    c.g[V1][0] = c.g[A3][0] & 0xff;
    c.g[A2][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1e88));
    c.g[A3][0] = 0x4d0000;
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5024));
    c.g[A1][0] = c.g[S1][0] & 0xff;
    c.swc1(2, (c.g[S3][0] as u32).wrapping_add(0xc));
    c.g[S1][0] = 0x4d0000;
    c.swc1(1, (c.g[FP][0] as u32).wrapping_add(0xc));
    c.g[V1][0] = c.g[V1][0] | 0x100;
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5020));
    c.g[A1][0] = c.g[A1][0] | 0x200;
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x501c));
    c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6c28));
    c.swc1(3, (c.g[S2][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = 0x4d0000;
    c.swc1(4, (c.g[S2][0] as u32).wrapping_add(0x8));
    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5018));
    c.g[S2][0] = 0x4d0000;
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5014));
    c.swc1(0, (c.g[S1][0] as u32).wrapping_sub(0x6c18));
    c.swc1(5, (c.g[S3][0] as u32).wrapping_add(0x4));
    c.g[S1][0] = c.g[A3][0];
    c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0x8));
    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6be8));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5010));
    c.g[S3][0] = 0x4d0000;
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x500c));
    c.swc1(0, (c.g[S2][0] as u32).wrapping_sub(0x6c08));
    c.swc1(7, (c.g[FP][0] as u32).wrapping_add(0x4));
    c.g[S2][0] = 0x3;
    c.swc1(8, (c.g[FP][0] as u32).wrapping_add(0x8));
    c.g[FP][0] = c.g[S3][0];
    c.swc1(0, (c.g[S3][0] as u32).wrapping_sub(0x6bf8));
    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x6bf8));
    c.g[S3][0] = 0x4;
    c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
    c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x6be8));
    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = 0x8;
    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1e78), c.g[V0][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1e88), c.g[A2][0]);
    c.g[V0][0] = 0x6;
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1e90), c.g[V1][0]);
    c.sw((c.g[FP][0] as u32).wrapping_add(0x4), 0u64);
    c.g[V1][0] = 0x7;
    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
    c.g[FP][0] = 0x5;
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1e84), 0u64);
    c.g[S1][0] = 0x9;
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1e80), 0u64);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1e98), c.g[A1][0]);
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1ea0), c.g[S2][0]);
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1ea8), c.g[S3][0]);
    c.g[S2][0] = 0xa;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1eb0), c.g[FP][0]);
    c.g[S3][0] = 0xb;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1eb8), c.g[V0][0]);
    c.g[FP][0] = 0xc;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1ec0), c.g[V1][0]);
    c.g[V0][0] = 0xd;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1ec8), c.g[A3][0]);
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1ed0), c.g[S1][0]);
    c.g[T1][0] = c.g[V1][0] & 0xff;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1ed8), c.g[S2][0]);
    c.g[T0][0] = c.g[A3][0] & 0xff;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1ee0), c.g[S3][0]);
    c.g[A3][0] = c.g[S1][0] & 0xff;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1ee8), c.g[FP][0]);
    c.g[A2][0] = c.g[S2][0] & 0xff;
    c.sb((c.g[GP][0] as u32).wrapping_add(0x1ef0), c.g[V0][0]);
    c.g[A1][0] = c.g[S3][0] & 0xff;
    c.g[T2][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1ea0));
    c.g[V1][0] = c.g[FP][0] & 0xff;
    c.g[T4][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1ea8));
    c.g[V0][0] = c.g[V0][0] & 0xff;
    c.g[T5][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1eb0));
    c.g[T2][0] = c.g[T2][0] | 0x300;
    c.g[T3][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1eb8));
    c.g[T4][0] = c.g[T4][0] | 0x400;
    c.g[T5][0] = c.g[T5][0] | 0x500;
    c.g[T1][0] = c.g[T1][0] | 0x700;
    c.g[T3][0] = c.g[T3][0] | 0x600;
    c.g[T0][0] = c.g[T0][0] | 0x800;
    c.g[A3][0] = c.g[A3][0] | 0x900;
    c.g[A2][0] = c.g[A2][0] | 0xa00;
    c.g[A1][0] = c.g[A1][0] | 0xb00;
    c.g[V1][0] = c.g[V1][0] | 0xc00;
    c.g[V0][0] = c.g[V0][0] | 0xd00;
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1ea0), c.g[T2][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1ea8), c.g[T4][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1eb0), c.g[T5][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1eb8), c.g[T3][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1ec0), c.g[T1][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1ec8), c.g[T0][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1ed0), c.g[A3][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1ed8), c.g[A2][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1ee0), c.g[A1][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1ee8), c.g[V1][0]);
    c.sw((c.g[GP][0] as u32).wrapping_add(0x1ef0), c.g[V0][0]);
    call_ptr(c, 0x00226830, 0x002437b8)?;
    c.g[A0][0] = 0x4d0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6ab8));
    call_ptr(c, 0x00317930, 0x002437c4)?;
    Ok(())
}

/// The `init == 0` branch: destroy the SSXApp object.
fn ssxapp_destruct(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    c.g[S0][0] = 0x4d0000;
    c.g[A1][0] = 0x2;
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6bd8));
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xcc));
    call_ptr(c, 0x00318260, 0x002437e0)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc8));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x00318260, 0x002437ec)?;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc4));
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x00318260, 0x002437f8)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x00316d28, 0x00243804)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fakes, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the callee fakes: they run only when the machine has it.
    const OC519_CTRL: u32 = DATA + 0x20_0000;
    const OC519_MAGIC: u32 = 0x5353_5843;
    static OC519_CTOR_PREV: AtomicUsize = AtomicUsize::new(0);
    static OC519_DTOR_PREV: AtomicUsize = AtomicUsize::new(0);

    /// cSSXApp_cSSXApp (0x226830) with the gate on: logged and returned.
    fn oc519_ctor_fake(c: &mut Cpu) {
        if c.bus.read32(OC519_CTRL) != OC519_MAGIC {
            return fakes::fall_back(c, &OC519_CTOR_PREV, ee::s05::f_00226830__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, 0x0022_6830, ret)
    }

    /// cBXString dtor (0x318260) with the gate on: logged and returned.
    fn oc519_dtor_fake(c: &mut Cpu) {
        if c.bus.read32(OC519_CTRL) != OC519_MAGIC {
            return fakes::fall_back(c, &OC519_DTOR_PREV, ee::s10::f_00318260__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_8260, ret)
    }

    fn oc519_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00226830),
            oc519_ctor_fake,
            &OC519_CTOR_PREV,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            oc519_dtor_fake,
            &OC519_DTOR_PREV,
        );
    }

    /// Both branches of `prio` and `init`: init on even seeds, destroy on odd,
    /// and a prio other than 0xFFFF on every fourth seed.
    fn oc519_setup(c: &mut Cpu, seed: u32) {
        oc519_install_fakes();
        let mut r = fakes::machine(c, seed, 0x5353_5841, 4);
        c.bus.write32(OC519_CTRL, OC519_MAGIC);
        c.g[A0][0] = if seed % 2 == 0 {
            0
        } else {
            (r.next_u32() | 1) as u64
        };
        c.g[A1][0] = if seed % 4 == 3 {
            sx(r.next_u32() & 0xfffe)
        } else {
            0xffff
        };
        c.pc = 0x0024_2eb8;
    }

    #[test]
    fn ssxapp_static_init_destruct_matches() {
        fakes::covered(
            ee::s06::f_00242eb8__generated,
            ssxapp_static_init_destruct,
            200,
            &oc519_setup,
        );
    }
}
