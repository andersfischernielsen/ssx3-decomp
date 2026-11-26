//! `src/fe/festatecharsetup.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

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
}
