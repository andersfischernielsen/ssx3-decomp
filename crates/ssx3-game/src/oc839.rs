//! Staging module oc839 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc839`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `StrHash(const char*)`.
const OC839_STR_HASH: u32 = 0x0031_7670;
/// `UIScreen_FindObject(screen, hash)`.
const OC839_FIND_OBJECT: u32 = 0x0039_d860;
/// `cUIText_setUnicodeString(text, str)`.
const OC839_SET_UNICODE: u32 = 0x003a_0e90;
/// `cUIText_setAsciiString(text, str)`.
const OC839_SET_ASCII: u32 = 0x003a_0c50;
/// `sprintf(buf, fmt, ...)`.
const OC839_SPRINTF: u32 = 0x0041_7828;

/// `StrHash(name)`, returning to `ret`.
fn oc839_hash(c: &mut Cpu, name: u32, ret: u32) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, OC839_STR_HASH, ret)
}

/// `UIScreen_FindObject(*(this+0x40), v0)`, returning to `ret`.
fn oc839_find(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, OC839_FIND_OBJECT, ret)
}

/// `StrHash(name)` then the find; the object is left in `v0`.
fn oc839_lookup(c: &mut Cpu, name: u32, ret_hash: u32, ret_find: u32) -> Flow {
    oc839_hash(c, name, ret_hash)?;
    oc839_find(c, ret_find)
}

/// Call the method at `*(*(s0+8) + off + 4)` on `s0` (`this` adjusted by the
/// halfword at `off`), `a1` already set.
fn oc839_vcall_s0(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(off));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(off + 4));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// The same call on the object in `v0` (the method pointer goes through
/// `a2`), `a1 = arg`.
fn oc839_vcall_v0(c: &mut Cpu, arg: u64, ret: u32) -> Flow {
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[A1][0] = arg;
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
    let t = c.g[A2][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// The world's string-table lookup: `v0` = world; `s0 = world->+0x8c`,
/// `s1 = s0->+4`, `s0 += s1->+0x20`, `v0 = StrHash(name)`, then
/// `s1->+0x24(s0, v0)`; the result ends up in `v0`.
fn oc839_world_text(c: &mut Cpu, name: u32, ret_hash: u32, ret_call: u32) -> Flow {
    c.g[A0][0] = sx(name);
    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    call_ptr(c, OC839_STR_HASH, ret_hash)?;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
    c.g[A1][0] = c.g[V0][0];
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, t, ret_call)
}

/// The shared tail of the type-1 and save branches: call the `+0x48` method
/// of the object in `s0` (`a1` set by the branch), `a0 = 0x460000`.
fn oc839_seven(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    oc839_vcall_s0(c, 0x48, 0x0018_dad4)?;
    c.g[A0][0] = 0x46_0000;
    Ok(())
}

/// `cFEStateOptionsDeviceSelect_Init(self)` (0x18d868): builds the memory
/// card device-select screen ("120MCselect"): hides some texts, sets the
/// title (load replay / save / load options), then fills the four memory
/// card slot texts ("%dmemory_card", "%d_b") with the "empty" string.
pub fn c_fe_state_options_device_select_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S2, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        c.g[A0][0] = 0x46_0000;
        frame.save(c, S4, 0x50);
        c.g[A0][0] = sx(0x46_0000u32.wrapping_sub(0x1ec0));
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save_ra(c, 0x20);
        frame.save(c, S0, 0x90);
        c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        call_ptr(c, OC839_STR_HASH, 0x0018_d8a0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_d8b4)?;
        let none = c.g[V0][0] == 0;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if none {
            oc839_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 36;
        c.g[A2][0] = 0;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0039_c870, 0x0018_d8cc)?;
        c.g[V1][0] = 0x4a_0000;
        c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x15d0));
        let name = c.g[S4][0] as u32;
        oc839_hash(c, name, 0x0018_d8dc)?;
        oc839_find(c, 0x0018_d8e8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        oc839_vcall_s0(c, 0x48, 0x0018_d904)?;
        c.g[V0][0] = 0x4a_0000;
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x15d8));
        let name = c.g[S3][0] as u32;
        oc839_hash(c, name, 0x0018_d914)?;
        oc839_find(c, 0x0018_d920)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        oc839_vcall_s0(c, 0x48, 0x0018_d93c)?;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
        c.g[V0][0] = 1;
        if c.g[A0][0] != c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x22c));
            c.cycles += 2;
            let load = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if !load {
                // Save screen.
                c.cycles += 32;
                oc839_world_text(c, 0x46_0000 - 0x1e88, 0x0018_da5c, 0x0018_da6c)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, OC839_STR_HASH, 0x0018_da78)?;
                oc839_find(c, 0x0018_da84)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                oc839_vcall_s0(c, 0x48, 0x0018_daa0)?;
                oc839_lookup(c, 0x46_0000 - 0x1e70, 0x0018_daac, 0x0018_dab8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                oc839_seven(c)?;
            } else {
                // Load screen.
                c.cycles += 63;
                oc839_world_text(c, 0x46_0000 - 0x1e60, 0x0018_daf8, 0x0018_db08)?;
                c.g[S1][0] = c.g[V0][0];
                let name = c.g[S3][0] as u32;
                oc839_hash(c, name, 0x0018_db14)?;
                oc839_find(c, 0x0018_db20)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                oc839_vcall_s0(c, 0x48, 0x0018_db3c)?;
                oc839_lookup(c, 0x46_0000 - 0x1e48, 0x0018_db48, 0x0018_db54)?;
                c.g[S0][0] = c.g[V0][0];
                oc839_hash(c, 0x4a_1460, 0x0018_db64)?;
                oc839_find(c, 0x0018_db70)?;
                oc839_vcall_v0(c, 0, 0x0018_db88)?;
                c.g[A1][0] = 1;
                oc839_vcall_s0(c, 0x48, 0x0018_dba0)?;
                oc839_lookup(c, 0x4a_1458, 0x0018_dbac, 0x0018_dbb8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                oc839_vcall_s0(c, 0x48, 0x0018_dbd4)?;
                c.g[A0][0] = 0x46_0000;
            }
        } else {
            // Replay-load screen.
            c.cycles += 59;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            oc839_world_text(c, 0x46_0000 - 0x1eb0, 0x0018_d96c, 0x0018_d97c)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, OC839_STR_HASH, 0x0018_d988)?;
            oc839_find(c, 0x0018_d994)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            oc839_vcall_s0(c, 0x48, 0x0018_d9b0)?;
            oc839_lookup(c, 0x46_0000 - 0x1e98, 0x0018_d9bc, 0x0018_d9c8)?;
            c.g[S0][0] = c.g[V0][0];
            oc839_hash(c, 0x4a_1460, 0x0018_d9d8)?;
            oc839_find(c, 0x0018_d9e4)?;
            oc839_vcall_v0(c, 0, 0x0018_d9fc)?;
            c.g[A1][0] = 1;
            oc839_vcall_s0(c, 0x48, 0x0018_da14)?;
            oc839_lookup(c, 0x4a_1458, 0x0018_da20, 0x0018_da2c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            oc839_seven(c)?;
        }
        // Title and slot texts.
        c.cycles += 46;
        c.g[S3][0] = 1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1e38));
        c.g[S6][0] = 0x46_0000;
        call_ptr(c, OC839_STR_HASH, 0x0018_dbe8)?;
        c.g[S5][0] = 0x4c_0000;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        c.g[S4][0] = 0x4a_0000;
        call_ptr(c, OC839_FIND_OBJECT, 0x0018_dbfc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, OC839_SET_UNICODE, 0x0018_dc08)?;
        oc839_lookup(c, 0x46_0000 - 0x1e28, 0x0018_dc14, 0x0018_dc20)?;
        c.g[S0][0] = 0x4a_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1408));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, OC839_SET_ASCII, 0x0018_dc34)?;
        oc839_lookup(c, 0x46_0000 - 0x1e18, 0x0018_dc40, 0x0018_dc4c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, OC839_SET_ASCII, 0x0018_dc58)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        oc839_world_text(c, 0x46_0000 - 0x1e00, 0x0018_dc78, 0x0018_dc88)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[SP][0];
        loop {
            c.cycles += 33;
            c.g[A2][0] = c.g[S3][0];
            c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1df0));
            call_ptr(c, OC839_SPRINTF, 0x0018_dc9c)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, OC839_STR_HASH, 0x0018_dca4)?;
            oc839_find(c, 0x0018_dcb0)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            call_ptr(c, OC839_SET_UNICODE, 0x0018_dcc8)?;
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x66c8));
            c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.ldr(2, c.g[A1][0] as u32);
            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
            c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
            c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x8f));
            c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x88));
            c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x97));
            c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x90));
            c.g[A1][0] = 1;
            oc839_vcall_s0(c, 0xc8, 0x0018_dd04)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
            c.g[V0][0] = 1;
            let replay = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = ((c.g[S3][0] as i64) < 2) as u64;
            if replay {
                c.cycles += 1;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 31;
                    c.g[A1][0] = 1;
                    oc839_vcall_s0(c, 0x40, 0x0018_dd34)?;
                    c.g[A1][0] = 0;
                    oc839_vcall_s0(c, 0x48, 0x0018_dd4c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x12c0));
                    call_ptr(c, OC839_SPRINTF, 0x0018_dd5c)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, OC839_STR_HASH, 0x0018_dd64)?;
                    oc839_find(c, 0x0018_dd70)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    oc839_vcall_s0(c, 0x48, 0x0018_dd8c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, OC839_SET_ASCII, 0x0018_dd98)?;
                    c.cycles += 1;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 5) as u64;
            let again = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[SP][0];
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0018_dc90);
            }
        }
        oc839_epilogue(c, frame);
        Ok(())
    })
}

fn oc839_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 10;
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0018_d868,
    c_fe_state_options_device_select_init,
    "cFEStateOptionsDeviceSelect_Init",
)];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA, GAME_GP, SP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word: the fakes below log and return only when it holds the magic.
    const OC839_CTRL: u32 = DATA + 0xe_0000;
    const OC839_MAGIC: u32 = 0x0c83_9a11;
    /// Word after the gate: what the screen-creation fake returns.
    const OC839_SCREEN: u32 = OC839_CTRL + 4;

    static OC839_PREV_ADD: AtomicUsize = AtomicUsize::new(0);
    static OC839_PREV_PLAY: AtomicUsize = AtomicUsize::new(0);
    static OC839_PREV_FIND: AtomicUsize = AtomicUsize::new(0);
    static OC839_PREV_ASCII: AtomicUsize = AtomicUsize::new(0);
    static OC839_PREV_UNICODE: AtomicUsize = AtomicUsize::new(0);
    static OC839_PREV_SPRINTF: AtomicUsize = AtomicUsize::new(0);

    /// The object every `UIScreen_FindObject` returns, with a vtable whose
    /// method slots point at the call fake.
    const OC839_OBJ: u32 = DATA + 0x3000;
    const OC839_VT: u32 = DATA + 0x3100;

    fn oc839_gated(c: &mut Cpu, target: u32, prev: &AtomicUsize, generated: GuestFn) -> bool {
        if c.bus.read32(OC839_CTRL) != OC839_MAGIC {
            fakes::fall_back(c, prev, generated);
            return false;
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        true
    }

    fn oc839_fake_add(c: &mut Cpu) {
        if oc839_gated(
            c,
            0x0039_7cc0,
            &OC839_PREV_ADD,
            ee::s13::f_00397cc0__generated,
        ) && c.pc == c.g[RA][0] as u32
        {
            c.g[V0][0] = c.bus.read32(OC839_SCREEN) as u64;
        }
    }
    fn oc839_fake_play(c: &mut Cpu) {
        oc839_gated(
            c,
            0x0039_c870,
            &OC839_PREV_PLAY,
            ee::s13::f_0039c870__generated,
        );
    }
    fn oc839_fake_find(c: &mut Cpu) {
        let ra = c.g[RA][0] as u32;
        if oc839_gated(
            c,
            0x0039_d860,
            &OC839_PREV_FIND,
            ee::s13::f_0039d860__generated,
        ) && c.pc == ra
        {
            c.g[V0][0] = OC839_OBJ as u64;
        }
    }
    fn oc839_fake_ascii(c: &mut Cpu) {
        oc839_gated(
            c,
            0x003a_0c50,
            &OC839_PREV_ASCII,
            ee::s13::f_003a0c50__generated,
        );
    }
    fn oc839_fake_unicode(c: &mut Cpu) {
        oc839_gated(
            c,
            0x003a_0e90,
            &OC839_PREV_UNICODE,
            ee::s13::f_003a0e90__generated,
        );
    }
    fn oc839_fake_sprintf(c: &mut Cpu) {
        oc839_gated(
            c,
            0x0041_7828,
            &OC839_PREV_SPRINTF,
            ee::s15::f_00417828__generated,
        );
    }

    fn oc839_install() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397CC0),
            oc839_fake_add,
            &OC839_PREV_ADD,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc839_fake_play,
            &OC839_PREV_PLAY,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc839_fake_find,
            &OC839_PREV_FIND,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C50),
            oc839_fake_ascii,
            &OC839_PREV_ASCII,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0E90),
            oc839_fake_unicode,
            &OC839_PREV_UNICODE,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc839_fake_sprintf,
            &OC839_PREV_SPRINTF,
        );
    }

    /// The memory-card device-select screen: `self` at DATA+0x400 with the
    /// type (+0x1bc: 1 = load replay) and save flag (+0x22c); the world's
    /// string table (`gp-0x848` -> +0x8c -> +4 -> method at +0x24 = fake) and
    /// a UI object whose vtable slots +0x44/+0x4c/+0xcc are fakes. Seeds pick
    /// the null-screen exit and the three branches.
    #[test]
    fn c_fe_state_options_device_select_init_matches() {
        check(
            ee::s02::f_0018d868__generated,
            c_fe_state_options_device_select_init,
            90,
            &|c, seed| {
                oc839_install();
                let _ = machine(c, seed, 0x18_d868, 40);
                c.bus.write32(OC839_CTRL, OC839_MAGIC);
                c.bus
                    .write32(OC839_SCREEN, if seed % 5 == 0 { 0 } else { DATA + 0x5000 });
                let this = DATA + 0x400;
                c.bus.write32(this + 0x10, DATA + 0x1000);
                let kind = match seed % 3 {
                    0 => 1,
                    1 => 0,
                    _ => 2,
                };
                c.bus.write32(this + 0x1bc, kind);
                c.bus
                    .write32(this + 0x22c, if (seed / 3) % 2 == 0 { 0 } else { 1 });
                // World -> string table -> entry whose method is the fake.
                let world = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x8c, DATA + 0x2100);
                c.bus.write32(DATA + 0x2104, DATA + 0x2200);
                c.bus.write16(DATA + 0x2220, 0x10);
                c.bus.write32(DATA + 0x2224, fakes::FAKE);
                // The UI object.
                c.bus.write32(OC839_OBJ + 8, OC839_VT);
                for off in [0x40u32, 0x48, 0xc8] {
                    c.bus.write16(OC839_VT + off, 0);
                    c.bus.write32(OC839_VT + off + 4, fakes::FAKE);
                }
                // An empty sprintf buffer on the stack (the fake writes nothing).
                for i in 0..8 {
                    c.bus.write32(SP - 0xa0 + 4 * i, 0);
                }
                c.g[A0][0] = this as u64;
            },
        );
    }
}
