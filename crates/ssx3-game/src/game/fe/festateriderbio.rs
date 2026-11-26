//! `src/fe/festateriderbio.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
pub(crate) use crate::game::fe::feasyncfile::lh_2 as lh;
pub(crate) use crate::game::fe::feasyncfile::lw;
pub(crate) use crate::game::fe::fepopup::oc508_vtable_dtor;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateoptions::screen_object;
pub(crate) use crate::game::wscript::wscriptman::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_3550,
        c_fe_state_rider_detail_on_render,
        "cFEStateRiderDetail_OnRender",
    ),
    (
        0x0018_35c8,
        cfe_state_rider_detail_update,
        "cFEStateRiderDetail_Update",
    ),
    (
        0x0018_3a98,
        c_fe_state_uber_trick_ctor,
        "cFEStateUberTrick_ctor",
    ),
    (
        0x0018_3b08,
        cfestate_uber_trick_select_left_panel,
        "cFEStateUberTrick_SelectLeftPanel",
    ),
    (
        0x0018_3c08,
        cfestate_uber_trick_select_right_panel,
        "cFEStateUberTrick_SelectRightPanel",
    ),
    (
        0x0018_3d08,
        cfestateubertrick_set_button_icon,
        "cFEStateUberTrick_SetButtonIcon",
    ),
    (
        0x001d_d8c8,
        c_fe_state_rider_detail_dtor,
        "cFEStateRiderDetail_dtor",
    ),
    (0x0018_32a8, rider_detail_init, "cFEStateRiderDetail_Init"),
    (
        0x0018_33d0,
        rider_detail_on_activate,
        "cFEStateRiderDetail_OnActivate",
    ),
    (
        0x0018_35a8,
        rider_detail_render,
        "cFEStateRiderDetail_Render",
    ),
    (
        0x0018_35f8,
        rider_detail_on_child_event,
        "cFEStateRiderDetail_OnChildEvent",
    ),
    (
        0x0018_3710,
        rider_detail_on_widget_action,
        "cFEStateRiderDetail_OnWidgetAction",
    ),
];

/// Direct calls through the generated function (which dispatches into the
/// replacement when one is installed).
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::fe::festateload::callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW;

    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;

    pub(crate) use crate::game::fe::festaterewards::callee::FE_STATE_CTOR;

    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;

    pub const C_FE_STATE_ON_ACTIVATE: Callee = Callee {
        addr: 0x0018_6518,
        f: ee::s02::f_00186518,
    };

    pub const SOUND_MANAGER_REF_DTOR: Callee = Callee {
        addr: 0x002a_dee8,
        f: ee::s08::f_002adee8,
    };
}

/// `gp`-relative globals used here.
pub(crate) mod g {}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {

    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
    pub(crate) use crate::game::wscript::wscriptman::table::UI_SCREEN_FIND_OBJECT;
}

/// `void cFEStateRiderDetail_Update(cFEStateRiderDetail* self)` (0x1835c8):
/// same shape as the character-setup state.
pub fn cfe_state_rider_detail_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_V13, 0x0018_35dc)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW.call(c, 0x0018_35e4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateRiderDetail_OnRender(cFEStateRiderDetail* self)`
/// (0x183550): resolve the "rider detail" panel and fade it in.
pub fn c_fe_state_rider_detail_on_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(this, 0x50)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = 0x45_d898; // D_0045D898
            call_ptr(c, table::STR_HASH, 0x0018_3578)?;
            c.g[A0][0] = Addr(p(this, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, table::UI_SCREEN_FIND_OBJECT, 0x0018_3584)?;
            let none = c.g[V0][0] == 0;
            frame.restore(c, S0, 0x10);
            if !none {
                c.cycles += 3;
                c.g[A0][0] = c.g[V0][0];
                callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF.call(c, 0x0018_3594)?;
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

/// `cFEStateUberTrick* cFEStateUberTrick_ctor(cFEStateUberTrick* s, int a1,
/// signed char idx)` (0x183a98).
pub fn c_fe_state_uber_trick_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 16, 0x20);
        sqsp(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = sx((c.g[A2][0] as u32) << 24);
        frame.save_ra(c, 0);
        let s1 = c.g[S1][0] as u32;
        c.g[S1][0] = sx(((s1 as i32) >> 24) as u32);
        callee::FE_STATE_CTOR.call(c, 0x0018_3ab8)?;
        c.g[V0][0] = 0x12;
        c.g[V1][0] = 0x0046_ccc0;
        Addr(p_2(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        Addr(p_2(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        Addr(p_2(r32(c, S0), 0x44)).sb(c, c.g[S1][0]);
        c.g[V0][0] = Addr(p_2(r32(c, S0), 0x10)).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        call::call_ptr(c, 0x001a_1cd0, 0x0018_3ae0)?;
        Addr(p_2(r32(c, S0), 0x15)).sb(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p_2(r32(c, S0), 0x4c)).sw(c, 0);
        Addr(p_2(r32(c, S0), 0x50)).sw(c, 0);
        Addr(p_2(r32(c, S0), 0x48)).sw(c, 0);
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void cFEStateUberTrick_SetButtonIcon(self, Rect* rect)` (0x183d08): pass
/// the rect's size (scaled) to the icon widget, or hide it when there is no
/// rect.
pub fn cfestateubertrick_set_button_icon(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.swc1(20, frame.at(c, 0x40));
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        let no_rect = c.g[S0][0] == 0;
        c.mov_s(20, 12);
        if !no_rect {
            c.cycles += 33;
            c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = Addr(p(r32(c, V0), 0xa0)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xa4)).lw(c);
            let getpos = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, getpos, 0x0018_3d44)?;
            c.lwc1(4, p(r32(c, S0), 0xc));
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(2, p(r32(c, S0), 0x14));
            c.lwc1(3, p(r32(c, S0), 0x10));
            c.lwc1(1, p(r32(c, S0), 0x18));
            c.g[AT][0] = 0x43800000;
            c.mtc1(1, 0);
            c.sub_s(2, 2, 3);
            c.sub_s(1, 1, 4);
            c.g[V1][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.mul_s(0, 20, 0);
            c.mul_s(1, 1, 0);
            c.mul_s(2, 2, 0);
            c.swc1(1, p(r32(c, SP), 4));
            c.swc1(2, r32(c, SP));
            c.g[A0][0] = Addr(p(r32(c, V1), 0xa8)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0xac)).lw(c);
            let setpos = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, setpos, 0x0018_3d90)?;
            c.g[V1][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = Addr(p(r32(c, V1), 0x48)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x4c)).lw(c);
            let show = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, show, 0x0018_3da8)?;
        } else {
            c.cycles += 7;
            c.g[V0][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let show = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, show, 0x0018_3dc8)?;
        }
        c.lq(S0, frame.at(c, 0x30));
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        c.lwc1(20, frame.at(c, 0x40));
        frame.close(c);
        Ok(())
    })
}

/// One `select panel` step: look up the item on the screen by string id and
/// set its enabled flag (or clear it when it is missing).
pub(crate) fn panel_select(
    c: &mut Cpu,
    this: u32,
    string_off: u32,
    flag: u64,
    ret: u32,
    charge: u32,
) -> crate::call::Flow {
    c.cycles += charge as u64;
    c.g[A0][0] = 0x0046_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(string_off));
    call_ptr(c, 0x0031_7670, ret.wrapping_sub(0x2c))?; // StrHash
    c.g[A0][0] = lw(c, p(this, 0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret.wrapping_sub(0x20))?; // UIScreen_FindObject
    c.g[A2][0] = c.g[V0][0];
    let missing = c.g[A2][0] == 0;
    c.g[A1][0] = flag;
    if !missing {
        c.cycles += 5;
        c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 8));
        c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x48));
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x4c));
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, ret)?;
    }
    Ok(())
}

/// `cFEStateUberTrick_SelectLeftPanel(self)` (0x183b08): mark the left-hand
/// UberTrick panel items enabled/disabled and refresh their state.
pub fn cfestate_uber_trick_select_left_panel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        panel_select(c, this, 0x2700, 0, 0x0018_3b50, 0)?;
        panel_select(c, this, 0x26f0, 1, 0x0018_3b88, 9)?;
        panel_select(c, this, 0x26e0, 1, 0x0018_3bc0, 9)?;
        panel_select(c, this, 0x26d0, 0, 0x0018_3bf8, 9)?;
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateUberTrick_SelectRightPanel(self)` (0x183c08): the mirrored
/// version of [`cfestate_uber_trick_select_left_panel`].
pub fn cfestate_uber_trick_select_right_panel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        panel_select(c, this, 0x2700, 1, 0x0018_3c50, 0)?;
        panel_select(c, this, 0x26f0, 0, 0x0018_3c88, 9)?;
        panel_select(c, this, 0x26e0, 0, 0x0018_3cc0, 9)?;
        panel_select(c, this, 0x26d0, 1, 0x0018_3cf8, 9)?;
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The same, fetching the function pointer into `v1` and the object's model
/// pointer (`*(obj+8)`) into `v0`.
pub(crate) fn call_object_v1(c: &mut Cpu, obj: u64, _arg: u64, ret: u32) -> Flow {
    c.g[V0][0] = Addr(obj as u32).off(8).lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).off(0x48).lh(c);
    c.g[V1][0] = Addr(r32(c, V0)).off(0x4c).lw(c);
    let func = c.g[V1][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}

// ---------------------------------------------------------------------------
// cFEStateRiderDetail_OnActivate.

/// The widget names `cFEStateRiderDetail_OnActivate` tags with state 5..9.
pub(crate) const RIDER_DETAIL_WIDGETS: [(u32, u64); 5] = [
    (0x0045_d888, 5),
    (0x0045_d898, 6),
    (0x0045_d8a8, 7),
    (0x0045_d8b8, 8),
    (0x0045_d8c8, 9),
];

/// `void cFEStateRiderDetail_OnActivate(FEState *s, int arg)` (0x1833d0): put
/// five rider-detail widgets into their state, then run the base activate.
pub fn c_fe_state_rider_detail_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = abi::arg64(c, 1);

        let this = r32(c, S1);
        for (i, &(name, state)) in RIDER_DETAIL_WIDGETS.iter().enumerate() {
            if i > 0 {
                c.cycles += 9;
            }
            let ret = 0x0018_33f8 + 0x40 * i as u32;
            screen_object(c, this, name, ret, ret + 0xc)?;
            let obj = c.g[V0][0];
            c.g[A1][0] = 1;
            if obj == 0 {
                continue;
            }
            c.cycles += 7;
            call_object_v1(c, obj, 1, ret + 0x2c)?;
            c.g[V0][0] = state;
            Addr(obj as u32).off(0x18).sw(c, c.g[V0][0]);
        }

        c.cycles += 9;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        callee::C_FE_STATE_ON_ACTIVATE.call(c, 0x0018_3538)?;

        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SndPlayParams_dtor.

/// `cFEStateRiderDetail::~cFEStateRiderDetail(this)` (0x1dd8c8).
pub fn c_fe_state_rider_detail_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc508_vtable_dtor(c, sx(0x0047_0000 - 0x2f30), 0x0039_e390, 0x001d_d8e4)
    })
}

/// `void cFEStateRiderDetail_Init(cFEStateRiderDetail* self)`: add the rider-detail screen to the
/// UI engine and play its first frame, set up the rider preview, make the human rider visible with
/// the current character, enable the audio and flag the cheat-character panel (`+0x50`) when the
/// player owns cheat characters.
/// (0x001832a8; every register, cycle and call of the generated body is kept.)
pub fn rider_detail_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1832fc: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2790));
            c.g[RA][0] = sx(0x001832d0);
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001832d0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001832e4);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001832e4)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1832fc;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001832fc);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001832fc)?;
                break 'b_1832fc;
            }
        }
        'b_1833b8: {
            c.cycles += 47;
            c.g[RA][0] = sx(0x00183304);
            c.g[A0][0] = c.g[S2][0];
            Callee {
                addr: 0x00182db8,
                f: ee::s02::cFEStateCharSetup_InitRiderPreview,
            }
            .call(c, 0x00183304)?;
            c.g[RA][0] = sx(0x0018330c);
            call_ptr(c, 0x0014dc80, 0x0018330c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00183318);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x00183318)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[RA][0] = sx(0x00183328);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00147488, 0x00183328)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00183340);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x00183340)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A2][0] = 0u64;
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
            c.g[RA][0] = sx(0x00183358);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a0570, 0x00183358)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[A3][0] = 0u64;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
            c.g[RA][0] = sx(0x00183374);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a0508, 0x00183374)?;
            c.g[RA][0] = sx(0x0018337c);
            call_ptr(c, 0x0028b180, 0x0018337c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00183388);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0028f140, 0x00183388)?;
            c.g[V0][0] = 0x1;
            c.g[RA][0] = sx(0x00183394);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x50), c.g[V0][0]);
            call_ptr(c, 0x0014dc80, 0x00183394)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001833a0);
            c.g[A1][0] = 0xd;
            call_ptr(c, 0x0014dd58, 0x001833a0)?;
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001833b0);
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x001577a0, 0x001833b0)?;
            if c.g[V0][0] == 0u64 {
                c.sw((c.g[S2][0] as u32).wrapping_add(0x50), 0u64);
                break 'b_1833b8;
            } else {
                break 'b_1833b8;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRiderDetail_OnActivate(self, arg)`: tag the five rider-detail widgets with
/// states 5..9 (show them), then run the base `cFEState_OnActivate`.
/// (0x001833d0; every register, cycle and call of the generated body is kept.)
pub fn rider_detail_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        'b_18342c: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S2][0] = c.g[A1][0];
            c.g[RA][0] = sx(0x001833f8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2778));
            call_ptr(c, 0x00317670, 0x001833f8)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x00183404);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00183404)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_18342c;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00183424);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00183424)?;
                c.g[V0][0] = 0x5;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_18342c;
            }
        }
        'b_18346c: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00183438);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2768));
            call_ptr(c, 0x00317670, 0x00183438)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x00183444);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00183444)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_18346c;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00183464);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00183464)?;
                c.g[V0][0] = 0x6;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_18346c;
            }
        }
        'b_1834ac: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00183478);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2758));
            call_ptr(c, 0x00317670, 0x00183478)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x00183484);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00183484)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_1834ac;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001834a4);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001834a4)?;
                c.g[V0][0] = 0x7;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_1834ac;
            }
        }
        'b_1834ec: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001834b8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2748));
            call_ptr(c, 0x00317670, 0x001834b8)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001834c4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001834c4)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_1834ec;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001834e4);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001834e4)?;
                c.g[V0][0] = 0x8;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_1834ec;
            }
        }
        'b_18352c: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001834f8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2738));
            call_ptr(c, 0x00317670, 0x001834f8)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x00183504);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00183504)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_18352c;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00183524);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00183524)?;
                c.g[V0][0] = 0x9;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_18352c;
            }
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x00183538);
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x00186518,
            f: ee::s02::cFEState_OnActivate,
        }
        .call(c, 0x00183538)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRiderDetail_Render(self)`: tail wrapper around `UIState_Render`.
/// (0x001835a8; every register, cycle and call of the generated body is kept.)
pub fn rider_detail_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x001835b8);
        call_ptr(c, 0x0039e6b8, 0x001835b8)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRiderDetail_OnChildEvent(self, item, msg)`: popup closed (msg 0x16). The cheat-char
/// popup (name hash `D_0045D780`) sets the player's cheat character when done; the keyboard popup
/// (hash 0) sets the player name when accepted and not empty. Then re-show the 3D character.
/// (0x001835f8; every register, cycle and call of the generated body is kept.)
pub fn rider_detail_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_1836f8: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x00183620);
            c.g[S3][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x00183620)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0018362c);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x0018362c)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = 0x16;
            let t = c.g[S0][0] != c.g[V0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            if t {
                break 'b_1836f8;
            } else {
                'b_1836ec: {
                    'b_1836e8: {
                        'b_1836a4: {
                            c.cycles += 6;
                            c.g[A0][0] = 0x460000;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[RA][0] = sx(0x0018364c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2880));
                            call_ptr(c, 0x00317670, 0x0018364c)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                break 'b_1836a4;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                                let t = c.g[V0][0] == 0u64;
                                c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                if t {
                                    break 'b_1836e8;
                                } else {
                                    c.cycles += 17;
                                    c.g[RA][0] = sx(0x00183668);
                                    call_ptr(c, 0x0014dc80, 0x00183668)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x00183674);
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x0014dd58, 0x00183674)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x44));
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x00183688);
                                    c.g[A2][0] = c.g[S1][0];
                                    call_ptr(c, 0x001474a8, 0x00183688)?;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0018369c);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018369c)?;
                                    let t = true;
                                    c.g[A0][0] = c.g[S3][0];
                                    if t {
                                        break 'b_1836ec;
                                    } else {
                                        break 'b_1836a4;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.g[S3][0];
                        if t {
                            break 'b_1836ec;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V0][0] == 0u64;
                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                            if t {
                                break 'b_1836ec;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001836c0);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00416810, 0x001836c0)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'b_1836e8;
                                } else {
                                    c.cycles += 8;
                                    c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x44));
                                    c.g[RA][0] = sx(0x001836d4);
                                    c.g[A2][0] = c.g[S0][0];
                                    call_ptr(c, 0x00147138, 0x001836d4)?;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001836e8);
                                    c.g[A0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001836e8)?;
                                    break 'b_1836e8;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S3][0];
                    break 'b_1836ec;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x001836f4);
                c.g[A1][0] = 0x1;
                Callee {
                    addr: 0x001831d0,
                    f: ee::s02::cFEStateCharSetup_SetRiderPreviewActive,
                }
                .call(c, 0x001836f4)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_1836f8;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRiderDetail_OnWidgetAction(self, item, msg)`: widget actions 5/6/7 of the
/// rider-detail screen: open the keyboard or cheat-character popup, switch the state, or open the
/// options state.
/// (0x00183710; every register, cycle and call of the generated body is kept.)
pub fn rider_detail_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_183a88: {
            'b_183a84: {
                'b_183a80: {
                    c.cycles += 7;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[V1][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    let t = c.g[V1][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_183a80;
                    } else {
                        'b_183a2c: {
                            'b_183a1c: {
                                'b_183a18: {
                                    'b_1839e0: {
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x6;
                                        let t = c.g[A2][0] == c.g[A0][0];
                                        c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                        if t {
                                            break 'b_1839e0;
                                        } else {
                                            'b_183764: {
                                                'b_183750: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x5;
                                                    if t {
                                                        break 'b_183750;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[A2][0] == c.g[V0][0];
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        if t {
                                                            break 'b_183764;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.lq(
                                                                17,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            if t {
                                                                break 'b_183a88;
                                                            } else {
                                                                break 'b_183750;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x7;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_183a2c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    );
                                                    if t {
                                                        break 'b_183a88;
                                                    } else {
                                                        break 'b_183764;
                                                    }
                                                }
                                            }
                                            'b_1839a8: {
                                                'b_183998: {
                                                    'b_1838ec: {
                                                        c.cycles += 4;
                                                        c.g[A2][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x18));
                                                        c.g[V0][0] = 0x8;
                                                        let t = c.g[A2][0] != c.g[V0][0];
                                                        c.g[A1][0] = 0x460000;
                                                        if t {
                                                            break 'b_1838ec;
                                                        } else {
                                                            c.cycles += 94;
                                                            c.g[A0][0] = 0x444;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0x2728));
                                                            c.g[A2][0] = 0x100;
                                                            c.g[RA][0] = sx(0x00183788);
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317d70, 0x00183788)?;
                                                            c.g[A1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[T0][0] = 0xf;
                                                            c.g[A2][0] = c.g[S1][0];
                                                            c.g[A3][0] = 0u64;
                                                            c.g[RA][0] = sx(0x001837a0);
                                                            c.g[A0][0] = c.g[V0][0];
                                                            call_ptr(c, 0x001cb030, 0x001837a0)?;
                                                            c.g[S0][0] = c.g[V0][0];
                                                            c.g[A1][0] = 0x8;
                                                            c.g[V0][0] = 0x37;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001837b8);
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x140),
                                                                c.g[V0][0],
                                                            );
                                                            call_ptr(c, 0x001cd088, 0x001837b8)?;
                                                            c.g[A2][0] = 0x1;
                                                            c.g[A3][0] = 0x2;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001837cc);
                                                            c.g[A1][0] = 0x4b;
                                                            call_ptr(c, 0x001ce3c8, 0x001837cc)?;
                                                            c.g[V0][0] = 0x1;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x58),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[RA][0] = sx(0x001837e0);
                                                            c.g[A1][0] = 0x1;
                                                            call_ptr(c, 0x001ce408, 0x001837e0)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0xe;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x001837f4);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x001837f4)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x27;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183808);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x00183808)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x28;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x0018381c);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x0018381c)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x34;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183830);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x00183830)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x1a;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183844);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x00183844)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x19;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183858);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x00183858)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x35;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x0018386c);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x0018386c)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x40;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183880);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x00183880)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x41;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183894);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x00183894)?;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[A3][0] = 0x2;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001838a8);
                                                            c.g[A1][0] = 0x42;
                                                            call_ptr(c, 0x001ce3c8, 0x001838a8)?;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A1][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001838b8);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            call_ptr(c, 0x0039f290, 0x001838b8)?;
                                                            c.g[RA][0] = sx(0x001838c0);
                                                            call_ptr(c, 0x0014dc80, 0x001838c0)?;
                                                            c.g[A0][0] = c.g[V0][0];
                                                            c.g[RA][0] = sx(0x001838cc);
                                                            c.g[A1][0] = 0x1;
                                                            call_ptr(c, 0x0014dd58, 0x001838cc)?;
                                                            c.g[A1][0] = c.lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                            c.g[RA][0] = sx(0x001838d8);
                                                            c.g[A0][0] = c.g[V0][0];
                                                            call_ptr(c, 0x00147170, 0x001838d8)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001838e4);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x001cb418, 0x001838e4)?;
                                                            let t = true;
                                                            c.g[A0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_183998;
                                                            } else {
                                                                break 'b_1838ec;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    if c.g[A2][0] != c.g[A0][0] {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                        break 'b_1839a8;
                                                    } else {
                                                        c.cycles += 19;
                                                        c.g[RA][0] = sx(0x001838fc);
                                                        call_ptr(c, 0x0014dc80, 0x001838fc)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x00183908);
                                                        c.g[A1][0] = 0x1;
                                                        call_ptr(c, 0x0014dd58, 0x00183908)?;
                                                        c.g[A1][0] = c
                                                            .lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                        c.g[RA][0] = sx(0x00183914);
                                                        c.g[A0][0] = c.g[V0][0];
                                                        call_ptr(c, 0x00147488, 0x00183914)?;
                                                        c.g[RA][0] = sx(0x0018391c);
                                                        c.g[S0][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0014dc80, 0x0018391c)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x00183928);
                                                        c.g[A1][0] = 0xd;
                                                        call_ptr(c, 0x0014dd58, 0x00183928)?;
                                                        c.g[A1][0] = c
                                                            .lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x00183938);
                                                        c.g[A2][0] = c.g[S0][0];
                                                        call_ptr(c, 0x001577a0, 0x00183938)?;
                                                        let t = (c.g[V0][0] as i64) <= 0;
                                                        c.g[A1][0] = 0x460000;
                                                        if t {
                                                            break 'b_183a18;
                                                        } else {
                                                            c.cycles += 22;
                                                            c.g[A0][0] = 0x1e4;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0x2718));
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183954);
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317d70, 0x00183954)?;
                                                            c.g[A3][0] = c.lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                            c.g[T0][0] = c.g[S0][0];
                                                            c.g[A1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A2][0] = c.g[S1][0];
                                                            c.g[RA][0] = sx(0x0018396c);
                                                            c.g[A0][0] = c.g[V0][0];
                                                            Callee { addr: 0x00182220, f: ee::s02::cFEStateCheatCharSelect_ctor }.call(c, 0x0018396c)?;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[S0][0] = c.g[V0][0];
                                                            c.g[A1][0] = c.lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                            c.g[RA][0] = sx(0x00183980);
                                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                            call_ptr(c, 0x001a1cd0, 0x00183980)?;
                                                            c.sb(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x15),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A1][0] = c.g[S0][0];
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[RA][0] = sx(0x00183994);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            call_ptr(c, 0x0039f290, 0x00183994)?;
                                                            c.g[A0][0] = c.g[S1][0];
                                                            break 'b_183998;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001839a0);
                                                c.g[A1][0] = 0u64;
                                                Callee { addr: 0x001831d0, f: ee::s02::cFEStateCharSetup_SetRiderPreviewActive }.call(c, 0x001839a0)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S1][0];
                                                if t {
                                                    break 'b_183a1c;
                                                } else {
                                                    break 'b_1839a8;
                                                }
                                            }
                                            c.cycles += 9;
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[V1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001839c4);
                                            c.g[A0][0] =
                                                sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001839c4)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = c.g[V0][0];
                                            if t {
                                                break 'b_183a18;
                                            } else {
                                                c.cycles += 5;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                                c.g[RA][0] = sx(0x001839d8);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                                call_ptr(c, 0x0039f400, 0x001839d8)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S1][0];
                                                if t {
                                                    break 'b_183a1c;
                                                } else {
                                                    break 'b_1839e0;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 11;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00183a04);
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00183a04)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_183a80;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = sx(0x00183a18);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039f400, 0x00183a18)?;
                                        break 'b_183a18;
                                    }
                                }
                                c.cycles += 1;
                                c.g[A0][0] = c.g[S1][0];
                                break 'b_183a1c;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00183a24);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001831d0,
                                f: ee::s02::cFEStateCharSetup_SetRiderPreviewActive,
                            }
                            .call(c, 0x00183a24)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_183a84;
                            } else {
                                break 'b_183a2c;
                            }
                        }
                        c.cycles += 21;
                        c.g[A1][0] = 0x460000;
                        c.g[A3][0] = 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2968));
                        c.g[A0][0] = 0x54;
                        c.g[RA][0] = sx(0x00183a44);
                        c.g[A2][0] = 0x100;
                        call_ptr(c, 0x00317d70, 0x00183a44)?;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00183a54);
                        c.g[A2][0] = 0x1;
                        Callee {
                            addr: 0x001887a0,
                            f: ee::s02::cFEStateOptions_ctor,
                        }
                        .call(c, 0x00183a54)?;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                        c.g[A0][0] = 0xffffffffffffc0ff;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A2][0] = 0u64;
                        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                        c.g[A3][0] = 0u64;
                        c.g[V1][0] = c.g[V1][0] | 0x100;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x00183a80);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f320, 0x00183a80)?;
                        break 'b_183a80;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_183a84;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_183a88;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;

    use crate::difftest::put_words;

    use crate::difftest::DATA;

    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_3;
    pub(crate) use crate::game::be::beintplayer::tests::w8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2 as strict;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base;

    pub(crate) use crate::game::sound::soundsys::tests::w;
    pub(crate) use crate::game::sound::soundsys::tests::wf;

    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;
    use ssx3_ee as ee;

    use ssx3_ee::s02;

    #[test]
    fn cfe_state_rider_detail_update_matches() {
        check(
            s02::f_001835c8__generated,
            cfe_state_rider_detail_update,
            120,
            &|c, seed| {
                machine(c, seed, 0x1835c8, 2);
                globals(c);
                clear(c, OBJ, 0x100);
                c.g[A0][0] = sx(OBJ);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_fe_state_rider_detail_on_render_matches() {
        check(
            ee::s02::f_00183550__generated,
            c_fe_state_rider_detail_on_render,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1835_50, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let screen = DATA + 0x800;
                clear_2(c, this, 0x100);
                clear_2(c, screen, 0x200);
                c.bus.write32(this + 0x40, screen);
                c.bus
                    .write32(this + 0x50, if seed % 2 == 0 { 0 } else { 1 });
                put_words(c, 0x0045_d898, &[0x5449_544c, 0]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_fe_state_uber_trick_ctor_matches() {
        check(
            ssx3_ee::s02::f_00183a98__generated,
            c_fe_state_uber_trick_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x183a98, 0);
                let s = DATA + 0x1000;
                let engine = DATA + 0x2000;
                let names = DATA + 0x3000;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = engine as u64;
                c.g[A2][0] = (seed % 8) as u64;
                w32(c, s + 0x10, engine);
                w32(c, engine, names);
                for i in 0..8u32 {
                    w8(c, names + i + 8, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn cfestateubertrick_set_button_icon_matches() {
        check(
            ee::s02::f_00183d08__generated,
            cfestateubertrick_set_button_icon,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x183d08, 3);
                let this = DATA;
                clear_3(c, this, 0x400);
                c.g[A0][0] = this as u64;
                let rect = DATA + 0x200;
                clear_3(c, rect, 0x100);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { rect as u64 };
                c.f[12] = if seed % 2 == 0 {
                    1.0f32.to_bits()
                } else {
                    0.5f32.to_bits()
                };
                wf(c, rect + 0xc, 1.0);
                wf(c, rect + 0x10, 2.0);
                wf(c, rect + 0x14, 3.0);
                wf(c, rect + 0x18, 4.0);
                let vt = vtable(c, &mut r, &[0xa0, 0xa8, 0x48]);
                w(c, this + 8, vt);
            },
        );
    }

    #[test]
    fn cfestate_uber_trick_select_left_panel_matches() {
        strict(
            ssx3_ee::s02::f_00183b08__generated,
            cfestate_uber_trick_select_left_panel,
            SEEDS,
            &|c, seed| {
                let _ = base_2(c, seed, 0x3b08);
                let this = DATA + 0x1000;
                // No screen: every item lookup misses and only the flags of
                // the (absent) items are touched.
                w(c, this + 0x40, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn cfestate_uber_trick_select_right_panel_matches() {
        strict(
            ssx3_ee::s02::f_00183c08__generated,
            cfestate_uber_trick_select_right_panel,
            SEEDS,
            &|c, seed| {
                let _ = base_2(c, seed, 0x3c08);
                let this = DATA + 0x1000;
                w(c, this + 0x40, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    pub(crate) use crate::game::fe::fepopup::tests::oc508_setup;

    #[test]
    fn c_fe_state_rider_detail_dtor_matches() {
        check(
            ee::s04::f_001dd8c8__generated,
            c_fe_state_rider_detail_dtor,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1dd8c8);
            },
        );
    }

    use crate::difftest::fakes;
    use crate::game::main::loadscreens::tests::oc736_fake_call;
    use crate::game::main::loadscreens::tests::oc736_fixture;
    use crate::game::main::loadscreens::tests::oc736_force;
    use crate::game::main::loadscreens::tests::oc736_force_zero;
    use crate::game::main::loadscreens::tests::FX_CTRL;
    use crate::game::main::loadscreens::tests::FX_IFVT;
    use crate::game::main::loadscreens::tests::FX_MAGIC;
    use std::sync::atomic::AtomicUsize;

    /// The rider-detail state (`this`), the popup / widget `item`, the UI
    /// manager chain and the screen object the lookups find.
    pub(crate) const OC827_THIS: u32 = DATA + 0x40_0000;

    pub(crate) const OC827_ITEM: u32 = DATA + 0x40_1000;

    pub(crate) const OC827_MGR: u32 = DATA + 0x40_2000;

    pub(crate) const OC827_OBJ: u32 = DATA + 0x40_3000;

    pub(crate) const OC827_VT: u32 = DATA + 0x40_4000;

    pub(crate) const OC827_SCR: u32 = DATA + 0x40_5000;

    pub(crate) const OC827_SVT: u32 = DATA + 0x40_6000;

    /// What the fake `StrHash` answers for the cheat-char popup name.
    pub(crate) const OC827_HASH: u32 = 0x1234_5678;

    /// Gated fakes (gate: `FX_CTRL == FX_MAGIC`, written by `oc736_fixture`)
    /// for every callee with an override slot; with the gate off they run the
    /// body they replaced.
    macro_rules! oc827_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(FX_CTRL) != FX_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc736_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc827_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc827_fakes!(
        (
            oc827_fake_317670,
            OC827_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc827_fake_397cc0,
            OC827_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc827_fake_39c870,
            OC827_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc827_fake_182db8,
            OC827_PREV_182DB8,
            ee::s02::OVERRIDE_00182DB8,
            ee::s02::f_00182db8__generated,
            0x0018_2db8
        ),
        (
            oc827_fake_14dc80,
            OC827_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc827_fake_14dd58,
            OC827_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc827_fake_147488,
            OC827_PREV_147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488
        ),
        (
            oc827_fake_1a0570,
            OC827_PREV_1A0570,
            ee::s03::OVERRIDE_001A0570,
            ee::s03::f_001a0570__generated,
            0x001a_0570
        ),
        (
            oc827_fake_1a0508,
            OC827_PREV_1A0508,
            ee::s03::OVERRIDE_001A0508,
            ee::s03::f_001a0508__generated,
            0x001a_0508
        ),
        (
            oc827_fake_28b180,
            OC827_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc827_fake_28f140,
            OC827_PREV_28F140,
            ee::s07::OVERRIDE_0028F140,
            ee::s07::f_0028f140__generated,
            0x0028_f140
        ),
        (
            oc827_fake_1577a0,
            OC827_PREV_1577A0,
            ee::s01::OVERRIDE_001577A0,
            ee::s01::f_001577a0__generated,
            0x0015_77a0
        ),
        (
            oc827_fake_39e6b8,
            OC827_PREV_39E6B8,
            ee::s13::OVERRIDE_0039E6B8,
            ee::s13::f_0039e6b8__generated,
            0x0039_e6b8
        ),
        (
            oc827_fake_39d860,
            OC827_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc827_fake_186518,
            OC827_PREV_186518,
            ee::s02::OVERRIDE_00186518,
            ee::s02::f_00186518__generated,
            0x0018_6518
        ),
        (
            oc827_fake_1474a8,
            OC827_PREV_1474A8,
            ee::s01::OVERRIDE_001474A8,
            ee::s01::f_001474a8__generated,
            0x0014_74a8
        ),
        (
            oc827_fake_147138,
            OC827_PREV_147138,
            ee::s01::OVERRIDE_00147138,
            ee::s01::f_00147138__generated,
            0x0014_7138
        ),
        (
            oc827_fake_416810,
            OC827_PREV_416810,
            ee::s15::OVERRIDE_00416810,
            ee::s15::f_00416810__generated,
            0x0041_6810
        ),
        (
            oc827_fake_1831d0,
            OC827_PREV_1831D0,
            ee::s02::OVERRIDE_001831D0,
            ee::s02::f_001831d0__generated,
            0x0018_31d0
        ),
        (
            oc827_fake_317d70,
            OC827_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc827_fake_1cd088,
            OC827_PREV_1CD088,
            ee::s04::OVERRIDE_001CD088,
            ee::s04::f_001cd088__generated,
            0x001c_d088
        ),
        (
            oc827_fake_1ce3c8,
            OC827_PREV_1CE3C8,
            ee::s04::OVERRIDE_001CE3C8,
            ee::s04::f_001ce3c8__generated,
            0x001c_e3c8
        ),
        (
            oc827_fake_1ce408,
            OC827_PREV_1CE408,
            ee::s04::OVERRIDE_001CE408,
            ee::s04::f_001ce408__generated,
            0x001c_e408
        ),
        (
            oc827_fake_147170,
            OC827_PREV_147170,
            ee::s01::OVERRIDE_00147170,
            ee::s01::f_00147170__generated,
            0x0014_7170
        ),
        (
            oc827_fake_1cb418,
            OC827_PREV_1CB418,
            ee::s04::OVERRIDE_001CB418,
            ee::s04::f_001cb418__generated,
            0x001c_b418
        ),
        (
            oc827_fake_1a1cd0,
            OC827_PREV_1A1CD0,
            ee::s03::OVERRIDE_001A1CD0,
            ee::s03::f_001a1cd0__generated,
            0x001a_1cd0
        ),
        (
            oc827_fake_39f400,
            OC827_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc827_fake_1887a0,
            OC827_PREV_1887A0,
            ee::s02::OVERRIDE_001887A0,
            ee::s02::f_001887a0__generated,
            0x0018_87a0
        ),
        (
            oc827_fake_39f320,
            OC827_PREV_39F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039_f320
        ),
        (
            oc827_fake_39e318,
            OC827_PREV_39E318,
            ee::s13::OVERRIDE_0039E318,
            ee::s13::f_0039e318__generated,
            0x0039_e318
        ),
        (
            oc827_fake_3180b8,
            OC827_PREV_3180B8,
            ee::s10::OVERRIDE_003180B8,
            ee::s10::f_003180b8__generated,
            0x0031_80b8
        ),
        (
            oc827_fake_398380,
            OC827_PREV_398380,
            ee::s13::OVERRIDE_00398380,
            ee::s13::f_00398380__generated,
            0x0039_8380
        ),
        (
            oc827_fake_15a478,
            OC827_PREV_15A478,
            ee::s01::OVERRIDE_0015A478,
            ee::s01::f_0015a478__generated,
            0x0015_a478
        ),
        (
            oc827_fake_397840,
            OC827_PREV_397840,
            ee::s13::OVERRIDE_00397840,
            ee::s13::f_00397840__generated,
            0x0039_7840
        ),
        (
            oc827_fake_3978f0,
            OC827_PREV_3978F0,
            ee::s13::OVERRIDE_003978F0,
            ee::s13::f_003978f0__generated,
            0x0039_78f0
        ),
    );

    /// The common machine: fake hooks, the fakes' gate, the world, the
    /// back-end interface (`0x14dd58` answers `FX_IFACE`, whose vtable has the
    /// methods at +8 and +0x10 as fake calls) and a clean object page.
    pub(crate) fn oc827_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        oc827_install_fakes();
        let _ = oc736_fixture(c, seed, salt, calls);
        c.bus.write16(FX_IFVT + 0x10, 0x10 * (seed % 3) as u16);
        c.bus.write32(FX_IFVT + 0x14, fakes::FAKE + 0x30);
        c.bus.write8(OC827_THIS + 0x44, seed as u8);
        c.bus.write32(OC827_THIS + 0x10, OC827_MGR);
        c.bus.write32(OC827_MGR, OC827_OBJ);
        c.bus.write32(OC827_OBJ + 4, OC827_VT);
        c.bus.write32(OC827_SCR + 8, OC827_SVT);
        c.bus.write16(OC827_SVT + 0x48, 0x10 * (seed % 3) as u16);
        c.bus.write32(OC827_SVT + 0x4c, fakes::FAKE + 0x40);
    }

    /// `cFEStateRiderDetail_Init`: the screen is added or not (`0x397cc0`
    /// answers 0), the cheat-character count is zero or not.
    #[test]
    fn rider_detail_init_matches() {
        check(
            ee::s02::f_001832a8__generated,
            rider_detail_init,
            200,
            &|c, seed| {
                oc827_base(c, seed, 0x1832a8, 15);
                if seed % 2 == 0 {
                    oc736_force_zero(c, 0x0039_7cc0);
                }
                if (seed / 2) % 2 == 0 {
                    oc736_force_zero(c, 0x0015_77a0);
                }
                c.g[A0][0] = sx(OC827_THIS);
                c.pc = 0x0018_32a8;
            },
        );
    }

    /// `cFEStateRiderDetail_OnActivate`: the five widget lookups all find the
    /// screen object or all miss.
    #[test]
    fn rider_detail_on_activate_matches() {
        check(
            ee::s02::f_001833d0__generated,
            rider_detail_on_activate,
            200,
            &|c, seed| {
                oc827_base(c, seed, 0x1833d0, 16);
                if seed % 2 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC827_SCR);
                }
                c.g[A0][0] = sx(OC827_THIS);
                c.g[A1][0] = sx(seed.wrapping_mul(0x9e37_79b1));
                c.pc = 0x0018_33d0;
            },
        );
    }

    /// `cFEStateRiderDetail_Render`: one call to `UIState_Render`.
    #[test]
    fn rider_detail_render_matches() {
        check(
            ee::s02::f_001835a8__generated,
            rider_detail_render,
            60,
            &|c, seed| {
                oc827_base(c, seed, 0x1835a8, 1);
                c.g[A0][0] = sx(OC827_THIS);
                c.pc = 0x0018_35a8;
            },
        );
    }

    /// `cFEStateRiderDetail_OnChildEvent`: another message; the cheat-char
    /// popup (not done / done); the keyboard popup (name hash 0; not
    /// accepted, empty name, name); an unknown popup.
    #[test]
    fn rider_detail_on_child_event_matches() {
        check(
            ee::s02::f_001835f8__generated,
            rider_detail_on_child_event,
            300,
            &|c, seed| {
                oc827_base(c, seed, 0x1835f8, 9);
                oc736_force(c, 0x0031_7670, OC827_HASH);
                let msg = if seed % 7 == 0 { 0x15 } else { 0x16 };
                let item = OC827_ITEM;
                match (seed / 7) % 6 {
                    0 => {
                        c.bus.write32(item + 0xc, OC827_HASH);
                        c.bus.write32(item + 0x74, 0);
                    }
                    1 => {
                        c.bus.write32(item + 0xc, OC827_HASH);
                        c.bus.write32(item + 0x74, 1);
                        c.bus.write32(item + 0x70, 3);
                    }
                    2 => {
                        c.bus.write32(item + 0xc, 0);
                        c.bus.write32(item + 0x5c, 0);
                    }
                    3 => {
                        c.bus.write32(item + 0xc, 0);
                        c.bus.write32(item + 0x5c, 1);
                        oc736_force_zero(c, 0x0041_6810);
                    }
                    4 => {
                        c.bus.write32(item + 0xc, 0);
                        c.bus.write32(item + 0x5c, 1);
                        c.bus.write32(item + 0x74, 0x4142_4300);
                    }
                    _ => c.bus.write32(item + 0xc, 0x77),
                }
                c.g[A0][0] = sx(OC827_THIS);
                c.g[A1][0] = sx(item);
                c.g[A2][0] = msg;
                c.pc = 0x0018_35f8;
            },
        );
    }

    /// `cFEStateRiderDetail_OnWidgetAction` by `seed % 8`: no item, msg 6,
    /// msg 5 with item state 8 / 6 / other, msg 7, msg below 5, msg above 7;
    /// the virtual calls answer 0 (the real leaf `0x1406c0`) or a pointer;
    /// `0x1577a0` answers 0 or non-zero.
    #[test]
    fn rider_detail_on_widget_action_matches() {
        check(
            ee::s02::f_00183710__generated,
            rider_detail_on_widget_action,
            320,
            &|c, seed| {
                oc827_base(c, seed, 0x183710, 25);
                let item = OC827_ITEM;
                let sel = seed % 8;
                let (msg, state) = match sel {
                    0 | 1 => (6, 3),
                    2 => (5, 8),
                    3 => (5, 6),
                    4 => (5, 3),
                    5 => (7, 3),
                    6 => (3, 3),
                    _ => (9, 3),
                };
                c.bus.write32(item + 0x18, state);
                let method = if (seed / 8) % 2 == 0 {
                    0x0014_06c0
                } else {
                    fakes::FAKE + 0x50
                };
                for off in [0x20u32, 0x28] {
                    c.bus.write16(OC827_VT + off, 0x10 * (seed % 3) as u16);
                    c.bus.write32(OC827_VT + off + 4, method);
                }
                if (seed / 16) % 2 == 0 {
                    oc736_force_zero(c, 0x0015_77a0);
                } else {
                    oc736_force(c, 0x0015_77a0, 1);
                }
                c.g[A0][0] = sx(OC827_THIS);
                c.g[A1][0] = if sel == 0 { 0 } else { sx(item) };
                c.g[A2][0] = msg;
                c.pc = 0x0018_3710;
            },
        );
    }
}
