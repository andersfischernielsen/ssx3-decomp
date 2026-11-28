//! `src/ui/uiengine.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::script::scriptcontroller::vslot;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::dirtysock::tagsunk::vslot as vslot_2;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::fe::feasyncfile::gp_18 as gp;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::fe::feasyncfile::sb_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fepopupmisc::gp_6 as gp_2;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::object::instancenode::ui_vector_lw;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::ui::uitext::oc445_add;
pub(crate) use crate::game::ui::uitext::oc445_addu;
pub(crate) use crate::game::ui::uitext::oc445_lw;
pub(crate) use crate::game::ui::uitext::oc445_vcall;
pub(crate) use crate::game::visualfx::boardwakefx::at759;
pub(crate) use crate::game::wscript::wscriptman::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_79f8, ui_list_accept_visitor, "UIList_AcceptVisitor"),
    (0x0039_7b08, ui_engine_ctor, "UIEngine_ctor"),
    (0x0039_7b70, ui_engine_dtor, "UIEngine_dtor"),
    (0x0039_7c40, ui_engine_load_package, "UIEngine_LoadPackage"),
    (
        0x0039_8018,
        ui_engine_render_state_mgr,
        "UIEngine_RenderStateMgr",
    ),
    (
        0x0039_8078,
        ui_engine_texture_set_dtor,
        "UIEngine_TextureSet_dtor",
    ),
    (0x0039_8380, ui_texture_set_find, "UITextureSet_Find"),
    (0x0039_83f0, ui_font_set_find_font, "UIFontSet_FindFont"),
    (0x0039_8438, ui_font_set_free_fonts, "UIFontSet_FreeFonts"),
    (0x0039_84b0, uifont_set_add_font, "UIFontSet_AddFont"),
    (0x0039_8638, cui_group_set_focus, "cUIGroup_SetFocus"),
    (0x0039_86b0, c_ui_group_find_by_id, "cUIGroup_FindById"),
    (0x0039_8738, ui_group_refresh_text, "cUIGroup_RefreshText"),
    (0x0039_8798, ui_group_handle_input, "cUIGroup_HandleInput"),
    (0x0039_87f8, c_ui_group_v18, "cUIGroup_v18"),
    (0x0039_8910, c_ui_group_update_anim, "cUIGroup_UpdateAnim"),
    (0x0039_8998, ui_icon_load, "UIIcon_Load"),
    (0x0039_9730, ui_icon_set_texture, "UIIcon_SetTexture"),
    (0x0039_9820, uilistbox_load, "UIListBox_Load"),
    (
        0x0039_9920,
        ui_list_box_get_mode_entry,
        "UIListBox_GetModeEntry",
    ),
    (
        0x0039_9d80,
        ui_list_box_set_item_wide_string,
        "UIListBox_SetItemWideString",
    ),
    (0x0039_9e28, uilistbox_refresh_text, "UIListBox_RefreshText"),
    (0x0039_8868, c_ui_group_render, "cUIGroup_Render"),
    (0x0039_8a78, ui_icon_render, "UIIcon_Render"),
    (0x0039_9f00, ui_list_box_render, "UIListBox_Render"),
    (
        0x0039_7cc0,
        ui_engine_create_screen,
        "UIEngine_CreateScreen",
    ),
    (0x0039_8618, ui_group_load, "cUIGroup_Load"),
    (0x0039_9768, uilistbox_ctor_w244, "UIListBox_ctor"),
    (0x0039_7ad8, ui_ascii_to_wide, "UI_AsciiToWide"),
    (
        0x0039_8568,
        ui_font_set_load_fonts_811,
        "UIFontSet_LoadFonts",
    ),
    (
        0x0039_8138,
        ui_engine_load_textures,
        "UIEngine_LoadTextures",
    ),
    (
        0x0039_84e8,
        ui_font_set_parse_font_pack,
        "UIFontSet_ParseFontPack",
    ),
    (0x0039_8a60, ui_icon_swap_words, "UIIcon_SwapWords"),
    (
        0x0039_9970,
        ui_list_box_handle_input,
        "UIListBox_HandleInput",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::camera::camera::callee::UI_TEXTURE_SET_FIND;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST as UI_LIST_FIRST_4;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST_3;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2 as UI_OBJECT_UPDATE_ANIM;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXTURE_SET_FIND as UI_TEXTURE_SET_FIND_2;

    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;

    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;

    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;

    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2;

    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_STATE_MGR_RENDER_STUB;

    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {}

// ---------------------------------------------------------------------------
// UI font set

/// `void UIFontSet_AddFont(UIFontSet *set, int index, void *a, void *b)`
/// (0x3984b0): while fewer than 4 fonts are registered, store the two words at
/// `set + index*0xc + 4` and bump the count at +0x34.
pub fn uifont_set_add_font(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x34).lb(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
    let full = c.g[V0][0] == 0;
    c.g[V0][0] = 0xc;
    if !full {
        c.cycles += 8;
        c.mult(0, 8, 5, 2);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(this));
        Addr(c.g[V1][0] as u32).off(0x8).sw(c, c.g[A2][0]);
        c.g[V0][0] = c.g[V1][0];
        Addr(c.g[V0][0] as u32).off(0x4).sw(c, c.g[A3][0]);
        c.g[V1][0] = Addr(this).off(0x34).lbu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        Addr(this).off(0x34).sb(c, c.g[V1][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void UIEngine_RenderStateMgr(this)` (0x00398018): render the state
/// manager at `+0x18`.
pub fn ui_engine_render_state_mgr(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18)),
        callee::UI_STATE_MGR_RENDER_STUB,
        0x0039_8028,
    )
}

// ---------------------------------------------------------------------------
// UITextureSet_Find / UIIcon_SetTexture.

/// `Texture* UITextureSet_Find(UITextureSet* set, int id)` (0x398380): linear
/// search of the `count` (`+0x10`) 0x20-byte entries at `*(set + 0xc)` for the
/// one whose id (`+0x1c`) matches; null when there is none.
pub fn ui_texture_set_find(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = c.g[A0][0];
    c.g[A2][0] = Addr((self_ as u32).wrapping_add(0x10)).lw(c);
    let count = c.g[A2][0];
    c.g[V1][0] = 0;
    if (count as i64) > 0 {
        c.cycles += 1;
        c.g[A0][0] = Addr((self_ as u32).wrapping_add(0xc)).lw(c);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1c)).lw(c);
            let mismatch = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if mismatch {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_8390);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void UIIcon_SetTexture(this, int id)` (0x399730): look the texture up in
/// the icon's texture set (via `+0x5c` -> `+0xd0` -> `+0x10`) and store it at
/// `+0x7c`.
pub fn ui_icon_set_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = Addr((this as u32).wrapping_add(0x5c)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xd0).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58));
        callee::UI_TEXTURE_SET_FIND.call(c, 0x0039_9754)?;
        let found = c.g[V0][0];
        c.sw((this as u32).wrapping_add(0x7c), found);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI objects.

/// `int UIFontSet_FindFont(bank, key)` (0x3983f0): linear search of the
/// `u8`-counted array of 0xc-byte font entries at `bank+4`; returns the
/// entry's `+8` value, or 0.
pub fn ui_font_set_find_font(c: &mut Cpu) {
    c.cycles += 3;
    let bank = c.g[A0][0] as u32;
    c.g[V1][0] = Addr(bank).off(0x34).lb(c);
    c.g[A2][0] = c.g[V1][0];
    if (c.g[V1][0] as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        crate::abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[A0][0] = sx(bank.wrapping_add(4));
    c.g[V1][0] = 0x0100_0000;
    c.g[A3][0] = 0x0100_0000;
    loop {
        c.cycles += 3;
        let entry = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(entry).lw(c);
        let found = c.g[V0][0] == c.g[A1][0];
        c.g[V0][0] = c.g[V1][0];
        if !found {
            c.cycles += 5;
            c.g[A0][0] = sx(entry.wrapping_add(0xc));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            if c.g[V0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0039_8408);
                }
                continue;
            }
            break;
        }
        c.cycles += 2;
        c.g[V0][0] = Addr(entry.wrapping_add(8)).lw(c);
        crate::abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    crate::abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI and streaming.

/// `int UIListBox_GetModeEntry(UIListBox* self)` (0x399920): read the entry
/// table of the list box's state data for the current mode (2-bit index).
pub fn ui_list_box_get_mode_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_GET_STATE_DATA.call(c, 0x0039_9934)?;
        c.g[A0][0] = c.g[V0][0];
        let mut entry = 0u64;
        if c.g[A0][0] != 0 {
            c.g[V1][0] = Addr(p(this, 0x74)).lbu(c);
            c.cycles += 6;
            c.g[V0][0] = 0xc;
            c.g[A0][0] = Addr(r32(c, A0)).off(8).lw(c);
            c.g[V1][0] &= 0x3;
            c.mult(0, V1, V1, V0);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            entry = Addr(r32(c, V1)).off(0xc).lw(c);
        } else {
            c.cycles += 2;
        }
        c.g[V0][0] = entry;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIGroup_RefreshText(cUIGroup* self)` (0x398738): call every element
/// of the group's UI list to re-read its text.
pub fn ui_group_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
        call_ptr(c, 0x0039_7840, 0x0039_874c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = lw(c, p_2(c.g[S0][0], 8));
            loop {
                c.cycles += 9;
                c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x80)).lh(c);
                c.g[V1][0] = lw(c, p_2(c.g[V0][0], 0x84));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0039_8770)?;
                c.g[S0][0] = lw(c, p_2(c.g[S0][0], 4));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039_7718, 0x0039_877c)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                    break;
                }
                c.g[V0][0] = lw(c, p_2(c.g[S0][0], 8));
                if event_due(c) {
                    deliver_event_here(c, 0x0039_8760);
                }
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int cUIGroup_HandleInput(cUIGroup* self, void* arg)` (0x398798): unless the
/// group is a passive display, offer the input to every list node through a
/// small functor; true when a node claimed it.
pub fn ui_group_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = lw(c, p_2(c.g[A0][0], 0x14));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
        c.g[V0][0] &= 1;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 11;
            c.g[V0][0] = 0x0049_0000;
            Addr(p_2(c.g[SP][0], 4)).sw(c, c.g[A1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4bb8));
            c.g[S0][0] = 0x0049_0000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4ce8));
            Addr(p_2(c.g[SP][0], 0)).sw(c, c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x0039_79f8, 0x0039_87dc)?;
            Addr(p_2(c.g[SP][0], 0)).sw(c, c.g[S0][0]);
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Constructors.

/// `UIEngine* UIEngine_ctor(UIEngine* self)` (0x397b08): clear the engine
/// header, construct both UI lists and clear the tail pointers.
pub fn ui_engine_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.sw(c.g[S0][0] as u32, 0u64);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
        c.g[RA][0] = sx(0x0039_7b38);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
        call_ptr(c, 0x0039_77e8, 0x0039_7b38)?;
        c.g[RA][0] = sx(0x0039_7b40);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
        call_ptr(c, 0x0039_77e8, 0x0039_7b40)?;
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x60), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x64), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x68), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[S0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), c.g[S0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x20);
        Ok(())
    })
}

/// `void cUIGroup_Render(cUIGroup* self, int a1)` (0x3987f8): call every
/// child node's render method. The original's loop head is a resume entry
/// (an event at the back edge returns at 0x398828).
pub fn c_ui_group_v18(c: &mut Cpu) {
    let frame = Frame { size: 0x30 };
    guest(c, |c| {
        match c.pc {
            0x0039_87f8 => {
                c.cycles += 10;
                c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
                c.g[A0][0] = sx(at(c, A0, 0x74));
                c.sq(17, frame.at(c, 0x10));
                c.sq(16, frame.at(c, 0x20));
                frame.save_ra(c, 0);
                c.g[S1][0] = c.g[A1][0];
                call_ptr(c, 0x0039_7840, 0x0039_8814)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    frame.restore(c, S0, 0x20);
                    c.cycles += 4;
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 2;
            }
            0x0039_8820 => c.cycles += 2,
            0x0039_8828 => ui_group_dispatch(c)?,
            _ => {
                c.jump_out(c.pc, 0x0039_87f8, 0x0039_8864);
                return Ok(());
            }
        }
        loop {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039_7718, 0x0039_8848)?;
            if c.g[V0][0] != 0 {
                break;
            }
            c.g[V0][0] = Addr(at(c, S0, 8)).lw(c);
            if event_due(c) && back_edge_unwinds(c, 0x0039_8828) {
                return Ok(());
            }
            ui_group_dispatch(c)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The render dispatch of one [`c_ui_group_v18`] node (0x398828).
pub(crate) fn ui_group_dispatch(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = Addr(at(c, V0, 0x90)).lh(c);
    c.g[V1][0] = Addr(at(c, V0, 0x94)).lw(c);
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(at(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x0039_883c)?;
    c.g[S0][0] = Addr(at(c, S0, 4)).lw(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Lists and screens.

/// `UIList_AcceptVisitor` (0x3979F8): walk the list at `self->head` (self+4)
/// backwards calling each node's visitor (`node+0x10` delta, `node+0x14`
/// function, `this = node + delta`) with `arg`; return the first non-zero
/// result, else 0.
pub fn ui_list_accept_visitor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(r32(c, A0).wrapping_add(0x4)).lw(c);
        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
        let at_head = c.g[V0][0] == c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        let mut visited = false;
        if !at_head {
            c.cycles += 2;
            c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x8)).lw(c);
            loop {
                c.cycles += 8;
                c.g[A1][0] = c.g[S1][0];
                c.g[S0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lh(c);
                let visitor = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
                c.g[V0][0] = visitor;
                c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
                call_ptr(c, visitor as u32, 0x0039_7a38)?;
                if c.g[V0][0] != 0 {
                    visited = true;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
                if c.g[V0][0] == c.g[A2][0] {
                    break;
                }
                c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0039_7a20);
                }
            }
        }
        if !visited {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `lb off(reg)` (sign-extended).
#[inline]
pub(crate) fn lb_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lb(c)
}

// ---------------------------------------------------------------------------
// Loops and worker threads.

/// `func_00398438` (0x398438): free every font in a font set. Walks the
/// `count` (int8 at `+0x34`) entries of the 0xc-byte table at `+0x4`; each
/// font (`+8`) is destroyed (`vtable[8]`, flag 3) and its slots cleared, then
/// the count byte is zeroed. Signature: `void free_fonts(UIFontSet* this)`.
pub fn ui_font_set_free_fonts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lb_r(c, S2, 0x34);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = sx(low(c, S2).wrapping_add(0x4));
            loop {
                c.cycles += 3;
                c.g[A0][0] = lw_r(c, S0, 0x8);
                if c.g[A0][0] == 0 {
                    sw_r(c, S0, 0x4, 0);
                } else {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0039_8474);
                    c.g[A1][0] = 0x3;
                    call_ptr(c, 0x0039_16c0, 0x0039_8474)?;
                    sw_r(c, S0, 0x4, 0);
                }
                c.cycles += 6;
                c.g[S1][0] = sx(low(c, S1).wrapping_add(1));
                sw_r(c, S0, 0x0, 0);
                c.g[V0][0] = lb_r(c, S2, 0x34);
                let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[S0][0] = sx(low(c, S0).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_8460);
                }
            }
        }
        c.cycles += 7;
        sb_r(c, S2, 0x34, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIGroup_SetFocus(self, on)` (0x398638): run the accept-visitor over
/// the group's list with a small functor, then set the group's focus flag.
pub fn cui_group_set_focus(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x50));
        c.g[V0][0] = 0x49_0000;
        c.sq(16, sp_2(c).wrapping_add(0x40));
        c.g[V0][0] = sx(low(c, V0).wrapping_add(0x4b98));
        c.sq(17, sp_2(c).wrapping_add(0x30));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, sp_2(c).wrapping_add(0x20));
        c.g[S1][0] = 0x49_0000;
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp_2(c).wrapping_add(0x10), c.g[RA][0]);
        c.g[S1][0] = sx(low(c, S1).wrapping_add(0x4ce8));
        sw_r(c, SP, 0x4, c.g[S0][0]);
        c.g[A0][0] = sx(low(c, S2).wrapping_add(0x74));
        sw_r(c, SP, 0, c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        callee::UI_LIST_ACCEPT_VISITOR.call(c, 0x0039_8678)?;
        c.g[V1][0] = lw_r(c, S2, 0x14);
        c.g[V0][0] = 0xffff_ffff_ffff_ffef;
        c.g[S0][0] = (0u64 < c.g[S0][0]) as u64;
        sw_r(c, SP, 0, c.g[S1][0]);
        c.g[S0][0] = sx(low(c, S0) << 4);
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[V1][0] = c.g[V1][0] | c.g[S0][0];
        c.lq(17, sp_2(c).wrapping_add(0x30));
        sw_r(c, S2, 0x14, c.g[V1][0]);
        c.lq(16, sp_2(c).wrapping_add(0x40));
        c.lq(18, sp_2(c).wrapping_add(0x20));
        c.g[RA][0] = Addr(sp_2(c).wrapping_add(0x10)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `int UIEngine_LoadPackage(engine, name)` (0x00397c40): load a UI package
/// file, store its handle and load its textures.
pub fn ui_engine_load_package(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        frame.save(c, S0, 0x90);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = sx(0x4a0000);
        frame.save_ra(c, 0x70);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = sx(0x4a0000u32.wrapping_add(0x4770));
        callee::SPRINTF_2.call(c, 0x0039_7c6c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0;
        callee::FILE_LOAD.call(c, 0x0039_7c78)?;
        Addr(at_2(c, S0, 0x4)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(word(c, S0).wrapping_add(0x50));
        c.g[A1][0] = Addr(at_2(c, V0, 0x8)).lw(c);
        c.g[A1][0] = sx(word(c, V0).wrapping_add(word(c, A1)));
        callee::UI_ENGINE_STORE_WORD.call(c, 0x0039_7c8c)?;
        c.g[V0][0] = Addr(at_2(c, S0, 0x4)).lw(c);
        c.g[A0][0] = sx(word(c, S0).wrapping_add(0x58));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = Addr(at_2(c, V0, 0x10)).lw(c);
        c.g[A2][0] = sx(word(c, V0).wrapping_add(word(c, A2)));
        callee::UI_ENGINE_LOAD_TEXTURES.call(c, 0x0039_7ca4)?;
        c.lq(16, frame.at(c, 0x90));
        c.g[V0][0] = 0x1;
        c.lq(17, frame.at(c, 0x80));
        c.g[RA][0] = c.ld(frame.at(c, 0x70));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI groups.

pub(crate) const UI_GROUP_UPDATE_ENTRY: u32 = 0x0039_8910;

pub(crate) const UI_GROUP_UPDATE_END: u32 = 0x0039_8994;

/// Resumed after an event in the node walk (the animation applies loop).
pub(crate) const UI_GROUP_UPDATE_NODE: u32 = 0x0039_8948;

/// `void cUIGroup_UpdateAnim(cUIGroup* self, u16 ev)` (0x398910): apply the
/// event to every live node of the group's list, then update the group's own
/// animation.
pub fn c_ui_group_update_anim(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                UI_GROUP_UPDATE_ENTRY => {
                    c.cycles += 12;
                    let frame = Frame::open(c, 0x40);
                    frame.save(c, S2, 0x10);
                    frame.save(c, S1, 0x20);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S0, 0x30);
                    c.g[S1][0] = c.g[A1][0] & 0xffff;
                    frame.save_ra(c, 0);
                    c.g[RA][0] = sx(0x0039_8934);
                    c.g[A0][0] = sx(p(r32(c, S2), 0x74));
                    callee::UI_LIST_FIRST_3.call(c, 0x0039_8934)?;
                    c.g[S0][0] = c.g[V0][0];
                    let empty = c.g[S0][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    if empty {
                        pc = 0x0039_8974;
                        continue;
                    }
                    pc = 0x0039_8940;
                }
                0x0039_8940 => {
                    // The generated code's always-taken branch to the check.
                    c.cycles += 2;
                    pc = 0x0039_8960;
                }
                UI_GROUP_UPDATE_NODE => {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = Addr(p(r32(c, V0), 0x98)).lh(c);
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x9c)).lw(c);
                    let target = r32(c, V1);
                    c.g[RA][0] = sx(0x0039_895c);
                    c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                    call_ptr(c, target, 0x0039_895c)?;
                    c.g[S0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                    pc = 0x0039_8960;
                }
                0x0039_8960 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0039_8968);
                    c.g[A0][0] = c.g[S0][0];
                    callee::UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_8968)?;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                        if event_due(c) {
                            c.pc = UI_GROUP_UPDATE_NODE;
                            if c.event() {
                                return Err(crate::call::Unwound);
                            }
                        }
                        pc = UI_GROUP_UPDATE_NODE;
                    } else {
                        pc = 0x0039_8970;
                    }
                }
                0x0039_8970 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S2][0];
                    pc = 0x0039_8974;
                }
                0x0039_8974 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x0039_897c);
                    c.g[A1][0] = c.g[S1][0];
                    callee::UI_OBJECT_UPDATE_ANIM.call(c, 0x0039_897c)?;
                    frame_restore_group(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, UI_GROUP_UPDATE_ENTRY, UI_GROUP_UPDATE_END);
                    return Ok(());
                }
            }
        }
    })
}

/// The shared epilogue of [`c_ui_group_update_anim`].
pub(crate) fn frame_restore_group(c: &mut Cpu) {
    let frame = Frame { size: 0x40 };
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `UIObject* cUIGroup_FindById(cUIGroup* self, int id)` (0x3986b0): search
/// the group's own objects, then walk the sub-list looking for an object with
/// the id.
pub fn c_ui_group_find_by_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at_3(sp0, 0x20));
        c.sq(17, at_3(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        callee::UI_OBJECT_FIND_BY_ID.call(c, 0x0039_86cc)?;
        let hit = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S0][0];
        // The generated code charges the final +1 on every path except the
        // in-list hit, which leaves through the outer block.
        let mut charge_one = true;
        if !hit {
            c.cycles += 5;
            let this = c.g[S0][0];
            c.g[A0][0] = sx(at_3(this as u32, 0x74));
            callee::UI_LIST_FIRST_4.call(c, 0x0039_86dc)?;
            c.g[S0][0] = c.g[V0][0];
            let empty = c.g[S0][0] == 0;
            c.g[V0][0] = 0;
            if !empty {
                c.cycles += 2;
                c.g[V1][0] = lw_at(c, c.g[S0][0], 8);
                loop {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = lh_at(c, c.g[V1][0], 0x70);
                    c.g[V0][0] = lw_at(c, c.g[V1][0], 0x74);
                    let func = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(at_3(c.g[S0][0] as u32, c.g[A0][0] as u32));
                    call_ptr(c, func, 0x0039_8704)?;
                    if c.g[V0][0] != 0 {
                        c.lq(16, at_3(sp0, 0x20));
                        charge_one = false;
                        break;
                    }
                    c.cycles += 5;
                    c.g[S0][0] = lw_at(c, c.g[S0][0], 4);
                    c.g[A0][0] = c.g[S0][0];
                    callee::UI_LIST_NODE_SENTINEL.call(c, 0x0039_8718)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                        break;
                    }
                    c.g[V1][0] = lw_at(c, c.g[S0][0], 8);
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_86f0);
                    }
                }
            }
        }
        if charge_one {
            c.cycles += 1;
            c.lq(16, at_3(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at_3(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_3(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void UIListBox_SetItemWideString(listbox, index, str)` (0x399d80): replace
/// item `index`'s wide string with a copy of `str`.
pub fn ui_list_box_set_item_wide_string(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        c.cycles += 13;
        c.g[A3][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[A1][0] &= 0xff;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = lw(c, at_4(c.g[A3][0], 0x5c));
        c.g[V0][0] = lw(c, at_4(c.g[A0][0], 0xd0));
        c.g[V1][0] = lw(c, at_4(c.g[V0][0], 0x10));
        c.g[A0][0] = lw(c, at_4(c.g[V1][0], 0x10));
        let t = c.g[A0][0] == 0;
        c.g[S2][0] = c.g[A2][0];
        if t {
            c.cycles += 6;
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        {
            c.cycles += 7;
            c.g[V0][0] = 0x14;
            c.g[V1][0] = sx(at_4(c.g[A3][0], 0x9c));
            c.mult(0, 2, 5, 2);
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = lw(c, c.g[S1][0] as u32);
            let t = c.g[A0][0] == 0;
            if !t {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0039_9dd8);
                call_ptr(c, 0x0031_7e98, 0x0039_9dd8)?;
            }
        }
        c.cycles += 14;
        c.g[RA][0] = sx(0x0039_9de0);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x003d_bca0, 0x0039_9de0)?;
        c.g[V0][0] = sx(at_4(c.g[V0][0], 1));
        c.g[A1][0] = sx(0x0049_0000);
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[RA][0] = sx(0x0039_9dfc);
        c.g[A1][0] = sx(0x0049_3dc0);
        call_ptr(c, 0x0031_7e30, 0x0039_9dfc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0039_9e0c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003d_be20, 0x0039_9e0c)?;
        Addr(c.g[S1][0] as u32).sw(c, c.g[S0][0]);
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIEngine_TextureSet_dtor` (0x00398078).
//
// `void UIEngine::TextureSet::~TextureSet(void* self, int flags)`: release
// every texture through the engine's virtual `textureRelease`, free the two
// arrays, and (flags & 1) delete the set itself.

pub fn ui_engine_texture_set_dtor(c: &mut Cpu) {
    guest(c, |c| {
        'b_3980e0: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = 0u64;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = c.g[A1][0];
            if t {
                break 'b_3980e0;
            } else {
                c.cycles += 2;
                c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                'l_3980a8: loop {
                    c.cycles += 14;
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A2][0] = vslot(c, c.g[V0][0], 0x190, A2);
                    c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A2][0] as u32;
                    c.g[RA][0] = sx(0x003980d0);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x003980d0)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                    if t {
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x003980a8);
                        }
                        continue 'l_3980a8;
                    } else {
                        break 'b_3980e0;
                    }
                }
            }
        }
        'b_3980f8: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                break 'b_3980f8;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003980f4);
                call_ptr(c, 0x00317e98, 0x003980f4)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                break 'b_3980f8;
            }
        }
        'b_39810c: {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[V0][0] = c.g[S2][0] & 0x1;
            if t {
                break 'b_39810c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00398108);
                call_ptr(c, 0x00317e98, 0x00398108)?;
                c.g[V0][0] = c.g[S2][0] & 0x1;
                break 'b_39810c;
            }
        }
        'b_398120: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_398120;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0039811c);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00317e50, 0x0039811c)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_398120;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `UIIcon_Load(this, data)` (0x398998): load the icon's UI object, copy the
/// clip/align flags, find the texture in the scene's texture set and store
/// the scaled size.
pub fn ui_icon_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 49;
        let this = abi::arg(c, 0);
        let data = abi::arg(c, 1);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = abi::arg64(c, 1);
        callee::UI_OBJECT_LOAD.call(c, 0x0039_89b4)?;

        c.g[V0][0] = Addr(data).off(0x20).lw(c);
        c.g[A0][0] = sx(0xffff_fffd_u32);
        c.g[V1][0] = Addr(this).off(0x74).lw(c);
        c.g[A1][0] = sx(0xffff_fff3_u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[A2][0] = Addr(this).off(0x5c).lw(c);
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] & 2;
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        Addr(this).off(0x74).sw(c, c.g[V1][0]);
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V0][0] = Addr(data).off(0x20).lbu(c);
        c.g[V0][0] = c.g[V0][0] & 3;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        Addr(this).off(0x74).sw(c, c.g[V1][0]);

        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0xd0).lw(c);
        c.g[A1][0] = Addr(data).off(0x24).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58));
        callee::UI_TEXTURE_SET_FIND_2.call(c, 0x0039_8a08)?;
        Addr(this).off(0x7c).sw(c, c.g[V0][0]);

        c.lwc1(2, gp(c, -0x2548));
        c.g[V0][0] = Addr(data).off(0x2a).lhu(c);
        c.g[V1][0] = Addr(data).off(0x28).lhu(c);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        frame.restore(c, 17, 0x20);
        c.mtc1(3, 1);
        c.cvt_s_w(1, 1);
        frame.restore_ra(c, 0x10);
        c.mul_s(0, 0, 2);
        c.mul_s(1, 1, 2);
        c.swc1(0, sp(c).wrapping_add(4));
        c.swc1(1, sp(c));
        c.ldl(2, sp(c).wrapping_add(7));
        c.ldr(2, sp(c));
        c.sdl(2, this.wrapping_add(0x87));
        c.sdr(2, this.wrapping_add(0x80));
        frame.restore(c, 16, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void UIEngine_dtor(UIEngine *self, int flags)` (0x397b70): destroy the
/// engine's child lists, texture set and state manager, then free the object.
pub fn ui_engine_dtor(c: &mut Cpu) {
    guest(c, |c| {
        'b_397ba8: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
            let t = c.g[A2][0] == 0u64;
            c.g[S1][0] = c.g[A1][0];
            if t {
                break 'b_397ba8;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x3;
                vslot_2(c, c.g[V0][0], 0x8, V1);
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00397ba8);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00397ba8)?;
                break 'b_397ba8;
            }
        }
        'b_397bc0: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            if c.g[A0][0] == 0u64 {
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                break 'b_397bc0;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00397bbc);
                call_ptr(c, 0x00317e98, 0x00397bbc)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                break 'b_397bc0;
            }
        }
        'b_397bdc: {
            c.cycles += 2;
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = 0x3;
            if t {
                break 'b_397bdc;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                vslot_2(c, c.g[V0][0], 0x8, V1);
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00397bdc);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00397bdc)?;
                break 'b_397bdc;
            }
        }
        'b_397bfc: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = 0x3;
            if t {
                break 'b_397bfc;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                vslot_2(c, c.g[V0][0], 0x8, V1);
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00397bfc);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00397bfc)?;
                break 'b_397bfc;
            }
        }
        'b_397c2c: {
            c.cycles += 9;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
            c.g[RA][0] = sx(0x00397c08);
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x00398078, 0x00397c08)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            c.g[RA][0] = sx(0x00397c14);
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x0039e9d8, 0x00397c14)?;
            c.g[V0][0] = c.g[S1][0] & 0x1;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_397c2c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00397c28);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00317e50, 0x00397c28)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_397c2c;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// List box.

/// `void UIListBox_RefreshText(UIListBox* self)` (0x399e28): set the text of
/// every row from the localised string table, then relayout the box.
pub fn uilistbox_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x5c)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0xd0)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lw(c);
        c.g[S2][0] = Addr(p(r32(c, A0), 0x10)).lw(c);
        let no_storage = c.g[S2][0] == 0;
        c.lq(S0, frame.at(c, 0x50));
        if !no_storage {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S1), 0x318)).lbu(c);
            let none = c.g[V0][0] == 0;
            c.g[S0][0] = 0;
            if !none {
                c.cycles += 3;
                c.g[S4][0] = 0x14;
                c.g[S3][0] = sx(p(r32(c, S1), 0x98));
                c.mult(0, 3, 16, 20);
                loop {
                    c.cycles += 4;
                    c.g[V0][0] = sx(p(r32(c, V1), r32(c, S3)));
                    c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                    let empty = c.g[A1][0] == 0;
                    c.g[V1][0] = sx(r32(c, S0).wrapping_add(1));
                    if !empty {
                        c.cycles += 7;
                        c.g[V1][0] = Addr(p(r32(c, S2), 4)).lw(c);
                        c.g[A0][0] = Addr(p(r32(c, V1), 0x20)).lh(c);
                        c.g[V0][0] = Addr(p(r32(c, V1), 0x24)).lw(c);
                        let get = r32(c, V0);
                        c.g[A0][0] = sx(p(r32(c, S2), r32(c, A0)));
                        call_ptr(c, get, 0x0039_9e9c)?;
                        let is_null = c.g[V0][0] == 0;
                        c.g[A2][0] = c.g[V0][0];
                        if is_null {
                            c.cycles += 1;
                            c.g[V1][0] = sx(r32(c, S0).wrapping_add(1));
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, 0x0039_9d80, 0x0039_9eb0)?;
                            c.cycles += 1;
                            c.g[V1][0] = sx(r32(c, S0).wrapping_add(1));
                        }
                    } else {
                        c.cycles += 1;
                        c.g[V1][0] = sx(r32(c, S0).wrapping_add(1));
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(r32(c, S1), 0x318)).lbu(c);
                    c.g[S0][0] = c.g[V1][0] & 0xff;
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.mult(0, 3, 16, 20);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_9e78);
                    }
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(p(r32(c, V0), 0x28)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x2c)).lw(c);
        let relayout = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, relayout, 0x0039_9ee0)?;
        c.lq(S0, frame.at(c, 0x50));
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00399820(UIListBox* self, sUIData* data)` (0x399820): load the
/// list box's shared UI-object data, take its horizontal alignment and
/// visibility from `data`, and when the "wrap" flag is set compute the four
/// texel-size floats from the byte channels at `data+0x24`.
pub fn uilistbox_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        let data = c.g[A1][0];
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = data;
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x0039_983c);
        c.g[S1][0] = c.g[A0][0];
        c.pc = 0x0039_fe00;
        ee::s13::UIObject_Load(c);
        if c.pc != 0x0039_983c {
            return Err(Unwound);
        }
        c.g[V1][0] = c.lbu(p(c.g[S0][0] as u32, 0x20));
        c.g[A0][0] = 0xffff_ffff_ffff_fffc;
        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x74));
        c.g[V1][0] = c.g[V1][0] & 0x3;
        c.g[A2][0] = c.lw(p(c.g[S1][0] as u32, 8)); // vtable
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.sw(p(c.g[S1][0] as u32, 0x74), c.g[V0][0]);
        c.g[A0][0] = c.lh(p(c.g[A2][0] as u32, 0xb8));
        c.g[A1][0] = c.lhu(p(c.g[S0][0] as u32, 0x22));
        c.g[V1][0] = c.lw(p(c.g[A2][0] as u32, 0xbc));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let func = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039_9874);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        call_ptr(c, func, 0x0039_9874)?;
        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x74));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        if c.g[V0][0] != 0 {
            c.cycles += 30;
            c.g[V0][0] = c.lbu(p(c.g[S0][0] as u32, 0x27));
            c.g[V1][0] = c.lbu(p(c.g[S0][0] as u32, 0x24));
            c.g[A0][0] = c.lbu(p(c.g[S0][0] as u32, 0x25));
            c.mtc1(2, 4);
            c.cvt_s_w(4, 4);
            c.g[A1][0] = c.lbu(p(c.g[S0][0] as u32, 0x26));
            c.mtc1(3, 2);
            c.cvt_s_w(2, 2);
            c.mtc1(4, 3);
            c.cvt_s_w(3, 3);
            c.lwc1(0, gp_2(c, -0x2544)); // texel scale
            c.mtc1(5, 1);
            c.cvt_s_w(1, 1);
            c.mul_s(4, 4, 0);
            c.mul_s(2, 2, 0);
            c.mul_s(3, 3, 0);
            c.mul_s(1, 1, 0);
            c.swc1(4, p(c.g[SP][0] as u32, 0xc));
            c.swc1(2, c.g[SP][0] as u32);
            c.swc1(3, p(c.g[SP][0] as u32, 4));
            c.swc1(1, p(c.g[SP][0] as u32, 8));
            c.ldl(2, p(c.g[SP][0] as u32, 7));
            c.ldr(2, c.g[SP][0] as u32);
            c.ldl(3, p(c.g[SP][0] as u32, 0xf));
            c.ldr(3, p(c.g[SP][0] as u32, 8));
            c.sdl(2, p(c.g[S1][0] as u32, 0x7f));
            c.sdr(2, p(c.g[S1][0] as u32, 0x78));
            c.sdl(3, p(c.g[S1][0] as u32, 0x87));
            c.sdr(3, p(c.g[S1][0] as u32, 0x80));
            c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x14));
        } else {
            c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x14));
        }
        c.cycles += 7;
        c.lq(16, p(c.g[SP][0] as u32, 0x30));
        c.g[V0][0] = c.g[V0][0] | 0x80;
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        c.sw(p(c.g[S1][0] as u32, 0x14), c.g[V0][0]);
        c.lq(17, p(c.g[SP][0] as u32, 0x20));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UIIcon_Render (0x398a78)
// ---------------------------------------------------------------------------

/// `GrMan` (the UI render context) at `gp - 0x854`.
pub(crate) const UI_ICON_RENDER_GRMAN: u32 = 0x854;

/// GrMan: the current render-state record (0x14 bytes each, a stack).
pub(crate) const UI_ICON_RENDER_CTX_TOP: u32 = 0xe84;

/// GrMan: its vtable (slot at +0x238: draw sprite quad (n, verts, 0)).
pub(crate) const UI_ICON_RENDER_CTX_VT: u32 = 0x10d8;

/// Icon fields.
pub(crate) const UI_ICON_RENDER_VT: u32 = 0x8;

pub(crate) const UI_ICON_RENDER_BLEND: u32 = 0x15;

pub(crate) const UI_ICON_RENDER_COLOR: u32 = 0x1c;

pub(crate) const UI_ICON_RENDER_ROT: u32 = 0x2c;

pub(crate) const UI_ICON_RENDER_POS: u32 = 0x44;

pub(crate) const UI_ICON_RENDER_SCALE: u32 = 0x50;

pub(crate) const UI_ICON_RENDER_FLAGS: u32 = 0x74;

pub(crate) const UI_ICON_RENDER_TEX_ID: u32 = 0x78;

pub(crate) const UI_ICON_RENDER_TEX: u32 = 0x7c;

/// The 3x3 base matrix the rotations start from (`D_004FF170`).
pub(crate) const UI_ICON_RENDER_BASE_MATRIX: u32 = 0x004f_f170;

/// `sp + off` (the frame: four 0x30-byte vertices at 0, the position at
/// 0xc0, the size at 0xd0, the scale at 0xe0, the matrix at 0xf0).
#[inline(always)]
pub(crate) fn ui_icon_render_sp(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `s0 + off` (the icon).
#[inline(always)]
pub(crate) fn ui_icon_render_s0(c: &Cpu, off: u32) -> u32 {
    (c.g[S0][0] as u32).wrapping_add(off)
}

/// One of the three optional axis rotations of [`ui_icon_render`]
/// (`Rotate_8A78`): the per-axis stack slots and scratch registers the
/// compiled code uses.
pub(crate) struct UiIconRenderAxis {
    /// Which component of the unit axis at sp+0x120 is 1.
    axis: u32,
    /// Math_SinCos outputs.
    sin: u32,
    cos: u32,
    /// The rotation matrix r.
    rot: u32,
    /// The copy of m.
    copy: u32,
    /// GPRs carrying m's four doublewords, and the one for its last word.
    gprs: [usize; 4],
    word: usize,
    /// Math_SinCos's return address.
    ret: u32,
}

/// m = r * m for a rotation of angle `f12` about one unit axis (the block
/// after the `c.eq.s f12, f27` test; `f27` = 0.0). Charges the block's 138
/// cycles.
pub(crate) fn ui_icon_render_rotate(c: &mut Cpu, a: &UiIconRenderAxis) -> Flow {
    let (sin, cos, rot, copy, word) = (a.sin, a.cos, a.rot, a.copy, a.word);
    let [g0, g1, g2, g3] = a.gprs;
    c.cycles += 138;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 20); // f20 = 1.0
    c.g[A0][0] = sx(ui_icon_render_sp(c, sin));
    c.g[A1][0] = sx(ui_icon_render_sp(c, cos));
    for i in 0..3 {
        let f = if i == a.axis { 20 } else { 27 };
        c.swc1(f, ui_icon_render_sp(c, 0x120 + 4 * i));
    }
    call_ptr(c, 0x0031_be50, a.ret)?; // Math_SinCos(&sin, &cos, angle)

    // r = the rotation matrix, then copy m, then m = r * copy.
    c.lwc1(10, ui_icon_render_sp(c, cos));
    c.lwc1(0, ui_icon_render_sp(c, 0x120));
    c.sub_s(20, 20, 10);
    c.lwc1(2, ui_icon_render_sp(c, 0x128));
    c.lwc1(4, ui_icon_render_sp(c, 0x124));
    c.lwc1(6, ui_icon_render_sp(c, sin));
    c.mul_s(1, 20, 2);
    c.mul_s(3, 20, 0);
    c.mul_s(20, 20, 4);
    c.mul_s(11, 6, 2);
    c.mul_s(12, 6, 0);
    c.mul_s(8, 1, 2);
    c.mul_s(9, 3, 2);
    c.mul_s(5, 1, 0);
    c.mul_s(7, 3, 0);
    c.mul_s(2, 20, 2);
    c.mul_s(0, 20, 0);
    c.mul_s(6, 6, 4);
    c.mul_s(1, 1, 4);
    c.mul_s(3, 3, 4);
    c.mul_s(20, 20, 4);
    c.add_s(5, 5, 6);
    c.sub_s(0, 0, 11);
    c.sub_s(1, 1, 12);
    c.add_s(8, 8, 10);
    c.swc1(5, ui_icon_render_sp(c, rot + 0x8));
    c.add_s(7, 7, 10);
    c.swc1(0, ui_icon_render_sp(c, rot + 0x4));
    c.add_s(3, 3, 11);
    c.swc1(1, ui_icon_render_sp(c, rot + 0x14));
    c.sub_s(9, 9, 6);
    c.swc1(8, ui_icon_render_sp(c, rot + 0x20));
    c.add_s(20, 20, 10);
    c.swc1(7, ui_icon_render_sp(c, rot));
    c.add_s(2, 2, 12);
    c.swc1(3, ui_icon_render_sp(c, rot + 0xc));
    c.swc1(9, ui_icon_render_sp(c, rot + 0x18));
    c.swc1(20, ui_icon_render_sp(c, rot + 0x10));
    c.swc1(2, ui_icon_render_sp(c, rot + 0x1c));
    c.ldl(g0, ui_icon_render_sp(c, 0xf7));
    c.ldr(g0, ui_icon_render_sp(c, 0xf0));
    c.ldl(g1, ui_icon_render_sp(c, 0xff));
    c.ldr(g1, ui_icon_render_sp(c, 0xf8));
    c.ldl(g2, ui_icon_render_sp(c, 0x107));
    c.ldr(g2, ui_icon_render_sp(c, 0x100));
    c.ldl(g3, ui_icon_render_sp(c, 0x10f));
    c.ldr(g3, ui_icon_render_sp(c, 0x108));
    c.sdl(g0, ui_icon_render_sp(c, copy + 0x7));
    c.sdr(g0, ui_icon_render_sp(c, copy));
    c.sdl(g1, ui_icon_render_sp(c, copy + 0xf));
    c.sdr(g1, ui_icon_render_sp(c, copy + 0x8));
    c.sdl(g2, ui_icon_render_sp(c, copy + 0x17));
    c.sdr(g2, ui_icon_render_sp(c, copy + 0x10));
    c.sdl(g3, ui_icon_render_sp(c, copy + 0x1f));
    c.sdr(g3, ui_icon_render_sp(c, copy + 0x18));
    c.g[word][0] = c.lw(ui_icon_render_sp(c, 0x110));
    c.sw(ui_icon_render_sp(c, copy + 0x20), c.g[word][0]);
    c.lwc1(0, ui_icon_render_sp(c, copy));
    c.lwc1(7, ui_icon_render_sp(c, rot));
    c.lwc1(2, ui_icon_render_sp(c, copy + 0xc));
    c.lwc1(16, ui_icon_render_sp(c, rot + 0x4));
    c.mul_s(9, 0, 7);
    c.lwc1(19, ui_icon_render_sp(c, rot + 0xc));
    c.lwc1(20, ui_icon_render_sp(c, rot + 0x10));
    c.mul_s(12, 2, 16);
    c.lwc1(6, ui_icon_render_sp(c, rot + 0x18));
    c.mul_s(8, 0, 19);
    c.lwc1(15, ui_icon_render_sp(c, rot + 0x1c));
    c.mul_s(5, 2, 20);
    c.mul_s(0, 0, 6);
    c.lwc1(3, ui_icon_render_sp(c, copy + 0x4));
    c.mul_s(2, 2, 15);
    c.lwc1(10, ui_icon_render_sp(c, copy + 0x10));
    c.lwc1(1, ui_icon_render_sp(c, copy + 0x18));
    c.mul_s(4, 3, 7);
    c.lwc1(14, ui_icon_render_sp(c, rot + 0x8));
    c.mul_s(17, 10, 16);
    c.lwc1(11, ui_icon_render_sp(c, rot + 0x20));
    c.add_s(0, 0, 2);
    c.lwc1(18, ui_icon_render_sp(c, rot + 0x14));
    c.mul_s(2, 1, 14);
    c.mul_s(13, 1, 11);
    c.add_s(9, 9, 12);
    c.lwc1(12, ui_icon_render_sp(c, copy + 0x1c));
    c.add_s(8, 8, 5);
    c.mul_s(1, 1, 18);
    c.mul_s(5, 12, 14);
    c.add_s(4, 4, 17);
    c.add_s(9, 9, 2);
    c.add_s(8, 8, 1);
    c.add_s(0, 0, 13);
    c.add_s(4, 4, 5);
    c.swc1(9, ui_icon_render_sp(c, 0xf0));
    c.swc1(8, ui_icon_render_sp(c, 0xfc));
    c.mul_s(2, 10, 15);
    c.swc1(0, ui_icon_render_sp(c, 0x108));
    c.mul_s(5, 3, 6);
    c.swc1(4, ui_icon_render_sp(c, 0xf4));
    c.mul_s(3, 3, 19);
    c.lwc1(1, ui_icon_render_sp(c, copy + 0x8));
    c.mul_s(10, 10, 20);
    c.lwc1(0, ui_icon_render_sp(c, copy + 0x14));
    c.add_s(5, 5, 2);
    c.mul_s(6, 1, 6);
    c.lwc1(2, ui_icon_render_sp(c, copy + 0x20));
    c.mul_s(15, 0, 15);
    c.mul_s(7, 1, 7);
    c.mul_s(16, 0, 16);
    c.mul_s(0, 0, 20);
    c.mul_s(1, 1, 19);
    c.mul_s(14, 2, 14);
    c.add_s(3, 3, 10);
    c.add_s(1, 1, 0);
    c.mul_s(0, 2, 11);
    c.mul_s(11, 12, 11);
    c.add_s(7, 7, 16);
    c.add_s(6, 6, 15);
    c.mul_s(2, 2, 18);
    c.mul_s(12, 12, 18);
    c.add_s(5, 5, 11);
    c.add_s(7, 7, 14);
    c.add_s(3, 3, 12);
    c.add_s(1, 1, 2);
    c.swc1(5, ui_icon_render_sp(c, 0x10c));
    c.add_s(6, 6, 0);
    c.swc1(7, ui_icon_render_sp(c, 0xf8));
    c.swc1(3, ui_icon_render_sp(c, 0x100));
    c.swc1(1, ui_icon_render_sp(c, 0x104));
    c.swc1(6, ui_icon_render_sp(c, 0x110));
    Ok(())
}

/// `void UIIcon_Render(cUIIcon* self, float x, float y)` (0x398a78,
/// `cUIIcon_render2D`): draws the icon as one textured quad at (x, y).
/// Picks the texture (the texture page entry's uv rectangle, else the whole
/// texture `texId`, else untextured), mirrors the uvs (flag 2) and turns
/// them by quarter turns (flags bits 2-3), sizes the quad (the size getter
/// times the scale, aligned by `UIObject_ApplyAlignment`), rotates it about
/// the icon's position by the x/y/z angles, sets the colour (alpha, r, g,
/// b times 128), pushes a render state with the blend mode from +0x15 and
/// calls the context's sprite drawer with the four vertices.
pub fn ui_icon_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x2a0);
        let gp = c.g[GP][0] as u32;
        c.g[A1][0] = c.lw(gp.wrapping_sub(UI_ICON_RENDER_GRMAN));
        frame.save(c, S0, 0x240);
        c.g[V0][0] = 0x3;
        frame.save(c, S1, 0x230);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x220);
        c.g[S1][0] = sx(ui_icon_render_sp(c, 0xd0)); // &size
        frame.save_fpr(c, 21, 0x258);
        c.g[S2][0] = sx(ui_icon_render_sp(c, 0xc0)); // &pos
        frame.save_fpr(c, 20, 0x250);
        c.mov_s(21, 13); // y
        frame.save_ra(c, 0x210);
        c.mov_s(20, 12); // x
        for (f, off) in [
            (28, 0x290),
            (27, 0x288),
            (26, 0x280),
            (25, 0x278),
            (24, 0x270),
            (23, 0x268),
            (22, 0x260),
        ] {
            frame.save_fpr(c, f, off);
        }
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_TEX));

        // The texture and the vertices' uvs (vertex i at sp + 0x30*i: u, v).
        if c.g[A0][0] != 0 {
            // A texture page entry: id, then the uv rectangle (v0 0xc, u0
            // 0x10, u1 0x14, v1 0x18).
            c.cycles += 21;
            c.g[A0][0] = c.lw(c.g[A0][0] as u32);
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
            c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
            c.g[V1][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_TEX));
            for (f, src, dst) in [
                (0, 0x10, 0x0),
                (1, 0xc, 0x4),
                (0, 0x14, 0x30),
                (1, 0xc, 0x34),
                (0, 0x10, 0x60),
                (1, 0x18, 0x64),
                (0, 0x14, 0x90),
                (1, 0x18, 0x94),
            ] {
                c.lwc1(f, (c.g[V1][0] as u32).wrapping_add(src));
                c.swc1(f, ui_icon_render_sp(c, dst));
            }
        } else {
            c.g[V1][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_TEX_ID));
            c.cycles += 2;
            if (c.g[V1][0] as i64) <= 0 {
                // Untextured.
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            } else {
                // The whole texture: uvs (0,0) (1,0) (0,1) (1,1).
                c.cycles += 13;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.swc1(0, ui_icon_render_sp(c, 0x94));
                c.sw(ui_icon_render_sp(c, 0x0), 0);
                c.sw(ui_icon_render_sp(c, 0x4), 0);
                c.swc1(0, ui_icon_render_sp(c, 0x30));
                c.sw(ui_icon_render_sp(c, 0x34), 0);
                c.sw(ui_icon_render_sp(c, 0x60), 0);
                c.swc1(0, ui_icon_render_sp(c, 0x64));
                c.swc1(0, ui_icon_render_sp(c, 0x90));
            }
        }

        // Flag 2: mirror (swap u0/u1, v0/v1, u2/u3, v2/v3).
        c.cycles += 4;
        c.g[V0][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_FLAGS)) & 0x2;
        c.g[A0][0] = c.g[SP][0];
        if c.g[V0][0] != 0 {
            c.cycles += 11;
            let swap = Callee {
                addr: 0x0039_8a60,
                f: ee::s13::UIIcon_SwapWords,
            };
            c.g[A1][0] = sx(ui_icon_render_sp(c, 0x30));
            swap.call(c, 0x0039_8b8c)?;
            c.g[A0][0] = c.g[SP][0] | 0x4;
            c.g[A1][0] = sx(ui_icon_render_sp(c, 0x34));
            swap.call(c, 0x0039_8b98)?;
            c.g[A0][0] = sx(ui_icon_render_sp(c, 0x60));
            c.g[A1][0] = sx(ui_icon_render_sp(c, 0x90));
            swap.call(c, 0x0039_8ba4)?;
            c.g[A0][0] = sx(ui_icon_render_sp(c, 0x64));
            c.g[A1][0] = sx(ui_icon_render_sp(c, 0x94));
            swap.call(c, 0x0039_8bb0)?;
        }

        // Flags bits 2-3: quarter turns of the uvs (0 <- 2 <- 3 <- 1 <- 0),
        // the counter kept in the top byte of v1.
        c.cycles += 5;
        c.g[V0][0] = sx(c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_FLAGS)) as u32 >> 2) & 0x3;
        c.lwc1(8, ui_icon_render_sp(c, 0x0));
        if c.g[V0][0] != 0 {
            c.cycles += 11;
            c.g[A0][0] = c.g[V0][0];
            c.lwc1(7, ui_icon_render_sp(c, 0x60));
            c.g[V1][0] = 0x1000000;
            c.lwc1(6, ui_icon_render_sp(c, 0x90));
            c.g[A1][0] = 0x1000000;
            c.lwc1(5, ui_icon_render_sp(c, 0x30));
            c.lwc1(4, ui_icon_render_sp(c, 0x4));
            c.lwc1(3, ui_icon_render_sp(c, 0x64));
            c.lwc1(2, ui_icon_render_sp(c, 0x94));
            c.lwc1(1, ui_icon_render_sp(c, 0x34));
            loop {
                c.cycles += 15;
                c.mov_s(0, 8);
                c.mov_s(8, 7);
                c.mov_s(7, 6);
                c.mov_s(6, 5);
                let i = (c.g[V1][0] as u32 as i32) >> 24;
                c.g[V0][0] = ((sx(i as u32) as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.mov_s(5, 0);
                c.mov_s(0, 4);
                c.mov_s(4, 3);
                c.mov_s(3, 2);
                c.mov_s(2, 1);
                c.mov_s(1, 0);
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_8bf0);
                }
            }
            c.cycles += 8;
            for (f, off) in [
                (1, 0x34),
                (2, 0x94),
                (3, 0x64),
                (4, 0x4),
                (5, 0x30),
                (6, 0x90),
                (7, 0x60),
                (8, 0x0),
            ] {
                c.swc1(f, ui_icon_render_sp(c, off));
            }
        }

        // pos = self->pos, scale = self->scale, size = getSize() * scale,
        // aligned; pos += (x, y); m = the base matrix.
        c.cycles += 61;
        c.ldl(2, ui_icon_render_s0(c, UI_ICON_RENDER_POS + 7));
        c.ldr(2, ui_icon_render_s0(c, UI_ICON_RENDER_POS));
        c.g[V1][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_POS + 8));
        c.sdl(2, ui_icon_render_sp(c, 0xc7));
        c.sdr(2, ui_icon_render_sp(c, 0xc0));
        c.sw(ui_icon_render_sp(c, 0xc8), c.g[V1][0]);
        c.g[V1][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_VT));
        c.g[A1][0] = c.g[S1][0];
        c.ldl(6, ui_icon_render_s0(c, UI_ICON_RENDER_SCALE + 7));
        c.ldr(6, ui_icon_render_s0(c, UI_ICON_RENDER_SCALE));
        c.g[A3][0] = c.lw(ui_icon_render_s0(c, UI_ICON_RENDER_SCALE + 8));
        c.sdl(6, ui_icon_render_sp(c, 0xe7));
        c.sdr(6, ui_icon_render_sp(c, 0xe0));
        c.sw(ui_icon_render_sp(c, 0xe8), c.g[A3][0]);
        c.mtc1(0, 27);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let get_size = c.g[V0][0] as u32;
        call_ptr(c, get_size, 0x0039_8c98)?; // self->getSize(&size)
        c.lwc1(2, ui_icon_render_sp(c, 0xd0));
        c.g[A1][0] = c.g[S2][0];
        c.lwc1(0, ui_icon_render_sp(c, 0xd4));
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(1, ui_icon_render_sp(c, 0xe4));
        c.g[A2][0] = c.g[S1][0];
        c.lwc1(3, ui_icon_render_sp(c, 0xe0));
        c.mul_s(0, 0, 1);
        c.mul_s(2, 2, 3);
        c.swc1(0, ui_icon_render_sp(c, 0xd4));
        c.swc1(2, ui_icon_render_sp(c, 0xd0));
        Callee {
            addr: 0x0039_feb8,
            f: ee::s13::UIObject_ApplyAlignment,
        }
        .call(c, 0x0039_8cc8)?; // UIObject_ApplyAlignment(self, &pos, &size)
        c.lwc1(0, ui_icon_render_sp(c, 0xc0));
        c.g[V0][0] = 0x500000;
        c.lwc1(1, ui_icon_render_sp(c, 0xc4));
        c.lwc1(12, ui_icon_render_s0(c, UI_ICON_RENDER_ROT));
        c.add_s(0, 0, 20);
        c.add_s(1, 1, 21);
        c.c_eq_s(12, 27);
        c.swc1(0, ui_icon_render_sp(c, 0xc0));
        c.swc1(1, ui_icon_render_sp(c, 0xc4));
        c.g[A1][0] = sx(UI_ICON_RENDER_BASE_MATRIX); // 0x500000 - 0xe90
        c.ldl(11, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(11, c.g[A1][0] as u32);
        c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0x17));
        c.ldr(4, (c.g[A1][0] as u32).wrapping_add(0x10));
        c.sdl(11, ui_icon_render_sp(c, 0xf7));
        c.sdr(11, ui_icon_render_sp(c, 0xf0));
        c.sdl(3, ui_icon_render_sp(c, 0xff));
        c.sdr(3, ui_icon_render_sp(c, 0xf8));
        c.sdl(4, ui_icon_render_sp(c, 0x107));
        c.sdr(4, ui_icon_render_sp(c, 0x100));
        c.ldl(11, (c.g[A1][0] as u32).wrapping_add(0x1f));
        c.ldr(11, (c.g[A1][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x20));
        c.sdl(11, ui_icon_render_sp(c, 0x10f));
        c.sdr(11, ui_icon_render_sp(c, 0x108));
        c.sw(ui_icon_render_sp(c, 0x110), c.g[V1][0]);

        // The x, y and z rotations (each only for a non-zero angle).
        if !c.fcc() {
            ui_icon_render_rotate(
                c,
                &UiIconRenderAxis {
                    axis: 0,
                    sin: 0x1f0,
                    cos: 0x1f4,
                    rot: 0x130,
                    copy: 0x160,
                    gprs: [A2, A3, T0, T1],
                    word: A2,
                    ret: 0x0039_8d60,
                },
            )?;
        }
        c.lwc1(12, ui_icon_render_s0(c, UI_ICON_RENDER_ROT + 4));
        c.cycles += 4;
        c.c_eq_s(12, 27);
        if !c.fcc() {
            ui_icon_render_rotate(
                c,
                &UiIconRenderAxis {
                    axis: 1,
                    sin: 0x1f8,
                    cos: 0x1fc,
                    rot: 0x190,
                    copy: 0x1c0,
                    gprs: [T3, V0, V1, A0],
                    word: T3,
                    ret: 0x0039_8f98,
                },
            )?;
        }
        c.lwc1(12, ui_icon_render_s0(c, UI_ICON_RENDER_ROT + 8));
        c.cycles += 4;
        c.c_eq_s(12, 27);
        if !c.fcc() {
            ui_icon_render_rotate(
                c,
                &UiIconRenderAxis {
                    axis: 2,
                    sin: 0x200,
                    cos: 0x204,
                    rot: 0x130,
                    copy: 0x160,
                    gprs: [A1, A2, A3, T0],
                    word: A1,
                    ret: 0x0039_91d0,
                },
            )?;
        }
        c.lwc1(26, ui_icon_render_s0(c, UI_ICON_RENDER_POS));

        // The corners (pos - self->pos and pos + size - self->pos) through
        // the upper two rows of m, w = 1, then + self->pos with VU0.
        c.cycles += 146;
        c.g[A0][0] = sx(ui_icon_render_sp(c, 0x20));
        c.lwc1(9, ui_icon_render_sp(c, 0xc0));
        c.g[A3][0] = 0x3;
        c.lwc1(25, ui_icon_render_s0(c, UI_ICON_RENDER_POS + 4));
        c.lwc1(10, ui_icon_render_sp(c, 0xc4));
        c.sub_s(5, 9, 26);
        c.lwc1(0, ui_icon_render_sp(c, 0xd0));
        c.sub_s(8, 10, 25);
        c.lwc1(4, ui_icon_render_sp(c, 0xf0));
        c.add_s(9, 9, 0);
        c.lwc1(19, ui_icon_render_sp(c, 0xfc));
        c.lwc1(2, ui_icon_render_sp(c, 0xf4));
        c.mul_s(23, 4, 5);
        c.lwc1(17, ui_icon_render_sp(c, 0x100));
        c.mul_s(14, 19, 8);
        c.lwc1(1, ui_icon_render_sp(c, 0xf8));
        c.sub_s(20, 9, 26);
        c.lwc1(15, ui_icon_render_sp(c, 0x104));
        c.mul_s(22, 2, 5);
        c.lwc1(0, ui_icon_render_sp(c, 0xd4));
        c.mul_s(12, 17, 8);
        c.mul_s(21, 1, 5);
        c.lwc1(13, ui_icon_render_sp(c, 0xc8));
        c.mul_s(11, 15, 8);
        c.lwc1(3, ui_icon_render_sp(c, 0xd8));
        c.add_s(10, 10, 0);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 18);
        c.add_s(0, 23, 14);
        c.lwc1(24, ui_icon_render_s0(c, UI_ICON_RENDER_POS + 8));
        c.add_s(6, 22, 12);
        c.swc1(18, ui_icon_render_sp(c, 0x16c));
        c.sub_s(16, 10, 25);
        c.swc1(5, ui_icon_render_sp(c, 0x120));
        c.add_s(7, 21, 11);
        c.swc1(0, ui_icon_render_sp(c, 0x160));
        c.mul_s(4, 4, 20);
        c.swc1(6, ui_icon_render_sp(c, 0x164));
        c.mul_s(2, 2, 20);
        c.swc1(8, ui_icon_render_sp(c, 0x124));
        c.mul_s(1, 1, 20);
        c.swc1(7, ui_icon_render_sp(c, 0x168));
        c.add_s(3, 13, 3);
        c.swc1(9, ui_icon_render_sp(c, 0x140));
        c.mul_s(19, 19, 16);
        c.lq(2, ui_icon_render_sp(c, 0x160));
        c.add_s(14, 4, 14);
        c.swc1(10, ui_icon_render_sp(c, 0x144));
        c.add_s(12, 2, 12);
        c.sq(2, ui_icon_render_sp(c, 0x20));
        c.add_s(11, 1, 11);
        c.swc1(3, ui_icon_render_sp(c, 0x148));
        c.mul_s(17, 17, 16);
        c.swc1(14, ui_icon_render_sp(c, 0x160));
        c.mul_s(15, 15, 16);
        c.swc1(12, ui_icon_render_sp(c, 0x164));
        c.sub_s(0, 3, 24);
        c.swc1(11, ui_icon_render_sp(c, 0x168));
        c.sub_s(13, 13, 24);
        c.swc1(20, ui_icon_render_sp(c, 0x130));
        c.add_s(23, 23, 19);
        c.lq(2, ui_icon_render_sp(c, 0x160));
        c.add_s(22, 22, 17);
        c.swc1(0, ui_icon_render_sp(c, 0x138));
        c.add_s(21, 21, 15);
        c.swc1(13, ui_icon_render_sp(c, 0x128));
        c.swc1(16, ui_icon_render_sp(c, 0x134));
        c.add_s(4, 4, 19);
        c.swc1(23, ui_icon_render_sp(c, 0x160));
        c.add_s(2, 2, 17);
        c.swc1(22, ui_icon_render_sp(c, 0x164));
        c.add_s(1, 1, 15);
        c.swc1(21, ui_icon_render_sp(c, 0x168));
        c.mov_s(28, 18);
        c.sq(2, ui_icon_render_sp(c, 0x50));
        c.lq(3, ui_icon_render_sp(c, 0x160));
        c.swc1(4, ui_icon_render_sp(c, 0x160));
        c.swc1(2, ui_icon_render_sp(c, 0x164));
        c.swc1(1, ui_icon_render_sp(c, 0x168));
        c.sq(3, ui_icon_render_sp(c, 0x80));
        c.lq(2, ui_icon_render_sp(c, 0x160));
        c.swc1(20, ui_icon_render_sp(c, 0x150));
        c.sq(2, ui_icon_render_sp(c, 0x140));
        c.sq(2, ui_icon_render_sp(c, 0xb0));
        c.swc1(26, ui_icon_render_sp(c, 0x140));
        c.swc1(25, ui_icon_render_sp(c, 0x144));
        c.swc1(24, ui_icon_render_sp(c, 0x148));
        c.swc1(16, ui_icon_render_sp(c, 0x154));
        c.swc1(27, ui_icon_render_sp(c, 0x158));
        c.swc1(18, ui_icon_render_sp(c, 0x15c));
        c.swc1(27, ui_icon_render_sp(c, 0x14c));
        c.lqc2(3, ui_icon_render_sp(c, 0x20));
        c.lqc2(4, ui_icon_render_sp(c, 0x140));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, ui_icon_render_sp(c, 0x20));
        c.g[AT][0] = 0x43000000;
        c.mtc1(1, 4); // 128.0
        for vert in [0x50, 0x80, 0xb0] {
            c.lwc1(0, ui_icon_render_s0(c, UI_ICON_RENDER_POS));
            c.lwc1(1, ui_icon_render_s0(c, UI_ICON_RENDER_POS + 4));
            c.lwc1(2, ui_icon_render_s0(c, UI_ICON_RENDER_POS + 8));
            c.swc1(0, ui_icon_render_sp(c, 0x140));
            c.swc1(1, ui_icon_render_sp(c, 0x144));
            c.swc1(2, ui_icon_render_sp(c, 0x148));
            c.swc1(27, ui_icon_render_sp(c, 0x14c));
            c.lqc2(3, ui_icon_render_sp(c, vert));
            c.lqc2(4, ui_icon_render_sp(c, 0x140));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, ui_icon_render_sp(c, vert));
        }

        // The colour (a, r, g, b) * 128 as integers.
        c.lwc1(1, ui_icon_render_s0(c, UI_ICON_RENDER_COLOR));
        c.lwc1(2, ui_icon_render_s0(c, UI_ICON_RENDER_COLOR + 4));
        c.lwc1(3, ui_icon_render_s0(c, UI_ICON_RENDER_COLOR + 8));
        c.mul_s(1, 1, 4);
        c.lwc1(0, ui_icon_render_s0(c, UI_ICON_RENDER_COLOR + 0xc));
        c.mul_s(2, 2, 4);
        c.mul_s(3, 3, 4);
        c.mul_s(0, 0, 4);
        c.cvt_w_s(4, 1);
        c.mfc1(6, 4);
        c.cvt_w_s(1, 2);
        c.mfc1(5, 1);
        c.cvt_w_s(1, 3);
        c.mfc1(3, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);

        // Each vertex: pos.z = 0, colour, q = 1.
        loop {
            c.cycles += 9;
            let v = c.g[A0][0] as u32; // &verts[i].pos
            c.sw(v.wrapping_add(0x8), 0);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
            c.sw(v.wrapping_sub(0x4), c.g[A2][0]);
            c.sw(v.wrapping_sub(0x10), c.g[A1][0]);
            c.sw(v.wrapping_sub(0xc), c.g[V1][0]);
            c.sw(v.wrapping_sub(0x8), c.g[V0][0]);
            c.swc1(28, v.wrapping_sub(0x18));
            c.g[A0][0] = sx(v.wrapping_add(0x30));
            if (c.g[A3][0] as i64) < 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_9620);
            }
        }

        // Push the render state (blend mode (blend & 0x1f) + 9 into bits
        // 5-9 of +8, flags |= 0xc), draw the quad, pop.
        c.cycles += 58;
        let gp = c.g[GP][0] as u32;
        c.g[T0][0] = c.lw(gp.wrapping_sub(UI_ICON_RENDER_GRMAN));
        c.g[T2][0] = 0xfffffffffffffc1f;
        c.g[A1][0] = 0x4;
        c.g[A2][0] = c.g[SP][0];
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
        c.g[A3][0] = 0;
        c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[V1][0] as u32);
        c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[T1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x1b));
        c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x14));
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x23));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x1c));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x24), c.g[T1][0]);
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
        c.g[T1][0] = c.lw(gp.wrapping_sub(UI_ICON_RENDER_GRMAN));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
        c.sw(
            (c.g[T0][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP),
            c.g[V0][0],
        );
        c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
        c.g[V0][0] = c.lw(c.g[A0][0] as u32) | 0xc;
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        c.g[V1][0] = c.lbu(ui_icon_render_s0(c, UI_ICON_RENDER_BLEND));
        c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
        c.g[V1][0] &= 0x1f;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x9));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
        c.g[V0][0] &= c.g[T2][0];
        c.g[V1][0] &= 0x3e0;
        c.g[V0][0] |= c.g[V1][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_VT));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x238));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
        let draw = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, draw, 0x0039_96e0)?; // ctx->drawSprites(4, verts, 0)

        // Pop the render state, epilogue.
        let gp = c.g[GP][0] as u32;
        c.g[V1][0] = c.lw(gp.wrapping_sub(UI_ICON_RENDER_GRMAN));
        frame.restore(c, S0, 0x240);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP));
        frame.restore(c, S1, 0x230);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
        frame.restore(c, S2, 0x220);
        frame.restore_ra(c, 0x210);
        for (f, off) in [
            (28, 0x290),
            (27, 0x288),
            (26, 0x280),
            (25, 0x278),
            (24, 0x270),
            (23, 0x268),
            (22, 0x260),
            (21, 0x258),
            (20, 0x250),
        ] {
            frame.restore_fpr(c, f, off);
        }
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(UI_ICON_RENDER_CTX_TOP),
            c.g[V0][0],
        );
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cUIGroup_Render (0x398868)

/// `cUIGroup`'s child list (`cList` at +0x74).
pub(crate) const UI_GROUP_CHILDREN: u32 = 0x74;

pub(crate) const UI_LIST_FIRST: Callee = Callee {
    addr: 0x0039_7840,
    f: ee::s13::UIList_First,
};

pub(crate) const UI_LIST_NODE_IS_SENTINEL: Callee = Callee {
    addr: 0x0039_7718,
    f: ee::s13::UIListNode_IsSentinel,
};

/// `void cUIGroup::render2D(cUIGroup* this, float x, float y)` (0x398868):
/// render each visible child (flags `+0x14` bit 6) at the group's position
/// plus (x, y) through its vtable `+0x88`.
pub fn c_ui_group_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x10);
        frame.save_fpr(c, 21, 0x38);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(21, 12);
        frame.save(c, S0, 0x20);
        c.mov_s(20, 13);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(UI_GROUP_CHILDREN));
        UI_LIST_FIRST.call(c, 0x0039_8894)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = ui_vector_lw(c, c.g[S0][0], 0x14);
            loop {
                c.cycles += 4;
                let node = c.g[S0][0];
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32) & 1;
                if c.g[V0][0] != 0 {
                    c.cycles += 10;
                    let this = c.g[S1][0] as u32;
                    let vt = ui_vector_lw(c, node, 8);
                    c.g[V1][0] = vt;
                    c.lwc1(12, this + 0x44);
                    c.lwc1(13, this + 0x48);
                    let delta = Addr((vt as u32).wrapping_add(0x88)).lh(c);
                    c.add_s(12, 12, 21);
                    c.add_s(13, 13, 20);
                    let func = ui_vector_lw(c, vt, 0x8c);
                    c.g[V0][0] = func;
                    c.g[A0][0] = sx((node as u32).wrapping_add(delta as u32));
                    call_ptr(c, func as u32, 0x0039_88dc)?;
                }
                let node = c.g[S0][0];
                c.g[S0][0] = ui_vector_lw(c, node, 4);
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_88e8)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V0][0] = ui_vector_lw(c, c.g[S0][0], 0x14);
                if event_due(c) {
                    deliver_event_here(c, 0x0039_88a8);
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 6;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `reg + off` as a 32-bit guest address.
#[inline(always)]
pub(crate) fn oc391_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

// ---------------------------------------------------------------------------
// UIListBox_Render (0x399f00)

/// `UIListBox_GetModeEntry` (0x399920): the font of the current mode.
pub(crate) const UILB_GET_MODE_ENTRY: Callee = Callee {
    addr: 0x0039_9920,
    f: ee::s13::UIListBox_GetModeEntry,
};

/// `UIObject_ApplyAlignment` (0x39feb8).
pub(crate) const UILB_APPLY_ALIGNMENT: Callee = Callee {
    addr: 0x0039_feb8,
    f: ee::s13::UIObject_ApplyAlignment,
};

/// `Font_MeasureTextW` (0x3921f0, another shard: through the table).
pub(crate) const UILB_MEASURE_TEXT: u32 = 0x0039_21f0;

/// `Font_DrawTextW` (0x391e30, another shard: through the table).
pub(crate) const UILB_DRAW_TEXT: u32 = 0x0039_1e30;

/// The renderer global (`gp-0x854`): `+0xe84` its render state, `+0x10d8`
/// its vtable.
pub(crate) const UILB_RENDERER_GP: u32 = 0x854;

/// `ldl/ldr` 16 bytes at `src` into v0/v1 and `sdl/sdr` them to `dst`.
pub(crate) fn ui_list_box_render_copy16(c: &mut Cpu, src: u32, dst: u32) {
    c.ldl(2, src.wrapping_add(7));
    c.ldr(2, src);
    c.ldl(3, src.wrapping_add(0xf));
    c.ldr(3, src.wrapping_add(8));
    c.sdl(2, dst.wrapping_add(7));
    c.sdr(2, dst);
    c.sdl(3, dst.wrapping_add(0xf));
    c.sdr(3, dst.wrapping_add(8));
}

/// Set the render state's layer bits (`state+8`, bits 5..9) to the list
/// box's layer (`this+0x15 & 0x1f`) + 9. Leaves `v0`/`v1` as the original.
pub(crate) fn ui_list_box_render_set_layer(c: &mut Cpu, state_reg: usize) {
    let st = c.g[state_reg][0] as u32;
    c.g[V1][0] = Addr(st.wrapping_add(8)).lw(c);
    c.g[V0][0] &= 0x1f;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(9));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
    c.g[V1][0] &= c.g[S6][0];
    c.g[V0][0] &= 0x3e0;
    c.g[V1][0] |= c.g[V0][0];
    Addr(st.wrapping_add(8)).sw(c, c.g[V1][0]);
}

/// `void UIListBox_Render(UIListBox* self, float x, float y)` (0x399f00):
/// draw the selected string of a list box with its mode's font (scale,
/// colour, optional drop shadow), aligned at `pos + (x, y)`; with the debug
/// outline flag of the state data set, also draw the box outline (a red
/// line strip, or a dark red filled quad when the text overflows the box).
pub fn ui_list_box_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x1d0);
        frame.save(c, S0, 0x1b0);
        frame.save_fpr(c, 21, 0x1c8);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x1c0);
        c.mov_s(21, 13); // y
        frame.save(c, S1, 0x1a0);
        frame.save(c, S2, 0x190);
        frame.save(c, S3, 0x180);
        frame.save(c, S4, 0x170);
        frame.save(c, S5, 0x160);
        frame.save(c, S6, 0x150);
        frame.save_ra(c, 0x140);
        c.mov_s(20, 12); // x
        UILB_GET_MODE_ENTRY.call(c, 0x0039_9f3c)?;
        c.g[S1][0] = c.g[V0][0];
        if c.g[S1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(oc391_at(c, S0, 0x318)).lbu(c); // item count
            if c.g[V0][0] != 0 {
                ui_list_box_render_body(c)?;
            }
        }
        frame.restore(c, S0, 0x1b0);
        c.cycles += 11;
        frame.restore(c, S1, 0x1a0);
        frame.restore(c, S2, 0x190);
        frame.restore(c, S3, 0x180);
        frame.restore(c, S4, 0x170);
        frame.restore(c, S5, 0x160);
        frame.restore(c, S6, 0x150);
        frame.restore_ra(c, 0x140);
        frame.restore_fpr(c, 21, 0x1c8);
        frame.restore_fpr(c, 20, 0x1c0);
        frame.close(c);
        Ok(())
    })
}

/// The drawing part of [`ui_list_box_render`] (s0 = self, s1 = font).
pub(crate) fn ui_list_box_render_body(c: &mut Cpu) -> crate::call::Flow {
    let sp0 = sp(c);
    // Font scale times the list box's scale.
    c.cycles += 16;
    c.lwc1(3, oc391_at(c, S0, 0x50));
    c.lwc1(1, oc391_at(c, S0, 0x54));
    c.swc1(3, sp0);
    c.swc1(1, sp0 + 4);
    c.lwc1(0, oc391_at(c, S1, 0x30));
    c.lwc1(2, oc391_at(c, S1, 0x34));
    c.mul_s(0, 0, 3);
    c.swc1(0, oc391_at(c, S1, 0x38));
    c.lwc1(1, sp0 + 4);
    c.mul_s(2, 2, 1);
    c.swc1(2, oc391_at(c, S1, 0x3c));
    // Colour: the highlight colour when both highlight bits are set.
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x14)).lw(c);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32) & 1;
    let mut highlight = false;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = Addr(oc391_at(c, S0, 0x74)).lw(c);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32) & 1;
        highlight = c.g[V0][0] != 0;
    }
    if highlight {
        c.cycles += 10;
        ui_list_box_render_copy16(c, oc391_at(c, S0, 0x88), oc391_at(c, S1, 0x40));
    } else {
        c.cycles += 9;
        ui_list_box_render_copy16(c, oc391_at(c, S0, 0x1c), oc391_at(c, S1, 0x40));
    }
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x74)).lw(c);
    // Drop shadow.
    c.cycles += 4;
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32) & 1;
    if c.g[V0][0] == 0 {
        Addr(sp0 + 0x10).sw(c, 0);
        c.cycles += 6;
        Addr(sp0 + 0x14).sw(c, 0);
        c.ldl(2, sp0 + 0x17);
        c.ldr(2, sp0 + 0x10);
        c.sdl(2, oc391_at(c, S1, 0x2f));
        c.sdr(2, oc391_at(c, S1, 0x28));
    } else {
        c.cycles += 28;
        c.lwc1(0, oc391_at(c, S0, 0x1c));
        ui_list_box_render_copy16(c, oc391_at(c, S0, 0x78), sp0 + 0x10);
        c.swc1(0, sp0 + 0x10);
        c.g[AT][0] = 0x4000_0000; // 2.0f
        c.mtc1(1, 1);
        ui_list_box_render_copy16(c, sp0 + 0x10, oc391_at(c, S1, 0x50));
        c.swc1(1, sp0 + 0x24);
        c.swc1(1, sp0 + 0x20);
        c.ldl(2, sp0 + 0x27);
        c.ldr(2, sp0 + 0x20);
        c.sdl(2, oc391_at(c, S1, 0x2f));
        c.sdr(2, oc391_at(c, S1, 0x28));
    }
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x74)).lw(c);
    // The string: the first one, or the selected one (+0x319).
    c.cycles += 3;
    c.g[V0][0] &= 0x10;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = Addr(oc391_at(c, S0, 0x319)).lbu(c);
        c.cycles += 4;
        c.g[V1][0] = 0x14;
        c.mult(0, A0, V0, V1);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[S2][0] = Addr(oc391_at(c, V0, 0x9c)).lw(c);
    } else {
        c.cycles += 2;
        c.g[S2][0] = Addr(oc391_at(c, S0, 0x9c)).lw(c);
    }
    // Measure the string.
    c.cycles += 28;
    c.g[V1][0] = Addr(oc391_at(c, GP, 0u32.wrapping_sub(UILB_RENDERER_GP))).lw(c);
    c.g[S6][0] = 0xffff_ffff_ffff_fc1f;
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x15)).lbu(c);
    c.g[S3][0] = sx(sp0 + 0x10);
    c.g[T0][0] = Addr(oc391_at(c, V1, 0xe84)).lw(c);
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[A3][0] = 0;
    ui_list_box_render_set_layer(c, T0);
    c.g[S5][0] = 0;
    c.g[S4][0] = sx(oc391_at(c, S0, 0x60));
    c.lwc1(12, sp0);
    c.lwc1(13, sp0 + 4);
    call_ptr(c, UILB_MEASURE_TEXT, 0x0039_a108)?;
    c.lwc1(5, sp0 + 0x18);
    c.lwc1(0, oc391_at(c, S0, 0x60));
    c.c_lt_s(0, 5);
    if c.fcc() {
        c.g[S5][0] = 1; // the text is wider than the box
    }
    // Align and draw the text.
    c.cycles += 31;
    c.lwc1(2, oc391_at(c, S1, 0x14));
    c.cvt_s_w(2, 2);
    c.lwc1(4, oc391_at(c, S1, 0x34));
    c.lwc1(3, oc391_at(c, S0, 0x54));
    c.g[A1][0] = sx(sp0 + 0x40);
    c.lwc1(1, oc391_at(c, S0, 0x44));
    c.g[A0][0] = c.g[S0][0];
    c.mul_s(2, 2, 4);
    c.lwc1(0, oc391_at(c, S0, 0x48));
    c.add_s(1, 1, 20);
    c.swc1(5, sp0 + 0x30);
    c.add_s(0, 0, 21);
    Addr(sp0 + 0x38).sw(c, 0);
    c.mul_s(3, 3, 2);
    Addr(sp0 + 0x48).sw(c, 0);
    c.swc1(1, sp0 + 0x40);
    c.g[A2][0] = sx(sp0 + 0x30);
    c.swc1(0, sp0 + 0x44);
    c.swc1(3, sp0 + 0x34);
    UILB_APPLY_ALIGNMENT.call(c, 0x0039_a170)?;
    c.lwc1(12, sp0 + 0x40);
    c.g[A0][0] = c.g[S1][0];
    c.lwc1(13, sp0 + 0x44);
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, UILB_DRAW_TEXT, 0x0039_a184)?;
    // The debug outline: state data +0x10, bit 26 of its first word.
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x5c)).lw(c);
    c.g[V1][0] = Addr(oc391_at(c, V0, 0xd0)).lw(c);
    c.g[A0][0] = Addr(oc391_at(c, V1, 0x10)).lw(c);
    c.g[V0][0] = Addr(oc391_at(c, A0, 0x10)).lw(c);
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 5;
    c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32) & 1;
    c.g[A2][0] = c.g[S4][0];
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        return Ok(());
    }
    c.cycles += 32;
    c.ldl(5, oc391_at(c, S0, 0x4b));
    c.ldr(5, oc391_at(c, S0, 0x44));
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x4c)).lw(c);
    c.sdl(5, sp0 + 0x17);
    c.sdr(5, sp0 + 0x10);
    Addr(sp0 + 0x18).sw(c, c.g[V0][0]);
    c.g[A1][0] = c.g[S3][0];
    c.g[A0][0] = c.g[S0][0];
    c.lwc1(0, sp0 + 0x10);
    c.lwc1(1, sp0 + 0x14);
    c.add_s(0, 0, 20);
    c.add_s(1, 1, 21);
    c.swc1(0, sp0 + 0x10);
    c.swc1(1, sp0 + 0x14);
    UILB_APPLY_ALIGNMENT.call(c, 0x0039_a1ec)?;
    c.g[V1][0] = Addr(oc391_at(c, GP, 0u32.wrapping_sub(UILB_RENDERER_GP))).lw(c);
    c.g[A2][0] = sx(sp0 + 0x50);
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x15)).lbu(c);
    c.g[A3][0] = u64::MAX;
    c.g[A0][0] = Addr(oc391_at(c, V1, 0xe84)).lw(c);
    c.g[A1][0] = 4;
    ui_list_box_render_set_layer(c, A0);
    let over = c.g[S5][0] != 0;
    c.g[V0][0] = Addr(oc391_at(c, GP, 0u32.wrapping_sub(UILB_RENDERER_GP))).lw(c);
    let verts = c.g[A2][0] as u32;
    if !over {
        // Five red vertices: a closed line strip around the box.
        c.cycles += 6;
        c.g[AT][0] = 0x3f80_0000; // 1.0f
        c.mtc1(1, 3);
        c.g[A3][0] = 0xff;
        c.g[A1][0] = sx(sp0 + 0x70);
        c.g[A0][0] = c.g[A2][0];
        c.g[T0][0] = sx(verts.wrapping_add(0xf0));
        loop {
            ui_list_box_render_vertex(c, A0, A3, A3, A1);
            c.g[V1][0] = (c.g[A0][0] < c.g[T0][0]) as u64;
            if c.g[V1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_a248);
            }
        }
        c.cycles += 24;
        c.lwc1(5, oc391_at(c, S0, 0x64));
        c.g[A1][0] = 5;
        c.lwc1(4, oc391_at(c, S0, 0x60));
        c.g[A3][0] = 0;
        c.lwc1(3, sp0 + 0xa0);
        c.lwc1(1, sp0 + 0xd0);
        c.lwc1(2, sp0 + 0xd4);
        c.add_s(3, 3, 4);
        c.lwc1(0, sp0 + 0x104);
        c.add_s(1, 1, 4);
        c.add_s(2, 2, 5);
        c.g[T0][0] = Addr(oc391_at(c, GP, 0u32.wrapping_sub(UILB_RENDERER_GP))).lw(c);
        c.add_s(0, 0, 5);
        c.swc1(3, sp0 + 0xa0); // v1.x += w
        c.swc1(1, sp0 + 0xd0); // v2.x += w
        c.swc1(2, sp0 + 0xd4); // v2.y += h
        c.swc1(0, sp0 + 0x104); // v3.y += h
        ui_list_box_render_draw(c, 0x248, 0x0039_a2f0)?;
        return Ok(());
    }
    // Four dark red vertices: a filled quad over the box.
    c.cycles += 10;
    c.g[A0][0] = u64::MAX;
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 3);
    c.g[T1][0] = 0x40;
    c.g[V1][0] = Addr(oc391_at(c, V0, 0xe84)).lw(c);
    c.g[T0][0] = 0x80;
    c.g[A3][0] = sx(sp0 + 0x70);
    c.g[A1][0] = c.g[A2][0];
    Addr(oc391_at(c, V1, 0x10)).sh(c, c.g[A0][0]);
    c.g[A0][0] = sx(verts.wrapping_add(0xc0));
    loop {
        ui_list_box_render_vertex(c, A1, T1, T0, A3);
        c.g[V1][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
        if c.g[V1][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_a320);
        }
    }
    c.cycles += 22;
    c.lwc1(5, oc391_at(c, S0, 0x64));
    c.g[A1][0] = 4;
    c.lwc1(4, oc391_at(c, S0, 0x60));
    c.g[A3][0] = 0;
    c.lwc1(3, sp0 + 0xa0);
    c.lwc1(1, sp0 + 0xd4);
    c.lwc1(2, sp0 + 0x100);
    c.add_s(3, 3, 4);
    c.lwc1(0, sp0 + 0x104);
    c.add_s(1, 1, 5);
    c.add_s(2, 2, 4);
    c.g[T0][0] = Addr(oc391_at(c, GP, 0u32.wrapping_sub(UILB_RENDERER_GP))).lw(c);
    c.add_s(0, 0, 5);
    c.swc1(3, sp0 + 0xa0); // v1.x += w
    c.swc1(1, sp0 + 0xd4); // v2.y += h
    c.swc1(2, sp0 + 0x100); // v3.x += w
    c.swc1(0, sp0 + 0x104); // v3.y += h
    ui_list_box_render_draw(c, 0x238, 0x0039_a3c8)?;
    c.cycles += 1;
    Ok(())
}

/// One outline vertex at `reg vert` (0x30 bytes): q = 1 (f3), u = v = 0,
/// alpha from `alpha`, red from `red`, g = b = 0; its position (the aligned
/// box corner at sp+0x10, w = 1) goes through sp+0x20 to `pos` (sp+0x70 +
/// 0x30*i). Advances both pointers.
pub(crate) fn ui_list_box_render_vertex(
    c: &mut Cpu,
    vert: usize,
    alpha: usize,
    red: usize,
    pos: usize,
) {
    let sp0 = sp(c);
    c.cycles += 20;
    let v = c.g[vert][0] as u32;
    c.swc1(3, v + 8);
    Addr(v).sw(c, 0);
    Addr(v + 4).sw(c, 0);
    Addr(v + 0x1c).sw(c, c.g[alpha][0]);
    Addr(v + 0x10).sw(c, c.g[red][0]);
    Addr(v + 0x14).sw(c, 0);
    Addr(v + 0x18).sw(c, 0);
    c.swc1(3, sp0 + 0x2c);
    c.g[vert][0] = sx(v.wrapping_add(0x30));
    c.lwc1(2, sp0 + 0x10);
    c.lwc1(0, sp0 + 0x14);
    c.lwc1(1, sp0 + 0x18);
    c.swc1(2, sp0 + 0x20);
    c.swc1(0, sp0 + 0x24);
    c.swc1(1, sp0 + 0x28);
    c.lq(2, sp0 + 0x20);
    c.sq(2, c.g[pos][0] as u32);
    c.g[pos][0] = sx((c.g[pos][0] as u32).wrapping_add(0x30));
}

/// Call renderer method `slot` (`t0` = the renderer) with `a1` vertices at
/// `a2`, returning to `ret`.
pub(crate) fn ui_list_box_render_draw(c: &mut Cpu, slot: u32, ret: u32) -> crate::call::Flow {
    let vt = Addr(oc391_at(c, T0, 0x10d8)).lw(c);
    c.g[V0][0] = vt;
    c.g[A0][0] = Addr((vt as u32).wrapping_add(slot)).lh(c);
    c.g[V1][0] = Addr((vt as u32).wrapping_add(slot + 4)).lw(c);
    let func = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}

/// `lbu off(reg)`.
#[inline]
pub(crate) fn oc445_lbu(c: &mut Cpu, base: usize, off: u32) -> u64 {
    Addr((c.g[base][0] as u32).wrapping_add(off)).lbu(c)
}

// ---------------------------------------------------------------------------
// UIEngine_LoadTextures.

/// `void UIEngine_LoadTextures(UITextureBank *bank, int id, UITexData *data)`
/// (0x398138): load the screen's `<id>.ssh` pack (`FILE_loadpack`), upload
/// every shape through the UI engine (vtable slot 0x170) into a fresh id
/// array (`bank+4`, `count` at `bank+8`), then build the bank's 32-byte
/// element records (`bank+0xc`, `count` at `bank+0x10`) from `data` (a
/// count and 24-byte source records that name a texture index).
///
/// `FILE_loadpack` reaches `EAFile_CallWithPriority`,
/// which a fixture cannot bound, so only the prologue up to the first call
/// is differentially tested. Transcribed from the generated body.
pub fn ui_engine_load_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0xe0);
        c.g[V0][0] = 0x490000;
        frame.save(c, 20, 0x90);
        frame.save(c, 17, 0xc0);
        c.g[S4][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, 16, 0xd0);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, 19, 0xa0);
        c.g[A1][0] = oc445_add(c, V0, 0x3d90);
        frame.save(c, 18, 0xb0);
        frame.save(c, 21, 0x80);
        frame.save_ra(c, 0x70);
        c.g[S0][0] = 0;
        // sprintf(name, "%s.ssh"-style format, id)
        call_ptr(c, 0x0041_7828, 0x0039_8178)?;
        c.g[A1][0] = 0x3000000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] |= 0x100;
        // FILE_loadpack(name, 0x3000100)
        call_ptr(c, 0x003e_22f0, 0x0039_8188)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A1][0] = 0x490000;
        c.g[V0][0] = oc445_lw(c, S3, 0x8);
        c.g[A2][0] = 0;
        c.g[A1][0] = oc445_add(c, A1, 0x3da0);
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
        Addr(oc445_add(c, S1, 0x8) as u32).sw(c, c.g[V0][0]);
        // operator new(count * 4, "UITextureIDs", 0, 0)
        call_ptr(c, 0x0031_7e30, 0x0039_81ac)?;
        c.g[A2][0] = oc445_lw(c, S1, 0x8);
        c.g[A0][0] = c.g[V0][0];
        Addr(oc445_add(c, S1, 0x4) as u32).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0xff;
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        // memset(ids, 0xff, count * 4)
        call_ptr(c, 0x0041_6210, 0x0039_81c4)?;
        c.g[V1][0] = oc445_lw(c, S1, 0x8);
        let no_shapes = (c.g[V1][0] as i64) <= 0;
        c.g[S2][0] = oc445_add(c, S3, 0x14);
        if !no_shapes {
            c.cycles += 2;
            c.g[S5][0] = 0x4a0000;
            loop {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                c.g[A1][0] = oc445_addu(c, S3, V0);
                let skip = c.g[A1][0] == 0;
                c.g[T2][0] = oc445_lw(c, GP, 0u32.wrapping_sub(0x854));
                if !skip {
                    c.cycles += 13;
                    c.g[A2][0] = oc445_add(c, S5, 0x4778);
                    c.g[A3][0] = 0;
                    c.g[T0][0] = 0x1;
                    c.g[V1][0] = oc445_lw(c, T2, 0x10d8);
                    c.g[T1][0] = u64::MAX;
                    // UIEngine vtable slot 0x170: load one shape into a page.
                    oc445_vcall(c, V1, T2, 0x170, V0, 0x0039_820c)?;
                    c.g[A0][0] = oc445_lw(c, S1, 0x4);
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V1][0] = oc445_addu(c, V1, A0);
                    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = oc445_lw(c, S1, 0x8);
                c.g[S0][0] = oc445_add(c, S0, 0x1);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = oc445_add(c, S2, 0x8);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_81d8);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 16;
        c.g[A0][0] = c.g[S3][0];
        c.g[S2][0] = oc445_add(c, S4, 0x4);
        // MEM_free(fsh)
        call_ptr(c, 0x0025_23a8, 0x0039_823c)?;
        c.g[S3][0] = oc445_add(c, S1, 0xc);
        Addr(c.g[S1][0] as u32).sw(c, c.g[S4][0]);
        c.g[A1][0] = 0x490000;
        c.g[A1][0] = oc445_add(c, A1, 0x3db0);
        c.g[A2][0] = 0;
        c.g[S0][0] = Addr(c.g[S4][0] as u32).lw(c);
        c.g[A3][0] = 0;
        Addr(oc445_add(c, S1, 0x10) as u32).sw(c, c.g[S0][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32) << 5);
        // operator new(count * 32, "UITextureELs", 0, 0)
        call_ptr(c, 0x0031_7e30, 0x0039_8264)?;
        c.g[A0][0] = oc445_add(c, S0, 0xffff_ffff);
        let no_elems = c.g[S0][0] == 0;
        c.g[V1][0] = c.g[V0][0];
        if !no_elems {
            c.cycles += 4;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.g[A1][0] = u64::MAX;
            // Initialise every record: id -1, 1.0 scales, zero offsets.
            loop {
                c.cycles += 10;
                Addr(c.g[V1][0] as u32).sw(c, c.g[A1][0]);
                c.g[A0][0] = oc445_add(c, A0, 0xffff_ffff);
                c.swc1(0, oc445_add(c, V1, 0x4) as u32);
                c.swc1(0, oc445_add(c, V1, 0x8) as u32);
                Addr(oc445_add(c, V1, 0xc) as u32).sw(c, 0);
                Addr(oc445_add(c, V1, 0x10) as u32).sw(c, 0);
                c.swc1(0, oc445_add(c, V1, 0x14) as u32);
                c.swc1(0, oc445_add(c, V1, 0x18) as u32);
                let more = c.g[A0][0] != c.g[A1][0];
                c.g[V1][0] = oc445_add(c, V1, 0x20);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_8280);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 9;
        Addr(c.g[S3][0] as u32).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0xff;
        c.g[A2][0] = oc445_lw(c, S1, 0x10);
        c.g[A0][0] = oc445_lw(c, S1, 0xc);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        // memset(elems, 0xff, count * 4)
        call_ptr(c, 0x0041_6210, 0x0039_82c0)?;
        c.g[V1][0] = oc445_lw(c, S1, 0x10);
        let no_copy = (c.g[V1][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !no_copy {
            c.cycles += 1;
            loop {
                c.cycles += 34;
                c.g[V1][0] = oc445_lw(c, S1, 0xc);
                c.g[A1][0] = sx((c.g[A3][0] as u32) << 5);
                c.lwc1(0, oc445_add(c, S2, 0xc) as u32);
                c.g[A3][0] = oc445_add(c, A3, 0x1);
                c.g[V1][0] = oc445_addu(c, A1, V1);
                c.swc1(0, oc445_add(c, V1, 0xc) as u32);
                c.g[V0][0] = oc445_lw(c, S1, 0xc);
                c.lwc1(0, oc445_add(c, S2, 0x8) as u32);
                c.g[V0][0] = oc445_addu(c, A1, V0);
                c.swc1(0, oc445_add(c, V0, 0x10) as u32);
                c.g[V1][0] = oc445_lw(c, S1, 0xc);
                c.lwc1(0, oc445_add(c, S2, 0x14) as u32);
                c.g[V1][0] = oc445_addu(c, A1, V1);
                c.swc1(0, oc445_add(c, V1, 0x18) as u32);
                c.g[V0][0] = oc445_lw(c, S1, 0xc);
                c.lwc1(0, oc445_add(c, S2, 0x10) as u32);
                c.g[V0][0] = oc445_addu(c, A1, V0);
                c.swc1(0, oc445_add(c, V0, 0x14) as u32);
                c.g[V1][0] = oc445_lw(c, S1, 0xc);
                c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
                c.g[V1][0] = oc445_addu(c, A1, V1);
                Addr(oc445_add(c, V1, 0x1c) as u32).sw(c, c.g[A0][0]);
                c.g[V0][0] = oc445_lbu(c, S2, 0x5);
                c.g[A2][0] = oc445_lw(c, S1, 0x4);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = oc445_lw(c, S1, 0xc);
                c.g[V0][0] = oc445_addu(c, V0, A2);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A1][0] = oc445_addu(c, A1, A0);
                Addr(c.g[A1][0] as u32).sw(c, c.g[V1][0]);
                c.g[V0][0] = oc445_lw(c, S1, 0x10);
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = oc445_add(c, S2, 0x18);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_82d0);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 9;
        frame.restore(c, 16, 0xd0);
        frame.restore(c, 17, 0xc0);
        frame.restore(c, 18, 0xb0);
        frame.restore(c, 19, 0xa0);
        frame.restore(c, 20, 0x90);
        frame.restore(c, 21, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `EA_MemAlloc(size, name)`: the named pool allocator (0x317d70).
pub(crate) const WPQ_ALLOC: u32 = 0x0031_7d70;

/// `EA_MemAlloc` for the 0xd4-byte screen and its blocks (0x317d70).
pub(crate) const UI_SCREEN_ALLOC: u32 = 0x0031_7d70;

/// `tAttrScreen_tAttrScreen` (0x1dd580) on the new screen's attribute block.
pub(crate) const UI_TATTR_CTOR: u32 = 0x001d_d580;

/// `cList_cList` (0x3977e8), the screen's lists.
pub(crate) const UI_LIST_CTOR: u32 = 0x0039_77e8;

/// `UIObject_ctor` (0x39fb30) on the screen's base object.
pub(crate) const UI_OBJECT_CTOR: u32 = 0x0039_fb30;

/// `cUIScreen_setData` (0x39c688).
pub(crate) const UI_SCREEN_SET_DATA: u32 = 0x0039_c688;

/// `cUIState_addScreen` (0x39e758).
pub(crate) const UI_STATE_ADD_SCREEN: u32 = 0x0039_e758;

/// `cUIScreen_createAllObjects` (0x39ced0).
pub(crate) const UI_SCREEN_CREATE_OBJECTS: u32 = 0x0039_ced0;

/// `void cUIEngine_addScreenByHashName(void* table, cUIState* state,
/// u32 hash, cUIScreen* screen)` (0x397cc0): looks the screen's hash up in
/// the table at `table + 4` (a word count, then 8-byte `{key, offset}`
/// records, with the offset relative to the table's base). If the caller
/// gave no screen, a new 0xd4-byte `cUIScreen` is allocated and built (its
/// attribute block, two lists and its base object). The screen's data is set
/// from the record, the screen is added to the state, and its objects are
/// created. Returns the screen in `v0`.
pub fn ui_engine_create_screen(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);

        // The table pointer and the table base (A1 = T0 + offset).
        c.g[T0][0] = Addr(c.g[A0][0] as u32).off(4).lw(c);
        c.g[A0][0] = 0;
        c.g[V1][0] = Addr(c.g[T0][0] as u32).off(0xc).lw(c);
        c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));

        // Search the records for the key in A2. A0 is the record found, or 0.
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(c.g[A1][0] as u32).off(4).lw(c);
            loop {
                c.cycles += 2;
                if c.g[V0][0] == c.g[A2][0] {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[V1][0];
                    break;
                }
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0039_7d08);
                }
                c.cycles += 4;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[A3][0] < c.g[V0][0]) as u64;
                let exhausted = c.g[V0][0] == 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(8));
                if exhausted {
                    break;
                }
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            }
        }

        // The record's offset gives the key's value: S2 = base + offset.
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(4).lw(c);
        let has_screen = c.g[S1][0] != 0;
        c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if !has_screen {
            create_ui_screen(c)?;
        }

        // 0x397db4: set the data, add the screen to the state, create its objects.
        c.cycles += 16;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: UI_SCREEN_SET_DATA,
            f: ee::s13::f_0039c688,
        }
        .call(c, 0x0039_7dc0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S3][0];
        Callee {
            addr: UI_STATE_ADD_SCREEN,
            f: ee::s13::f_0039e758,
        }
        .call(c, 0x0039_7dcc)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: UI_SCREEN_CREATE_OBJECTS,
            f: ee::s13::f_0039ced0,
        }
        .call(c, 0x0039_7dd4)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The new-screen branch of `ui_engine_create_screen` (0x397d34): allocates
/// the 0xd4-byte screen, builds it, and leaves it in `s1`.
pub(crate) fn create_ui_screen(c: &mut Cpu) -> Flow {
    c.cycles += 32;
    c.g[A1][0] = sx(0x0049_3d80);
    c.g[A3][0] = 0;
    c.g[A2][0] = 0x100;
    c.g[A0][0] = 0xd4;
    call_ptr(c, UI_SCREEN_ALLOC, 0x0039_7d4c)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = sx(0x0049_4670);
    let s0 = c.g[S0][0] as u32;
    c.g[A0][0] = sx(s0.wrapping_add(0x14));
    let vtable = c.g[V1][0];
    Addr(s0.wrapping_add(0x8)).sw(c, vtable);
    c.g[S1][0] = c.g[S0][0];
    let this = c.g[S0][0];
    Addr(s0.wrapping_add(0x4)).sw(c, this);
    Addr(s0).sw(c, this);
    Addr(s0.wrapping_add(0xc)).sw(c, 0);
    Addr(s0.wrapping_add(0x10)).sw(c, 0);
    call_ptr(c, UI_TATTR_CTOR, 0x0039_7d78)?;
    let s0 = c.g[S0][0] as u32;
    c.g[A0][0] = sx(s0.wrapping_add(0x18));
    Callee {
        addr: UI_LIST_CTOR,
        f: ee::s13::f_003977e8,
    }
    .call(c, 0x0039_7d80)?;
    let s0 = c.g[S0][0] as u32;
    c.g[A0][0] = sx(s0.wrapping_add(0x40));
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    Addr(s0.wrapping_add(0x34)).sw(c, 0);
    Addr(s0.wrapping_add(0x38)).sw(c, 0);
    Addr(s0.wrapping_add(0x3c)).sw(c, 0);
    Callee {
        addr: UI_OBJECT_CTOR,
        f: ee::s13::f_0039fb30,
    }
    .call(c, 0x0039_7d9c)?;
    let s0 = c.g[S0][0] as u32;
    c.g[V1][0] = sx(0x0049_49e8);
    c.g[A0][0] = sx(s0.wrapping_add(0xb4));
    let item_vtable = c.g[V1][0];
    Addr(s0.wrapping_add(0x48)).sw(c, item_vtable);
    Callee {
        addr: UI_LIST_CTOR,
        f: ee::s13::f_003977e8,
    }
    .call(c, 0x0039_7db0)?;
    let s0 = c.g[S0][0] as u32;
    let s3 = c.g[S3][0];
    Addr(s0.wrapping_add(0xd0)).sw(c, s3);
    Ok(())
}

/// `BoltFile_FreeSlot` (0x19d4b8).
pub(crate) const BOLT_FREE_SLOT: u32 = 0x0019_d4b8;

/// `cUIListBox::USTR_length` (0x3dbca0).
pub(crate) const LIST_STRING_LENGTH: u32 = 0x003d_bca0;

/// `USTR_copy` (0x3dbe20).
pub(crate) const LIST_STRING_COPY: u32 = 0x003d_be20;

/// `EA_MemAlloc` for the item text (0x317e30).
pub(crate) const LIST_TEXT_ALLOC: u32 = 0x0031_7e30;

/// `UIObject::Load` (0x39fe00).
pub(crate) const OC752_UI_OBJECT_LOAD: Callee = Callee {
    addr: 0x0039_fe00,
    f: ee::s13::f_0039fe00,
};

/// `void cUIGroup_Load(cUIGroup* self)` (0x398618): a forwarding wrapper,
/// `UIObject_Load(self)` (7 cycles, a 0x10 frame holding `ra`).
pub fn ui_group_load(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(OC752_UI_OBJECT_LOAD), 0x0039_8628)
}

// ---------------------------------------------------------------------------
// UIListBox_ctor (0x399768)

/// `void UIListBox_ctor(UIListBox* self)` (0x399768): base `UIObject_ctor`,
/// vtable, attribute bits, 32 cleared 0x14-byte rows at +0x98, entry counters
/// cleared, the row area `memset` (0x280 bytes) and the static colour copied
/// from the global at 0x4c66c8 to +0x88.
pub fn uilistbox_ctor_w244(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0039_977c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_fb30, 0x0039_977c)?;
        c.g[V0][0] = c.lbu(at759(c.g[S0][0], 0x74));
        c.g[V1][0] = 0x490000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4798));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = c.g[V0][0] | 0x4;
        c.sw(at759(c.g[S0][0], 0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] & 0xc4;
        c.g[A0][0] = sx(at759(c.g[S0][0], 0x98));
        c.sb(at759(c.g[S0][0], 0x74), c.g[V0][0]);
        c.g[V0][0] = 0x1f;
        loop {
            c.cycles += 8;
            c.sw(c.g[A0][0] as u32, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.sw(at759(c.g[A0][0], 0x4), 0);
            c.sw(at759(c.g[A0][0], 0x8), 0);
            c.sw(at759(c.g[A0][0], 0xc), 0);
            c.sb(at759(c.g[A0][0], 0x10), 0);
            let more = c.g[V0][0] != c.g[A1][0];
            c.g[A0][0] = sx(at759(c.g[A0][0], 0x14));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_97a8);
            }
        }
        c.cycles += 21;
        c.sb(at759(c.g[S0][0], 0x318), 0);
        c.g[A0][0] = sx(at759(c.g[S0][0], 0x98));
        c.sb(at759(c.g[S0][0], 0x319), 0);
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0039_97e0);
        c.g[A2][0] = 0x280;
        call_ptr(c, 0x0041_6210, 0x0039_97e0)?;
        c.g[V1][0] = 0x4c0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66c8));
        c.ldl(4, at759(c.g[A3][0], 0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.ldl(5, at759(c.g[A3][0], 0xf));
        c.ldr(5, at759(c.g[A3][0], 0x8));
        c.sdl(4, at759(c.g[S0][0], 0x8f));
        c.sdr(4, at759(c.g[S0][0], 0x88));
        c.sdl(5, at759(c.g[S0][0], 0x97));
        c.sdr(5, at759(c.g[S0][0], 0x90));
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UI_AsciiToWide(wide, ascii)` (0x00397ad8): widen a NUL-terminated byte string
/// into 16-bit units (sign-extending each byte), then store the terminator.
/// The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ui_ascii_to_wide(c: &mut Cpu) {
    if c.pc != 0x00397ad8 {
        return ui_ascii_to_wide_resume(c);
    }
    guest(c, |c| {
        c.cycles += 2;
        c.g[V0][0] = c.lb(c.g[A1][0] as u32);
        loop {
            // loop test (0x00397af8); its delay slot loads the next unsigned byte
            c.cycles += 2;
            let t = c.g[V0][0] != 0u64;
            c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
            if !t {
                break;
            }
            if back_edge_unwinds(c, 0x00397ae0) {
                return Ok(());
            }
            // body (0x00397ae0): widen the byte, advance both pointers
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.sh(c.g[A0][0] as u32, c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
            c.g[V0][0] = c.lb(c.g[A1][0] as u32);
        }
        // terminator (0x00397b00)
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.sh(c.g[A0][0] as u32, 0u64);
        c.pc = t;
        Ok(())
    });
}

/// Native resume of `ui_ascii_to_wide` (converted from the `generated` body): any entry
/// other than 0x00397ad8, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn ui_ascii_to_wide_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00397ad8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                    if t {
                        pc = 0x00397af8;
                        continue;
                    }
                    pc = 0x00397ae0;
                    continue;
                }
                0x00397ae0 => {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.sh(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
                    pc = 0x00397af8;
                    continue;
                }
                0x00397af8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x00397ae0) {
                            return Ok(());
                        }
                        pc = 0x00397ae0;
                        continue;
                    }
                    pc = 0x00397b00;
                    continue;
                }
                0x00397b00 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.sh(c.g[A0][0] as u32, 0u64);
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00397ad8, 0x00397b08);
                    return Ok(());
                }
            }
        }
    });
}

/// `UIFontSet_LoadFonts(self)` (0x00398568): for each of the signed byte count
/// (`+0x34`) font entries (12 bytes from `+4`, name pointer at `+4` of the entry)
/// create a font object (`EA_MemAllocNamed`, 0x84 bytes), construct it from the
/// name (`Font_ctor`) and store it at `entry + 0xc`.
/// The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ui_font_set_load_fonts_811(c: &mut Cpu) {
    // no resume points: the generated body ignores `c.pc` and always starts at the
    // entry, so every pc runs the body below (the guard is not needed).
    guest(c, |c| {
        'b_3985f4: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x34));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S1][0] = 0u64;
            if t {
                break 'b_3985f4;
            } else {
                c.cycles += 4;
                c.g[S5][0] = 0x4a0000;
                c.g[S4][0] = 0xc;
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = sx((c.g[S1][0] as u32) << 24);
                'l_3985a8: loop {
                    c.cycles += 19;
                    c.g[A0][0] = 0x84;
                    c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4780));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x003985c0);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x003985c0)?;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x003985d8);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x003915e8, 0x003985d8)?;
                    c.mult(0, 3, 16, 20);
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.g[V0][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S1][0] as u32) << 24);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003985a8);
                        }
                        continue 'l_3985a8;
                    } else {
                        break 'b_3985f4;
                    }
                }
            }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `bool UIListBox_HandleInput(UIListBox* self, Input* in)` (0x399970): the
/// list box's navigation. With no entries (byte +0x318) it returns 0. Otherwise
/// the input object's vtable (slots +0x88/+0x90/+0x98/+0xa0, +0x30 for the
/// accept key) is asked whether up (19 / 18), down (20 / 17) or the accept
/// key is pressed; the current entry (byte +0x319) moves (wrapping when the
/// attribute word +0x14 has bit 7), the screen object gets focus-notify (9 /
/// 6) and focus-move calls, a key-frame thread is started when the move
/// result has bit 8, and the UI sound listener is told the result through
/// its vtable (+0x14). Returns 1 when the input was consumed, else 0.
pub fn ui_list_box_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_399d60: {
            'b_399d5c: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
                let t = c.g[V0][0] == 0u64;
                c.g[S1][0] = c.g[A1][0];
                if t {
                    break 'b_399d5c;
                } else {
                    'b_399d38: {
                        'b_399d2c: {
                            'b_399d20: {
                                'b_399b18: {
                                    'b_399a10: {
                                        'b_3999e4: {
                                            c.cycles += 13;
                                            c.g[S2][0] =
                                                sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                                            c.g[RA][0] = sx(0x003999a4);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x0039ff50, 0x003999a4)?; // UIObject_GetStateData
                                            c.g[S3][0] = 0u64;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                                            if c.g[A1][0] != 0 {
                                                c.g[S2][0] = c.g[A1][0];
                                            }
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x003999c4);
                                            c.g[A0][0] =
                                                sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x003999c4)?;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_3999e4;
                                            } else {
                                                c.cycles += 5;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                if c.g[V0][0] != 0u64 {
                                                    c.g[V0][0] = c.lbu(
                                                        (c.g[S0][0] as u32).wrapping_add(0x319),
                                                    );
                                                    break 'b_399a10;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    break 'b_3999e4;
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x003999f4);
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x003999f4)?;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            break 'b_399b18;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_399b18;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    c.lbu((c.g[S0][0] as u32).wrapping_add(0x319));
                                                break 'b_399a10;
                                            }
                                        }
                                    }
                                    'b_399a8c: {
                                        'b_399a48: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                            if t {
                                                break 'b_399a48;
                                            } else {
                                                c.cycles += 12;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                c.sb(
                                                    (c.g[S0][0] as u32).wrapping_add(0x319),
                                                    c.g[V0][0],
                                                );
                                                c.g[A1][0] = c.g[S0][0];
                                                c.g[A2][0] = 0x9;
                                                c.g[A3][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                                                c.g[V0][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00399a40);
                                                c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00399a40)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                if t {
                                                    break 'b_399a8c;
                                                } else {
                                                    break 'b_399a48;
                                                }
                                            }
                                        }
                                        'b_399a88: {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            c.g[V0][0] =
                                                sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = c.g[S0][0];
                                            if t {
                                                break 'b_399a88;
                                            } else {
                                                c.cycles += 11;
                                                c.g[V0][0] =
                                                    c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                c.g[A2][0] = 0x9;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                c.sb(
                                                    (c.g[S0][0] as u32).wrapping_add(0x319),
                                                    c.g[V0][0],
                                                );
                                                c.g[A3][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                                                c.g[V0][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00399a88);
                                                c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00399a88)?;
                                                break 'b_399a88;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        break 'b_399a8c;
                                    }
                                    'b_399ab0: {
                                        'b_399aa8: {
                                            c.cycles += 4;
                                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = c.g[S0][0];
                                            if t {
                                                break 'b_399aa8;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                let t = true;
                                                c.g[A2][0] = 0u64;
                                                if t {
                                                    break 'b_399ab0;
                                                } else {
                                                    break 'b_399aa8;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                        c.g[A2][0] = 0x3;
                                        break 'b_399ab0;
                                    }
                                    'b_399b08: {
                                        'b_399b04: {
                                            c.cycles += 10;
                                            c.g[A3][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                                            c.g[V1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0xb0));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xb4));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x00399ac8);
                                            c.g[A0][0] =
                                                sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00399ac8)?;
                                            c.g[S1][0] = c.g[V0][0];
                                            c.g[V0][0] = c.g[S1][0] & 0x100;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x8;
                                            if t {
                                                break 'b_399b04;
                                            } else {
                                                c.cycles += 6;
                                                c.g[RA][0] = sx(0x00399ae0);
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, 0x0039ff78, 0x00399ae0)?; // UIObject_GetEventThread
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[V0][0] = 0xffff;
                                                let t = c.g[A1][0] == c.g[V0][0];
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                if t {
                                                    break 'b_399b08;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                    let t = c.g[A0][0] == 0u64;

                                                    if t {
                                                        break 'b_399b08;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[RA][0] = sx(0x00399b04);
                                                        c.g[A2][0] = 0x1;
                                                        call_ptr(c, 0x0039c870, 0x00399b04)?; // UIScreen_StartThread
                                                        break 'b_399b04;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.g[S1][0] & 0x1;
                                        break 'b_399b08;
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.g[S1][0] & 0x10;
                                    if t {
                                        break 'b_399d20;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[S3][0] = 0x1;
                                        if t {
                                            break 'b_399d38;
                                        } else {
                                            break 'b_399b18;
                                        }
                                    }
                                }
                                'b_399c78: {
                                    'b_399b74: {
                                        'b_399b48: {
                                            c.cycles += 6;
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x00399b28);
                                            c.g[A0][0] =
                                                sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00399b28)?;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_399b48;
                                            } else {
                                                c.cycles += 5;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                if c.g[V0][0] != 0u64 {
                                                    c.g[V1][0] = c.lbu(
                                                        (c.g[S0][0] as u32).wrapping_add(0x319),
                                                    );
                                                    break 'b_399b74;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    break 'b_399b48;
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x00399b58);
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x00399b58)?;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            break 'b_399c78;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_399c78;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V1][0] =
                                                    c.lbu((c.g[S0][0] as u32).wrapping_add(0x319));
                                                break 'b_399b74;
                                            }
                                        }
                                    }
                                    'b_399bec: {
                                        'b_399bb4: {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                            c.g[V0][0] =
                                                ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = c.g[S0][0];
                                            if t {
                                                break 'b_399bb4;
                                            } else {
                                                c.cycles += 11;
                                                c.sb(
                                                    (c.g[S0][0] as u32).wrapping_add(0x319),
                                                    c.g[V1][0],
                                                );
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                c.g[A2][0] = 0x9;
                                                c.g[A3][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                                                c.g[V0][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00399bac);
                                                c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00399bac)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                                if t {
                                                    break 'b_399bec;
                                                } else {
                                                    break 'b_399bb4;
                                                }
                                            }
                                        }
                                        'b_399be8: {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            c.g[V0][0] =
                                                sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A2][0] = 0x9;
                                            if t {
                                                break 'b_399be8;
                                            } else {
                                                c.cycles += 8;
                                                c.sb((c.g[S0][0] as u32).wrapping_add(0x319), 0u64);
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                c.g[A3][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                                                c.g[V0][0] =
                                                    c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00399be8);
                                                c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00399be8)?;
                                                break 'b_399be8;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        break 'b_399bec;
                                    }
                                    'b_399c10: {
                                        'b_399c08: {
                                            c.cycles += 4;
                                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = c.g[S0][0];
                                            if t {
                                                break 'b_399c08;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                let t = true;
                                                c.g[A2][0] = 0x1;
                                                if t {
                                                    break 'b_399c10;
                                                } else {
                                                    break 'b_399c08;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                        c.g[A2][0] = 0x2;
                                        break 'b_399c10;
                                    }
                                    'b_399c68: {
                                        'b_399c64: {
                                            c.cycles += 10;
                                            c.g[A3][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                                            c.g[V1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0xb0));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xb4));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x00399c28);
                                            c.g[A0][0] =
                                                sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00399c28)?;
                                            c.g[S1][0] = c.g[V0][0];
                                            c.g[V0][0] = c.g[S1][0] & 0x100;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x9;
                                            if t {
                                                break 'b_399c64;
                                            } else {
                                                c.cycles += 6;
                                                c.g[RA][0] = sx(0x00399c40);
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, 0x0039ff78, 0x00399c40)?; // UIObject_GetEventThread
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[V0][0] = 0xffff;
                                                let t = c.g[A1][0] == c.g[V0][0];
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                if t {
                                                    break 'b_399c68;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                    let t = c.g[A0][0] == 0u64;

                                                    if t {
                                                        break 'b_399c68;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[RA][0] = sx(0x00399c64);
                                                        c.g[A2][0] = 0x1;
                                                        call_ptr(c, 0x0039c870, 0x00399c64)?; // UIScreen_StartThread
                                                        break 'b_399c64;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.g[S1][0] & 0x1;
                                        break 'b_399c68;
                                    }
                                    c.cycles += 2;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[S3][0] = 0x1;
                                        break 'b_399d38;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = c.g[S1][0] & 0x10;
                                        if t {
                                            break 'b_399d20;
                                        } else {
                                            break 'b_399c78;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x00399c88);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00399c88)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = c.g[S0][0];
                                if t {
                                    break 'b_399d5c;
                                } else {
                                    'b_399d0c: {
                                        'b_399d08: {
                                            c.cycles += 21;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                            c.g[A2][0] = 0x6;
                                            c.g[A3][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                                            c.g[V0][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00399cb0);
                                            c.g[A0][0] =
                                                sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00399cb0)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[A2][0] = 0x4;
                                            c.g[A3][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                                            c.g[V1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0xb0));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xb4));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x00399cd4);
                                            c.g[A0][0] =
                                                sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00399cd4)?;
                                            c.g[S1][0] = c.g[V0][0];
                                            c.g[V0][0] = c.g[S1][0] & 0x100;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x3;
                                            if t {
                                                break 'b_399d08;
                                            } else {
                                                c.cycles += 6;
                                                c.g[RA][0] = sx(0x00399cec);
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, 0x0039ff78, 0x00399cec)?; // UIObject_GetEventThread
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[V0][0] = 0xffff;
                                                let t = c.g[A1][0] == c.g[V0][0];
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                if t {
                                                    break 'b_399d0c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                                    c.g[RA][0] = sx(0x00399d08);
                                                    c.g[A2][0] = 0x1;
                                                    call_ptr(c, 0x0039c870, 0x00399d08)?; // UIScreen_StartThread
                                                    break 'b_399d08;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.g[S1][0] & 0x1;
                                        break 'b_399d0c;
                                    }
                                    'b_399d1c: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = c.g[S0][0];
                                        if t {
                                            break 'b_399d1c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_399d2c;
                                            } else {
                                                break 'b_399d1c;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.g[S1][0] & 0x10;
                                    break 'b_399d20;
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_399d38;
                            } else {
                                c.cycles += 1;
                                c.g[A1][0] = 0u64;
                                break 'b_399d2c;
                            }
                        }
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x00399d34);
                        call_ptr(c, 0x003a0318, 0x00399d34)?; // UIObject_GetUserData
                        c.g[S3][0] = c.g[V0][0];
                        break 'b_399d38;
                    }
                    'b_399d54: {
                        c.cycles += 2;
                        let t = c.g[S2][0] == 0u64;
                        c.g[A1][0] = c.g[S3][0];
                        if t {
                            break 'b_399d54;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00399d54);
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00399d54)?;
                            break 'b_399d54;
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_399d60;
                    } else {
                        break 'b_399d5c;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_399d60;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    })
}

/// `void UIIcon_SwapWords(u32* a, u32* b)` (0x398a60): exchange the two
/// words (the values go through the FPU registers f0/f1, as compiled).
pub fn ui_icon_swap_words(c: &mut Cpu) {
    c.cycles += 5;
    let a = Addr(abi::arg(c, 0));
    let b = Addr(abi::arg(c, 1));
    c.f[0] = b.read_u32(c);
    c.f[1] = a.read_u32(c);
    let (v0, v1) = (c.f[0], c.f[1]);
    a.write_u32(c, v0);
    b.write_u32(c, v1);
    abi::ret(c);
}

/// `void UIFontSet_ParseFontPack(UIFontSet* self, u32* pack)` (0x3984e8):
/// keep the pack pointer in `*self`, then add each font entry of the pack
/// (`pack[0]` entries, each `8 + step` bytes apart) through
/// `UIFontSet_AddFont` (0x3984b0).
pub fn ui_font_set_parse_font_pack(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
        frame.save_ra(c, 0);
        let set = c.g[S2][0] as u32;
        let pack = c.g[A1][0];
        Addr(set).sw(c, pack);
        c.g[V0][0] = Addr(pack as u32).lw(c);
        c.g[S1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            loop {
                c.cycles += 13;
                let entry = c.g[S0][0] as u32;
                c.g[A3][0] = Addr(entry).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = sx(entry.wrapping_add(0x8));
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039_84b0, 0x0039_852c)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = Addr(set).lw(c);
                c.g[V1][0] = Addr(entry.wrapping_add(0x4)).lw(c);
                let first = c.g[V0][0] as u32;
                c.g[A0][0] = Addr(first).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = (c.g[S1][0] < c.g[A0][0]) as u64;
                let more = c.g[A0][0] != 0;
                c.g[S0][0] = sx(entry.wrapping_add(c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_8518);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    pub(crate) use crate::game::sound::soundsys::tests::oc752_install_fakes;

    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::rider::tests::clear_2;

    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::be::bedbprofile::tests::strict as strict_2;
    pub(crate) use crate::game::be::beintplayer::tests::base_10 as base_7;

    pub(crate) use crate::game::be::beintplayer::tests::strict as strict_3;
    pub(crate) use crate::game::be::beintplayer::tests::w as w_9;
    pub(crate) use crate::game::be::beintplayer::tests::SEEDS_7 as SEEDS_9;
    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::dirtysock::tags::tests::put_str;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::string;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_4;

    pub(crate) use crate::game::fe::fereal::tests::base as base_2;
    pub(crate) use crate::game::main::gamemode::tests::w as w_4;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::object::debouncenode::tests::vtable_2;
    pub(crate) use crate::game::render::lightman::tests::eafile_pools;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_3;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_6;

    pub(crate) use crate::game::sound::soundsys::tests::w as w_8;

    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn uifont_set_add_font_matches() {
        check_lohi(
            ssx3_ee::s13::f_003984b0,
            uifont_set_add_font,
            200,
            &|c, seed| {
                let mut r = Rng(0x3984_b000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
                c.g[A3][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
                c.bus.write8(
                    DATA + 0x34,
                    if seed % 4 == 0 { 4 } else { (seed % 4) as u8 },
                );
            },
        );
    }

    #[test]
    fn ui_engine_render_state_mgr_matches() {
        check(
            ee::s13::f_00398018__generated,
            ui_engine_render_state_mgr,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39_8018, 0);
                c.g[A0][0] = ptr(&mut r) as u64;
            },
        );
    }

    // -----------------------------------------------------------------------
    // UITextureSet_Find / UIIcon_SetTexture.

    /// A texture set at `set`: `count` entries of 0x20 bytes with ids at
    /// `+0x1c`; `seed % 4 == 0` asks for an id that is not there.
    pub(crate) fn texture_set(c: &mut Cpu, r: &mut Rng, set: u32, count: u32) -> u32 {
        let entries = DATA + 0x4000;
        c.bus.write32(set + 0x10, count);
        c.bus.write32(set + 0xc, entries);
        for i in 0..8u32 {
            c.bus.write32(entries + i * 0x20, r.next_u32());
            c.bus.write32(entries + i * 0x20 + 0x1c, 0x1000 + i);
        }
        let n = r.next_u32() % (count + 1);
        if n < count {
            0x1000 + n
        } else {
            0x9000
        }
    }

    pub(crate) fn find_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3983_80, 0);
        c.g[A0][0] = DATA as u64;
        let count = 1 + r.next_u32() % 6;
        let id = texture_set(c, &mut r, DATA, count);
        c.g[A1][0] = id as u64;
    }

    driven!(gen_texture_find, ssx3_ee::s13::f_00398380__generated);

    driven!(hand_texture_find, ui_texture_set_find);

    #[test]
    fn ui_texture_set_find_matches() {
        check(
            ssx3_ee::s13::f_00398380__generated,
            ui_texture_set_find,
            64,
            &find_setup,
        );
        check_events(gen_texture_find, hand_texture_find, 64, &|c, seed| {
            find_setup(c, seed);
            let mut r = Rng(0x3983_80 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x0039_8380);
            // A short search may not reach the scheduled event: make the
            // first back edge due, identically for both bodies.
            c.next_event = c.cycles + 4;
            mode
        });
    }

    #[test]
    fn ui_icon_set_texture_matches() {
        check(
            ssx3_ee::s13::f_00399730__generated,
            ui_icon_set_texture,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3997_30, 0);
                let this = DATA;
                let icon = DATA + 0x800;
                let scene = DATA + 0x1000;
                let set = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x5c, icon);
                c.bus.write32(icon + 0xd0, scene);
                // The texture set lives at `*(scene + 0x10) + 0x58`.
                c.bus.write32(scene + 0x10, set);
                c.bus.write32(this + 0x7c, r.next_u32());
                let count = 1 + r.next_u32() % 4;
                let id = texture_set(c, &mut r, set + 0x58, count);
                c.g[A1][0] = id as u64;
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x3983f0 UIFontSet_FindFont.

    #[test]
    fn ui_font_set_find_font_matches() {
        check(
            ee::s13::f_003983f0__generated,
            ui_font_set_find_font,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3983_f0, 0);
                let bank = DATA + 0x100;
                c.g[A0][0] = sx(bank);
                let count = (seed % 5) as u8;
                c.bus.write8(bank + 0x34, count);
                let key = r.next_u32();
                let hit = (seed % 3) as u32;
                for i in 0..4u32 {
                    let entry = bank + 4 + 0xc * i;
                    c.bus
                        .write32(entry, if i == hit { key } else { r.next_u32() });
                    c.bus.write32(entry + 8, r.next_u32());
                }
                c.g[A1][0] = key as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // UI and streaming.

    #[test]
    fn ui_list_box_get_mode_entry_matches() {
        check(
            ee::s13::f_00399920__generated,
            ui_list_box_get_mode_entry,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3999_20, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let p1 = DATA + 0x800;
                let p2 = DATA + 0x820;
                let p3 = DATA + 0x840;
                let table = DATA + 0x900;
                clear(c, this, 0x100);
                clear(c, p1, 0x200);
                clear(c, table, 0x80);
                c.g[A0][0] = sx(this);
                c.bus.write8(this + 0x74, (seed % 4) as u8);
                if seed % 2 == 0 {
                    c.bus.write32(this + 0x5c, p1);
                    c.bus.write32(p1 + 0xd0, p2);
                    c.bus.write32(p2 + 0x10, p3);
                    c.bus.write32(p3 + 8, table);
                    for i in 0..4u32 {
                        c.bus.write32(table + i * 0xc + 0xc, r.next_u32());
                    }
                }
            },
        );
    }

    #[test]
    fn ui_group_refresh_text_matches() {
        check(
            ee::s13::f_00398738__generated,
            ui_group_refresh_text,
            SEEDS_2,
            &|c, seed| {
                let _ = base(c, seed, 0x3987_38);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                let list = s + 0x74;
                let sent = DATA + 0x500;
                w(c, sent, sent as u32);
                w(c, sent + 4, sent as u32);
                if seed % 3 == 0 {
                    w(c, list + 4, sent as u32);
                } else {
                    let n = DATA + 0x540;
                    clear(c, n, 0x20);
                    w(c, n, list as u32);
                    w(c, n + 4, sent as u32);
                    let vt = DATA + 0x580;
                    vtable_2(c, vt, &[(0x80, FAKE + 0x20)]);
                    w(c, n + 8, vt as u32);
                    w(c, list + 4, n as u32);
                }
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn ui_group_handle_input_matches() {
        check(
            ee::s13::f_00398798__generated,
            ui_group_handle_input,
            SEEDS_2,
            &|c, seed| {
                let mut r = base(c, seed, 0x3987_98);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                w(c, s + 0x14, if seed % 3 == 0 { 0x20 } else { 0 });
                let list = s + 0x74;
                let sent = DATA + 0x500;
                w(c, sent, sent as u32);
                w(c, sent + 4, sent as u32);
                if seed % 2 == 0 {
                    w(c, list + 4, sent as u32);
                } else {
                    let n = DATA + 0x540;
                    clear(c, n, 0x20);
                    w(c, n, list as u32);
                    w(c, n + 4, sent as u32);
                    let vt = DATA + 0x580;
                    vtable_2(c, vt, &[(0x10, FAKE + 0x30)]);
                    w(c, n + 8, vt as u32);
                    w(c, list + 4, n as u32);
                }
                c.g[A0][0] = sx(s);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ui_engine_ctor_matches() {
        check(
            ee::s13::f_00397b08__generated,
            ui_engine_ctor,
            30,
            &|c, seed| {
                base_2(c, seed, 0x39_7b08);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn c_ui_group_v18_matches() {
        check(
            ee::s13::f_003987f8__generated,
            c_ui_group_v18,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3987_f8, 0);
                let n0 = DATA + 0x1000;
                let n1 = DATA + 0x1200;
                let obj = DATA + 0x1300;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(DATA + 0x78, n0);
                if seed % 2 == 0 {
                    c.bus.write32(n0, n0);
                    c.bus.write32(n0 + 4, n0);
                } else {
                    c.bus.write32(n0, 0);
                    c.bus.write32(n0 + 4, n1);
                    c.bus.write32(n0 + 8, obj);
                    c.bus.write32(n1, n1);
                    c.bus.write32(n1 + 4, n1);
                    put_vtable(c, obj, &[0x90]);
                }
                c.pc = 0x0039_87f8;
            },
        );
    }

    /// A list of `n` nodes at `base`, the last pointing at itself; every node
    /// points at the visitor table `vt` and holds an empty string at +0x20.
    pub(crate) fn list(c: &mut Cpu, base: u32, n: u32, head_field: u32, vt: u32) {
        w_2(c, head_field, base);
        for i in 0..n {
            let node = base + 0x100 * i;
            let next = if i + 1 == n { node } else { node + 0x100 };
            w_2(c, node + 4, next);
            w_2(c, node + 8, vt);
            string(c, node + 0x20, "");
        }
    }

    // ------------------------------------------------------------ list walks

    #[test]
    fn ui_list_accept_visitor_matches() {
        check(
            ee::s13::f_003979f8__generated,
            ui_list_accept_visitor,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x39_79f8, 4);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x500) as u64;
                string(c, DATA + 0x500, "");
                list(c, DATA + 0x100, 3, DATA + 4, DATA + 0x600);
                let real = seed % 2 == 0;
                slot(
                    c,
                    DATA + 0x600,
                    0x10,
                    if real { 0x20 } else { 0 },
                    if real { 0x0041_aa88 } else { FAKE },
                );
                c.pc = 0x0039_79f8;
            },
        );
    }

    #[test]
    fn ui_font_set_free_fonts_matches() {
        check(
            ee::s13::f_00398438__generated,
            ui_font_set_free_fonts,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x398438, 4);
                be_library(c);
                world(c);
                let set = DATA;
                c.g[A0][0] = set as u64;
                let n = (seed % 4) as u8;
                b(c, set + 0x34, n);
                // Font objects are left null: the non-null path destroys them
                // through the fixed font destructor (0x3916c0), which frees the
                // pointer and would need a real heap allocation in the fixture.
                for i in 0..n as u32 {
                    let s = set + 4 + i * 0xc;
                    w_3(c, s, (seed + i) as u32);
                    w_3(c, s + 8, 0);
                }
            },
        );
    }

    #[test]
    fn cui_group_set_focus_matches() {
        check(
            ee::s13::f_00398638__generated,
            cui_group_set_focus,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x398638, 1);
                be_library(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                if seed % 2 == 0 {
                    w_4(c, obj + 0x74 + 4, 0);
                } else {
                    let s = DATA + 0x2000;
                    let a = DATA + 0x2100;
                    w_4(c, obj + 0x74 + 4, a);
                    w_4(c, a + 4, s);
                    w_4(c, a, 0);
                    w_4(c, s + 4, s);
                    w_4(c, s, 0);
                }
                w_4(c, obj + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_engine_load_package_matches() {
        check(
            ee::s13::f_00397c40__generated,
            ui_engine_load_package,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x397c_40, 8);
                eafile_pools(c);
                let engine = DATA;
                let name = DATA + 0x1000;
                put_str(c, name, b"package");
                c.g[A0][0] = engine as u64;
                c.g[A1][0] = name as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // UI groups.

    #[test]
    fn c_ui_group_update_anim_matches() {
        check(
            ssx3_ee::s13::f_00398910__generated,
            c_ui_group_update_anim,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x398910, 4);
                be_library(c);
                world(c);
                let group = DATA + 0x100;
                c.g[A0][0] = group as u64;
                c.g[A1][0] = (r.next_u32() & 0xffff) as u64;
                c.pc = 0x0039_8910;
                let list = group + 0x74;
                if seed % 4 == 0 {
                    // An empty list: the sentinel links back to itself.
                    let sent = DATA + 0x300;
                    w_6(c, list + 4, sent);
                    w_6(c, sent, sent);
                    w_6(c, sent + 4, sent);
                } else {
                    let n1 = DATA + 0x300;
                    let sent = DATA + 0x340;
                    let vt = DATA + 0x500;
                    w_6(c, list + 4, n1);
                    w_6(c, n1, r.next_u32());
                    w_6(c, n1 + 4, sent);
                    w_6(c, n1 + 8, vt);
                    slot_2(c, vt, 0x98, 0, FAKE);
                    slot_2(c, vt, 0x9c, 0, FAKE + 0x10);
                    w_6(c, sent, sent);
                    w_6(c, sent + 4, sent);
                }
            },
        );
    }

    #[test]
    fn c_ui_group_find_by_id_matches() {
        check(
            ee::s13::f_003986b0__generated,
            c_ui_group_find_by_id,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3986b0, 2);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 5) as u64;
                // The group's sub-list at_3 `+0x74` is empty.
                c.bus.write32(DATA + 0x74, 0);
                let node = DATA + 0x1000;
                let vt = vtable(c, &mut r, &[0x70]);
                c.bus.write32(node + 8, vt);
                c.bus.write32(node + 4, DATA + 0x1100);
                c.bus
                    .write32(DATA + 0x74, if seed % 3 == 0 { 0 } else { DATA + 0x1200 });
            },
        );
    }

    /// A real heap block of `size` bytes.
    pub(crate) fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run_2(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0x2000_0000, 0],
        ) as u32
    }

    // ----------------------------------------------------- list box wide str

    #[test]
    fn ui_list_box_set_item_wide_string_matches() {
        check(
            ee::s13::f_00399d80__generated,
            ui_list_box_set_item_wide_string,
            SEEDS_7,
            &|c, seed| {
                let _ = base_4(c, seed, 0x399d_80, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (obj + 0x1000) as u64;
                // The list-box chain that gates the update.
                let a = obj + 0x2000;
                let b = a + 0x100;
                let d = b + 0x100;
                w_7(c, obj + 0x5c, a);
                w_7(c, a + 0xd0, b);
                w_7(c, b + 0x10, d);
                w_7(c, d + 0x10, if seed % 3 == 0 { 0 } else { obj + 0x2300 });
                // The item's old string (a real block when present, so the
                // replacement's free of it is valid).
                let old = alloc(c, 8);
                w_7(
                    c,
                    obj + 0x9c + 0x14 * (seed % 4),
                    if seed % 2 == 0 { 0 } else { old },
                );
                // The wide source string.
                wh(c, obj + 0x1000, b'H' as u16);
                wh(c, obj + 0x1002, b'i' as u16);
                wh(c, obj + 0x1004, 0);
            },
        );
    }

    /// First size-class pool region (8-byte blocks): freeds must land here.
    pub(crate) const POOL0: u32 = DATA + 0x40_0000;

    #[test]
    fn ui_engine_texture_set_dtor_matches() {
        strict(
            ee::s13::f_00398078__generated,
            ui_engine_texture_set_dtor,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3980, 12);
                let pool = POOL0;
                let (this, array, extra) = (pool + 0x300, pool + 0x400, pool + 0x500);
                c.bus.write32(GAME_GP + 0x2a70, pool);
                c.bus.write32(pool + 0x800, 0);
                c.bus.write32(GAME_GP + 0xdd8, 1);
                let obj = DATA + 0x2000;
                let vt = DATA + 0x2100;
                c.bus.write16(vt + 0x190, 0);
                c.bus.write32(vt + 0x194, FAKE);
                c.bus.write32(GAME_GP - 0x854, obj);
                c.bus.write32(obj + 0x10d8, vt);
                c.bus.write32(this + 8, (seed % 4) as u32);
                c.bus.write32(this + 4, array);
                c.bus
                    .write32(this + 0xc, if seed % 5 == 0 { 0 } else { extra });
                for i in 0..8u32 {
                    c.bus.write32(array + 4 * i, DATA + 0x800 + 0x40 * i);
                    c.bus.write32(extra + 4 * i, DATA + 0x900 + 0x40 * i);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn ui_icon_load_matches() {
        check(
            ee::s13::f_00398998__generated,
            ui_icon_load,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3989, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                let data = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = data as u64;
                c.bus.write32(this + 0x74, r.next_u32());
                c.bus.write32(this + 0x5c, DATA + 0x2400);
                c.bus.write32(data + 0x20, r.next_u32());
                c.bus.write32(data + 0x24, seed % 4);
                c.bus.write16(data + 0x28, seed as u16);
                c.bus.write16(data + 0x2a, (seed * 3) as u16);
                c.bus.write32(GAME_GP - 0x2548, 0.01f32.to_bits());
                c.bus.write32(DATA + 0x2400 + 0xd0, DATA + 0x2800);
                c.bus.write32(DATA + 0x2800 + 0x10, DATA + 0x2c00);
            },
        );
    }

    #[test]
    fn ui_engine_dtor_matches() {
        strict_2(
            ee::s13::f_00397b70__generated,
            ui_engine_dtor,
            60,
            &|c, seed| {
                let _r = base_5(c, seed, 0x397b);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA, 0);
                c.bus.write32(DATA + 4, 0);
                c.bus
                    .write32(DATA + 0xc, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.bus
                    .write32(DATA + 0x14, if seed % 4 == 0 { 0 } else { DATA + 0x2200 });
                c.bus.write32(DATA + 0x2008, DATA + 0x2100);
                c.bus.write16(DATA + 0x2100 + 8, 0);
                c.bus.write32(DATA + 0x2100 + 0xc, FAKE);
                c.bus.write32(DATA + 0x2200, DATA + 0x2300);
                c.bus.write16(DATA + 0x2300 + 8, 0);
                c.bus.write32(DATA + 0x2300 + 0xc, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn uilistbox_refresh_text_matches() {
        check(
            ee::s13::f_00399e28__generated,
            uilistbox_refresh_text,
            12,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x399e28, 3);
                let this = DATA;
                clear_2(c, this, 0x800);
                c.g[A0][0] = this as u64;
                let a = DATA + 0x200;
                let b = DATA + 0x300;
                let loc = DATA + 0x400;
                clear_2(c, a, 0x100);
                clear_2(c, b, 0x100);
                clear_2(c, loc, 0x100);
                w_8(c, this + 0x5c, a);
                w_8(c, a + 0xd0, b);
                w_8(c, b + 0x10, loc);
                w_8(c, loc + 0x10, DATA + 0x500);
                let get = vtable(c, &mut r, &[0x20]);
                w_8(c, DATA + 0x500 + 4, get);
                let count = if seed % 3 == 0 { 0 } else { 2 };
                w_8(c, this + 0x318, count);
                for i in 0..3u32 {
                    w_8(
                        c,
                        this + 0x98 + 0x14 * i,
                        if i < count { 0x100 + i * 0x10 } else { 0 },
                    );
                }
                let relayout = vtable(c, &mut r, &[0x28]);
                w_8(c, this + 8, relayout);
            },
        );
    }

    /// A UI element whose vtable slots are fake calls, plus its data block.
    pub(crate) fn ui_fixture(c: &mut Cpu, seed: u32, salt: u64, slots: &[u32]) -> (u32, u32) {
        let mut r = base_7(c, seed, salt, 0);
        let this = DATA + 0x1000;
        let data = DATA + 0x2000;
        let vt = crate::difftest::fakes::vtable(c, &mut r, slots);
        w_9(c, this + 8, vt);
        w_9(c, this + 0x14, r.next_u32());
        // bit 2 of +0x74 survives the load and selects the texel block
        w_9(
            c,
            this + 0x74,
            if seed % 2 == 0 {
                r.next_u32() & !4
            } else {
                r.next_u32() | 4
            },
        );
        w_9(c, this + 0x78, r.next_u32());
        w_9(c, this + 0x80, r.next_u32());
        for k in 0..0x30u32 {
            c.bus.write8(data + 0x20 + k, r.next_u32() as u8);
        }
        w_9(c, data + 4, r.next_u32());
        w_9(c, data + 8, r.next_u32());
        w_9(c, data + 0xc, r.next_u32());
        w_9(c, data + 0x24, r.next_u32());
        w_9(c, GAME_GP.wrapping_sub(0x2544), r.f32(4.0));
        w_9(c, GAME_GP.wrapping_sub(0x24f8), r.f32(4.0));
        (this, data)
    }

    #[test]
    fn uilistbox_load_matches() {
        strict_3(
            ee::s13::f_00399820__generated,
            uilistbox_load,
            SEEDS_9,
            &|c, seed| {
                let (this, data) = ui_fixture(c, seed, 0x9820, &[0x48, 0x4c, 0xb8, 0xbc]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(data);
            },
        );
    }

    use crate::difftest::fill;
    use crate::difftest::put_words;

    pub(crate) use crate::game::object::instancenode::tests::oc383_floats;
    pub(crate) use crate::game::object::movenode::tests::oc383_vtable;

    /// Fixture layout: the icon, GrMan, its render-state stack and vtable,
    /// the icon's vtable and a texture page entry.
    pub(crate) const UI_ICON_RENDER_T_ICON: u32 = DATA;

    pub(crate) const UI_ICON_RENDER_T_CTX: u32 = DATA + 0x1000;

    pub(crate) const UI_ICON_RENDER_T_STATES: u32 = DATA + 0x3000;

    pub(crate) const UI_ICON_RENDER_T_CTX_VT: u32 = DATA + 0x4000;

    pub(crate) const UI_ICON_RENDER_T_ICON_VT: u32 = DATA + 0x4800;

    pub(crate) const UI_ICON_RENDER_T_TEX: u32 = DATA + 0x5000;

    /// Every path: textured by a page entry / by texture id / untextured,
    /// mirrored or not, 0-3 quarter turns, each axis rotation on or off.
    /// The size getter is the real `UIObject_GetVec3At60` (0x3a4c90), the
    /// drawer the real leaf at 0x1406c0.
    #[test]
    fn ui_icon_render_matches() {
        check(
            ee::s13::f_00398a78__generated,
            ui_icon_render,
            300,
            &|c, seed| {
                let mut r = Rng(0x0398_a780 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x0039_8a78;
                c.g[GP][0] = GAME_GP as u64;
                // Math_SinCos's .lit4 constants.
                put_words(
                    c,
                    0x004a_0230,
                    &[
                        0x3f22f983, 0x3fc90fdb, 0x3638ef1f, 0xb9500d03, 0x3c088889, 0xbe2aaaab,
                    ],
                );
                let icon = UI_ICON_RENDER_T_ICON;
                c.g[A0][0] = sx(icon);
                c.f[12] = r.f32(400.0);
                c.f[13] = r.f32(400.0);

                // GrMan: the render-state stack (two records) and the drawer.
                let ctx = UI_ICON_RENDER_T_CTX;
                c.bus.write32(GAME_GP - UI_ICON_RENDER_GRMAN, ctx);
                c.bus
                    .write32(ctx + UI_ICON_RENDER_CTX_TOP, UI_ICON_RENDER_T_STATES);
                fill(c, &mut r, UI_ICON_RENDER_T_STATES, 10);
                c.bus
                    .write32(ctx + UI_ICON_RENDER_CTX_VT, UI_ICON_RENDER_T_CTX_VT);
                c.bus.write16(UI_ICON_RENDER_T_CTX_VT + 0x238, 0);
                c.bus.write32(UI_ICON_RENDER_T_CTX_VT + 0x23c, 0x0014_06c0);

                // The icon: vtable slot 20 = the size getter (size at +0x60).
                c.bus
                    .write32(icon + UI_ICON_RENDER_VT, UI_ICON_RENDER_T_ICON_VT);
                c.bus.write16(UI_ICON_RENDER_T_ICON_VT + 0xa0, 0);
                c.bus.write32(UI_ICON_RENDER_T_ICON_VT + 0xa4, 0x003a_4c90);
                c.bus.write32(icon + 0x14, r.next_u32()); // alignment bits, blend byte
                for i in 0..4 {
                    c.bus
                        .write32(icon + UI_ICON_RENDER_COLOR + 4 * i, r.f32(1.2));
                }
                for i in 0..3 {
                    let angle = if (seed >> (2 + i)) & 1 == 0 {
                        0
                    } else {
                        r.f32(7.0)
                    };
                    c.bus.write32(icon + UI_ICON_RENDER_ROT + 4 * i, angle);
                    c.bus
                        .write32(icon + UI_ICON_RENDER_POS + 4 * i, r.f32(300.0));
                    c.bus
                        .write32(icon + UI_ICON_RENDER_SCALE + 4 * i, r.f32(3.0));
                    c.bus.write32(icon + 0x60 + 4 * i, r.f32(100.0));
                }
                c.bus.write32(icon + UI_ICON_RENDER_FLAGS, r.next_u32());
                let tex_id = match seed % 3 {
                    0 => r.next_u32() & 0x7fff,
                    1 => 0,
                    _ => 0x8000_0000 | r.next_u32(),
                };
                c.bus.write32(icon + UI_ICON_RENDER_TEX_ID, tex_id);
                let tex = if seed % 4 == 0 {
                    UI_ICON_RENDER_T_TEX
                } else {
                    0
                };
                c.bus.write32(icon + UI_ICON_RENDER_TEX, tex);
                c.bus.write32(UI_ICON_RENDER_T_TEX, r.next_u32() & 0xffff);
                for off in [0xc, 0x10, 0x14, 0x18] {
                    c.bus.write32(UI_ICON_RENDER_T_TEX + off, r.f32(1.0));
                }

                // The base matrix: identity or random.
                for i in 0..9 {
                    let v = if seed % 5 < 2 {
                        if i % 4 == 0 {
                            0x3f80_0000
                        } else {
                            0
                        }
                    } else {
                        r.f32(2.0)
                    };
                    c.bus.write32(UI_ICON_RENDER_BASE_MATRIX + 4 * i, v);
                }
            },
        );
    }

    /// Link `nodes` into the `cList` at `list` (+4: first), ending at the
    /// self-linked `sentinel`; an empty list is sometimes a null link.
    pub(crate) fn oc383_ui_list(c: &mut Cpu, r: &mut Rng, list: u32, nodes: &[u32], sentinel: u32) {
        if nodes.is_empty() && r.next_u32() % 2 == 0 {
            c.bus.write32(list + 4, 0);
            return;
        }
        let mut prev = list;
        for &n in nodes {
            c.bus.write32(prev + 4, n);
            c.bus.write32(n, prev);
            prev = n;
        }
        c.bus.write32(prev + 4, sentinel);
        c.bus.write32(sentinel, prev);
        c.bus.write32(sentinel + 4, sentinel);
    }

    #[test]
    fn c_ui_group_render_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s13::f_00398868__generated,
            c_ui_group_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8868, 4);
                c.pc = 0x0039_8868;
                let this = DATA;
                fill(c, &mut r, this, 0x20);
                oc383_floats(c, &mut r, this + 0x44, 2, 300.0);
                let n = (seed % 5) as usize;
                let nodes: Vec<u32> = (0..n as u32).map(|i| DATA + 0x1000 + 0x100 * i).collect();
                for &node in &nodes {
                    fill(c, &mut r, node, 8);
                    c.bus.write32(node + 8, DATA + 0x800);
                }
                oc383_ui_list(c, &mut r, this + UI_GROUP_CHILDREN, &nodes, DATA + 0x1800);
                oc383_vtable(c, &mut r, DATA + 0x800, &[0x88]);
                c.g[A0][0] = sx(this);
                c.f[12] = r.f32(100.0);
                c.f[13] = r.f32(100.0);
            },
        );
    }

    /// UIListBox_Render: a list box with a mode font from its state data,
    /// empty strings (the measure call returns a zero rectangle), the
    /// renderer at `gp-0x854` and `gp+0x2a90` with fake draw methods; the
    /// seeds vary the early exits, the colour, shadow and string flags, the
    /// overflow test and the debug outline.
    #[test]
    fn ui_list_box_render_matches() {
        check(
            ee::s13::f_00399f00__generated,
            ui_list_box_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x399f00, 3);
                c.pc = 0x0039_9f00;
                let this = DATA + 0x1000;
                let font = DATA + 0x2000;
                let (p1, p2, p3) = (DATA + 0x3000, DATA + 0x3100, DATA + 0x3200);
                let table = DATA + 0x3300;
                let loc = DATA + 0x3400;
                let renderer = DATA + 0x5000;
                let state = DATA + 0x5800;
                c.g[A0][0] = sx(this);
                c.f[12] = r.f32(200.0);
                c.f[13] = r.f32(200.0);
                // The list box.
                for off in (0..0x320).step_by(4) {
                    c.bus.write32(this + off, r.next_u32());
                }
                for off in [0x1c, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x60, 0x64] {
                    c.bus.write32(this + off, r.f32(100.0));
                }
                for i in 0..4u32 {
                    c.bus
                        .write32(this + 0x9c + 0x14 * i, DATA + 0x6000 + 0x20 * i);
                }
                c.bus
                    .write8(this + 0x318, if seed % 7 == 1 { 0 } else { 3 });
                c.bus.write8(this + 0x319, (seed % 3) as u8);
                c.bus.write32(this + 0x5c, p1);
                c.bus.write32(p1 + 0xd0, p2);
                c.bus.write32(p2 + 0x10, p3);
                c.bus.write32(p3 + 8, table);
                let has_font = seed % 11 != 3;
                for i in 0..4u32 {
                    c.bus
                        .write32(table + 0xc + 0xc * i, if has_font { font } else { 0 });
                }
                c.bus
                    .write32(p3 + 0x10, if seed % 5 == 0 { 0 } else { loc });
                c.bus.write32(loc, r.next_u32());
                // The font.
                for off in (0..0x60).step_by(4) {
                    c.bus.write32(font + off, r.f32(4.0));
                }
                c.bus.write32(font + 0x14, r.next_u32() & 0x3f);
                // The renderer.
                let vt = vtable(c, &mut r, &[0x288, 0x248, 0x238]);
                c.bus.write32(renderer + 0x10d8, vt);
                c.bus.write32(renderer + 0xe84, state);
                c.bus.write32(state + 8, r.next_u32());
                c.bus.write32(GAME_GP.wrapping_sub(0x854), renderer);
                c.bus.write32(GAME_GP + 0x2a90, renderer);
            },
        );
    }

    use crate::difftest::fakes;
    pub(crate) use crate::game::ui::uitext::tests::oc445_gate;

    pub(crate) use crate::game::ui::uitext::tests::OC445_OBJ;

    pub(crate) const OC445_OBJ2: u32 = DATA + 0x2000;

    /// `UIEngine_LoadTextures` up to its first call: the frame, the saved
    /// registers and the `sprintf` arguments (the fake unwinds there).
    #[test]
    fn ui_engine_load_textures_prologue_matches() {
        check(
            ee::s13::f_00398138__generated,
            ui_engine_load_textures,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x398138, 1);
                oc445_gate(c);
                c.bus.write32(fakes::LOG_UNWIND_AT, 0);
                c.g[A0][0] = sx(OC445_OBJ);
                c.g[A2][0] = sx(OC445_OBJ2);
                c.pc = 0x0039_8138;
            },
        );
    }

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    pub(crate) const SEEDS_10: u32 = 200;

    /// Gate of the callee fakes: a magic word in the machine's RAM turns a
    /// fake on (both sides of a diff get their own machine).
    pub(crate) const CTRL_MAGIC: u32 = 0x0726_0726;

    pub(crate) const CTRL: u32 = DATA + 0x20_0000;

    /// A gated fake of `$target`: logs the call and returns to `ra`, or runs
    /// the body it replaced (the previous override, else `$generated`).
    macro_rules! gated_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(CTRL) != CTRL_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    gated_fake!(
        oc726_fake_alloc,
        OC726_PREV_ALLOC,
        WPQ_ALLOC,
        ee::s10::f_00317d70
    );

    gated_fake!(
        oc726_fake_tattr,
        OC726_PREV_TATTR,
        UI_TATTR_CTOR,
        ee::s04::f_001dd580
    );

    gated_fake!(
        oc726_fake_list,
        OC726_PREV_LIST,
        UI_LIST_CTOR,
        ee::s13::f_003977e8
    );

    gated_fake!(
        oc726_fake_set_data,
        OC726_PREV_SET_DATA,
        UI_SCREEN_SET_DATA,
        ee::s13::f_0039c688
    );

    gated_fake!(
        oc726_fake_add_screen,
        OC726_PREV_ADD_SCREEN,
        UI_STATE_ADD_SCREEN,
        ee::s13::f_0039e758
    );

    gated_fake!(
        oc726_fake_create_objects,
        OC726_PREV_CREATE_OBJECTS,
        UI_SCREEN_CREATE_OBJECTS,
        ee::s13::f_0039ced0
    );

    gated_fake!(
        oc726_fake_free_slot,
        OC726_PREV_FREE_SLOT,
        BOLT_FREE_SLOT,
        ee::s03::f_0019d4b8
    );

    gated_fake!(
        oc726_fake_release_a,
        OC726_PREV_RELEASE_A,
        0x003d_f980,
        ee::s14::f_003df980
    );

    gated_fake!(
        oc726_fake_release_b,
        OC726_PREV_RELEASE_B,
        0x003d_f9d8,
        ee::s14::f_003df9d8
    );

    gated_fake!(
        oc726_fake_str_length,
        OC726_PREV_STR_LENGTH,
        LIST_STRING_LENGTH,
        ee::s14::f_003dbca0
    );

    gated_fake!(
        oc726_fake_str_copy,
        OC726_PREV_STR_COPY,
        LIST_STRING_COPY,
        ee::s14::f_003dbe20
    );

    gated_fake!(
        oc726_fake_text_alloc,
        OC726_PREV_TEXT_ALLOC,
        LIST_TEXT_ALLOC,
        ee::s10::f_00317e30
    );

    pub(crate) fn oc726_hook() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc726_fake_alloc,
            &OC726_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001DD580),
            oc726_fake_tattr,
            &OC726_PREV_TATTR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003977E8),
            oc726_fake_list,
            &OC726_PREV_LIST,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C688),
            oc726_fake_set_data,
            &OC726_PREV_SET_DATA,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E758),
            oc726_fake_add_screen,
            &OC726_PREV_ADD_SCREEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039CED0),
            oc726_fake_create_objects,
            &OC726_PREV_CREATE_OBJECTS,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_0019D4B8),
            oc726_fake_free_slot,
            &OC726_PREV_FREE_SLOT,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF980),
            oc726_fake_release_a,
            &OC726_PREV_RELEASE_A,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF9D8),
            oc726_fake_release_b,
            &OC726_PREV_RELEASE_B,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DBCA0),
            oc726_fake_str_length,
            &OC726_PREV_STR_LENGTH,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DBE20),
            oc726_fake_str_copy,
            &OC726_PREV_STR_COPY,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc726_fake_text_alloc,
            &OC726_PREV_TEXT_ALLOC,
        );
    }

    #[test]
    fn ui_engine_create_screen_matches() {
        oc726_hook();
        check(
            ee::s13::f_00397cc0__generated,
            ui_engine_create_screen,
            SEEDS_10,
            &|c, seed| {
                let mut r = machine(c, seed, 727, 1);
                // The object at DATA+0x1000 holds the table pointer at +4, and
                // the table's base offset at table+0xc.
                let object = DATA + 0x1000;
                let table = DATA + 0x2000;
                let base = table + 0x100;
                c.bus.write32(object + 4, table);
                c.bus.write32(table + 0xc, 0x100);
                let count = seed % 4;
                c.bus.write32(base, count);
                let mut keys = [0u32; 3];
                for (i, key) in keys.iter_mut().enumerate() {
                    *key = r.next_u32() | 1;
                    let rec = base + 4 + 8 * i as u32;
                    c.bus.write32(rec, *key);
                    c.bus.write32(rec + 4, 0x40 + 0x10 * i as u32);
                }
                // The key is one of the records (found) or a random one.
                let hash = if count > 0 && seed % 3 != 0 {
                    keys[(seed % count) as usize]
                } else {
                    r.next_u32()
                };
                c.g[A0][0] = object as u64;
                c.g[A1][0] = fakes::ptr(&mut r) as u64;
                c.g[A2][0] = hash as u64;
                // A non-zero screen skips the allocation branch.
                c.g[A3][0] = if seed % 2 == 0 {
                    0
                } else {
                    (DATA + 0x3000) as u64
                };
                c.pc = 0x0039_7cc0;
                c.bus.write32(CTRL, CTRL_MAGIC);
            },
        );
    }

    use crate::difftest::fakes::covered;
    pub(crate) use crate::game::sound::soundsys::tests::OC752_CTRL;
    pub(crate) use crate::game::sound::soundsys::tests::OC752_MAGIC;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::oc759_machine;

    /// `cUIGroup_Load`: the one forwarded call, answered or unwound.
    #[test]
    fn ui_group_load_matches() {
        oc752_install_fakes();
        covered(
            ssx3_ee::s13::f_00398618__generated,
            ui_group_load,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x0039_8618, 1);
                c.bus.write32(OC752_CTRL, OC752_MAGIC);
            },
        );
    }

    /// `UIListBox_ctor` against the generated body: base ctor and memset are
    /// fakes (unwinds in each); the row clear loop, flags and the colour
    /// copy are real.
    #[test]
    fn uilistbox_ctor_w244_matches() {
        check(
            ee::s13::f_00399768__generated,
            uilistbox_ctor_w244,
            80,
            &|c, seed| {
                let mut r = oc759_machine(c, seed, 0x399768, 2, 0x0039_9768);
                let this = DATA + 0x5000;
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this, 0x100);
                fill(c, &mut r, 0x004c_66c8, 4);
            },
        );
    }

    /// Gate of the oc811 fakes (a magic word in the machine's RAM) and the
    /// mailbox of the file loader fake (0: the fake's pointer, 1: return 0).
    pub(crate) const OC811_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC811_MAIL: u32 = OC811_CTRL + 4;

    pub(crate) const OC811_MAGIC: u32 = 0x4f43_3831;

    /// The fake callee: logs like `fakes::fake_call`; the file loader fake can
    /// be told to return 0.
    pub(crate) fn oc811_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC811_MAIL);
        fakes::fake_call(c, addr);
        if addr == 0x003e_18c8 && c.pc == ra && mail == 1 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc811_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC811_CTRL) != OC811_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc811_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc811_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc811_fakes!(
        (
            oc811_fake_file_load,
            OC811_PREV_FILE_LOAD,
            ee::s14::OVERRIDE_003E18C8,
            ee::s14::f_003e18c8__generated,
            0x003e_18c8
        ),
        (
            oc811_fake_new_named,
            OC811_PREV_NEW_NAMED,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc811_fake_mem_free,
            OC811_PREV_MEM_FREE,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc811_fake_memcpy,
            OC811_PREV_MEMCPY,
            ee::s15::OVERRIDE_0041605C,
            ee::s15::f_0041605c__generated,
            0x0041_605c
        ),
        (
            oc811_fake_alloc_named,
            OC811_PREV_ALLOC_NAMED,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc811_fake_font_ctor,
            OC811_PREV_FONT_CTOR,
            ee::s12::OVERRIDE_003915E8,
            ee::s12::f_003915e8__generated,
            0x0039_15e8
        ),
    );

    /// UI_AsciiToWide on random byte strings (including the empty string and
    /// bytes with the high bit set), with events due inside the loop.
    #[test]
    fn ui_ascii_to_wide_matches() {
        driven!(gen_ascii, ee::s13::f_00397ad8__generated);
        driven!(hand_ascii, ui_ascii_to_wide);
        check_events(gen_ascii, hand_ascii, 96, &|c, seed| {
            let mut r = Rng(0x3979_d8 ^ (seed as u64) << 8);
            let src = DATA + 0x100;
            let dst = DATA + 0x400;
            let len = match seed % 5 {
                0 => 0,
                _ => (r.next_u32() % 12) as u32,
            };
            for i in 0..len {
                let b = (r.next_u32() as u8).max(1);
                c.bus.write8(src + i, b);
            }
            c.bus.write8(src + len, 0);
            for i in 0..0x40 {
                c.bus.write8(dst + i, 0x5a);
            }
            c.g[A0][0] = dst as u64;
            c.g[A1][0] = src as u64;
            c.g[RA][0] = crate::difftest::RA as u64;
            c.g[V1][0] = r.next_u32() as u64;
            schedule(c, &mut r, seed, 0x0039_7ad8)
        });
    }

    /// Build the font set: `count` entries of 12 bytes from `+4`, each with a
    /// random name pointer at `+4` of the entry.
    pub(crate) fn oc811_font_set(c: &mut Cpu, r: &mut Rng, count: i8) -> u32 {
        let this = DATA;
        c.bus.write8(this + 0x34, count as u8);
        for i in 0..4 {
            let e = this + 4 + 12 * i;
            c.bus.write32(e, r.next_u32());
            c.bus.write32(e + 4, DATA + 0x800 + (r.next_u32() & 0xff0));
            c.bus.write32(e + 8, r.next_u32());
        }
        c.g[A0][0] = this as u64;
        this
    }

    /// UIFontSet_LoadFonts: with the allocator and the font constructor
    /// faked (some seeds unwind inside a call), then with events due at the
    /// loop's back edge.
    #[test]
    fn ui_font_set_load_fonts_matches() {
        oc811_install_fakes();
        check(
            ee::s13::f_00398568__generated,
            ui_font_set_load_fonts_811,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0039_8568, 8);
                c.pc = 0x0039_8568;
                c.bus.write32(OC811_CTRL, OC811_MAGIC);
                oc811_font_set(c, &mut r, [-2, 0, 1, 2, 3, 4][seed as usize % 6]);
            },
        );
        driven!(gen_fonts, ee::s13::f_00398568__generated);
        driven!(hand_fonts, ui_font_set_load_fonts_811);
        check_events(gen_fonts, hand_fonts, 48, &|c, seed| {
            let mut r = machine(c, seed, 0x0039_8569, 0);
            c.bus.write32(OC811_CTRL, OC811_MAGIC);
            oc811_font_set(c, &mut r, 2 + (seed % 3) as i8);
            c.g[RA][0] = crate::difftest::RA as u64;
            schedule(c, &mut r, seed, 0x0039_8568)
        });
    }

    pub(crate) const OC1066_GATE: u32 = DATA + 0x2c_0000;

    pub(crate) const OC1066_MAGIC: u32 = 0x1066_0001;

    pub(crate) fn oc1066_no_prev(_c: &mut Cpu) {
        unreachable!("oc1066 fake called before it was hooked");
    }

    /// Gated fake of `UIScreen_StartThread` (0x39c870): logs and returns.
    pub(crate) fn oc1066_fake_start_thread(c: &mut Cpu) {
        if c.bus.read32(OC1066_GATE) != OC1066_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1066_fake_start_thread),
                oc1066_no_prev,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0039_c870, ra);
    }

    /// `UIListBox_HandleInput`: the input vtable's up/down/accept slots answer
    /// zero (a real leaf) or non-zero (a logged fake) at random, the attribute
    /// word and the current entry are random (wrap or not), and the move
    /// result (read through the real `UIObject_GetStateData` chain) takes the
    /// values 0, 1, 0x10 and 0x100 in all combinations (with and without a
    /// key-frame thread for the event). The listener is the object's own or
    /// the default one in the globals. Fakes unwind at random calls.
    #[test]
    fn ui_list_box_handle_input_matches() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc1066_fake_start_thread,
            fakes::prev_cell(oc1066_fake_start_thread),
        );
        covered(
            ee::s13::f_00399970__generated,
            ui_list_box_handle_input,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x0039_9970, 6);
                let zero = 0x0014_06c0u32;
                let this = DATA + 0x1000;
                let scr = DATA + 0x1400;
                let a3 = DATA + 0x1800;
                let ov = DATA + 0x1c00;
                let state = DATA + 0x2000;
                let listener = DATA + 0x2400;
                let lvt = DATA + 0x2800;
                let table = DATA + 0x1200;
                let p = DATA + 0x3000;
                let q = DATA + 0x3400;
                let inp = DATA + 0x4000;
                let iv = DATA + 0x4400;
                c.bus.write32(OC1066_GATE, OC1066_MAGIC);
                // The list box.
                let count = if seed % 9 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 6
                };
                c.bus.write8(this + 0x318, count as u8);
                c.bus
                    .write8(this + 0x319, (r.next_u32() % (count + 1)) as u8);
                c.bus.write32(this + 0x14, r.next_u32());
                c.bus
                    .write32(this + 0x5c, if seed % 11 == 0 { 0 } else { scr });
                c.bus
                    .write32(this + 0x3c, if seed % 3 == 0 { 0 } else { table });
                for k in 0..16u32 {
                    let v = if r.next_u32() % 2 == 0 {
                        0xffff
                    } else {
                        0x10 + k
                    };
                    c.bus.write16(table + 2 * k, v as u16);
                }
                // Screen object chain: scr+0xd0 -> a3, a3+0x10 -> state.
                c.bus.write32(scr + 0xd0, a3);
                c.bus.write32(a3 + 8, ov);
                c.bus.write32(a3 + 0x10, state);
                c.bus
                    .write32(state + 0x14, if seed % 2 == 0 { 0 } else { listener });
                c.bus.write32(listener, lvt);
                c.bus.write32(GAME_GP + 0x2968, lvt);
                c.bus
                    .write16(lvt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(lvt + 0x14, fakes::FAKE);
                // Object vtable: focus notify (0x98), focus move (0xb0, the
                // real chain reader returning the move result).
                c.bus
                    .write16(ov + 0x98, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(ov + 0x9c, fakes::FAKE + 0x10);
                c.bus.write16(ov + 0xb0, 0);
                c.bus.write32(ov + 0xb4, 0x0039_ff50);
                c.bus.write32(a3 + 0x5c, p);
                c.bus.write32(p + 0xd0, q);
                const MOVES: [u32; 8] = [0, 1, 0x10, 0x11, 0x100, 0x101, 0x110, 0x111];
                c.bus.write32(q + 0x10, MOVES[(r.next_u32() % 8) as usize]);
                // The input's vtable slots: zero or non-zero answers.
                c.bus.write32(inp + 8, iv);
                for (i, off) in [0x30u32, 0x88, 0x90, 0x98, 0xa0].into_iter().enumerate() {
                    c.bus
                        .write16(iv + off, (r.next_u32() as i16 as i32 & !3) as u16);
                    let f = if r.next_u32() % 2 == 0 {
                        zero
                    } else {
                        fakes::FAKE + 0x20 + 0x10 * i as u32
                    };
                    c.bus.write32(iv + off + 4, f);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(inp);
                c.pc = 0x0039_9970;
            },
        );
    }

    #[test]
    fn ui_icon_swap_words_matches() {
        check(
            ssx3_ee::s13::f_00398a60__generated,
            ui_icon_swap_words,
            200,
            &|c, seed| {
                let mut r = Rng(0x398a_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                // Both distinct and aliased pointers.
                let b = if seed % 5 == 0 { DATA } else { DATA + 0x40 };
                c.g[A1][0] = sx(b);
                c.bus.write32(DATA, r.next_u32());
                c.bus.write32(DATA + 0x40, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_font_set_parse_font_pack_matches() {
        check(
            ssx3_ee::s13::f_003984e8__generated,
            ui_font_set_parse_font_pack,
            200,
            &|c, seed| {
                let mut r = Rng(0x3984_e800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                // The font set's style byte: below 4 adds an entry, else not.
                c.bus.write8(DATA + 0x34, (seed % 6) as u8);
                // Pack: entry count, then entries of (word, step) with the
                // next entry `step + 8` bytes on.
                let n = seed % 4;
                c.bus.write32(DATA + 0x200, n);
                let mut e = DATA + 0x204;
                for _ in 0..4 {
                    let step = r.next_u32() % 0x10;
                    c.bus.write32(e, r.next_u32());
                    c.bus.write32(e + 4, step);
                    e += step + 8;
                }
            },
        );
    }
}
