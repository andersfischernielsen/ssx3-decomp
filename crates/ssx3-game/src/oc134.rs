//! Phase 2 wave 47 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for 50 small cold-path functions (tag serialisation, UI/FE
//! widget plumbing, camera/anim/lobby helpers, memory destructors), each with
//! a differential test against the generated body.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0016_d260, camera_trigger_man_pop_trigger_stack, "CameraTriggerMan_PopTriggerStack"),(0x003a_0528, ui_text_refresh_text, "UIText_RefreshText"),(0x001c_4f58, fe_state_stats_format_time, "cFEStateStats_FormatTime"),(0x0026_5398, setup_man_host_recv_cancel, "cSetupMan_HostRecvCancel"),(0x001c_84a8, fe_popup_confirm_measure_line, "cFEPopupConfirm_MeasureLine"),
    (0x001c_8568, fe_popup_confirm_measure_line_w, "cFEPopupConfirm_MeasureLineW"),(0x0028_f200, ssx_audio_system_on_replay_begin, "SSXAudioSystem_OnReplayBegin"),(0x0014_c3c8, bolts_index_table3_by_char, "Bolts_IndexTable3ByChar"),
    (0x0014_c488, bolts_index_table4_by_char, "Bolts_IndexTable4ByChar"),(0x0034_3010, conveyor_ctor, "Conveyor_ctor"),
    (0x0034_3a68, crowd_man3d_ctor, "CrowdMan3d_ctor"),(0x0039_f400, ui_state_mgr_switch_state, "UIStateMgr_SwitchState"),(0x0015_db58, game_camera_reset_all, "GameCamera_ResetAll"),
];

// ---------------------------------------------------------------------------
// `ScriptCamController_dtor` (0x001690d0).
//
// `void ScriptCamController::~ScriptCamController()`: install the class
// vtable, release and `delete` the two script elements, free the element
// array, then run the base destructor.

pub fn script_cam_controller_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x460000;
        frame.save(c, 17, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3bd8));
        frame.save(c, 18, 0x20);
        let this = c.g[A0][0] as u32;
        frame.save(c, 19, 0x10);
        c.g[S2][0] = 0;
        frame.save(c, 16, 0x40);
        let flags = c.g[A1][0];
        frame.save_ra(c, 0);
        sw(c, this, 0x14, c.g[V0][0]);
        let mut i: u32 = 0;
        loop {
            c.cycles += 9;
            c.g[S0][0] = sx(i << 2);
            let array = lw(c, this, 0x1c) as u32;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(array));
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            call_ptr(c, 0x0028_32d8, 0x0016_9118)?;
            c.g[V0][0] = lw(c, this, 0x1c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = 0x3;
            if c.g[S0][0] != 0 {
                c.cycles += 5;
                let vt = lw(c, c.g[S0][0] as u32, 0xc);
                c.g[V0][0] = vt;
                let delta = lh(c, vt as u32, 8);
                let target = lw(c, vt as u32, 0xc);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(delta as u32));
                let t = target as u32;
                call_ptr(c, t, 0x0016_9140)?;
            }
            c.cycles += 4;
            i = i.wrapping_add(1);
            c.g[S2][0] = sx(i);
            c.g[V0][0] = (i < 2) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = lw(c, this, 0x1c);
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0016_9108);
                }
                continue;
            }
            break;
        }
        c.cycles += 3;
        let array = lw(c, this, 0x1c);
        c.g[A0][0] = array;
        if array != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0016_9164)?;
            c.g[A0][0] = sx(this);
        } else {
            c.g[A0][0] = sx(this);
        }
        c.cycles += 9;
        c.g[A1][0] = flags;
        call_ptr(c, 0x0015_cc10, 0x0016_9170)?;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
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
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        let mut i: u32 = 0;
        frame.save(c, 18, 0x10);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        let count = lw(c, this, 8);
        if (count as i64) > 0 {
            c.cycles += 2;
            c.g[A3][0] = Addr(gp(c, -0x854)).lw(c);
            loop {
                c.cycles += 14;
                c.g[V1][0] = sx(i << 2);
                c.g[A0][0] = lw(c, this, 4);
                i = i.wrapping_add(1);
                c.g[S0][0] = sx(i);
                let obj = c.g[A3][0] as u32;
                c.g[V0][0] = Addr(obj.wrapping_add(0x10d8)).lw(c);
                let vt = c.g[V0][0] as u32;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A0][0] = lh(c, vt, 0x190);
                c.g[A2][0] = lw(c, vt, 0x194);
                c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
                let t = c.g[A2][0] as u32;
                c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0039_80d0)?;
                c.g[V0][0] = lw(c, this, 8);
                c.g[V0][0] = ((i as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A3][0] = Addr(gp(c, -0x854)).lw(c);
                if more {
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x0039_80a8);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 3;
        let array = lw(c, this, 4);
        c.g[A0][0] = array;
        if array == 0 {
            c.g[A0][0] = lw(c, this, 0xc);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0039_80f4)?;
            c.g[A0][0] = lw(c, this, 0xc);
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[S2][0] & 0x1;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0039_8108)?;
            c.g[V0][0] = c.g[S2][0] & 0x1;
        }
        c.cycles += 2;
        let delete_this = c.g[V0][0] != 0;
        frame.restore(c, 16, 0x30);
        if delete_this {
            c.cycles += 3;
            c.g[A0][0] = sx(this);
            call_ptr(c, 0x0031_7e50, 0x0039_811c)?;
            frame.restore(c, 16, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `RewardsFile_Unload` (0x0015ae00).
//
// `void RewardsFile::Unload(RewardsFile* self)`: drop the open sound handle
// from the BE sound object, free the loaded data and clear the file record.

pub fn rewards_file_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0);
        let handle = lw(c, this, 0);
        let world = Addr(gp(c, -0x848)).lw(c);
        if (handle as i64) >= 0 {
            c.cycles += 9;
            c.g[A1][0] = 0;
            let snd = Addr((world as u32).wrapping_add(0x7c)).lw(c) as u32;
            let snd = snd.wrapping_add(0x1a70);
            c.g[S0][0] = sx(snd);
            c.g[A0][0] = sx(snd);
            call_ptr(c, 0x0019_dc20, 0x0015_ae34)?;
            c.g[A2][0] = lw(c, this, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0019_d4b8, 0x0015_ae44)?;
        }
        c.cycles += 3;
        let data = lw(c, this, 4);
        c.g[A0][0] = data;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if data != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0015_ae58)?;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 24;
        sw(c, this, 0x4c, 0);
        sw(c, this, 0, c.g[V0][0]);
        for off in [4u32, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x44, 0x48] {
            sw(c, this, off, 0);
        }
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIMenu_Render` (0x0039b6a0).
//
// `void UIMenu::Render(UIMenu* self, float x, float y)`: refresh the scroll
// range, then walk the item list, offsetting each item by the menu position
// and issuing its render call.

pub fn ui_menu_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x10);
        frame.save_fpr(c, 21, 0x38);
        let this = c.g[A0][0] as u32;
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(21, 12);
        frame.save(c, 16, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = lw(c, this, 0x9c);
        c.mov_s(20, 13);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            c.g[A1][0] = lbu(c, this, 0x98);
            c.g[A2][0] = lbu(c, this, 0x97);
            c.g[A3][0] = lbu(c, this, 0x96);
            call_ptr(c, 0x0039_da20, 0x0039_b6dc)?;
        }
        c.cycles += 5;
        c.g[A0][0] = sx(this.wrapping_add(0x74));
        call_ptr(c, 0x0039_7840, 0x0039_b6e4)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            frame.restore(c, 16, 0x20);
        } else {
            c.cycles += 2;
            c.g[V0][0] = lw(c, c.g[S0][0] as u32, 0x14);
            loop {
                c.cycles += 4;
                c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 6) as u32);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = lw(c, c.g[S0][0] as u32, 4);
                } else {
                    c.cycles += 10;
                    c.g[V1][0] = lw(c, c.g[S0][0] as u32, 8);
                    ldf(c, 12, this, 0x44);
                    ldf(c, 13, this, 0x48);
                    c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x88);
                    c.add_s(12, 12, 21);
                    c.add_s(13, 13, 20);
                    c.g[V0][0] = lw(c, c.g[V1][0] as u32, 0x8c);
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0039_b72c)?;
                    c.g[S0][0] = lw(c, c.g[S0][0] as u32, 4);
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039_7718, 0x0039_b738)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    frame.restore(c, 16, 0x20);
                    break;
                }
                c.g[V0][0] = lw(c, c.g[S0][0] as u32, 0x14);
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0039_b6f8);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// `lw` (sign-extended) of `a + off`.
#[inline(always)]
fn lw(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lw(c)
}

/// `sw v` to `a + off`.
#[inline(always)]
fn sw(c: &mut Cpu, a: u32, off: u32, v: u64) {
    Addr(p(a, off)).sw(c, v);
}

/// `lbu` of `a + off`.
#[inline(always)]
fn lbu(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lbu(c)
}

/// `lh` of `a + off`.
#[inline(always)]
fn lh(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lh(c)
}

/// `lb` of `a + off`.
#[inline(always)]
fn lb(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lb(c)
}

/// `lwc1 fN, a+off`.
#[inline(always)]
fn ldf(c: &mut Cpu, n: usize, a: u32, off: u32) -> FpuF32 {
    let v = Addr(p(a, off)).read_fpu(c);
    c.f[n] = v.0;
    v
}

/// FPU register `n`.
#[inline(always)]
fn fr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

// ---------------------------------------------------------------------------
// Callees (direct calls). Indirect calls go through [`call_ptr`].

mod callee {
    use super::{ee, Callee};

    pub const TAG_FIELD_COPY_TAG: Callee = Callee { addr: 0x003e_b198, f: ee::s14::f_003eb198 };
    pub const TAG_FIELD_SET_RAW: Callee = Callee { addr: 0x003e_b1e0, f: ee::s14::f_003eb1e0 };
}

// ---------------------------------------------------------------------------
// `TagFieldSetBinary` (0x003ec300).
//
// `int TagFieldSetBinary(Record* r, int len, int, const u8* src, int size)`:
// append the field name, '$', then the two-character hex encoding of every
// source byte, NUL-terminate, and store the raw text in the record.

const HEX_HIGH: u32 = 0x0049_5fc0; // D_00495FC0: high nibble -> character
const HEX_LOW: u32 = 0x0049_60c0; // D_004960C0: low nibble -> character

pub fn tag_field_set_binary(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x1150);
        frame.save(c, 18, 0x1120);
        frame.save(c, 19, 0x1110);
        let record = c.g[A0][0];
        frame.save(c, 16, 0x1140);
        let len = c.g[A1][0];
        frame.save(c, 17, 0x1130);
        frame.save_ra(c, 0x1100);
        let size = c.g[T0][0] as i64;
        let src = c.g[A3][0] as u32;
        let tmp = sp(c);
        // TagFieldCopyTag(record, tmp) writes the field name and returns its
        // end.
        callee::TAG_FIELD_COPY_TAG.call(c, 0x003e_c330)?;
        c.g[A0][0] = c.g[V0][0];
        let mut data = c.g[V0][0] as u32;
        c.g[V0][0] = 0x24;
        Addr(data).sb(c, c.g[V0][0]);
        data = data.wrapping_add(1);
        if size > 0 {
            c.cycles += 5;
            let high = HEX_HIGH;
            let low = HEX_LOW;
            let mut remaining = size as u32;
            let mut src_ptr = src;
            loop {
                c.cycles += 13;
                remaining = remaining.wrapping_sub(1);
                c.g[S1][0] = sx(remaining);
                let byte = lbu(c, src_ptr, 0) as u32;
                c.g[V0][0] = sx(byte);
                c.g[V0][0] = sx(byte.wrapping_add(high));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                Addr(data).sb(c, c.g[V1][0]);
                data = data.wrapping_add(1);
                let byte = lbu(c, src_ptr, 0) as u32;
                c.g[V0][0] = sx(byte);
                src_ptr = src_ptr.wrapping_add(1);
                c.g[S0][0] = sx(src_ptr);
                c.g[V0][0] = sx(byte.wrapping_add(low));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                Addr(data).sb(c, c.g[V1][0]);
                data = data.wrapping_add(1);
                c.g[A0][0] = sx(data);
                if (remaining as i32) > 0 {
                    if c.cycles >= c.next_event {
                        c.pc = 0x003e_c358;
                        c.event();
                    }
                } else {
                    break;
                }
            }
        }
        c.cycles += 12;
        Addr(data as u32).sb(c, 0);
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx(sp(c));
        callee::TAG_FIELD_SET_RAW.call(c, 0x003e_c3a0)?;
        frame.restore(c, 16, 0x1140);
        frame.restore(c, 17, 0x1130);
        frame.restore(c, 18, 0x1120);
        frame.restore(c, 19, 0x1110);
        frame.restore_ra(c, 0x1100);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStatePeakRoom_OnWidgetNotify` (0x001d3e08).
//
// `int cFEStatePeakRoom::OnWidgetNotify(cFEState* self, UIWidget* item,
//  int key, int id)`: keys 8 and 9 report "handled with 0x100"; key 6 routes
// the selection into the lodge screen and reports 0x10/0x101 by whether the
// comparison succeeded; everything else reports 0x101.

pub fn fe_state_peak_room_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 6;
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        let this = c.g[A0][0] as u32;
        frame.save(c, 19, 0x10);
        let extra = c.g[A3][0];
        frame.save(c, 16, 0x40);
        let item = c.g[A1][0] as u32;
        let key = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(this);
        c.g[S2][0] = extra;
        c.g[S3][0] = sx(item);
        let mut result = 0u64;
        if key == 6 {
            c.cycles += 18;
            call_ptr(c, 0x0014_dc80, 0x001d_3e64)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xd;
            call_ptr(c, 0x0014_dd58, 0x001d_3e70)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = sx(item.wrapping_add(0x74));
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x0039_7870, 0x001d_3e80)?;
            c.g[A3][0] = lw(c, this, 0xe0);
            c.g[A0][0] = c.g[S0][0];
            c.g[T0][0] = lw(c, c.g[V0][0] as u32, 0x18);
            c.g[A1][0] = lb(c, this, 0x44);
            c.g[A2][0] = lw(c, this, 0xdc);
            call_ptr(c, 0x0015_7bf0, 0x001d_3e98)?;
            c.g[V1][0] = c.g[V0][0];
            let zero = c.g[V1][0] == 0;
            result = 0x10;
            if !zero {
                c.cycles += 1;
                result = 0x101;
            }
        } else if key < 6 {
            c.cycles += 3;
            result = 0x101;
        } else if key < 10 {
            c.cycles += 3;
            if key < 8 {
                c.cycles += 2;
                result = 0x101;
            } else {
                c.cycles += 4;
                result = 0x100;
            }
        } else {
            c.cycles += 4;
            result = 0x101;
        }
        c.g[V0][0] = result;
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `CameraTriggerMan_PopTriggerStack` (0x0016d260).
//
// `int CameraTriggerMan::PopTriggerStack(TriggerStack* s, int key, int* changed,
//  int* out)`: remove `key` from the stack; report whether the top changed.

pub fn camera_trigger_man_pop_trigger_stack(c: &mut Cpu) {
    c.cycles += 3;
    let stack = c.g[A0][0] as u32;
    let key = c.g[A1][0];
    let changed = c.g[A2][0] as u32;
    let out = c.g[A3][0] as u32;
    let mut t1 = lw(c, stack, 0);
    c.g[T1][0] = t1;
    c.g[T0][0] = 0;
    if t1 != 0 {
        c.cycles += 3;
        let mut i: u32 = 0;
        let mut v1 = stack.wrapping_add(8);
        let t2 = v1;
        c.g[V1][0] = sx(v1);
        c.g[T2][0] = sx(t2);
        loop {
            c.cycles += 3;
            let v0 = lw(c, v1, 0);
            if v0 != key {
                t1 = lw(c, stack, 0);
                c.g[T1][0] = t1;
            } else {
                c.cycles += 4;
                let mut j = i.wrapping_add(1);
                c.g[A1][0] = sx(j);
                let in_range = (j as u64) < t1;
                c.g[V0][0] = in_range as u64;
                c.g[V0][0] = sx(j << 2);
                if in_range {
                    c.cycles += 1;
                    v1 = (j << 2).wrapping_add(t2);
                    c.g[V1][0] = sx(v1);
                    loop {
                        c.cycles += 7;
                        let v = lw(c, v1, 0);
                        j = j.wrapping_add(1);
                        c.g[A1][0] = sx(j);
                        sw(c, v1.wrapping_sub(4), 0, v);
                        c.g[V0][0] = lw(c, stack, 0);
                        c.g[V0][0] = ((j as u64) < c.g[V0][0]) as u64;
                        let more = c.g[V0][0] != 0;
                        v1 = v1.wrapping_add(4);
                        c.g[V1][0] = sx(v1);
                        if !more {
                            break;
                        }
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x0016_d298);
                        }
                    }
                }
                c.g[A1][0] = sx(i.wrapping_add(1));
                c.cycles += 4;
                c.g[V0][0] = lw(c, stack, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                sw(c, stack, 0, c.g[V0][0]);
                break;
            }
            c.cycles += 4;
            c.g[T0][0] = sx(i.wrapping_add(1));
            i = i.wrapping_add(1);
            c.g[V0][0] = ((i as u64) < c.g[T1][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(v1.wrapping_add(4));
            v1 = v1.wrapping_add(4);
            if !more {
                break;
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0016_d278);
            }
        }
    }
    c.cycles += 3;
    let count = lw(c, stack, 0);
    let v0 = sx((count as u32).wrapping_sub(1));
    if count == 0 {
        c.cycles += 3;
        sw(c, changed, 0, 0);
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    let top = lw(c, stack, 4);
    c.cycles += 6;
    let slot = (v0 as u32) << 2;
    let addr = (stack as u64).wrapping_add(slot as u64);
    let item = lw(c, addr as u32, 8);
    c.g[A0][0] = item;
    c.g[V1][0] = top;
    c.g[V0][0] = 1;
    if item != top {
        c.cycles += 2;
        sw(c, changed, 0, 1);
        sw(c, out, 0, item);
    } else {
        c.cycles += 2;
        sw(c, changed, 0, 0);
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// `ScriptMovie_Open` (0x002838e8).
//
// `int ScriptMovie_Open(ScriptMovie* self, int file)`: if a movie is already
// open, report 0; otherwise allocate a video object, set up its parameters and
// start playback (0 on failure, 1 on success).

pub fn script_movie_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0x30);
        let file = c.g[A1][0];
        c.g[S1][0] = file;
        c.g[S0][0] = sx(this);
        let mut result;
        if lw(c, this, 0) != 0 {
            c.cycles += 2;
            result = 0;
        } else {
            c.cycles += 25;
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = 0x100;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1fc8));
            c.g[A3][0] = 0;
            c.g[A0][0] = 0x44;
            call_ptr(c, 0x0031_7d70, 0x0028_3928)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0025_33c8, 0x0028_3930)?;
            sw(c, this, 0, c.g[V0][0]);
            c.g[A0][0] = sx(sp(c));
            call_ptr(c, 0x0025_3390, 0x0028_393c)?;
            c.g[A0][0] = lw(c, this, 0);
            c.g[V0][0] = 0x1;
            c.g[V1][0] = 0x100;
            c.g[A2][0] = 0x9;
            sw(c, sp(c), 0, c.g[S1][0]);
            c.g[A1][0] = sx(sp(c));
            sw(c, sp(c), 8, c.g[V0][0]);
            sw(c, sp(c), 0x18, c.g[V1][0]);
            sw(c, sp(c), 0x1c, c.g[A2][0]);
            sw(c, sp(c), 0xc, 0);
            sw(c, sp(c), 0x10, 0);
            call_ptr(c, 0x0025_34a8, 0x0028_396c)?;
            if c.g[V0][0] != 0 {
                result = 1;
            } else {
                c.cycles += 3;
                let m = lw(c, this, 0);
                c.g[A0][0] = m;
                if m == 0 {
                    sw(c, this, 0, 0);
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = 3;
                    call_ptr(c, 0x0025_3418, 0x0028_3988)?;
                    sw(c, this, 0, 0);
                }
                c.cycles += 1;
                result = 0;
            }
        }
        c.g[V0][0] = result;
        c.cycles += 5;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `AnimController_PlayAnimDirect` (0x003129e0).
//
// `void AnimController::PlayAnimDirect(AnimController* self, AnimTable* table,
//  int anim, AnimSeqList* seq, float speed, int flags)`: resolve the table
// entry, look up its bank, play the animation and record the sequence.

pub fn anim_controller_play_anim_direct(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        let table = c.g[A1][0];
        frame.save(c, 20, 0x10);
        let this = c.g[A0][0];
        frame.save_fpr(c, 20, 0x60);
        let seq = c.g[A2][0];
        frame.save(c, 19, 0x20);
        c.mov_s(20, 12);
        frame.save(c, 16, 0x50);
        frame.save_ra(c, 0);
        c.g[S1][0] = table;
        c.g[S2][0] = this;
        c.g[S4][0] = seq;
        c.g[A0][0] = table;
        call_ptr(c, 0x0031_15c8, 0x0031_2a18)?;
        c.mtc1(0, 0);
        c.c_lt_s(20, 0);
        let entry = lw(c, c.g[V0][0] as u32, 0xc);
        c.g[S3][0] = entry;
        if !c.fcc() {
            c.cycles += 3;
            c.g[A0][0] = table;
            call_ptr(c, 0x0031_15c8, 0x0031_2a34)?;
            ldf(c, 20, c.g[V0][0] as u32, 0x10);
        }
        c.cycles += 25;
        c.g[V0][0] = 0x490000;
        let idx = c.g[S3][0] as u32;
        c.g[S0][0] = sx(idx << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x27f8));
        c.g[V1][0] = lw(c, this as u32, 0x50);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = sx(idx << 3);
        c.g[T1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S4][0];
        c.mov_s(12, 20);
        c.g[A0][0] = sx(this as u32);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x0031_1f00, 0x0031_2a70)?;
        c.g[S0][0] = sx((this as u32).wrapping_add(c.g[S0][0] as u32));
        frame.restore(c, 17, 0x40);
        sw(c, c.g[S0][0] as u32, 0, c.g[V0][0]);
        frame.restore(c, 16, 0x50);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIText_RefreshText` (0x003a0528).
//
// `void UIText::RefreshText(UIText* self)`: resolve the current string and
// either re-wrap it or pass it to the wide-string setter.

pub fn ui_text_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[S0][0] = sx(this);
        c.g[A1][0] = lw(c, this, 0xb0);
        let string = c.g[A1][0];
        // The two branches that return early (no text or no owner) share this
        // body; the rest of the function only runs for a live string.
        let mut done = false;
        if string == 0 {
            done = true;
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw(c, this, 0x5c);
            c.g[V1][0] = lw(c, c.g[V0][0] as u32, 0xd0);
            c.g[A0][0] = lw(c, c.g[V1][0] as u32, 0x10);
            c.g[A2][0] = lw(c, c.g[A0][0] as u32, 0x10);
            if c.g[A2][0] == 0 {
                done = true;
            } else {
                c.cycles += 8;
                c.g[V1][0] = lw(c, c.g[A2][0] as u32, 4);
                c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x20);
                c.g[V0][0] = lw(c, c.g[V1][0] as u32, 0x24);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x003a_0570)?;
                c.g[A1][0] = c.g[V0][0];
                if c.g[A1][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, this, 8);
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = lw(c, this, 0x14);
                    c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 7) as u32);
                    c.g[V0][0] = c.g[V0][0] & 1;
                    let wrapped = c.g[V0][0] != 0;
                    let wide = if wrapped {
                        true
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = lw(c, this, 0x74);
                        c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 6) as u32);
                        c.g[V0][0] = c.g[V0][0] & 1;
                        c.g[V0][0] != 0
                    };
                    if wide {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x003a_0d00, 0x003a_05ac)?;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x003a_0eb0, 0x003a_05bc)?;
                    }
                    c.g[V0][0] = lw(c, this, 8);
                }
                // Common tail: notify the owner through its virtual slot.
                c.cycles += 6;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = lh(c, c.g[V0][0] as u32, 0x28);
                c.g[V1][0] = lw(c, c.g[V0][0] as u32, 0x2c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x003a_05d4)?;
                done = true;
            }
        }
        let _ = done;
        frame.restore(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `FEModule_LoadAnimData` (0x001a2b00).
//
// `void cRealFE::loadCharAnimations()`: load the character-animation table
// file and install its (single) entry.

pub fn fe_module_load_anim_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[A0][0] = 0x460000;
        frame.save(c, 16, 0x40);
        c.g[A1][0] = 0;
        frame.save(c, 17, 0x30);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1330));
        frame.save(c, 18, 0x20);
        c.g[A2][0] = 0x64;
        frame.save_ra(c, 0x10);
        c.g[A3][0] = sx(sp(c));
        c.g[S0][0] = 0;
        call_ptr(c, 0x003d_ed50, 0x001a_2b30)?;
        c.g[S2][0] = 0x460000;
        c.g[S1][0] = sx(gp(c, -0x1730));
        c.g[A2][0] = 0;
        loop {
            c.cycles += 20;
            c.g[A3][0] = 0;
            c.g[A0][0] = 0x18;
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1348));
            call_ptr(c, 0x0031_7d70, 0x001a_2b50)?;
            c.g[A1][0] = lw(c, c.g[S1][0] as u32, 0);
            c.g[A0][0] = c.g[V0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            call_ptr(c, 0x0031_1250, 0x001a_2b60)?;
            c.g[V1][0] = Addr(gp(c, 0xd08)).lw(c);
            c.g[A0][0] = c.g[S0][0] & 0xff;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = Addr(gp(c, 0xd8c)).lw(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            sw(c, c.g[V1][0] as u32, 0, c.g[V0][0]);
            call_ptr(c, 0x0031_4f30, 0x001a_2b88)?;
            let more = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = 0;
            if more {
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x001a_2b40);
                }
                continue;
            }
            break;
        }
        c.cycles += 11;
        c.g[A0][0] = Addr(gp(c, 0xd8c)).lw(c);
        call_ptr(c, 0x0031_50b0, 0x001a_2b98)?;
        c.g[A0][0] = lw(c, sp(c), 0);
        c.g[A1][0] = 0x64;
        call_ptr(c, 0x003d_edc0, 0x001a_2ba4)?;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `NetFEState_OnSessionLostPopupEvent` (0x001a9578).
//
// `int NetFEState::OnSessionLostPopupEvent(NetFEState* self, int, int key)`:
// keys 0xf/0x14 close the messenger and move on to the chat/online menu,
// everything else forwards to the default child handler.

pub fn net_fe_state_on_session_lost_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[S0][0] = sx(this);
        let key = c.g[A2][0];
        if key == 0xf || key == 0x14 {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a_93d0, 0x001a_95a4)?;
            c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
            call_ptr(c, 0x0025_da90, 0x001a_95ac)?;
            let first = c.g[V0][0] == 0;
            c.g[V0][0] = Addr(gp(c, 0x1d98)).lw(c);
            let mut alloc_online = first;
            if !first {
                c.cycles += 2;
                alloc_online = c.g[V0][0] != 0;
                c.g[A1][0] = 0x460000;
            }
            if first {
                c.cycles += 1;
                c.g[A1][0] = 0x460000;
            }
            if alloc_online {
                c.cycles += 9;
                c.g[A0][0] = 0x6d0;
                c.g[A1][0] = sx(0x460000u32.wrapping_sub(0xd8));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x001a_9600)?;
                c.g[A1][0] = lw(c, this, 0x10);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x001b_fbb8, 0x001a_960c)?;
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.cycles += 11;
                c.g[A0][0] = 0x73c;
                c.g[A1][0] = sx(0x460000u32.wrapping_sub(0xf0));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x001a_95d4)?;
                c.g[A1][0] = lw(c, this, 0x10);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x001b_1f88, 0x001a_95e0)?;
                c.g[A0][0] = c.g[S0][0];
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x001a_8a40, 0x001a_9618)?;
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a_93d0, 0x001a_9628)?;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEUseExistingAccount_OnAbortPopupEvent` (0x001aca88).
//
// `int cFEUseExistingAccount::OnAbortPopupEvent(self, widget, key)`: 0xf/0x14
// close the network messenger popups, everything else goes to the default
// child event handler.

pub fn fe_use_existing_account_on_abort_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        let key = c.g[A2][0];
        frame.save(c, 18, 0x10);
        let this = c.g[A0][0];
        frame.save_ra(c, 0);
        let widget = c.g[A1][0];
        c.g[S2][0] = widget;
        c.g[S1][0] = this;
        c.g[S0][0] = key;
        if key == 0xf || key == 0x14 {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                call_ptr(c, 0x0026_0f80, 0x001a_cad0)?;
            }
            c.cycles += 4;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            call_ptr(c, 0x0026_1460, 0x001a_cad8)?;
            let t = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            if !t {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                call_ptr(c, 0x0026_13c8, 0x001a_cae8)?;
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                call_ptr(c, 0x0025_b800, 0x001a_cafc)?;
            }
            c.cycles += 10;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            call_ptr(c, 0x0026_1408, 0x001a_cb04)?;
            c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
            call_ptr(c, 0x0025_b800, 0x001a_cb0c)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x001a_97b8, 0x001a_cb1c)?;
            frame.restore(c, 16, 0x30);
        } else {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x001a_97b8, 0x001a_cb30)?;
            frame.restore(c, 16, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateLogin_v33` (0x001bf378): the same popup-close sequence as
// `cFEUseExistingAccount::OnAbortPopupEvent`.
pub fn fe_state_login_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        let key = c.g[A2][0];
        frame.save(c, 18, 0x10);
        let this = c.g[A0][0];
        frame.save_ra(c, 0);
        let widget = c.g[A1][0];
        c.g[S2][0] = widget;
        c.g[S1][0] = this;
        c.g[S0][0] = key;
        if key == 0xf || key == 0x14 {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                call_ptr(c, 0x0026_0f80, 0x001b_f3c0)?;
            }
            c.cycles += 4;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            call_ptr(c, 0x0026_1460, 0x001b_f3c8)?;
            let t = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            if !t {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                call_ptr(c, 0x0026_13c8, 0x001b_f3d8)?;
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                call_ptr(c, 0x0025_b800, 0x001b_f3ec)?;
            }
            c.cycles += 10;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            call_ptr(c, 0x0026_1408, 0x001b_f3f4)?;
            c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
            call_ptr(c, 0x0025_b800, 0x001b_f3fc)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x001a_97b8, 0x001b_f40c)?;
            frame.restore(c, 16, 0x30);
        } else {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x001a_97b8, 0x001b_f420)?;
            frame.restore(c, 16, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateStats_FormatTime` (0x001c4f58).
//
// `void FormatTime(void* self, char* buf, int frames)`: format a frame count
// as "h:mm:ss.mmm" (or "m:ss.mmm" when under an hour).

pub fn fe_state_stats_format_time(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.mtc1(6, 0);
        c.cvt_s_w(0, 0);
        ldf(c, 1, gp(c, -0x590c), 0);
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x3c;
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x3c;
        c.mul_s(0, 0, 1);
        c.cycles += 22;
        c.g[AT][0] = 0x447a0000;
        c.mtc1(1, 2);
        let buf = c.g[A1][0];
        c.g[T2][0] = buf;
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[T0][0] = c.g[V0][0];
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.div(0, 8, 3);
        c.sub_s(0, 0, 1);
        c.mul_s(0, 0, 2);
        c.cvt_w_s(1, 0);
        c.mfc1(9, 1);
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = c.g[A0][0];
        c.div(0, 7, 3);
        c.g[A2][0] = c.lo[0];
        c.mult(0, 2, 7, 3);
        c.mult(0, 3, 6, 3);
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        let hours = c.g[A2][0] as i64;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        if hours > 0 {
            c.cycles += 6;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = c.g[T2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f00));
            call_ptr(c, 0x0041_7828, 0x001c_4fe8)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 8;
            c.g[A2][0] = c.g[A3][0];
            c.g[A1][0] = 0x460000;
            c.g[A3][0] = c.g[T0][0];
            c.g[A0][0] = c.g[T2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f18));
            c.g[T0][0] = c.g[T1][0];
            call_ptr(c, 0x0041_7828, 0x001c_500c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MessageCenter_GetIndexInType` (0x001e2dc0).
//
// `int MessageCenter::GetIndexInType(int type, int id)`: walk the message
// type's entries to the one holding `id` and return its running index.

pub fn message_center_get_index_in_type(c: &mut Cpu) {
    c.cycles += 15;
    let arg = c.g[A0][0];
    c.g[A3][0] = arg;
    c.g[V1][0] = 0x18;
    c.mult(0, 3, 7, 3);
    c.g[V0][0] = 0x4c0000;
    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c08));
    c.g[A0][0] = 0x14;
    c.g[V0][0] = 0x440000;
    c.g[T1][0] = 0;
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
    c.g[T0][0] = 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = lw(c, c.g[V1][0] as u32, 4);
    c.mult(0, 2, 2, 4);
    let negative = (c.g[A3][0] as i64) < 0;
    c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    if !negative {
        c.cycles += 5;
        c.g[T5][0] = c.g[A2][0];
        c.g[T6][0] = c.g[A1][0];
        c.g[T4][0] = 0x18;
        c.g[T3][0] = 0x14;
        loop {
            let found = 'found: {
                c.cycles += 12;
                c.mult(0, 3, 8, 12);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T5][0] as u32));
                c.g[V1][0] = lw(c, c.g[V0][0] as u32, 4);
                c.mult(0, 2, 3, 11);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T6][0] as u32));
                c.g[A0][0] = lw(c, c.g[V1][0] as u32, 4);
                c.g[A2][0] = lw(c, c.g[V1][0] as u32, 8);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = lw(c, c.g[V1][0] as u32, 0xc);
                    break 'found true;
                }
                c.cycles += 4;
                c.g[V1][0] = lw(c, c.g[V1][0] as u32, 0xc);
                c.g[V0][0] = lw(c, c.g[T2][0] as u32, 0xc);
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                if t {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(1));
                    abi::ret(c);
                    return;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
                    c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    break 'found false;
                }
            };
            let _ = found;
            c.cycles += 7;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V1][0] = lw(c, c.g[T2][0] as u32, 0xc);
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
            c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
            let done = c.g[A1][0] == 0;
            if c.g[V0][0] == 0 {
                c.g[T1][0] = c.g[A0][0];
            }
            if done {
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x001e_2e10);
                }
            } else {
                break;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(1));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// `OVScreenMultiplayerStandings_Init` (0x001e8f00).
//
// `void OVScreenMultiplayerStandings::Init(self)`: build the standings and
// header widgets and hook up the audio cue.

pub fn ov_screen_multiplayer_standings_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 17, 0x10);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0);
        frame.save(c, 16, 0x20);
        c.g[A0][0] = sx(0x46e9c0u32);
        let s0 = lw(c, this, 0x10);
        call_ptr(c, 0x0031_7670, 0x001e_8f24)?;
        c.g[A0][0] = s0;
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = sx(this);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001e_8f38)?;
        let empty = c.g[V0][0] == 0;
        sw(c, this, 0x40, c.g[V0][0]);
        if !empty {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001e_8f50)?;
        }
        c.cycles += 11;
        c.g[A0][0] = sx(0x46e378u32);
        let s0 = lw(c, this, 0x10);
        call_ptr(c, 0x0031_7670, 0x001e_8f60)?;
        c.g[A0][0] = s0;
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = sx(this);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001e_8f74)?;
        c.g[A0][0] = c.g[V0][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001e_8f88)?;
        }
        c.cycles += 14;
        c.g[A0][0] = sx(this);
        call_ptr(c, 0x001e_8fc0, 0x001e_8f90)?;
        c.g[A0][0] = sx(this);
        call_ptr(c, 0x0020_a380, 0x001e_8f98)?;
        call_ptr(c, 0x0028_b180, 0x001e_8fa0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x0029_4f78, 0x001e_8fac)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateNetMessenger_SubmitKeyboardInput` (0x001a4750).
//
// `void cFEStateNetMessenger::SubmitKeyboardInput(self)`: if the messenger
// is in the "typing" or "waiting" state, submit the text through the active
// dialog.

pub fn fe_state_net_messenger_submit_keyboard_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x1;
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(this);
        c.g[A1][0] = lw(c, this, 0x69c);
        c.g[V1][0] = lw(c, c.g[A1][0] as u32, 0x18);
        let kind = c.g[V1][0];
        if kind == 1 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
            call_ptr(c, 0x001a_4810, 0x001a_4794)?;
            frame.restore(c, 16, 0x20);
            c.cycles += 1;
            frame.restore(c, 17, 0x10);
        } else if kind == 2 {
            c.cycles += 2;
            frame.restore(c, 16, 0x20);
            c.cycles += 3;
            c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                call_ptr(c, 0x0026_0f80, 0x001a_47b0)?;
            }
            c.cycles += 4;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            call_ptr(c, 0x0026_1460, 0x001a_47b8)?;
            let t = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            if !t {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                call_ptr(c, 0x0026_13c8, 0x001a_47c8)?;
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
            }
            c.cycles += 2;
            let t = c.g[V0][0] != 0;
            c.g[S0][0] = Addr(gp(c, 0x238)).lw(c);
            if !t {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                call_ptr(c, 0x0025_b800, 0x001a_47dc)?;
                c.g[S0][0] = Addr(gp(c, 0x238)).lw(c);
            }
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001a_98b0, 0x001a_47e8)?;
            c.g[A2][0] = lw(c, this, 0x69c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x74));
            call_ptr(c, 0x0026_2690, 0x001a_47fc)?;
            frame.restore(c, 16, 0x20);
            c.cycles += 1;
            frame.restore(c, 17, 0x10);
        } else {
            c.cycles += 2;
            frame.restore(c, 16, 0x20);
            c.cycles += 2;
            frame.restore(c, 17, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `LobbyMan_CreateRoom` (0x0025dac0).
//
// `void LobbyMan_CreateRoom(LobbyMan* self, char* name, int players, int mode)`:
// format the room name, append the optional password and send the create
// request.

pub fn lobby_man_create_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x240);
        let v1 = c.g[A2][0];
        frame.save(c, 17, 0x220);
        frame.save(c, 18, 0x210);
        let mode = c.g[A3][0];
        frame.save(c, 16, 0x230);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0x200);
        c.g[S2][0] = sx(this);
        c.g[S1][0] = mode;
        c.g[V0][0] = lw(c, this, 0);
        let name = c.g[A1][0];
        c.g[A0][0] = name;
        if c.g[V0][0] != 0 {
            c.cycles += 16;
            c.g[A2][0] = c.g[A0][0];
            c.g[S0][0] = sx(sp(c).wrapping_add(0x100));
            c.g[A1][0] = 0x4a0000;
            c.g[A3][0] = v1;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3150));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0041_7828, 0x0025_db08)?;
            c.g[A2][0] = 0x4a0000;
            c.g[A3][0] = c.g[S0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f58));
            Addr(sp(c)).sb(c, 0);
            c.g[A0][0] = sx(sp(c));
            c.g[A1][0] = 0x100;
            call_ptr(c, 0x003e_c180, 0x0025_db24)?;
            if c.g[S1][0] != 0 {
                c.cycles += 5;
                c.g[A3][0] = c.g[S1][0];
                c.g[A2][0] = sx(0x4a0000u32.wrapping_add(0x3040));
                c.g[A0][0] = sx(sp(c));
                c.g[A1][0] = 0x100;
                call_ptr(c, 0x003e_c180, 0x0025_db40)?;
            }
            c.cycles += 10;
            c.g[T0][0] = 0x270000;
            c.g[A2][0] = 0x726f0000;
            c.g[AT][0] = 0x41a00000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S2][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x72a0));
            c.g[A1][0] = 0x11;
            c.g[A2][0] = c.g[A2][0] | 0x6f6d;
            c.g[A3][0] = sx(sp(c));
            call_ptr(c, 0x0025_b608, 0x0025_db68)?;
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x230);
        frame.restore(c, 17, 0x220);
        frame.restore(c, 18, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `LobbyMan_GetErrorString` (0x00260e50).
//
// `char* LobbyMan_GetErrorString(LobbyMan* self, char* buf, int code)`:
// decode the packed error/status word into text.

const MSG_LOBBY_DUPLICATE: u32 = 0x6475_706c; // "udpl"
const MSG_LOBBY_ITEM: u32 = 0x7469_6d65; // "item"

pub fn lobby_man_get_error_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x6475_0000;
        frame.save(c, 16, 0x20);
        c.g[T1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.g[V0][0] | 0x706c;
        let t = c.g[T1][0] == c.g[V0][0];
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        if t {
            c.cycles += 6;
            c.g[A1][0] = 0x480000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe10));
            call_ptr(c, 0x0031_82d8, 0x0026_0e98)?;
            c.g[V0][0] = c.g[S0][0];
        } else if c.g[T1][0] == 0x7469_6d65 {
            c.cycles += 4;
            c.cycles += 6;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3320));
            call_ptr(c, 0x0031_82d8, 0x0026_0eb0)?;
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 4;
            c.cycles += 2;
            c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
            c.cycles += 18;
            c.g[T0][0] = sx((c.g[T1][0] as u32) >> 16);
            c.g[A1][0] = 0x480000;
            c.g[T1][0] = sx((c.g[T1][0] as u32) >> 8);
            c.g[A0][0] = sx(sp(c));
            c.g[T0][0] = c.g[T0][0] & 0xff;
            c.g[T1][0] = c.g[T1][0] & 0xff;
            c.g[A3][0] = sx((c.g[A2][0] as u32) >> 24);
            c.g[T2][0] = c.g[A2][0] & 0xff;
            sw(c, sp(c), 0, c.g[V0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe20));
            call_ptr(c, 0x0031_94a8, 0x0026_0ee4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(sp(c));
            call_ptr(c, 0x0031_7fe8, 0x0026_0ef0)?;
            c.g[A0][0] = sx(sp(c));
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x0031_8260, 0x0026_0efc)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cSetupMan_HostRecvCancel` (0x00265398).
//
// `void cSetupMan_HostRecvCancel(cSetupMan* self, Packet* out)`: acknowledge a
// cancelled join and pass the queue on.

pub fn setup_man_host_recv_cancel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x3;
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save_ra(c, 0x20);
        let this = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(this).lw(c);
        c.g[A1][0] = 0x480000;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 2;
            c.g[V0][0] = 0x2;
            sw(c, this, 0, c.g[V0][0]);
        }
        c.cycles += 24;
        c.g[S0][0] = Addr(gp(c, -0xc8)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0x8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_53d8)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0xf7;
        c.g[V0][0] = 0x480000;
        sw(c, c.g[S1][0] as u32, 0, c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = 0x480000;
        sw(c, c.g[S1][0] as u32, 4, c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        let s0 = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(s0.wrapping_add(0xf0)).lw(c);
        c.g[A3][0] = 0;
        sw(c, sp(c), 0x10, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_5410)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0 {
            sw(c, c.g[A1][0] as u32, 8, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = lw(c, sp(c), 0x10);
        frame.restore(c, 16, 0x40);
        sw(c, c.g[A1][0] as u32, 0, c.g[V0][0]);
        frame.restore(c, 17, 0x30);
        c.g[V1][0] = lw(c, c.g[V0][0] as u32, 4);
        frame.restore_ra(c, 0x20);
        sw(c, c.g[A1][0] as u32, 4, c.g[V1][0]);
        c.g[A0][0] = lw(c, c.g[V0][0] as u32, 4);
        sw(c, c.g[A0][0] as u32, 0, c.g[A1][0]);
        c.g[V0][0] = lw(c, sp(c), 0x10);
        sw(c, c.g[V0][0] as u32, 4, c.g[A1][0]);
        sw(c, sp(c), 0, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `InSeq_EventTrackStart` (0x00273208).
//
// `int InSeq_EventTrackStart(EventTrack* t, void* seq, void* source, void* owner)`:
// start firing the sequence's keys.

pub fn in_seq_event_track_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(this);
        let seq = c.g[A1][0];
        let source = c.g[A2][0];
        c.g[S1][0] = c.g[A3][0];
        sw(c, this, 8, seq);
        sw(c, this, 0xc, source);
        sw(c, this, 0x10, seq);
        c.g[V0][0] = Addr(seq as u32).lhu(c);
        let empty = c.g[V0][0] == 0;
        if !empty {
            c.cycles += 12;
            c.g[V0][0] = lw(c, source as u32, 8);
            c.g[A1][0] = c.lhu((seq as u32).wrapping_add(2));
            c.g[A0][0] = lw(c, c.g[V0][0] as u32, 8);
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
            call_ptr(c, 0x0027_2308, 0x0027_324c)?;
            c.g[V1][0] = lw(c, this, 0x10);
            sw(c, this, 0x1c, c.g[V0][0]);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            sw(c, this, 0x18, 0);
            let ok = c.g[V0][0] != 0;
            sw(c, this, 0x14, c.g[V0][0]);
            if ok {
                c.cycles += 3;
                c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(6));
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = lw(c, this, 0x14);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = lw(c, c.g[S1][0] as u32, 0);
                    loop {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[V0][0] = c.g[V0][0] | 0x10;
                        sw(c, c.g[S1][0] as u32, 0, c.g[V0][0]);
                        c.g[A1][0] = lw(c, this, 0x14);
                        call_ptr(c, 0x0027_3418, 0x0027_328c)?;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0027_3468, 0x0027_3294)?;
                        c.g[V0][0] = lw(c, this, 0x14);
                        let done = c.g[V0][0] == 0;
                        sw(c, this, 0x24, 0);
                        if done {
                            break;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(2));
                        if c.g[V0][0] != 0 {
                            break;
                        }
                        c.g[V0][0] = lw(c, c.g[S1][0] as u32, 0);
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x0027_3278);
                        }
                    }
                }
            }
        }
        // The early (no-key) path re-reads the current key.
        c.cycles += 1;
        c.g[V0][0] = lw(c, this, 0x14);
        c.cycles += 6;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `ChaseCamController_BeginScriptCamera` (0x00161fa0).
//
// `void ChaseCamController::BeginScriptCamera(self, int state)`: latch the
// new script-camera state and install the matching algorithm.

pub fn chase_cam_controller_begin_script_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0x4c;
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(this);
        c.g[S1][0] = c.g[A1][0];
        c.cycles += 0; // the flags word is loaded below
        c.g[V0][0] = lw(c, this, 0x30);
        c.g[A0][0] = c.g[V0][0] | 0x2;
        let flags = c.g[A0][0];
        sw(c, this, 0x30, flags);
        if c.g[S1][0] == 0x4c {
            c.cycles += 7;
            c.g[S1][0] = lw(c, this, 0x24);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0;
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0015_d050, 0x0016_1fe0)?;
            sw(c, this, 0x2c, c.g[S1][0]);
        } else if c.g[S1][0] != 0x5d {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0;
            call_ptr(c, 0x0015_d050, 0x0016_2048)?;
            sw(c, this, 0x2c, c.g[S1][0]);
        } else {
            c.cycles += 3;
            let v0 = (flags & 1) as u64;
            let bit4 = (flags & 4) as u64;
            if v0 != 0 {
                c.cycles += 6;
                c.g[A1][0] = lw(c, this, 0x28);
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = lw(c, this, 0x34);
                call_ptr(c, 0x0015_d050, 0x0016_2010)?;
                sw(c, this, 0x2c, c.g[S1][0]);
            } else {
                c.cycles += 2;
                c.g[V0][0] = bit4;
                c.g[A0][0] = c.g[S0][0];
                if bit4 == 0 {
                    c.cycles += 1;
                    c.g[A1][0] = lw(c, this, 0x24);
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = 0x42;
                }
                c.cycles += 4;
                c.g[A2][0] = 0;
                call_ptr(c, 0x0015_d050, 0x0016_2034)?;
                sw(c, this, 0x2c, c.g[S1][0]);
            }
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MCManager_OnOperationDone` (0x0023ea90).
//
// `void MCManager::OnOperationDone(MCManager* self)`: finish a memory-card
// operation, refresh the slot and return to the slot state.

pub fn mc_manager_on_operation_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[S0][0] = sx(this);
        c.g[V0][0] = lw(c, this, 0xbc);
        let mut v0_reg;
        let mut dispatch = true;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = lw(c, this, 0x12c);
            v0_reg = c.g[V0][0];
            dispatch = false;
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw(c, this, 0x118);
            let t = c.g[V0][0] == 0;
            sw(c, gp(c, -0x478), 0, 0);
            if !t {
                c.cycles += 6;
                c.g[A1][0] = lw(c, this, 0x434);
                c.g[V0][0] = lw(c, c.g[A1][0] as u32, 0);
                c.g[A0][0] = lh(c, c.g[V0][0] as u32, 0x160);
                let target = lw(c, c.g[V0][0] as u32, 0x164);
                let t = target as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0023_ead0)?;
            }
            c.cycles += 3;
            c.g[V0][0] = lw(c, this, 0x748);
            c.g[A1][0] = 0x2;
            v0_reg = c.g[V0][0];
        }
        if !dispatch {
            // The "operation still pending" check, then the shared status walk.
            c.cycles += 2;
            let t = v0_reg == 0;
            sw(c, this, 0x440, 0);
            if !t {
                c.cycles += 7;
                c.g[A1][0] = 0;
                c.g[A0][0] = sx(this);
                call_ptr(c, 0x0023_fb18, 0x0023_eaf0)?;
                c.g[A0][0] = sx(this);
                call_ptr(c, 0x0023_fae0, 0x0023_eaf8)?;
                c.g[V0][0] = lw(c, this, 0x748);
                c.g[A1][0] = 0x31;
                dispatch = true;
            } else {
                // Status walk (b_23eb38).
                c.cycles += 4;
                c.g[A0][0] = sx(this);
                call_ptr(c, 0x0023_c8d0, 0x0023_eb20)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = sx(this);
                    call_ptr(c, 0x0024_2050, 0x0023_eb40)?;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = sx(this);
                    call_ptr(c, 0x0024_1fd0, 0x0023_eb30)?;
                    frame.restore(c, 16, 0x10);
                }
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        if dispatch {
            c.cycles += 6;
            c.g[A0][0] = lh(c, c.g[V0][0] as u32, 8);
            let target = lw(c, c.g[V0][0] as u32, 0xc);
            let t = target as u32;
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0023_eb10)?;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MCManager_State22_Load` (0x0023ff00).
//
// `void MCManager::State22_Load(MCManager* self)`: poll the memory-card
// operation and either finish it or start the next step.

pub fn mc_manager_state22_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[S0][0] = sx(this);
        c.g[V0][0] = lw(c, this, 0x108);
        let expired = (c.g[V0][0] as i64) <= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if expired {
            c.cycles += 8;
            c.g[A1][0] = lw(c, this, 0x434);
            c.g[V1][0] = lw(c, c.g[A1][0] as u32, 0);
            c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x58);
            let target = lw(c, c.g[V1][0] as u32, 0x5c);
            let t = target as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0023_ff3c)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, 16, 0x10);
            } else {
                c.cycles += 8;
                c.g[A1][0] = lw(c, this, 0x434);
                c.g[V1][0] = lw(c, c.g[A1][0] as u32, 0);
                c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x248);
                let target = lw(c, c.g[V1][0] as u32, 0x24c);
                let t = target as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0023_ff5c)?;
                let t = c.g[V0][0] == 0;
                c.g[A1][0] = 0x24;
                if !t {
                    c.cycles += 10;
                    c.g[V0][0] = lw(c, this, 0x748);
                    c.g[A0][0] = lh(c, c.g[V0][0] as u32, 8);
                    let target = lw(c, c.g[V0][0] as u32, 0xc);
                    let t = target as u32;
                    c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0023_ff78)?;
                    c.g[V0][0] = lw(c, this, 0x2c);
                    let target = c.g[V0][0] as u32;
                    call_ptr(c, target, 0x0023_ff84)?;
                    frame.restore(c, 16, 0x10);
                } else {
                    c.cycles += 9;
                    c.g[V0][0] = lw(c, this, 0x748);
                    c.g[A1][0] = 0x23;
                    c.g[A0][0] = lh(c, c.g[V0][0] as u32, 8);
                    let target = lw(c, c.g[V0][0] as u32, 0xc);
                    let t = target as u32;
                    c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0023_ffa4)?;
                    c.g[V0][0] = lw(c, this, 0x28);
                    let target = c.g[V0][0] as u32;
                    call_ptr(c, target, 0x0023_ffb0)?;
                    c.cycles += 1;
                    frame.restore(c, 16, 0x10);
                }
            }
        } else {
            c.cycles += 2;
            sw(c, this, 0x108, c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEPopupConfirm_MeasureLine` (0x001c84a8).
//
// `void cFEPopupConfirm::MeasureLine(self, TextMetrics* m, float* out, void* dst)`:
// measure one line of the popup text and scale it by the font metrics.

pub fn fe_popup_confirm_measure_line(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        let metrics = c.g[A1][0] as u32;
        frame.save(c, 18, 0x20);
        let out = c.g[A2][0];
        frame.save_ra(c, 0x10);
        let dst = c.g[A3][0];
        c.g[S0][0] = sx(metrics);
        c.g[S1][0] = out;
        c.g[S2][0] = dst;
        c.ldl(2, metrics.wrapping_add(0x57));
        c.ldr(2, metrics.wrapping_add(0x50));
        c.g[V1][0] = lw(c, metrics, 0x58);
        c.sdl(2, sp(c).wrapping_add(7));
        c.sdr(2, sp(c));
        sw(c, sp(c), 8, c.g[V1][0]);
        c.g[A0][0] = sx(metrics);
        call_ptr(c, 0x003a_04f0, 0x001c_84e8)?;
        ldf(c, 2, sp(c), 0);
        c.g[A0][0] = sx(metrics);
        ldf(c, 3, sp(c), 4);
        let v0 = c.g[V0][0] as u32;
        ldf(c, 0, v0, 0x30);
        ldf(c, 1, v0, 0x34);
        c.mul_s(0, 0, 2);
        c.mul_s(1, 1, 3);
        c.swc1(0, v0.wrapping_add(0x38));
        c.swc1(1, v0.wrapping_add(0x3c));
        call_ptr(c, 0x003a_04f0, 0x001c_8510)?;
        let v0 = c.g[V0][0] as u32;
        c.g[A0][0] = sx(v0);
        c.g[A1][0] = c.g[S1][0];
        ldf(c, 12, v0, 0x38);
        c.g[A2][0] = c.g[S2][0];
        ldf(c, 13, v0, 0x3c);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_1fb0, 0x001c_852c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003a_04f0, 0x001c_8534)?;
        ldf(c, 0, c.g[V0][0] as u32, 0x14);
        c.cvt_s_w(0, 0);
        ldf(c, 1, c.g[V0][0] as u32, 0x34);
        ldf(c, 2, sp(c), 4);
        frame.restore(c, 16, 0x40);
        c.mul_s(0, 0, 1);
        frame.restore(c, 17, 0x30);
        c.mul_s(2, 2, 0);
        c.swc1(2, c.g[S2][0] as u32 + 0xc);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEPopupConfirm_MeasureLineW` (0x001c8568): the wide-string variant of
// [`fe_popup_confirm_measure_line`].
pub fn fe_popup_confirm_measure_line_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        let metrics = c.g[A1][0] as u32;
        frame.save(c, 18, 0x20);
        let out = c.g[A2][0];
        frame.save_ra(c, 0x10);
        let dst = c.g[A3][0];
        c.g[S0][0] = sx(metrics);
        c.g[S1][0] = out;
        c.g[S2][0] = dst;
        c.ldl(2, metrics.wrapping_add(0x57));
        c.ldr(2, metrics.wrapping_add(0x50));
        c.g[V1][0] = lw(c, metrics, 0x58);
        c.sdl(2, sp(c).wrapping_add(7));
        c.sdr(2, sp(c));
        sw(c, sp(c), 8, c.g[V1][0]);
        c.g[A0][0] = sx(metrics);
        call_ptr(c, 0x003a_04f0, 0x001c_85a8)?;
        ldf(c, 2, sp(c), 0);
        c.g[A0][0] = sx(metrics);
        ldf(c, 3, sp(c), 4);
        let v0 = c.g[V0][0] as u32;
        ldf(c, 0, v0, 0x30);
        ldf(c, 1, v0, 0x34);
        c.mul_s(0, 0, 2);
        c.mul_s(1, 1, 3);
        c.swc1(0, v0.wrapping_add(0x38));
        c.swc1(1, v0.wrapping_add(0x3c));
        call_ptr(c, 0x003a_04f0, 0x001c_85d0)?;
        let v0 = c.g[V0][0] as u32;
        c.g[A0][0] = sx(v0);
        c.g[A1][0] = c.g[S1][0];
        ldf(c, 12, v0, 0x38);
        c.g[A2][0] = c.g[S2][0];
        ldf(c, 13, v0, 0x3c);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_21f0, 0x001c_85ec)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003a_04f0, 0x001c_85f4)?;
        ldf(c, 0, c.g[V0][0] as u32, 0x14);
        c.cvt_s_w(0, 0);
        ldf(c, 1, c.g[V0][0] as u32, 0x34);
        ldf(c, 2, sp(c), 4);
        frame.restore(c, 16, 0x40);
        c.mul_s(0, 0, 1);
        frame.restore(c, 17, 0x30);
        c.mul_s(2, 2, 0);
        c.swc1(2, c.g[S2][0] as u32 + 0xc);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cSetupMan_ClientRecvMsg0` (0x00269240) and `cSetupMan_ClientRecvMsg2`
// (0x00269300): store the received remote character and queue a reply.

/// The shared reply tail: link the freshly allocated packet at `a1` into the
/// pending queue and restore the frame.
fn client_recv_reply(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = lw(c, sp(c), 0x10);
    let a16 = sp(c).wrapping_add(0x50);
    c.lq(16, a16);
    sw(c, c.g[A1][0] as u32, 0, c.g[V0][0]);
    let a17 = sp(c).wrapping_add(0x40);
    let v0 = c.g[V0][0] as u32;
    c.g[V1][0] = lw(c, v0, 4);
    let a18 = sp(c).wrapping_add(0x30);
    c.lq(18, a18);
    sw(c, c.g[A1][0] as u32, 4, c.g[V1][0]);
    c.g[A0][0] = lw(c, v0, 4);
    sw(c, c.g[A0][0] as u32, 0, c.g[A1][0]);
    c.g[V0][0] = lw(c, sp(c), 0x10);
    sw(c, c.g[V0][0] as u32, 4, c.g[A1][0]);
    sw(c, sp(c), 0, c.g[A1][0]);
    let _ = a17;
}

fn recv_msg(c: &mut Cpu, reply: u64, character: u64, ret1: u32, ret2: u32, ret3: u32) -> Flow {
    c.cycles += 33;
    let frame = Frame::open(c, 0x60);
    frame.save(c, 16, 0x50);
    frame.save(c, 17, 0x40);
    c.g[S0][0] = reply;
    frame.save(c, 18, 0x30);
    frame.save_ra(c, 0x20);
    let this = c.g[A0][0] as u32;
    sw(c, this, 0, c.g[A2][0]);
    c.g[A2][0] = character;
    call_ptr(c, 0x0026_4d80, ret1)?;
    c.g[A1][0] = 0x480000;
    let s1 = Addr(gp(c, -0xc8)).lw(c) as u32;
    c.g[S1][0] = sx(s1);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
    c.g[A0][0] = 0x8;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, ret2)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = 0x480000;
    c.g[V0][0] = 0x480000;
    sw(c, c.g[S2][0] as u32, 0, c.g[S0][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    sw(c, c.g[S2][0] as u32, 4, c.g[V0][0]);
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[V0][0] = lw(c, s1, 0xf0);
    sw(c, sp(c), 0x10, c.g[V0][0]);
    call_ptr(c, 0x0031_7d70, ret3)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
    if c.g[V0][0] != 0 {
        sw(c, c.g[A1][0] as u32, 8, c.g[S2][0]);
    }
    client_recv_reply(c);
    let a17 = sp(c).wrapping_add(0x40);
    c.lq(17, a17);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

pub fn setup_man_client_recv_msg0(c: &mut Cpu) {
    guest(c, |c| {
        recv_msg(c, 0xf3, c.g[A3][0], 0x0026_9264, 0x0026_9280, 0x0026_92b4)
    })
}

pub fn setup_man_client_recv_msg2(c: &mut Cpu) {
    guest(c, |c| recv_msg(c, 0xf5, 0, 0x0026_9324, 0x0026_9340, 0x0026_9374))
}

// ---------------------------------------------------------------------------
// `BoltFile_CacheShutdown` (0x0019cf40).
//
// `void BoltFile_CacheShutdown(BoltFile* self)`: free every cached slot of
// every bolt file and drop the cache.

pub fn bolt_file_cache_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0xb0000;
        frame.save(c, 20, 0x30);
        c.g[V0][0] = c.g[V0][0] | 0x4008;
        frame.save(c, 17, 0x60);
        let this = c.g[A0][0];
        frame.save(c, 21, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, 22, 0x10);
        c.g[S4][0] = this;
        c.g[S5][0] = sx((this as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save(c, 16, 0x70);
        c.g[S6][0] = 0x1;
        frame.save(c, 18, 0x50);
        frame.save(c, 19, 0x40);
        frame.save_ra(c, 0);
        loop {
            c.cycles += 4;
            c.g[S0][0] = 0;
            c.g[S2][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[A2][0] = c.g[S0][0];
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                call_ptr(c, 0x0019_d4b8, 0x0019_cfa0)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 256) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = c.g[S0][0];
                if more {
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x0019_cf90);
                    }
                    continue;
                }
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[S1][0] = c.g[S3][0];
                c.g[V0][0] = ((c.g[S1][0] as i64) < 10) as u64;
                let more_files = c.g[V0][0] != 0;
                sw(c, c.g[V1][0] as u32, 0, c.g[S6][0]);
                if more_files {
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x0019_cf80);
                    }
                    continue;
                }
                c.cycles += 3;
                c.g[A0][0] = Addr(gp(c, -0x1788)).lw(c);
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    call_ptr(c, 0x0031_7e50, 0x0019_cfd4)?;
                }
                frame.restore(c, 16, 0x70);
                c.cycles += 10;
                frame.restore(c, 17, 0x60);
                frame.restore(c, 18, 0x50);
                frame.restore(c, 19, 0x40);
                frame.restore(c, 20, 0x30);
                frame.restore(c, 21, 0x20);
                frame.restore(c, 22, 0x10);
                frame.restore_ra(c, 0);
                sw(c, gp(c, -0x1788), 0, 0);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

// ---------------------------------------------------------------------------
// `GameStateEventSwitchLoad_Enter` (0x002368a0).
//
// `void GameStateEventSwitchLoad::Enter(self)`: load the game-state data and
// start playing its event tracks.

pub fn game_state_event_switch_load_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 19, 0x10);
        let this = c.g[A0][0];
        frame.save(c, 17, 0x30);
        frame.save(c, 16, 0x40);
        frame.save(c, 18, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_de68, 0x0023_68c4)?;
        c.g[V0][0] = Addr(gp(c, -0x488)).lw(c);
        c.g[A0][0] = lw(c, c.g[V0][0] as u32, 0x28);
        call_ptr(c, 0x0026_f228, 0x0023_68d0)?;
        call_ptr(c, 0x0014_dc80, 0x0023_68d8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_68e4)?;
        c.g[V1][0] = 0x530000;
        c.g[S1][0] = sx(0x530000u32.wrapping_add(0x5bc8));
        c.g[S3][0] = this;
        let s1 = c.g[S1][0] as u32;
        c.g[V0][0] = lb(c, s1, 0x49);
        if c.g[V0][0] != 0 {
            sw(c, this as u32, 0x10, 0);
        } else {
            c.cycles += 9;
            call_ptr(c, 0x0014_dc80, 0x0023_6900)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0014_dd58, 0x0023_690c)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = lw(c, s1, 0x30);
            if (c.g[V0][0] as i64) <= 0 {
                sw(c, this as u32, 0x10, 0);
            } else {
                c.cycles += 1;
                c.g[S0][0] = 0;
                c.g[A1][0] = c.g[S0][0];
                loop {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    call_ptr(c, 0x0014_5cb0, 0x0023_6930)?;
                    c.g[V0][0] = lw(c, s1, 0x30);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[S0][0];
                    if more {
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x0023_6920);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    sw(c, this as u32, 0x10, 0);
                    break;
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `SSXAudioSystem_OnReplayBegin` (0x0028f200).
//
// `void SSXAudioSystem::OnReplayBegin(self)`: start the replay music unless it
// is already playing.

pub fn ssx_audio_system_on_replay_begin(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(this);
        let mon = this.wrapping_add(0x118);
        c.g[S1][0] = sx(mon);
        sw(c, this, 0x6294, 0);
        c.g[V0][0] = lw(c, this, 0x530);
        let t = c.g[V0][0] == 0;
        sw(c, this, 0x6298, 0);
        if !t {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x002b_49e0, 0x0028_f230)?;
            c.g[V1][0] = 0xc9;
            let not_playing = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 1;
            if not_playing {
                c.cycles += 1;
                sw(c, this, 0x6298, c.g[V0][0]);
                c.cycles += 5;
                frame.restore(c, 16, 0x20);
                frame.restore(c, 17, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_d960, 0x0028_f244)?;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = 1;
        if !t {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x002b_49e0, 0x0028_f254)?;
            c.g[V1][0] = 0xc9;
            if c.g[V0][0] != c.g[V1][0] {
                c.g[V0][0] = lw(c, this, 0x118);
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw(c, this, 0x62ac);
                sw(c, this, 0x508, c.g[V0][0]);
                c.g[V0][0] = lw(c, this, 0x118);
            }
            c.cycles += 13;
            c.g[V1][0] = 0x4a0000;
            c.ldl(6, 0x4a364f);
            c.ldr(6, 0x4a3648);
            c.g[A1][0] = 0xa;
            c.g[A0][0] = lw(c, c.g[V0][0] as u32, 0);
            c.g[A3][0] = 0;
            c.g[T0][0] = 3;
            c.g[T1][0] = 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            call_ptr(c, 0x002a_dca0, 0x0028_f29c)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 2;
        sw(c, this, 0x6294, c.g[V0][0]);
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `WScriptProcess_dtor` (0x00307738).
//
// `void WScriptProcess::~WScriptProcess(WScriptProcess* self, int flags)`:
// destroy the element array, release the script object and optionally delete
// the process.

pub fn w_script_process_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x490000;
        frame.save(c, 17, 0x20);
        c.g[V0][0] = sx(0x490000u32.wrapping_sub(0x6490));
        frame.save(c, 18, 0x10);
        let this = c.g[A0][0] as u32;
        frame.save(c, 16, 0x30);
        let flags = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(this);
        c.g[S2][0] = flags;
        sw(c, this, 0x5c, c.g[V0][0]);
        let arr = lw(c, this, 0x14);
        c.g[A2][0] = arr;
        if arr != 0 {
            c.cycles += 6;
            c.g[V1][0] = Addr((arr as u32).wrapping_sub(0x10)).lw(c);
            c.g[V0][0] = 0x44;
            c.mult(0, 4, 3, 2);
            let s0 = (c.g[A0][0] as u32).wrapping_add(arr as u32);
            c.g[S0][0] = sx(s0);
            let mut tail = arr as u32;
            if arr as u32 == c.g[S0][0] as u32 {
                tail = lw(c, this, 0x14) as u32;
                c.g[A0][0] = sx(tail);
            } else {
                c.cycles += 2;
                let mut p = s0.wrapping_sub(0x44);
                c.g[S0][0] = sx(p);
                loop {
                    c.cycles += 9;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = lw(c, p, 0x40);
                    c.g[A0][0] = lh(c, c.g[V0][0] as u32, 8);
                    let target = lw(c, c.g[V0][0] as u32, 0xc);
                    let t = target as u32;
                    c.g[A0][0] = sx(p.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0030_77a0)?;
                    c.g[V0][0] = lw(c, this, 0x14);
                    if c.g[V0][0] != c.g[S0][0] {
                        p = p.wrapping_sub(0x44);
                        c.g[S0][0] = sx(p);
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x0030_7788);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    tail = lw(c, this, 0x14) as u32;
                    c.g[A0][0] = sx(tail);
                    break;
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx(tail.wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x0030_77b8)?;
        }
        c.cycles += 3;
        c.g[A0][0] = lw(c, this, 0x1c);
        let t = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S2][0] & 1;
        if !t {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0022_4df0, 0x0030_77cc)?;
            c.g[V0][0] = c.g[S2][0] & 1;
        }
        c.cycles += 2;
        let del = c.g[V0][0] != 0;
        frame.restore(c, 16, 0x30);
        if del {
            c.cycles += 3;
            c.g[A0][0] = sx(this);
            call_ptr(c, 0x0031_7e50, 0x0030_77e0)?;
            frame.restore(c, 16, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `WScriptProcess_v04` (0x00307c40).
//
// `void WScriptProcess::v04(WScriptProcess* self)`: (re)initialise the
// process's element storage and run its entry function.

pub fn w_script_process_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[S0][0] = sx(this);
        c.g[A0][0] = lw(c, this, 0x1c);
        c.g[A1][0] = 0x490000;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0022_4df0, 0x0030_7c64)?;
            c.g[A1][0] = 0x490000;
        }
        c.cycles += 14;
        c.g[A2][0] = 0x2000_0000;
        c.g[A0][0] = 4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6778));
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0030_7c7c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 8;
        call_ptr(c, 0x0022_4da0, 0x0030_7c88)?;
        c.g[V1][0] = 1;
        c.g[A2][0] = lw(c, this, 0x28);
        sw(c, this, 0x1c, c.g[V0][0]);
        sw(c, this, 0, c.g[V1][0]);
        let t = c.g[A2][0] == 0;
        sw(c, this, 8, 0);
        if !t {
            c.cycles += 3;
            c.g[A0][0] = lw(c, this, 0x10);
            c.g[A1][0] = sx(this);
            call_ptr(c, 0x0030_ac98, 0x0030_7cac)?;
        }
        c.cycles += 8;
        c.g[A0][0] = lw(c, this, 0x10);
        c.g[A1][0] = sx(this);
        call_ptr(c, 0x0030_9a60, 0x0030_7cb8)?;
        c.g[A1][0] = lw(c, this, 8);
        c.g[V0][0] = lw(c, this, 4);
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 10;
            c.g[A0][0] = 0x44;
            c.g[V1][0] = lw(c, this, 0x14);
            c.mult(0, 4, 5, 4);
            c.g[A0][0] = sx(0x44u32.wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = lw(c, c.g[A0][0] as u32, 0x40);
            c.g[V1][0] = lh(c, c.g[V0][0] as u32, 0x20);
            let target = lw(c, c.g[V0][0] as u32, 0x24);
            let t = target as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            call_ptr(c, t, 0x0030_7cf0)?;
        }
        frame.restore(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `OVScreenInGameError_OnWidgetAction` (0x001fb0c0).
//
// `int OVScreenInGameError::OnWidgetAction(self, UIWidget* w, int action)`:
// map the dialog buttons onto the highlight/select handlers.

pub fn ov_screen_in_game_error_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        let widget = c.g[A1][0];
        frame.save(c, 16, 0x30);
        let this = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = widget;
        c.g[S2][0] = this;
        let action = c.g[A2][0];
        if widget == 0 {
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            if action == 1 {
                // Scroll button: compare the widget id with the two constants.
                c.cycles += 6;
                c.g[A0][0] = 0x470000;
                c.g[S0][0] = lw(c, widget as u32, 0x38);
                c.g[A0][0] = sx(0x470000u32.wrapping_sub(0x570));
                call_ptr(c, 0x0031_7670, 0x001f_b114)?;
                let t = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = 0x470000;
                if !t {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x001f_b588, 0x001f_b128)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x001f_b458, 0x001f_b134)?;
                    frame.restore(c, 16, 0x30);
                } else {
                    c.cycles += 5;
                    c.g[S0][0] = lw(c, widget as u32, 0x38);
                    c.g[A0][0] = sx(0x470000u32.wrapping_sub(0x4b8));
                    call_ptr(c, 0x0031_7670, 0x001f_b148)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    frame.restore(c, 16, 0x30);
                    if !t {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = 1;
                        call_ptr(c, 0x001f_b458, 0x001f_b15c)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = 0;
                        call_ptr(c, 0x001f_b588, 0x001f_b168)?;
                        c.cycles += 1;
                        frame.restore(c, 16, 0x30);
                    }
                }
            } else {
                c.cycles += 2;
                let not_five = action != 5;
                frame.restore(c, 16, 0x30);
                if !not_five {
                    c.cycles += 4;
                    c.g[A2][0] = 5;
                    call_ptr(c, 0x001f_b2b8, 0x001f_b0fc)?;
                    frame.restore(c, 16, 0x30);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `BERewardInt_IsPeakFullyCompleted` (0x001577e0).
//
// `int BERewardInt::IsPeakFullyCompleted(self, peak, medal, index)`: true when
// all three peak medals are complete and the record levels add up to 24.

pub fn be_reward_int_is_peak_fully_completed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, 18, 0x20);
        let s1 = c.g[A2][0];
        frame.save(c, 19, 0x10);
        let s2 = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = s1;
        c.g[S2][0] = s2;
        c.g[S3][0] = this;
        c.g[A0][0] = this;
        loop {
            c.cycles += 6;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            call_ptr(c, 0x0015_7920, 0x0015_7820)?;
            let complete = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            if !complete {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 3) as u64;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = c.g[S3][0];
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0015_7810);
                }
                continue;
            }
            c.cycles += 9;
            call_ptr(c, 0x0014_dc80, 0x0015_7844)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 8;
            call_ptr(c, 0x0014_dd58, 0x0015_7850)?;
            c.g[S1][0] = 0;
            c.g[S0][0] = 0;
            c.g[A1][0] = c.g[S0][0];
            c.g[S2][0] = c.g[V0][0];
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0015_52b0, 0x0015_7868)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = ((c.g[S0][0] as i64) < 8) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S0][0];
                if more {
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x0015_7860);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 24) as u64;
                c.g[V0][0] = c.g[V0][0] ^ 1;
                break;
            }
            break;
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Bolts_IndexTable3ByChar` (0x0014c3c8) / `Bolts_IndexTable4ByChar`
// (0x0014c488): clear a per-character index table and rebuild it from the
// character bytes, keeping the first occurrence of each character.

pub fn bolts_index_table3_by_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x40);
        c.g[A2][0] = 0x78;
        let this = c.g[A0][0] as u32;
        frame.save(c, 17, 0x30);
        c.g[S0][0] = sx(this);
        c.g[S1][0] = sx(this.wrapping_add(0x2fc));
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[A0][0] = sx(this.wrapping_add(0x2fc));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(this.wrapping_add(0x374));
        call_ptr(c, 0x003e_6448, 0x0014_c3fc)?;
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = sx(this.wrapping_add(0x374));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x78;
        call_ptr(c, 0x003e_6448, 0x0014_c410)?;
        c.g[V0][0] = lw(c, this, 0x24);
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !t {
            c.cycles += 3;
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[V1][0] = lw(c, this, 0xc);
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = lb(c, c.g[V0][0] as u32, 0);
                c.g[A2][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = lw(c, c.g[V1][0] as u32, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                let same = c.g[S3][0] == c.g[A1][0];
                sw(c, c.g[V1][0] as u32, 0, c.g[V0][0]);
                if !same {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[S3][0] = c.g[A1][0];
                    sw(c, c.g[V0][0] as u32, 0, c.g[A3][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = lw(c, this, 0x24);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = lw(c, this, 0xc);
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0014_c428);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub fn bolts_index_table4_by_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x40);
        c.g[A2][0] = 0x78;
        let this = c.g[A0][0] as u32;
        frame.save(c, 17, 0x30);
        c.g[S0][0] = sx(this);
        c.g[S1][0] = sx(this.wrapping_add(0x3ec));
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[A0][0] = sx(this.wrapping_add(0x3ec));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(this.wrapping_add(0x464));
        call_ptr(c, 0x003e_6448, 0x0014_c4bc)?;
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = sx(this.wrapping_add(0x464));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x78;
        call_ptr(c, 0x003e_6448, 0x0014_c4d0)?;
        c.g[V0][0] = lw(c, this, 0x28);
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !t {
            c.cycles += 3;
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[V1][0] = lw(c, this, 0x10);
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = lb(c, c.g[V0][0] as u32, 0);
                c.g[A2][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = lw(c, c.g[V1][0] as u32, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                let same = c.g[S3][0] == c.g[A1][0];
                sw(c, c.g[V1][0] as u32, 0, c.g[V0][0]);
                if !same {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[S3][0] = c.g[A1][0];
                    sw(c, c.g[V0][0] as u32, 0, c.g[A3][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = lw(c, this, 0x28);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = lw(c, this, 0x10);
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0014_c4e8);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cUIScroll_SetPartsVisible` (0x0039de68).
//
// `void cUIScroll::SetPartsVisible(self, bool visible)`: forward the
// visibility flag to the scroll bar, track and thumb objects.

pub fn ui_scroll_set_parts_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(this);
        c.g[A2][0] = lw(c, this, 0x7c);
        let t = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !t {
            c.cycles += 5;
            c.g[V0][0] = lw(c, c.g[A2][0] as u32, 8);
            c.g[A0][0] = lh(c, c.g[V0][0] as u32, 0x48);
            let target = lw(c, c.g[V0][0] as u32, 0x4c);
            let t = target as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0039_de9c)?;
        }
        c.cycles += 3;
        c.g[A2][0] = lw(c, this, 0x80);
        c.g[A1][0] = c.g[S1][0];
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = lw(c, c.g[A2][0] as u32, 8);
            c.g[A0][0] = lh(c, c.g[V0][0] as u32, 0x48);
            let target = lw(c, c.g[V0][0] as u32, 0x4c);
            let t = target as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0039_debc)?;
        }
        c.cycles += 3;
        c.g[A2][0] = lw(c, this, 0x84);
        c.g[A1][0] = c.g[S1][0];
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = lw(c, c.g[A2][0] as u32, 8);
            c.g[A0][0] = lh(c, c.g[V0][0] as u32, 0x48);
            let target = lw(c, c.g[V0][0] as u32, 0x4c);
            let t = target as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0039_dedc)?;
        }
        c.cycles += 19;
        c.g[A2][0] = lw(c, this, 0x78);
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = lw(c, c.g[A2][0] as u32, 8);
        c.g[A0][0] = lh(c, c.g[V0][0] as u32, 0x48);
        let target = lw(c, c.g[V0][0] as u32, 0x4c);
        let t = target as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0039_def8)?;
        c.g[A2][0] = lw(c, this, 0x74);
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = lw(c, c.g[A2][0] as u32, 8);
        c.g[A0][0] = lh(c, c.g[V0][0] as u32, 0x48);
        let target = lw(c, c.g[V0][0] as u32, 0x4c);
        let t = target as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0039_df14)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Conveyor_ctor` (0x00343010).
//
// `void Conveyor::Conveyor(self, int, void* params, void* dir)`: build the
// world object, copy the eight direction bytes and tag the render flags.

pub fn conveyor_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        let dir = c.g[A3][0];
        frame.save(c, 17, 0x20);
        let this = c.g[A0][0] as u32;
        c.g[S1][0] = sx(this);
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A2][0] = 0xf;
        c.g[S0][0] = dir;
        call_ptr(c, 0x0034_fb00, 0x0034_3034)?;
        c.g[V0][0] = 0x490000;
        c.g[A3][0] = sx(this.wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x570));
        c.g[A2][0] = sx((dir as u32).wrapping_add(0x10));
        sw(c, this, 0xc, c.g[V0][0]);
        c.g[A1][0] = 0;
        ldf(c, 0, dir as u32, 4);
        c.swc1(0, this.wrapping_add(0x1c));
        ldf(c, 1, dir as u32, 8);
        ldf(c, 0, dir as u32, 0xc);
        c.swc1(1, sp(c));
        c.swc1(0, sp(c).wrapping_add(4));
        c.ldl(2, sp(c).wrapping_add(7));
        c.ldr(2, sp(c));
        c.sdl(2, this.wrapping_add(0x3f));
        c.sdr(2, this.wrapping_add(0x38));
        sw(c, this, 0x40, 0);
        sw(c, this, 0x44, 0);
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = lbu(c, c.g[A2][0] as u32, 0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            c.g[V0][0] = ((c.g[A1][0] as i64) < 8) as u64;
            let more = c.g[V0][0] != 0;
            Addr(c.g[A0][0] as u32).sb(c, c.g[V1][0]);
            if more {
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0034_3080);
                }
                continue;
            }
            c.cycles += 13;
            c.g[A1][0] = lw(c, this, 0x18);
            c.g[A0][0] = 0xffff_ffff_ffff_0000;
            c.g[A0][0] = c.g[A0][0] | 0xfffd;
            c.g[V0][0] = sx(this);
            c.g[V1][0] = lw(c, c.g[A1][0] as u32, 8);
            frame.restore(c, 16, 0x30);
            c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
            frame.restore(c, 17, 0x20);
            c.g[V1][0] = c.g[V1][0] | 4;
            frame.restore_ra(c, 0x10);
            sw(c, c.g[A1][0] as u32, 8, c.g[V1][0]);
            frame.close(c);
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// `CrowdMan3d_ctor` (0x00343a68).
//
// `void CrowdMan3d::CrowdMan3d(self)`: initialise the four crowd groups and
// their slot records and reset each group.

pub fn crowd_man3d_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 4;
        frame.save(c, 16, 0x50);
        frame.save(c, 19, 0x20);
        c.g[S0][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, 20, 0x10);
        let this = c.g[A0][0];
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        c.g[S0][0] = c.g[S0][0] | 0xffff;
        frame.save_ra(c, 0);
        c.g[S3][0] = this;
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        let mut a0 = c.g[A0][0];
        let mut v0: u32 = 4;
        loop {
            c.cycles += 6;
            c.g[S2][0] = sx(v0.wrapping_sub(1));
            sw(c, a0 as u32, 4, c.g[S0][0]);
            c.g[S1][0] = sx((a0 as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((a0 as u32).wrapping_add(0xc));
            c.g[V1][0] = 0x1f;
            let mut v0r = c.g[V0][0] as u32;
            loop {
                c.cycles += 7;
                sw(c, v0r, 0, c.g[S0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                let t = c.g[V1][0] != c.g[S4][0];
                v0r = v0r.wrapping_add(8);
                c.g[V0][0] = sx(v0r);
                if !t {
                    break;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0034_3ab0);
                }
            }
            c.cycles += 5;
            call_ptr(c, 0x0034_42e0, 0x0034_3ad4)?;
            c.g[V0][0] = c.g[S2][0];
            let t = c.g[V0][0] != c.g[S4][0];
            c.g[A0][0] = c.g[S1][0];
            if !t {
                break;
            }
            v0 = c.g[V0][0] as u32;
            a0 = c.g[A0][0];
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0034_3a98);
            }
        }
        c.cycles += 4;
        sw(c, gp(c, 0xf00), 0, c.g[S3][0]);
        c.g[S0][0] = c.g[S3][0];
        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x9c4));
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0034_42e0, 0x0034_3af8)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S3][0];
            if !more {
                break;
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0034_3af0);
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateRewardsRoom_OnWidgetAction` (0x001d4ea8).
//
// `int cFEStateRewardsRoom::OnWidgetAction(self, widget, action)`: route the
// rewards-room buttons.

pub fn fe_state_rewards_room_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        let widget = c.g[A1][0];
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = widget;
        let this = c.g[A0][0] as u32;
        c.g[S0][0] = sx(this);
        if widget != 0 {
            let action = c.g[A2][0];
            c.cycles += 3;
            c.g[V0][0] = 5;
            let is_five = action == c.g[V0][0];
            c.g[V0][0] = (action < 6) as u64;
            if is_five {
                c.cycles += 15;
                c.g[V0][0] = lw(c, this, 0x10);
                c.g[A1][0] = sx(this);
                c.g[A2][0] = lw(c, widget as u32, 0x18);
                c.g[A3][0] = lw(c, c.g[V0][0] as u32, 0);
                c.g[V1][0] = lw(c, c.g[A3][0] as u32, 4);
                c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x20);
                let target = lw(c, c.g[V1][0] as u32, 0x24);
                let t = target as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001d_4f0c)?;
                c.g[A0][0] = lw(c, this, 0x10);
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f400, 0x001d_4f1c)?;
                frame.restore(c, 16, 0x10);
            } else if c.g[V0][0] != 0 {
                c.cycles += 2;
                frame.restore(c, 16, 0x10);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 6;
                if action == c.g[V0][0] {
                    c.g[V0][0] = lw(c, this, 0x10);
                    c.cycles += 10;
                    c.g[A1][0] = sx(this);
                    c.g[A2][0] = lw(c, widget as u32, 0x18);
                    c.g[A3][0] = lw(c, c.g[V0][0] as u32, 0);
                    c.g[V1][0] = lw(c, c.g[A3][0] as u32, 4);
                    c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x28);
                    let target = lw(c, c.g[V1][0] as u32, 0x2c);
                    let t = target as u32;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001d_4f44)?;
                    let zero = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[V0][0];
                    if !zero {
                        c.cycles += 3;
                        c.g[A0][0] = lw(c, this, 0x10);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039_f400, 0x001d_4f58)?;
                    }
                    c.cycles += 1;
                    frame.restore(c, 16, 0x10);
                } else {
                    c.cycles += 2;
                    frame.restore(c, 16, 0x10);
                }
            }
        } else {
            c.cycles += 1;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `ReplayStream_ReadObjectRef` (0x0026eb48).
//
// `void* ReplayStream::ReadObjectRef(self)`: read an object reference from
// the stream and resolve it against the current layer's object table.

pub fn replay_stream_read_object_ref(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, 16, 0x20);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        frame.save_ra(c, 0x10);
        let this = c.g[A0][0] as u32;
        c.g[A1][0] = sx(sp(c));
        c.g[A2][0] = 4;
        c.g[S0][0] = 0;
        let obj = lw(c, this, 0);
        c.g[A3][0] = obj;
        sw(c, sp(c), 0, c.g[V0][0]);
        c.g[V0][0] = lh(c, obj as u32, 0x10);
        let target = lw(c, obj as u32, 0x14);
        let t = target as u32;
        c.g[A0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, t, 0x0026_eb80)?;
        let word = lw(c, sp(c), 0);
        c.g[A1][0] = word;
        c.g[V0][0] = !(0u64 | word);
        let masked = c.g[V0][0] == 0;
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        if masked {
            // b_26ebf4 tail.
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw(c, c.g[V1][0] as u32, 0x84);
            c.g[A0][0] = lw(c, c.g[V0][0] as u32, 0x88);
            call_ptr(c, 0x0035_12c0, 0x0026_eb9c)?;
            let s0 = c.g[V0][0];
            c.g[S0][0] = s0;
            c.g[V0][0] = s0;
            if s0 != 0 {
                // b_26ebf8 tail: straight to the epilogue.
                c.cycles += 4;
                frame.restore_ra(c, 0x10);
                frame.restore(c, 16, 0x20);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 10;
            c.g[V1][0] = Addr(gp(c, 0x16c8)).lw(c);
            c.g[A1][0] = lw(c, sp(c), 0);
            c.g[A0][0] = lw(c, c.g[V1][0] as u32, 0);
            c.g[V0][0] = c.g[A1][0] & 0xff;
            c.g[V1][0] = lw(c, c.g[A0][0] as u32, 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V1][0] = lw(c, c.g[V0][0] as u32, 0);
            let v1_zero = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
            if v1_zero {
                // b_26ebec tail.
                c.cycles += 1;
                c.g[V0][0] = 0;
                c.cycles += 1;
                c.g[S0][0] = 0;
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 7;
                c.g[A0][0] = lw(c, c.g[V1][0] as u32, 0x1c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = lw(c, c.g[V0][0] as u32, 0);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nz = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if nz {
                    // b_26ebf0 / b_26ebf4 tails.
                    c.cycles += 1;
                    c.g[S0][0] = c.g[V0][0];
                    c.cycles += 1;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    c.cycles += 1;
                    c.g[S0][0] = 0;
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0x10);
        frame.restore(c, 16, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIText_GetTextSize` (0x003a1030).
//
// `void UIText::GetTextSize(self, V2* out)`: return the measured text size,
// from the layout object when it is available and from the font otherwise.

pub fn ui_text_get_text_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0x10);
        let out = c.g[A1][0];
        c.g[S0][0] = sx(this);
        c.g[S1][0] = out;
        c.g[V0][0] = lw(c, this, 0xb4);
        let no_layout = c.g[V0][0] == 0;
        let mut call_vtable = no_layout;
        if !no_layout {
            c.cycles += 5;
            c.g[V0][0] = lw(c, this, 0x14);
            c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 2) as u32);
            c.g[V0][0] = c.g[V0][0] & 1;
            call_vtable = c.g[V0][0] == 0;
        }
        if call_vtable {
            c.cycles += 5;
            c.g[V0][0] = lw(c, this, 8);
            c.g[A0][0] = lh(c, c.g[V0][0] as u32, 0x80);
            let target = lw(c, c.g[V0][0] as u32, 0x84);
            let t = target as u32;
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x003a_1078)?;
        }
        // b_3a10dc: the layout object is unavailable; fall back to the font.
        c.cycles += 5;
        c.g[A0][0] = sx(this);
        call_ptr(c, 0x003a_04f0, 0x003a_1080)?;
        c.g[A1][0] = lw(c, this, 0xb4);
        let font = c.g[V0][0];
        c.g[A1][0] = lw(c, this, 0xb4);
        let mut v0;
        if c.g[A1][0] == 0 {
            v0 = 0x500000u64;
        } else {
            c.cycles += 2;
            let t = font == 0;
            c.g[A0][0] = font;
            if !t {
                c.cycles += 10;
                ldf(c, 13, this, 0x54);
                ldf(c, 12, this, 0x50);
                c.g[A2][0] = sx(sp(c));
                c.g[A3][0] = 0;
                call_ptr(c, 0x0039_21f0, 0x003a_10a8)?;
                ldf(c, 1, sp(c), 8);
                ldf(c, 0, sp(c), 0xc);
                c.swc1(1, out as u32);
                c.swc1(0, out as u32 + 4);
                c.cycles += 5;
                frame.restore(c, 16, 0x30);
                frame.restore(c, 17, 0x20);
                frame.restore_ra(c, 0x10);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
            v0 = 0x500000;
        }
        c.cycles += 7;
        c.g[A2][0] = sx((v0 as u32).wrapping_sub(0xf28));
        let a2 = c.g[A2][0] as u32;
        c.ldl(3, a2.wrapping_add(7));
        c.ldr(3, a2);
        c.g[A0][0] = lw(c, a2, 8);
        c.sdl(3, out as u32 + 7);
        c.sdr(3, out as u32);
        sw(c, out as u32, 8, c.g[A0][0]);
        c.cycles += 5;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateBuyAttrib_OnWidgetCreated` (0x001f4910).
//
// `int cFEStateBuyAttrib::OnWidgetCreated(self, UIWidget* w)`: install the
// created widget into the state's slots by its id.

pub fn fe_state_buy_attrib_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        frame.save(c, 17, 0x20);
        let this = c.g[A0][0];
        frame.save_ra(c, 0);
        let widget = c.g[A1][0];
        frame.save(c, 16, 0x30);
        c.g[S2][0] = this;
        c.g[S1][0] = widget;
        c.g[A0][0] = 0x470000;
        c.g[S0][0] = lw(c, widget as u32, 0x38);
        c.g[A0][0] = sx(0x470000u32.wrapping_sub(0xff8));
        call_ptr(c, 0x0031_7670, 0x001f_493c)?;
        if c.g[S0][0] != c.g[V0][0] {
            c.g[A0][0] = 0x470000;
        } else {
            c.cycles += 8;
            c.g[A0][0] = sx(0x470000u32.wrapping_sub(0xfe0));
            call_ptr(c, 0x0031_7670, 0x001f_4950)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x001f_495c)?;
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
            c.cycles += 5;
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        // Slot chain.
        c.cycles += 5;
        c.g[S0][0] = lw(c, widget as u32, 0x38);
        c.g[A0][0] = sx(0x470000u32.wrapping_sub(0xfc0));
        call_ptr(c, 0x0031_7670, 0x001f_4970)?;
        if c.g[S0][0] == c.g[V0][0] {
            c.cycles += 2;
            sw(c, this as u32, 0xac, widget);
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
            c.cycles += 5;
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.g[A0][0] = 0x4a0000;
        c.cycles += 5;
        c.g[S0][0] = lw(c, widget as u32, 0x38);
        c.g[A0][0] = sx(0x4a0000u32.wrapping_add(0x2320));
        call_ptr(c, 0x0031_7670, 0x001f_498c)?;
        if c.g[S0][0] == c.g[V0][0] {
            c.cycles += 2;
            sw(c, widget as u32, 0x18, 0);
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
            c.cycles += 5;
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.g[A0][0] = 0x4a0000;
        c.cycles += 5;
        c.g[S0][0] = lw(c, widget as u32, 0x38);
        c.g[A0][0] = sx(0x4a0000u32.wrapping_add(0x2328));
        call_ptr(c, 0x0031_7670, 0x001f_49a8)?;
        let t = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, 16, 0x30);
        if t {
            c.cycles += 5;
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        c.g[V0][0] = 1;
        sw(c, widget as u32, 0x18, c.g[V0][0]);
        c.cycles += 1;
        frame.restore(c, 16, 0x30);
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIStateMgr_SwitchState` (0x0039f400).
//
// `void UIStateMgr::SwitchState(UIStateMgr* m, UIListNode* n)`: push the new
// state node in front of the current one and patch its visibility bits.

pub fn ui_state_mgr_switch_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 16, 0x30);
        let mgr = c.g[A0][0];
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        let node = c.g[A1][0];
        c.g[S1][0] = mgr;
        c.g[S0][0] = node;
        call_ptr(c, 0x0039_7840, 0x0039_f420)?;
        c.g[A0][0] = c.g[V0][0];
        let a0 = c.g[A0][0];
        c.g[S2][0] = 0;
        let mut v0;
        if a0 == 0 {
            v0 = c.g[V0][0];
        } else {
            c.cycles += 5;
            let v1 = lw(c, a0 as u32, 0x1c);
            c.g[V1][0] = v1;
            c.g[V0][0] = sx((((v1 as u32) as i32) >> 5) as u32);
            c.g[V0][0] = c.g[V0][0] & 1;
            let clear5 = c.g[V0][0] == 0;
            c.g[V0][0] = sx((((v1 as u32) as i32) >> 6) as u32);
            if clear5 {
                v0 = c.g[V0][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 1;
                let set6 = c.g[V0][0] != 0;
                c.g[V0][0] = sx((v1 as u32) >> 8);
                if set6 {
                    v0 = c.g[V0][0];
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[V0][0] & 0x3f;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 6) as u64;
                    let big = c.g[V0][0] == 0;
                    c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
                    if big {
                        v0 = c.g[V0][0];
                    } else {
                        c.cycles += 4;
                        c.g[S2][0] = 1;
                        c.g[V0][0] = v1 & c.g[V0][0];
                        c.g[V0][0] = c.g[V0][0] | 0x680;
                        sw(c, a0 as u32, 0x1c, c.g[V0][0]);
                        v0 = c.g[V0][0];
                    }
                }
            }
        }
        // b_39f48c / b_39f498.
        c.cycles += 2;
        let s2 = c.g[S2][0] == 0;
        c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
        if s2 {
            c.cycles += 3;
            c.g[V0][0] = lw(c, node as u32, 0x1c);
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x220;
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw(c, node as u32, 0x1c);
            c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x100;
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        }
        let _ = v0;
        c.cycles += 10;
        sw(c, node as u32, 0x1c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0039_78f0, 0x0039_f4a8)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `ConsoleClock_FormatTime` (0x002c6f78).
//
// `void ConsoleClock::FormatTime(char* out, int pad, int h, int m, int s,
//  int mode)`: format the clock into `out`.

pub fn console_clock_format_time(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 17, 0x50);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, 16, 0x60);
        c.g[A2][0] = c.g[A3][0];
        frame.save_ra(c, 0x40);
        c.g[A3][0] = c.g[T0][0];
        c.g[S0][0] = c.g[A1][0];
        let t2 = c.g[T2][0];
        c.g[T0][0] = c.g[T1][0];
        let mut copied = true;
        if t2 == 0 {
            c.cycles += 6;
            c.g[A1][0] = 0x480000;
            c.g[A0][0] = sx(sp(c));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6500));
            call_ptr(c, 0x002c_27c0, 0x002c_6fc4)?;
            c.g[A0][0] = c.g[S0][0];
        } else if t2 == 1 {
            c.cycles += 3;
            c.g[V0][0] = 1;
            c.g[A0][0] = c.g[S0][0];
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 12) as u64;
            let small = c.g[V0][0] == 0;
            c.g[A0][0] = sx(sp(c));
            if !small {
                c.cycles += 5;
                c.g[A1][0] = 0x480000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6510));
                call_ptr(c, 0x002c_27c0, 0x002c_6fe4)?;
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0xd;
                c.g[A1][0] = 0x480000;
                c.div(0, 6, 2);
                c.cycles += 6;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6528));
                c.g[A0][0] = sx(sp(c));
                c.g[A2][0] = c.hi[0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xc));
                call_ptr(c, 0x002c_27c0, 0x002c_7014)?;
                c.g[A0][0] = c.g[S0][0];
            }
        } else {
            c.cycles += 3;
            c.cycles += 2;
            c.cycles += 1;
            c.g[A2][0] = c.g[S1][0];
        }
        let _ = &mut copied;
        c.cycles += 7;
        c.g[A1][0] = sx(sp(c));
        call_ptr(c, 0x002c_25b8, 0x002c_7024)?;
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `GameCamera_ResetAll` (0x0015db58).
//
// `void GameCamera_ResetAll()`: reset the chase camera state of both camera
// slots of the race manager.

pub fn game_camera_reset_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = 0x4c0000;
        frame.save(c, 17, 0x10);
        c.g[S0][0] = sx(0x4c0000u32.wrapping_add(0x5830));
        frame.save_ra(c, 0);
        call_ptr(c, 0x0016_c968, 0x0015_db78)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_d1b8, 0x0015_db80)?;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V1][0] = lw(c, c.g[V0][0] as u32, 0x84);
        let s1 = lw(c, c.g[V1][0] as u32, 0x84) as u32;
        c.g[S1][0] = sx(s1);
        c.g[V0][0] = lw(c, s1, 4);
        c.g[S0][0] = lw(c, s1, 0);
        c.g[A0][0] = lw(c, c.g[V0][0] as u32, 0xa8);
        call_ptr(c, 0x0016_2170, 0x0015_db9c)?;
        c.g[V0][0] = lw(c, s1, 4);
        c.g[A1][0] = lw(c, c.g[V0][0] as u32, 0xa8);
        c.g[V1][0] = lw(c, c.g[A1][0] as u32, 0x14);
        c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x20);
        let target = lw(c, c.g[V1][0] as u32, 0x24);
        let t = target as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0015_dbb8)?;
        c.g[V1][0] = lw(c, s1, 4);
        c.g[A0][0] = 1;
        c.g[S0][0] = ((c.g[S0][0] as i64) < 2) as u64;
        sw(c, c.g[V1][0] as u32, 0x460, 0);
        c.g[V0][0] = lw(c, s1, 4);
        sw(c, c.g[V0][0] as u32, 0x4a4, c.g[A0][0]);
        if c.g[S0][0] != 0 {
            c.cycles += 5;
            frame.restore(c, 16, 0x20);
            frame.restore(c, 17, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 12;
        c.g[V0][0] = lw(c, s1, 8);
        c.g[A0][0] = lw(c, c.g[V0][0] as u32, 0xa8);
        call_ptr(c, 0x0016_2170, 0x0015_dbe0)?;
        c.g[V0][0] = lw(c, s1, 8);
        c.g[A2][0] = lw(c, c.g[V0][0] as u32, 0xa8);
        c.g[V1][0] = lw(c, c.g[A2][0] as u32, 0x14);
        c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x20);
        let target = lw(c, c.g[V1][0] as u32, 0x24);
        let t = target as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0015_dbfc)?;
        c.g[V0][0] = lw(c, s1, 8);
        sw(c, c.g[V0][0] as u32, 0x460, 0);
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Avalanche_FreeHistories` (0x002d9a80).
//
// `void Avalanche::FreeHistories()`: stop every avalanche that has a history
// and release the history links.

pub fn avalanche_free_histories(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x540000;
        frame.save(c, 16, 0x20);
        c.g[V0][0] = sx(0x540000u32.wrapping_sub(0x76c8));
        frame.save(c, 17, 0x10);
        let mut s0 = c.g[V0][0] as u32;
        c.g[S0][0] = sx(s0);
        frame.save_ra(c, 0);
        let s1 = s0.wrapping_add(0x1c0);
        c.g[S1][0] = sx(s1);
        c.g[V0][0] = lw(c, s0, 0);
        loop {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = sx(s0);
                call_ptr(c, 0x002d_81b0, 0x002d_9ab8)?;
            }
            s0 = s0.wrapping_add(0x1c);
            c.g[S0][0] = sx(s0);
            c.cycles += 3;
            c.g[V0][0] = ((s0 as i64) < s1 as i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = lw(c, s0, 0);
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x002d_9aa8);
            }
        }
        // Release the history lists.
        c.cycles += 3;
        c.g[V0][0] = Addr(gp(c, 0x9c8)).lw(c);
        let t = c.g[V0][0] == 0;
        c.g[S1][0] = Addr(gp(c, 0x9c0)).lw(c);
        if t {
            c.cycles += 1;
            frame.restore(c, 16, 0x20);
        } else {
            c.cycles += 2;
            let t = c.g[S1][0] == 0;
            frame.restore(c, 16, 0x20);
            if t {
                c.cycles += 1;
                frame.restore(c, 16, 0x20);
            } else {
                c.cycles += 1;
                let mut node = lw(c, c.g[S1][0] as u32, 4);
                c.g[S0][0] = node;
                loop {
                    c.cycles += 2;
                    if node == 0 {
                        c.g[S1][0] = lw(c, c.g[S1][0] as u32, 8);
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = lbu(c, node as u32, 0xf3);
                        loop {
                            c.cycles += 2;
                            if c.g[V0][0] == 0 {
                                Addr((node as u32).wrapping_add(0xf3)).sb(c, 0);
                                break;
                            }
                            c.cycles += 3;
                            c.g[A0][0] = lw(c, node as u32, 0xfc);
                            if c.g[A0][0] == 0 {
                                Addr((node as u32).wrapping_add(0xf3)).sb(c, 0);
                                break;
                            }
                            c.cycles += 3;
                            call_ptr(c, 0x0031_7e98, 0x002d_9b0c)?;
                            Addr((node as u32).wrapping_add(0xf3)).sb(c, 0);
                            break;
                        }
                        c.cycles += 4;
                        sw(c, node as u32, 0xfc, 0);
                        node = lw(c, node as u32, 0xf4);
                        c.g[S0][0] = node;
                        if node != 0 {
                            c.g[V0][0] = lbu(c, node as u32, 0xf3);
                            if crate::events::event_due(c) {
                                crate::events::deliver_event_here(c, 0x002d_9af0);
                            }
                            continue;
                        }
                        c.cycles += 1;
                        c.g[S1][0] = lw(c, c.g[S1][0] as u32, 8);
                    }
                    c.cycles += 2;
                    if c.g[S1][0] == 0 {
                        break;
                    }
                    node = lw(c, c.g[S1][0] as u32, 4);
                    c.g[S0][0] = node;
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x002d_9ae0);
                    }
                }
                c.cycles += 1;
                frame.restore(c, 16, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Avalanche_DestroyAll` (0x002d9b40).
//
// `void Avalanche::DestroyAll()`: free every avalanche's node list and clear
// the manager's tables.

pub fn avalanche_destroy_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 16, 0x30);
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002d_9a80, 0x002d_9b5c)?;
        let mut s1 = Addr(gp(c, 0x9c0)).lw(c);
        c.g[S1][0] = s1;
        let t = s1 == 0;
        c.g[A0][0] = 0x540000;
        let mut fell_through = t;
        if !t {
            c.cycles += 2;
            let mut node = lw(c, s1 as u32, 4);
            c.g[S0][0] = node;
            loop {
                c.cycles += 2;
                let s2 = c.g[S1][0];
                c.g[S2][0] = s2;
                if node != 0 {
                    c.cycles += 2;
                    c.g[A0][0] = node;
                    loop {
                        c.cycles += 4;
                        node = lw(c, node as u32, 0xf4);
                        c.g[S0][0] = node;
                        call_ptr(c, 0x0031_7e50, 0x002d_9b88)?;
                        let more = node != 0;
                        c.g[A0][0] = node;
                        if !more {
                            break;
                        }
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x002d_9b80);
                        }
                    }
                }
                c.cycles += 5;
                c.g[S1][0] = lw(c, c.g[S1][0] as u32, 8);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0031_7e50, 0x002d_9b9c)?;
                if c.g[S1][0] == 0 {
                    c.cycles += 1;
                    c.g[A0][0] = 0x540000;
                    fell_through = true;
                    break;
                }
                node = lw(c, c.g[S1][0] as u32, 4);
                c.g[S0][0] = node;
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x002d_9b70);
                }
            }
        }
        let _ = fell_through;
        c.cycles += 9;
        sw(c, gp(c, 0x9c0), 0, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7bb0));
        sw(c, gp(c, 0x9d0), 0, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x400;
        call_ptr(c, 0x0041_6210, 0x002d_9bc0)?;
        c.g[A0][0] = Addr(gp(c, 0x9c4)).lw(c);
        let t = c.g[A0][0] == 0;
        c.g[V0][0] = Addr(gp(c, 0x9c8)).lw(c);
        if t {
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                call_ptr(c, 0x0031_7e98, 0x002d_9bdc)?;
                sw(c, gp(c, 0x9c4), 0, 0);
                sw(c, gp(c, 0x9c8), 0, 0);
            }
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        sw(c, gp(c, 0x9cc), 0, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, elf_image, scramble, Rng, DATA, GAME_GP};

                        /// First size-class pool region (8-byte blocks): freeds must land here.
    const POOL0: u32 = DATA + 0x40_0000;

    /// The engine's size-class pool allocator: the "pools ready" flag, seven
    /// class regions with a free-list head past their blocks, and a short
    /// free chain inside each region.
    fn pool8(c: &mut Cpu, _hint: u32) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        c.bus.write32(GAME_GP + 0xdd8, 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x40_0000 + 0x4_0000 * i as u32;
            for n in 0..7u32 {
                let node = pool + 0x100 + 0x100 * n;
                c.bus.write32(node, if n == 6 { 0 } else { node + 0x100 });
            }
            c.bus.write32(pool + head, pool + 0x100);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }

    fn slot(c: &mut Cpu, vt: u32, off: u32, f: u32) {
        c.bus.write16(vt + off, 0);
        c.bus.write32(vt + off + 4, f);
    }

        #[test]
    fn camera_trigger_man_pop_trigger_stack_matches() {
        check(ee::s02::f_0016d260__generated, camera_trigger_man_pop_trigger_stack, 150, &|c, seed| {
            let mut r = Rng(0x0016_d260 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let stack = DATA;
            let count = (seed % 4) as u32;
            c.bus.write32(stack, count);
            c.bus.write32(stack + 4, r.next_u32());
            for i in 0..4 {
                c.bus.write32(stack + 8 + 4 * i, (r.next_u32() % 4) as u32);
            }
            c.g[A0][0] = sx(stack);
            c.g[A1][0] = (r.next_u32() % 5) as u64;
            c.g[A2][0] = sx(DATA + 0x100);
            c.g[A3][0] = sx(DATA + 0x200);
        });
    }

            #[test]
    fn ui_text_refresh_text_matches() {
        check(ee::s13::f_003a0528__generated, ui_text_refresh_text, 100, &|c, seed| {
            let mut r = Rng(0x003a_0528 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let this = DATA;
            c.bus.write32(this + 0xb0, if seed % 3 == 0 { 0 } else { 1 });
            let a = DATA + 0x100;
            let b = DATA + 0x200;
            let d = DATA + 0x300;
            c.bus.write32(this + 0x5c, a);
            c.bus.write32(a + 0xd0, b);
            c.bus.write32(b + 0x10, d);
            c.bus.write32(d + 0x10, 0); // hit the "no owner" early return
            c.g[A0][0] = sx(this);
        });
    }

                    #[test]
    fn fe_state_stats_format_time_matches() {
        check(ee::s03::f_001c4f58__generated, fe_state_stats_format_time, 80, &|c, seed| {
            let mut r = Rng(0x001c_4f58 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP - 0x590c, (1.0f32 / 60.0).to_bits());
            c.g[A1][0] = sx(DATA);
            c.g[A2][0] = ((r.next_u32() % 200_000) as i32) as u64;
        });
    }

                        #[test]
    fn setup_man_host_recv_cancel_matches() {
        check(ee::s07::f_00265398__generated, setup_man_host_recv_cancel, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x2653, 8);
            let pool = DATA + 0x30_0000;
            pool8(c, pool);
            let s0 = DATA + 0x1000;
            c.bus.write32(GAME_GP - 0xc8, s0);
            c.bus.write32(s0 + 0xf0, DATA + 0x2000);
            c.bus.write32(DATA + 0x2000 + 4, DATA + 0x3000);
            c.bus.write32(DATA, [0u32, 3][(seed % 2) as usize]);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x400);
        });
    }

                    #[test]
    fn fe_popup_confirm_measure_line_matches() {
        check(ee::s03::f_001c84a8__generated, fe_popup_confirm_measure_line, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1c84, 4);
            let metrics = DATA + 0x100;
            let out = DATA + 0x200;
            for k in 0..4 {
                c.bus.write32(metrics + 0x30 + 4 * k, r.edge_f32());
                c.bus.write32(metrics + 0x50 + 4 * k, r.edge_f32());
                c.bus.write32(out + 4 * k, r.edge_f32());
            }
            c.bus.write32(metrics + 0x14, r.next_u32());
            c.bus.write32(metrics + 0x58, r.next_u32());
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(metrics);
            c.g[A2][0] = sx(out);
            c.g[A3][0] = sx(DATA + 0x300);
        });
    }

    #[test]
    fn fe_popup_confirm_measure_line_w_matches() {
        check(ee::s03::f_001c8568__generated, fe_popup_confirm_measure_line_w, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1c85, 4);
            let metrics = DATA + 0x100;
            let out = DATA + 0x200;
            for k in 0..4 {
                c.bus.write32(metrics + 0x30 + 4 * k, r.edge_f32());
                c.bus.write32(metrics + 0x50 + 4 * k, r.edge_f32());
                c.bus.write32(out + 4 * k, r.edge_f32());
            }
            c.bus.write32(metrics + 0x14, r.next_u32());
            c.bus.write32(metrics + 0x58, r.next_u32());
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(metrics);
            c.g[A2][0] = sx(out);
            c.g[A3][0] = sx(DATA + 0x300);
        });
    }

                    #[test]
    fn ssx_audio_system_on_replay_begin_matches() {
        check(ee::s07::f_0028f200__generated, ssx_audio_system_on_replay_begin, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x28f2, 6);
            elf_image(c);
            let this = DATA;
            c.bus.write32(this + 0x530, [0u32, 1][(seed % 2) as usize]);
            c.bus.write32(this + 0x118, DATA + 0x2000);
            c.bus.write32(this + 0x62ac, r.next_u32());
            c.bus.write32(this + 0x508, r.next_u32());
            c.g[A0][0] = sx(this);
        });
    }

                    #[test]
    fn bolts_index_table3_by_char_matches() {
        check(ee::s01::f_0014c3c8__generated, bolts_index_table3_by_char, 40, &|c, seed| {
            let mut r = Rng(0x0014_c3c8 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let this = DATA;
            c.bus.write32(this + 0x24, (seed % 4) as u32);
            c.bus.write32(this + 0xc, DATA + 0x1000);
            for i in 0..4 {
                c.bus.write32(DATA + 0x1000 + 8 * i, r.next_u32());
                c.bus.write8(DATA + 0x1000 + 8 * i, (r.next_u32() % 5) as u8);
            }
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn bolts_index_table4_by_char_matches() {
        check(ee::s01::f_0014c488__generated, bolts_index_table4_by_char, 40, &|c, seed| {
            let mut r = Rng(0x0014_c488 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let this = DATA;
            c.bus.write32(this + 0x28, (seed % 4) as u32);
            c.bus.write32(this + 0x10, DATA + 0x1000);
            for i in 0..4 {
                c.bus.write32(DATA + 0x1000 + 8 * i, r.next_u32());
                c.bus.write8(DATA + 0x1000 + 8 * i, (r.next_u32() % 5) as u8);
            }
            c.g[A0][0] = sx(this);
        });
    }

        #[test]
    fn conveyor_ctor_matches() {
        check(ee::s11::f_00343010__generated, conveyor_ctor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x3430, 4);
            let this = DATA;
            let params = DATA + 0x1000;
            let dir = DATA + 0x2000;
            for k in 0..4 {
                c.bus.write32(dir + 4 * k, r.edge_f32());
                c.bus.write32(dir + 0x10 + 4 * k, (r.next_u32() % 256) as u32);
            }
            c.bus.write32(this + 0x18, DATA + 0x3000);
            c.bus.write32(DATA + 0x3000 + 8, r.next_u32());
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(r.next_u32());
            c.g[A2][0] = sx(params);
            c.g[A3][0] = sx(dir);
        });
    }

    #[test]
    fn crowd_man3d_ctor_matches() {
        check(ee::s11::f_00343a68__generated, crowd_man3d_ctor, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x343a, 4);
            let this = DATA;
            for k in 0..4 {
                c.bus.write32(this + 0x1f4 * k + 0x18, DATA + 0x8000);
            }
            c.g[A0][0] = sx(this);
            c.bus.write32(GAME_GP + 0xf00, r.next_u32());
        });
    }

                    #[test]
    fn ui_state_mgr_switch_state_matches() {
        check(ee::s13::f_0039f400__generated, ui_state_mgr_switch_state, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x39f4, 4);
            let mgr = DATA;
            let node = DATA + 0x1000;
            c.bus.write32(mgr, mgr);
            c.bus.write32(mgr + 4, node);
            c.bus.write32(node + 0x1c, r.next_u32());
            c.bus.write32(node + 4, mgr);
            c.g[A0][0] = sx(mgr);
            c.g[A1][0] = sx(node);
        });
    }

        #[test]
    fn game_camera_reset_all_matches() {
        check(ee::s01::f_0015db58__generated, game_camera_reset_all, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x15db, 8);
            let w = DATA + 0x1000;
            let x = DATA + 0x2000;
            let rm = DATA + 0x3000;
            c.bus.write32(GAME_GP - 0x848, w);
            c.bus.write32(w + 0x84, x);
            c.bus.write32(x + 0x84, rm);
            c.bus.write32(rm, [0u32, 1, 2][(seed % 3) as usize]);
            c.bus.write32(rm + 4, DATA + 0x4000);
            c.bus.write32(rm + 8, DATA + 0x5000);
            for cam in [DATA + 0x4000, DATA + 0x5000] {
                c.bus.write32(cam + 0xa8, DATA + 0x6000);
                c.bus.write32(DATA + 0x6000 + 0x14, DATA + 0x7000);
                c.bus.write16(DATA + 0x7000 + 0x20, 0);
                c.bus.write32(DATA + 0x7000 + 0x24, FAKE);
            }
            c.g[A0][0] = sx(DATA);
            c.bus.write32(0x004e_5830, r.next_u32());
        });
    }

        }
