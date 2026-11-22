//! Native construction of serialized UI screen objects.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0039_d078,
    ui_screen_create_object,
    "UIScreen_CreateObject",
)];

const ALLOC_NAMED: u32 = 0x0031_7d70;
const OBJECT_CTOR: u32 = 0x0039_fb30;
const LIST_CTOR: u32 = 0x0039_77e8;
const FIND_OBJECT: u32 = 0x0039_d860;
const UNLINK: u32 = 0x0039_7740;
const PUSH_BACK: u32 = 0x0039_7910;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> Addr {
    Addr(r(c, register).wrapping_add(offset))
}

fn allocate(c: &mut Cpu, name: u32, bytes: u32, ret: u32) -> Flow {
    c.g[A1][0] = sx(name);
    c.g[A3][0] = 0;
    c.g[A0][0] = bytes as u64;
    c.g[A2][0] = 0x2000_0000;
    call_ptr(c, ALLOC_NAMED, ret)
}

fn construct_base(c: &mut Cpu, object: usize, ret: u32) -> Flow {
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[object][0];
    call_ptr(c, OBJECT_CTOR, ret)
}

fn construct_group(c: &mut Cpu, object: usize, ret: u32) -> Flow {
    c.g[V1][0] = sx(0x0049_49e8);
    c.g[A0][0] = sx(r(c, object).wrapping_add(0x74));
    at(c, object, 8).sw(c, c.g[V1][0]);
    call_ptr(c, LIST_CTOR, ret)
}

/// Invoke a method from the vtable already held in `table`, retaining the
/// original target register and the adjusted `this` argument.
fn method(c: &mut Cpu, object: usize, table: usize, target: usize, slot: u32, ret: u32) -> Flow {
    c.g[A0][0] = at(c, table, slot).lh(c);
    c.g[target][0] = at(c, table, slot + 4).lw(c);
    let function = r(c, target);
    c.g[A0][0] = sx(r(c, object).wrapping_add(r(c, A0)));
    call_ptr(c, function, ret)
}

fn find(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S5][0];
    call_ptr(c, FIND_OBJECT, ret)
}

/// UIMenu and cUIGroup walk the serialized hashes forward and append each
/// existing child. This preserves the serialized order, including disabled
/// children; selection filtering belongs to the menu's later input code.
fn link_children(
    c: &mut Cpu,
    byte_count: bool,
    count_offset: u32,
    list_offset: u32,
    head: u32,
    find_ret: u32,
    unlink_ret: u32,
    append_ret: u32,
) -> Flow {
    loop {
        c.cycles += 6;
        c.g[A1][0] = at(c, S3, 0).lw(c);
        find(c, find_ret)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, UNLINK, unlink_ret)?;
            at(c, S0, 0x40).sw(c, c.g[S1][0]);
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(r(c, S1).wrapping_add(list_offset));
            call_ptr(c, PUSH_BACK, append_ret)?;
        }
        c.g[V0][0] = if byte_count {
            at(c, S4, count_offset).lbu(c)
        } else {
            at(c, S4, count_offset).lw(c)
        };
        c.cycles += 4;
        c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
        c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S3][0] = sx(r(c, S3).wrapping_add(4));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

fn load_s0(c: &mut Cpu, already_have_table: bool) -> Flow {
    if !already_have_table {
        c.cycles += 2;
        c.g[A1][0] = c.g[S4][0];
        c.g[V0][0] = at(c, S0, 8).lw(c);
    }
    c.cycles += 4;
    method(c, S0, V0, V1, 0x50, 0x0039_d728)?;
    c.cycles += 2;
    c.g[V0][0] = c.g[S0][0];
    Ok(())
}

fn load_s1(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = c.g[S4][0];
    method(c, S1, V0, V1, 0x50, 0x0039_d6dc)?;
    c.cycles += 2;
    c.g[V0][0] = c.g[S1][0];
    Ok(())
}

/// `UIObject* UIScreen_CreateObject(UIScreen* screen, UIObjectDef* def)`
/// (0x39d078): allocate the concrete serialized type, attach already-created
/// children by hash, and load the object's type-specific data. It neither
/// creates missing children recursively nor assigns menu row IDs (+0x18).
pub fn ui_screen_create_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = at(c, S4, 0).lhu(c);
        c.g[A0][0] = sx(r(c, V0).wrapping_sub(0x10));
        c.g[V1][0] = (c.g[A0][0] < 0x12) as u64;
        let supported = c.g[V1][0] != 0;
        c.g[V0][0] = 0x0049_0000;
        let target = if supported {
            c.cycles += 6;
            c.g[V1][0] = sx(r(c, A0) << 2);
            c.g[V0][0] = sx(0x0049_3f70);
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
            c.g[A0][0] = at(c, V1, 0).lw(c);
            r(c, A0)
        } else {
            0x0039_d730
        };

        match target {
            // Type 0x10: cUIGroup.
            0x0039_d124 => {
                c.cycles += 21;
                c.g[S3][0] = sx(r(c, S4).wrapping_add(0x24));
                allocate(c, 0x0049_3f00, 0x90, 0x0039_d140)?;
                c.g[S2][0] = 0;
                c.g[S1][0] = c.g[V0][0];
                construct_base(c, S1, 0x0039_d158)?;
                construct_group(c, S1, 0x0039_d16c)?;
                c.g[V1][0] = at(c, S4, 0x20).lw(c);
                if c.g[V1][0] != 0 {
                    link_children(
                        c,
                        false,
                        0x20,
                        0x74,
                        0x0039_d178,
                        0x0039_d184,
                        0x0039_d198,
                        0x0039_d1a8,
                    )?;
                    c.cycles += 2;
                }
                c.g[V0][0] = at(c, S1, 8).lw(c);
                load_s1(c)?;
            }
            // Type 0x11: icon.
            0x0039_d0cc => {
                c.cycles += 22;
                allocate(c, 0x004a_4788, 0x88, 0x0039_d0e4)?;
                c.g[S0][0] = c.g[V0][0];
                construct_base(c, S0, 0x0039_d0f8)?;
                c.g[V1][0] = sx(0x0049_4aa8);
                c.g[A0][0] = sx(r(c, S0).wrapping_add(0x74));
                at(c, S0, 8).sw(c, c.g[V1][0]);
                call_ptr(c, 0x0021_ccc8, 0x0039_d10c)?;
                c.g[V1][0] = u64::MAX;
                c.g[V0][0] = at(c, S0, 8).lw(c);
                at(c, S0, 0x7c).sw(c, 0);
                c.g[A1][0] = c.g[S4][0];
                at(c, S0, 0x78).sw(c, c.g[V1][0]);
                load_s0(c, true)?;
            }
            // Type 0x12: UIListBox.
            0x0039_d1c4 => {
                c.cycles += 12;
                allocate(c, 0x0049_3f10, 0x31c, 0x0039_d1dc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = 0;
                call_ptr(c, 0x0039_9768, 0x0039_d1ec)?;
                c.g[S0][0] = c.g[V0][0];
                load_s0(c, false)?;
            }
            // Type 0x13: UIMenu, serialized child count is a byte.
            0x0039_d1f4 => {
                c.cycles += 33;
                c.g[S3][0] = sx(r(c, S4).wrapping_add(0x24));
                allocate(c, 0x004a_4790, 0xa8, 0x0039_d210)?;
                c.g[S2][0] = 0;
                c.g[S1][0] = c.g[V0][0];
                construct_base(c, S1, 0x0039_d228)?;
                construct_group(c, S1, 0x0039_d23c)?;
                c.g[V1][0] = sx(0x0049_4928);
                c.g[A0][0] = sx(r(c, S1).wrapping_add(0x90));
                at(c, S1, 8).sw(c, c.g[V1][0]);
                call_ptr(c, 0x003a_5338, 0x0039_d250)?;
                c.g[V0][0] = at(c, S4, 0x20).lbu(c);
                c.g[V1][0] = 0xff;
                at(c, S1, 0xa4).sb(c, c.g[V1][0]);
                for offset in [0x95, 0x96, 0x97, 0x98] {
                    at(c, S1, offset).sb(c, 0);
                }
                at(c, S1, 0x9c).sw(c, 0);
                let has_children = c.g[V0][0] != 0;
                at(c, S1, 0xa0).sw(c, 0);
                if has_children {
                    link_children(
                        c,
                        true,
                        0x20,
                        0x74,
                        0x0039_d278,
                        0x0039_d284,
                        0x0039_d298,
                        0x0039_d2a8,
                    )?;
                    c.cycles += 2;
                } else {
                    c.cycles += 1;
                }
                c.g[V0][0] = at(c, S1, 8).lw(c);
                load_s1(c)?;
            }
            0x0039_d2c4 => create_pair(c)?,
            0x0039_d3d4 => create_control19(c)?,
            0x0039_d488 => create_control21(c)?,
            0x0039_d554 => create_control16(c)?,
            // Type 0x17: UIText.
            0x0039_d60c => {
                c.cycles += 12;
                allocate(c, 0x004a_47a0, 0xbc, 0x0039_d624)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = 0;
                call_ptr(c, 0x003a_0348, 0x0039_d634)?;
                c.g[S0][0] = c.g[V0][0];
                load_s0(c, false)?;
            }
            // Type 0x18: UIVector.
            0x0039_d6e4 => {
                c.cycles += 11;
                allocate(c, 0x0049_3f60, 0x90, 0x0039_d6fc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = 0;
                call_ptr(c, 0x003a_3280, 0x0039_d70c)?;
                c.g[S0][0] = c.g[V0][0];
                load_s0(c, false)?;
            }
            // Type 0x20: cUITextScroll.
            0x0039_d63c => {
                c.cycles += 16;
                c.g[S3][0] = sx(r(c, S4).wrapping_add(0x2c));
                allocate(c, 0x0049_3f50, 0xd0, 0x0039_d658)?;
                c.g[S2][0] = 0;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = 0;
                call_ptr(c, 0x003a_1148, 0x0039_d66c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V0][0] = at(c, S4, 0x22).lbu(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    link_children(
                        c,
                        true,
                        0x22,
                        0x8c,
                        0x0039_d680,
                        0x0039_d68c,
                        0x0039_d6a0,
                        0x0039_d6b0,
                    )?;
                    c.cycles += 1;
                }
                c.g[V0][0] = at(c, S1, 8).lw(c);
                load_s1(c)?;
            }
            0x0039_d730 => {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            destination => {
                c.jump_out(destination, 0x0039_d078, 0x0039_d758);
                return Ok(());
            }
        }
        c.cycles += 9;
        for (register, offset) in [
            (S0, 0x60),
            (S1, 0x50),
            (S2, 0x40),
            (S3, 0x30),
            (S4, 0x20),
            (S5, 0x10),
        ] {
            frame.restore(c, register, offset);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

fn create_pair(c: &mut Cpu) -> Flow {
    c.cycles += 34;
    c.g[S2][0] = sx(r(c, S4).wrapping_add(0x20));
    allocate(c, 0x004a_4798, 0x80, 0x0039_d2e0)?;
    c.g[S0][0] = c.g[V0][0];
    construct_base(c, S0, 0x0039_d2f4)?;
    c.g[V1][0] = sx(0x0049_4868);
    c.g[A0][0] = sx(r(c, S0).wrapping_add(0x74));
    at(c, S0, 8).sw(c, c.g[V1][0]);
    call_ptr(c, 0x003a_5508, 0x0039_d308)?;
    c.g[V1][0] = at(c, S0, 0x14).lw(c);
    c.g[A1][0] = c.g[S4][0];
    c.g[V0][0] = at(c, S0, 8).lw(c);
    c.g[V1][0] |= 2;
    at(c, S0, 0x78).sw(c, 0);
    at(c, S0, 0x7c).sw(c, 0);
    at(c, S0, 0x14).sw(c, c.g[V1][0]);
    method(c, S0, V0, V1, 0x50, 0x0039_d334)?;
    c.g[A1][0] = at(c, S4, 0x20).lw(c);
    find(c, 0x0039_d340)?;
    c.g[S1][0] = c.g[V0][0];
    if c.g[S1][0] != 0 {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, UNLINK, 0x0039_d354)?;
        at(c, S1, 0x40).sw(c, c.g[S0][0]);
        c.g[A2][0] = at(c, S0, 0x78).lw(c);
        let empty = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !empty {
            c.cycles += 5;
            c.g[V0][0] = at(c, A2, 8).lw(c);
            method(c, A2, V0, V1, 8, 0x0039_d378)?;
        }
        c.cycles += 2;
        at(c, S0, 0x78).sw(c, c.g[S1][0]);
    }
    c.g[A1][0] = at(c, S2, 4).lw(c);
    c.cycles += 5;
    find(c, 0x0039_d388)?;
    c.g[S1][0] = c.g[V0][0];
    let found = c.g[S1][0] != 0;
    c.g[V1][0] = u64::MAX - 1;
    if found {
        c.cycles += 9;
        c.g[V0][0] = at(c, S1, 0x14).lw(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        at(c, S1, 0x14).sw(c, c.g[V0][0]);
        call_ptr(c, UNLINK, 0x0039_d3a8)?;
        at(c, S1, 0x40).sw(c, c.g[S0][0]);
        c.g[A2][0] = at(c, S0, 0x7c).lw(c);
        let empty = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !empty {
            c.cycles += 5;
            c.g[V0][0] = at(c, A2, 8).lw(c);
            method(c, A2, V0, V1, 8, 0x0039_d3cc)?;
        }
        c.cycles += 2;
        at(c, S0, 0x7c).sw(c, c.g[S1][0]);
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[S0][0];
    Ok(())
}

fn create_control19(c: &mut Cpu) -> Flow {
    c.cycles += 30;
    allocate(c, 0x0049_3f20, 0x88, 0x0039_d3ec)?;
    c.g[S1][0] = c.g[V0][0];
    construct_base(c, S1, 0x0039_d400)?;
    at(c, S1, 0x74).sw(c, 0);
    c.g[V1][0] = sx(0x0049_4408);
    at(c, S1, 0x78).sw(c, 0);
    c.g[A1][0] = c.g[S4][0];
    c.g[A0][0] = at(c, V1, 0x50).lh(c);
    c.g[V0][0] = at(c, V1, 0x54).lw(c);
    at(c, S1, 0x7c).sw(c, 0);
    c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, A0)));
    at(c, S1, 0x80).sw(c, 0);
    at(c, S1, 0x84).sw(c, 0);
    let function = r(c, V0);
    at(c, S1, 8).sw(c, c.g[V1][0]);
    call_ptr(c, function, 0x0039_d434)?;
    c.g[A1][0] = at(c, S4, 0x20).lw(c);
    find(c, 0x0039_d440)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] != 0 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, UNLINK, 0x0039_d454)?;
        at(c, S0, 0x40).sw(c, c.g[S1][0]);
        at(c, S1, 0x74).sw(c, c.g[S0][0]);
    }
    c.g[A1][0] = at(c, S4, 0x24).lw(c);
    c.cycles += 5;
    find(c, 0x0039_d468)?;
    c.g[S0][0] = c.g[V0][0];
    let found = c.g[S0][0] != 0;
    c.g[V0][0] = c.g[S1][0];
    if found {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, UNLINK, 0x0039_d47c)?;
        at(c, S0, 0x40).sw(c, c.g[S1][0]);
        at(c, S1, 0x78).sw(c, c.g[S0][0]);
        c.cycles += 2;
        c.g[V0][0] = c.g[S1][0];
    }
    Ok(())
}

fn create_control21(c: &mut Cpu) -> Flow {
    c.cycles += 36;
    allocate(c, 0x0049_3f30, 0xcc, 0x0039_d4a0)?;
    c.g[S0][0] = c.g[V0][0];
    construct_base(c, S0, 0x0039_d4b4)?;
    at(c, S0, 0x74).sw(c, 0);
    c.g[V1][0] = sx(0x0049_44c8);
    at(c, S0, 0x78).sw(c, 0);
    c.g[A1][0] = c.g[S4][0];
    c.g[A0][0] = at(c, V1, 0x50).lh(c);
    c.g[V0][0] = at(c, V1, 0x54).lw(c);
    at(c, S0, 0x7c).sw(c, 0);
    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
    for offset in [0x80, 0x84, 0x88, 0xc0, 0xc4, 0xc8] {
        at(c, S0, offset).sw(c, 0);
    }
    at(c, S0, 0x8c).sb(c, 0);
    at(c, S0, 0xa5).sb(c, 0);
    let function = r(c, V0);
    at(c, S0, 8).sw(c, c.g[V1][0]);
    call_ptr(c, function, 0x0039_d500)?;
    c.g[A1][0] = at(c, S4, 0x20).lw(c);
    find(c, 0x0039_d50c)?;
    c.g[S1][0] = c.g[V0][0];
    if c.g[S1][0] != 0 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, UNLINK, 0x0039_d520)?;
        at(c, S1, 0x40).sw(c, c.g[S0][0]);
        at(c, S0, 0x74).sw(c, c.g[S1][0]);
    }
    c.g[A1][0] = at(c, S4, 0x24).lw(c);
    c.cycles += 5;
    find(c, 0x0039_d534)?;
    c.g[S1][0] = c.g[V0][0];
    let found = c.g[S1][0] != 0;
    c.g[V0][0] = c.g[S0][0];
    if found {
        c.cycles += 5;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, UNLINK, 0x0039_d548)?;
        at(c, S1, 0x40).sw(c, c.g[S0][0]);
        at(c, S0, 0x78).sw(c, c.g[S1][0]);
        c.cycles += 2;
        c.g[V0][0] = c.g[S0][0];
    }
    Ok(())
}

fn create_control16(c: &mut Cpu) -> Flow {
    c.cycles += 31;
    allocate(c, 0x0049_3f40, 0x90, 0x0039_d56c)?;
    c.g[S1][0] = c.g[V0][0];
    construct_base(c, S1, 0x0039_d580)?;
    at(c, S1, 0x74).sw(c, 0);
    c.g[V1][0] = sx(0x0049_4348);
    at(c, S1, 0x78).sw(c, 0);
    c.g[A1][0] = c.g[S4][0];
    c.g[A0][0] = at(c, V1, 0x50).lh(c);
    c.g[V0][0] = at(c, V1, 0x54).lw(c);
    at(c, S1, 0x7c).sw(c, 0);
    c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, A0)));
    for offset in [0x80, 0x88, 0x8c] {
        at(c, S1, offset).sw(c, 0);
    }
    let function = r(c, V0);
    at(c, S1, 8).sw(c, c.g[V1][0]);
    call_ptr(c, function, 0x0039_d5b8)?;
    c.g[A1][0] = at(c, S4, 0x24).lw(c);
    find(c, 0x0039_d5c4)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] != 0 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, UNLINK, 0x0039_d5d8)?;
        at(c, S0, 0x40).sw(c, c.g[S1][0]);
        at(c, S1, 0x88).sw(c, c.g[S0][0]);
    }
    c.g[A1][0] = at(c, S4, 0x28).lw(c);
    c.cycles += 5;
    find(c, 0x0039_d5ec)?;
    c.g[S0][0] = c.g[V0][0];
    let found = c.g[S0][0] != 0;
    c.g[V0][0] = c.g[S1][0];
    if found {
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, UNLINK, 0x0039_d600)?;
        at(c, S0, 0x40).sw(c, c.g[S1][0]);
        at(c, S1, 0x8c).sw(c, c.g[S0][0]);
        c.cycles += 2;
        c.g[V0][0] = c.g[S1][0];
    }
    Ok(())
}
