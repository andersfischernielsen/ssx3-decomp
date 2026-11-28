//! Staging module oc1570 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1570`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_fef8, menu_sys_group_item_on_blur, "MenuSysGroupItem_OnBlur"),
    (0x002d_02b0, menu_sys_color_item_ctor, "MenuSysColorItem_ctor"),
    (0x002d_03d0, menu_sys_color_item_handle_input, "MenuSysColorItem_HandleInput"),
    (0x002d_03f0, menu_sys_vector_label_item_ctor, "MenuSysVectorLabelItem_ctor"),
    (0x002d_05a8, menu_sys_vector_item_ctor, "MenuSysVectorItem_ctor"),
    (0x002d_0808, menu_sys_point2d_item_ctor, "MenuSysPoint2DItem_ctor"),
    (0x002d_1740, menu_sys_message_page_ctor, "MenuSysMessagePage_ctor"),
    (0x002d_17f0, menu_sys_message_page_ctor_2, "MenuSysMessagePage_ctor_2"),
];

/// `void MenuSysGroupItem_OnBlur(group)` (0x2cfef8): blurs the group's base
/// item (`MenuSysItem_OnBlur(group)`), then deactivates the page embedded at
/// +0x18 (`MenuSysPage_OnDeactivate(group + 0x18)`).
pub fn menu_sys_group_item_on_blur(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a370,
            f: ee::s09::MenuSysItem_OnBlur,
        }
        .call(c, 0x002c_ff0c)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        Callee {
            addr: 0x002c_bfd0,
            f: ee::s09::MenuSysPage_OnDeactivate,
        }
        .call(c, 0x002c_ff14)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cARGBMenuItem_cARGBMenuItem(item, name, colour, adjust)` (0x2d02b0):
/// the colour-item constructor. Runs the group base on the item
/// (`MenuSysGroupItem_ctor_2(item, name, item + 0x640)`), builds the four
/// channel items at +0x5b0, +0x5d4, +0x5f8 and +0x61c (channels 0 to 3) and
/// the colour swatch at +0x640, stores the vtable (0x486908) at +0x10 and the
/// adjust pointer at +0x5ac, then adds the four sub-items to the group.
/// Returns `item`.
pub fn menu_sys_color_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 72;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S5, 0x30);
        frame.save(c, S7, 0x10);
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x640));
        frame.save(c, S1, 0x70);
        c.g[S7][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        c.g[A2][0] = c.g[S5][0];
        frame.save(c, S6, 0x20);
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b0));
        frame.save_ra(c, 0);
        c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5d4));
        Callee {
            addr: 0x002c_f860,
            f: ee::s09::MenuSysGroupItem_ctor_2,
        }
        .call(c, 0x002d_02f8)?;
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5f8));
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5ac), c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6908));
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A2][0] = 0u64;
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x61c));
        Callee {
            addr: 0x002c_ed20,
            f: ee::s09::MenuSysColorChannelItem_ctor,
        }
        .call(c, 0x002d_0320)?;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x1;
        Callee {
            addr: 0x002c_ed20,
            f: ee::s09::MenuSysColorChannelItem_ctor,
        }
        .call(c, 0x002d_0330)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x2;
        Callee {
            addr: 0x002c_ed20,
            f: ee::s09::MenuSysColorChannelItem_ctor,
        }
        .call(c, 0x002d_0340)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x3;
        Callee {
            addr: 0x002c_ed20,
            f: ee::s09::MenuSysColorChannelItem_ctor,
        }
        .call(c, 0x002d_0350)?;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S7][0];
        c.g[A2][0] = c.g[S1][0];
        Callee {
            addr: 0x002c_ff28,
            f: ee::s09::MenuSysColorSwatchItem_ctor,
        }
        .call(c, 0x002d_0360)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_0370)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_0380)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_0390)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_03a0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x70);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysColorItem_HandleInput(item)` (0x2d03d0): forwards to the group
/// handler at 0x2cfa08 (`MenuSysGroupItem_HandleInput(item)`), returns what it
/// returns.
pub fn menu_sys_color_item_handle_input(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x002c_fa08,
            f: ee::s09::MenuSysGroupItem_HandleInput,
        }),
        0x002d_03e0,
    )
}

/// `void* cVectorTitleMenuItem_cVectorTitleMenuItem(item, caption, vector)`
/// (0x2d03f0): the vector-title item constructor. Runs the item base with
/// id -1, then stores the vector pointer at +0x14, the caption pointer at
/// +0x18 and the vtable (0x4868b8) at +0x10. Returns `item`.
pub fn menu_sys_vector_label_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002d_0418)?;
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x68b8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cVectorMenuItem_cVectorMenuItem(item, name, values, caption, minValue,
/// maxValue)` (0x2d05a8): the vector item constructor. Runs the group base on
/// the item (`MenuSysGroupItem_ctor_2(item, name, item + 0x654)`), stores the
/// vtable (0x486858) at +0x10 and the values pointer at +0x5ac, builds three
/// float editors for the components (`values[0..2]`) at +0x5b8, +0x5ec and
/// +0x620, the vector label (caption) at +0x654, adds the four sub-items to
/// the group and clears the active flag at +0x670. The bounds come in f12
/// (minValue) and f13 (maxValue) and are kept in f20 and f21.
pub fn menu_sys_vector_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 80;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S5, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S6, 0x10);
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x654));
        frame.save_fpr(c, 21, 0x88);
        c.g[S6][0] = c.g[A3][0];
        frame.save_fpr(c, 20, 0x80);
        c.mov_s(21, 13);
        frame.save(c, S2, 0x50);
        c.mov_s(20, 12);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        c.g[A2][0] = c.g[S5][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b8));
        Callee {
            addr: 0x002c_f860,
            f: ee::s09::MenuSysGroupItem_ctor_2,
        }
        .call(c, 0x002d_05f8)?;
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5ec));
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5ac), c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6858));
        c.g[A3][0] = sx(0x004a_39d8);
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S1][0];
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = 0u64;
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x620));
        Callee {
            addr: 0x002c_dce8,
            f: ee::s09::MenuSysFloatItem_ctor_2,
        }
        .call(c, 0x002d_0630)?;
        c.g[A3][0] = sx(0x004a_39e0);
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A1][0] = 0x1;
        Callee {
            addr: 0x002c_dce8,
            f: ee::s09::MenuSysFloatItem_ctor_2,
        }
        .call(c, 0x002d_0650)?;
        c.g[A3][0] = sx(0x004a_39e8);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x2;
        Callee {
            addr: 0x002c_dce8,
            f: ee::s09::MenuSysFloatItem_ctor_2,
        }
        .call(c, 0x002d_0670)?;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S6][0];
        c.g[A2][0] = c.g[S1][0];
        Callee {
            addr: 0x002d_03f0,
            f: ee::s09::MenuSysVectorLabelItem_ctor,
        }
        .call(c, 0x002d_0680)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_0690)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_06a0)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_f908,
            f: ee::s09::MenuSysGroupItem_AddItem,
        }
        .call(c, 0x002d_06b0)?;
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x670), 0u64);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `void* cGraphMenuItem_cGraphMenuItem(item, numPoints, xLockBits, yLockBits,
/// slope, points, minPoint, maxPoint)` (0x2d0808): the graph item constructor.
/// Runs the item base with id -1, stores the vtable (0x486808) and the graph
/// fields (point count, lock bits, slope, points pointer), copies the two
/// 8-byte bound vectors (minPoint, maxPoint) into +0x2c and +0x34 with the
/// unaligned `ldl/ldr/sdl/sdr` pairs, and clears +0x24 and +0x40. Returns `item`.
pub fn menu_sys_point2d_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 53;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S1][0] = c.g[T1][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A3][0];
        frame.save(c, S6, 0x20);
        c.g[S5][0] = c.g[T2][0];
        frame.save(c, S7, 0x10);
        c.g[S6][0] = c.g[T3][0];
        c.g[S7][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = u64::MAX;
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::MenuSysItem_ctor,
        }
        .call(c, 0x002d_0858)?;
        c.g[V1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6808));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S3][0]);
        c.g[A0][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S4][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6738));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[S7][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.ldl(V1, (c.g[S5][0] as u32).wrapping_add(0x7));
        c.ldr(V1, c.g[S5][0] as u32);
        c.sdl(V1, (c.g[S0][0] as u32).wrapping_add(0x33));
        c.sdr(V1, (c.g[S0][0] as u32).wrapping_add(0x2c));
        c.ldl(V1, (c.g[S6][0] as u32).wrapping_add(0x7));
        c.ldr(V1, c.g[S6][0] as u32);
        c.sdl(V1, (c.g[S0][0] as u32).wrapping_add(0x3b));
        c.sdr(V1, (c.g[S0][0] as u32).wrapping_add(0x34));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0u64);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cErrorMenu_cErrorMenu(page, name, ...)` (0x2d1740): the error-menu
/// constructor. Builds the page (`MenuSysPage_ctor(page)`), the title label at
/// +0x130 (`MenuSysLabelItem_ctor`, vtable 0x4867b0 stored at +0x12c), a spacer
/// of 10, the back item at +0x164, and inserts the three items into the page.
/// Returns `page`.
pub fn menu_sys_message_page_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
        frame.save_ra(c, 0);
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
        Callee {
            addr: 0x002c_aa58,
            f: ee::s09::MenuSysPage_ctor,
        }
        .call(c, 0x002d_1768)?;
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
        c.g[V0][0] = 0x480000;
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x67b0));
        c.g[A1][0] = 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
        Callee {
            addr: 0x002c_d2a0,
            f: ee::s09::MenuSysLabelItem_ctor,
        }
        .call(c, 0x002d_1784)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0xa;
        Callee {
            addr: 0x002c_d350,
            f: ee::s09::MenuSysSpacerItem_ctor,
        }
        .call(c, 0x002d_1790)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0u64;
        Callee {
            addr: 0x002c_cdf0,
            f: ee::s09::MenuSysBackItem_ctor,
        }
        .call(c, 0x002d_179c)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::MenuSysPage_InsertItem,
        }
        .call(c, 0x002d_17ac)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::MenuSysPage_InsertItem,
        }
        .call(c, 0x002d_17bc)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::MenuSysPage_InsertItem,
        }
        .call(c, 0x002d_17cc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cErrorMenu_cErrorMenu1(page, name, flag)` (0x2d17f0): the second
/// error-menu constructor, the same page as [`menu_sys_message_page_ctor`]
/// with the title label taking `name` (a1) and the back item taking `flag`
/// (a2) as its second argument. Returns `page`.
pub fn menu_sys_message_page_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
        Callee {
            addr: 0x002c_aa58,
            f: ee::s09::MenuSysPage_ctor,
        }
        .call(c, 0x002d_1820)?;
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
        c.g[V0][0] = 0x480000;
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x67b0));
        c.g[A0][0] = c.g[S2][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
        Callee {
            addr: 0x002c_d2a0,
            f: ee::s09::MenuSysLabelItem_ctor,
        }
        .call(c, 0x002d_1840)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x1e;
        Callee {
            addr: 0x002c_d350,
            f: ee::s09::MenuSysSpacerItem_ctor,
        }
        .call(c, 0x002d_184c)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S3][0];
        Callee {
            addr: 0x002c_cdf0,
            f: ee::s09::MenuSysBackItem_ctor,
        }
        .call(c, 0x002d_1858)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::MenuSysPage_InsertItem,
        }
        .call(c, 0x002d_1868)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::MenuSysPage_InsertItem,
        }
        .call(c, 0x002d_1878)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::MenuSysPage_InsertItem,
        }
        .call(c, 0x002d_1888)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x40);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, fakes, fill, Rng, DATA};

    /// The machine for the constructor tests: the pool allocator and the
    /// dlmalloc arena (the sub-items allocate), with the game image loaded for
    /// the globals the real callees read. Scrambled registers come first.
    pub(crate) fn oc1570_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine_rt(c, seed, salt, 0);
        elf_image(c);
        r
    }

    /// `void MenuSysGroupItem_OnBlur(group)`: the generated body against the port,
    /// the group blurred with its page at +0x18 (real callees, zeroed object).
    #[test]
    fn menu_sys_group_item_on_blur_matches() {
        check(
            ee::s09::f_002cfef8__generated,
            menu_sys_group_item_on_blur,
            40,
            &|c, seed| {
                let _ = oc1570_machine(c, seed, 0x002c_fef8);
                c.g[A0][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    /// `cARGBMenuItem` constructor: the four channels, the swatch and the five
    /// sub-items, with a random colour and adjust pointer.
    #[test]
    fn menu_sys_color_item_ctor_matches() {
        check(
            ee::s09::f_002d02b0__generated,
            menu_sys_color_item_ctor,
            40,
            &|c, seed| {
                let mut r = oc1570_machine(c, seed, 0x002d_02b0);
                let (item, name, colour, adjust) =
                    (DATA + 0x1000, DATA + 0x1_0000, DATA + 0x1_0100, DATA + 0x1_0200);
                for i in 0..4 {
                    c.bus.write32(colour + 4 * i, r.f32(100.0));
                }
                c.g[A0][0] = item as u64;
                c.g[A1][0] = name as u64;
                c.g[A2][0] = colour as u64;
                c.g[A3][0] = adjust as u64;
            },
        );
    }

    /// `MenuSysColorItem_HandleInput(item)`: forwards to the group handler.
    #[test]
    fn menu_sys_color_item_handle_input_matches() {
        check(
            ee::s09::f_002d03d0__generated,
            menu_sys_color_item_handle_input,
            40,
            &|c, seed| {
                let _ = oc1570_machine(c, seed, 0x002d_03d0);
                c.g[A0][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    /// `cVectorTitleMenuItem` constructor: the item base, vector and vtable.
    #[test]
    fn menu_sys_vector_label_item_ctor_matches() {
        check(
            ee::s09::f_002d03f0__generated,
            menu_sys_vector_label_item_ctor,
            40,
            &|c, seed| {
                let mut r = oc1570_machine(c, seed, 0x002d_03f0);
                let (item, caption, vector) = (DATA + 0x1000, DATA + 0x1_0000, DATA + 0x1_0100);
                fill(c, &mut r, vector, 4);
                c.g[A0][0] = item as u64;
                c.g[A1][0] = caption as u64;
                c.g[A2][0] = vector as u64;
            },
        );
    }

    /// `cVectorMenuItem` constructor: the group base, three float editors, the
    /// vector label and the four sub-items; min and max come in f12 and f13.
    #[test]
    fn menu_sys_vector_item_ctor_matches() {
        check(
            ee::s09::f_002d05a8__generated,
            menu_sys_vector_item_ctor,
            40,
            &|c, seed| {
                let mut r = oc1570_machine(c, seed, 0x002d_05a8);
                let (item, name, values, caption) =
                    (DATA + 0x1000, DATA + 0x1_0000, DATA + 0x1_0100, DATA + 0x1_0200);
                for i in 0..3 {
                    c.bus.write32(values + 4 * i, r.f32(100.0));
                }
                c.g[A0][0] = item as u64;
                c.g[A1][0] = name as u64;
                c.g[A2][0] = values as u64;
                c.g[A3][0] = caption as u64;
                c.f[12] = r.f32(50.0);
                c.f[13] = r.f32(50.0);
            },
        );
    }

    /// `cGraphMenuItem` constructor: the point count, lock bits, slope, the
    /// points pointer and the two 8-byte bound vectors copied unaligned.
    #[test]
    fn menu_sys_point2d_item_ctor_matches() {
        check(
            ee::s09::f_002d0808__generated,
            menu_sys_point2d_item_ctor,
            40,
            &|c, seed| {
                let mut r = oc1570_machine(c, seed, 0x002d_0808);
                let (item, points, min, max) =
                    (DATA + 0x1000, DATA + 0x1_0100, DATA + 0x1_0200, DATA + 0x1_0300);
                fill(c, &mut r, min, 2);
                fill(c, &mut r, max, 2);
                c.g[A0][0] = item as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = points as u64;
                c.g[T2][0] = min as u64;
                c.g[T3][0] = max as u64;
            },
        );
    }

    /// `cErrorMenu_cErrorMenu(page, ...)`: the page, the title label, the
    /// spacer, the back item and three inserts.
    #[test]
    fn menu_sys_message_page_ctor_matches() {
        check(
            ee::s09::f_002d1740__generated,
            menu_sys_message_page_ctor,
            40,
            &|c, seed| {
                let _ = oc1570_machine(c, seed, 0x002d_1740);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x1_0000) as u64;
            },
        );
    }

    /// `cErrorMenu_cErrorMenu1(page, name, flag)`: the second error menu.
    #[test]
    fn menu_sys_message_page_ctor_2_matches() {
        check(
            ee::s09::f_002d17f0__generated,
            menu_sys_message_page_ctor_2,
            40,
            &|c, seed| {
                let mut r = oc1570_machine(c, seed, 0x002d_17f0);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x1_0000) as u64;
                c.g[A2][0] = (r.next_u32() % 4) as u64;
            },
        );
    }
}
