//! `src/fe/fememcard.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Frame;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::dirtysock::tags2::at;
pub(crate) use crate::game::dirtysock::tags2::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tags2::leave;
pub(crate) use crate::game::dirtysock::tags2::sw;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_3;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_2;
use crate::guest::Addr;
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
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::MEMCARD_SCAN_STEP;

    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;

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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

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
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::pool_7 as pool;

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
}
