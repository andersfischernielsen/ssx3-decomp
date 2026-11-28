//! Staging module oc1559 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1559`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_f7b8, menu_sys_group_item_page_forward_v04, "MenuSysGroupItem_PageForwardV04"),
    (0x002c_f7e8, menu_sys_group_item_ctor, "MenuSysGroupItem_ctor"),
    (0x002c_f860, menu_sys_group_item_ctor_2, "MenuSysGroupItem_ctor_2"),
    (0x002c_f908, menu_sys_group_item_add_item, "MenuSysGroupItem_AddItem"),
    (0x002c_f928, menu_sys_group_item_remove_item, "MenuSysGroupItem_RemoveItem"),
    (0x002c_f948, menu_sys_group_item_reset, "MenuSysGroupItem_Reset"),
    (0x002c_f988, menu_sys_group_item_set_header_item, "MenuSysGroupItem_SetHeaderItem"),
    (0x002c_fa08, menu_sys_group_item_handle_input, "MenuSysGroupItem_HandleInput"),
];

/// Offsets in a `MenuSysGroupItem`: the header item (+0x14) and the page it
/// holds (+0x18: item count at +0, selected index at +4, item pointers from +0xc).
const MSGI_HEADER: u32 = 0x14;
const MSGI_PAGE: u32 = 0x18;
const MSGI_PAGE_SELECTED: u32 = MSGI_PAGE + 4;
const MSGI_PAGE_ITEMS: u32 = MSGI_PAGE + 0xc;

/// `void MenuSysGroupItem_PageForwardV04(MenuSysGroupItem* this, int a)` (0x2cf7b8):
/// a virtual call on the object at +0x130, with the object's `+0x50` thunk
/// delta applied to `this`; returns what the callee returns.
pub fn menu_sys_group_item_page_forward_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x130)).lw(c);
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x50)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x54)).lw(c);
        let method = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x002c_f7d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MenuSysGroupItem* MenuSysGroupItem_ctor(MenuSysGroupItem* this)` (0x2cf7e8):
/// construct the base item with id -1, the page at +0x18 (vtable 0x4869b8 at +0x10),
/// clear the header, the page's id slot at +0x144 (0x486a18), and store `this` at +0x148.
pub fn menu_sys_group_item_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::f_002ca258,
        }
        .call(c, 0x002c_f800)?;
        c.g[V1][0] = 0x480000;
        Addr((c.g[S0][0] as u32).wrapping_add(MSGI_HEADER)).sw(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x69b8));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(MSGI_PAGE));
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x10)).sw(c, vtable);
        Callee {
            addr: 0x002c_aa58,
            f: ee::s09::f_002caa58,
        }
        .call(c, 0x002c_f818)?;
        c.g[V0][0] = 0x480000;
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6a18));
        c.g[V1][0] = 0x45;
        let id_slot = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x144)).sw(c, id_slot);
        loop {
            c.cycles += 7;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = c.g[V1][0] != c.g[A0][0];
            c.g[V0][0] = c.g[S0][0];
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002c_f830);
                }
                continue;
            }
            c.cycles += 5;
            let this = c.g[S0][0];
            Addr((c.g[S0][0] as u32).wrapping_add(0x148)).sw(c, this);
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `MenuSysGroupItem* MenuSysGroupItem_ctor_2(MenuSysGroupItem* this, void* name,
/// int id)` (0x2cf860): the same construction with a header item (`id` at +0x14);
/// a non-zero id is added to the page.
pub fn menu_sys_group_item_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        Callee {
            addr: 0x002c_a258,
            f: ee::s09::f_002ca258,
        }
        .call(c, 0x002c_f87c)?;
        c.g[V1][0] = 0x480000;
        let id = c.g[S0][0];
        Addr((c.g[S1][0] as u32).wrapping_add(MSGI_HEADER)).sw(c, id);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x69b8));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(MSGI_PAGE));
        let vtable = c.g[V1][0];
        Addr((c.g[S1][0] as u32).wrapping_add(0x10)).sw(c, vtable);
        Callee {
            addr: 0x002c_aa58,
            f: ee::s09::f_002caa58,
        }
        .call(c, 0x002c_f894)?;
        c.g[V0][0] = 0x480000;
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6a18));
        c.g[V1][0] = 0x45;
        let id_slot = c.g[V0][0];
        Addr((c.g[S1][0] as u32).wrapping_add(0x144)).sw(c, id_slot);
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(MSGI_HEADER)).lw(c);
        let t = c.g[A1][0] == 0;
        let this = c.g[S1][0];
        Addr((c.g[S1][0] as u32).wrapping_add(0x148)).sw(c, this);
        if !t {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = u64::MAX;
            Callee {
                addr: 0x002c_f908,
                f: ee::s09::f_002cf908,
            }
            .call(c, 0x002c_f8c0)?;
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_AddItem(MenuSysGroupItem* this, MenuSysItem* item, int index)`
/// (0x2cf908): insert the item into the group's page.
pub fn menu_sys_group_item_add_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(MSGI_PAGE));
        Callee {
            addr: 0x002c_bcc8,
            f: ee::s09::f_002cbcc8,
        }
        .call(c, 0x002c_f918)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_RemoveItem(MenuSysGroupItem* this, int item)` (0x2cf928):
/// remove the item from the group's page.
pub fn menu_sys_group_item_remove_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(MSGI_PAGE));
        Callee {
            addr: 0x002c_bd50,
            f: ee::s09::f_002cbd50,
        }
        .call(c, 0x002c_f938)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_Reset(MenuSysGroupItem* this)` (0x2cf948): empty the
/// page, then add the header item back if there is one.
pub fn menu_sys_group_item_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(MSGI_PAGE));
        Callee {
            addr: 0x002c_be10,
            f: ee::s09::f_002cbe10,
        }
        .call(c, 0x002c_f960)?;
        c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(MSGI_HEADER)).lw(c);
        let t = c.g[A1][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !t {
            c.cycles += 2;
            c.g[A2][0] = u64::MAX;
            Callee {
                addr: 0x002c_f908,
                f: ee::s09::f_002cf908,
            }
            .call(c, 0x002c_f974)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysGroupItem_SetHeaderItem(MenuSysGroupItem* this, MenuSysItem* item)`
/// (0x2cf988): make `item` the header. A previous header is taken out of the page
/// first, and the new one is added to it. A resumable dispatcher: the event check
/// in the search loop returns with `pc` at the loop head, and the function is
/// re-entered there.
pub fn menu_sys_group_item_set_header_item(c: &mut Cpu) {
    guest(c, set_header_item_steps);
}

fn set_header_item_steps(c: &mut Cpu) -> Flow {
    let frame = Frame { size: 0x30 };
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x002c_f988 => {
                c.cycles += 9;
                let _ = Frame::open(c, 0x30);
                frame.save(c, S0, 0x20);
                frame.save(c, S1, 0x10);
                c.g[S0][0] = c.g[A0][0];
                c.g[S1][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                c.g[A1][0] = 0;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                pc = 0x002c_f9b4;
            }
            0x002c_f9ac => {
                c.cycles += 1;
                pc = 0x002c_f9b0;
            }
            0x002c_f9b0 => {
                c.cycles += 1;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                pc = 0x002c_f9b4;
            }
            0x002c_f9b4 => {
                c.cycles += 4;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(MSGI_PAGE)).lw(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    let item = c.g[S1][0];
                    Addr((c.g[S0][0] as u32).wrapping_add(MSGI_HEADER)).sw(c, item);
                    pc = 0x002c_f9e0;
                } else {
                    pc = 0x002c_f9c4;
                }
            }
            0x002c_f9c4 => {
                c.cycles += 4;
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(MSGI_HEADER)).lw(c);
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                if t {
                    if back_edge_unwinds(c, 0x002c_f9b0) {
                        return Ok(());
                    }
                    pc = 0x002c_f9b0;
                } else {
                    pc = 0x002c_f9d4;
                }
            }
            0x002c_f9d4 => {
                c.cycles += 3;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(MSGI_PAGE));
                Callee {
                    addr: 0x002c_bd50,
                    f: ee::s09::f_002cbd50,
                }
                .call(c, 0x002c_f9dc)?;
                let item = c.g[S1][0];
                Addr((c.g[S0][0] as u32).wrapping_add(MSGI_HEADER)).sw(c, item);
                pc = 0x002c_f9e0;
            }
            0x002c_f9e0 => {
                c.cycles += 9;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0;
                Callee {
                    addr: 0x002c_f908,
                    f: ee::s09::f_002cf908,
                }
                .call(c, 0x002c_f9f0)?;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            _ => {
                c.jump_out(pc, 0x002c_f988, 0x002c_fa04);
                return Ok(());
            }
        }
    }
}

/// Leave `HandleInput` after its selection step: `cycles += 1`, s0 restored,
/// then the epilogue.
fn handle_input_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    frame.restore(c, S0, 0x40);
    handle_input_epilogue(c, frame);
    Ok(())
}

/// The epilogue of `HandleInput`: restore s1..s3 and ra, pop the frame.
fn handle_input_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// Key 3 (`MenuSysPage_SelectNext`): when the selection did not move back,
/// set the selection to the previous index and return 3.
fn handle_input_select_next(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = c.g[S3][0];
    Callee {
        addr: 0x002c_ab08,
        f: ee::s09::f_002cab08,
    }
    .call(c, 0x002c_fa8c)?;
    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(MSGI_PAGE_SELECTED)).lw(c);
    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let moved_back = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !moved_back {
        c.cycles += 5;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x002c_be68,
            f: ee::s09::f_002cbe68,
        }
        .call(c, 0x002c_faa8)?;
        c.g[V0][0] = 3;
    }
    handle_input_exit(c, frame)
}

/// Key 4 (`MenuSysPage_SelectPrev`): when the selection did not move back,
/// set the selection to the previous index and return 4.
fn handle_input_select_prev(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = c.g[S3][0];
    Callee {
        addr: 0x002c_abe0,
        f: ee::s09::f_002cabe0,
    }
    .call(c, 0x002c_fab8)?;
    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(MSGI_PAGE_SELECTED)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
    let moved_back = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !moved_back {
        c.cycles += 4;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x002c_be68,
            f: ee::s09::f_002cbe68,
        }
        .call(c, 0x002c_fad4)?;
        c.g[V0][0] = 4;
    }
    handle_input_exit(c, frame)
}

/// `int MenuSysGroupItem_HandleInput(MenuSysGroupItem* this)` (0x2cfa08): validate
/// the selection, ask the selected item's virtual method for a key, and handle
/// the keys 3 (next) and 4 (previous) itself; other keys are returned.
pub fn menu_sys_group_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(MSGI_PAGE));
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S3][0];
        Callee {
            addr: 0x002c_aab0,
            f: ee::s09::f_002caab0,
        }
        .call(c, 0x002c_fa30)?;
        c.g[S2][0] = Addr((c.g[S1][0] as u32).wrapping_add(MSGI_PAGE_SELECTED)).lw(c);
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(MSGI_PAGE_ITEMS)).lw(c);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lw(c);
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x002c_fa54)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x002c_a3b0,
            f: ee::s09::f_002ca3b0,
        }
        .call(c, 0x002c_fa60)?;
        let disabled = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S0][0];
        if disabled {
            return handle_input_exit(c, frame);
        }
        c.cycles += 3;
        c.g[V0][0] = 3;
        let is_next = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = 4;
        if is_next {
            return handle_input_select_next(c, frame);
        }
        c.cycles += 2;
        let is_prev = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = c.g[S0][0];
        if is_prev {
            return handle_input_select_prev(c, frame);
        }
        c.cycles += 2;
        frame.restore(c, S0, 0x40);
        handle_input_epilogue(c, frame);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA};

    /// Scrambled registers and the fake hooks (no unwinds) for a fixture.
    fn msgi_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        fakes::machine(c, seed, salt, 0)
    }

    /// A vtable at `vt` whose slot at +0x10 is `{delta, f}` (the group item's
    /// method table: `+0x10` is the thunk delta, `+0x14` the method).
    fn msgi_vtable(c: &mut Cpu, vt: u32, delta: u16, f: u32) {
        c.bus.write16(vt + 0x10, delta);
        c.bus.write32(vt + 0x14, f);
    }

    #[test]
    fn menu_sys_group_item_page_forward_v04_matches() {
        check(
            ee::s09::f_002cf7b8__generated,
            menu_sys_group_item_page_forward_v04,
            40,
            &|c, seed| {
                let mut r = msgi_setup(c, seed, 0x7b8);
                let this = DATA + 0x1000;
                let page = DATA + 0x2000;
                let item = DATA + 0x3000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x130, page);
                c.bus.write32(page + 0x10, item);
                c.bus.write16(item + 0x50, (r.next_u32() & 0xff) as u16);
                // A real leaf (MenuSysItem_IsVisible) on even seeds, a fake otherwise.
                let target = if seed % 2 == 0 { 0x002c_a378 } else { FAKE };
                c.bus.write32(item + 0x54, target);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_ctor_matches() {
        check(
            ee::s09::f_002cf7e8__generated,
            menu_sys_group_item_ctor,
            40,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0x7e8);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_ctor_2_matches() {
        check(
            ee::s09::f_002cf860__generated,
            menu_sys_group_item_ctor_2,
            40,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0x860);
                c.g[A0][0] = sx(DATA + 0x1000);
                // A zero id skips the add; otherwise the id is added to the page.
                c.g[A2][0] = if seed % 3 == 0 {
                    0
                } else {
                    sx(DATA + 0x5000 + 0x40 * (seed % 4))
                };
            },
        );
    }

    #[test]
    fn menu_sys_group_item_add_item_matches() {
        check(
            ee::s09::f_002cf908__generated,
            menu_sys_group_item_add_item,
            60,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0x908);
                let this = DATA + 0x1000;
                let page = this + MSGI_PAGE;
                let count = seed % 4;
                c.bus.write32(page, count);
                c.bus.write32(page + 4, seed % 3);
                c.bus.write32(page + 0x124, seed % 2);
                for i in 0..count {
                    c.bus.write32(page + 0xc + 4 * i, DATA + 0x6000 + 0x40 * i);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x7000);
                c.g[A2][0] = match seed % 4 {
                    0 => u64::MAX,
                    1 => 0,
                    2 => count as u64,
                    _ => 1,
                };
            },
        );
    }

    #[test]
    fn menu_sys_group_item_remove_item_matches() {
        check(
            ee::s09::f_002cf928__generated,
            menu_sys_group_item_remove_item,
            60,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0x928);
                let this = DATA + 0x1000;
                let page = this + MSGI_PAGE;
                let count = 1 + seed % 3;
                c.bus.write32(page, count);
                c.bus.write32(page + 4, seed % count);
                for i in 0..count {
                    c.bus.write32(page + 0xc + 4 * i, DATA + 0x6000 + 0x40 * i);
                }
                c.g[A0][0] = sx(this);
                // The item to remove: one of the page's items, or one that is not there.
                c.g[A1][0] = sx(DATA + 0x6000 + 0x40 * (seed % (count + 1)));
            },
        );
    }

    #[test]
    fn menu_sys_group_item_reset_matches() {
        check(
            ee::s09::f_002cf948__generated,
            menu_sys_group_item_reset,
            60,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0x948);
                let this = DATA + 0x1000;
                let page = this + MSGI_PAGE;
                let count = seed % 3;
                c.bus.write32(page, count);
                for i in 0..count {
                    c.bus.write32(page + 0xc + 4 * i, DATA + 0x6000 + 0x40 * i);
                }
                let header = if seed % 2 == 0 { 0 } else { DATA + 0x6000 };
                c.bus.write32(this + MSGI_HEADER, header);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_set_header_item_matches() {
        check(
            ee::s09::f_002cf988__generated,
            menu_sys_group_item_set_header_item,
            80,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0x988);
                let this = DATA + 0x1000;
                let page = this + MSGI_PAGE;
                let count = seed % 4;
                c.bus.write32(page, count);
                for i in 0..count {
                    c.bus.write32(page + 0xc + 4 * i, DATA + 0x6000 + 0x40 * i);
                }
                // The old header: one of the items (found, removed), or not in the page.
                let header = if seed % 5 == 4 {
                    DATA + 0x7000
                } else {
                    DATA + 0x6000 + 0x40 * (seed % 3)
                };
                c.bus.write32(this + MSGI_HEADER, header);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x8000);
            },
        );
    }

    #[test]
    fn menu_sys_group_item_handle_input_matches() {
        check(
            ee::s09::f_002cfa08__generated,
            menu_sys_group_item_handle_input,
            120,
            &|c, seed| {
                let _ = msgi_setup(c, seed, 0xa08);
                let this = DATA + 0x1000;
                let page = this + MSGI_PAGE;
                let count = 1 + seed % 4;
                c.bus.write32(page, count);
                c.bus.write32(page + 4, seed % count);
                // Every item shares one method table; its method is a fake that
                // returns the key (the first fake results of the call log).
                let vt = DATA + 0x20_0000;
                for i in 0..count {
                    let item = DATA + 0x6000 + 0x100 * i;
                    c.bus.write32(item + 0x10, vt);
                    c.bus.write32(page + 0xc + 4 * i, item);
                }
                msgi_vtable(c, vt, 0, FAKE);
                let key = [3u32, 4, 5, 1][(seed % 4) as usize];
                for n in 0..8 {
                    c.bus.write32(LOG_RESULTS + 4 * n, key);
                }
                c.bus.write32(this + 4, if seed % 4 == 0 { 1 } else { 0 });
                c.g[A0][0] = sx(this);
            },
        );
    }
}
