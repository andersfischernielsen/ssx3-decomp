//! `src/fe/festatecharequip2.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::rider::world;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_4;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_2;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::scripter::ssxscriptengine::gp;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
use crate::guest::Addr;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_a238,
        cfe_state_char_equip_show_equip_mode,
        "cFEStateCharEquip_ShowEquipMode",
    ),
    (
        0x0019_a308,
        cfe_state_char_equip_show_buy_mode,
        "cFEStateCharEquip_ShowBuyMode",
    ),
    (
        0x0019_a3d0,
        cfe_state_char_equip_toggle_buy_equip,
        "cFEStateCharEquip_ToggleBuyEquip",
    ),
    (
        0x0019_a498,
        c_fe_state_char_equip_release_preview,
        "cFEStateCharEquip_ReleasePreview",
    ),
    (
        0x0019_a9b8,
        cfe_state_char_equip_refresh_item_info,
        "cFEStateCharEquip_RefreshItemInfo",
    ),
    (
        0x0019_aca0,
        c_fe_state_char_equip_refresh_rider_bolts,
        "cFEStateCharEquip_RefreshRiderBolts",
    ),
    (
        0x0019_b458,
        cfe_state_char_equip_remove_list_item,
        "cFEStateCharEquip_RemoveListItem",
    ),
    (
        0x0019_b518,
        c_fe_state_char_equip_fill_equip_rows,
        "cFEStateCharEquip_FillEquipRows",
    ),
    (
        0x0019_b598,
        c_fe_state_char_equip_fill_buy_rows,
        "cFEStateCharEquip_FillBuyRows",
    ),
    (
        0x0019_bba0,
        c_fe_state_char_equip_update_money_text,
        "cFEStateCharEquip_UpdateMoneyText",
    ),
    (
        0x0019_bbe0,
        cfe_state_char_equip_set_category_title,
        "cFEStateCharEquip_SetCategoryTitle",
    ),
    (
        0x0019_bc90,
        cfe_state_char_equip_update_stat_bar,
        "cFEStateCharEquip_UpdateStatBar",
    ),
    (
        0x0019_c7e8,
        cfe_state_char_equip_open_buy_popup,
        "cFEStateCharEquip_OpenBuyPopup",
    ),
    (
        0x0019_c8f8,
        bolt_file_index_manager_get_next_node,
        "BoltFileIndexManager_GetNextNode",
    ),
    (
        0x0019_cb60,
        bolt_file_index_manager_reset,
        "BoltFileIndexManager_Reset",
    ),
    (0x0019_ce68, boltfile_cache_init, "BoltFile_CacheInit"),
    (
        0x0019_cf40,
        bolt_file_cache_shutdown,
        "BoltFile_CacheShutdown",
    ),
    (0x0019_e238, bolt_file_is_loaded, "BoltFile_IsLoaded"),
    (
        0x0019_e538,
        femodule_rider_preview_set_visible,
        "FEModule_RiderPreviewSetVisible",
    ),
];

/// Same-shard direct callees used above.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;

    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;

    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_GET_RIDER_PREVIEW_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2 as UI_TEXT_SET_TEXT_ID;

    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;

    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;

    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;

    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;

    pub const BOLT_FILE_RELEASE: Callee = Callee {
        addr: 0x0019_dc20,
        f: ee::s03::f_0019dc20,
    };
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;

    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;

    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
}

// ---------------------------------------------------------------------------
// Bolt file stream (front-end file manager).

/// `bool BoltFile_IsLoaded(BoltFile *file, int outer, int inner)` (0x19e238):
/// reads the word at `file + outer*0x12000 + inner*0x120` and returns whether
/// it equals 1.
pub fn bolt_file_is_loaded(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = 0x10000;
    c.g[V1][0] = 0x120;
    c.g[V0][0] = c.g[V0][0] | 0x2000; // 0x12000: stride of the outer table
    c.mult(0, A2, A2, V1); // inner *= 0x120
    c.mult(0, A1, A1, V0); // outer *= 0x12000
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end module: rider previews.

/// `void FEModule_RiderPreviewSetVisible(FEModule *self, uint visible)`
/// (0x19e538): when `visible` is zero clears the +0xcc4/+0xcc8 flags; otherwise
/// sets +0xcc4 to 1 if any of the +0xcc0/+0xcb8/+0xcb4 words is zero or +0xcd4
/// is non-negative, else sets +0xcc8 to 1.
pub fn femodule_rider_preview_set_visible(c: &mut Cpu) {
    c.cycles += 2;
    let this = Addr(abi::arg(c, 0));
    if abi::arg64(c, 1) == 0 {
        this.off(0xcc4).sw(c, 0);
        c.cycles += 2;
        this.off(0xcc8).sw(c, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    let mut early;
    c.g[V0][0] = this.off(0xcc0).lw(c);
    early = c.g[V0][0] == 0;
    c.g[V0][0] = 0x1;
    if !early {
        c.cycles += 3;
        c.g[V0][0] = this.off(0xcb8).lw(c);
        early = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if !early {
            c.cycles += 3;
            c.g[V0][0] = this.off(0xcb4).lw(c);
            early = c.g[V0][0] == 0;
            c.g[V0][0] = 0x1;
            if !early {
                c.cycles += 3;
                c.g[V0][0] = this.off(0xcd4).lw(c);
                early = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = 0x1;
                if !early {
                    c.cycles += 2;
                    this.off(0xcc8).sw(c, c.g[V0][0]);
                    abi::ret(c);
                    return;
                }
            }
        }
    }
    c.cycles += 2;
    this.off(0xcc4).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Bolt file index manager.

/// Shared body of `BoltFileIndexManager_GetPrev/NextNode` (0x19c8d0/0x19c8f8):
/// follow the 16-bit node link at `node + link` and return `base + index*16`.
pub(crate) fn bolt_file_index_manager_get_node(c: &mut Cpu, link: u32) {
    c.cycles += 2;
    let node = abi::arg64(c, 1);
    if node != 0 {
        c.cycles += 3;
        let index = Addr(node as u32).off(link).lh(c);
        c.g[V0][0] = index;
        let negative = (index as i64) < 0;
        let scaled = sx((index as u32) << 4);
        c.g[V0][0] = scaled;
        if !negative {
            c.cycles += 2;
            c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(scaled as u32));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `Node* BoltFileIndexManager_GetNextNode(Node *base, Node *node)` (0x19c8f8).
pub fn bolt_file_index_manager_get_next_node(c: &mut Cpu) {
    bolt_file_index_manager_get_node(c, 0xe);
}

/// `void cFEStateCharEquip_UpdateMoneyText(State* s)` (0x19bba0): the same
/// with the text at `+0x64` and the value at `+0xc0`.
pub fn c_fe_state_char_equip_update_money_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0x64);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            c.g[A0][0] = lw(c, this, 0xc0);
            callee::FE_FORMAT_MONEY.call(c, 0x0019_bbc4)?;
            let this = crate::abi::r32(c, S0);
            c.g[A0][0] = lw(c, this, 0x64);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_STRING.call(c, 0x0019_bbd0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_ReleasePreview(self)` (0x19a498): release the
/// previewed equipment's bolt file (if a slot is held) and clear the preview
/// state.
pub fn c_fe_state_char_equip_release_preview(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[A2][0] = Addr(this).off(0xa6c).lw(c);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        if (c.g[A2][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[A1][0] = Addr(this).off(0xbc).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x7c).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
            BOLT_FILE_RELEASE.call(c, 0x0019_a4c4)?;
        }
        c.cycles += 8;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        Addr(this).off(0x960).sb(c, 0);
        Addr(this).off(0x95c).sw(c, c.g[V0][0]);
        Addr(this).off(0xa6c).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_RefreshItemInfo(cFEStateCharEquipDetail* s)`
/// (0x19a9b8): refresh the buy panel when the detail flags word at +0xc8 is
/// 1, else the equip panel, then the help text. 8 (+4, +2) + 6 cycles plus
/// the callees.
pub fn cfe_state_char_equip_refresh_item_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(p(self_, 0xc8));
        c.g[V0][0] ^= 1;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            callee::CHAR_EQUIP_REFRESH_BUY_INFO.call(c, 0x0019_a9e0)?;
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::CHAR_EQUIP_REFRESH_EQUIP_INFO.call(c, 0x0019_a9f0)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::CHAR_EQUIP_UPDATE_HELP_TEXT.call(c, 0x0019_a9f8)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Riders, cameras, memory cards.

/// `cFEStateCharEquip_RefreshRiderBolts` (0x19ACA0): ask the char-equip state's
/// rider preview for the rider's currently equipped bolts.
pub fn c_fe_state_char_equip_refresh_rider_bolts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x7c)).lw(c);
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x44)).lb(c);
        c.g[A0][0] = low(c, A0).wrapping_add(0xb0);
        call_ptr(c, addr::RIDER_PREVIEW, 0x0019_acc8)?;
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, addr::BE_GET_BE, 0x0019_acd0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 9;
        call_ptr(c, addr::BE_GET_INTERFACE, 0x0019_acdc)?;
        c.g[A2][0] = lw_at(c, S0, 0xbc);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x44)).lb(c);
        call_ptr(c, addr::BE_GET_INTERFACE_9, 0x0019_acec)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        c.g[A3][0] = 1;
        call_ptr(c, addr::RIDER_PREVIEW_BOLTS, 0x0019_ad00)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_FillEquipRows(self)` (0x0019b518): fill up to six
/// equip rows for the selected profile.
pub fn c_fe_state_char_equip_fill_equip_rows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(c, S2, 0x5c)).lw(c);
        c.g[V0][0] = Addr(at(c, S2, 0x958)).lw(c);
        c.g[S0][0] = Addr(at(c, V1, 0x98)).lbu(c);
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let none = c.g[V0][0] == 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW.call(c, 0x0019_b558)?;
                c.g[S1][0] = sx(word(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
                let six = c.g[V0][0] == 0;
                c.g[S0][0] = sx(word(c, S0).wrapping_add(1));
                if six {
                    break;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr(at(c, S2, 0x958)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_b550);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_FillBuyRows(self)` (0x0019b598): fill up to six
/// buy rows for the selected profile.
pub fn c_fe_state_char_equip_fill_buy_rows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(c, S2, 0x5c)).lw(c);
        c.g[V0][0] = Addr(at(c, S2, 0x958)).lw(c);
        c.g[S0][0] = Addr(at(c, V1, 0x98)).lbu(c);
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let none = c.g[V0][0] == 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW.call(c, 0x0019_b5d8)?;
                c.g[S1][0] = sx(word(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
                let six = c.g[V0][0] == 0;
                c.g[S0][0] = sx(word(c, S0).wrapping_add(1));
                if six {
                    break;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr(at(c, S2, 0x958)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_b5d0);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BoltFileIndexManager_Reset(BoltFileIndexManager* m)` (0x0019cb60):
/// clear the index and the tail table, point every one of the 400 entries at
/// the list head and mark the table's end with -1.
pub fn bolt_file_index_manager_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[A1][0] = sx(0xffff_ffff);
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[A2][0] = 0x1900;
        call_ptr(c, 0x003e_6448, 0x0019_cb80)?; // EA_MemSet
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1900));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x640;
        call_ptr(c, 0x003e_6448, 0x0019_cb90)?; // EA_MemSet
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f40), c.g[S0][0]);
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        loop {
            c.cycles += 10;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.sh((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[A1][0] = c.g[V1][0];
            c.sh((c.g[A0][0] as u32).wrapping_add(0xe), c.g[V1][0]);
            c.g[V0][0] = ((c.g[A1][0] as i64) < 400) as u64;
            c.sh((c.g[A0][0] as u32).wrapping_add(4), 0);
            c.sh((c.g[A0][0] as u32).wrapping_add(6), 0);
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            if !more {
                c.cycles += 6;
                c.g[V0][0] = sx(0xffff_ffff);
                c.g[RA][0] = c.ld(sp(c));
                c.sh((c.g[S0][0] as u32).wrapping_add(0x18fe), c.g[V0][0]);
                c.lq(S0, sp(c).wrapping_add(0x10));
                c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
                abi::ret(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_cba0);
            }
        }
    })
}

/// `void cFEStateCharEquip_OpenBuyPopup(self, item)` (0x19c7e8): allocate a buy
/// popup, fill it from the item's price/id and post it to the screen.
pub fn cfe_state_char_equip_open_buy_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x30);
        c.g[A3][0] = 0;
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x460000;
        frame.save(c, S2, 0x10);
        c.g[A0][0] = 0x70;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x790));
        call_ptr(c, table_callee::EA_MEM_ALLOC_NAMED, 0x0019_c81c)?;
        c.g[A1][0] = lw_2(c, S1, 0x10);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = Addr(c.g[S1][0] as u32).off(0x44).lb(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, table_callee::BUY_POPUP_CTOR, 0x0019_c830)?;
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0xe).lh(c);
        c.g[S2][0] = c.g[V0][0];
        let no_price = (c.g[A2][0] as i64) <= 0;
        c.g[A1][0] = lw_2(c, S0, 0x14);
        if !no_price {
            c.cycles += 2;
            c.g[V0][0] = 0xa;
            c.mult(0, 6, 6, 2);
        }
        c.cycles += 13;
        c.g[A3][0] = lw_2(c, S1, 0xc0);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x48));
        callee::BUY_POPUP_INIT_BUY_BOLT.call(c, 0x0019_c854)?;
        c.g[A0][0] = lw_2(c, S1, 0x10);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, table_callee::UI_OBJECT_PUSH, 0x0019_c864)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_SetCategoryTitle(self, int id)` (0x19bbe0): write
/// the category's localised title into the screen's text widget; `id` picks a
/// table entry by hash, with a special case for the "default" category id.
pub fn cfe_state_char_equip_set_category_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = Addr(p(this as u32, 0x58)).lw(c);
        let no_text = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if no_text {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, GP), 0xffff_e0e0)).lw(c);
            let default_id = c.g[S1][0] != c.g[V0][0];
            c.g[A0][0] = 0x460000;
            if !default_id {
                // the "default" category's own name
                c.cycles += 7;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9b0));
                call_ptr(c, 0x0031_7670, 0x0019_bc18)?;
                c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x58)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x0019_bc24)?;
                frame.restore(c, S0, 0x30);
            } else {
                // look `id` up in the BE entry table and hash its name
                c.cycles += 10;
                call_ptr(c, 0x0014_bdb8, 0x0019_bc34)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xbc)).lw(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x0014_d448, 0x0019_bc48)?;
                c.g[A0][0] = c.g[V0][0];
                let found = (c.g[A0][0] as i64) >= 0;
                c.g[V0][0] = 0;
                if found {
                    c.cycles += 4;
                    c.g[V0][0] = 0x38;
                    c.g[V1][0] = Addr(p(c.g[S2][0] as u32, 4)).lw(c);
                    c.mult(0, 2, 4, 2);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                }
                c.cycles += 5;
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x14)).lw(c);
                call_ptr(c, 0x0031_7670, 0x0019_bc6c)?;
                c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x58)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x0019_bc78)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_UpdateStatBar(self)` (0x19bc90): animate the
/// stat-bar fill (0..1 normalised by the global max) and hand the value to
/// the bar widget.
pub fn cfe_state_char_equip_update_stat_bar(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.swc1(20, p(c.g[SP][0] as u32, 0x30));
        c.g[A2][0] = Addr(p(this as u32, 0x70)).lw(c);
        let no_bar = c.g[A2][0] == 0;
        c.g[V1][0] = Addr(p(r32(c, GP), 0xffff_e7e8)).lw(c);
        if !no_bar {
            c.cycles += 13;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0xc4)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5dc));
            c.mtc1(0, 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5dc));
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.mtc1(3, 0);
            c.cvt_s_w(0, 0);
            c.div_s(20, 1, 0);
            c.c_lt_s(20, 2);
            if c.fcc() {
                c.mov_s(20, 2);
            }
            c.cycles += 19;
            c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 8)).lw(c);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_bcfc)?;
            c.neg_s(1, 20);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x70)).lw(c);
            c.swc1(0, p(c.g[SP][0] as u32, 8));
            c.swc1(1, p(c.g[SP][0] as u32, 4));
            c.swc1(0, c.g[SP][0] as u32);
            c.ldl(3, p(c.g[SP][0] as u32, 7));
            c.ldr(3, c.g[SP][0] as u32);
            c.g[A0][0] = Addr(p(c.g[SP][0] as u32, 8)).lw(c);
            c.sdl(3, p(c.g[V0][0] as u32, 0x57));
            c.sdr(3, p(c.g[V0][0] as u32, 0x50));
            c.sw(p(c.g[V0][0] as u32, 0x58), c.g[A0][0]);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore_ra(c, 0x10);
        c.lwc1(20, p(c.g[SP][0] as u32, 0x30));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE lists.

/// `int cFEStateCharEquip_RemoveListItem(cFEStateCharEquip* self, int idx)`
/// (0x19b458): drop entry `idx` from the array at +0x124 (`count` at +0x958),
/// zero the vacated tail slot and clamp the list view (`top` at list+0x98,
/// `sel` at list+0x95). Returns 1, or 0 when `idx` is out of range.
pub fn cfe_state_char_equip_remove_list_item(c: &mut Cpu) {
    c.cycles += 5;
    let this = c.g[A0][0];
    c.g[T3][0] = this;
    let count = Addr(this as u32).off(0x958).lw(c);
    c.g[V1][0] = count;
    let in_range = (c.g[A1][0] as i64) < (count as i64);
    c.g[V0][0] = in_range as u64;
    c.g[V0][0] = sx((count as u32).wrapping_sub(1));
    if !in_range {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    let shift = (c.g[A1][0] as i64) < (c.g[V0][0] as i64);
    c.g[V0][0] = shift as u64;
    let items = sx((this as u32).wrapping_add(0x124));
    c.g[A2][0] = items;
    if shift {
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(items as u32));
        loop {
            c.cycles += 8;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
            let n = Addr(this as u32).off(0x958).lw(c);
            c.g[V0][0] = n;
            c.g[V0][0] = sx((n as u32).wrapping_sub(1));
            let more = (c.g[A1][0] as i64) < (c.g[V0][0] as i64);
            c.g[V0][0] = more as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_b488);
            }
        }
    }
    c.cycles += 27;
    c.g[A0][0] = Addr(this as u32).off(0x958).lw(c);
    c.g[T1][0] = u64::MAX;
    c.g[V0][0] = 1;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    Addr(this as u32).off(0x958).sw(c, c.g[A0][0]);
    c.g[V1][0] = sx((items as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, 0);
    c.g[A3][0] = Addr(this as u32).off(0x5c).lw(c);
    c.g[A2][0] = Addr(this as u32).off(0x958).lw(c);
    c.g[A0][0] = Addr(c.g[A3][0] as u32).off(0x98).lbu(c);
    c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_sub(6));
    c.g[A1][0] = Addr(c.g[A3][0] as u32).off(0x95).lbu(c);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(6));
    c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
    c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[A0][0] = c.g[T2][0];
    }
    c.g[A2][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
    c.g[V1][0] = ((c.g[T1][0] as i64) < (c.g[A0][0] as i64)) as u64;
    if c.g[A2][0] == 0 {
        c.g[A1][0] = c.g[T0][0];
    }
    if c.g[V1][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[T1][0] = ((c.g[T1][0] as i64) < (c.g[A1][0] as i64)) as u64;
    Addr(c.g[A3][0] as u32).off(0x98).sb(c, c.g[A0][0]);
    if c.g[T1][0] == 0 {
        c.g[A1][0] = 0;
    }
    c.g[V1][0] = Addr(this as u32).off(0x5c).lw(c);
    let ra = c.g[RA][0] as u32;
    Addr(c.g[V1][0] as u32).off(0x95).sb(c, c.g[A1][0]);
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// `BoltFile_CacheShutdown` (0x0019cf40).
//
// `void BoltFile_CacheShutdown(BoltFile* self)`: free every cached slot of
// every bolt file and drop the cache.

pub fn bolt_file_cache_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V0][0] = 0xb0000;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = c.g[V0][0] | 0x4008;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = c.g[A0][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = 0u64;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S6][0] = 0x1;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        'l_19cf80: loop {
            c.cycles += 4;
            c.g[S0][0] = 0u64;
            c.g[S2][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.g[A2][0] = c.g[S0][0];
            'l_19cf90: loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0019cfa0);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x0019d4b8, 0x0019cfa0)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 256i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A2][0] = c.g[S0][0];
                if t {
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x0019cf90);
                    }
                    continue 'l_19cf90;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[S1][0] = c.g[S3][0];
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 10i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.sw(c.g[V1][0] as u32, c.g[S6][0]);
                    if t {
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x0019cf80);
                        }
                        continue 'l_19cf80;
                    } else {
                        'b_19cfd8: {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                            let t = c.g[A0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                            if t {
                                break 'b_19cfd8;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x0019cfd4);
                                call_ptr(c, 0x00317e50, 0x0019cfd4)?;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                                break 'b_19cfd8;
                            }
                        }
                        c.cycles += 10;
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1788), 0u64);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `cFEStateCharEquip_ShowBuyMode(this)` (0x19a308): relabel the money text
/// and dismiss the three item panels before showing the rider preview.
pub fn cfe_state_char_equip_show_buy_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[V0][0] = Addr(this).off(0x48).lw(c);
        let no_text = c.g[V0][0] == 0;
        c.g[A0][0] = 0x0046_0000;
        if !no_text {
            c.cycles += 5;
            c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x810));
            callee::STR_HASH.call(c, 0x0019_a32c)?;
            c.g[A0][0] = Addr(this).off(0x48).lw(c);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_TEXT_ID.call(c, 0x0019_a338)?;
        }

        // The item panel at +0x80 (close with 0).
        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0x80).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a358)?;
        }

        // The item panel at +0x6c.
        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0x6c).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        if !none {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a37c)?;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        }

        // Hide the rider preview, then the panel at +0x60 (close with 1).
        c.cycles += 10;
        c.g[A1][0] = Addr(this).off(0x44).lb(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x7c).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        callee::FE_MODULE_GET_RIDER_PREVIEW.call(c, 0x0019_a390)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE.call(c, 0x0019_a39c)?;
        c.g[A2][0] = Addr(this).off(0x60).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 1;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a3bc)?;
        }

        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Character-equip screen.

/// `void cFEStateCharEquip_ToggleBuyEquip(cFEStateCharEquip *self)` (0x19a3d0):
/// flip the `is-buy` flag, switch the detail panel's mode, retitle the
/// category, rebuild and refresh the item list.
pub fn cfe_state_char_equip_toggle_buy_equip(c: &mut Cpu) {
    guest(c, |c| {
        'b_19a450: {
            'b_19a44c: {
                'b_19a420: {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_19a420;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x0019a3f8);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc8), 0u64);
                        call_ptr(c, 0x0019a238, 0x0019a3f8)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0x460000;
                        if t {
                            break 'b_19a44c;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x0019a40c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x810));
                            call_ptr(c, 0x00317670, 0x0019a40c)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                            c.g[RA][0] = sx(0x0019a418);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x0019a418)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f20));
                            if t {
                                break 'b_19a450;
                            } else {
                                break 'b_19a420;
                            }
                        }
                    }
                }
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0019a42c);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc8), c.g[V0][0]);
                call_ptr(c, 0x0019a308, 0x0019a42c)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = 0x460000;
                if t {
                    break 'b_19a44c;
                } else {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0019a440);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x820));
                    call_ptr(c, 0x00317670, 0x0019a440)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[RA][0] = sx(0x0019a44c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x0019a44c)?;
                    break 'b_19a44c;
                }
            }
            c.cycles += 1;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f20));
            break 'b_19a450;
        }
        c.cycles += 18;
        c.g[A2][0] = 0x50;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x120), 0u64);
        c.g[RA][0] = sx(0x0019a460);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd0));
        call_ptr(c, 0x003e6448, 0x0019a460)?;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f20));
        c.g[RA][0] = sx(0x0019a46c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0019bbe0, 0x0019a46c)?;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f20));
        c.g[RA][0] = sx(0x0019a478);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0019b180, 0x0019a478)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x1;
        c.g[RA][0] = sx(0x0019a488);
        c.g[A2][0] = 0x1;
        call_ptr(c, 0x0019b618, 0x0019a488)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateCharEquip_ShowEquipMode(cFEStateCharEquip *self)` (0x19a238):
/// refresh the equip-mode labels, the preview panel, and the rider preview.
pub fn cfe_state_char_equip_show_equip_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p_2(c.g[S0][0], 0x48)).lw(c);
        let no_label = c.g[V0][0] == 0;
        c.g[A0][0] = 0x0046_0000;
        if !no_label {
            c.cycles += 5;
            c.g[A0][0] = 0x0046_0820;
            call_ptr(c, 0x0031_7670, 0x0019_a25c)?;
            c.g[A0][0] = Addr(p_2(c.g[S0][0], 0x48)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x0019_a268)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p_2(c.g[S0][0], 0x60)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(p_2(c.g[A2][0], 0x8)).lw(c);
            c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x48)).lh(c);
            c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a288)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p_2(c.g[S0][0], 0x88)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(p_2(c.g[A2][0], 0x8)).lw(c);
            c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x48)).lh(c);
            c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a2a8)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p_2(c.g[S0][0], 0x80)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[V0][0] = sx(world(c));
        if !none {
            c.cycles += 7;
            c.g[V0][0] = Addr(p_2(c.g[A2][0], 0x8)).lw(c);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x48)).lh(c);
            c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a2cc)?;
            c.g[V0][0] = sx(world(c));
        }
        c.cycles += 13;
        c.g[A1][0] = Addr(p_2(c.g[S0][0], 0x44)).lb(c);
        c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x7c)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        callee::FEMODULE_GET_RIDER_PREVIEW_2.call(c, 0x0019_a2e0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE.call(c, 0x0019_a2ec)?;
        c.g[V0][0] = 0x1;
        frame.restore_ra(c, 0);
        Addr(p_2(c.g[S0][0], 0xae0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[RA][0] = c.g[RA][0];
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bolt cache.

/// `void BoltFile_CacheInit(void* table)` (0x19ce68): clear the ten 256-entry
/// bolt rows and their counts, then create (or reset) the global index
/// manager.
pub fn boltfile_cache_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x10));
        c.g[V0][0] = 0xb0000;
        c.g[T2][0] = c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | 0x4008;
        c.g[A2][0] = 0x10000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[T0][0] = sx(p(r32(c, T2), r32(c, V0)));
        c.g[A0][0] = 0;
        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
        c.g[T1][0] = 1;
        c.g[A2][0] = c.g[A2][0] | 0x2000;
        loop {
            c.cycles += 6;
            c.mult(0, 2, 4, 6);
            c.g[A1][0] = sx(r32(c, A0) << 2);
            c.g[V1][0] = sx(p(r32(c, V0), r32(c, T2)));
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(1));
            c.g[A0][0] = 0xff;
            loop {
                c.cycles += 10;
                c.sw(r32(c, V1), 0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                Addr(p(r32(c, V1), 4)).sb(c, 0);
                Addr(p(r32(c, V1), 0x104)).sw(c, 0);
                Addr(p(r32(c, V1), 0x118)).sw(c, 0);
                Addr(p(r32(c, V1), 0x108)).sw(c, 0);
                Addr(p(r32(c, V1), 0x114)).sw(c, 0);
                Addr(p(r32(c, V1), 0x110)).sw(c, c.g[A3][0]);
                let more = (c.g[A0][0] as i64) >= 0;
                c.g[V1][0] = sx(p(r32(c, V1), 0x120));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_ceb0);
                }
            }
            c.cycles += 5;
            c.g[V1][0] = sx(p(r32(c, T0), r32(c, A1)));
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
            let more = c.g[V0][0] != 0;
            Addr(r32(c, V1)).sw(c, c.g[T1][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_ce98);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = 0xb0000;
        c.g[A0][0] = Addr(gp(c, -0x1788)).lw(c);
        c.g[V0][0] = sx(p(r32(c, T2), r32(c, V0)));
        Addr(p(r32(c, V0), 0x4000)).sw(c, 0);
        let have = c.g[A0][0] != 0;
        Addr(p(r32(c, V0), 0x4004)).sw(c, 0);
        if have {
            c.cycles += 2;
            call_ptr(c, 0x0019_cb60, 0x0019_cf34)?;
        } else {
            c.cycles += 10;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x1f44;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x9c0));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0019_cf1c)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0019_cb38, 0x0019_cf24)?;
            Addr(gp(c, -0x1788)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(r32(c, SP));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `BoltFile_Release(self, slot, layer)` (0x19dc20): drop one reference on a
/// bolt-file slot's entry; at zero either clear the slot (when it is not in
/// the loading state) or unpack it and free the slot.
pub fn boltfile_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        let _this = abi::arg(c, 0);
        let _slot = abi::arg(c, 1);
        let _layer = abi::arg(c, 2);
        c.g[A3][0] = 0x1_0000;
        frame.save(c, 18, 0x30);
        c.g[A3][0] |= 0x2000;
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.mult(0, V0, S2, A3); // v0 = slot * 0x12000
        c.g[S3][0] = c.g[A2][0];
        c.g[V1][0] = 0x120;
        frame.save(c, 17, 0x40);
        c.mult(0, V1, S3, V1); // v1 = layer * 0x120
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x50);
        c.sd(p(sp(c), 0x10), c.g[RA][0]);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[S0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let entry = c.g[S0][0] as u32;
        c.g[V0][0] = lw_4(c, p(entry, 0x114));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let t = (c.g[V0][0] as i64) > 0;
        c.sw(p(entry, 0x114), c.g[V0][0]);
        let mut skip_one = false;
        if !t {
            c.cycles += 4;
            c.g[V1][0] = lw_4(c, entry);
            c.g[V0][0] = 2;
            let mut go = true;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = lw_4(c, p(entry, 0x104));
            } else {
                c.cycles += 5;
                call_ptr(c, 0x003d_f980, 0x0019_dc88)?;
                c.g[V1][0] = 1;
                if c.g[V0][0] != c.g[V1][0] {
                    skip_one = true;
                    go = false;
                } else {
                    c.cycles += 16;
                    c.g[A0][0] = lw_4(c, p(entry, 0x108));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0019_dca4);
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x003d_f9d8, 0x0019_dca4)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0019_dcb4);
                    c.g[A2][0] = c.g[S0][0];
                    c.pc = 0x0019_d000;
                    ee::s03::BoltFile_UnpackSlot(c);
                    if c.pc != 0x0019_dcb4 {
                        return Err(Unwound);
                    }
                    c.sw(p(c.g[S0][0] as u32, 0x10c), 0);
                    c.g[V1][0] = 0xb_0000;
                    c.sw(c.g[S0][0] as u32, 0);
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = lw_4(c, p(c.g[V1][0] as u32, 0x4004));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.sw(p(c.g[V1][0] as u32, 0x4004), c.g[V0][0]);
                    c.g[V0][0] = lw_4(c, p(c.g[S0][0] as u32, 0x104));
                }
            }
            if go {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.sw(c.g[S0][0] as u32, 0);
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0019_dcec);
                    c.g[A2][0] = c.g[S3][0];
                    c.pc = 0x0019_d4b8;
                    ee::s03::BoltFile_FreeSlot(c);
                    if c.pc != 0x0019_dcec {
                        return Err(Unwound);
                    }
                }
            }
        }
        if !skip_one {
            c.cycles += 1;
        }
        frame.restore(c, 16, 0x50);
        c.cycles += 6;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::computer::tests::wb;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear;
    pub(crate) use crate::game::be::bedbprofile::tests::strict as strict_2;
    pub(crate) use crate::game::be::beintplayer::tests::w8;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::bb;

    pub(crate) use crate::game::dirtysock::tags2::tests::world_2 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS;

    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_2;

    pub(crate) use crate::game::fe::festaterewards::tests::base_4;

    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::ovstates::tests::base_3 as base_6;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2 as belib;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_8;

    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn bolt_file_is_loaded_matches() {
        check_lohi(
            ssx3_ee::s03::f_0019e238,
            bolt_file_is_loaded,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0001 ^ seed as u64);
                scramble(c, &mut r);
                let outer = (seed % 2) as u32;
                let inner = ((seed / 2) % 4) as u32;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = outer as u64;
                c.g[A2][0] = inner as u64;
                let base = DATA
                    .wrapping_add(outer.wrapping_mul(0x12000))
                    .wrapping_add(inner.wrapping_mul(0x120));
                c.bus
                    .write32(base, if seed % 3 == 0 { 1 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn femodule_rider_preview_set_visible_matches() {
        check(
            ssx3_ee::s03::f_0019e538,
            femodule_rider_preview_set_visible,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_0004 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 5 == 0 {
                    0
                } else {
                    (r.next_u32() as u64) | 1
                };
                for (k, off) in [0xcc0u32, 0xcb8, 0xcb4, 0xcd4].iter().enumerate() {
                    let v = if (seed >> k) & 1 == 0 {
                        0
                    } else if k == 3 && (seed >> 6) & 1 == 0 {
                        0x8000_0000
                    } else {
                        r.next_u32() | 1
                    };
                    c.bus.write32(DATA + off, v);
                }
                c.bus.write32(DATA + 0xcc4, r.next_u32());
                c.bus.write32(DATA + 0xcc8, r.next_u32());
            },
        );
    }

    #[test]
    fn bolt_file_index_manager_get_next_node_matches() {
        check(
            ssx3_ee::s03::f_0019c8f8,
            bolt_file_index_manager_get_next_node,
            200,
            &|c, seed| {
                let mut r = Rng(0x19c9 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let node = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                c.g[A1][0] = node as u64;
                if node != 0 {
                    let val = if seed % 2 == 0 {
                        r.next_u32() & 0x7fff
                    } else {
                        0x8000 | (r.next_u32() & 0x7fff)
                    };
                    c.bus.write16(node + 0xe, val as u16);
                }
            },
        );
    }

    // ---------------------------------------------------------- money text

    pub(crate) fn money_text_test(
        generated: GuestFn,
        hand: GuestFn,
        text_off: u32,
        value_off: u32,
        salt: u64,
    ) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            // The money formatter asks the BE library; a zeroed singleton
            // keeps it from lazily loading the rewards file from disc.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
            c.g[A0][0] = sx(DATA);
            if seed % 3 == 0 {
                c.bus.write32(DATA + text_off, 0);
            } else {
                c.bus.write32(DATA + text_off, DATA + 0x3000);
            }
            c.bus.write32(
                DATA + value_off,
                if seed % 4 == 0 {
                    0
                } else {
                    r.next_u32() % 100_000
                },
            );
            // The formatted string is a static buffer in the game image; the
            // text object at DATA + 0x3000 is only a target of the setter.
            c.bus.write32(DATA + 0x3000, 0);
        });
    }

    #[test]
    fn c_fe_state_char_equip_update_money_text_matches() {
        money_text_test(
            ssx3_ee::s03::f_0019bba0__generated,
            c_fe_state_char_equip_update_money_text,
            0x64,
            0xc0,
            0x1b,
        );
    }

    // -----------------------------------------------------------------------
    // 0x19a498 cFEStateCharEquip_ReleasePreview.

    #[test]
    fn c_fe_state_char_equip_release_preview_matches() {
        check(
            ee::s03::f_0019a498__generated,
            c_fe_state_char_equip_release_preview,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x19a4_98, 1);
                let this = DATA + 0x100;
                let world = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x7c, DATA + 0x2000);
                let idx = if seed % 2 == 0 { 0xffff_ffffu32 } else { 2 };
                c.bus.write32(this + 0xa6c, idx);
                c.bus.write32(this + 0xbc, 0); // slot 0: offsets collapse to +0x120
                                               // A live refcount so BoltFile_Release only decrements it.
                let bolt = DATA + 0x2000 + 0x1a70;
                let node = bolt + 2 * 0x120;
                c.bus.write32(node + 0x114, 5);
                c.bus.write32(node, 0);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_refresh_item_info_matches() {
        check(
            ee::s03::f_0019a9b8__generated,
            cfe_state_char_equip_refresh_item_info,
            24,
            &|c, seed| {
                let _r = base(c, seed, 0x19a9);
                c.bus.write32(DATA + 0xc8, (seed % 3) as u32);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    // ------------------------------------------------ riders, cards, misc

    #[test]
    fn c_fe_state_char_equip_refresh_rider_bolts_matches() {
        check(
            ee::s03::f_0019aca0__generated,
            c_fe_state_char_equip_refresh_rider_bolts,
            12,
            &|c, seed| {
                machine(c, seed, 0x19_aca0, 5);
                belib(c);
                world(c);
                heap_setup(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                bb(c, DATA + 0x2000 + 0x44, (seed % 3) as u8);
                w(c, DATA + 0x2000 + 0xbc, 7 + seed % 3);
                c.pc = 0x0019_aca0;
            },
        );
    }

    pub(crate) fn fill_rows_test(gen: GuestFn, hand: GuestFn, salt: u64, row_target: u32) {
        check(gen, hand, 8, &|c, seed| {
            let _ = alloc_base(c, seed, salt, 4);
            let self_ = DATA;
            let profile = DATA + 0x1000;
            w(c, self_ + 0x5c, profile);
            wb(c, profile + 0x98, (seed % 4) as u8);
            w(c, self_ + 0x958, (seed % 10) as u32);
            let _ = row_target;
            c.g[A0][0] = self_ as u64;
        });
    }

    #[test]
    fn c_fe_state_char_equip_fill_equip_rows_matches() {
        fill_rows_test(
            ee::s03::f_0019b518__generated,
            c_fe_state_char_equip_fill_equip_rows,
            0x19b5_18,
            0x0019_b7e0,
        );
    }

    #[test]
    fn c_fe_state_char_equip_fill_buy_rows_matches() {
        fill_rows_test(
            ee::s03::f_0019b598__generated,
            c_fe_state_char_equip_fill_buy_rows,
            0x19b5_98,
            0x0019_ba60,
        );
    }

    #[test]
    fn bolt_file_index_manager_reset_matches() {
        check(
            ee::s03::f_0019cb60__generated,
            bolt_file_index_manager_reset,
            60,
            &|c, seed| {
                let _ = base_3(c, seed, 0x19cb, 1);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_open_buy_popup_matches() {
        check_lohi(
            ee::s03::f_0019c7e8__generated,
            cfe_state_char_equip_open_buy_popup,
            30,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x19c7e8, 4);
                let obj = DATA + 0x1000;
                let item = DATA + 0x2000;
                let screen = DATA + 0x3000;
                let list = DATA + 0x4000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = item as u64;
                w(c, obj + 0x10, screen);
                w(c, obj + 0xc0, r.next_u32());
                w8(c, obj + 0x44, seed as u8);
                w(c, screen + 0x18, list);
                w16(
                    c,
                    item + 0xe,
                    if seed % 3 == 0 {
                        0
                    } else {
                        (seed % 0x100) as u16
                    },
                );
                w(c, item + 0x14, r.next_u32());
            },
        );
    }

    // ------------------------------------------------ more front end / scripts

    #[test]
    fn cfe_state_char_equip_set_category_title_matches() {
        check(
            ee::s03::f_0019bbe0__generated,
            cfe_state_char_equip_set_category_title,
            9,
            &|c, seed| {
                let _ = base_5(c, seed, 0x0019_bbe0, 3);
                w(c, GAME_GP.wrapping_sub(0x1f20), 0x1234);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0x1234
                } else {
                    (0x1000 + seed * 7) as u64
                };
                w(c, OBJ + 0x58, if seed % 4 == 0 { 0 } else { OBJ2 });
                w(c, OBJ + 0xbc, (seed % 5) as u32);
                w(c, OBJ2 + 0x18, 0);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_update_stat_bar_matches() {
        check(
            ee::s03::f_0019bc90__generated,
            cfe_state_char_equip_update_stat_bar,
            8,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x0019_bc90, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[GP][0] = GAME_GP as u64;
                w(c, GAME_GP.wrapping_sub(0x1818), 0x5dc + (seed % 200));
                w(c, OBJ + 0xc4, 0x5dc + (seed % 300));
                w(c, OBJ + 0x70, if seed % 4 == 0 { 0 } else { OBJ2 });
                let vt = vtable(c, &mut r, &[0x48]);
                w(c, OBJ2 + 8, vt);
                w(c, OBJ2 + 0x50, 0);
                w(c, OBJ2 + 0x54, 0);
                w(c, OBJ2 + 0x58, 0);
                c.f[20] = r.edge_f32();
            },
        );
    }

    // -------------------------------------------------------------- FE lists.

    #[test]
    fn cfe_state_char_equip_remove_list_item_matches() {
        check(
            ee::s03::f_0019b458__generated,
            cfe_state_char_equip_remove_list_item,
            200,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x19_b458);
                c.g[A0][0] = sx(DATA);
                let count = (seed % 6) as u32;
                let idx = r.next_u32() % (count + 2);
                c.g[A1][0] = sx(idx);
                c.bus.write32(DATA + 0x958, count);
                for i in 0..8 {
                    c.bus.write32(DATA + 0x124 + i * 4, r.next_u32());
                }
                let list = DATA + 0x1000;
                c.bus.write32(DATA + 0x5c, list);
                c.bus.write8(list + 0x95, (r.next_u32() % 9) as u8);
                c.bus.write8(list + 0x98, (r.next_u32() % 9) as u8);
            },
        );
    }

    #[test]
    fn bolt_file_cache_shutdown_matches() {
        strict(
            ee::s03::f_0019cf40__generated,
            bolt_file_cache_shutdown,
            6,
            &|c, _seed| {
                let _r = machine(c, 0, 0x19cf, 8);
                c.bus.write32(GAME_GP - 0x1788, 0);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_show_buy_mode_matches() {
        check(
            ee::s03::f_0019a308__generated,
            cfe_state_char_equip_show_buy_mode,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19a3, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus
                    .write32(this + 0x48, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                for (i, off) in [0x80u32, 0x6c, 0x60].iter().enumerate() {
                    if seed % (i as u32 + 2) != 0 {
                        let o = DATA + 0x2400 + 0x100 * i as u32;
                        c.bus.write32(this + *off, o);
                        let vt = vtable(c, &mut r, &[0x48]);
                        c.bus.write32(o + 8, vt);
                    }
                }
                c.bus.write32(this + 0x44, seed % 4);
                c.bus.write32(WORLD_2 + 0x7c, DATA + 0x2c00);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_show_equip_mode_matches() {
        check(
            ee::s03::f_0019a238__generated,
            cfe_state_char_equip_show_equip_mode,
            60,
            &|c, seed| {
                let _r = base_7(c, seed, 0x19a2);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 0x48, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.bus.write8(DATA + 0x44, seed as u8);
                for (field, ret) in [(0x60u32, 0x19a288u32), (0x88, 0x19a2a8), (0x80, 0x19a2cc)] {
                    let obj = DATA + field;
                    c.bus
                        .write32(DATA + field, if seed % 3 == 0 { 0 } else { obj });
                    c.bus.write32(obj + 8, obj + 0x100);
                    c.bus.write16(obj + 0x100 + 0x48, 0);
                    c.bus.write32(obj + 0x100 + 0x4c, FAKE + (ret & 0xf));
                }
                c.bus.write32(DATA + 0x2008, DATA + 0x2100);
                c.bus.write16(DATA + 0x2100 + 0x48, 0);
                c.bus.write32(DATA + 0x2100 + 0x4c, FAKE);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_toggle_buy_equip_matches() {
        strict_2(
            ee::s03::f_0019a3d0__generated,
            cfe_state_char_equip_toggle_buy_equip,
            120,
            &|c, seed| {
                let _r = base_7(c, seed, 0x19a3);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0xc8, (seed & 1) as u32);
                if seed % 4 == 0 {
                    c.bus.write32(DATA + 0x50, 0);
                } else {
                    c.bus.write32(DATA + 0x50, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2008, DATA + 0x2100);
                    c.bus.write16(DATA + 0x2100 + 0x48, 0);
                    c.bus.write32(DATA + 0x2100 + 0x4c, FAKE);
                }
                for off in [0x48u32, 0x60, 0x80, 0x88, 0x6c] {
                    c.bus.write32(DATA + off, 0);
                }
            },
        );
    }

    #[test]
    fn boltfile_cache_init_matches() {
        check_lohi(
            ee::s03::f_0019ce68__generated,
            boltfile_cache_init,
            8,
            &|c, seed| {
                let _ = base_8(c, seed, 0x19ce68, 2);
                // A large private region for the 10x256 entry table.
                let base_at = 0x0100_0000u32;
                c.g[A0][0] = base_at as u64;
                w(
                    c,
                    GAME_GP.wrapping_sub(0x1788),
                    if seed % 2 == 0 { DATA + 0x2_0000 } else { 0 },
                );
                clear(c, DATA + 0x2_0000, 0x3000);
            },
        );
    }
}
