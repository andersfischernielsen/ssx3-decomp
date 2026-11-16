//! Staging module oc385 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc385`.

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
    0x0039_d078,
    ui_screen_create_object,
    "UIScreen_CreateObject",
)];

// ------------------------------------------------------------ UIScreen_CreateObject

/// The record-type jump table (`rodata`, indexed by `type - 0x10`).
pub(crate) const UCS_TABLE: u32 = 0x0049_3f70;
/// `operator new(size, name, flags, 0)` (called through the function table).
const UCS_ALLOC: u32 = 0x0031_7d70;
/// `cUIIcon`'s image-handle constructor (called through the function table).
const UCS_ICON_HANDLE_CTOR: u32 = 0x0021_ccc8;

const UCS_OBJECT_CTOR: Callee = Callee {
    addr: 0x0039_fb30,
    f: ee::s13::UIObject_ctor,
};
const UCS_LIST_CTOR: Callee = Callee {
    addr: 0x0039_77e8,
    f: ee::s13::UIList_ctor,
};
const UCS_FIND: Callee = Callee {
    addr: 0x0039_d860,
    f: ee::s13::UIScreen_FindObject,
};
const UCS_UNLINK: Callee = Callee {
    addr: 0x0039_7740,
    f: ee::s13::UIListNode_Unlink,
};
const UCS_PUSH_BACK: Callee = Callee {
    addr: 0x0039_7910,
    f: ee::s13::UIList_PushBack,
};
const UCS_LIST_BOX_CTOR: Callee = Callee {
    addr: 0x0039_9768,
    f: ee::s13::UIListBox_ctor,
};
const UCS_MENU_FLAGS_CTOR: Callee = Callee {
    addr: 0x003a_5338,
    f: ee::s13::UIMenuFlags_ctor,
};
const UCS_PAIR_FLAGS_CTOR: Callee = Callee {
    addr: 0x003a_5508,
    f: ee::s13::UIPairFlags_ctor,
};
const UCS_TEXT_CTOR: Callee = Callee {
    addr: 0x003a_0348,
    f: ee::s13::UIText_ctor,
};
const UCS_TEXT_SCROLL_CTOR: Callee = Callee {
    addr: 0x003a_1148,
    f: ee::s13::cUITextScroll_ctor,
};
const UCS_VECTOR_CTOR: Callee = Callee {
    addr: 0x003a_3280,
    f: ee::s13::UIVector_ctor,
};

/// Object vtables the constructor stores (`cUIIcon`, `cUIGroup`, `cUIMenu`,
/// `cUIPair`, `cUIProgress`, type 0x21, type 0x16).
const UCS_VT_ICON: u32 = 0x0049_4aa8;
const UCS_VT_GROUP: u32 = 0x0049_49e8;
const UCS_VT_MENU: u32 = 0x0049_4928;
const UCS_VT_PAIR: u32 = 0x0049_4868;
const UCS_VT_PROGRESS: u32 = 0x0049_4408;
const UCS_VT_21: u32 = 0x0049_44c8;
const UCS_VT_16: u32 = 0x0049_4348;

/// Object field offsets.
const UCS_OBJ_VTABLE: u32 = 0x8;
const UCS_OBJ_FLAGS: u32 = 0x14;
/// A child's parent pointer.
const UCS_CHILD_PARENT: u32 = 0x40;
/// Vtable slot of `initFromRec(rec)` (`{delta: i16, fn}`) and of the deleting
/// destructor.
const UCS_VT_INIT: u32 = 0x50;
const UCS_VT_DELETE: u32 = 0x8;

#[inline]
fn ucs_reg(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `operator new(size, name, 0x20000000, 0)` returning to `ret`.
fn ucs_alloc(c: &mut Cpu, size: u32, name: u32, ret: u32) -> Flow {
    c.g[A1][0] = sx(name);
    c.g[A3][0] = 0;
    c.g[A0][0] = size as u64;
    c.g[A2][0] = 0x2000_0000;
    call_ptr(c, UCS_ALLOC, ret)
}

/// `ctor(obj_reg, screen, 0)`: `a0 = obj`, `a1 = s5`, `a2 = 0`.
fn ucs_ctor(c: &mut Cpu, ctor: Callee, obj_reg: usize, ret: u32) -> Flow {
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[obj_reg][0];
    ctor.call(c, ret)
}

/// `ctor(new block in v0, screen, 0)`.
fn ucs_ctor_v0(c: &mut Cpu, ctor: Callee, ret: u32) -> Flow {
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S5][0];
    c.g[A2][0] = 0;
    ctor.call(c, ret)
}

/// Store vtable `vt` (left in `v1`) into the object in `obj_reg` and point
/// `a0` at its member at `member` (the next constructor's `this`).
fn ucs_set_vtable(c: &mut Cpu, obj_reg: usize, vt: u32, member: u32) {
    let obj = ucs_reg(c, obj_reg);
    c.g[V1][0] = sx(vt);
    c.g[A0][0] = sx(obj.wrapping_add(member));
    Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).sw(c, sx(vt));
}

/// The virtual call through vtable `vt`'s slot `slot` on `this`
/// (`a0 = this + delta`, `v1` = the method), returning to `ret`.
fn ucs_virtual(c: &mut Cpu, vt: u32, slot: u32, this: u32, ret: u32) -> Flow {
    c.g[A0][0] = Addr(vt.wrapping_add(slot)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(slot + 4)).lw(c);
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret)
}

/// 0x39d6c8: `s1->initFromRec(rec)` through the vtable in `v0`, return `s1`.
fn ucs_init_s1(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = c.g[S4][0];
    let vt = c.g[V0][0] as u32;
    let this = ucs_reg(c, S1);
    ucs_virtual(c, vt, UCS_VT_INIT, this, 0x0039_d6dc)?;
    c.cycles += 2;
    c.g[V0][0] = c.g[S1][0];
    Ok(())
}

/// 0x39d6c4: load `s1`'s vtable, then [`ucs_init_s1`].
fn ucs_init_s1_load(c: &mut Cpu) -> Flow {
    c.cycles += 1;
    let obj = ucs_reg(c, S1);
    c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
    ucs_init_s1(c)
}

/// 0x39d718: `s0->initFromRec(a1)` through the vtable in `v0`, return `s0`.
fn ucs_init_s0(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    let vt = c.g[V0][0] as u32;
    let this = ucs_reg(c, S0);
    ucs_virtual(c, vt, UCS_VT_INIT, this, 0x0039_d728)?;
    c.cycles += 2;
    c.g[V0][0] = c.g[S0][0];
    Ok(())
}

/// 0x39d710: `a1 = rec`, load `s0`'s vtable, then [`ucs_init_s0`].
fn ucs_init_s0_load(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[A1][0] = c.g[S4][0];
    let obj = ucs_reg(c, S0);
    c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
    ucs_init_s0(c)
}

/// Where a child loop reads its count from the record.
#[derive(Clone, Copy)]
enum UcsCount {
    /// `lw rec+off`.
    Word(u32),
    /// `lbu rec+off`.
    Byte(u32),
}

fn ucs_count(c: &mut Cpu, count: UcsCount) -> u64 {
    let rec = ucs_reg(c, S4);
    match count {
        UcsCount::Word(off) => Addr(rec.wrapping_add(off)).lw(c),
        UcsCount::Byte(off) => Addr(rec.wrapping_add(off)).lbu(c),
    }
}

/// The child loop (head `head`): for each name hash at `s3` (s2 counting up
/// to the record's count), find the child on the screen, unlink it, parent
/// it to `s1` and append it to `s1`'s list at `list`. Left at the exit test.
fn ucs_adopt_children(c: &mut Cpu, count: UcsCount, list: u32, head: u32, rets: [u32; 3]) -> Flow {
    loop {
        c.cycles += 6;
        let hash_at = ucs_reg(c, S3);
        c.g[A1][0] = Addr(hash_at).lw(c);
        c.g[A0][0] = c.g[S5][0];
        UCS_FIND.call(c, rets[0])?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            UCS_UNLINK.call(c, rets[1])?;
            let (child, owner) = (ucs_reg(c, S0), c.g[S1][0]);
            Addr(child.wrapping_add(UCS_CHILD_PARENT)).sw(c, owner);
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(ucs_reg(c, S1).wrapping_add(list));
            UCS_PUSH_BACK.call(c, rets[2])?;
        }
        c.g[V0][0] = ucs_count(c, count);
        c.cycles += 4;
        c.g[S2][0] = sx(ucs_reg(c, S2).wrapping_add(1));
        let more = c.g[S2][0] < c.g[V0][0];
        c.g[V0][0] = more as u64;
        c.g[S3][0] = sx(ucs_reg(c, S3).wrapping_add(4));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// Two named parts (types 0x19, 0x21, 0x16): the hashes at `rec+hashes[i]`
/// name screen objects; each one found is unlinked, parented to the object
/// in `obj` and stored at `obj+slots[i]`. Returns the object in `v0`.
fn ucs_attach_two(
    c: &mut Cpu,
    obj: usize,
    part: usize,
    hashes: [u32; 2],
    slots: [u32; 2],
    rets: [u32; 4],
) -> Flow {
    let rec = ucs_reg(c, S4);
    c.g[A1][0] = Addr(rec.wrapping_add(hashes[0])).lw(c);
    c.g[A0][0] = c.g[S5][0];
    UCS_FIND.call(c, rets[0])?;
    c.g[part][0] = c.g[V0][0];
    if c.g[part][0] != 0 {
        c.cycles += 5;
        c.g[A0][0] = c.g[part][0];
        UCS_UNLINK.call(c, rets[1])?;
        ucs_parent_part(c, obj, part, slots[0]);
    }
    let rec = ucs_reg(c, S4);
    c.g[A1][0] = Addr(rec.wrapping_add(hashes[1])).lw(c);
    c.cycles += 5;
    c.g[A0][0] = c.g[S5][0];
    UCS_FIND.call(c, rets[2])?;
    c.g[part][0] = c.g[V0][0];
    c.g[V0][0] = c.g[obj][0];
    if c.g[part][0] == 0 {
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[part][0];
    UCS_UNLINK.call(c, rets[3])?;
    ucs_parent_part(c, obj, part, slots[1]);
    c.cycles += 2;
    c.g[V0][0] = c.g[obj][0];
    Ok(())
}

/// `part->parent = obj; obj->slot = part`.
fn ucs_parent_part(c: &mut Cpu, obj: usize, part: usize, slot: u32) {
    let (o, p) = (c.g[obj][0], c.g[part][0]);
    Addr((p as u32).wrapping_add(UCS_CHILD_PARENT)).sw(c, o);
    Addr((o as u32).wrapping_add(slot)).sw(c, p);
}

/// `cUIPair`: delete the part already at `s0+slot` (virtual deleting
/// destructor with `a1 = 3`), then store the new part `s1` there.
fn ucs_replace_part(c: &mut Cpu, slot: u32, ret: u32) -> Flow {
    let obj = ucs_reg(c, S0);
    c.g[A2][0] = Addr(obj.wrapping_add(slot)).lw(c);
    c.g[A1][0] = 3;
    if c.g[A2][0] != 0 {
        c.cycles += 5;
        let old = ucs_reg(c, A2);
        c.g[V0][0] = Addr(old.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
        let vt = c.g[V0][0] as u32;
        ucs_virtual(c, vt, UCS_VT_DELETE, old, ret)?;
    }
    c.cycles += 2;
    let (obj, part) = (ucs_reg(c, S0), c.g[S1][0]);
    Addr(obj.wrapping_add(slot)).sw(c, part);
    Ok(())
}

/// The object for record type `ty` (0x10..=0x21); `v0` = the object or 0.
fn ucs_build(c: &mut Cpu, ty: u32) -> Flow {
    match ty {
        // cUIGroup: children named by the word count at +0x20.
        0x10 => {
            c.cycles += 21;
            c.g[S3][0] = sx(ucs_reg(c, S4).wrapping_add(0x24));
            ucs_alloc(c, 0x90, 0x0049_3f00, 0x0039_d140)?;
            c.g[S2][0] = 0;
            c.g[S1][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S1, 0x0039_d158)?;
            ucs_set_vtable(c, S1, UCS_VT_GROUP, 0x74);
            UCS_LIST_CTOR.call(c, 0x0039_d16c)?;
            let rec = ucs_reg(c, S4);
            c.g[V1][0] = Addr(rec.wrapping_add(0x20)).lw(c);
            if c.g[V1][0] != 0 {
                ucs_adopt_children(
                    c,
                    UcsCount::Word(0x20),
                    0x74,
                    0x0039_d178,
                    [0x0039_d184, 0x0039_d198, 0x0039_d1a8],
                )?;
                c.cycles += 2;
            }
            let obj = ucs_reg(c, S1);
            c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
            ucs_init_s1(c)
        }
        // cUIIcon.
        0x11 => {
            c.cycles += 22;
            ucs_alloc(c, 0x88, 0x004a_4788, 0x0039_d0e4)?;
            c.g[S0][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S0, 0x0039_d0f8)?;
            ucs_set_vtable(c, S0, UCS_VT_ICON, 0x74);
            call_ptr(c, UCS_ICON_HANDLE_CTOR, 0x0039_d10c)?;
            let obj = ucs_reg(c, S0);
            c.g[V1][0] = u64::MAX;
            c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
            Addr(obj.wrapping_add(0x7c)).sw(c, 0);
            c.g[A1][0] = c.g[S4][0];
            Addr(obj.wrapping_add(0x78)).sw(c, u64::MAX);
            ucs_init_s0(c)
        }
        // cUIListBox.
        0x12 => {
            c.cycles += 12;
            ucs_alloc(c, 0x31c, 0x0049_3f10, 0x0039_d1dc)?;
            ucs_ctor_v0(c, UCS_LIST_BOX_CTOR, 0x0039_d1ec)?;
            c.g[S0][0] = c.g[V0][0];
            ucs_init_s0_load(c)
        }
        // cUIMenu: a group with menu flags; byte count at +0x20.
        0x13 => {
            c.cycles += 33;
            c.g[S3][0] = sx(ucs_reg(c, S4).wrapping_add(0x24));
            ucs_alloc(c, 0xa8, 0x004a_4790, 0x0039_d210)?;
            c.g[S2][0] = 0;
            c.g[S1][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S1, 0x0039_d228)?;
            ucs_set_vtable(c, S1, UCS_VT_GROUP, 0x74);
            UCS_LIST_CTOR.call(c, 0x0039_d23c)?;
            ucs_set_vtable(c, S1, UCS_VT_MENU, 0x90);
            UCS_MENU_FLAGS_CTOR.call(c, 0x0039_d250)?;
            let (rec, obj) = (ucs_reg(c, S4), ucs_reg(c, S1));
            c.g[V0][0] = Addr(rec.wrapping_add(0x20)).lbu(c);
            c.g[V1][0] = 0xff;
            Addr(obj.wrapping_add(0xa4)).sb(c, 0xff);
            for off in [0x95, 0x96, 0x97, 0x98] {
                Addr(obj.wrapping_add(off)).sb(c, 0);
            }
            Addr(obj.wrapping_add(0x9c)).sw(c, 0);
            Addr(obj.wrapping_add(0xa0)).sw(c, 0);
            if c.g[V0][0] == 0 {
                return ucs_init_s1_load(c);
            }
            ucs_adopt_children(
                c,
                UcsCount::Byte(0x20),
                0x74,
                0x0039_d278,
                [0x0039_d284, 0x0039_d298, 0x0039_d2a8],
            )?;
            c.cycles += 2;
            let obj = ucs_reg(c, S1);
            c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
            ucs_init_s1(c)
        }
        // cUIPair: two parts named at +0x20/+0x24, replacing older ones.
        0x15 => {
            c.cycles += 34;
            c.g[S2][0] = sx(ucs_reg(c, S4).wrapping_add(0x20));
            ucs_alloc(c, 0x80, 0x004a_4798, 0x0039_d2e0)?;
            c.g[S0][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S0, 0x0039_d2f4)?;
            ucs_set_vtable(c, S0, UCS_VT_PAIR, 0x74);
            UCS_PAIR_FLAGS_CTOR.call(c, 0x0039_d308)?;
            let obj = ucs_reg(c, S0);
            c.g[V1][0] = Addr(obj.wrapping_add(UCS_OBJ_FLAGS)).lw(c);
            c.g[A1][0] = c.g[S4][0];
            c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
            c.g[V1][0] |= 2;
            Addr(obj.wrapping_add(0x78)).sw(c, 0);
            Addr(obj.wrapping_add(0x7c)).sw(c, 0);
            let flags = c.g[V1][0];
            Addr(obj.wrapping_add(UCS_OBJ_FLAGS)).sw(c, flags);
            let vt = c.g[V0][0] as u32;
            ucs_virtual(c, vt, UCS_VT_INIT, obj, 0x0039_d334)?;
            let rec = ucs_reg(c, S4);
            c.g[A1][0] = Addr(rec.wrapping_add(0x20)).lw(c);
            c.g[A0][0] = c.g[S5][0];
            UCS_FIND.call(c, 0x0039_d340)?;
            c.g[S1][0] = c.g[V0][0];
            if c.g[S1][0] != 0 {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                UCS_UNLINK.call(c, 0x0039_d354)?;
                let (part, owner) = (ucs_reg(c, S1), c.g[S0][0]);
                Addr(part.wrapping_add(UCS_CHILD_PARENT)).sw(c, owner);
                ucs_replace_part(c, 0x78, 0x0039_d378)?;
            }
            let names = ucs_reg(c, S2);
            c.g[A1][0] = Addr(names.wrapping_add(4)).lw(c);
            c.cycles += 5;
            c.g[A0][0] = c.g[S5][0];
            UCS_FIND.call(c, 0x0039_d388)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = !1u64;
            if c.g[S1][0] != 0 {
                // The second part starts hidden (flag bit 0 clear).
                c.cycles += 9;
                let part = ucs_reg(c, S1);
                c.g[V0][0] = Addr(part.wrapping_add(UCS_OBJ_FLAGS)).lw(c);
                c.g[A0][0] = c.g[S1][0];
                c.g[V0][0] &= c.g[V1][0];
                let flags = c.g[V0][0];
                Addr(part.wrapping_add(UCS_OBJ_FLAGS)).sw(c, flags);
                UCS_UNLINK.call(c, 0x0039_d3a8)?;
                let (part, owner) = (ucs_reg(c, S1), c.g[S0][0]);
                Addr(part.wrapping_add(UCS_CHILD_PARENT)).sw(c, owner);
                ucs_replace_part(c, 0x7c, 0x0039_d3cc)?;
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[S0][0];
            Ok(())
        }
        // Type 0x16: parts named at +0x24/+0x28 into +0x88/+0x8c.
        0x16 => {
            c.cycles += 31;
            ucs_alloc(c, 0x90, 0x0049_3f40, 0x0039_d56c)?;
            c.g[S1][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S1, 0x0039_d580)?;
            let obj = ucs_reg(c, S1);
            Addr(obj.wrapping_add(0x74)).sw(c, 0);
            c.g[V1][0] = sx(UCS_VT_16);
            Addr(obj.wrapping_add(0x78)).sw(c, 0);
            c.g[A1][0] = c.g[S4][0];
            c.g[A0][0] = Addr(UCS_VT_16 + UCS_VT_INIT).lh(c);
            c.g[V0][0] = Addr(UCS_VT_16 + UCS_VT_INIT + 4).lw(c);
            Addr(obj.wrapping_add(0x7c)).sw(c, 0);
            c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
            for off in [0x80, 0x88, 0x8c] {
                Addr(obj.wrapping_add(off)).sw(c, 0);
            }
            let method = c.g[V0][0] as u32;
            Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).sw(c, sx(UCS_VT_16));
            call_ptr(c, method, 0x0039_d5b8)?;
            ucs_attach_two(
                c,
                S1,
                S0,
                [0x24, 0x28],
                [0x88, 0x8c],
                [0x0039_d5c4, 0x0039_d5d8, 0x0039_d5ec, 0x0039_d600],
            )
        }
        // cUIText.
        0x17 => {
            c.cycles += 12;
            ucs_alloc(c, 0xbc, 0x004a_47a0, 0x0039_d624)?;
            ucs_ctor_v0(c, UCS_TEXT_CTOR, 0x0039_d634)?;
            c.g[S0][0] = c.g[V0][0];
            ucs_init_s0_load(c)
        }
        // cUIVector.
        0x18 => {
            c.cycles += 11;
            ucs_alloc(c, 0x90, 0x0049_3f60, 0x0039_d6fc)?;
            ucs_ctor_v0(c, UCS_VECTOR_CTOR, 0x0039_d70c)?;
            c.g[S0][0] = c.g[V0][0];
            ucs_init_s0_load(c)
        }
        // cUIProgress: parts named at +0x20/+0x24 into +0x74/+0x78.
        0x19 => {
            c.cycles += 30;
            ucs_alloc(c, 0x88, 0x0049_3f20, 0x0039_d3ec)?;
            c.g[S1][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S1, 0x0039_d400)?;
            let obj = ucs_reg(c, S1);
            Addr(obj.wrapping_add(0x74)).sw(c, 0);
            c.g[V1][0] = sx(UCS_VT_PROGRESS);
            Addr(obj.wrapping_add(0x78)).sw(c, 0);
            c.g[A1][0] = c.g[S4][0];
            c.g[A0][0] = Addr(UCS_VT_PROGRESS + UCS_VT_INIT).lh(c);
            c.g[V0][0] = Addr(UCS_VT_PROGRESS + UCS_VT_INIT + 4).lw(c);
            Addr(obj.wrapping_add(0x7c)).sw(c, 0);
            c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
            Addr(obj.wrapping_add(0x80)).sw(c, 0);
            Addr(obj.wrapping_add(0x84)).sw(c, 0);
            let method = c.g[V0][0] as u32;
            Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).sw(c, sx(UCS_VT_PROGRESS));
            call_ptr(c, method, 0x0039_d434)?;
            ucs_attach_two(
                c,
                S1,
                S0,
                [0x20, 0x24],
                [0x74, 0x78],
                [0x0039_d440, 0x0039_d454, 0x0039_d468, 0x0039_d47c],
            )
        }
        // cUITextScroll: children named by the byte count at +0x22.
        0x20 => {
            c.cycles += 16;
            c.g[S3][0] = sx(ucs_reg(c, S4).wrapping_add(0x2c));
            ucs_alloc(c, 0xd0, 0x0049_3f50, 0x0039_d658)?;
            c.g[S2][0] = 0;
            ucs_ctor_v0(c, UCS_TEXT_SCROLL_CTOR, 0x0039_d66c)?;
            c.g[S1][0] = c.g[V0][0];
            let rec = ucs_reg(c, S4);
            c.g[V0][0] = Addr(rec.wrapping_add(0x22)).lbu(c);
            if c.g[V0][0] == 0 {
                let obj = ucs_reg(c, S1);
                c.g[V0][0] = Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).lw(c);
                return ucs_init_s1(c);
            }
            c.cycles += 1;
            ucs_adopt_children(
                c,
                UcsCount::Byte(0x22),
                0x8c,
                0x0039_d680,
                [0x0039_d68c, 0x0039_d6a0, 0x0039_d6b0],
            )?;
            ucs_init_s1_load(c)
        }
        // Type 0x21: parts named at +0x20/+0x24 into +0x74/+0x78.
        0x21 => {
            c.cycles += 36;
            ucs_alloc(c, 0xcc, 0x0049_3f30, 0x0039_d4a0)?;
            c.g[S0][0] = c.g[V0][0];
            ucs_ctor(c, UCS_OBJECT_CTOR, S0, 0x0039_d4b4)?;
            let obj = ucs_reg(c, S0);
            Addr(obj.wrapping_add(0x74)).sw(c, 0);
            c.g[V1][0] = sx(UCS_VT_21);
            Addr(obj.wrapping_add(0x78)).sw(c, 0);
            c.g[A1][0] = c.g[S4][0];
            c.g[A0][0] = Addr(UCS_VT_21 + UCS_VT_INIT).lh(c);
            c.g[V0][0] = Addr(UCS_VT_21 + UCS_VT_INIT + 4).lw(c);
            Addr(obj.wrapping_add(0x7c)).sw(c, 0);
            c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
            for off in [0x80, 0x84, 0x88, 0xc0, 0xc4, 0xc8] {
                Addr(obj.wrapping_add(off)).sw(c, 0);
            }
            Addr(obj.wrapping_add(0x8c)).sb(c, 0);
            Addr(obj.wrapping_add(0xa5)).sb(c, 0);
            let method = c.g[V0][0] as u32;
            Addr(obj.wrapping_add(UCS_OBJ_VTABLE)).sw(c, sx(UCS_VT_21));
            call_ptr(c, method, 0x0039_d500)?;
            ucs_attach_two(
                c,
                S0,
                S1,
                [0x20, 0x24],
                [0x74, 0x78],
                [0x0039_d50c, 0x0039_d520, 0x0039_d534, 0x0039_d548],
            )
        }
        // 0x14, 0x1a..=0x1f: no object.
        _ => {
            c.cycles += 1;
            c.g[V0][0] = 0;
            Ok(())
        }
    }
}

/// `cUIThing* cUIScreen::createObjectByStruct(cUIScreen* self, Rec* rec)`
/// (`UIScreen_CreateObject`, 0x39d078): builds the UI object a screen file
/// record describes (type halfword at +0: icon, group, list box, menu,
/// pair, progress bar, text, text scroll, vector, ...), hands it the record
/// (`initFromRec`), and takes the named child objects off the screen's
/// object list into the new object. Returns the object, or 0 for unknown
/// types.
pub fn ui_screen_create_object(c: &mut Cpu) {
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
    let ty = Addr(ucs_reg(c, S4)).lhu(c);
    c.g[V0][0] = ty;
    c.g[A0][0] = sx((ty as u32).wrapping_sub(0x10));
    let known = c.g[A0][0] < 0x12;
    c.g[V1][0] = known as u64;
    c.g[V0][0] = 0x49_0000;
    if known {
        c.cycles += 6;
        let index = c.g[A0][0] as u32;
        c.g[V1][0] = sx(UCS_TABLE + (index << 2));
        c.g[V0][0] = sx(UCS_TABLE);
        // The jump table entry (left in a0 as the compiled code does).
        c.g[A0][0] = Addr(UCS_TABLE + (index << 2)).lw(c);
        if ucs_build(c, 0x10 + index).is_err() {
            return;
        }
    } else {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 9;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{check, fill, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate word for this test's fakes, and the find results they return.
    const UCS_CTRL: u32 = DATA + 0xf_0000;
    const UCS_MAGIC: u32 = 0x0c38_5d07;
    const UCS_FIND_NEXT: u32 = UCS_CTRL + 4;
    const UCS_FIND_RESULTS: u32 = UCS_CTRL + 0x10;
    /// Where the fake `operator new` puts every object.
    const UCS_NEW_OBJ: u32 = DATA + 0x1_0000;
    const UCS_REC: u32 = DATA + 0x2_0000;
    const UCS_CHILDREN: u32 = DATA + 0x3_0000;
    const UCS_SCREEN: u32 = DATA + 0x4_0000;
    /// The vtable the faked constructors leave in the object.
    const UCS_FAKE_VT: u32 = DATA + 0x5_0000;
    /// The jump table's real entries (types 0x10..=0x21).
    const UCS_JUMP: [u32; 18] = [
        0x0039_d124,
        0x0039_d0cc,
        0x0039_d1c4,
        0x0039_d1f4,
        0x0039_d730,
        0x0039_d2c4,
        0x0039_d554,
        0x0039_d60c,
        0x0039_d6e4,
        0x0039_d3d4,
        0x0039_d730,
        0x0039_d730,
        0x0039_d730,
        0x0039_d730,
        0x0039_d730,
        0x0039_d730,
        0x0039_d63c,
        0x0039_d488,
    ];

    const UCS_K_PLAIN: u32 = 0;
    const UCS_K_ALLOC: u32 = 1;
    const UCS_K_THIS: u32 = 2;
    const UCS_K_FIND: u32 = 3;

    /// The logging fake, then a result by kind: `new` returns the fixed
    /// object, constructors return `this`, finds the next fixture result.
    fn ucs_fake(c: &mut Cpu, addr: u32, kind: u32) {
        let this = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match kind {
            UCS_K_ALLOC => c.g[V0][0] = sx(UCS_NEW_OBJ),
            UCS_K_THIS => c.g[V0][0] = this,
            UCS_K_FIND => {
                let i = c.bus.read32(UCS_FIND_NEXT);
                c.bus.write32(UCS_FIND_NEXT, i + 1);
                let found = c.bus.read32(UCS_FIND_RESULTS + 4 * (i & 0xf));
                c.g[V0][0] = sx(found);
            }
            _ => {}
        }
    }

    macro_rules! ucs_fakes {
        ($($name:ident $prev:ident $addr:literal $shard:ident $slot:ident $gen:ident $kind:ident;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(UCS_CTRL) != UCS_MAGIC {
                        return fakes::fall_back(c, &$prev, ee::$shard::$gen);
                    }
                    ucs_fake(c, $addr, $kind)
                }
            )*
            fn ucs_install_fakes() {
                $(
                    fakes::hook(std::ptr::addr_of_mut!(ee::$shard::$slot), $name, &$prev);
                )*
            }
        };
    }

    ucs_fakes! {
        ucs_f_alloc UCS_P_ALLOC 0x0031_7d70 s10 OVERRIDE_00317D70 f_00317d70__generated UCS_K_ALLOC;
        ucs_f_list_ctor UCS_P_LIST_CTOR 0x0039_77e8 s13 OVERRIDE_003977E8 f_003977e8__generated UCS_K_PLAIN;
        ucs_f_find UCS_P_FIND 0x0039_d860 s13 OVERRIDE_0039D860 f_0039d860__generated UCS_K_FIND;
        ucs_f_unlink UCS_P_UNLINK 0x0039_7740 s13 OVERRIDE_00397740 f_00397740__generated UCS_K_PLAIN;
        ucs_f_push_back UCS_P_PUSH_BACK 0x0039_7910 s13 OVERRIDE_00397910 f_00397910__generated UCS_K_PLAIN;
        ucs_f_menu_flags UCS_P_MENU_FLAGS 0x003a_5338 s13 OVERRIDE_003A5338 f_003a5338__generated UCS_K_PLAIN;
        ucs_f_pair_flags UCS_P_PAIR_FLAGS 0x003a_5508 s13 OVERRIDE_003A5508 f_003a5508__generated UCS_K_PLAIN;
        ucs_f_text UCS_P_TEXT 0x003a_0348 s13 OVERRIDE_003A0348 f_003a0348__generated UCS_K_THIS;
        ucs_f_text_scroll UCS_P_TEXT_SCROLL 0x003a_1148 s13 OVERRIDE_003A1148 f_003a1148__generated UCS_K_THIS;
        ucs_f_vector UCS_P_VECTOR 0x003a_3280 s13 OVERRIDE_003A3280 f_003a3280__generated UCS_K_THIS;
    }

    /// Every record type (and unknown ones), child loops of 0..4 entries,
    /// finds that miss, hit children, or hit a pair's own part slots (so the
    /// old part's deleting destructor runs), and unwinds at random calls.
    /// `new` and the constructors are fakes; `UIObject_ctor`, the list box
    /// constructor (with its real memset) and the icon handle run for real.
    #[test]
    fn ui_screen_create_object_matches() {
        ucs_install_fakes();
        check(
            ee::s13::f_0039d078__generated,
            ui_screen_create_object,
            300,
            &|c, seed| {
                ucs_install_fakes();
                let mut r = fakes::machine(c, seed, 0x0039_d078, 14);
                c.bus.write32(UCS_CTRL, UCS_MAGIC);
                c.bus.write32(UCS_FIND_NEXT, 0);
                for i in 0..16u32 {
                    let found = match r.next_u32() % 7 {
                        0 | 1 => 0,
                        2..=5 => UCS_CHILDREN + 0x100 * (r.next_u32() % 4),
                        _ => UCS_NEW_OBJ + 0x38 + 4 * (i & 1),
                    };
                    c.bus.write32(UCS_FIND_RESULTS + 4 * i, found);
                }
                for (i, &e) in UCS_JUMP.iter().enumerate() {
                    c.bus.write32(UCS_TABLE + 4 * i as u32, e);
                }
                // initFromRec and the deleting destructor in every vtable
                // the object can carry.
                let vts = [
                    UCS_VT_ICON,
                    UCS_VT_GROUP,
                    UCS_VT_MENU,
                    UCS_VT_PAIR,
                    UCS_VT_PROGRESS,
                    UCS_VT_21,
                    UCS_VT_16,
                    0x0049_4798,
                    UCS_FAKE_VT,
                ];
                for (i, &vt) in vts.iter().enumerate() {
                    let i = i as u32;
                    c.bus
                        .write16(vt + UCS_VT_INIT, (r.next_u32() & 0xfc) as u16);
                    c.bus.write32(vt + UCS_VT_INIT + 4, FAKE + 0x20 * i);
                    c.bus
                        .write16(vt + UCS_VT_DELETE, (r.next_u32() & 0xfc) as u16);
                    c.bus
                        .write32(vt + UCS_VT_DELETE + 4, FAKE + 0x20 * i + 0x10);
                }
                fill(c, &mut r, UCS_NEW_OBJ, 0xd0);
                c.bus.write32(UCS_NEW_OBJ + UCS_OBJ_VTABLE, UCS_FAKE_VT);
                fill(c, &mut r, UCS_CHILDREN, 0x100);
                fill(c, &mut r, UCS_REC, 0x10);
                let ty: u32 = match seed % 21 {
                    k @ 0..=17 => 0x10 + k,
                    18 => 0x22,
                    19 => 0x3,
                    _ => 0xfff0,
                };
                c.bus.write16(UCS_REC, ty as u16);
                c.bus.write32(UCS_REC + 0x20, (seed / 21) % 5);
                if ty == 0x20 {
                    c.bus.write8(UCS_REC + 0x22, ((seed / 7) % 5) as u8);
                }
                c.g[A0][0] = sx(UCS_SCREEN);
                c.g[A1][0] = sx(UCS_REC);
                c.pc = 0x0039_d078;
            },
        );
    }
}
