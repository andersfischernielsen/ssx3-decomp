//! `src/main/all_in_one_main.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Flow;
use crate::call::Frame;
pub(crate) use crate::game::fe::debugfe::menu_item_dtor;
pub(crate) use crate::game::fe::debugfe::menu_page_dtor;
pub(crate) use crate::game::fe::debugfe::menu_sx_add;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001d_d580, ui_screen_flags_ctor, "UIScreenFlags_ctor"),
    (0x001d_c900, module_base_dtor, "ModuleBase_dtor"),
    (0x001d_c940, lobby_menu_base_dtor, "LobbyMenuBase_dtor"),
    (0x001d_cac0, fe_module_camera_dtor, "FEModuleCamera_dtor"),
    (
        0x001d_caf0,
        c_private_message_dtor_2,
        "cPrivateMessage_dtor_2",
    ),
    (
        0x001d_cd58,
        lobby_menu_message_page_dtor,
        "LobbyMenuMessagePage_dtor",
    ),
];

/// `UIScreenFlags* UIScreenFlags_ctor(UIScreenFlags* self)` (0x001dd580):
/// sets bit 0 and clears bit 1 of the flag halfword at +0 and returns `self`.
pub fn ui_screen_flags_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = c.lhu(c.g[A0][0] as u32);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] |= 0x1;
    c.g[V1][0] &= 0xfffd;
    let ra = c.g[RA][0] as u32;
    c.sh(c.g[A0][0] as u32, c.g[V1][0]);
    c.pc = ra;
}

/// The `delete` of a gcc dtor (0x317e50).
pub(crate) const DELETE_THIS: u32 = 0x0031_7e50;

/// The shape of a gcc `dtor(this, flags)` that stores its vtable at
/// `this + field` and deletes `this` (0x317e50) when `flags & 1`.
pub(crate) fn vtable_dtor(c: &mut Cpu, vtable: u32, field: u32, ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = sx(vtable);
    frame.save_ra(c, 0);
    c.g[A1][0] &= 1;
    let delete = c.g[A1][0] != 0;
    let this = c.g[A0][0] as u32;
    c.sw(this.wrapping_add(field), c.g[V0][0]);
    if delete {
        c.cycles += 2;
        call_ptr(c, DELETE_THIS, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `ModuleBase_dtor` (0x1dc900).
pub fn module_base_dtor(c: &mut Cpu) {
    guest(c, |c| vtable_dtor(c, 0x0046_d970, 0, 0x001d_c924));
}

/// `LobbyMenuBase_dtor` (0x1dc940).
pub fn lobby_menu_base_dtor(c: &mut Cpu) {
    guest(c, |c| vtable_dtor(c, 0x0046_d948, 0, 0x001d_c964));
}

/// `FEModuleCamera_dtor` (0x1dcac0): the vtable is stored at +0x90.
pub fn fe_module_camera_dtor(c: &mut Cpu) {
    guest(c, |c| vtable_dtor(c, 0x0045_b8b0, 0x90, 0x001d_cae4));
}

/// `cPrivateMessage_dtor_2` (0x1dcaf0): stores the vtable at +0xc, the
/// three members at +8, +4 and +0 are destroyed (0x318260 with 2), and the
/// object is deleted (0x317e50) when `flags & 1`.
pub fn c_private_message_dtor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x2740));
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 17, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = c.g[S1][0] as u32;
        c.sw(this.wrapping_add(0xc), c.g[V0][0]);
        c.g[A0][0] = sx(this.wrapping_add(0x8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001d_cb20)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001d_cb2c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001d_cb38)?;
        c.g[S0][0] &= 1;
        let delete = c.g[S0][0] != 0;
        frame.restore(c, 16, 0x20);
        if delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, DELETE_THIS, 0x001d_cb4c)?;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenuMessagePage::~LobbyMenuMessagePage(this, flags)` (0x1dcd58): destroys
/// the message page's item at `+0x130`, then the base page.
pub fn lobby_menu_message_page_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let s0 = c.g[S0][0];
        let s1 = c.g[S1][0];
        menu_item_dtor(c, menu_sx_add(s0, 0x130), 0x001d_cd7c)?;
        menu_page_dtor(c, s0, s1, 0x001d_cd88)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use ssx3_ee as ee;

    use crate::difftest::fakes::machine;

    pub(crate) use crate::game::be::belibrary::tests::OC431_BASE;

    /// Flag halfword with every combination of bits 0 and 1 and random
    /// upper bits.
    #[test]
    fn ui_screen_flags_ctor_matches() {
        check(
            ee::s04::f_001dd580__generated,
            ui_screen_flags_ctor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dd580, 0);
                c.g[A0][0] = (OC431_BASE + 0x100) as u64;
                c.bus.write32(OC431_BASE + 0x100, r.next_u32());
                c.pc = 0x001d_d580;
            },
        );
    }

    use crate::difftest::fakes::covered;

    use crate::difftest::DATA;

    pub(crate) use crate::game::fe::debugfe::tests::oc505_install_fakes;
    pub(crate) use crate::game::fe::debugfe::tests::oc505_machine;

    pub(crate) use crate::game::fe::debugfe::tests::OC506_THIS;

    pub(crate) use crate::game::fe::debugfe::tests::oc506_arm;
    pub(crate) use crate::game::fe::debugfe::tests::oc506_hook_all;

    #[test]
    fn module_base_dtor_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dc900__generated,
            module_base_dtor,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dc9_00, 1);
                c.pc = 0x001d_c900;
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_base_dtor_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dc940__generated,
            lobby_menu_base_dtor,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dc9_40, 1);
                c.pc = 0x001d_c940;
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn fe_module_camera_dtor_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dcac0__generated,
            fe_module_camera_dtor,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dca_c0, 1);
                c.pc = 0x001d_cac0;
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn c_private_message_dtor_2_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dcaf0__generated,
            c_private_message_dtor_2,
            80,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dca_f0, 4);
                c.pc = 0x001d_caf0;
                c.g[A0][0] = (DATA + 0x500) as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_message_page_dtor_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcd58__generated,
            lobby_menu_message_page_dtor,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dcd58, 3);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }
}
