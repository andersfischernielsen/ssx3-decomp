//! `src/fe/fememcard.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::dirtysock::tagsunk::at_3 as at;
pub(crate) use crate::game::dirtysock::tagsunk::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tagsunk::sw;
pub(crate) use crate::game::fe::feasyncfile::fe_ctor_with_vtable;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_3;
pub(crate) use crate::game::fe::festatecredits::oc830_ea;
pub(crate) use crate::game::fe::festatecredits::oc830_gp_lw;
pub(crate) use crate::game::fe::festatecredits::oc830_sx_sub;
pub(crate) use crate::game::fe::festateoptions::oc832_gp_lw;
pub(crate) use crate::game::fe::festateoptions::oc832_gp_sw;
pub(crate) use crate::game::fe::festateoptions::oc832_lh;
pub(crate) use crate::game::fe::festateoptions::oc832_lw;
pub(crate) use crate::game::fe::festateoptions::oc832_sw;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_2;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_6b50,
        c_fe_state_mem_card_init,
        "cFEStateMemCard_Init",
    ),
    (0x0018_6bf0, cfe_mem_card_update, "cFEStateMemCard_Update"),
    (
        0x0018_6c98,
        cfe_state_mem_card_show_not_formatted_msg,
        "cFEStateMemCard_ShowNotFormattedMsg",
    ),
    (
        0x0018_6f40,
        c_fe_state_mem_card_is_scan_done_with_card,
        "cFEStateMemCard_IsScanDoneWithCard",
    ),
    (
        0x0018_7148,
        cfe_state_mem_card_close_and_set_state6,
        "cFEStateMemCard_CloseAndSetState6",
    ),
    (
        0x0018_7180,
        fe_state_memcard_update_card_scan,
        "cFEStateMemCard_UpdateCardScan",
    ),
    (
        0x0018_71a8,
        c_fe_state_mem_card_restart_scan,
        "cFEStateMemCard_RestartScan",
    ),
    (
        0x0018_7230,
        c_fe_state_mem_card_reset_scan,
        "cFEStateMemCard_ResetScan",
    ),
    (
        0x0018_7270,
        cfe_mem_card_on_scan_done,
        "cFEStateMemCard_OnScanDone",
    ),
    (
        0x0018_7318,
        fe_state_mem_card_finish,
        "cFEStateMemCard_Finish",
    ),
    (0x0018_74e8, fe_state_mem_card_v33, "cFEStateMemCard_v33"),
    (
        0x0018_7a38,
        c_fe_state_mem_card_scan_step,
        "cFEStateMemCard_ScanStep",
    ),
    (
        0x001d_e950,
        c_fe_state_mem_card_ctor_2,
        "cFEStateMemCard_ctor_2",
    ),
    (0x0018_76a8, cfe_state_mem_card_v26, "cFEStateMemCard_v26"),
    (
        0x0018_6f90,
        c_fe_state_mem_card_check_free_space,
        "cFEStateMemCard_CheckFreeSpace",
    ),
    (
        0x0018_77b0,
        fe_state_mem_card_start_profile_search,
        "cFEStateMemCard_StartProfileSearch",
    ),
    (
        0x0018_6d60,
        cfe_state_mem_card_show_not_detected_msg,
        "cFEStateMemCard_ShowNotDetectedMsg",
    ),
    (
        0x0018_7360,
        cfe_state_mem_card_show_no_space_msg,
        "cFEStateMemCard_ShowNoSpaceMsg",
    ),
    (
        0x0018_7580,
        cfe_state_mem_card_on_child_event,
        "cFEStateMemCard_OnChildEvent",
    ),
    (
        0x0018_7920,
        cfe_state_mem_card_start_system_load,
        "cFEStateMemCard_StartSystemLoad",
    ),
    (
        0x0018_7c18,
        fe_screen_autosave_popup_ctor,
        "FEScreenAutosavePopup_ctor",
    ),
    (
        0x0018_7c10,
        c_fe_state_mem_card_handle_input_00187c10,
        "cFEStateMemCard_HandleInput",
    ),
    (
        0x001d_e940,
        c_fe_state_mem_card_on_widget_action_001de940,
        "cFEStateMemCard_OnWidgetAction",
    ),
    (
        0x001d_e948,
        c_fe_state_mem_card_v37_001de948,
        "cFEStateMemCard_v37",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::MEMCARD_SCAN_STEP;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SSX_APP_START_FE_MODULE;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;

    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;

    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;

    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {}

// ---------------------------------------------------------------------------
// cFEStateMemCard_UpdateCardScan.

/// `void cFEStateMemCard_UpdateCardScan(cFEStateMemCard* s)` (0x187180): poll
/// the memory cards, then advance the scan at `D_004A14B8`.
pub fn fe_state_memcard_update_card_scan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x001d_83a8, 0x0018_7190)?;
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x1c38));
        callee::MEMCARD_SCAN_STEP.call(c, 0x0018_7198)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMemCard_CloseAndSetState6(void)` (0x187148): call vtable slot
/// 0x148 of the current front-end state, then move it to state 6.
pub fn cfe_state_mem_card_close_and_set_state6(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1c38)).lw(c);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(8).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x148).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x14c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_7168)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1c38)).lw(c);
        c.g[V0][0] = 6;
        frame.restore_ra(c, 0);
        let a0 = c.g[A0][0];
        let v0 = c.g[V0][0];
        Addr(a0 as u32).off(0x1c0).sw(c, v0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMemCard_ResetScan(void)` (0x187230): restart the memory
/// card manager's scan (mode 2) with no slot selected.
pub fn c_fe_state_mem_card_reset_scan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        let g = crate::abi::r32(c, GP);
        c.g[A0][0] = lw(c, g, 0xfffff7b8); // gp - 0x848
        frame.save_ra(c, 0);
        callee::MC_MANAGER_GET.call(c, 0x0018_7244)?;
        let g = crate::abi::r32(c, GP);
        let mgr = lw(c, g, 0xffffe3c8); // gp - 0x1c38
        c.g[A2][0] = mgr;
        c.g[V1][0] = u64::MAX;
        Addr(crate::abi::r32(c, GP).wrapping_sub(0x1c34)).sw(c, 0);
        c.g[A0][0] = c.g[V0][0];
        Addr(p(mgr as u32, 0x22c)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 2;
        Addr(p(mgr as u32, 0x234)).sw(c, 0);
        callee::MC_MANAGER_BEGIN_LOAD.call(c, 0x0018_7264)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMemCard_Finish(cFEStateMemCard* self)` (0x187318): ask the
/// front end to start its module once (when the "started" flag is clear),
/// then mark the state started.
pub fn fe_state_mem_card_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 6;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        sw(c, r32(c, S0), 0x1c0, c.g[V0][0]);
        c.g[V1][0] = c.lw(at(c, S0, 0x1ac));
        let already = c.g[V1][0] != 0;
        c.g[V0][0] = 1;
        if !already {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gp(c, -0x848));
            call_ptr(c, callee::SSX_APP_START_FE_MODULE, 0x0018_7344)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        sw(c, r32(c, S0), 0x1ac, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
        Ok(())
    });
}

/// `bool cFEStateMemCard_IsScanDoneWithCard(self)` (0x186f40): false until the
/// memory-card manager is in a scan state (>= 4) and reports the card ready.
pub fn c_fe_state_mem_card_is_scan_done_with_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0022_7f80, 0x0018_6f54)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1c34)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 4) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 5;
            call_ptr(c, 0x0023_c8d0, 0x0018_6f70)?;
            c.g[V1][0] = c.g[V0][0];
            let ready = c.g[V1][0] != 0;
            c.g[V0][0] = 1;
            if !ready {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMemCard_RestartScan(cFEStateMemCard* self)` (0x1871a8):
/// rewind the memory-card manager, close its slots, start the card state's
/// own scan and switch the UI state.
pub fn c_fe_state_mem_card_restart_scan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0018_71c0);
        call_ptr(c, 0x0022_7f80, 0x0018_71c0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0018_71d0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023_fb18, 0x0018_71d0)?;
        c.g[RA][0] = sx(0x0018_71d8);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023_fae0, 0x0018_71d8)?;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x748)).lw(c);
        c.g[A1][0] = 0x36;
        c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0018_71f0);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, target, 0x0018_71f0)?;
        c.g[A2][0] = Addr(gp(c, -0x1c38)).lw(c);
        c.g[V0][0] = u64::MAX;
        Addr(gp(c, -0x1c34)).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Addr(p(r32(c, A2), 0x22c)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x2;
        c.g[RA][0] = sx(0x0018_7210);
        Addr(p(r32(c, A2), 0x234)).sw(c, 0);
        call_ptr(c, 0x0023_c8f0, 0x0018_7210)?;
        c.g[V1][0] = Addr(gp(c, -0x1c38)).lw(c);
        c.g[V0][0] = 0x2;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(p(r32(c, V1), 0x1bc)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateMemCard_v33` (0x1874e8): memory-card state helper: once the card
/// worker is busy, finish; otherwise start the profile search unless a save
/// is in progress.
pub fn fe_state_mem_card_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = Addr(p(this, 0x1ac)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 6;
            c.g[RA][0] = sx(0x0018_750c);
            c.g[A0][0] = Addr(gp_2(c, -0x848)).lw(c);
            call_ptr(c, callee::MC_GET_WORLD, 0x0018_750c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = Addr(p(r(c, A0), 0x440)).lw(c);
            let busy = c.g[V0][0] != 0;
            c.g[V1][0] = 0;
            if !busy {
                c.cycles += 2;
                c.g[V0][0] = Addr(p(r(c, A0), 0x43c)).lw(c);
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(gp_2(c, -0x1c34)).lw(c);
                c.g[V0][0] = 4;
                let mut finish = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(this, 0x238)).lw(c);
                    c.g[V0][0] = c.g[V0][0] ^ 1;
                    c.g[V0][0] = c.g[V0][0] & 1;
                    if c.g[V0][0] != 0 {
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x0018_7558);
                        call_ptr(c, callee::MC_MANAGER_GET_2, 0x0018_7558)?;
                        c.g[RA][0] = sx(0x0018_7560);
                        callee::FE_MEMCARD_START_PROFILE_SEARCH.call(c, 0x0018_7560)?;
                        frame.restore(c, S0, 0x10);
                        finish = false;
                    }
                }
                if finish {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0018_7570);
                    c.g[A0][0] = c.g[S0][0];
                    callee::FE_MEMCARD_FINISH.call(c, 0x0018_7570)?;
                    frame.restore(c, S0, 0x10);
                }
            }
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory card front end.

/// `void cFEStateMemCard_Init(cFEStateMemCard* self)` (0x186b50): create the
/// screen named by the state's engine string, start it, then find the "text"
/// object and call its setup method.
pub fn c_fe_state_mem_card_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2390));
        c.g[S0][0] = lw_2(c, p(c.g[S1][0] as u32, 0x10));
        call_ptr(c, 0x0031_7670, 0x0018_6b74)?; // StrHash
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_6b88)?; // UIEngine_CreateScreen
        let failed = c.g[V0][0] == 0;
        sw_2(c, p(c.g[S1][0] as u32, 0x40), c.g[V0][0]);
        if !failed {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_6ba0)?; // UIScreen_StartThread
        }
        c.cycles += 19;
        c.g[A0][0] = lw_2(c, gp(c, -0x848));
        call_ptr(c, 0x0022_7f80, 0x0018_6ba8)?; // MCManager_Get
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2380));
        call_ptr(c, 0x0031_7670, 0x0018_6bb4)?;
        c.g[A0][0] = lw_2(c, p(c.g[S1][0] as u32, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_6bc0)?; // UIScreen_FindObject
        c.g[V1][0] = lw_2(c, p(c.g[V0][0] as u32, 8));
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x48)).lh(c);
        c.g[A2][0] = lw_2(c, p(c.g[V1][0] as u32, 0x4c));
        c.g[A2][0] = sx(c.g[A2][0] as u32);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[A2][0] as u32, 0x0018_6bd8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateMemCard_Update(self)` (0x186bf0): run the memcard state,
/// then refresh its status label when the scan finished or the state is 6.
pub fn cfe_mem_card_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001d_8de0, 0x00186c04)?;
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        call_ptr(c, 0x0022_7f80, 0x00186c0c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0023_c898, 0x00186c14)?;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x1c0)).lw(c);
            c.g[V0][0] = 6;
            if c.g[V1][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
        }
        c.g[A0][0] = sx(0x004a_0000);
        c.cycles += 7;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x14c0));
        call_ptr(c, 0x0031_7670, 0x00186c38)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x00186c44)?;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0039_fd38, 0x00186c54)?;
        }
        c.g[A0][0] = sx(0x0046_0000);
        c.cycles += 12;
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x2380));
        call_ptr(c, 0x0031_7670, 0x00186c60)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x00186c6c)?;
        c.g[V1][0] = Addr(p(r32(c, V0), 8)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(p(r32(c, V1), 0x48)).lh(c);
        c.g[A2][0] = Addr(p(r32(c, V1), 0x4c)).lw(c);
        let set = r32(c, A2);
        c.g[A0][0] = sx(p(r32(c, V0), r32(c, A0)));
        call_ptr(c, set, 0x00186c84)?;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMemCard_OnScanDone(self)` (0x187270): start the system load
/// or the profile search according to the pending flags, else finish.
pub fn cfe_mem_card_on_scan_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        Addr(p(r32(c, S1), 0x234)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0022_7f80, 0x00187294)?;
        c.g[V1][0] = Addr(p(r32(c, S1), 0x238)).lw(c);
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = c.g[V1][0] & 2;
        let flagged = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0] ^ 1;
        if flagged {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] & 1;
            let finish = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if finish {
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0018_7318, 0x00187300)?;
            } else {
                c.cycles += 8;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0023_fb18, 0x001872e0)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0023_fae0, 0x001872e8)?;
                call_ptr(c, 0x0018_77b0, 0x001872f0)?;
            }
        } else {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0023_fb18, 0x001872b4)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0023_fae0, 0x001872bc)?;
            call_ptr(c, 0x0018_7920, 0x001872c4)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateMemCard_ShowNotFormattedMsg(this)` (0x186c98): format "slot N"
/// for the current slot and open the memcard state's not-formatted dialog.
pub fn cfe_state_mem_card_show_not_formatted_msg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 49;
        let frame = Frame::open(c, 0x6a0);
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, 16, 0x690);
        frame.save(c, 17, 0x680);
        frame.save(c, 18, 0x670);
        frame.save_ra(c, 0x660);
        callee::MC_MANAGER_GET.call(c, 0x0018_6cb8)?;
        let mp = abi::r32(c, V0);
        c.g[A0][0] = mp as u64;
        c.g[S2][0] = sx(sp(c).wrapping_add(0x640));
        c.g[A2][0] = Addr(mp).off(0x428).lw(c);
        c.g[A1][0] = c.g[S2][0];
        callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0018_6ccc)?;

        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x2370));
        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
        c.g[S1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        callee::STR_HASH.call(c, 0x0018_6cec)?;
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_6cfc)?;

        c.g[A0][0] = sp(c) as u64;
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S2][0];
        callee::U_STR_SPRINTF.call(c, 0x0018_6d0c)?;

        c.g[V1][0] = Addr(gp(c, -0x1c38)).lw(c);
        c.g[V0][0] = 4;
        c.g[A1][0] = sp(c) as u64;
        c.g[A2][0] = 1;
        Addr(c.g[V1][0] as u32).off(0x19c).sw(c, c.g[V0][0]);
        c.g[A3][0] = 1;
        c.g[T3][0] = Addr(c.g[V1][0] as u32).off(8).lw(c);
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[A0][0] = Addr(c.g[T3][0] as u32).off(0x120).lh(c);
        c.g[V0][0] = Addr(c.g[T3][0] as u32).off(0x124).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_6d44)?;

        frame.restore(c, 16, 0x690);
        frame.restore(c, 17, 0x680);
        frame.restore(c, 18, 0x670);
        frame.restore_ra(c, 0x660);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00187A38(void* self)` (0x187a38): one step of the memory-card
/// scan state machine: drive the card manager's scan (state in `D_004A14BC`),
/// turn the screen's flags off when the card call reports busy, and save or
/// restore the screen's saved flags.
pub fn c_fe_state_mem_card_scan_step(c: &mut Cpu) {
    let frame = Frame::open(c, 0x40);
    'b_187a78: {
        c.cycles += 15;
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = lw_3(c, r32(c, GP).wrapping_sub(0x848));
        frame.save(c, S1, 0x20);
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[RA][0] = sx(0x00187a5c);
        if call_ptr(c, 0x00227f80, 0x00187a5c).is_err() {
            return;
        }
        c.g[S1][0] = c.g[V0][0];
        c.g[S0][0] = lw_3(c, at(c, S1, 0x434));
        c.g[RA][0] = sx(0x00187a6c);
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, 0x0023c8d0, 0x00187a6c).is_err() {
            return;
        }
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x3;
        if t {
            break 'b_187a78;
        } else {
            c.cycles += 1;
            sw_3(c, r32(c, GP).wrapping_sub(0x1c34), c.g[V0][0]);
            break 'b_187a78;
        }
    }
    'b_187c00: {
        'b_187bfc: {
            'b_187bf8: {
                'b_187be0: {
                    'b_187bac: {
                        'b_187b14: {
                            c.cycles += 4;
                            c.g[V1][0] = lw_3(c, r32(c, GP).wrapping_sub(0x1c34));
                            c.g[V0][0] = 0x1;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_187b14;
                            } else {
                                'b_187ab8: {
                                    'b_187aa0: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] = 0x2;
                                            break 'b_187aa0;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[V1][0] == 0u64 {
                                                c.g[V1][0] = lw_3(c, r32(c, S0));
                                                break 'b_187ab8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                frame.restore(c, S0, 0x30);
                                                if t {
                                                    break 'b_187bfc;
                                                } else {
                                                    break 'b_187aa0;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_187bac;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        frame.restore(c, S0, 0x30);
                                        if t {
                                            break 'b_187be0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            frame.restore(c, S1, 0x20);
                                            if t {
                                                break 'b_187c00;
                                            } else {
                                                break 'b_187ab8;
                                            }
                                        }
                                    }
                                }
                                'b_187adc: {
                                    c.cycles += 6;
                                    c.g[A0][0] = lh(c, at(c, V1, 0x68));
                                    c.g[V0][0] = lw_3(c, at(c, V1, 0x6c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00187ac8);
                                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x00187ac8).is_err() {
                                        return;
                                    }
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V1][0] = 0xfffffffffffffffd;
                                    if t {
                                        break 'b_187adc;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = lw_3(c, at(c, S2, 0x22c));
                                        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                        sw_3(c, at(c, S2, 0x22c), c.g[V0][0]);
                                        break 'b_187adc;
                                    }
                                }
                                c.cycles += 14;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x00187ae8);
                                c.g[A1][0] = 0x1;
                                if call_ptr(c, 0x0023c8f0, 0x00187ae8).is_err() {
                                    return;
                                }
                                c.g[V0][0] = lw_3(c, at(c, S1, 0x748));
                                c.g[A1][0] = 0x33;
                                c.g[A0][0] = lh(c, at(c, V0, 0x8));
                                c.g[V1][0] = lw_3(c, at(c, V0, 0xc));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00187b00);
                                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                                if call_ptr(c, t, 0x00187b00).is_err() {
                                    return;
                                }
                                c.g[V0][0] = lw_3(c, r32(c, GP).wrapping_sub(0x1c34));
                                c.g[V0][0] = sx(at(c, V0, 0x1));
                                sw_3(c, r32(c, GP).wrapping_sub(0x1c34), c.g[V0][0]);
                                let t = true;
                                sw_3(c, at(c, S2, 0x1bc), 0u64);
                                if t {
                                    break 'b_187bf8;
                                } else {
                                    break 'b_187b14;
                                }
                            }
                        }
                        'b_187b3c: {
                            c.cycles += 7;
                            c.g[V1][0] = lw_3(c, r32(c, S0));
                            c.g[A0][0] = lh(c, at(c, V1, 0x68));
                            c.g[V0][0] = lw_3(c, at(c, V1, 0x6c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x00187b28);
                            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                            if call_ptr(c, t, 0x00187b28).is_err() {
                                return;
                            }
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = 0xfffffffffffffffe;
                            if t {
                                break 'b_187b3c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = lw_3(c, at(c, S2, 0x22c));
                                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                sw_3(c, at(c, S2, 0x22c), c.g[V0][0]);
                                break 'b_187b3c;
                            }
                        }
                        'b_187b74: {
                            c.cycles += 3;
                            c.g[V0][0] = lw_3(c, at(c, S1, 0x428));
                            if c.g[V0][0] != 0u64 {
                                c.g[A0][0] = c.g[S1][0];
                                break 'b_187b74;
                            } else {
                                'b_187b70: {
                                    c.cycles += 9;
                                    c.g[A1][0] = lw_3(c, at(c, S1, 0x434));
                                    c.g[V1][0] = lw_3(c, r32(c, A1));
                                    c.g[A0][0] = lh(c, at(c, V1, 0x88));
                                    c.g[V0][0] = lw_3(c, at(c, V1, 0x8c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00187b60);
                                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x00187b60).is_err() {
                                        return;
                                    }
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    sw_3(c, r32(c, GP).wrapping_sub(0x1c3c), c.g[V0][0]);
                                    if t {
                                        break 'b_187b70;
                                    } else {
                                        c.cycles += 1;
                                        sw_3(c, r32(c, GP).wrapping_sub(0x1c3c), 0u64);
                                        break 'b_187b70;
                                    }
                                }
                                c.cycles += 1;
                                c.g[A0][0] = c.g[S1][0];
                                break 'b_187b74;
                            }
                        }
                        c.cycles += 14;
                        c.g[RA][0] = sx(0x00187b7c);
                        c.g[A1][0] = 0u64;
                        if call_ptr(c, 0x0023c8f0, 0x00187b7c).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw_3(c, at(c, S1, 0x748));
                        c.g[A1][0] = 0x33;
                        c.g[A0][0] = lh(c, at(c, V0, 0x8));
                        c.g[V1][0] = lw_3(c, at(c, V0, 0xc));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00187b94);
                        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x00187b94).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw_3(c, r32(c, GP).wrapping_sub(0x1c34));
                        c.g[V1][0] = 0x1;
                        c.g[V0][0] = sx(at(c, V0, 0x1));
                        sw_3(c, r32(c, GP).wrapping_sub(0x1c34), c.g[V0][0]);
                        let t = true;
                        sw_3(c, at(c, S2, 0x1bc), c.g[V1][0]);
                        if t {
                            break 'b_187bf8;
                        } else {
                            break 'b_187bac;
                        }
                    }
                    'b_187bd4: {
                        c.cycles += 7;
                        c.g[V1][0] = lw_3(c, r32(c, S0));
                        c.g[A0][0] = lh(c, at(c, V1, 0x68));
                        c.g[V0][0] = lw_3(c, at(c, V1, 0x6c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x00187bc0);
                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x00187bc0).is_err() {
                            return;
                        }
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = 0xfffffffffffffffb;
                        if t {
                            break 'b_187bd4;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = lw_3(c, at(c, S2, 0x22c));
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            sw_3(c, at(c, S2, 0x22c), c.g[V0][0]);
                            break 'b_187bd4;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = lw_3(c, r32(c, GP).wrapping_sub(0x1c34));
                    c.g[V0][0] = sx(at(c, V0, 0x1));
                    sw_3(c, r32(c, GP).wrapping_sub(0x1c34), c.g[V0][0]);
                    break 'b_187be0;
                }
                c.cycles += 3;
                c.g[V0][0] = lw_3(c, at(c, S1, 0x428));
                if c.g[V0][0] != 0u64 {
                    sw_3(c, at(c, S2, 0x234), 0u64);
                    break 'b_187bf8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw_3(c, at(c, S2, 0x22c));
                    sw_3(c, at(c, S2, 0x238), c.g[V0][0]);
                    sw_3(c, at(c, S2, 0x234), 0u64);
                    break 'b_187bf8;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            break 'b_187bfc;
        }
        c.cycles += 1;
        frame.restore(c, S1, 0x20);
        break 'b_187c00;
    }
    c.cycles += 4;
    frame.restore(c, S2, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x40));
    c.pc = t;
    return;
}

/// `MemCardStateBase_ctor` (0x1d53b0): the base constructor of the memory card
/// state, called directly by `cFEStateMemCard_ctor_2`.
pub(crate) const FE_MEMCARD_STATE_BASE_CTOR: Callee = Callee {
    addr: 0x001d_53b0,
    f: ee::s04::MemCardStateBase_ctor,
};

/// `cFEStateMemCard` constructor (0x1de950): `MemCardStateBase_ctor(this, 0, 0)`,
/// then the vtable at `+8`.
pub fn c_fe_state_mem_card_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            15,
            true,
            true,
            Target::Direct(FE_MEMCARD_STATE_BASE_CTOR),
            0x001d_e96c,
            0x4a90,
        )
    });
}

/// Menu-kind values `cFEStateMemCard_v26` compares the manager's word at
/// +0x130 against, in the order the generated compare chain tests them.
pub(crate) const CFE_MEM_CARD_KINDS: [u64; 13] = [
    0x1f, 0x21, 0x20, 0x22, 0x31, 0x32, 0x33, 0x35, 0x36, 0x34, 0x2, 0x3, 0x6,
];

/// `cFEStateMemCard_v26` (0x1876a8, reached through `cFEStateMemCard`): asks
/// the UI screen (`gp-0x1c38`) for an object by hash name, asks the MC
/// manager for its message kind (`+0x130`), and either pulls the message
/// from the overlay manager and sets the text (kinds in the table above), or
/// calls the object's vtable slot. Frame 0x660, s0 saved at +0x650.
pub fn cfe_state_mem_card_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x660);
        frame.save(c, S0, 0x650);
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x2380));
        frame.save_ra(c, 0x640);
        c.g[S0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1c38)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0018_76c4)?;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_76d0)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x0022_7f80, 0x0018_76dc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x130)).lw(c);

        // The kind compare chain: each step costs 2 cycles after the first.
        let mut matched = false;
        for (i, &kind) in CFE_MEM_CARD_KINDS.iter().enumerate() {
            if i > 0 {
                c.cycles += 2;
            }
            let hit = c.g[V1][0] == kind;
            c.g[V0][0] = CFE_MEM_CARD_KINDS[(i + 1).min(CFE_MEM_CARD_KINDS.len() - 1)];
            if hit {
                matched = true;
                break;
            }
        }

        if matched {
            c.cycles += 14;
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = 0x320;
            call_ptr(c, 0x0024_1e18, 0x0018_775c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0d00, 0x0018_7768)?;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_7780)?;
            frame.restore(c, S0, 0x650);
        } else {
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
            c.cycles += 6;
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_779c)?;
            frame.restore(c, S0, 0x650);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x640);
        frame.close(c);
        Ok(())
    });
}

/// Offsets in the mem-card UI state (`gp - 0x1c38` points at it).
pub(crate) const MEMCARD_FILES_FLAGS: u32 = 0x22c;

pub(crate) const MEMCARD_SPACE_NEEDED: u32 = 0x230;

pub(crate) const MEMCARD_SPACE_FAILED: u32 = 0x234;

/// `void cFEStateMemCard_CheckFreeSpace(void)` (0x186f90): the card-scan
/// step that checks that the memory card has room for the missing save files.
/// When the check has already failed (state +0x234 set) it only forwards to
/// the card vtable (+0x70) once the stage is 4 or more. Otherwise, unless the
/// card has a critical error, it sums the space needed (the profile file
/// when flag bit 0 is set) against the free space and the free directory
/// entries; on a shortage it sets +0x234 and switches the manager to state
/// 0x17; else it finishes the scan (`cFEStateMemCard_OnScanDone`).
pub fn c_fe_state_mem_card_check_free_space(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let state_ptr = (c.g[GP][0] as u32).wrapping_sub(0x1c38);
        let stage_ptr = (c.g[GP][0] as u32).wrapping_sub(0x1c34);
        let app_ptr = (c.g[GP][0] as u32).wrapping_sub(0x848);
        c.g[V0][0] = c.lw(state_ptr);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(MEMCARD_SPACE_FAILED));
        let failed = c.g[V1][0] != 0;
        c.g[V0][0] = c.lw(stage_ptr);
        'epilogue: {
            if failed {
                // The check failed earlier: forward to the card once it is ready.
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
                let early = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x60);
                if early {
                    break 'epilogue;
                }
                c.cycles += 13;
                c.g[A0][0] = c.lw(app_ptr);
                call_ptr(c, 0x0022_7f80, 0x0018_6fd4)?;
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x434));
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xf8));
                c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                c.g[A2][0] = c.lw(state_ptr);
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x9c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018_6ff8)?;
                frame.restore(c, S0, 0x60);
                break 'epilogue;
            }
            'scan_done: {
                c.cycles += 7;
                c.g[A0][0] = c.lw(app_ptr);
                call_ptr(c, 0x0022_7f80, 0x0018_7008)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0023_c8d0, 0x0018_7014)?; // critical card error?
                if c.g[V0][0] != 0 {
                    break 'scan_done;
                }
                c.cycles += 15;
                c.g[S5][0] = 0;
                call_ptr(c, 0x0014_dc80, 0x0018_7024)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x5;
                call_ptr(c, 0x0014_dd58, 0x0018_7030)?;
                c.g[S4][0] = 0;
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x428));
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0024_19d8, 0x0018_7040)?; // free space of the device
                c.g[V1][0] = c.lw(state_ptr);
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(MEMCARD_FILES_FLAGS));
                c.g[V0][0] &= 0x1;
                let no_profile = c.g[V0][0] == 0;
                c.sw((c.g[V1][0] as u32).wrapping_add(MEMCARD_SPACE_NEEDED), 0);
                if !no_profile {
                    c.cycles += 14;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                    c.g[S4][0] = 0x1;
                    c.g[S1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xb0));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x0015_2ba8, 0x0018_7070)?; // save-profile size
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb4));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x0018_7080)?; // card space for that size
                    c.g[A0][0] = c.lw(state_ptr);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(MEMCARD_SPACE_NEEDED));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(
                        (c.g[A0][0] as u32).wrapping_add(MEMCARD_SPACE_NEEDED),
                        c.g[V1][0],
                    );
                }
                // Not enough space, or not enough directory entries?
                c.cycles += 5;
                c.g[V1][0] = c.lw(state_ptr);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(MEMCARD_SPACE_NEEDED));
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[S5][0] = 0x1;
                } else {
                    c.cycles += 9;
                    memcard_free_dir_entries(c, 0x0018_70bc)?;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    if c.g[V0][0] != c.g[V1][0] {
                        c.cycles += 9;
                        memcard_free_dir_entries(c, 0x0018_70e0)?;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[S5][0] = 0x1;
                        }
                    }
                }
                c.cycles += 2;
                let ok = c.g[S5][0] == 0;
                c.g[A0][0] = c.lw(state_ptr);
                if ok {
                    break 'scan_done;
                }
                // Failed: remember it and switch the manager to state 0x17.
                c.cycles += 10;
                c.g[V1][0] = 0x1;
                c.g[A1][0] = 0x17;
                c.sw(
                    (c.g[A0][0] as u32).wrapping_add(MEMCARD_SPACE_FAILED),
                    c.g[V1][0],
                );
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x748));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018_7114)?;
                frame.restore(c, S0, 0x60);
                break 'epilogue;
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw(state_ptr);
            call_ptr(c, 0x0018_7270, 0x0018_7124)?; // cFEStateMemCard_OnScanDone
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `mgr->memCard->vtable[0xc8]` call (free directory entries), returning to
/// `ret`: `a1 = mgr[+0x434]`, then the slot's delta and function.
pub(crate) fn memcard_free_dir_entries(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xcc));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// `void cFEStateMemCard_StartProfileSearch(void)` (0x1877b0, frame 0x90): the memory
/// card screen starts reading the profile file. Sets the state stage and the
/// instance's stage word to 5, asks the BE interface for interface 5, sets up
/// the memory card manager (profile read size and file type 1, file cursor and
/// slot from the profile file global, state 53), formats the file name and
/// product code into the instance's text buffer (+0x11c), marks the load
/// pressed and processing (+0x1a8), and plays the `@loadanim` animation on
/// the `!` screen object when both exist.
pub fn fe_state_mem_card_start_profile_search(c: &mut Cpu) {
    guest(c, |c| {
        'b_187908: {
            c.cycles += 71;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S1][0] = 0x5;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c34), c.g[S1][0]);
            c.g[RA][0] = sx(0x001877d4);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1c0), c.g[S1][0]);
            call_ptr(c, 0x0014dc80, 0x001877d4)?;
            c.g[A1][0] = 0x5;
            c.g[RA][0] = sx(0x001877e0);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014dd58, 0x001877e0)?;
            c.g[RA][0] = sx(0x001877e8);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            call_ptr(c, 0x00227f80, 0x001877e8)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            c.g[S0][0] = c.g[V0][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1bc), 0u64);
            c.g[RA][0] = sx(0x001877fc);
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c34), c.g[S1][0]);
            call_ptr(c, 0x00152ba8, 0x001877fc)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00187808);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0023caa0, 0x00187808)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00187814);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0023c8f0, 0x00187814)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c3c));
            c.g[A1][0] = 0x35;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1c4), c.g[V0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1bc), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00187840);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x00187840)?;
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = 0x14;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00187854);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x00241d40, 0x00187854)?;
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00187868);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14c8));
            call_ptr(c, 0x002c2540, 0x00187868)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A1][0] = 0x10;
            c.g[RA][0] = sx(0x00187878);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00241aa0, 0x00187878)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00187890);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11c));
            call_ptr(c, 0x002c26d0, 0x00187890)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            c.g[RA][0] = sx(0x0018789c);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x001d9308, 0x0018789c)?;
            c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            c.g[V0][0] = 0x1;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14c0));
            c.g[RA][0] = sx(0x001878b4);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[V0][0]);
            call_ptr(c, 0x00317670, 0x001878b4)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001878c0);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001878c0)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S1][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1c38));
            if t {
                break 'b_187908;
            } else {
                c.cycles += 10;
                c.g[A0][0] = 0x460000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2300));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = sx(0x001878e0);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                call_ptr(c, 0x00317670, 0x001878e0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001878ec);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x00397140, 0x001878ec)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S1][0];
                if t {
                    break 'b_187908;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0x9;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x00187908);
                    c.g[T0][0] = 0u64;
                    call_ptr(c, 0x0039fcc8, 0x00187908)?;
                    break 'b_187908;
                }
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateMemCard::ShowNotDetectedMsg` (0x186d60): format the "memory card
/// not detected" text for the current slot and show it with two buttons.
pub fn cfe_state_mem_card_show_not_detected_msg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 120;
        let frame = Frame::open(c, 0x6b0);
        c.g[A0][0] = oc830_gp_lw(c, 0x848);
        frame.save(c, 16, 0x6a0);
        frame.save(c, 17, 0x690);
        frame.save(c, 18, 0x680);
        frame.save(c, 19, 0x670);
        frame.save_ra(c, 0x660);
        call_ptr(c, 0x0022_7f80, 0x0018_6d84)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[SP][0];
        let a = oc830_ea(c, S1, 0x428);
        c.g[A2][0] = c.lw(a);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0024_1dc8, 0x0018_6d98)?;
        call_ptr(c, 0x0014_dc80, 0x0018_6da0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x5;
        call_ptr(c, 0x0014_dd58, 0x0018_6dac)?;
        let a = oc830_ea(c, S1, 0x434);
        c.g[S0][0] = c.lw(a);
        c.g[S2][0] = c.lw(c.g[S0][0] as u32);
        let a = oc830_ea(c, S2, 0xb0);
        c.g[V0][0] = c.lh(a);
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0015_2ba8, 0x0018_6dc0)?;
        let a = oc830_ea(c, S2, 0xb4);
        c.g[V1][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, target, 0x0018_6dd0)?;
        c.g[A1][0] = oc830_gp_lw(c, 0x848);
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = 0x46_0000;
        c.g[S2][0] = sx(oc830_ea(c, SP, 0x20));
        let a = oc830_ea(c, A1, 0x8c);
        c.g[S0][0] = c.lw(a);
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2358);
        let a = oc830_ea(c, S0, 0x4);
        c.g[S1][0] = c.lw(a);
        let a = oc830_ea(c, S1, 0x20);
        c.g[V0][0] = c.lh(a);
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0031_7670, 0x0018_6df8)?;
        let a = oc830_ea(c, S1, 0x24);
        c.g[V1][0] = c.lw(a);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_6e08)?;
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        call_ptr(c, 0x002c_26d0, 0x0018_6e1c)?;
        c.g[T3][0] = oc830_gp_lw(c, 0x1c38);
        c.g[V0][0] = 0x9;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        let a = oc830_ea(c, T3, 0x19c);
        c.sw(a, c.g[V0][0]);
        c.g[T2][0] = 0;
        let a = oc830_ea(c, T3, 0x8);
        c.g[V0][0] = c.lw(a);
        c.g[A3][0] = 0;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0;
        let a = oc830_ea(c, V0, 0x120);
        c.g[A0][0] = c.lh(a);
        let a = oc830_ea(c, V0, 0x124);
        c.g[V1][0] = c.lw(a);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(oc830_ea(c, T3, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_6e54)?;
        // Button 1 (a2 = 0), then button 2 (a2 = 1): text lookup + button add.
        c.g[A1][0] = oc830_gp_lw(c, 0x1c38);
        c.g[A0][0] = 0x46_0000;
        c.g[V0][0] = oc830_gp_lw(c, 0x848);
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2338);
        let a = oc830_ea(c, A1, 0x218);
        c.g[S1][0] = c.lw(a);
        let a = oc830_ea(c, V0, 0x8c);
        c.g[S0][0] = c.lw(a);
        let a = oc830_ea(c, S1, 0x8);
        c.g[S3][0] = c.lw(a);
        let a = oc830_ea(c, S0, 0x4);
        c.g[S2][0] = c.lw(a);
        let a = oc830_ea(c, S3, 0xd0);
        c.g[V0][0] = c.lh(a);
        let a = oc830_ea(c, S2, 0x20);
        c.g[V1][0] = c.lh(a);
        c.g[S1][0] = sx(oc830_ea(c, S1, 0).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, 0x0031_7670, 0x0018_6e88)?;
        let a = oc830_ea(c, S2, 0x24);
        c.g[V1][0] = c.lw(a);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_6e98)?;
        let a = oc830_ea(c, S3, 0xd4);
        c.g[A3][0] = c.lw(a);
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        let target = c.g[A3][0] as u32;
        c.g[A2][0] = 0;
        call_ptr(c, target, 0x0018_6eac)?;
        c.g[V1][0] = oc830_gp_lw(c, 0x1c38);
        c.g[A0][0] = 0x46_0000;
        c.g[V0][0] = oc830_gp_lw(c, 0x848);
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2328);
        let a = oc830_ea(c, V1, 0x218);
        c.g[S1][0] = c.lw(a);
        let a = oc830_ea(c, V0, 0x8c);
        c.g[S0][0] = c.lw(a);
        let a = oc830_ea(c, S1, 0x8);
        c.g[S3][0] = c.lw(a);
        let a = oc830_ea(c, S0, 0x4);
        c.g[S2][0] = c.lw(a);
        let a = oc830_ea(c, S3, 0xd0);
        c.g[V0][0] = c.lh(a);
        let a = oc830_ea(c, S2, 0x20);
        c.g[V1][0] = c.lh(a);
        c.g[S1][0] = sx(oc830_ea(c, S1, 0).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, 0x0031_7670, 0x0018_6ee0)?;
        let a = oc830_ea(c, S2, 0x24);
        c.g[V1][0] = c.lw(a);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_6ef0)?;
        let a = oc830_ea(c, S3, 0xd4);
        c.g[A3][0] = c.lw(a);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        let target = c.g[A3][0] as u32;
        c.g[A2][0] = 0x1;
        call_ptr(c, target, 0x0018_6f04)?;
        oc830_msg_show(c, 0x0018_6f24)?;
        frame.restore(c, 16, 0x6a0);
        frame.restore(c, 17, 0x690);
        frame.restore(c, 18, 0x680);
        frame.restore(c, 19, 0x670);
        frame.restore_ra(c, 0x660);
        frame.close(c);
        Ok(())
    })
}

/// The last call of both memory-card messages: `popup->vtable[0xc8](popup +
/// delta, 1)` for the popup at `*(gp-0x1c38) + 0x218`.
#[inline(always)]
pub(crate) fn oc830_msg_show(c: &mut Cpu, ret: u32) -> Flow {
    c.g[V1][0] = oc830_gp_lw(c, 0x1c38);
    c.g[A1][0] = 0x1;
    let a = oc830_ea(c, V1, 0x218);
    c.g[A2][0] = c.lw(a);
    let a = oc830_ea(c, A2, 0x8);
    c.g[V0][0] = c.lw(a);
    let a = oc830_ea(c, V0, 0xc8);
    c.g[A0][0] = c.lh(a);
    let a = oc830_ea(c, V0, 0xcc);
    c.g[V1][0] = c.lw(a);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(oc830_ea(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `cFEStateMemCard::ShowNoSpaceMsg` (0x187360): format the "not enough
/// space" text for the current slot and show it with one button.
pub fn cfe_state_mem_card_show_no_space_msg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 98;
        let frame = Frame::open(c, 0x6b0);
        c.g[A0][0] = oc830_gp_lw(c, 0x848);
        frame.save(c, 16, 0x6a0);
        frame.save(c, 17, 0x690);
        frame.save(c, 18, 0x680);
        frame.save(c, 19, 0x670);
        frame.save_ra(c, 0x660);
        call_ptr(c, 0x0022_7f80, 0x0018_7384)?;
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_738c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x5;
        call_ptr(c, 0x0014_dd58, 0x0018_7398)?;
        c.g[V1][0] = oc830_gp_lw(c, 0x1c38);
        c.g[V0][0] = 0x14;
        let a = oc830_ea(c, V1, 0x19c);
        c.sw(a, c.g[V0][0]);
        let a = oc830_ea(c, S2, 0x434);
        c.g[S0][0] = c.lw(a);
        c.g[S1][0] = c.lw(c.g[S0][0] as u32);
        let a = oc830_ea(c, S1, 0xb0);
        c.g[V0][0] = c.lh(a);
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0015_2ba8, 0x0018_73b8)?;
        let a = oc830_ea(c, S1, 0xb4);
        c.g[V1][0] = c.lw(a);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_73c8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[S3][0] = sx(oc830_ea(c, SP, 0x640));
        let a = oc830_ea(c, A0, 0x428);
        c.g[A2][0] = c.lw(a);
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0024_1dc8, 0x0018_73e0)?;
        c.g[V0][0] = oc830_gp_lw(c, 0x848);
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2318);
        let a = oc830_ea(c, V0, 0x8c);
        c.g[S0][0] = c.lw(a);
        let a = oc830_ea(c, S0, 0x4);
        c.g[S1][0] = c.lw(a);
        let a = oc830_ea(c, S1, 0x20);
        c.g[V0][0] = c.lh(a);
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0031_7670, 0x0018_7400)?;
        let a = oc830_ea(c, S1, 0x24);
        c.g[V1][0] = c.lw(a);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_7410)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        call_ptr(c, 0x002c_26d0, 0x0018_7424)?;
        c.g[T3][0] = oc830_gp_lw(c, 0x1c38);
        c.g[A1][0] = c.g[SP][0];
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        let a = oc830_ea(c, T3, 0x8);
        c.g[V0][0] = c.lw(a);
        c.g[T2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A2][0] = 0;
        let a = oc830_ea(c, V0, 0x120);
        c.g[A0][0] = c.lh(a);
        let a = oc830_ea(c, V0, 0x124);
        c.g[V1][0] = c.lw(a);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(oc830_ea(c, T3, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_7454)?;
        c.g[A1][0] = oc830_gp_lw(c, 0x1c38);
        c.g[A0][0] = 0x46_0000;
        c.g[V0][0] = oc830_gp_lw(c, 0x848);
        c.g[A0][0] = oc830_sx_sub(c, A0, 0x2328);
        let a = oc830_ea(c, A1, 0x218);
        c.g[S1][0] = c.lw(a);
        let a = oc830_ea(c, V0, 0x8c);
        c.g[S0][0] = c.lw(a);
        let a = oc830_ea(c, S1, 0x8);
        c.g[S3][0] = c.lw(a);
        let a = oc830_ea(c, S0, 0x4);
        c.g[S2][0] = c.lw(a);
        let a = oc830_ea(c, S3, 0xd0);
        c.g[V0][0] = c.lh(a);
        let a = oc830_ea(c, S2, 0x20);
        c.g[V1][0] = c.lh(a);
        c.g[S1][0] = sx(oc830_ea(c, S1, 0).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx(oc830_ea(c, S0, 0).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, 0x0031_7670, 0x0018_7488)?;
        let a = oc830_ea(c, S2, 0x24);
        c.g[V1][0] = c.lw(a);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_7498)?;
        let a = oc830_ea(c, S3, 0xd4);
        c.g[A3][0] = c.lw(a);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        let target = c.g[A3][0] as u32;
        c.g[A2][0] = 0x1;
        call_ptr(c, target, 0x0018_74ac)?;
        oc830_msg_show(c, 0x0018_74cc)?;
        frame.restore(c, 16, 0x6a0);
        frame.restore(c, 17, 0x690);
        frame.restore(c, 18, 0x680);
        frame.restore(c, 19, 0x670);
        frame.restore_ra(c, 0x660);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMemCard_OnChildEvent(self, child, key)` (0x187580): the memory
/// card state's popup callback. After the base handler, state 0x15 starts the
/// system load or the profile search, or finishes; state 0xa and key 0xf finish;
/// states 0x4/0x14 handle the confirm keys (0x10 sets the retry flag, otherwise
/// the screen's vtable call and a rescan).
pub fn cfe_state_mem_card_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x001d_8700, 0x0018_759c)?;
        c.g[V1][0] = oc832_lw(c, S1, 0x19c);
        c.g[V0][0] = 0x15;
        let other_state = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 0xa;
        'done: {
            'rescan_or_set_flag: {
                'finish: {
                    'other: {
                        if other_state {
                            break 'other;
                        }
                        // State 0x15: start the load or search, or finish.
                        c.cycles += 12;
                        c.g[A0][0] = oc832_gp_lw(c, 0x848);
                        call_ptr(c, 0x0022_7f80, 0x0018_75b4)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0023_fb18, 0x0018_75c4)?;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0023_fae0, 0x0018_75cc)?;
                        c.g[V1][0] = oc832_lw(c, S1, 0x238);
                        c.g[V0][0] = c.g[V1][0] & 2;
                        let flagged = c.g[V0][0] != 0;
                        c.g[V0][0] = c.g[V1][0] ^ 1;
                        if !flagged {
                            c.cycles += 4;
                            call_ptr(c, 0x0018_7920, 0x0018_75e4)?;
                            frame.restore(c, S0, 0x20);
                            break 'done;
                        }
                        c.cycles += 3;
                        c.g[V0][0] &= 1;
                        if c.g[V0][0] == 0 {
                            break 'finish;
                        }
                        c.cycles += 4;
                        call_ptr(c, 0x0018_77b0, 0x0018_7600)?;
                        frame.restore(c, S0, 0x20);
                        break 'done;
                    }
                    // Other states: state 0xa with key 0xf, or state 4, finish.
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V1][0] = oc832_lw(c, S1, 0x19c);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xf;
                        if c.g[S0][0] == c.g[V0][0] {
                            break 'finish;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = oc832_lw(c, S1, 0x19c);
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x4;
                    let not_state4 = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xf));
                    if not_state4 {
                        break 'rescan_or_set_flag;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0018_7318, 0x0018_7634)?;
                frame.restore(c, S0, 0x20);
                break 'done;
            }
            // Keys 0xf and 0x10 in state 0x14.
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 2u64) as u64;
            let wrong_key = c.g[V0][0] == 0;
            c.g[V0][0] = 0x14;
            if !wrong_key {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    frame.restore(c, S0, 0x20);
                    break 'done;
                }
                c.cycles += 3;
                c.g[V0][0] = c.g[S0][0] ^ 0x10;
                let key_is_10 = c.g[V0][0] == 0;
                c.g[V1][0] = 1;
                if !key_is_10 {
                    c.cycles += 13;
                    c.g[V0][0] = oc832_lw(c, S1, 0x8);
                    c.g[A0][0] = oc832_lh(c, V0, 0x148);
                    c.g[V1][0] = oc832_lw(c, V0, 0x14c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0018_7670)?;
                    c.g[A0][0] = oc832_gp_lw(c, 0x848);
                    call_ptr(c, 0x0022_7f80, 0x0018_7678)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0023_d618, 0x0018_7680)?;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0018_7270, 0x0018_7688)?;
                    frame.restore(c, S0, 0x20);
                    break 'done;
                }
                c.cycles += 1;
                oc832_sw(c, S1, 0x1ec, c.g[V1][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMemCard_StartSystemLoad(void)` (0x187920): set the memcard
/// state (`gp-0x1c34` = 4, state object `+0x1c0` = 5, `+0x1bc` = 2), start the
/// manager's system-file read of the current size and type, play animation 9 of
/// the "load" widget on the screen, call `SetLoadPressed(state, 1)` and set
/// `+0x1a8`.
pub fn cfe_state_mem_card_start_system_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = oc832_gp_lw(c, 0x1c38);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[V1][0] = 5;
        frame.save_ra(c, 0);
        c.g[S1][0] = 2;
        oc832_gp_sw(c, 0x1c34, c.g[V0][0]);
        oc832_sw(c, A0, 0x1c0, c.g[V1][0]);
        call_ptr(c, 0x0014_dc80, 0x0018_794c)?;
        c.g[A1][0] = 5;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0018_7958)?;
        c.g[A0][0] = oc832_gp_lw(c, 0x848);
        call_ptr(c, 0x0022_7f80, 0x0018_7960)?;
        c.g[V1][0] = oc832_gp_lw(c, 0x1c38);
        c.g[S0][0] = c.g[V0][0];
        oc832_sw(c, V1, 0x1bc, c.g[S1][0]);
        oc832_sw(c, V1, 0x1c4, 0);
        call_ptr(c, 0x0015_2948, 0x0018_7974)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023_caa0, 0x0018_7980)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0023_c8f0, 0x0018_798c)?;
        c.g[V0][0] = oc832_lw(c, S0, 0x748);
        c.g[A1][0] = 0x35;
        c.g[A0][0] = oc832_lh(c, V0, 0x8);
        c.g[V1][0] = oc832_lw(c, V0, 0xc);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_79a4)?;
        c.g[S0][0] = oc832_gp_lw(c, 0x1c38);
        c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x14c0));
        oc832_sw(c, S0, 0x1bc, c.g[S1][0]);
        call_ptr(c, 0x0031_7670, 0x0018_79b8)?;
        c.g[A0][0] = oc832_lw(c, S0, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_79c4)?;
        c.g[S1][0] = c.g[V0][0];
        let no_widget = c.g[S1][0] == 0;
        c.g[V0][0] = oc832_gp_lw(c, 0x1c38);
        'played: {
            if no_widget {
                break 'played;
            }
            c.cycles += 10;
            c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x2300));
            c.g[S0][0] = oc832_lw(c, V0, 0x10);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
            call_ptr(c, 0x0031_7670, 0x0018_79e4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_7140, 0x0018_79f0)?;
            let no_anim = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if no_anim {
                break 'played;
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = 9;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, 0x0039_fcc8, 0x0018_7a0c)?;
        }
        c.cycles += 11;
        c.g[A0][0] = oc832_gp_lw(c, 0x1c38);
        c.g[A1][0] = 1;
        call_ptr(c, 0x001d_9308, 0x0018_7a18)?;
        c.g[V1][0] = oc832_gp_lw(c, 0x1c38);
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        oc832_sw(c, V1, 0x1a8, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `FEScreenAutosavePopup* FEScreenAutosavePopup_ctor(this)` (0x187c18): base
/// `cUIState` constructor, then vtable `0x46b4a0`, screen (`+0x48`, the frame
/// counter) cleared; returns `this`.
pub fn fe_screen_autosave_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e318, 0x0018_7c2c)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4b60));
        oc832_sw(c, S0, 0x48, 0);
        oc832_sw(c, S0, 0x8, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateMemCard_HandleInput` (0x0018_7c10): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_mem_card_handle_input_00187c10(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateMemCard_OnWidgetAction` (0x001d_e940): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_mem_card_on_widget_action_001de940(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateMemCard_v37` (0x001d_e948): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_mem_card_v37_001de948(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::fe::festatecredits::tests::oc830_install_fakes;
    pub(crate) use crate::game::sound::soundsys::tests::oc771_install_fakes;

    use crate::difftest::elf_image;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::bx::bxstring::tests::put_str_2 as put_str;
    pub(crate) use crate::game::bx::bxstringctor::tests::be_library;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::pool_7 as pool;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_2 as pool_allocator;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot;

    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;

    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCR;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_4;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_4;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_5;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2 as globals;

    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;

    use ssx3_ee as ee;

    #[test]
    fn fe_state_memcard_update_card_scan_matches() {
        crate::difftest::check(
            ee::s02::f_00187180__generated,
            fe_state_memcard_update_card_scan,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1871_80, 2);
                be_library(c);
                // `D_004A14B8`, the card scan state the second call advances.
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x1c38),
                    if seed % 2 == 0 { 0 } else { DATA + 0x800 },
                );
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x800, r.next_u32() % 4);
            },
        );
    }

    /// Write `{delta, fake}` at `at + off` (a gcc vtable slot).
    pub(crate) fn put_slot(c: &mut Cpu, r: &mut Rng, at: u32, off: u32) {
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(at + off, delta as u16);
        c.bus.write32(at + off + 4, FAKE);
    }

    #[test]
    fn cfe_state_mem_card_close_and_set_state6_matches() {
        check(
            ee::s02::f_00187148__generated,
            cfe_state_mem_card_close_and_set_state6,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1871_48, 0);
                let state = DATA + 0x400;
                fill(c, &mut r, state, 0x40);
                c.bus.write32(GAME_GP.wrapping_sub(0x1c38), state);
                c.bus.write32(state + 8, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 0x148);
            },
        );
    }

    /// A vtable whose one slot `off` is the fake, installed at `*(obj + 8)`.
    pub(crate) fn vtable_at(c: &mut Cpu, r: &mut Rng, obj: u32, off: u32) -> u32 {
        let vt = vtable(c, r, &[off]);
        c.bus.write32(obj + 8, vt);
        vt
    }

    #[test]
    fn c_fe_state_mem_card_reset_scan_matches() {
        check(
            ssx3_ee::s02::f_00187230__generated,
            c_fe_state_mem_card_reset_scan,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12, 1);
                globals(c);
                let mgr = DATA + 0x4000;
                // The manager singleton and its scan object, both zeroed
                // dummies (the lazy constructor would call the allocator).
                c.bus.write32(GAME_GP - 0x47c, mgr);
                c.bus.write32(GAME_GP - 0x1c38, mgr);
                c.bus.write32(GAME_GP - 0x1c34, r.next_u32());
                // The card object the manager scans (`+0x434`) and its vtable.
                c.bus.write32(mgr + 0x434, DATA + 0x5000);
                vtable_at(c, &mut r, DATA + 0x5000, 0x98);
            },
        );
    }

    #[test]
    fn fe_state_mem_card_finish_matches() {
        check(
            ssx3_ee::s02::f_00187318__generated,
            fe_state_mem_card_finish,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x187318, 0);
                be_library_2(c);
                pool_allocator(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 0x1ac, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(this_);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x186f40 cFEStateMemCard_IsScanDoneWithCard.

    #[test]
    fn c_fe_state_mem_card_is_scan_done_with_card_matches() {
        check(
            ee::s02::f_00186f40__generated,
            c_fe_state_mem_card_is_scan_done_with_card,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x186f_40, 0);
                let world = DATA + 0x1000;
                let mc = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(GAME_GP.wrapping_sub(0x47c), mc);
                c.bus.write32(mc + 0x43c, if seed % 2 == 0 { 0 } else { 1 });
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x1c34), (seed % 5) as u32);
            },
        );
    }

    #[test]
    fn c_fe_state_mem_card_restart_scan_matches() {
        check(
            ssx3_ee::s02::f_001871a8__generated,
            c_fe_state_mem_card_restart_scan,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1871a8, 6);
                be_library_3(c);
                world(c);
                let mgr = DATA + 0x200;
                let vt = DATA + 0x400;
                w(c, GAME_GP.wrapping_sub(0x1c38), mgr);
                w(c, GAME_GP.wrapping_sub(0x1c34), 0);
                w(c, mgr + 0x748, vt);
                // The slot itself is a vtable: delta at +8, function at +0xc.
                slot(c, vt, 8, 0, FAKE);
                slot(c, vt, 0xc, 0, FAKE + 0x10);
                c.g[A0][0] = DATA as u64;
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn fe_state_mem_card_v33_matches() {
        check(
            ee::s02::f_001874e8__generated,
            fe_state_mem_card_v33,
            SEEDS_2,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(c, this + 0x1ac, (seed & 1) as u32);
                w(c, gp_2(c, -0x1c34), (seed % 6) as u32);
                w(c, this + 0x238, (seed & 2) as u32);
                c.g[A0][0] = this as u64;
            }),
        );
    }

    // ----------------------------------------------------------- mem card

    #[test]
    fn c_fe_state_mem_card_init_matches() {
        check(
            ssx3_ee::s02::f_00186b50__generated,
            c_fe_state_mem_card_init,
            8,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0018_6b50);
                elf_image(c);
                heap(c);
                pool(c);
                let state = DATA + 0x100;
                let engine = DATA + 0x2000;
                clear(c, state, 0x100);
                clear(c, engine, 0x400);
                put_str(c, DATA + 0x3000, "memcard");
                w(c, state + 0x10, engine);
                c.g[A0][0] = sx(state);
            },
        );
    }

    #[test]
    fn cfe_mem_card_update_matches() {
        check(
            ee::s02::f_00186bf0__generated,
            cfe_mem_card_update,
            4,
            &|c, seed| {
                let _ = base_3(c, seed, 0x186bf0, 4);
                c.g[A0][0] = OBJ as u64;
                // A manager that is idle (IsIdle of a zeroed manager) and a
                // base_3 update that returns immediately.
                w(c, GAME_GP.wrapping_sub(0x47c), OBJ2);
                w(c, GAME_GP.wrapping_sub(0x848), WORLD_3);
                w(c, OBJ + 0x218, 1);
                w(
                    c,
                    OBJ + 0x1c0,
                    if seed % 3 == 0 { 6 } else { (seed % 8) as u32 },
                );
                w(c, OBJ + 0x40, SCR);
                w(c, 0x004a_14c0, 0);
                w(c, 0x0045_dc80, 0);
            },
        );
    }

    #[test]
    fn cfe_mem_card_on_scan_done_matches() {
        check(
            ee::s02::f_00187270__generated,
            cfe_mem_card_on_scan_done,
            8,
            &|c, seed| {
                let _ = base_3(c, seed, 0x187270, 4);
                c.g[A0][0] = OBJ as u64;
                w(c, GAME_GP.wrapping_sub(0x47c), OBJ2);
                w(c, OBJ + 0x234, 0);
                w(c, OBJ + 0x238, (seed % 4) as u32);
            },
        );
    }

    #[test]
    fn cfe_state_mem_card_show_not_formatted_msg_matches() {
        check(
            ee::s02::f_00186c98__generated,
            cfe_state_mem_card_show_not_formatted_msg,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x186c, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(RMAN, 0);
                let mc = DATA + 0x2000;
                c.bus.write32(WORLD_4 + 0x8c, mc);
                let obj = DATA + 0x2400;
                c.bus.write32(mc + 4, obj);
                c.bus.write32(obj + 0x20, 0);
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(obj, vt);
                c.bus.write32(GAME_GP - 0x1c38, DATA + 0x2800);
                c.bus.write32(DATA + 0x2800 + 8, DATA + 0x2c00);
                c.bus.write32(DATA + 0x2c00, vt.wrapping_add(0x20));
            },
        );
    }

    pub(crate) const MEMCARD: u32 = DATA + 0x1_0000;

    pub(crate) const MEMCARD_MP: u32 = DATA + 0x1_1000;

    pub(crate) const MEMCARD_CARD: u32 = DATA + 0x1_2000;

    #[test]
    fn c_fe_state_mem_card_scan_step_matches() {
        check_all(
            ee::s02::f_00187a38__generated,
            c_fe_state_mem_card_scan_step,
            24,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x187a_38, 10);
                c.g[A0][0] = sx(MEMCARD);
                c.bus.write32(0x004a_28a8, WORLD_5);
                c.bus.write32(WORLD_5 + 0xa4, MEMCARD_MP);
                c.bus.write32(MEMCARD_MP + 0x434, MEMCARD_CARD);
                c.bus.write32(MEMCARD_MP + 0x428, (seed % 2) as u32);
                c.bus.write32(0x004a_14bc, seed % 4); // scan step
                c.bus.write32(MEMCARD + 0x1bc, r.next_u32());
                c.bus.write32(MEMCARD + 0x234, r.next_u32() % 4);
                let _ = &mut r;
            },
        );
    }

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_state_case;

    #[test]
    fn c_fe_state_mem_card_ctor_2_matches() {
        heap_state_case(
            ee::s04::f_001de950__generated,
            c_fe_state_mem_card_ctor_2,
            0x001d_e950,
            false,
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::scramble;
    use std::ptr::addr_of_mut;

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC709_MAGIC` (otherwise the generated body runs).
    pub(crate) const OC709_CTRL: u32 = DATA + 0x0020_0000;

    pub(crate) const OC709_MAGIC: u32 = 0x0709_c0de;

    /// Per-call results of the path-exists fake (four words, one per call index).
    pub(crate) const OC709_RES: u32 = DATA + 0x0020_1000;

    pub(crate) const OC709_UI: u32 = DATA + 0x0020_3000;

    pub(crate) const OC709_OBJ: u32 = DATA + 0x0020_4000;

    pub(crate) const OC709_OBJ2: u32 = DATA + 0x0020_5000;

    pub(crate) const OC709_MGR: u32 = DATA + 0x0020_6000;

    /// The bank object the allocator fake returns.
    pub(crate) const OC709_BANK: u32 = DATA + 0x0030_0000;

    pub(crate) fn oc709_open(c: &mut Cpu) -> bool {
        c.bus.read32(OC709_CTRL) == OC709_MAGIC
    }

    pub(crate) fn oc709_fake_317670(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_317670),
                ee::s10::f_00317670__generated,
            );
        }
        fakes::fake_call(c, 0x0031_7670);
    }

    pub(crate) fn oc709_fake_39d860(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_39d860),
                ee::s13::f_0039d860__generated,
            );
        }
        fakes::fake_call(c, 0x0039_d860);
    }

    pub(crate) fn oc709_fake_227f80(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_227f80),
                ee::s05::f_00227f80__generated,
            );
        }
        fakes::fake_call(c, 0x0022_7f80);
    }

    pub(crate) fn oc709_fake_241e18(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_241e18),
                ee::s06::f_00241e18__generated,
            );
        }
        fakes::fake_call(c, 0x0024_1e18);
    }

    pub(crate) fn oc709_fake_417828(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_417828),
                ee::s15::f_00417828__generated,
            );
        }
        fakes::fake_call(c, 0x0041_7828);
    }

    pub(crate) fn oc709_fake_3e18c8(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_3e18c8),
                ee::s14::f_003e18c8__generated,
            );
        }
        fakes::fake_call(c, 0x003e_18c8);
    }

    /// `EAFile_PathExists`: answers the per-call word the setup wrote.
    pub(crate) fn oc709_fake_317f38(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_317f38),
                ee::s10::f_00317f38__generated,
            );
        }
        let n = c.bus.read32(fakes::LOG_COUNT);
        let exists = c.bus.read32(OC709_RES + 4 * (n % 4));
        fakes::fake_call(c, 0x0031_7f38);
        c.g[V0][0] = exists as u64;
    }

    /// The bank allocator: a fixed object (the constructor runs on it).
    pub(crate) fn oc709_fake_317d70(c: &mut Cpu) {
        if !oc709_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc709_fake_317d70),
                ee::s10::f_00317d70__generated,
            );
        }
        fakes::fake_call(c, 0x0031_7d70);
        c.g[V0][0] = OC709_BANK as u64;
    }

    pub(crate) fn oc709_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc709_fake_317670,
            fakes::prev_cell(oc709_fake_317670),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc709_fake_39d860,
            fakes::prev_cell(oc709_fake_39d860),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00227F80),
            oc709_fake_227f80,
            fakes::prev_cell(oc709_fake_227f80),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241E18),
            oc709_fake_241e18,
            fakes::prev_cell(oc709_fake_241e18),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc709_fake_417828,
            fakes::prev_cell(oc709_fake_417828),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E18C8),
            oc709_fake_3e18c8,
            fakes::prev_cell(oc709_fake_3e18c8),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317F38),
            oc709_fake_317f38,
            fakes::prev_cell(oc709_fake_317f38),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc709_fake_317d70,
            fakes::prev_cell(oc709_fake_317d70),
        );
    }

    #[test]
    fn cfe_state_mem_card_v26_matches() {
        oc709_install_fakes();
        check(
            ee::s02::f_001876a8__generated,
            cfe_state_mem_card_v26,
            300,
            &|c, seed| {
                let mut r = Rng(0x0018_76a8 ^ seed as u64);
                scramble(c, &mut r);
                let _ = fakes::machine(c, seed, 0x0018_76a8, 0);
                c.pc = 0x0018_76a8;
                c.bus.write32(OC709_CTRL, OC709_MAGIC);
                c.bus.write32(GAME_GP - 0x1c38, OC709_UI);
                // Call 1 returns the UI object, call 2 the manager.
                c.bus.write32(fakes::LOG_RESULTS + 4, OC709_OBJ);
                c.bus.write32(fakes::LOG_RESULTS + 8, OC709_MGR);
                c.bus.write32(OC709_OBJ + 8, OC709_OBJ2);
                c.bus
                    .write16(OC709_OBJ2 + 0x48, (0x10 * (seed % 3 + 1)) as u16);
                c.bus.write32(OC709_OBJ2 + 0x4c, fakes::FAKE);
                let kind = if seed % 4 == 0 {
                    r.next_u32()
                } else {
                    CFE_MEM_CARD_KINDS[(seed % 13) as usize] as u32
                };
                c.bus.write32(OC709_MGR + 0x130, kind);
            },
        );
    }

    use crate::difftest::fakes::covered;
    pub(crate) use crate::game::sound::bankmanager::tests::oc782_fixture;
    pub(crate) use crate::game::sound::bankmanager::tests::oc782_force;

    pub(crate) use crate::game::sound::bankmanager::tests::OC782_OBJ;
    pub(crate) use crate::game::sound::soundsys::tests::oc771_fake_call;
    pub(crate) use crate::game::sound::soundsys::tests::OC771_CTRL;
    pub(crate) use crate::game::sound::soundsys::tests::OC771_MAGIC;

    /// Base of the fixtures' data (clear of the fakes' call log at DATA+0x100000).
    pub(crate) const OC771_BASE: u32 = DATA + 0x22_0000;

    /// Force the result of callee `idx` (a gated fake's index, or 16 + the
    /// vtable slot number of a fake method) to `v`, sign-extended.
    pub(crate) fn oc771_mail(c: &mut Cpu, idx: u32, v: u32) {
        c.bus.write32(OC771_CTRL + 0x10 + 8 * idx, 1);
        c.bus.write32(OC771_CTRL + 0x14 + 8 * idx, v);
    }

    /// The machine's miss hook: fake methods at `FAKE + 0x10 * k` are
    /// callee `16 + k`.
    pub(crate) fn oc771_miss(c: &mut Cpu, target: u32) {
        let idx = 16 + target.wrapping_sub(fakes::FAKE) / 0x10;
        oc771_fake_call(c, target, idx.min(31));
    }

    /// `cFEStateMemCard_CheckFreeSpace`: the failed-before state (stage below
    /// or above 4), a critical card error, and the free-space / directory-entry
    /// checks (profile file needed or not, results of -1, 0, 2), with the
    /// faked manager, card and vtable calls (which may unwind).
    #[test]
    fn c_fe_state_mem_card_check_free_space_matches() {
        oc771_install_fakes();
        covered(
            ee::s02::f_00186f90__generated,
            c_fe_state_mem_card_check_free_space,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x186f90, 8);
                c.hooks.miss = oc771_miss;
                c.pc = 0x0018_6f90;
                c.bus.write32(OC771_CTRL, OC771_MAGIC);
                let gp = GAME_GP;
                let (state, mgr, card, vt, vt2) = (
                    OC771_BASE + 0x100,
                    OC771_BASE + 0x1000,
                    OC771_BASE + 0x2000,
                    OC771_BASE + 0x3000,
                    OC771_BASE + 0x5000,
                );
                c.bus.write32(gp.wrapping_sub(0x1c38), state);
                c.bus.write32(gp.wrapping_sub(0x1c34), seed / 3 % 8);
                c.bus.write32(gp.wrapping_sub(0x848), OC771_BASE + 0x800);
                c.bus
                    .write32(state + MEMCARD_SPACE_FAILED, (seed % 3 == 0) as u32);
                c.bus.write32(
                    state + MEMCARD_FILES_FLAGS,
                    r.next_u32() & 0xf | (seed / 2) % 2,
                );
                c.bus.write32(mgr + 0x428, r.next_u32());
                c.bus.write32(mgr + 0xf8, r.next_u32());
                c.bus.write32(mgr + 0x434, card);
                c.bus.write32(mgr + 0x748, vt2);
                c.bus.write16(vt2 + 8, (r.next_u32() & 0x3c) as u16);
                c.bus.write32(vt2 + 0xc, fakes::FAKE + 0x30);
                c.bus.write32(card, vt);
                for (slot, k) in [(0x70u32, 4u32), (0xb0, 1), (0xc8, 2)] {
                    c.bus.write16(vt + slot, (r.next_u32() & 0x7c) as u16);
                    c.bus.write32(vt + slot + 4, fakes::FAKE + 0x10 * k);
                }
                oc771_mail(c, 5, mgr);
                oc771_mail(c, 6, ((seed / 24) % 5 == 0) as u32);
                oc771_mail(c, 9, if (seed / 5) % 2 == 0 { 5 } else { 100 });
                oc771_mail(c, 10, 40);
                oc771_mail(c, 17, if (seed / 7) % 2 == 0 { 10 } else { 60 });
                oc771_mail(c, 18, [0xffff_ffff, 0, 2][(seed / 11) as usize % 3]);
            },
        );
    }

    /// `cFEStateMemCard_StartProfileSearch`: memory card manager (a forced
    /// pointer) with a vtable entry for the state call, fakes for the rest;
    /// the UI object and animation lookups return 0 for some calls (mask).
    #[test]
    fn fe_state_mem_card_start_profile_search_matches() {
        fakes::covered(
            ee::s02::f_001877b0__generated,
            fe_state_mem_card_start_profile_search,
            96,
            &|c, seed| {
                let mut r = oc782_fixture(c, seed, 0x782a, 17);
                let inst = OC782_OBJ + 0x1000;
                let mgr = OC782_OBJ + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x1c38), inst);
                c.bus.write32(GAME_GP.wrapping_sub(0x1c3c), r.next_u32());
                c.bus.write32(inst + 0x10, r.next_u32() & 0xfffff0);
                c.bus.write32(inst + 0x40, r.next_u32() & 0xfffff0);
                let vt = fakes::vtable(c, &mut r, &[8]);
                c.bus.write32(mgr + 0x748, vt);
                oc782_force(c, 2, mgr);
            },
        );
    }

    pub(crate) use crate::game::fe::festatecredits::tests::oc830_arm;
    pub(crate) use crate::game::fe::festatecredits::tests::oc830_force;
    pub(crate) use crate::game::fe::festatecredits::tests::oc830_gp_set;

    pub(crate) use crate::game::fe::festatecredits::tests::OC830_V;
    pub(crate) use crate::game::fe::festateoptions::tests::oc832_machine;
    pub(crate) use crate::game::fe::festateoptions::tests::oc832_w;

    pub(crate) use crate::game::fe::festateoptions::tests::OC832_ENGINE;
    pub(crate) use crate::game::fe::festateoptions::tests::OC832_FORCE;

    pub(crate) use crate::game::fe::festateoptions::tests::OC832_OBJ;
    pub(crate) use crate::game::fe::festateoptions::tests::OC832_VT;
    pub(crate) use crate::game::fe::festateoptions::tests::OC832_WIDGET;

    /// The memory-card message objects: `MCManager_Get` returns the manager,
    /// the world and the popup hang off gp, all with the shared vtable.
    pub(crate) fn oc830_memcard_world(c: &mut Cpu) {
        const M: u32 = DATA + 0x23_0000;
        const P: u32 = DATA + 0x23_1000;
        const WORLD: u32 = DATA + 0x23_2000;
        const X: u32 = DATA + 0x23_3000;
        const G2: u32 = DATA + 0x23_4000;
        const H: u32 = DATA + 0x23_5000;
        oc830_force(c, 0x0022_7f80, M);
        c.bus.write32(M + 0x428, 0x777);
        c.bus.write32(M + 0x434, P);
        c.bus.write32(P, OC830_V);
        oc830_gp_set(c, 0x848, WORLD);
        c.bus.write32(WORLD + 0x8c, X);
        c.bus.write32(X + 0x4, OC830_V);
        oc830_gp_set(c, 0x1c38, G2);
        c.bus.write32(G2 + 0x8, OC830_V);
        c.bus.write32(G2 + 0x218, H);
        c.bus.write32(H + 0x8, OC830_V);
    }

    #[test]
    fn cfe_state_mem_card_show_not_detected_msg_matches() {
        oc830_install_fakes();
        check(
            ee::s02::f_00186d60__generated,
            cfe_state_mem_card_show_not_detected_msg,
            32,
            &|c, seed| {
                machine(c, seed, 0x18_6d60, 14);
                oc830_arm(c);
                c.pc = 0x0018_6d60;
                oc830_memcard_world(c);
            },
        );
    }

    #[test]
    fn cfe_state_mem_card_show_no_space_msg_matches() {
        oc830_install_fakes();
        check(
            ee::s02::f_00187360__generated,
            cfe_state_mem_card_show_no_space_msg,
            32,
            &|c, seed| {
                machine(c, seed, 0x18_7360, 12);
                oc830_arm(c);
                c.pc = 0x0018_7360;
                oc830_memcard_world(c);
            },
        );
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc832_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(OC832_FORCE + (addr & 0xf_fffc), 1);
    }

    /// `cFEStateMemCard_OnChildEvent`: every (state, key, flags) combination.
    #[test]
    fn cfe_state_mem_card_on_child_event_matches() {
        check(
            ee::s02::f_00187580__generated,
            cfe_state_mem_card_on_child_event,
            60,
            &|c, seed| {
                oc832_machine(c, seed, 0x187580, 8);
                c.pc = 0x0018_7580;
                let state = [0x15u32, 0xa, 0x4, 0x14, 7][(seed % 5) as usize];
                let key = [0xfu32, 0x10, 5][(seed / 5 % 3) as usize];
                let flags = seed / 15 % 4;
                c.g[A0][0] = sx(OC832_OBJ);
                c.g[A2][0] = key as u64;
                oc832_w(c, OC832_OBJ + 0x19c, state);
                oc832_w(c, OC832_OBJ + 0x238, flags);
                oc832_w(c, OC832_OBJ + 8, OC832_VT);
                c.bus.write16(OC832_VT + 0x148, 0x10 * (seed % 3) as u16);
                oc832_w(c, OC832_VT + 0x14c, FAKE + 0x10);
            },
        );
    }

    /// `cFEStateMemCard_StartSystemLoad`: with and without the load widget and
    /// its animation.
    #[test]
    fn cfe_state_mem_card_start_system_load_matches() {
        check(
            ee::s02::f_00187920__generated,
            cfe_state_mem_card_start_system_load,
            12,
            &|c, seed| {
                oc832_machine(c, seed, 0x187920, 13);
                c.pc = 0x0018_7920;
                oc832_w(c, GAME_GP.wrapping_sub(0x1c38), OC832_OBJ);
                oc832_w(c, OC832_OBJ + 0x10, OC832_ENGINE);
                oc832_w(c, OC832_OBJ + 0x40, OC832_WIDGET);
                match seed % 3 {
                    1 => oc832_force_zero(c, 0x0039_d860),
                    2 => oc832_force_zero(c, 0x0039_7140),
                    _ => {}
                }
            },
        );
    }

    /// `FEScreenAutosavePopup_ctor`.
    #[test]
    fn fe_screen_autosave_popup_ctor_matches() {
        check(
            ee::s02::f_00187c18__generated,
            fe_screen_autosave_popup_ctor,
            4,
            &|c, seed| {
                oc832_machine(c, seed, 0x187c18, 1);
                c.pc = 0x0018_7c18;
                c.g[A0][0] = sx(OC832_OBJ);
                oc832_w(c, OC832_OBJ + 0x48, 77);
            },
        );
    }

    /// `cFEStateMemCard_HandleInput` (0x0018_7c10) against the generated body.
    #[test]
    fn c_fe_state_mem_card_handle_input_00187c10_matches() {
        check(
            ee::s02::f_00187c10__generated,
            c_fe_state_mem_card_handle_input_00187c10,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x187c10, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateMemCard_OnWidgetAction` (0x001d_e940) against the generated body.
    #[test]
    fn c_fe_state_mem_card_on_widget_action_001de940_matches() {
        check(
            ee::s04::f_001de940__generated,
            c_fe_state_mem_card_on_widget_action_001de940,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de940, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateMemCard_v37` (0x001d_e948) against the generated body.
    #[test]
    fn c_fe_state_mem_card_v37_001de948_matches() {
        check(
            ee::s04::f_001de948__generated,
            c_fe_state_mem_card_v37_001de948,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1de948, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
