//! `src/fe/festatecharsetup.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::gp;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_ACTIVATED;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
pub(crate) use crate::game::wscript::wscriptman::p;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_2c38,
        cfe_state_char_setup_update,
        "cFEStateCharSetup_Update",
    ),
    (
        0x0018_2c68,
        fe_state_rider_detail_on_widget_notify,
        "cFEStateRiderDetail_OnWidgetNotify",
    ),
    (
        0x0018_31d0,
        cfe_state_char_setup_set_rider_preview_active,
        "cFEStateCharSetup_SetRiderPreviewActive",
    ),
    (
        0x0018_3238,
        c_fe_state_rider_detail_ctor,
        "cFEStateRiderDetail_ctor",
    ),
    (0x0031_1a50, anim_controller_reset, "AnimController_Reset"),
    (
        0x001d_d8a0,
        fe_state_char_setup_dtor,
        "cFEStateCharSetup_dtor",
    ),
    (
        0x0018_2ec0,
        cfe_state_char_setup_update_rider_preview,
        "cFEStateCharSetup_UpdateRiderPreview",
    ),
    (
        0x0018_2928,
        cfe_state_char_setup_init,
        "cFEStateCharSetup_Init",
    ),
    (
        0x0018_2c08,
        cfe_state_char_setup_shutdown,
        "cFEStateCharSetup_Shutdown",
    ),
    (
        0x0018_2c80,
        c_fe_state_char_setup_on_widget_action,
        "cFEStateCharSetup_OnWidgetAction",
    ),
    (
        0x0018_2db8,
        cfe_state_char_setup_init_rider_preview,
        "cFEStateCharSetup_InitRiderPreview",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::festateload::callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW;

    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
}

/// `gp`-relative globals used here.
pub(crate) mod g {}

/// `void cFEStateCharSetup_Update(cFEStateCharSetup* self)` (0x182c38): the
/// base state's update, then the setup state's rider-preview update.
pub fn cfe_state_char_setup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_V13, 0x0018_2c4c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW.call(c, 0x0018_2c54)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int cFEStateRiderDetail_OnWidgetNotify(...)` (0x182C68): the `Ok` widget
/// is 9.
pub fn fe_state_rider_detail_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] ^= 9;
    c.g[V1][0] = WIDGET_ACTIVATED;
    c.g[V0][0] = WIDGET_IGNORED;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}

/// `void cFEStateCharSetup_SetRiderPreviewActive(self, int on)` (0x1831d0):
/// look the rider up in the game's rider manager and toggle the rider
/// preview.
pub fn cfe_state_char_setup_set_rider_preview_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        open(c, 0x30);
        c.g[V0][0] = c.lw(gp(c, -0x848));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        let no_mgr = c.g[A0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !no_mgr {
            c.cycles += 5;
            c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
            c.g[RA][0] = sx(0x0018_3200);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a_0548, 0x0018_3200)?;
            if c.g[S0][0] != 0 {
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[S0][0]);
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0018_3214);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0019_e538, 0x0018_3214)?;
                let still = c.g[S0][0] != 0;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[S0][0]);
                if !still {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                }
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x30);
        Ok(())
    })
}

/// `void* cFEStateRiderDetail_ctor(cFEStateRiderDetail* self, void* a1, int
/// index)` (0x183238): construct the char-setup base state and fill in the
/// rider-detail class and pad mask.
pub fn c_fe_state_rider_detail_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0018_28c0, 0x0018_325c)?;
        c.g[V0][0] = 0xd;
        c.g[V1][0] = 0x46cd_90;
        Addr(at(c, S0, 0xc)).sw(c, c.g[V0][0]);
        Addr(at(c, S0, 8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        Addr(at(c, S0, 0x44)).sb(c, c.g[S1][0]);
        c.g[V0][0] = Addr(at(c, S0, 0x10)).lw(c);
        c.g[A0][0] = Addr(at(c, V0, 0)).lw(c);
        call_ptr(c, 0x001a_1cd0, 0x0018_3284)?;
        Addr(at(c, S0, 0x15)).sb(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c, S0, 0x4c)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Animation controller.

/// `void AnimController_Reset(AnimController* self)` (0x311a50): clear the six
/// animation sequence lists at `+0x50` (8 bytes each), set each current
/// animation id (`+0..+0x14`) to 0x1b6, and restore the defaults: `+0x18` = 0,
/// `+0x1c` = 1.0, `+0x20` = -1 and the two 16-byte constants at `0x4ff230`
/// copied to `+0x30`/`+0x40`.
pub fn anim_controller_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[S1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0x1b6;
        loop {
            c.cycles += 9;
            c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x50));
            c.g[A0][0] = sx((c.g[S2][0] as u32) << 3);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, 0x0031_4668, 0x0031_1a8c)?; // AnimSeqList_Clear
            sw(c, c.g[S0][0] as u32, c.g[S3][0]);
            c.g[V0][0] = ((c.g[S2][0] as i64) < 6) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0031_1a78);
                }
                continue;
            }
            break;
        }
        c.cycles += 19;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = 0x0050_0000;
        sw(c, p(c.g[S1][0] as u32, 0x18), 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xdd0));
        c.swc1(0, p(c.g[S1][0] as u32, 0x1c));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        frame.restore(c, S0, 0x40);
        c.lq(2, c.g[A0][0] as u32);
        frame.restore(c, S2, 0x20);
        c.sq(2, p(c.g[S1][0] as u32, 0x30));
        frame.restore(c, S3, 0x10);
        c.lq(3, p(c.g[A0][0] as u32, 0x10));
        Addr(p(c.g[S1][0] as u32, 0x20)).sd(c, c.g[A1][0]);
        c.sq(3, p(c.g[S1][0] as u32, 0x40));
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The destructor-chain helper the front-end state dtors call (0x39e390).
pub(crate) const OC437_BASE_DTOR: u32 = 0x0039_e390;

/// A front-end state destructor: `this->vtable(+8) = 0x470000 - vtable_lo`,
/// then the base destructor helper (0x39e390) with the caller's `a0`/`a1`.
pub(crate) fn oc437_fe_state_dtor(c: &mut Cpu, vtable_lo: u32, ret_to: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x47_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(0x47_0000u32.wrapping_sub(vtable_lo));
        c.sw((c.g[V1][0] as u32).wrapping_add(8), c.g[V0][0]);
        call_ptr(c, OC437_BASE_DTOR, ret_to)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCharSetup_dtor(this, flags)` (0x1dd8a0).
pub fn fe_state_char_setup_dtor(c: &mut Cpu) {
    oc437_fe_state_dtor(c, 0x2f30, 0x001d_d8bc)
}

/// `cFEStateCharSetup_UpdateRiderPreview(self)` (0x182ec0): looks up the
/// human rider of the setup screen (`self+0x44`, through the world's
/// rider manager at `world+0x7c`), and when its textures are loaded and the
/// 3D preview is on (`self+0x4c`), sets the rider's pose and board pose
/// (three quaternion multiplies of the fixed preview rotation), and on the
/// first frame (`self+0x48`) resets and plays the intro animation and
/// speaks; finally shows the rider.
#[allow(unused_labels, unused_variables, unused_assignments)]
pub fn cfe_state_char_setup_update_rider_preview(c: &mut Cpu) {
    guest(c, |c| {
        'b_182f2c: {
            c.cycles += 22;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[S7][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
            c.g[A1][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a0548, 0x00182f04)?;
            c.g[S6][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(c.g[S6][0] as u32);
            c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = 0u64;
            if t {
                break 'b_182f2c;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xcb8));
                let t = c.g[V0][0] == 0u64;

                if t {
                    break 'b_182f2c;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xcb4));
                    c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                    break 'b_182f2c;
                }
            }
        }
        'b_1831a4: {
            c.cycles += 2;
            let t = c.g[V1][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
            if t {
                break 'b_1831a4;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x4c));
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                if t {
                    break 'b_1831a4;
                } else {
                    'b_183198: {
                        c.cycles += 134;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 20);
                        c.g[AT][0] = 0x43080000;
                        c.mtc1(1, 0);
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x74));
                        c.g[AT][0] = 0xffffffffc37a0000;
                        c.mtc1(1, 1);
                        c.g[S0][0] = 0x500000;
                        c.g[AT][0] = 0xffffffffc2a40000;
                        c.mtc1(1, 2);
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xec0));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S2][0] = 0x500000;
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0xeb0));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x38));
                        c.g[S4][0] = 0x500000;
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x3c));
                        c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_sub(0xea0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sq(2, c.g[SP][0] as u32);
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x2c));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        call_ptr(c, 0x0031be50, 0x00182fb0)?;
                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x78));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x7c));
                        c.lwc1(2, (c.g[S0][0] as u32).wrapping_sub(0xec0));
                        c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x4));
                        c.mul_s(1, 0, 1);
                        c.mul_s(2, 0, 2);
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x74));
                        c.mul_s(0, 0, 4);
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x5c));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.mtc1(0, 12);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        call_ptr(c, 0x0031be50, 0x0018302c)?;
                        c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x78));
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x84));
                        c.lwc1(2, (c.g[S2][0] as u32).wrapping_sub(0xeb0));
                        c.lwc1(4, (c.g[S3][0] as u32).wrapping_add(0x4));
                        c.mul_s(1, 0, 1);
                        c.mul_s(2, 0, 2);
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x7c));
                        c.mul_s(0, 0, 4);
                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5a08));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4c));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        call_ptr(c, 0x0031be50, 0x001830a8)?;
                        c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0x8));
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.g[A0][0] = c.g[S6][0];
                        c.lwc1(3, (c.g[S4][0] as u32).wrapping_sub(0xea0));
                        c.g[A1][0] = c.g[S0][0];
                        c.lwc1(4, (c.g[S5][0] as u32).wrapping_add(0x4));
                        c.mul_s(1, 0, 1);
                        c.mul_s(3, 0, 3);
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x84));
                        c.mul_s(0, 0, 4);
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x4c));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        call_ptr(c, 0x0019f3e8, 0x00183124)?;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5a04));
                        c.g[A1][0] = c.g[S0][0];
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x3c));
                        c.g[A0][0] = c.g[S6][0];
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        call_ptr(c, 0x0019f548, 0x0018314c)?;
                        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x48));
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S6][0];
                        if t {
                            break 'b_183198;
                        } else {
                            'b_18318c: {
                                c.cycles += 3;
                                c.g[S0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                                if c.g[S0][0] == 0u64 {
                                    c.sw((c.g[S6][0] as u32).wrapping_add(0xccc), 0u64);
                                    break 'b_18318c;
                                } else {
                                    c.cycles += 10;
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x00311a50, 0x0018316c)?;
                                    c.g[AT][0] = 0xffffffffbf800000;
                                    c.mtc1(1, 12);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x1b3;
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x003128e8, 0x00183184)?;
                                    c.sw((c.g[S6][0] as u32).wrapping_add(0xccc), 0u64);
                                    c.g[A0][0] = c.g[S6][0];
                                    break 'b_18318c;
                                }
                            }
                            c.cycles += 3;
                            c.sw((c.g[S7][0] as u32).wrapping_add(0x48), 0u64);
                            call_ptr(c, 0x001a0358, 0x00183194)?;
                            c.g[A0][0] = c.g[S6][0];
                            break 'b_183198;
                        }
                    }
                    c.cycles += 3;
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0019e538, 0x001831a0)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
                    break 'b_1831a4;
                }
            }
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x120));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.pc = t;
        Ok(())
    });
}

/// Offset of the engine pointer in a UI state.
pub(crate) const CHAR_SETUP_ENGINE: u32 = 0x10;

/// Offset of the state stack inside the UI engine.
pub(crate) const CHAR_SETUP_STATE_STACK: u32 = 0x18;

/// The shared epilogue of `cFEStateCharSetup_OnWidgetAction`: 3 cycles, `ra`
/// reloaded, frame popped.
pub(crate) fn char_setup_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cFEStateCharSetup_OnWidgetAction(State* self, Item* item, int event)`
/// (0x182c80): the character-setup menu's widget handler. Event 5 / 6 ask the
/// UI factory for the next / previous state (vtable +0x20 / +0x28) and push
/// it on the engine's state stack, then hide the rider preview
/// (`SetRiderPreviewActive(self, 0)`; event 6 only when a state came back);
/// event 7 builds a `cFEStateOptions` (84 bytes, mode bits set to 1) and pushes
/// it immediately. Other events, or a null item, do nothing.
pub fn c_fe_state_char_setup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        'done: {
            if c.g[V1][0] == 0 {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = 6;
            let is6 = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = (c.g[A2][0] < 7u64) as u64;
            if is6 {
                // Event 6: previous state; a state is pushed (and the preview
                // hidden) only when the factory returned one.
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CHAR_SETUP_ENGINE));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018_2d30)?;
                c.g[A1][0] = c.g[V0][0];
                if c.g[V0][0] == 0 {
                    break 'done;
                }
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CHAR_SETUP_ENGINE));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(CHAR_SETUP_STATE_STACK));
                call_ptr(c, 0x0039_f400, 0x0018_2d44)?;
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.cycles += 2;
                let ge7 = c.g[V0][0] == 0;
                c.g[V0][0] = 0x5;
                if !ge7 {
                    c.cycles += 2;
                    if c.g[A2][0] != c.g[V0][0] {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x10);
                        char_setup_exit(c, frame);
                        return Ok(());
                    }
                    // Event 5: next state, pushed when there is one; the
                    // preview is hidden either way.
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CHAR_SETUP_ENGINE));
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018_2cf0)?;
                    c.g[A1][0] = c.g[V0][0];
                    if c.g[V0][0] == 0 {
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S0][0];
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CHAR_SETUP_ENGINE));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(CHAR_SETUP_STATE_STACK));
                        call_ptr(c, 0x0039_f400, 0x0018_2d04)?;
                        c.g[A0][0] = c.g[S0][0];
                    }
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x7;
                    let is7 = c.g[A2][0] == c.g[V0][0];
                    c.g[A1][0] = 0x46_0000;
                    if !is7 {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x10);
                        char_setup_exit(c, frame);
                        return Ok(());
                    }
                    // Event 7: a new options state, pushed immediately.
                    c.cycles += 20;
                    c.g[A3][0] = 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2968));
                    c.g[A0][0] = 0x54;
                    c.g[A2][0] = 0x100;
                    call_ptr(c, 0x0031_7d70, 0x0018_2d6c)?; // alloc(84, "FEOptions", 0x100, 0)
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CHAR_SETUP_ENGINE));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 1;
                    call_ptr(c, 0x0018_87a0, 0x0018_2d7c)?; // cFEStateOptions_ctor(mem, engine, 1)
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = 0xffff_ffff_ffff_c0ff;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0;
                    c.g[V1][0] &= c.g[A0][0];
                    c.g[A3][0] = 0;
                    c.g[V1][0] |= 0x100;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CHAR_SETUP_ENGINE));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(CHAR_SETUP_STATE_STACK));
                    call_ptr(c, 0x0039_f320, 0x0018_2da8)?; // pushSpecial(stack, st, 0, 0)
                    break 'done;
                }
            }
            // Hide the rider preview, then leave.
            c.cycles += 4;
            c.g[A1][0] = 0;
            call_ptr(c, 0x0018_31d0, 0x0018_2d50)?;
            frame.restore(c, S0, 0x10);
            char_setup_exit(c, frame);
            return Ok(());
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        char_setup_exit(c, frame);
        Ok(())
    })
}

/// `void cFEStateCharSetup::Init(cFEStateCharSetup* this)` (0x182928).
///
/// Opens the "set character" screen of the front end: looks up the screen
/// (hash of its name, 0x317670, then `cUIEngine_addScreenByHashName`, 0x397cc0)
/// into `this + 0x40` and plays its first frame, shows the rider preview
/// (`cFEStateCharSetup_InitRiderPreview`, 0x182db8), reads the player's
/// character id from the back end (interface 1) for the user number at
/// `this + 0x44`, switches the audio mode (0x28f140), and finally looks up the
/// option and title widgets by name (0x39d860) and shows, hides or enables
/// them through their vtables (the `Option_*` buttons set to 0..3, the title
/// text of player 1 or 2 depending on the user number and game path).
pub fn cfe_state_char_setup_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_18297c: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2840));
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x00182950)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x00182964)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_18297c;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x0018297c)?;
                break 'b_18297c;
            }
        }
        'b_182aa8: {
            c.cycles += 44;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x00182db8, 0x00182984)?;
            call_ptr(c, 0x0014dc80, 0x0018298c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x00182998)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00147488, 0x001829a8)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001829c0)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A2][0] = 0u64;
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a0570, 0x001829d8)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[A3][0] = 0u64;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a0508, 0x001829f4)?;
            call_ptr(c, 0x0028b180, 0x001829fc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0028f140, 0x00182a08)?;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1410));
            call_ptr(c, 0x00317670, 0x00182a14)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00182a20)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A0][0] = 0x460000;
            if t {
                break 'b_182aa8;
            } else {
                c.cycles += 10;
                call_ptr(c, 0x0014dc80, 0x00182a34)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00182a40)?;
                c.g[V1][0] = 0x530000;
                c.g[A0][0] = 0x2;
                c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                if c.g[V0][0] != c.g[A0][0] {
                    c.g[A0][0] = 0x460000;
                    break 'b_182aa8;
                } else {
                    'b_182a90: {
                        'b_182a70: {
                            c.cycles += 3;
                            c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_182a70;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[A0][0] = 0x460000;
                                if t {
                                    break 'b_182a90;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_182aa8;
                                    } else {
                                        break 'b_182a70;
                                    }
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2830));
                        call_ptr(c, 0x00317670, 0x00182a7c)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x00182a88)?;
                        let t = true;
                        c.g[A0][0] = 0x460000;
                        if t {
                            break 'b_182aa8;
                        } else {
                            break 'b_182a90;
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2810));
                    call_ptr(c, 0x00317670, 0x00182a98)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00182aa4)?;
                    c.g[A0][0] = 0x460000;
                    break 'b_182aa8;
                }
            }
        }
        'b_182ae0: {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27f0));
            call_ptr(c, 0x00317670, 0x00182ab0)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00182abc)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_182ae0;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00182adc)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                break 'b_182ae0;
            }
        }
        'b_182b20: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27e0));
            call_ptr(c, 0x00317670, 0x00182aec)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00182af8)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_182b20;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00182b18)?;
                c.g[V0][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_182b20;
            }
        }
        'b_182b60: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27d0));
            call_ptr(c, 0x00317670, 0x00182b2c)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00182b38)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_182b60;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00182b58)?;
                c.g[V0][0] = 0x2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_182b60;
            }
        }
        'b_182ba0: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27c0));
            call_ptr(c, 0x00317670, 0x00182b6c)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00182b78)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_182ba0;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00182b98)?;
                c.g[V0][0] = 0x3;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_182ba0;
            }
        }
        'b_182bf0: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27b0));
            call_ptr(c, 0x00317670, 0x00182bac)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00182bb8)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_182bf0;
            } else {
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00182bd8)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00182bf0)?;
                break 'b_182bf0;
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

/// `void cFEStateCharSetup_InitRiderPreview(cFEStateCharSetup* self)` (0x182db8): finds the
/// preview widget of the screen (`self+0x40`) by its hash name, points it at the short name
/// of the chosen character (`self+0x44`), sets up the preview camera (position/target/
/// fov in the world's camera at `world+0x7c`), activates the rider preview, marks
/// `self+0x48` and updates it.
pub fn cfe_state_char_setup_init_rider_preview(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 66;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = 0x460000;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[RA][0] = sx(0x00182ddc);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27a0));
        call_ptr(c, 0x00317670, 0x00182ddc)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x00182de8);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x00182de8)?;
        c.g[RA][0] = sx(0x00182df0);
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0014dc80, 0x00182df0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00182dfc);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014dd58, 0x00182dfc)?;
        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
        c.g[RA][0] = sx(0x00182e08);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x00147488, 0x00182e08)?;
        c.g[S0][0] = sx((c.g[V0][0] as u32) << 24);
        c.g[RA][0] = sx(0x00182e14);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0014dc80, 0x00182e14)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00182e20);
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0014dd58, 0x00182e20)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x00182e30);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014eec8, 0x00182e30)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00182e3c);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x003a0c50, 0x00182e3c)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 1);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[AT][0] = 0x43480000;
        c.mtc1(1, 0);
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.sw(c.g[SP][0] as u32, 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
        c.g[RA][0] = sx(0x00182e84);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
        call_ptr(c, 0x0015e050, 0x00182e84)?;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5a0c));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x1;
        c.g[RA][0] = sx(0x00182e98);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x10));
        Callee {
            addr: 0x001831d0,
            f: ee::s02::cFEStateCharSetup_SetRiderPreviewActive,
        }
        .call(c, 0x00182e98)?;
        c.g[V0][0] = 0x1;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x00182ea8);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
        Callee {
            addr: 0x00182ec0,
            f: ee::s02::cFEStateCharSetup_UpdateRiderPreview,
        }
        .call(c, 0x00182ea8)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateCharSetup::Shutdown(this)` (0x182c08): turns the rider preview
/// off (`SetRiderPreviewActive(this, 0)`), then runs the base `UIState`
/// shutdown (`0x39e4a0`, which deletes the state's UI list).
pub fn cfe_state_char_setup_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[A1][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0018_31d0,
            f: ee::s02::f_001831d0,
        }
        .call(c, 0x0018_2c20)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e4a0, 0x0018_2c28)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::fe::fereal::tests::oc787_install_fakes;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc774_install_fakes;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc796_install_fakes;
    pub(crate) use crate::game::sound::soundsys::tests::oc771_install_fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;

    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;

    use ssx3_ee as ee;

    use ssx3_ee::s02;

    #[test]
    fn cfe_state_char_setup_update_matches() {
        check(
            s02::f_00182c38__generated,
            cfe_state_char_setup_update,
            120,
            &|c, seed| {
                machine(c, seed, 0x182c38, 2);
                globals(c);
                clear(c, OBJ, 0x100);
                c.g[A0][0] = sx(OBJ);
                let _ = seed;
            },
        );
    }

    #[test]
    fn fe_state_rider_detail_on_widget_notify_matches() {
        check(
            ee::s02::f_00182c68__generated,
            fe_state_rider_detail_on_widget_notify,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0018_2c68, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 9,
                    1 => 0,
                    _ => seed as u64,
                };
            },
        );
    }

    #[test]
    fn cfe_state_char_setup_set_rider_preview_active_matches() {
        check(
            ee::s02::f_001831d0__generated,
            cfe_state_char_setup_set_rider_preview_active,
            40,
            &|c, seed| {
                base(c, seed, 0x18_31d0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x100) };
                // The rider lookup leaves v0 = a2 + module; keep a2 small so the
                // preview object stays inside the test data.
                c.g[A2][0] = 0;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
                c.bus.write32(
                    DATA + 0x1000 + 0x7c,
                    if seed % 4 == 0 { 0 } else { DATA + 0x1100 },
                );
                c.bus.write8(DATA + 0x44, (seed % 4) as u8);
                c.bus.write32(DATA + 0x48, 0xdead);
                c.bus.write32(DATA + 0x4c, 0xdead);
            },
        );
    }

    #[test]
    fn c_fe_state_rider_detail_ctor_matches() {
        check(
            ee::s02::f_00183238__generated,
            c_fe_state_rider_detail_ctor,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1832_38);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (1 + seed % 4) as u64;
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    // --------------------------------------------------- animation controller

    #[test]
    fn anim_controller_reset_matches() {
        check(
            ssx3_ee::s10::f_00311a50__generated,
            anim_controller_reset,
            12,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x0031_1a50);
                let anim = DATA + 0x100;
                let lists = if seed % 7 == 0 {
                    DATA + 0x2000
                } else {
                    DATA + 0x2000
                };
                clear_2(c, anim, 0x60);
                clear_2(c, lists, 0x30);
                w(c, anim + 0x50, lists);
                c.g[A0][0] = sx(anim);
                for off in [0u32, 0x18, 0x1c, 0x20, 0x30, 0x40] {
                    w(c, anim + off, r.next_u32());
                }
            },
        );
    }

    use crate::difftest::fakes;

    pub(crate) use crate::game::fe::festatecharequipdetail::tests::oc737_machine;
    pub(crate) use crate::game::fe::festatecharequipdetail::tests::oc737_queue;

    pub(crate) use crate::game::fe::festatecharequipdetail::tests::OC737_Q_RIDER;

    pub(crate) const OC737_FX_SELF: u32 = DATA + 0x21_0000;

    pub(crate) const OC737_FX_WORLD: u32 = DATA + 0x22_0000;

    pub(crate) const OC737_FX_OBJ: u32 = DATA + 0x23_0000;

    pub(crate) const OC737_FX_RIDER: u32 = DATA + 0x24_0000;

    /// `cFEStateCharSetup_UpdateRiderPreview`: the rider the manager returns
    /// (or none), whether it is loaded and compiled, the draw-3D and
    /// first-frame flags, and whether the rider has an animation object.
    #[test]
    fn cfe_state_char_setup_update_rider_preview_matches() {
        fakes::covered(
            ee::s02::f_00182ec0__generated,
            cfe_state_char_setup_update_rider_preview,
            200,
            &|c, seed| {
                let mut r = oc737_machine(c, seed, 0x0018_2ec0, 9);
                let gp = GAME_GP;
                c.bus.write32(gp.wrapping_sub(0x848), OC737_FX_WORLD);
                c.bus.write32(OC737_FX_WORLD + 0x7c, OC737_FX_OBJ);
                c.bus.write8(OC737_FX_SELF + 0x44, (r.next_u32() % 4) as u8);
                c.bus.write32(OC737_FX_SELF + 0x48, r.next_u32() & 1);
                c.bus.write32(OC737_FX_SELF + 0x4c, (seed % 3 != 0) as u32);
                let kind = if r.next_u32() % 6 == 0 {
                    12
                } else {
                    r.next_u32() % 10
                };
                c.bus.write32(OC737_FX_RIDER, kind);
                let loaded = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    OC737_FX_OBJ
                };
                c.bus.write32(OC737_FX_RIDER + 0xcb8, loaded);
                let models = if r.next_u32() % 8 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 100
                };
                c.bus.write32(OC737_FX_RIDER + 0xcb4, models);
                let anim = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    OC737_FX_OBJ
                };
                c.bus.write32(OC737_FX_RIDER + 0xc, anim);
                // The preview rotation axes and the gp-relative constants.
                for i in 0..12u32 {
                    let v = r.f32(2.0);
                    c.bus.write32(0x004f_f140 + 4 * i, v);
                }
                let (k1, k2) = (r.f32(2.0), r.f32(2.0));
                c.bus.write32(gp.wrapping_sub(0x5a08), k1);
                c.bus.write32(gp.wrapping_sub(0x5a04), k2);
                let rider = if seed % 7 == 0 { 0 } else { OC737_FX_RIDER };
                oc737_queue(c, OC737_Q_RIDER, &[rider]);
                c.g[A0][0] = sx(OC737_FX_SELF);
                c.pc = 0x0018_2ec0;
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes::covered;

    pub(crate) use crate::game::fe::fememcard::tests::oc771_mail;
    pub(crate) use crate::game::fe::fememcard::tests::oc771_miss;
    pub(crate) use crate::game::fe::fememcard::tests::OC771_BASE;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc774_mail;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc796_arm;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC774_CTRL;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC774_MAGIC;

    pub(crate) use crate::game::sound::bankmonitor::tests::OC796_OBJ;

    pub(crate) use crate::game::sound::soundsys::tests::OC771_CTRL;
    pub(crate) use crate::game::sound::soundsys::tests::OC771_MAGIC;

    /// `cFEStateCharSetup_OnWidgetAction`: events 5/6/7 and others (also with
    /// a 64-bit event word), a null item, the factory returning a state or
    /// null (vtable +0x24 / +0x2c), and the faked pushes, preview call,
    /// allocator and constructor (which may unwind).
    #[test]
    fn c_fe_state_char_setup_on_widget_action_matches() {
        oc771_install_fakes();
        const EVENTS: [u64; 12] = [
            5,
            6,
            7,
            5,
            6,
            7,
            5,
            6,
            7,
            1,
            0x1_0000_0005,
            0x8000_0000_0000_0007,
        ];
        covered(
            ee::s02::f_00182c80__generated,
            c_fe_state_char_setup_on_widget_action,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x182c80, 3);
                c.hooks.miss = oc771_miss;
                c.pc = 0x0018_2c80;
                c.bus.write32(OC771_CTRL, OC771_MAGIC);
                let this = OC771_BASE;
                let (engine, custom, vt, item) = (
                    OC771_BASE + 0x1000,
                    OC771_BASE + 0x2000,
                    OC771_BASE + 0x3000,
                    OC771_BASE + 0x4000,
                );
                c.bus.write32(this + CHAR_SETUP_ENGINE, engine);
                c.bus.write32(engine, custom);
                c.bus.write32(custom + 4, vt);
                for (slot, k) in [(0x20u32, 1u32), (0x28, 2)] {
                    c.bus.write16(vt + slot, (r.next_u32() & 0x7c) as u16);
                    c.bus.write32(vt + slot + 4, fakes::FAKE + 0x10 * k);
                }
                c.bus.write32(item + 0x18, r.next_u32());
                // The factory's next / previous state: null or a pointer.
                let state = if (seed / 12) % 2 == 0 {
                    0
                } else {
                    OC771_BASE + 0x5000
                };
                oc771_mail(c, 17, state);
                oc771_mail(
                    c,
                    18,
                    if (seed / 24) % 2 == 0 {
                        0
                    } else {
                        state | 0x10
                    },
                );
                c.g[A0][0] = this as u64;
                c.g[A1][0] = if seed % 11 == 0 { 0 } else { item as u64 };
                c.g[A2][0] = EVENTS[seed as usize % 12];
            },
        );
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc774_arm(c: &mut Cpu) {
        c.bus.write32(OC774_CTRL, OC774_MAGIC);
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc774_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc774_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc774_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc774_mail(addr), 1);
    }

    pub(crate) const OC774_OBJ: u32 = DATA + 0x1_0000;

    pub(crate) const OC774_WORLD: u32 = DATA + 0x2_0000;

    pub(crate) const OC774_MOD: u32 = DATA + 0x2_1000;

    pub(crate) const OC774_UI: u32 = DATA + 0x2_2000;

    pub(crate) const OC774_AUDIO: u32 = DATA + 0x4_0000;

    /// `cFEStateCharSetup_Init`: the user number (0, 1, other) at `+0x44`,
    /// the game-path/audio flag at 0x535c11 (2 or not), widgets found or not
    /// (the name lookup returns 0), the screen lookup result zero or not, and
    /// the unwinds of its calls. The rider preview and the audio mode switch
    /// (0x182db8, 0x28f140) are the real, unported functions.
    #[test]
    fn cfe_state_char_setup_init_matches() {
        oc774_install_fakes();
        fakes::covered(
            ee::s02::f_00182928__generated,
            cfe_state_char_setup_init,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x774c, 30);
                elf_image(c);
                oc774_arm(c);
                c.pc = 0x0018_2928;
                c.g[A0][0] = sx(OC774_OBJ);
                c.bus.write32(OC774_OBJ + 0x10, OC774_UI);
                c.bus
                    .write8(OC774_OBJ + 0x44, [0u8, 1, 2][(seed % 3) as usize]);
                c.bus
                    .write8(0x0053_5c11, if (seed / 3) % 2 == 0 { 2 } else { 0 });
                c.bus.write32(GAME_GP - 0x848, OC774_WORLD);
                c.bus.write32(OC774_WORLD + 0x7c, OC774_MOD);
                if (seed / 6) % 2 == 0 {
                    oc774_force_zero(c, 0x0039_7cc0);
                }
                if (seed / 12) % 3 == 0 {
                    oc774_force_zero(c, 0x0039_d860);
                }
                // 0x28b180 returns the audio system whose current mode
                // (`+0x6290`) is 1 (no change) or something else.
                oc774_force(c, 0x0028_b180, OC774_AUDIO);
                c.bus.write32(
                    OC774_AUDIO + 0x6290,
                    if (seed / 4) % 2 == 0 {
                        1
                    } else {
                        r.next_u32() % 12
                    },
                );
            },
        );
    }

    /// The word that arms oc787's callee fakes in a machine's RAM.
    pub(crate) const OC787_CTRL: u32 = DATA + 0x32_0000;

    pub(crate) const OC787_MAGIC: u32 = 0x0787_f4ce;

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc787_arm(c: &mut Cpu) {
        c.bus.write32(OC787_CTRL, OC787_MAGIC);
    }

    /// Object area of the fixtures (inside `DATA..DATA+0xfffff`; the fakes'
    /// pointer results land in `DATA..DATA+0x3fff0`, which the fixtures only
    /// use as opaque pointers).
    pub(crate) const OC787_OBJ: u32 = DATA + 0x1000;

    /// Fixed fixture structures above the object area.
    pub(crate) const OC787_AUX: u32 = DATA + 0xc_0000;

    /// `cFEStateCharSetup_InitRiderPreview`: the user number byte (negative,
    /// zero, other) at `+0x44`, a world whose camera is a plain record, and
    /// the unwinds of its calls (the widget text setter 0x3a0c50 is the real
    /// wrapper, its worker 0x3a0c70 a fake).
    #[test]
    fn cfe_state_char_setup_init_rider_preview_matches() {
        oc787_install_fakes();
        fakes::covered(
            ee::s02::f_00182db8__generated,
            cfe_state_char_setup_init_rider_preview,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x787a, 12);
                oc787_arm(c);
                c.pc = 0x0018_2db8;
                c.g[A0][0] = sx(OC787_OBJ);
                c.bus.write32(OC787_OBJ + 0x40, OC787_AUX);
                c.bus.write8(
                    OC787_OBJ + 0x44,
                    [0u8, 1, 2, 0xff, 0x80][(seed % 5) as usize],
                );
                c.bus.write32(OC787_OBJ + 0x48, r.next_u32());
                c.bus.write32(GAME_GP - 0x848, OC787_AUX + 0x100);
                c.bus.write32(OC787_AUX + 0x100 + 0x7c, OC787_AUX + 0x200);
                c.bus.write32(GAME_GP - 0x5a0c, r.next_u32());
            },
        );
    }

    /// `cFEStateCharSetup_Shutdown`: the rider preview call, the real
    /// `UIState_Shutdown` (its `UIList_DeleteAll` faked) and the unwinds.
    #[test]
    fn cfe_state_char_setup_shutdown_matches() {
        oc796_install_fakes();
        fakes::covered(
            ee::s02::f_00182c08__generated,
            cfe_state_char_setup_shutdown,
            90,
            &|c, seed| {
                let _ = machine(c, seed, 0x796b, 3);
                oc796_arm(c);
                c.pc = 0x0018_2c08;
                c.g[A0][0] = sx(OC796_OBJ);
            },
        );
    }
}
