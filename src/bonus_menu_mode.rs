//! Native Event Select panel switching, including host catalog panels.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0020_30a0, event_select_set_mode, "EventSelect_SetMode"),
];

const SET_MAP_TAB_MODE: u32 = 0x0020_3cb8;
const STR_HASH: u32 = 0x0031_7670;
const FIND_OBJECT: u32 = 0x0039_d860;
const HIDE_STATE_OBJECT: u32 = 0x0039_e8b8;
const SET_TEXT_ID: u32 = 0x003a_1000;
const REFRESH_PEAK_LIST: u32 = 0x0020_3d40;
const REFRESH_PEAK_GOALS: u32 = 0x0020_5098;
const SCREEN: u32 = 0x2e8;
const STATE: u32 = 0x2ec;
const CONFIRM: u32 = 0x2cc;
const CONFIRM_NO_DISABLED: u32 = 0x2d0;

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> Addr {
    Addr((c.g[register][0] as u32).wrapping_add(offset))
}

#[inline]
fn global(c: &Cpu, negative_offset: u32) -> Addr {
    Addr((c.g[GP][0] as u32).wrapping_sub(negative_offset))
}

#[inline]
fn call_register(c: &mut Cpu, register: usize, ret: u32) -> Flow {
    let target = c.g[register][0] as u32;
    call_ptr(c, target, ret)
}

/// Find a screen object by the hash of the string already in a0.
fn find_object(c: &mut Cpu, hash_ret: u32, find_ret: u32) -> Flow {
    call_ptr(c, STR_HASH, hash_ret)?;
    c.g[A0][0] = at(c, S2, SCREEN).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, FIND_OBJECT, find_ret)
}

/// The initial hide pass uses v0 for the vtable and v1 for the target.
fn hide_object(c: &mut Cpu, register: usize, first_ret: u32, second_ret: u32) -> Flow {
    let object = c.g[register][0] as u32;
    c.g[V0][0] = Addr(object.wrapping_add(8)).lw(c);
    c.g[A0][0] = at(c, V0, 0x38).lh(c);
    c.g[V1][0] = at(c, V0, 0x3c).lw(c);
    c.g[A0][0] = sx(object.wrapping_add(c.g[A0][0] as u32));
    call_register(c, V1, first_ret)?;
    c.g[V0][0] = Addr(object.wrapping_add(8)).lw(c);
    c.g[A1][0] = 0;
    c.g[A0][0] = at(c, V0, 0x48).lh(c);
    c.g[V1][0] = at(c, V0, 0x4c).lw(c);
    c.g[A0][0] = sx(object.wrapping_add(c.g[A0][0] as u32));
    call_register(c, V1, second_ret)
}

/// Stack-held objects have extra loads in the original hide pass.
fn hide_stack_object(c: &mut Cpu, offset: u32, first_ret: u32, second_ret: u32) -> Flow {
    c.g[V0][0] = at(c, V0, 8).lw(c);
    c.g[A1][0] = 0;
    c.g[A0][0] = at(c, V0, 0x38).lh(c);
    c.g[V1][0] = at(c, V0, 0x3c).lw(c);
    c.g[V0][0] = at(c, SP, offset).lw(c);
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_register(c, V1, first_ret)?;
    c.g[V1][0] = at(c, SP, offset).lw(c);
    c.g[A1][0] = 0;
    c.g[V0][0] = at(c, V1, 8).lw(c);
    c.g[A0][0] = at(c, V0, 0x48).lh(c);
    c.g[V1][0] = at(c, V0, 0x4c).lw(c);
    c.g[V0][0] = at(c, SP, offset).lw(c);
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_register(c, V1, second_ret)
}

/// Mark the selected panel dirty, show it, and enable it. Its two virtual
/// methods use v1 for the vtable and v0 for the target in the original.
fn show_panel(c: &mut Cpu, register: usize, first_ret: u32, second_ret: u32) -> Flow {
    c.g[V0][0] = at(c, register, 0x14).lw(c);
    c.g[V1][0] = at(c, register, 8).lw(c);
    c.g[V0][0] |= 1;
    at(c, register, 0x14).sw(c, c.g[V0][0]);
    c.g[A0][0] = at(c, V1, 0x48).lh(c);
    c.g[V0][0] = at(c, V1, 0x4c).lw(c);
    c.g[A0][0] = sx((c.g[register][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_register(c, V0, first_ret)?;
    c.g[V1][0] = at(c, register, 8).lw(c);
    c.g[A1][0] = 1;
    c.g[A0][0] = at(c, V1, 0x38).lh(c);
    c.g[V0][0] = at(c, V1, 0x3c).lw(c);
    c.g[A0][0] = sx((c.g[register][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_register(c, V0, second_ret)
}

/// `void EventSelect_SetMode(EventSelect* self, int mode)` (0x2030a0).
///
/// Modes 0/1/2 select the peak list, the peak's Race/Freestyle/Freeride
/// goals, or that goal's event list. All panel choices are explicit; no
/// fourth-peak index is used to address the original three-peak data.
pub fn event_select_set_mode(c: &mut Cpu) {
    let this = c.g[A0][0] as u32;
    let mode = c.g[A1][0] as u32;
    let back = c.g[RA][0] as u32;
    original_set_mode(c);
    if c.pc != back || !crate::custom_tracks::assets_available() {
        return;
    }
    guest(c, |c| refresh_catalog_panels(c, this, mode, back));
}

/// Native panel refresh also changes the containers and subtitle. Apply the
/// custom choice after that pass while preserving its caller's registers.
pub fn refresh_catalog_panels(c: &mut Cpu, this: u32, mode: u32, back: u32) -> Flow {
    if !crate::custom_tracks::assets_available() { return Ok(()); }
    let registers = c.g;
    let floats = c.f;
    sync_catalog_panels(c, this, mode, back)?;
    c.g = registers;
    c.f = floats;
    c.pc = back;
    Ok(())
}

fn catalog_object(c: &mut Cpu, this: u32, name: &[u8], back: u32)
    -> Result<u32, crate::call::Unwound>
{
    c.g[A0][0] = sx(c.bus.read32(this.wrapping_add(SCREEN)));
    c.g[A1][0] = sx(crate::game::hashvalue::str_hash(name));
    call_ptr(c, FIND_OBJECT, back)?;
    Ok(c.g[V0][0] as u32)
}

fn catalog_panel(c: &mut Cpu, this: u32, name: &[u8], visible: bool, back: u32) -> Flow {
    let object = catalog_object(c, this, name, back)?;
    if object == 0 { return Ok(()) }
    if visible {
        let flags = c.bus.read32(object + 0x14);
        c.bus.write32(object + 0x14, flags | 1);
    }
    let slots = if visible { [0x48, 0x38] } else { [0x38, 0x48] };
    for slot in slots {
        let table = c.bus.read32(object + 8);
        let delta = c.bus.read16(table + slot) as i16 as i32;
        let target = c.bus.read32(table + slot + 4);
        c.g[A0][0] = sx(object.wrapping_add(delta as u32));
        c.g[A1][0] = visible as u64;
        call_ptr(c, target, back)?;
    }
    Ok(())
}

fn set_catalog_text(c: &mut Cpu, this: u32, name: &[u8], text: u32, back: u32) -> Flow {
    let row = catalog_object(c, this, name, back)?;
    if row != 0 {
        if c.bus.read32(row + 0xb0) == text && c.bus.read32(row + 0xb4) != 0 {
            return Ok(());
        }
        c.g[A0][0] = sx(row);
        c.g[A1][0] = sx(text);
        call_ptr(c, SET_TEXT_ID, back)?;
        c.g[A0][0] = sx(row);
        call_ptr(c, 0x003a_0528, back)?;
    }
    Ok(())
}

/// Request an original Tricky card through the native asynchronous image
/// slot. Added MAPGFX members avoid the three-entry retail peak table.
pub fn load_catalog_preview(c: &mut Cpu, this: u32, back: u32) -> Flow {
    let Some((variant, path)) = crate::custom_tracks::menu_preview() else {
        c.pc = back;
        return Ok(());
    };
    if c.bus.read32(this + 0x2d4) == variant {
        c.bus.write32(this + 0x2f0, 0);
        c.pc = back;
        return Ok(());
    }
    let saved = c.g;
    let floats = c.f;
    let gp = c.g[GP][0] as u32;
    let world = c.bus.read32(gp.wrapping_sub(0x848));
    let manager = c.bus.read32(world + 0x11c);
    let slot = c.bus.read32(this + 0x2d8);
    c.g[A0][0] = sx(manager);
    c.g[A1][0] = sx(slot);
    call_ptr(c, 0x001a_38c0, back)?;
    c.g[A0][0] = sx(manager);
    c.g[A1][0] = 0;
    c.g[A2][0] = sx(slot);
    c.g[A3][0] = 1;
    call_ptr(c, 0x001a_3768, back)?;
    let request = manager + slot * 0x11c;
    for (index, byte) in path.bytes().chain(std::iter::once(0)).enumerate() {
        c.bus.write8(request + index as u32, byte);
    }
    c.bus.write32(this + 0x2d4, variant);
    c.bus.write32(this + 0x2dc, 0);
    c.bus.write32(this + 0x2f0, 0);
    c.g = saved;
    c.f = floats;
    c.pc = back;
    Ok(())
}

pub fn refresh_catalog_preview(c: &mut Cpu, this: u32, back: u32) -> Flow {
    if crate::custom_tracks::menu_preview().is_none() { return Ok(()); }
    let saved = c.g;
    let floats = c.f;
    for name in [b"SP_Info".as_slice(), b"SP_GameInfo".as_slice(), b"SE_Info".as_slice()] {
        catalog_panel(c, this, name, false, back)?;
    }
    for name in [b"Indicators".as_slice(), b"peak1outline".as_slice(),
                 b"peak2outline".as_slice(), b"peak3outline   ".as_slice()] {
        catalog_panel(c, this, name, false, back)?;
    }
    let gp = c.g[GP][0] as u32;
    let mode = c.bus.read32(gp.wrapping_sub(0xb54));
    if mode == 0 {
        let row = catalog_object(c, this, b"Peak4", back)?;
        let highlight = catalog_object(c, this, b"phl", back)?;
        if row != 0 && highlight != 0 {
            let y = f32::from_bits(c.bus.read32(row + 0x48)) + 100.0;
            c.bus.write32(highlight + 0x48, y.to_bits());
            let flags = c.bus.read32(highlight + 0x14);
            c.bus.write32(highlight + 0x14, flags | 1);
        }
    }
    let tab = c.bus.read32(gp.wrapping_sub(0xb50));
    // PEAKno is the visible caption above the image. SubTitle is a
    // separate instruction widget; changing it does not update this label.
    let subtitle = if mode != 2 {
        crate::game::hashvalue::str_hash(b"kT_BonusPeakName")
    } else if tab == 1 {
        crate::game::hashvalue::str_hash(b"kT_TrickyFreestyle")
    } else { 0x032d_0353 };
    set_catalog_text(c, this, b"PEAKno", subtitle, back)?;
    c.g = saved;
    c.f = floats;
    c.pc = back;
    Ok(())
}

fn sync_catalog_panels(c: &mut Cpu, this: u32, mode: u32, back: u32) -> Flow {
    let gp = c.g[GP][0] as u32;
    let selected = crate::custom_tracks::bonus_peak_selected()
        && c.bus.read32(gp.wrapping_sub(0xb5c)) < 2
        && c.bus.read32(this.wrapping_add(CONFIRM)) == 0;
    let tab = c.bus.read32(gp.wrapping_sub(0xb50));
    let panels: [&[u8]; 3] = [b"SPG_BonusGoals", b"BonusRaceLocations", b"BonusFreestyleLocations"];
    for (i, name) in panels.into_iter().enumerate() {
        let visible = selected && ((i == 0 && mode == 1)
            || (mode == 2 && i == (if tab == 1 { 2 } else { 1 })));
        catalog_panel(c, this, name, visible, back)?;
    }
    if !selected || !(mode == 1 || mode == 2) { return Ok(()); }
    let hash = crate::game::hashvalue::str_hash;
    let original: &[u8] = if mode == 1 { b"SPG_Peak1Goals" }
        else if tab == 1 { b"Peak1FreestyleLocations" } else { b"Peak1RaceLocations" };
    catalog_panel(c, this, original, false, back)?;
    if mode == 1 {
        set_catalog_text(c, this, b"BonusRace", 0x032d_0353, back)?;
        set_catalog_text(c, this, b"BonusFreestyle", hash(b"kT_TrickyFreestyle"), back)?;
    } else {
        let kind = if tab == 1 { "Freestyle" } else { "Race" };
        for (index, track) in crate::custom_tracks::menu_tracks(tab).enumerate() {
            let name = format!("Tricky{kind}{index}");
            set_catalog_text(c, this, name.as_bytes(), hash(track.locale_key.as_bytes()), back)?;
        }
    }
    set_catalog_text(c, this, b"title_title", hash(b"kT_BonusPeakName"), back)?;
    let subtitle = if mode == 1 { hash(b"kT_BonusPeakName") }
        else if tab == 1 { hash(b"kT_TrickyFreestyle") } else { 0x032d_0353 };
    set_catalog_text(c, this, b"PEAKno", subtitle, back)?;
    Ok(())
}

fn original_set_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0xb0);
        frame.save(c, S2, 0x90);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, FP, 0x30);
        frame.save(c, S1, 0xa0);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        call_ptr(c, SET_MAP_TAB_MODE, 0x0020_30dc)?;
        c.g[A0][0] = 0x0047_0000;
        global(c, 0xb54).sw(c, c.g[S0][0]);
        c.g[A0][0] = sx(0x0047_0600);
        find_object(c, 0x0020_30ec, 0x0020_30f8)?;
        c.g[FP][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        if c.g[FP][0] != 0 {
            c.cycles += 11;
            hide_object(c, FP, 0x0020_3118, 0x0020_3130)?;
        }

        // SPG_Peak1Goals, SPG_Peak2Goals, SPG_Peak3Goals.
        for (name, register, hash_ret, find_ret, first_ret, second_ret) in [
            (0x0047_0918, S7, 0x0020_313c, 0x0020_3148, 0x0020_3168, 0x0020_3180),
            (0x0047_0928, S6, 0x0020_318c, 0x0020_3198, 0x0020_31b8, 0x0020_31d0),
            (0x0047_0938, S4, 0x0020_31dc, 0x0020_31e8, 0x0020_3208, 0x0020_3220),
        ] {
            c.cycles += 9;
            c.g[A0][0] = 0x0047_0000;
            c.g[A0][0] = sx(name);
            find_object(c, hash_ret, find_ret)?;
            c.g[register][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            if c.g[register][0] != 0 {
                c.cycles += 11;
                hide_object(c, register, first_ret, second_ret)?;
            }
        }

        // The three event lists of Peak 1 and Peak 2 are kept in the frame.
        for (name, offset, hash_ret, find_ret, first_ret, second_ret) in [
            (0x0047_0948, 0, 0x0020_322c, 0x0020_3238, 0x0020_325c, 0x0020_327c),
            (0x0047_0960, 4, 0x0020_3288, 0x0020_3294, 0x0020_32b8, 0x0020_32d8),
            (0x0047_0978, 8, 0x0020_32e4, 0x0020_32f0, 0x0020_3314, 0x0020_3334),
            (0x0047_0990, 0xc, 0x0020_3340, 0x0020_334c, 0x0020_3370, 0x0020_3390),
            (0x0047_09a8, 0x10, 0x0020_339c, 0x0020_33a8, 0x0020_33cc, 0x0020_33ec),
            (0x0047_09c0, 0x14, 0x0020_33f8, 0x0020_3404, 0x0020_3428, 0x0020_3448),
        ] {
            c.cycles += 8;
            c.g[A0][0] = 0x0047_0000;
            c.g[A0][0] = sx(name);
            find_object(c, hash_ret, find_ret)?;
            at(c, SP, offset).sw(c, c.g[V0][0]);
            if c.g[V0][0] != 0 {
                c.cycles += 15;
                hide_stack_object(c, offset, first_ret, second_ret)?;
            }
        }

        // Peak 3 Race/Freestyle/Freeride and the confirmation menu.
        for (name, register, hash_ret, find_ret, first_ret, second_ret) in [
            (0x0047_09d8, S5, 0x0020_3454, 0x0020_3460, 0x0020_3480, 0x0020_3498),
            (0x0047_09f0, S3, 0x0020_34a4, 0x0020_34b0, 0x0020_34d0, 0x0020_34e8),
            (0x0047_0a08, S1, 0x0020_34f4, 0x0020_3500, 0x0020_3520, 0x0020_3538),
            (0x0047_0a20, S0, 0x0020_3544, 0x0020_3550, 0x0020_3570, 0x0020_3588),
        ] {
            c.cycles += 9;
            c.g[A0][0] = 0x0047_0000;
            c.g[A0][0] = sx(name);
            find_object(c, hash_ret, find_ret)?;
            c.g[register][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            if c.g[register][0] != 0 {
                c.cycles += 11;
                hide_object(c, register, first_ret, second_ret)?;
            }
        }

        c.cycles += 3;
        c.g[V0][0] = at(c, S2, CONFIRM).lw(c);
        c.g[A1][0] = 0x004a_0000;
        if c.g[V0][0] != 0 {
            confirmation(c)?;
            frame.restore(c, S0, 0xb0);
            finish(c, frame, false, true);
            return Ok(());
        }

        c.cycles += 4;
        c.g[V1][0] = global(c, 0xb54).lw(c);
        c.g[A0][0] = 1;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        if c.g[V1][0] == 1 {
            let title_restored_s0 = goals_mode(c)?;
            finish(c, frame, !title_restored_s0, true);
            return Ok(());
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            let is_peak = c.g[V1][0] == 0;
            frame.restore(c, S0, 0xb0);
            if is_peak {
                let title_restored_s0 = peak_mode(c)?;
                finish(c, frame, !title_restored_s0, true);
            } else {
                c.cycles += 2;
                frame.restore(c, S1, 0xa0);
                finish(c, frame, false, false);
            }
        } else {
            c.g[V0][0] = 2;
            c.cycles += 2;
            let is_event = c.g[V1][0] == c.g[V0][0];
            frame.restore(c, S0, 0xb0);
            if is_event {
                event_mode(c)?;
                finish(c, frame, true, true);
            } else {
                c.cycles += 2;
                frame.restore(c, S1, 0xa0);
                finish(c, frame, false, false);
            }
        }
        Ok(())
    })
}

fn confirmation(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    c.g[A0][0] = at(c, S2, STATE).lw(c);
    c.g[A1][0] = sx(0x004a_2610); // Locks
    call_ptr(c, HIDE_STATE_OBJECT, 0x0020_35a0)?;
    c.g[V0][0] = at(c, S0, 8).lw(c);
    c.g[A1][0] = 1;
    c.g[A0][0] = at(c, V0, 0x38).lh(c);
    c.g[V1][0] = at(c, V0, 0x3c).lw(c);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_register(c, V1, 0x0020_35b8)?;
    c.g[V0][0] = at(c, S0, 8).lw(c);
    c.g[A1][0] = 1;
    c.g[A0][0] = at(c, V0, 0x48).lh(c);
    c.g[V1][0] = at(c, V0, 0x4c).lw(c);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_register(c, V1, 0x0020_35d0)?;
    c.g[V0][0] = at(c, S2, CONFIRM_NO_DISABLED).lw(c);
    c.g[A0][0] = 0x0047_0000;
    if c.g[V0][0] == 0 {
        c.cycles += 15;
        c.g[A0][0] = sx(0x0047_06c8); // ConfirmMenu_No
        find_object(c, 0x0020_35e4, 0x0020_35f0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[V0][0] = at(c, S0, 8).lw(c);
        c.g[A0][0] = at(c, V0, 0x48).lh(c);
        c.g[V1][0] = at(c, V0, 0x4c).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_register(c, V1, 0x0020_360c)?;
        c.g[V0][0] = at(c, S0, 8).lw(c);
        c.g[A1][0] = 0;
    } else {
        c.cycles += 14;
        c.g[A0][0] = sx(0x0047_06c8);
        find_object(c, 0x0020_3620, 0x0020_362c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[V0][0] = at(c, S0, 8).lw(c);
        c.g[A0][0] = at(c, V0, 0x48).lh(c);
        c.g[V1][0] = at(c, V0, 0x4c).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_register(c, V1, 0x0020_3648)?;
        c.g[V0][0] = at(c, S0, 8).lw(c);
        c.g[A1][0] = 1;
    }
    c.cycles += 6;
    c.g[A0][0] = at(c, V0, 0x40).lh(c);
    c.g[V1][0] = at(c, V0, 0x44).lw(c);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_register(c, V1, 0x0020_3660)
}

/// Returns whether the title tail already restored s0.
fn peak_mode(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, REFRESH_PEAK_LIST, 0x0020_36a8)?;
    c.g[A1][0] = 1;
    if c.g[FP][0] != 0 {
        c.cycles += 14;
        c.g[V0][0] = at(c, FP, 0x14).lw(c);
        c.g[V1][0] = at(c, FP, 8).lw(c);
        c.g[V0][0] |= 1;
        at(c, FP, 0x14).sw(c, c.g[V0][0]);
        c.g[A0][0] = at(c, V1, 0x38).lh(c);
        c.g[V0][0] = at(c, V1, 0x3c).lw(c);
        c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_register(c, V0, 0x0020_36d0)?;
        c.g[V1][0] = at(c, FP, 8).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = at(c, V1, 0x48).lh(c);
        c.g[V0][0] = at(c, V1, 0x4c).lw(c);
        c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_register(c, V0, 0x0020_36e8)?;
    }
    c.cycles += 4;
    c.g[V0][0] = global(c, 0xb5c).lw(c);
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    c.g[A0][0] = 0x0047_0000;
    if c.g[V0][0] == 0 { return Ok(false) }
    c.cycles += 9;
    c.g[A0][0] = sx(0x0047_0518); // title_title
    find_object(c, 0x0020_3700, 0x0020_370c)?;
    c.g[A0][0] = 0x0047_0000;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = sx(0x0047_07c0); // kT_240SelectPeak
    shared_title(c)?;
    Ok(true)
}

fn goals_mode(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 5;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, REFRESH_PEAK_GOALS, 0x0020_3724)?;
    c.g[V0][0] = global(c, 0xb4c).lw(c);
    let peak_zero = c.g[V0][0] == 0;
    c.g[V1][0] = global(c, 0xb4c).lw(c);
    let mut selected = false;
    if peak_zero {
        c.cycles += 2;
        c.g[A1][0] = 1;
        if c.g[S7][0] != 0 {
            c.cycles += 16;
            show_panel(c, S7, 0x0020_3758, 0x0020_3770)?;
            c.g[V0][0] = global(c, 0xb5c).lw(c);
            selected = true;
        }
    }
    if !selected {
        c.cycles += 3;
        c.g[V0][0] = 1;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 2;
            c.g[A1][0] = 1;
            if c.g[S6][0] != 0 {
                c.cycles += 16;
                show_panel(c, S6, 0x0020_37ac, 0x0020_37c4)?;
                c.g[V0][0] = global(c, 0xb5c).lw(c);
                selected = true;
            }
        }
    }
    if !selected {
        c.cycles += 2;
        c.g[A1][0] = 1;
        if c.g[S4][0] != 0 {
            c.cycles += 14;
            show_panel(c, S4, 0x0020_37f4, 0x0020_380c)?;
        }
        c.cycles += 1;
        c.g[V0][0] = global(c, 0xb5c).lw(c);
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    c.g[A0][0] = 0x0047_0000;
    if c.g[V0][0] == 0 { return Ok(false) }
    c.cycles += 8;
    c.g[A0][0] = sx(0x0047_0518);
    find_object(c, 0x0020_3824, 0x0020_3830)?;
    c.g[A0][0] = 0x0047_0000;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = sx(0x0047_0a30); // kT_FEHelpSelectMode
    shared_title(c)?;
    Ok(true)
}

fn shared_title(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    call_ptr(c, STR_HASH, 0x0020_3844)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, SET_TEXT_ID, 0x0020_3850)?;
    c.lq(S0, (c.g[SP][0] as u32).wrapping_add(0xb0));
    Ok(())
}

fn event_mode(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = global(c, 0xb4c).lw(c);
    let peak = c.g[V0][0];
    c.g[S0][0] = 0;
    if peak == 0 {
        c.cycles += 3;
        c.g[V0][0] = global(c, 0xb50).lw(c);
        let tab_zero = c.g[V0][0] == 0;
        c.g[S0][0] = at(c, SP, 0).lw(c);
        if !tab_zero {
            c.cycles += 2;
            let tab_one = c.g[V0][0] == c.g[A0][0];
            c.g[S0][0] = at(c, SP, 8).lw(c);
            if tab_one {
                c.cycles += 2;
                c.g[S0][0] = at(c, SP, 4).lw(c);
            } else {
                c.cycles += 2;
                c.g[V0][0] ^= 2;
                c.cycles += 1;
                if c.g[V0][0] != 0 { c.g[S0][0] = 0 }
            }
        }
    } else {
        c.cycles += 2;
        if c.g[V0][0] == c.g[A0][0] {
            c.cycles += 3;
            c.g[V0][0] = global(c, 0xb50).lw(c);
            let tab_zero = c.g[V0][0] == 0;
            c.g[S0][0] = at(c, SP, 0xc).lw(c);
            if !tab_zero {
                c.cycles += 2;
                let tab_one = c.g[V0][0] == c.g[A0][0];
                c.g[S0][0] = at(c, SP, 0x14).lw(c);
                if tab_one {
                    c.cycles += 2;
                    c.g[S0][0] = at(c, SP, 0x10).lw(c);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] ^= 2;
                    c.cycles += 1;
                    if c.g[V0][0] != 0 { c.g[S0][0] = 0 }
                }
            }
        } else {
            c.cycles += 2;
            let peak_two = c.g[V0][0] == c.g[V1][0];
            c.g[V0][0] = global(c, 0xb50).lw(c);
            if peak_two {
                c.cycles += 2;
                let tab_zero = c.g[V0][0] == 0;
                c.g[S0][0] = c.g[S5][0];
                if !tab_zero {
                    c.cycles += 2;
                    let tab_one = c.g[V0][0] == c.g[A0][0];
                    c.g[V0][0] ^= 2;
                    if tab_one {
                        c.cycles += 2;
                        c.g[S0][0] = c.g[S3][0];
                    } else {
                        c.cycles += 1;
                        c.g[S0][0] = c.g[S1][0];
                        c.cycles += 1;
                        if c.g[V0][0] != 0 { c.g[S0][0] = 0 }
                    }
                }
            }
        }
    }
    c.cycles += 2;
    c.g[A1][0] = 1;
    if c.g[S0][0] != 0 {
        c.cycles += 14;
        show_panel(c, S0, 0x0020_3904, 0x0020_391c)?;
    }
    c.cycles += 4;
    c.g[V0][0] = global(c, 0xb5c).lw(c);
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    c.g[A0][0] = 0x0047_0000;
    if c.g[V0][0] != 0 {
        c.cycles += 12;
        c.g[A0][0] = sx(0x0047_0518);
        find_object(c, 0x0020_3934, 0x0020_3940)?;
        c.g[A0][0] = 0x0047_0000;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sx(0x0047_0a48); // kT_240SelectEvent
        call_ptr(c, STR_HASH, 0x0020_3950)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, SET_TEXT_ID, 0x0020_395c)?;
    }
    Ok(())
}

fn finish(c: &mut Cpu, frame: Frame, restore_s0: bool, restore_s1: bool) {
    if restore_s0 {
        c.cycles += 1;
        frame.restore(c, S0, 0xb0);
    }
    if restore_s1 {
        c.cycles += 1;
        frame.restore(c, S1, 0xa0);
    }
    c.cycles += 10;
    frame.restore(c, S2, 0x90);
    frame.restore(c, S3, 0x80);
    frame.restore(c, S4, 0x70);
    frame.restore(c, S5, 0x60);
    frame.restore(c, S6, 0x50);
    frame.restore(c, S7, 0x40);
    frame.restore(c, FP, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
}
