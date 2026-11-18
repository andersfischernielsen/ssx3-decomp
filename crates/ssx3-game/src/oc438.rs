//! Staging module oc438 (wave wave222): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc438`.

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
///
/// `InitLoadScreen_Enter` (0x232568) is ported but not listed: its callee
/// (`cSSXApp_initload`, 0x2282d0) is a huge unported driver with no override
/// slot, so no bounded fixture exists for its return path.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001d_d840, fe_state_char_select_dtor, "cFEStateCharSelect_dtor"),
    (0x0016_cec8, camera_trigger_man_reload_if_global, "CameraTriggerMan_ReloadIf_Global"),
    (0x0016_cef0, camera_trigger_man_reload_if, "CameraTriggerMan_ReloadIf"),
    (0x0016_c3a8, camera_trigger_man_read_version, "CameraTriggerMan_ReadVersion"),
    (0x0014_4af8, be_new_race_int_set_num_extra_riders, "BENewRaceInt_SetNumExtraRiders"),
    (0x0015_2948, be_save_int_get_system_save_size, "BESaveInt_GetSystemSaveSize"),
    (0x0014_bd98, bolts_prepare_game_db, "Bolts_PrepareGameDB"),
    (0x0014_bd78, bolts_load_global_db, "Bolts_LoadGlobalDB"),
    (0x0016_d1b8, camera_trigger_man_update_triggers_mask2, "CameraTriggerMan_UpdateTriggersMask2"),
    (0x0014_4610, rider_anim_dtor, "RiderAnim_dtor"),
    (0x0024_4408, value_pair_set_second, "ValuePair_SetSecond"),
    (0x0024_4400, value_pair_set_first, "ValuePair_SetFirst"),
    (0x0022_6620, luno_vm_register_as_id, "LunoVMRegister_AsId"),
    (0x0028_2f80, script_binding_is_bound, "ScriptBinding_IsBound"),
    (0x001d_cc40, fe_locale_init_flags, "FELocale_InitFlags"),
    (0x0031_50b0, anim_data_empty_hook, "AnimData_EmptyHook"),
    (0x0028_2da8, c_transition_man_script_set_owner, "cTransitionMan_ScriptSetOwner"),
    (0x0028_1160, script_view_obj_set_view_index, "ScriptViewObj_SetViewIndex"),
    (0x0027_d680, script_obj_cam_set_owner, "ScriptObjCam_SetOwner"),
];

/// `void cFEStateCharSelect::~cFEStateCharSelect(this)` (0x1dd840): installs
/// the state's vtable (0x46d0d0) at `this+8`, then runs the `UIState`
/// destructor (0x39e390).
pub fn fe_state_char_select_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x47_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
        {
            let v = c.g[V0][0];
            Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, v);
        }
        call_ptr(c, 0x0039_e390, 0x001d_d85c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerMan_ReloadIf_Global(void* file, int flag)` (0x16cec8):
/// `CameraTriggerMan_ReloadIf(&gCameraTriggerMan (0x4c5830), file, flag)`.
pub fn camera_trigger_man_reload_if_global(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[A2][0] = c.g[A1][0];
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x4c_0000;
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5830));
        RELOAD_IF.call(c, 0x0016_cee4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraTriggerMan_ReloadIf` (0x16cef0).
pub(crate) const RELOAD_IF: Callee = Callee {
    addr: 0x0016_cef0,
    f: ee::s02::CameraTriggerMan_ReloadIf,
};

/// `void CameraTriggerMan_ReloadIf(CameraTriggerMan* man, void* file, int flag)`
/// (0x16cef0): `CameraTriggerMan_Reload(man, flag)` when `flag != 0`.
pub fn camera_trigger_man_reload_if(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A2][0];
        let skip = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if !skip {
            c.cycles += 2;
            Callee {
                addr: 0x0016_ce30,
                f: ee::s02::CameraTriggerMan_Reload,
            }
            .call(c, 0x0016_cf08)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerMan_ReadVersion(CameraTriggerList* list, Stream* s)`
/// (0x16c3a8): `Serialize_ReadU32_2(s, &list->version (+0xc))`.
pub fn camera_trigger_man_read_version(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d_2558, 0x0016_c3c0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BENewRaceInt_SetNumExtraRiders(int a0_unused, int n)` (0x144af8):
/// the new-race record at 0x535bc8 stores `extra = n` (+0x38) and
/// `total = base (+0x34) + local (+0x3c) + n` (+0x30).
pub fn be_new_race_int_set_num_extra_riders(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V1][0] = sx(0x53_0000u32.wrapping_add(0x5bc8));
    let rec = c.g[V1][0] as u32;
    c.g[V0][0] = Addr(rec.wrapping_add(0x34)).lw(c);
    c.g[A0][0] = Addr(rec.wrapping_add(0x3c)).lw(c);
    {
        let v = c.g[A1][0];
        Addr(rec.wrapping_add(0x38)).sw(c, v);
    }
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    {
        let v = c.g[V0][0];
        Addr(rec.wrapping_add(0x30)).sw(c, v);
    }
    ret(c);
}

/// `int BESaveInt_GetSystemSaveSize(void)` (0x152948): 0xccc.
pub fn be_save_int_get_system_save_size(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xccc;
    ret(c);
}

/// `void Bolts_PrepareGameDB(void)` (0x14bd98): `Bolts_PrepareForGame(0x4a6750)`.
pub fn bolts_prepare_game_db(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x4a_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6750));
        Callee {
            addr: 0x0014_d068,
            f: ee::s01::Bolts_PrepareForGame,
        }
        .call(c, 0x0014_bdac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Bolts_LoadGlobalDB(void)` (0x14bd78): `Bolts_LoadDB(0x4a6750)`.
pub fn bolts_load_global_db(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x4a_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6750));
        Callee {
            addr: 0x0014_c6a0,
            f: ee::s01::Bolts_LoadDB,
        }
        .call(c, 0x0014_bd8c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTriggerMan_UpdateTriggersMask2(CameraTriggerMan* man)`
/// (0x16d1b8): `CameraTriggerMan_UpdateTriggers(man, 2)`.
pub fn camera_trigger_man_update_triggers_mask2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x2;
        Callee {
            addr: 0x0016_cf48,
            f: ee::s02::CameraTriggerMan_UpdateTriggers,
        }
        .call(c, 0x0016_d1c8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cRiderAnim::~cRiderAnim(this)` (0x144610): runs the
/// `AnimController` destructor (0x311958) on `this`.
pub fn rider_anim_dtor(c: &mut Cpu) {
    crate::call::tail_wrapper(c, None, crate::call::Target::Table(0x0031_1958), 0x0014_4620);
}

/// `void ValuePair_SetSecond(ValuePair* p, int v)` (0x244408): `p->second = v`.
pub fn value_pair_set_second(c: &mut Cpu) {
    c.cycles += 2;
    {
        let v = c.g[A1][0];
        Addr((c.g[A0][0] as u32).wrapping_add(0x4)).sw(c, v);
    }
    ret(c);
}

/// `void ValuePair_SetFirst(ValuePair* p, int v)` (0x244400): `p->first = v`.
pub fn value_pair_set_first(c: &mut Cpu) {
    c.cycles += 2;
    {
        let v = c.g[A1][0];
        Addr(c.g[A0][0] as u32).sw(c, v);
    }
    ret(c);
}

/// `LunoVMRegister LunoVMRegister_AsId(id)` (0x226620): returns its argument.
pub fn luno_vm_register_as_id(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `bool ScriptBinding_IsBound(ScriptBinding* b)` (0x282f80): `b->id (+8) != 0`.
pub fn script_binding_is_bound(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lw(c);
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    ret(c);
}

/// `tAttrLocale* FELocale_InitFlags(tAttrLocale* p)` (0x1dcc40): keeps only
/// the top three bits (0xe0) of the flag byte at `p`; returns `p`.
pub fn fe_locale_init_flags(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lbu(c);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] &= 0xe0;
    {
        let v = c.g[V1][0];
        Addr(c.g[A0][0] as u32).sb(c, v);
    }
    ret(c);
}

/// `void AnimData_EmptyHook(void)` (0x3150b0): does nothing.
pub fn anim_data_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void cTransitionMan_ScriptSetOwner(cTransitionMan* m, int owner)`
/// (0x282da8): `m->owner (+0x10) = owner`.
pub fn c_transition_man_script_set_owner(c: &mut Cpu) {
    c.cycles += 2;
    {
        let v = c.g[A1][0];
        Addr((c.g[A0][0] as u32).wrapping_add(0x10)).sw(c, v);
    }
    ret(c);
}

/// `void ScriptViewObj_SetViewIndex(ScriptViewObj* o, int i)` (0x281160):
/// `o->view_index (+0xf8) = i`.
pub fn script_view_obj_set_view_index(c: &mut Cpu) {
    c.cycles += 2;
    {
        let v = c.g[A1][0];
        Addr((c.g[A0][0] as u32).wrapping_add(0xf8)).sw(c, v);
    }
    ret(c);
}

/// `void ScriptObjCam_SetOwner(ScriptObjCam* o, int owner)` (0x27d680):
/// `o->owner (+0x10) = owner`.
pub fn script_obj_cam_set_owner(c: &mut Cpu) {
    c.cycles += 2;
    {
        let v = c.g[A1][0];
        Addr((c.g[A0][0] as u32).wrapping_add(0x10)).sw(c, v);
    }
    ret(c);
}

/// `void cInitLoadScreen::load(this)` (0x232568): `cSSXApp_initload(SSXApp)`
/// with `SSXApp = *(gp - 0x848)`. Not installed (see `OVERRIDES`): the callee
/// is a huge unported driver with no slot to fake, so the return path has no
/// bounded fixture.
#[allow(dead_code)]
pub fn init_load_screen_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0022_82d0, 0x0023_257c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, fake, fall_back, hook, machine};
    use crate::difftest::{check, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (in the machine's own RAM).
    const OC438_CTRL: u32 = DATA + 0xf_fe00;
    const OC438_MAGIC: u32 = 0x4f43_4438;

    macro_rules! oc438_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC438_CTRL) == OC438_MAGIC {
                    let ra = c.g[RA][0] as u32;
                    return fake(c, $addr, ra);
                }
                fall_back(c, &$prev, $gen)
            }
        };
    }

    oc438_gated!(oc438_fake_ui_state_dtor, OC438_PREV_UI_STATE_DTOR, 0x0039_e390, ee::s13::f_0039e390__generated);
    oc438_gated!(oc438_fake_anim_ctrl_dtor, OC438_PREV_ANIM_CTRL_DTOR, 0x0031_1958, ee::s10::f_00311958__generated);
    oc438_gated!(oc438_fake_read_u32, OC438_PREV_READ_U32, 0x002d_2558, ee::s09::f_002d2558__generated);
    oc438_gated!(oc438_fake_bolts_load_db, OC438_PREV_BOLTS_LOAD_DB, 0x0014_c6a0, ee::s01::f_0014c6a0__generated);
    oc438_gated!(oc438_fake_bolts_prepare, OC438_PREV_BOLTS_PREPARE, 0x0014_d068, ee::s01::f_0014d068__generated);
    oc438_gated!(oc438_fake_update_triggers, OC438_PREV_UPDATE_TRIGGERS, 0x0016_cf48, ee::s02::f_0016cf48__generated);
    oc438_gated!(oc438_fake_ctm_unload, OC438_PREV_CTM_UNLOAD, 0x0016_c9b0, ee::s02::f_0016c9b0__generated);
    oc438_gated!(oc438_fake_ctm_parse, OC438_PREV_CTM_PARSE, 0x0016_c228, ee::s02::f_0016c228__generated);

    fn oc438_install_fakes() {
        hook(addr_of_mut!(ee::s13::OVERRIDE_0039E390), oc438_fake_ui_state_dtor, &OC438_PREV_UI_STATE_DTOR);
        hook(addr_of_mut!(ee::s10::OVERRIDE_00311958), oc438_fake_anim_ctrl_dtor, &OC438_PREV_ANIM_CTRL_DTOR);
        hook(addr_of_mut!(ee::s09::OVERRIDE_002D2558), oc438_fake_read_u32, &OC438_PREV_READ_U32);
        hook(addr_of_mut!(ee::s01::OVERRIDE_0014C6A0), oc438_fake_bolts_load_db, &OC438_PREV_BOLTS_LOAD_DB);
        hook(addr_of_mut!(ee::s01::OVERRIDE_0014D068), oc438_fake_bolts_prepare, &OC438_PREV_BOLTS_PREPARE);
        hook(addr_of_mut!(ee::s02::OVERRIDE_0016CF48), oc438_fake_update_triggers, &OC438_PREV_UPDATE_TRIGGERS);
        hook(addr_of_mut!(ee::s02::OVERRIDE_0016C9B0), oc438_fake_ctm_unload, &OC438_PREV_CTM_UNLOAD);
        hook(addr_of_mut!(ee::s02::OVERRIDE_0016C228), oc438_fake_ctm_parse, &OC438_PREV_CTM_PARSE);
    }

    /// Fixture of a call-making function: scrambled machine, gate on, `pc` at
    /// the entry; `calls` is how many callees it makes.
    fn oc438_machine(c: &mut Cpu, seed: u32, entry: u32, calls: u32) -> Rng {
        let r = machine(c, seed, entry as u64, calls);
        c.pc = entry;
        c.bus.write32(OC438_CTRL, OC438_MAGIC);
        r
    }

    /// A `CameraTriggerMan` at `man`: trigger count at +4 (zero, negative or
    /// 1-3), pointer array at +0 to objects in the data area.
    fn oc438_man(c: &mut Cpu, r: &mut Rng, man: u32) {
        let count = match r.next_u32() % 4 {
            0 => 0,
            1 => (-(r.next_u32() as i32 % 3)) as u32,
            _ => 1 + r.next_u32() % 3,
        };
        let array = DATA + 0x2000;
        c.bus.write32(man, array);
        c.bus.write32(man + 4, count);
        for k in 0..4 {
            c.bus.write32(array + 4 * k, DATA + 0x3000 + 0x100 * k);
            c.bus.write32(DATA + 0x3000 + 0x100 * k + 0x20, r.next_u32());
        }
    }

    /// `cFEStateCharSelect_dtor`: stores the vtable at `this+8`, calls the
    /// `UIState` dtor (faked, sometimes unwinding).
    #[test]
    fn fe_state_char_select_dtor_matches() {
        oc438_install_fakes();
        covered(ee::s04::f_001dd840__generated, fe_state_char_select_dtor, 200, &|c, seed| {
            let mut r = oc438_machine(c, seed, 0x001d_d840, 1);
            c.g[A0][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
        });
    }

    /// `CameraTriggerMan_ReloadIf_Global`: reloads the global manager
    /// (0x4c5830) when the flag is non-zero; Unload, Parse and UpdateTriggers
    /// are faked.
    #[test]
    fn camera_trigger_man_reload_if_global_matches() {
        oc438_install_fakes();
        covered(ee::s02::f_0016cec8__generated, camera_trigger_man_reload_if_global, 200, &|c, seed| {
            let mut r = oc438_machine(c, seed, 0x0016_cec8, 3);
            oc438_man(c, &mut r, 0x004c_5830);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = if seed % 4 == 0 { 0 } else { r.next_u32() as u64 | 1 };
        });
    }

    /// `CameraTriggerMan_ReloadIf`: the flag in a2 off (no call) or on
    /// (`Reload`: Unload, Parse, a loop over the triggers, UpdateTriggers).
    #[test]
    fn camera_trigger_man_reload_if_matches() {
        oc438_install_fakes();
        covered(ee::s02::f_0016cef0__generated, camera_trigger_man_reload_if, 200, &|c, seed| {
            let mut r = oc438_machine(c, seed, 0x0016_cef0, 3);
            let man = DATA + 0x100;
            oc438_man(c, &mut r, man);
            c.g[A0][0] = man as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = if seed % 4 == 0 { 0 } else { r.next_u32() as u64 | 1 };
        });
    }

    /// `CameraTriggerMan_ReadVersion`: `Serialize_ReadU32_2(s, &list->version)`.
    #[test]
    fn camera_trigger_man_read_version_matches() {
        oc438_install_fakes();
        covered(ee::s02::f_0016c3a8__generated, camera_trigger_man_read_version, 200, &|c, seed| {
            let mut r = oc438_machine(c, seed, 0x0016_c3a8, 1);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    /// `BENewRaceInt_SetNumExtraRiders`: record at 0x535bc8 with random
    /// counts.
    #[test]
    fn be_new_race_int_set_num_extra_riders_matches() {
        check(ee::s01::f_00144af8__generated, be_new_race_int_set_num_extra_riders, 200, &|c, seed| {
            let mut r = Rng(0x4a1 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0014_4af8;
            c.bus.write32(0x0053_5bc8 + 0x34, r.next_u32());
            c.bus.write32(0x0053_5bc8 + 0x3c, r.next_u32());
            c.bus.write32(0x0053_5bc8 + 0x30, r.next_u32());
        });
    }

    /// `BESaveInt_GetSystemSaveSize`: constant 0xccc.
    #[test]
    fn be_save_int_get_system_save_size_matches() {
        check(ee::s01::f_00152948__generated, be_save_int_get_system_save_size, 20, &|c, seed| {
            let mut r = Rng(0x5a ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0015_2948;
        });
    }

    /// `Bolts_PrepareGameDB`: forwards to `Bolts_PrepareForGame` (faked).
    #[test]
    fn bolts_prepare_game_db_matches() {
        oc438_install_fakes();
        covered(ee::s01::f_0014bd98__generated, bolts_prepare_game_db, 200, &|c, seed| {
            oc438_machine(c, seed, 0x0014_bd98, 1);
        });
    }

    /// `Bolts_LoadGlobalDB`: forwards to `Bolts_LoadDB` (faked).
    #[test]
    fn bolts_load_global_db_matches() {
        oc438_install_fakes();
        covered(ee::s01::f_0014bd78__generated, bolts_load_global_db, 200, &|c, seed| {
            oc438_machine(c, seed, 0x0014_bd78, 1);
        });
    }

    /// `CameraTriggerMan_UpdateTriggersMask2`: `UpdateTriggers(man, 2)`.
    #[test]
    fn camera_trigger_man_update_triggers_mask2_matches() {
        oc438_install_fakes();
        covered(ee::s02::f_0016d1b8__generated, camera_trigger_man_update_triggers_mask2, 200, &|c, seed| {
            let mut r = oc438_machine(c, seed, 0x0016_d1b8, 1);
            c.g[A0][0] = r.next_u32() as u64;
        });
    }

    /// `RiderAnim_dtor`: forwards to the `AnimController` dtor (faked).
    #[test]
    fn rider_anim_dtor_matches() {
        oc438_install_fakes();
        covered(ee::s01::f_00144610__generated, rider_anim_dtor, 200, &|c, seed| {
            let mut r = oc438_machine(c, seed, 0x0014_4610, 1);
            c.g[A0][0] = r.next_u32() as u64;
        });
    }

    /// Leaf setters and getters: random registers, argument pointing at the
    /// data area, struct contents random.
    fn oc438_leaf(gen: GuestFn, hand: GuestFn, entry: u32) {
        check(gen, hand, 200, &|c, seed| {
            let mut r = Rng(0x1eaf ^ entry as u64 ^ (seed as u64) << 32);
            scramble(c, &mut r);
            c.pc = entry;
            let p = DATA + (r.next_u32() & 0xf_ff00 & !3);
            c.g[A0][0] = p as u64;
            for k in 0..0x40 {
                c.bus.write32(p + 4 * k, r.next_u32());
            }
            // Flag bytes and ids: zero, small, negative, any.
            let word = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => 0x8000_0000,
                _ => r.next_u32(),
            };
            c.bus.write32(p + 8, word);
            c.bus.write32(p, word ^ 0xe5);
        });
    }

    #[test]
    fn value_pair_set_second_matches() {
        oc438_leaf(ee::s06::f_00244408__generated, value_pair_set_second, 0x0024_4408);
    }

    #[test]
    fn value_pair_set_first_matches() {
        oc438_leaf(ee::s06::f_00244400__generated, value_pair_set_first, 0x0024_4400);
    }

    #[test]
    fn luno_vm_register_as_id_matches() {
        oc438_leaf(ee::s05::f_00226620__generated, luno_vm_register_as_id, 0x0022_6620);
    }

    /// `ScriptBinding_IsBound`: `+8` zero, one, negative or random.
    #[test]
    fn script_binding_is_bound_matches() {
        oc438_leaf(ee::s07::f_00282f80__generated, script_binding_is_bound, 0x0028_2f80);
    }

    /// `FELocale_InitFlags`: the byte at `p` is random (word is random).
    #[test]
    fn fe_locale_init_flags_matches() {
        oc438_leaf(ee::s04::f_001dcc40__generated, fe_locale_init_flags, 0x001d_cc40);
    }

    #[test]
    fn anim_data_empty_hook_matches() {
        oc438_leaf(ee::s10::f_003150b0__generated, anim_data_empty_hook, 0x0031_50b0);
    }

    #[test]
    fn c_transition_man_script_set_owner_matches() {
        oc438_leaf(ee::s07::f_00282da8__generated, c_transition_man_script_set_owner, 0x0028_2da8);
    }

    #[test]
    fn script_view_obj_set_view_index_matches() {
        oc438_leaf(ee::s07::f_00281160__generated, script_view_obj_set_view_index, 0x0028_1160);
    }

    #[test]
    fn script_obj_cam_set_owner_matches() {
        oc438_leaf(ee::s07::f_0027d680__generated, script_obj_cam_set_owner, 0x0027_d680);
    }
}
