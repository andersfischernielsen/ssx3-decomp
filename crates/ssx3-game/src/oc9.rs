//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Trivial leaf replacements. Each copies the generated body's logic, cycle
//! charging and observable register (and FPU condition) results exactly, with
//! a differential test as its gate.

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0019_e238, bolt_file_is_loaded, "BoltFile_IsLoaded"),
    (0x0019_e2b0, bolt_file_get_data, "BoltFile_GetData"),
    (0x0019_e330, bolt_file_detach_data, "BoltFile_DetachData"),
    (0x0019_e538, femodule_rider_preview_set_visible, "FEModule_RiderPreviewSetVisible"),
    (0x001a_0538, femodule_get_rider_preview_2, "FEModule_GetRiderPreview_2"),
    (0x001a_0548, femodule_get_rider_preview, "FEModule_GetRiderPreview"),
    (0x001a_06f0, feflowstate_get_menu_selection, "FEFlowState_GetMenuSelection"),
    (0x001a_0708, feflowstate_set_menu_selection, "FEFlowState_SetMenuSelection"),
    (0x001a_4740, cfe_state_net_messenger_get_list_index, "cFEStateNetMessenger_GetListIndex"),
    (0x001b_9558, cfe_state_create_step1_all_fields_filled, "cFEStateCreateStep1_AllFieldsFilled"),
    (0x001b_9f68, cfe_state_create_step1_char_in_range, "cFEStateCreateStep1_CharInRange"),
    (0x001b_b968, cfe_state_create_step2_is_leap_year, "cFEStateCreateStep2_IsLeapYear"),
    (0x001c_5780, cfepopupconfirm_cfg_set_option_string, "cFEPopupConfirm_CfgSetOptionString"),
    (0x001c_57a0, cfepopupconfirm_cfg_set_option_wide_string, "cFEPopupConfirm_CfgSetOptionWideString"),
    (0x001c_57c0, cfepopupconfirm_cfg_set_option_text_id, "cFEPopupConfirm_CfgSetOptionTextId"),
    (0x001c_5800, cfepopupconfirm_cfg_get_option, "cFEPopupConfirm_CfgGetOption"),
    (0x001c_5838, cfepopupconfirm_cfg_set_option_confirm_wide, "cFEPopupConfirm_CfgSetOptionConfirmWide"),
    (0x001c_5860, cfepopupconfirm_cfg_clear_option_confirm, "cFEPopupConfirm_CfgClearOptionConfirm"),
    (0x001c_5878, cfepopupconfirm_cfg_set_option_confirm_text_id, "cFEPopupConfirm_CfgSetOptionConfirmTextId"),
    (0x001c_58d0, cfepopupconfirm_cfg_get_option_confirm, "cFEPopupConfirm_CfgGetOptionConfirm"),
];

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

/// `void* BoltFile_GetData(BoltFile *file, int outer, int inner)` (0x19e2b0):
/// 0 unless the state word at `file + outer*0x12000 + inner*0x120` is 1, else
/// the data pointer at `+0x104`.
pub fn bolt_file_get_data(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x10000;
    c.g[V1][0] = 0x120;
    c.g[V0][0] = c.g[V0][0] | 0x2000;
    c.mult(0, A2, A2, V1);
    c.mult(0, A1, A1, V0);
    c.g[A3][0] = c.g[A0][0];
    c.g[A0][0] = 0x1;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    let not_loaded = c.g[V1][0] != c.g[A0][0];
    c.g[V1][0] = c.g[V0][0];
    if not_loaded {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x104).lw(c);
    }
    abi::ret(c);
}

/// `void BoltFile_DetachData(BoltFile *file, int outer, int inner)`
/// (0x19e330): clears the state word and the data pointer of a loaded record,
/// and decrements the global live-data counter at `file + 0xb4000`.
pub fn bolt_file_detach_data(c: &mut Cpu) {
    c.cycles += 20;
    c.g[V0][0] = 0x10000;
    c.g[V1][0] = 0x120;
    c.g[V0][0] = c.g[V0][0] | 0x2000;
    c.mult(0, A2, A2, V1);
    c.mult(0, A1, A1, V0);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x104));
    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x118));
    c.g[V0][0] = 0xb0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    Addr(c.g[V1][0] as u32).sw(c, 0);
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x4000).lw(c);
    c.g[A1][0] = Addr(c.g[A3][0] as u32).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x4000).sw(c, c.g[V1][0]);
    Addr(c.g[A3][0] as u32).sw(c, 0);
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

/// `RiderPreview* FEModule_GetRiderPreview_2(FEModule *self, int index)`
/// (0x1a0538): `self + index*0xce0`, the preview record array.
pub fn femodule_get_rider_preview_2(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0xce0;
    c.mult(0, A2, A1, V1);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    abi::ret(c);
}

/// `RiderPreview* FEModule_GetRiderPreview(FEModule *self, int index)`
/// (0x1a0548): identical to [`femodule_get_rider_preview_2`].
pub fn femodule_get_rider_preview(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0xce0;
    c.mult(0, A2, A1, V1);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end flow state.

/// `int FEFlowState_GetMenuSelection(FEFlowState *self, int index)`
/// (0x1a06f0): signed byte from the selection array at `*(u32*)(self+4)`.
pub fn feflowstate_get_menu_selection(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
    abi::ret(c);
}

/// `void FEFlowState_SetMenuSelection(FEFlowState *self, int index, int value)`
/// (0x1a0708): stores the low byte of `value` into the selection array.
pub fn feflowstate_set_menu_selection(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[V0][0] as u32).sb(c, c.g[A2][0]);
    abi::ret(c);
}

/// `int cFEStateNetMessenger_GetListIndex(cFEStateNetMessenger *self, int i)`
/// (0x1a4740): `*(u32*)(self+0x6f4) + i`.
pub fn cfe_state_net_messenger_get_list_index(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x6f4).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end "create account" flow.

/// `bool cFEStateCreateStep1_AllFieldsFilled(cFEStateCreateStep1 *self)`
/// (0x1b9558): true when the four pointers at `*(u32*)(self+0x6d0) + 0/4/8/0xc`
/// all point at a block whose word at `-8` is non-zero.
pub fn cfe_state_create_step1_all_fields_filled(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x6d0).lw(c);
    c.g[A2][0] = 0x1;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xffff_fff8).lw(c); // -8
    let mut a1 = 0u64;
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xffff_fff8).lw(c);
        if c.g[V1][0] != 0 {
            a1 = c.g[A2][0];
        }
    }
    c.cycles += 3;
    c.g[A1][0] = a1;
    c.g[V1][0] = c.g[A1][0];
    let t = c.g[V1][0] == 0;
    c.g[V0][0] = 0;
    if !t {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x8).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xffff_fff8).lw(c);
        c.g[V0][0] = 0;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A2][0];
        }
    }
    c.cycles += 3;
    c.g[V1][0] = c.g[V0][0];
    let t = c.g[V1][0] == 0;
    c.g[V0][0] = 0;
    if !t {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0xc).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xffff_fff8).lw(c);
        c.g[V0][0] = 0;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A2][0];
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool cFEStateCreateStep1_CharInRange(void *self, int lo, int ch, int hi)`
/// (0x1b9f68): treats `lo`, `ch` and `hi` as signed bytes and returns whether
/// `lo <= ch <= hi`.
pub fn cfe_state_create_step1_char_in_range(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 24);
    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 24) as u32);
    c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
    c.g[V1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
    if c.g[A3][0] != 0 {
        c.cycles += 1;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 3;
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
        c.g[V0][0] = 0x1;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool cFEStateCreateStep2_IsLeapYear(void *self, int year)` (0x1bb968):
/// true for years divisible by 4 that are not divisible by 100 (unless also by
/// 400), computed with the original's `div`/`mfhi` sequence.
pub fn cfe_state_create_step2_is_leap_year(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A1][0] & 0x3;
    let mut a0 = 0u64;
    if c.g[V0][0] == 0 {
        c.cycles += 5;
        c.g[V0][0] = 0x64;
        a0 = 1;
        c.div(0, A1, V0);
        c.cycles += 3;
        c.g[V1][0] = c.hi[0];
        c.g[V0][0] = 0x190;
        if c.g[V1][0] == 0 {
            c.cycles += 3;
            c.div(0, A1, V0);
            c.cycles += 2;
            c.g[V1][0] = c.hi[0];
            a0 = (c.g[V1][0] < 0x1u64) as u64;
        }
    }
    c.cycles += 2;
    c.g[A0][0] = a0;
    c.g[V0][0] = a0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end popup confirm widget (option rows).

/// `void cFEPopupConfirm_CfgSetOptionString(Cfg* cfg, int row, void *s)`
/// (0x1c5780): stores the string `s` in an option row of kind 2.
pub fn cfepopupconfirm_cfg_set_option_string(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
    c.g[V0][0] = 0x2;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = c.g[V1][0];
    Addr(c.g[V1][0] as u32).off(0x28).sw(c, c.g[A2][0]);
    Addr(c.g[A0][0] as u32).off(0x24).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void cFEPopupConfirm_CfgSetOptionWideString(Cfg* cfg, int row, void *s)`
/// (0x1c57a0): same as [`cfepopupconfirm_cfg_set_option_string`] with kind 1.
pub fn cfepopupconfirm_cfg_set_option_wide_string(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
    c.g[V0][0] = 0x1;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = c.g[V1][0];
    Addr(c.g[V1][0] as u32).off(0x28).sw(c, c.g[A2][0]);
    Addr(c.g[A0][0] as u32).off(0x24).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void cFEPopupConfirm_CfgSetOptionTextId(Cfg* cfg, int row, int id)`
/// (0x1c57c0): same as [`cfepopupconfirm_cfg_set_option_string`] with kind 3.
pub fn cfepopupconfirm_cfg_set_option_text_id(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
    c.g[V0][0] = 0x3;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = c.g[V1][0];
    Addr(c.g[V1][0] as u32).off(0x28).sw(c, c.g[A2][0]);
    Addr(c.g[A0][0] as u32).off(0x24).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void* cFEPopupConfirm_CfgGetOption(Cfg* cfg, int row)` (0x1c5800):
/// `cfg + row*8 + 0x24`.
pub fn cfepopupconfirm_cfg_get_option(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x24));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `void cFEPopupConfirm_CfgSetOptionConfirmWide(Cfg* cfg, int row, void *s, int arg)`
/// (0x1c5838): sets up a kind-1 option row (`row*0xc` stride).
pub fn cfepopupconfirm_cfg_set_option_confirm_wide(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0xc;
    c.g[T0][0] = 0x1;
    c.mult(0, A1, A1, V0); // row *= 0xc
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[V0][0] as u32).off(0x58).sw(c, c.g[A2][0]);
    c.g[V1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[V0][0];
    Addr(c.g[V1][0] as u32).off(0x54).sw(c, c.g[T0][0]);
    Addr(c.g[A0][0] as u32).off(0x5c).sw(c, c.g[A3][0]);
    abi::ret(c);
}

/// `void cFEPopupConfirm_CfgClearOptionConfirm(Cfg* cfg, int row)`
/// (0x1c5860): zeroes the option-confirm kind field of `row*0xc`.
pub fn cfepopupconfirm_cfg_clear_option_confirm(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0xc;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x54).sw(c, 0);
    abi::ret(c);
}

/// `void cFEPopupConfirm_CfgSetOptionConfirmTextId(Cfg* cfg, int row, int id, int arg)`
/// (0x1c5878): like [`cfepopupconfirm_cfg_set_option_confirm_wide`] with
/// kind 3.
pub fn cfepopupconfirm_cfg_set_option_confirm_text_id(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0xc;
    c.g[T0][0] = 0x3;
    c.mult(0, A1, A1, V0);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[V0][0] as u32).off(0x58).sw(c, c.g[A2][0]);
    c.g[V1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[V0][0];
    Addr(c.g[V1][0] as u32).off(0x54).sw(c, c.g[T0][0]);
    Addr(c.g[A0][0] as u32).off(0x5c).sw(c, c.g[A3][0]);
    abi::ret(c);
}

/// `void* cFEPopupConfirm_CfgGetOptionConfirm(Cfg* cfg, int row)`
/// (0x1c58d0): `cfg + row*0xc + 0x54`.
pub fn cfepopupconfirm_cfg_get_option_confirm(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0xc;
    c.mult(0, V0, A1, V1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x54));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA};

    #[test]
    fn bolt_file_is_loaded_matches() {
        check_lohi(ssx3_ee::s03::f_0019e238, bolt_file_is_loaded, 200, &|c, seed| {
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
            c.bus.write32(base, if seed % 3 == 0 { 1 } else { r.next_u32() });
        });
    }

    #[test]
    fn bolt_file_get_data_matches() {
        check_lohi(ssx3_ee::s03::f_0019e2b0, bolt_file_get_data, 200, &|c, seed| {
            let mut r = Rng(0x1111_0002 ^ seed as u64);
            scramble(c, &mut r);
            let outer = (seed % 2) as u32;
            let inner = ((seed / 2) % 4) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = outer as u64;
            c.g[A2][0] = inner as u64;
            let base = DATA
                .wrapping_add(outer.wrapping_mul(0x12000))
                .wrapping_add(inner.wrapping_mul(0x120));
            c.bus.write32(base, if seed % 3 == 0 { 1 } else { r.next_u32() });
            c.bus.write32(base + 0x104, r.next_u32());
        });
    }

    #[test]
    fn bolt_file_detach_data_matches() {
        check_lohi(ssx3_ee::s03::f_0019e330, bolt_file_detach_data, 200, &|c, seed| {
            let mut r = Rng(0x1111_0003 ^ seed as u64);
            scramble(c, &mut r);
            let outer = (seed % 2) as u32;
            let inner = ((seed / 2) % 2) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = outer as u64;
            c.g[A2][0] = inner as u64;
            let d = outer.wrapping_mul(0x12000).wrapping_add(inner.wrapping_mul(0x120));
            let v1 = DATA.wrapping_add(0x104).wrapping_add(d);
            let a3 = DATA.wrapping_add(0x118).wrapping_add(d);
            let counter = DATA.wrapping_add(0xb4000);
            c.bus.write32(v1, r.next_u32());
            c.bus.write32(a3, r.next_u32());
            c.bus.write32(counter, r.next_u32());
        });
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
    fn femodule_get_rider_preview_2_matches() {
        check_lohi(ssx3_ee::s03::f_001a0538, femodule_get_rider_preview_2, 200, &|c, seed| {
            let mut r = Rng(0x1111_0005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 8) as u64;
        });
    }

    #[test]
    fn femodule_get_rider_preview_matches() {
        check_lohi(ssx3_ee::s03::f_001a0548, femodule_get_rider_preview, 200, &|c, seed| {
            let mut r = Rng(0x1111_0006 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 8) as u64;
        });
    }

    #[test]
    fn feflowstate_get_menu_selection_matches() {
        check(
            ssx3_ee::s03::f_001a06f0,
            feflowstate_get_menu_selection,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0007 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = seed as u64;
                c.bus.write32(DATA + 0x4, (DATA + 0x100) as u32);
                for i in 0..0x40 {
                    c.bus.write8(DATA + 0x80 + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn feflowstate_set_menu_selection_matches() {
        check(
            ssx3_ee::s03::f_001a0708,
            feflowstate_set_menu_selection,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0008 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(DATA + 0x4, (DATA + 0x100) as u32);
            },
        );
    }

    #[test]
    fn cfe_state_net_messenger_get_list_index_matches() {
        check(
            ssx3_ee::s03::f_001a4740,
            cfe_state_net_messenger_get_list_index,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0009 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(DATA + 0x6f4, r.next_u32());
            },
        );
    }

    #[test]
    fn cfe_state_create_step1_all_fields_filled_matches() {
        check(
            ssx3_ee::s03::f_001b9558,
            cfe_state_create_step1_all_fields_filled,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_000a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let s = DATA + 0x100;
                c.bus.write32(DATA + 0x6d0, s);
                for k in 0..4u32 {
                    let q = DATA + 0x200 + k * 0x20;
                    c.bus.write32(s + k * 4, q);
                    let v = if (seed >> k) & 1 == 0 { 0 } else { r.next_u32() | 1 };
                    c.bus.write32(q - 8, v);
                }
            },
        );
    }

    #[test]
    fn cfe_state_create_step1_char_in_range_matches() {
        check(
            ssx3_ee::s03::f_001b9f68,
            cfe_state_create_step1_char_in_range,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_000b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn cfe_state_create_step2_is_leap_year_matches() {
        check_lohi(
            ssx3_ee::s03::f_001bb968,
            cfe_state_create_step2_is_leap_year,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_000c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn cfepopupconfirm_cfg_set_option_string_matches() {
        check(
            ssx3_ee::s03::f_001c5780,
            cfepopupconfirm_cfg_set_option_string,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_000d ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn cfepopupconfirm_cfg_set_option_wide_string_matches() {
        check(
            ssx3_ee::s03::f_001c57a0,
            cfepopupconfirm_cfg_set_option_wide_string,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_000e ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn cfepopupconfirm_cfg_set_option_text_id_matches() {
        check(
            ssx3_ee::s03::f_001c57c0,
            cfepopupconfirm_cfg_set_option_text_id,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_000f ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn cfepopupconfirm_cfg_get_option_matches() {
        check(
            ssx3_ee::s03::f_001c5800,
            cfepopupconfirm_cfg_get_option,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_0010 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn cfepopupconfirm_cfg_set_option_confirm_wide_matches() {
        check_lohi(
            ssx3_ee::s03::f_001c5838,
            cfepopupconfirm_cfg_set_option_confirm_wide,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_0011 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn cfepopupconfirm_cfg_clear_option_confirm_matches() {
        check_lohi(
            ssx3_ee::s03::f_001c5860,
            cfepopupconfirm_cfg_clear_option_confirm,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_0012 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn cfepopupconfirm_cfg_set_option_confirm_text_id_matches() {
        check_lohi(
            ssx3_ee::s03::f_001c5878,
            cfepopupconfirm_cfg_set_option_confirm_text_id,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_0013 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn cfepopupconfirm_cfg_get_option_confirm_matches() {
        check_lohi(
            ssx3_ee::s03::f_001c58d0,
            cfepopupconfirm_cfg_get_option_confirm,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_0014 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }
}
