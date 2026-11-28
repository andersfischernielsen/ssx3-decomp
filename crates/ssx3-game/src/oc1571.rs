//! Staging module oc1571 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1571`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_1cc0, world_create_dead_node, "World_CreateDeadNode"),
    (
        0x002d_1cf0,
        world_get_avalanche_matrix,
        "World_GetAvalancheMatrix",
    ),
    (
        0x002d_3a68,
        menu_sys_bool_item_dtor,
        "MenuSysBoolItem_dtor",
    ),
    (
        0x002d_3a98,
        menu_sys_enum_item_dtor,
        "MenuSysEnumItem_dtor",
    ),
    (
        0x002d_3ae0,
        menu_sys_submenu_item_dtor,
        "MenuSysSubmenuItem_dtor",
    ),
    (0x002d_3b10, menu_sys_back_item_dtor, "MenuSysBackItem_dtor"),
    (
        0x002d_3b38,
        menu_sys_action_item_dtor,
        "MenuSysActionItem_dtor",
    ),
    (
        0x002d_3b80,
        menu_sys_callback_item_dtor,
        "MenuSysCallbackItem_dtor",
    ),
];

/// `void* World_CreateDeadNode(void* a0, void* a1)` (0x002d1cc0): forwards its
/// arguments unchanged to `WorldObjFactory_DeadNode` (0x002fc2c0), called
/// through the function table, in a 0x10 frame holding `ra`.
pub fn world_create_dead_node(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Table(0x002f_c2c0),
        0x002d_1cd0,
    );
}

/// `void* World_GetAvalancheMatrix(void* a0, void* a1)` (0x002d1cf0): forwards
/// its arguments unchanged to `Avalanche_GetObjectMatrix` (0x002d9c00) in a
/// 0x10 frame holding `ra`.
pub fn world_get_avalanche_matrix(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(crate::call::Callee {
            addr: 0x002d_9c00,
            f: ee::s09::f_002d9c00,
        }),
        0x002d_1d00,
    );
}

/// The forwarder every `MenuSys*Item` destructor thunk shares: its arguments
/// (the object and the deleting flag) go unchanged to `MenuSysItem_dtor`
/// (0x002ca280) in a 0x10 frame holding `ra`. `ret` is the address after the
/// call in the thunk (`back`).
fn menu_sys_item_dtor_forward(c: &mut Cpu, back: u32) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(crate::call::Callee {
            addr: 0x002c_a280,
            f: ee::s09::f_002ca280,
        }),
        back,
    );
}

/// Destructor thunk of the bool menu item (0x002d3a68): `MenuSysItem_dtor`.
pub fn menu_sys_bool_item_dtor(c: &mut Cpu) {
    menu_sys_item_dtor_forward(c, 0x002d_3a78);
}

/// Destructor thunk of the enum menu item (0x002d3a98): `MenuSysItem_dtor`.
pub fn menu_sys_enum_item_dtor(c: &mut Cpu) {
    menu_sys_item_dtor_forward(c, 0x002d_3aa8);
}

/// Destructor thunk of the submenu item (0x002d3ae0): `MenuSysItem_dtor`.
pub fn menu_sys_submenu_item_dtor(c: &mut Cpu) {
    menu_sys_item_dtor_forward(c, 0x002d_3af0);
}

/// Destructor thunk of the back item (0x002d3b10): `MenuSysItem_dtor`.
pub fn menu_sys_back_item_dtor(c: &mut Cpu) {
    menu_sys_item_dtor_forward(c, 0x002d_3b20);
}

/// Destructor thunk of the action item (0x002d3b38): `MenuSysItem_dtor`.
pub fn menu_sys_action_item_dtor(c: &mut Cpu) {
    menu_sys_item_dtor_forward(c, 0x002d_3b48);
}

/// Destructor thunk of the callback item (0x002d3b80): `MenuSysItem_dtor`.
pub fn menu_sys_callback_item_dtor(c: &mut Cpu) {
    menu_sys_item_dtor_forward(c, 0x002d_3b90);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate words (one per fake) in the machine's RAM: a fake answers only
    /// when its own word holds its own magic, else it falls back.
    const OC1571_DTOR_GATE: u32 = DATA + 0x5_7110;
    const OC1571_DEAD_NODE_GATE: u32 = DATA + 0x5_7114;
    const OC1571_AVALANCHE_GATE: u32 = DATA + 0x5_7118;
    const OC1571_DTOR_MAGIC: u32 = 0x1571_0001;
    const OC1571_DEAD_NODE_MAGIC: u32 = 0x1571_0002;
    const OC1571_AVALANCHE_MAGIC: u32 = 0x1571_0003;

    static OC1571_PREV_DTOR: AtomicUsize = AtomicUsize::new(0);
    static OC1571_PREV_DEAD_NODE: AtomicUsize = AtomicUsize::new(0);
    static OC1571_PREV_AVALANCHE: AtomicUsize = AtomicUsize::new(0);

    /// The gated fake of `MenuSysItem_dtor`: logs the call and returns to `ra`.
    fn oc1571_fake_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1571_DTOR_GATE) != OC1571_DTOR_MAGIC {
            return fakes::fall_back(c, &OC1571_PREV_DTOR, ee::s09::f_002ca280__generated);
        }
        fakes::fake_call(c, 0x002c_a280);
    }

    /// The gated fake of `WorldObjFactory_DeadNode`.
    fn oc1571_fake_dead_node(c: &mut Cpu) {
        if c.bus.read32(OC1571_DEAD_NODE_GATE) != OC1571_DEAD_NODE_MAGIC {
            return fakes::fall_back(
                c,
                &OC1571_PREV_DEAD_NODE,
                ee::s10::f_002fc2c0__generated,
            );
        }
        fakes::fake_call(c, 0x002f_c2c0);
    }

    /// The gated fake of `Avalanche_GetObjectMatrix`.
    fn oc1571_fake_avalanche(c: &mut Cpu) {
        if c.bus.read32(OC1571_AVALANCHE_GATE) != OC1571_AVALANCHE_MAGIC {
            return fakes::fall_back(c, &OC1571_PREV_AVALANCHE, ee::s09::f_002d9c00__generated);
        }
        fakes::fake_call(c, 0x002d_9c00);
    }

    fn oc1571_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA280),
            oc1571_fake_dtor,
            &OC1571_PREV_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_002FC2C0),
            oc1571_fake_dead_node,
            &OC1571_PREV_DEAD_NODE,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D9C00),
            oc1571_fake_avalanche,
            &OC1571_PREV_AVALANCHE,
        );
    }

    /// A destructor thunk against the generated one: the arguments are
    /// scrambled and pass through, the fake `MenuSysItem_dtor` returns to
    /// `ra`, and `machine` (calls = 1) makes some seeds unwind at the call.
    fn oc1571_dtor_check(generated: fn(&mut Cpu), port: fn(&mut Cpu), salt: u64) {
        oc1571_install_fakes();
        check(generated, port, 200, &|c, seed| {
            machine(c, seed, salt, 1);
            c.bus.write32(OC1571_DTOR_GATE, OC1571_DTOR_MAGIC);
        });
    }

    #[test]
    fn world_create_dead_node_matches() {
        oc1571_install_fakes();
        check(
            ee::s09::f_002d1cc0__generated,
            world_create_dead_node,
            200,
            &|c, seed| {
                machine(c, seed, 0x002d_1cc0, 1);
                c.bus.write32(OC1571_DEAD_NODE_GATE, OC1571_DEAD_NODE_MAGIC);
            },
        );
    }

    #[test]
    fn world_get_avalanche_matrix_matches() {
        oc1571_install_fakes();
        check(
            ee::s09::f_002d1cf0__generated,
            world_get_avalanche_matrix,
            200,
            &|c, seed| {
                machine(c, seed, 0x002d_1cf0, 1);
                c.bus.write32(OC1571_AVALANCHE_GATE, OC1571_AVALANCHE_MAGIC);
            },
        );
    }

    #[test]
    fn menu_sys_bool_item_dtor_matches() {
        oc1571_dtor_check(
            ee::s09::f_002d3a68__generated,
            menu_sys_bool_item_dtor,
            0x002d_3a68,
        );
    }

    #[test]
    fn menu_sys_enum_item_dtor_matches() {
        oc1571_dtor_check(
            ee::s09::f_002d3a98__generated,
            menu_sys_enum_item_dtor,
            0x002d_3a98,
        );
    }

    #[test]
    fn menu_sys_submenu_item_dtor_matches() {
        oc1571_dtor_check(
            ee::s09::f_002d3ae0__generated,
            menu_sys_submenu_item_dtor,
            0x002d_3ae0,
        );
    }

    #[test]
    fn menu_sys_back_item_dtor_matches() {
        oc1571_dtor_check(
            ee::s09::f_002d3b10__generated,
            menu_sys_back_item_dtor,
            0x002d_3b10,
        );
    }

    #[test]
    fn menu_sys_action_item_dtor_matches() {
        oc1571_dtor_check(
            ee::s09::f_002d3b38__generated,
            menu_sys_action_item_dtor,
            0x002d_3b38,
        );
    }

    #[test]
    fn menu_sys_callback_item_dtor_matches() {
        oc1571_dtor_check(
            ee::s09::f_002d3b80__generated,
            menu_sys_callback_item_dtor,
            0x002d_3b80,
        );
    }
}
