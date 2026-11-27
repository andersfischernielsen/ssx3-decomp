//! Staging module oc1127 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1127`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0019_9458, cfe_state_char_equip_on_widget_created, "cFEStateCharEquip_OnWidgetCreated"),
    (0x001a_4330, cfe_state_net_messenger_on_widget_action, "cFEStateNetMessenger_OnWidgetAction"),
    (0x001a_54d8, cfe_state_net_messenger_add_new_buddies, "cFEStateNetMessenger_AddNewBuddies"),
];

/// `cFEStateCharEquip_OnWidgetCreated(self, widget)` (0x00199458): the character
/// equip screen's widget-created handler. Compares the widget's name hash with
/// the screen's widget names (title, help text, button text and icon, heading,
/// menu, totals, bolts, points, the six row widgets and six value widgets),
/// stores the widget in the matching `self` field and sets it up (title text,
/// hide, row refresh through SetBuyRow / SetEquipRow). Converted from the
/// generated `match pc` body: every register, cycle and call kept; any other
/// `pc` (resume after an unwound callee or event) runs the generated body.
pub fn cfe_state_char_equip_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x00199458 {
        return ee::s03::f_00199458__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x00199458;
    loop {
        match pc {
            0x00199458 => {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S1][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = 0x460000;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7f8));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = sx(0x00199498);
                c.g[S5][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                call_ptr(c, 0x00317670, 0x00199498)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x460000;
                if t { pc = 0x001994d8; continue; }
                pc = 0x001994a0; continue;
            }
            0x001994a0 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                let t = c.g[V0][0] != 0u64; 
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[S1][0]);
                if t { pc = 0x001994c4; continue; }
                pc = 0x001994b0; continue;
            }
            0x001994b0 => {
                c.cycles += 5;
                c.g[A0][0] = 0x460000;
                c.g[RA][0] = sx(0x001994bc);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x810));
                call_ptr(c, 0x00317670, 0x001994bc)?;
                let t = true; 
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                if t { pc = 0x00199540; continue; }
                pc = 0x001994c4; continue;
            }
            0x001994c4 => {
                c.cycles += 5;
                c.g[A0][0] = 0x460000;
                c.g[RA][0] = sx(0x001994d0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x820));
                call_ptr(c, 0x00317670, 0x001994d0)?;
                let t = true; 
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                if t { pc = 0x00199540; continue; }
                pc = 0x001994d8; continue;
            }
            0x001994d8 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001994e0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                call_ptr(c, 0x00317670, 0x001994e0)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x4a0000;
                if t { pc = 0x001994f8; continue; }
                pc = 0x001994e8; continue;
            }
            0x001994e8 => {
                c.cycles += 4;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[S1][0]);
                c.g[A1][0] = 0u64;
                let t = true; 
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                if t { pc = 0x00199818; continue; }
                pc = 0x001994f8; continue;
            }
            0x001994f8 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00199500);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1458));
                call_ptr(c, 0x00317670, 0x00199500)?;
                
                if c.g[S5][0] != c.g[V0][0] { c.g[A0][0] = 0x4a0000; pc = 0x00199550; continue; }
                pc = 0x00199508; continue;
            }
            0x00199508 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xae4));
                let t = c.g[V0][0] == 0u64; 
                c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[S1][0]);
                if t { pc = 0x00199810; continue; }
                pc = 0x00199514; continue;
            }
            0x00199514 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                
                if c.g[V0][0] != 0u64 { c.g[A0][0] = 0x460000; pc = 0x00199530; continue; }
                pc = 0x00199524; continue;
            }
            0x00199524 => {
                c.cycles += 3;
                c.g[A0][0] = 0x460000;
                let t = true; 
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x820));
                if t { pc = 0x00199534; continue; }
                pc = 0x00199530; continue;
            }
            0x00199530 => {
                c.cycles += 1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x810));
                pc = 0x00199534; continue;
            }
            0x00199534 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0019953c);
                call_ptr(c, 0x00317670, 0x0019953c)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                pc = 0x00199540; continue;
            }
            0x00199540 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00199548);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x00199548)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                if t { pc = 0x00199848; continue; }
                pc = 0x00199550; continue;
            }
            0x00199550 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00199558);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1460));
                call_ptr(c, 0x00317670, 0x00199558)?;
                
                if c.g[S5][0] == c.g[V0][0] { c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xae4)); pc = 0x0019958c; continue; }
                pc = 0x00199560; continue;
            }
            0x00199560 => {
                c.cycles += 5;
                c.g[A0][0] = 0x4a0000;
                c.g[RA][0] = sx(0x0019956c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1910));
                call_ptr(c, 0x00317670, 0x0019956c)?;
                
                if c.g[S5][0] == c.g[V0][0] { c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xae4)); pc = 0x0019958c; continue; }
                pc = 0x00199574; continue;
            }
            0x00199574 => {
                c.cycles += 5;
                c.g[A0][0] = 0x4a0000;
                c.g[RA][0] = sx(0x00199580);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1918));
                call_ptr(c, 0x00317670, 0x00199580)?;
                
                if c.g[S5][0] != c.g[V0][0] { c.g[A0][0] = 0x4a0000; pc = 0x0019959c; continue; }
                pc = 0x00199588; continue;
            }
            0x00199588 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xae4));
                pc = 0x0019958c; continue;
            }
            0x0019958c => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[S1][0]);
                if t { pc = 0x00199844; continue; }
                pc = 0x00199594; continue;
            }
            0x00199594 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                if t { pc = 0x00199814; continue; }
                pc = 0x0019959c; continue;
            }
            0x0019959c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001995a4);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1920));
                call_ptr(c, 0x00317670, 0x001995a4)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x460000;
                if t { pc = 0x001995c4; continue; }
                pc = 0x001995ac; continue;
            }
            0x001995ac => {
                c.cycles += 6;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[S1][0]);
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001995bc);
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f20));
                Callee { addr: 0x0019bbe0, f: ee::s03::cFEStateCharEquip_SetCategoryTitle }.call(c, 0x001995bc)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                if t { pc = 0x00199848; continue; }
                pc = 0x001995c4; continue;
            }
            0x001995c4 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001995cc);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x830));
                call_ptr(c, 0x00317670, 0x001995cc)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x460000;
                if t { pc = 0x001995ec; continue; }
                pc = 0x001995d4; continue;
            }
            0x001995d4 => {
                c.cycles += 6;
                c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x958));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001995e4);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[S1][0]);
                call_ptr(c, 0x0039b760, 0x001995e4)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                if t { pc = 0x00199848; continue; }
                pc = 0x001995ec; continue;
            }
            0x001995ec => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001995f4);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x840));
                call_ptr(c, 0x00317670, 0x001995f4)?;
                
                if c.g[S5][0] != c.g[V0][0] { c.g[A0][0] = 0x4a0000; pc = 0x00199614; continue; }
                pc = 0x001995fc; continue;
            }
            0x001995fc => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                let t = c.g[V0][0] == 0u64; 
                c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[S1][0]);
                if t { pc = 0x00199844; continue; }
                pc = 0x0019960c; continue;
            }
            0x0019960c => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                if t { pc = 0x00199814; continue; }
                pc = 0x00199614; continue;
            }
            0x00199614 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0019961c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1928));
                call_ptr(c, 0x00317670, 0x0019961c)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x4a0000;
                if t { pc = 0x00199638; continue; }
                pc = 0x00199624; continue;
            }
            0x00199624 => {
                c.cycles += 5;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[S1][0]);
                c.g[RA][0] = sx(0x00199630);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x0019bba0, f: ee::s03::cFEStateCharEquip_UpdateMoneyText }.call(c, 0x00199630)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                if t { pc = 0x00199848; continue; }
                pc = 0x00199638; continue;
            }
            0x00199638 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00199640);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1930));
                call_ptr(c, 0x00317670, 0x00199640)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x4a0000;
                if t { pc = 0x00199654; continue; }
                pc = 0x00199648; continue;
            }
            0x00199648 => {
                c.cycles += 3;
                c.g[A1][0] = 0x4a0000;
                let t = true; 
                c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[S1][0]);
                if t { pc = 0x0019971c; continue; }
                pc = 0x00199654; continue;
            }
            0x00199654 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0019965c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1938));
                call_ptr(c, 0x00317670, 0x0019965c)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x460000;
                if t { pc = 0x0019966c; continue; }
                pc = 0x00199664; continue;
            }
            0x00199664 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S0][0] as u32).wrapping_add(0x84), c.g[S1][0]);
                if t { pc = 0x00199844; continue; }
                pc = 0x0019966c; continue;
            }
            0x0019966c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00199674);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x850));
                call_ptr(c, 0x00317670, 0x00199674)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x4a0000;
                if t { pc = 0x0019968c; continue; }
                pc = 0x0019967c; continue;
            }
            0x0019967c => {
                c.cycles += 4;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), c.g[S1][0]);
                c.g[A1][0] = 0u64;
                let t = true; 
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                if t { pc = 0x00199818; continue; }
                pc = 0x0019968c; continue;
            }
            0x0019968c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00199694);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1940));
                call_ptr(c, 0x00317670, 0x00199694)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x460000;
                if t { pc = 0x001996ac; continue; }
                pc = 0x0019969c; continue;
            }
            0x0019969c => {
                c.cycles += 4;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                c.g[A1][0] = 0u64;
                let t = true; 
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                if t { pc = 0x00199818; continue; }
                pc = 0x001996ac; continue;
            }
            0x001996ac => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001996b4);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x860));
                call_ptr(c, 0x00317670, 0x001996b4)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x460000;
                if t { pc = 0x001996cc; continue; }
                pc = 0x001996bc; continue;
            }
            0x001996bc => {
                c.cycles += 4;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x74), c.g[S1][0]);
                c.g[A1][0] = 0u64;
                let t = true; 
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                if t { pc = 0x00199818; continue; }
                pc = 0x001996cc; continue;
            }
            0x001996cc => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001996d4);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x870));
                call_ptr(c, 0x00317670, 0x001996d4)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x460000;
                if t { pc = 0x001996e4; continue; }
                pc = 0x001996dc; continue;
            }
            0x001996dc => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[S1][0]);
                if t { pc = 0x00199844; continue; }
                pc = 0x001996e4; continue;
            }
            0x001996e4 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001996ec);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x880));
                call_ptr(c, 0x00317670, 0x001996ec)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x4a0000;
                if t { pc = 0x00199704; continue; }
                pc = 0x001996f4; continue;
            }
            0x001996f4 => {
                c.cycles += 4;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x80), c.g[S1][0]);
                c.g[A1][0] = 0u64;
                let t = true; 
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                if t { pc = 0x00199818; continue; }
                pc = 0x00199704; continue;
            }
            0x00199704 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0019970c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1948));
                call_ptr(c, 0x00317670, 0x0019970c)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x460000;
                if t { pc = 0x00199730; continue; }
                pc = 0x00199714; continue;
            }
            0x00199714 => {
                c.cycles += 2;
                c.g[A1][0] = 0x4a0000;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), c.g[S1][0]);
                pc = 0x0019971c; continue;
            }
            0x0019971c => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00199728);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                call_ptr(c, 0x003a0c50, 0x00199728)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                if t { pc = 0x00199848; continue; }
                pc = 0x00199730; continue;
            }
            0x00199730 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00199738);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x890));
                call_ptr(c, 0x00317670, 0x00199738)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                c.g[V1][0] = 0x440000;
                if t { pc = 0x00199760; continue; }
                pc = 0x00199740; continue;
            }
            0x00199740 => {
                c.cycles += 4;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x88), c.g[S1][0]);
                c.g[A1][0] = 0u64;
                let t = true; 
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                if t { pc = 0x00199818; continue; }
                pc = 0x00199750; continue;
            }
            0x00199750 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00199758);
                c.g[A1][0] = c.g[S4][0];
                Callee { addr: 0x0019ba60, f: ee::s03::cFEStateCharEquip_SetBuyRow }.call(c, 0x00199758)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                if t { pc = 0x00199848; continue; }
                pc = 0x00199760; continue;
            }
            0x00199760 => {
                c.cycles += 6;
                c.g[V0][0] = 0x440000;
                c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1230));
                c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1218));
                c.g[S4][0] = 0u64;
                c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
                pc = 0x00199778; continue;
            }
            0x00199778 => {
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S6][0] as u32));
                c.g[RA][0] = sx(0x00199784);
                c.g[S3][0] = sx((c.g[S4][0] as u32) << 2);
                call_ptr(c, 0x00317670, 0x00199784)?;
                let t = c.g[S5][0] != c.g[V0][0]; 
                
                if t { pc = 0x001997c8; continue; }
                pc = 0x0019978c; continue;
            }
            0x0019978c => {
                c.cycles += 6;
                c.sw((c.g[S2][0] as u32), c.g[S1][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[S4][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x958));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                
                if c.g[V0][0] != 0u64 { c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8)); pc = 0x001997f4; continue; }
                pc = 0x001997a4; continue;
            }
            0x001997a4 => {
                c.cycles += 9;
                c.g[A2][0] = c.lw((c.g[S2][0] as u32));
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001997c0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001997c0)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                if t { pc = 0x00199848; continue; }
                pc = 0x001997c8; continue;
            }
            0x001997c8 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001997d0);
                c.g[A0][0] = c.lw((c.g[S7][0] as u32));
                call_ptr(c, 0x00317670, 0x001997d0)?;
                
                if c.g[S5][0] != c.g[V0][0] { c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1)); pc = 0x00199830; continue; }
                pc = 0x001997d8; continue;
            }
            0x001997d8 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa4), c.g[S1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x958));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                
                if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8)); pc = 0x00199814; continue; }
                pc = 0x001997f0; continue;
            }
            0x001997f0 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                pc = 0x001997f4; continue;
            }
            0x001997f4 => {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                let t = c.g[V0][0] == 0u64; 
                c.g[A0][0] = c.g[S0][0];
                if t { if back_edge_unwinds(c, 0x00199750) { return Ok(()); } pc = 0x00199750; continue; }
                pc = 0x00199800; continue;
            }
            0x00199800 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00199808);
                c.g[A1][0] = c.g[S4][0];
                Callee { addr: 0x0019b7e0, f: ee::s03::cFEStateCharEquip_SetEquipRow }.call(c, 0x00199808)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                if t { pc = 0x00199848; continue; }
                pc = 0x00199810; continue;
            }
            0x00199810 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                pc = 0x00199814; continue;
            }
            0x00199814 => {
                c.cycles += 1;
                c.g[A1][0] = 0u64;
                pc = 0x00199818; continue;
            }
            0x00199818 => {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00199828);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00199828)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                if t { pc = 0x00199848; continue; }
                pc = 0x00199830; continue;
            }
            0x00199830 => {
                c.cycles += 5;
                c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[S4][0] as i64) < 6i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                if t { if back_edge_unwinds(c, 0x00199778) { return Ok(()); } pc = 0x00199778; continue; }
                pc = 0x00199844; continue;
            }
            0x00199844 => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                pc = 0x00199848; continue;
            }
            0x00199848 => {
                c.cycles += 10;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x00199458, 0x00199870); return Ok(()); }
        }
    }
    });
}

/// `cFEStateNetMessenger_OnWidgetAction(self, widget, msg)` (0x001a4330): the buddy
/// messenger's widget event switch (prev: tell the parent; misc: open the
/// find-player keyboard; next: select the buddy row; keyboard end: submit; gain
/// and lose focus: highlight, scroll and colour the `persona%02d` row), then
/// the base `NetFEState_OnWidgetAction`. Converted from the generated
/// `match pc` body; a resume `pc` runs the generated body.
pub fn cfe_state_net_messenger_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001a4330 {
        return ee::s03::f_001a4330__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x001a4330;
    loop {
        match pc {
            0x001a4330 => {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S6][0] = c.g[A2][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S3][0] = c.g[A1][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[V0][0] = (c.g[V1][0] < 0xdu64) as u64;
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                let t = c.g[V0][0] == 0u64; 
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
                if t { pc = 0x001a4630; continue; }
                pc = 0x001a4370; continue;
            }
            0x001a4370 => {
                c.cycles += 7;
                c.g[V0][0] = 0x460000;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x15c0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                let t = c.g[A0][0] as u32;
                
                pc = t; continue;
            }
            0x001a438c => {
                c.cycles += 10;
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0x14;
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001a43ac);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001a43ac)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                if t { pc = 0x001a4644; continue; }
                pc = 0x001a43b4; continue;
            }
            0x001a43b4 => {
                c.cycles += 29;
                c.g[A0][0] = 0x460000;
                c.g[RA][0] = sx(0x001a43c0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1540));
                call_ptr(c, 0x00317670, 0x001a43c0)?;
                c.g[A3][0] = 0x4a0000;
                c.g[A2][0] = c.g[V0][0];
                c.g[T0][0] = 0u64;
                c.g[T1][0] = 0x1;
                c.g[T2][0] = 0x10;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1408));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001a43e4);
                c.g[A1][0] = 0u64;
                Callee { addr: 0x001a9150, f: ee::s03::NetFEState_OpenKeyboardTextId }.call(c, 0x001a43e4)?;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x69c));
                c.g[V1][0] = 0x1;
                c.g[A1][0] = 0x4b;
                c.g[A2][0] = 0x1;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x438), c.g[V1][0]);
                c.g[A3][0] = 0x2;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x69c));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                c.g[RA][0] = sx(0x001a440c);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x69c));
                call_ptr(c, 0x001ce3c8, 0x001a440c)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x69c));
                c.g[A1][0] = 0x35;
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x001a4420);
                c.g[A3][0] = 0x2;
                call_ptr(c, 0x001ce3c8, 0x001a4420)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                if t { pc = 0x001a4644; continue; }
                pc = 0x001a4428; continue;
            }
            0x001a4428 => {
                c.cycles += 3;
                c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                let t = (c.g[S0][0] as i64) < 0; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a4640; continue; }
                pc = 0x001a4434; continue;
            }
            0x001a4434 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a4444; continue; }
                pc = 0x001a443c; continue;
            }
            0x001a443c => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a4444);
                call_ptr(c, 0x00260f80, 0x001a4444)?;
                pc = 0x001a4444; continue;
            }
            0x001a4444 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a444c);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a444c)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a4460; continue; }
                pc = 0x001a4454; continue;
            }
            0x001a4454 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a445c);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a445c)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a4460; continue;
            }
            0x001a4460 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a4474; continue; }
                pc = 0x001a4468; continue;
            }
            0x001a4468 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a4470);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a4470)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a4474; continue;
            }
            0x001a4474 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a447c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x002625c8, 0x001a447c)?;
                let t = c.g[V0][0] == 0u64; 
                c.g[A1][0] = c.g[V0][0];
                if t { pc = 0x001a4640; continue; }
                pc = 0x001a4484; continue;
            }
            0x001a4484 => {
                c.cycles += 10;
                c.g[RA][0] = sx(0x001a448c);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x001a9890, f: ee::s03::NetFEState_SetSelectedUser }.call(c, 0x001a448c)?;
                c.g[RA][0] = sx(0x001a4494);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x001a98b0, f: ee::s03::NetFEState_GetSelectedUser }.call(c, 0x001a4494)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a44a4);
                c.g[A2][0] = 0u64;
                Callee { addr: 0x001ab370, f: ee::s03::NetFEState_ShowBuddyActionPopup }.call(c, 0x001a44a4)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                if t { pc = 0x001a4644; continue; }
                pc = 0x001a44ac; continue;
            }
            0x001a44ac => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a44b4);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x001a4750, f: ee::s03::cFEStateNetMessenger_SubmitKeyboardInput }.call(c, 0x001a44b4)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                if t { pc = 0x001a4644; continue; }
                pc = 0x001a44bc; continue;
            }
            0x001a44bc => {
                c.cycles += 20;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001a44c8);
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                Callee { addr: 0x001a6070, f: ee::s03::cFEStateNetMessenger_ShowBuddyInfo }.call(c, 0x001a44c8)?;
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001a44d4);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x001a5fb8, f: ee::s03::cFEStateNetMessenger_HighlightRow }.call(c, 0x001a44d4)?;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6ec));
                c.g[A0][0] = c.g[S1][0];
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6e8));
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6f4));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[RA][0] = sx(0x001a44f4);
                c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
                Callee { addr: 0x001a5128, f: ee::s03::cFEStateNetMessenger_UpdateScrollBar }.call(c, 0x001a44f4)?;
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001a4504);
                c.g[A2][0] = 0x1;
                Callee { addr: 0x001a66a8, f: ee::s03::cFEStateNetMessenger_SetRowColour }.call(c, 0x001a4504)?;
                let t = true; 
                c.g[A0][0] = c.g[S1][0];
                if t { pc = 0x001a461c; continue; }
                pc = 0x001a450c; continue;
            }
            0x001a450c => {
                c.cycles += 3;
                c.g[S2][0] = 0u64;
                c.g[S4][0] = 0x460000;
                c.g[A0][0] = c.g[SP][0];
                pc = 0x001a4518; continue;
            }
            0x001a4518 => {
                c.cycles += 8;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x15a0));
                c.g[RA][0] = sx(0x001a4524);
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x00417828, 0x001a4524)?;
                c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001a4530);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x001a4530)?;
                
                if c.g[S0][0] == c.g[V0][0] { c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6f4)); if back_edge_unwinds(c, 0x001a44bc) { return Ok(()); } pc = 0x001a44bc; continue; }
                pc = 0x001a4538; continue;
            }
            0x001a4538 => {
                c.cycles += 4;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < 15i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.g[SP][0];
                if t { if back_edge_unwinds(c, 0x001a4518) { return Ok(()); } pc = 0x001a4518; continue; }
                pc = 0x001a4548; continue;
            }
            0x001a4548 => {
                c.cycles += 2;
                let t = true; 
                c.g[A0][0] = c.g[S1][0];
                if t { pc = 0x001a461c; continue; }
                pc = 0x001a4550; continue;
            }
            0x001a4550 => {
                c.cycles += 6;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.g[S2][0] = 0u64;
                c.g[S4][0] = 0x460000;
                c.g[S5][0] = 0x460000;
                c.g[A0][0] = c.g[SP][0];
                pc = 0x001a4568; continue;
            }
            0x001a4568 => {
                c.cycles += 8;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x15a0));
                c.g[RA][0] = sx(0x001a4574);
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x00417828, 0x001a4574)?;
                c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001a4580);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x001a4580)?;
                
                if c.g[S0][0] != c.g[V0][0] { c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1)); pc = 0x001a460c; continue; }
                pc = 0x001a4588; continue;
            }
            0x001a4588 => {
                c.cycles += 12;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001a4598);
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x15b0));
                call_ptr(c, 0x00417828, 0x001a4598)?;
                c.g[RA][0] = sx(0x001a45a0);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x001a45a0)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001a45ac);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001a45ac)?;
                c.g[A2][0] = c.g[V0][0];
                let t = c.g[A2][0] == 0u64; 
                c.g[A1][0] = 0u64;
                if t { pc = 0x001a45cc; continue; }
                pc = 0x001a45b8; continue;
            }
            0x001a45b8 => {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001a45cc);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001a45cc)?;
                pc = 0x001a45cc; continue;
            }
            0x001a45cc => {
                c.cycles += 16;
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001a45f4);
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001a45f4)?;
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001a4604);
                c.g[A2][0] = 0u64;
                Callee { addr: 0x001a66a8, f: ee::s03::cFEStateNetMessenger_SetRowColour }.call(c, 0x001a4604)?;
                let t = true; 
                c.g[A0][0] = c.g[S1][0];
                if t { pc = 0x001a461c; continue; }
                pc = 0x001a460c; continue;
            }
            0x001a460c => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 15i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.g[SP][0];
                if t { if back_edge_unwinds(c, 0x001a4568) { return Ok(()); } pc = 0x001a4568; continue; }
                pc = 0x001a4618; continue;
            }
            0x001a4618 => {
                c.cycles += 1;
                c.g[A0][0] = c.g[S1][0];
                pc = 0x001a461c; continue;
            }
            0x001a461c => {
                c.cycles += 5;
                c.g[A1][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x001a4628);
                c.g[A2][0] = c.g[S6][0];
                Callee { addr: 0x001a8918, f: ee::s03::NetFEState_OnWidgetAction }.call(c, 0x001a4628)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                if t { pc = 0x001a4644; continue; }
                pc = 0x001a4630; continue;
            }
            0x001a4630 => {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x001a4640);
                c.g[A2][0] = c.g[S6][0];
                Callee { addr: 0x001a8918, f: ee::s03::NetFEState_OnWidgetAction }.call(c, 0x001a4640)?;
                pc = 0x001a4640; continue;
            }
            0x001a4640 => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                pc = 0x001a4644; continue;
            }
            0x001a4644 => {
                c.cycles += 10;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x001a4330, 0x001a466c); return Ok(()); }
        }
    }
    });
}

/// `cFEStateNetMessenger_AddNewBuddies(self)` (0x001a54d8): makes sure the buddy
/// manager exists and is connected, then for every roster entry that is a
/// non-temporary name not yet listed pushes a new `cBXString` copy onto the
/// name vector (`+0x6e4`). Converted from the generated `match pc` body; a
/// resume `pc` runs the generated body.
pub fn cfe_state_net_messenger_add_new_buddies(c: &mut Cpu) {
    if c.pc != 0x001a54d8 {
        return ee::s03::f_001a54d8__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x001a54d8;
    loop {
        match pc {
            0x001a54d8 => {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[V0][0] != 0u64; 
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                if t { pc = 0x001a5504; continue; }
                pc = 0x001a54fc; continue;
            }
            0x001a54fc => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5504);
                call_ptr(c, 0x00260f80, 0x001a5504)?;
                pc = 0x001a5504; continue;
            }
            0x001a5504 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a550c);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a550c)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5520; continue; }
                pc = 0x001a5514; continue;
            }
            0x001a5514 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a551c);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a551c)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5520; continue;
            }
            0x001a5520 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5534; continue; }
                pc = 0x001a5528; continue;
            }
            0x001a5528 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5530);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a5530)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5534; continue;
            }
            0x001a5534 => {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe8));
                let t = c.g[V1][0] == 0u64; 
                c.g[S2][0] = 0u64;
                if t { pc = 0x001a56b0; continue; }
                pc = 0x001a5540; continue;
            }
            0x001a5540 => {
                c.cycles += 2;
                let t = true; 
                c.g[S3][0] = 0x460000;
                if t { pc = 0x001a565c; continue; }
                pc = 0x001a5548; continue;
            }
            0x001a5548 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a5558; continue; }
                pc = 0x001a5550; continue;
            }
            0x001a5550 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5558);
                call_ptr(c, 0x00260f80, 0x001a5558)?;
                pc = 0x001a5558; continue;
            }
            0x001a5558 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5560);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5560)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5574; continue; }
                pc = 0x001a5568; continue;
            }
            0x001a5568 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5570);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5570)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5574; continue;
            }
            0x001a5574 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5588; continue; }
                pc = 0x001a557c; continue;
            }
            0x001a557c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5584);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a5584)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5588; continue;
            }
            0x001a5588 => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x001a5590);
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x002625c8, 0x001a5590)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] == 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5658; continue; }
                pc = 0x001a559c; continue;
            }
            0x001a559c => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a55ac; continue; }
                pc = 0x001a55a4; continue;
            }
            0x001a55a4 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a55ac);
                call_ptr(c, 0x00260f80, 0x001a55ac)?;
                pc = 0x001a55ac; continue;
            }
            0x001a55ac => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a55b4);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a55b4)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a55c8; continue; }
                pc = 0x001a55bc; continue;
            }
            0x001a55bc => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a55c4);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a55c4)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a55c8; continue;
            }
            0x001a55c8 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a55dc; continue; }
                pc = 0x001a55d0; continue;
            }
            0x001a55d0 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a55d8);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a55d8)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a55dc; continue;
            }
            0x001a55dc => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a55e4);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00262438, 0x001a55e4)?;
                let t = c.g[V0][0] == 0u64; 
                c.g[A0][0] = c.g[S1][0];
                if t { pc = 0x001a5658; continue; }
                pc = 0x001a55ec; continue;
            }
            0x001a55ec => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a55f4);
                c.g[A1][0] = c.g[S0][0];
                Callee { addr: 0x001a56d0, f: ee::s03::cFEStateNetMessenger_HasName }.call(c, 0x001a55f4)?;
                
                if c.g[V0][0] != 0u64 { c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1)); pc = 0x001a565c; continue; }
                pc = 0x001a55fc; continue;
            }
            0x001a55fc => {
                c.cycles += 14;
                c.g[A0][0] = 0x4;
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1798));
                c.g[A2][0] = 0x20000000;
                c.g[RA][0] = sx(0x001a5610);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x001a5610)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a561c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003182d8, 0x001a561c)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6ec));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6f0));
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6e4));
                let t = c.g[A1][0] == c.g[V0][0]; 
                c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                if t { pc = 0x001a5650; continue; }
                pc = 0x001a5634; continue;
            }
            0x001a5634 => {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                
                if c.g[A0][0] != 0u64 { c.sw((c.g[A0][0] as u32), c.g[V1][0]); pc = 0x001a5640; continue; }
                pc = 0x001a5640; continue;
            }
            0x001a5640 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6ec));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                let t = true; 
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6ec), c.g[V0][0]);
                if t { pc = 0x001a5658; continue; }
                pc = 0x001a5650; continue;
            }
            0x001a5650 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5658);
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, 0x001db778, 0x001a5658)?;
                pc = 0x001a5658; continue;
            }
            0x001a5658 => {
                c.cycles += 1;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                pc = 0x001a565c; continue;
            }
            0x001a565c => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a5670; continue; }
                pc = 0x001a5668; continue;
            }
            0x001a5668 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5670);
                call_ptr(c, 0x00260f80, 0x001a5670)?;
                pc = 0x001a5670; continue;
            }
            0x001a5670 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5678);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5678)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a568c; continue; }
                pc = 0x001a5680; continue;
            }
            0x001a5680 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5688);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5688)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a568c; continue;
            }
            0x001a568c => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a569c; continue; }
                pc = 0x001a5694; continue;
            }
            0x001a5694 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a569c);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a569c)?;
                pc = 0x001a569c; continue;
            }
            0x001a569c => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x001a56a4);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00262598, 0x001a56a4)?;
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { if back_edge_unwinds(c, 0x001a5548) { return Ok(()); } pc = 0x001a5548; continue; }
                pc = 0x001a56b0; continue;
            }
            0x001a56b0 => {
                c.cycles += 7;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x001a54d8, 0x001a56cc); return Ok(()); }
        }
    }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fakes, Rng, DATA, GAME_GP};

    /// The fakes' gate word and its magic: each machine of a diff has its own
    /// RAM, and with the gate shut every slot runs the body it held before.
    pub(crate) const OC1127_CTRL: u32 = DATA + 0x2e_0000;
    pub(crate) const OC1127_MAGIC: u32 = 0x4f43_3b27;
    /// Bit (call index & 31) of this word answers the boolean / pointer fakes.
    pub(crate) const OC1127_MASK: u32 = OC1127_CTRL + 4;
    /// StrHash calls so far, the call that matches the widget, and the widget's hash.
    pub(crate) const OC1127_HCOUNT: u32 = OC1127_CTRL + 8;
    pub(crate) const OC1127_HTARGET: u32 = OC1127_CTRL + 0xc;
    pub(crate) const OC1127_HASHNAME: u32 = OC1127_CTRL + 0x10;
    /// What the roster-count fake returns, and the pointer the pointer fakes return.
    pub(crate) const OC1127_COUNTWORD: u32 = OC1127_CTRL + 0x14;
    pub(crate) const OC1127_PTRWORD: u32 = OC1127_CTRL + 0x18;

    const K_PLAIN: u32 = 0;
    const K_BOOL: u32 = 1;
    const K_PTR: u32 = 2;
    const K_COUNT: u32 = 3;
    const K_HASH: u32 = 4;

    /// A gated fake call to `target`: logs the registers, then answers by kind
    /// (bool bit, pointer or zero, count, or the widget hash on the chosen
    /// `StrHash` call and its complement on all others).
    pub(crate) fn oc1127_call(c: &mut Cpu, target: u32, kind: u32) {
        let ret = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, target);
        if c.pc != ret {
            return;
        }
        let bit = (c.bus.read32(OC1127_MASK) >> (n & 31)) & 1;
        match kind {
            K_BOOL => c.g[V0][0] = bit as u64,
            K_PTR => {
                c.g[V0][0] = if bit != 0 {
                    c.bus.read32(OC1127_PTRWORD) as u64
                } else {
                    0
                }
            }
            K_COUNT => c.g[V0][0] = c.bus.read32(OC1127_COUNTWORD) as u64,
            K_HASH => {
                let k = c.bus.read32(OC1127_HCOUNT);
                c.bus.write32(OC1127_HCOUNT, k + 1);
                let h = c.bus.read32(OC1127_HASHNAME);
                let h = if k == c.bus.read32(OC1127_HTARGET) {
                    h
                } else {
                    h ^ 1
                };
                c.g[V0][0] = sx(h);
            }
            _ => {}
        }
    }

    macro_rules! oc1127_fakes {
        ($install:ident; $(($fake:ident, $slot:expr, $gen:expr, $addr:expr, $kind:expr)),* $(,)?) => {
            $(
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1127_CTRL) != OC1127_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                    }
                    oc1127_call(c, $addr, $kind);
                }
            )*
            pub(crate) fn $install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc1127_fakes!(
        oc1127_install;
        (oc1127_fake_strhash, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670, K_HASH),
        (oc1127_fake_alloc, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated, 0x0031_7d70, K_PLAIN),
        (oc1127_fake_bxstring, ee::s10::OVERRIDE_003182D8, ee::s10::f_003182d8__generated, 0x0031_82d8, K_PLAIN),
        (oc1127_fake_sprintf, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated, 0x0041_7828, K_PLAIN),
        (oc1127_fake_find_object, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039_d860, K_PTR),
        (oc1127_fake_reset_scroll, ee::s13::OVERRIDE_0039B760, ee::s13::f_0039b760__generated, 0x0039_b760, K_PLAIN),
        (oc1127_fake_set_string, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated, 0x003a_0c50, K_PLAIN),
        (oc1127_fake_set_text_id, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated, 0x003a_1000, K_PLAIN),
        (oc1127_fake_buddy_create, ee::s06::OVERRIDE_00260F80, ee::s06::f_00260f80__generated, 0x0026_0f80, K_PLAIN),
        (oc1127_fake_buddy_connected, ee::s06::OVERRIDE_00261460, ee::s06::f_00261460__generated, 0x0026_1460, K_BOOL),
        (oc1127_fake_buddy_reconnect, ee::s06::OVERRIDE_002613C8, ee::s06::f_002613c8__generated, 0x0026_13c8, K_PLAIN),
        (oc1127_fake_lobby_disconnect, ee::s06::OVERRIDE_0025B800, ee::s06::f_0025b800__generated, 0x0025_b800, K_PLAIN),
        (oc1127_fake_roster_count, ee::s07::OVERRIDE_00262598, ee::s07::f_00262598__generated, 0x0026_2598, K_COUNT),
        (oc1127_fake_roster_entry, ee::s07::OVERRIDE_002625C8, ee::s07::f_002625c8__generated, 0x0026_25c8, K_PTR),
        (oc1127_fake_is_temp, ee::s07::OVERRIDE_00262438, ee::s07::f_00262438__generated, 0x0026_2438, K_BOOL),
        (oc1127_fake_vector_insert, ee::s04::OVERRIDE_001DB778, ee::s04::f_001db778__generated, 0x001d_b778, K_PLAIN),
        (oc1127_fake_set_key_enabled, ee::s04::OVERRIDE_001CE3C8, ee::s04::f_001ce3c8__generated, 0x001c_e3c8, K_PLAIN),
        (oc1127_fake_has_name, ee::s03::OVERRIDE_001A56D0, ee::s03::f_001a56d0__generated, 0x001a_56d0, K_BOOL),
        (oc1127_fake_open_keyboard, ee::s03::OVERRIDE_001A9150, ee::s03::f_001a9150__generated, 0x001a_9150, K_PLAIN),
        (oc1127_fake_set_selected_user, ee::s03::OVERRIDE_001A9890, ee::s03::f_001a9890__generated, 0x001a_9890, K_PLAIN),
        (oc1127_fake_get_selected_user, ee::s03::OVERRIDE_001A98B0, ee::s03::f_001a98b0__generated, 0x001a_98b0, K_PLAIN),
        (oc1127_fake_buddy_popup, ee::s03::OVERRIDE_001AB370, ee::s03::f_001ab370__generated, 0x001a_b370, K_PLAIN),
        (oc1127_fake_submit_keyboard, ee::s03::OVERRIDE_001A4750, ee::s03::f_001a4750__generated, 0x001a_4750, K_PLAIN),
        (oc1127_fake_show_buddy_info, ee::s03::OVERRIDE_001A6070, ee::s03::f_001a6070__generated, 0x001a_6070, K_PLAIN),
        (oc1127_fake_highlight_row, ee::s03::OVERRIDE_001A5FB8, ee::s03::f_001a5fb8__generated, 0x001a_5fb8, K_PLAIN),
        (oc1127_fake_update_scroll_bar, ee::s03::OVERRIDE_001A5128, ee::s03::f_001a5128__generated, 0x001a_5128, K_PLAIN),
        (oc1127_fake_set_row_colour, ee::s03::OVERRIDE_001A66A8, ee::s03::f_001a66a8__generated, 0x001a_66a8, K_PLAIN),
        (oc1127_fake_net_on_widget_action, ee::s03::OVERRIDE_001A8918, ee::s03::f_001a8918__generated, 0x001a_8918, K_PLAIN),
        (oc1127_fake_update_money, ee::s03::OVERRIDE_0019BBA0, ee::s03::f_0019bba0__generated, 0x0019_bba0, K_PLAIN),
        (oc1127_fake_set_category_title, ee::s03::OVERRIDE_0019BBE0, ee::s03::f_0019bbe0__generated, 0x0019_bbe0, K_PLAIN),
        (oc1127_fake_set_buy_row, ee::s03::OVERRIDE_0019BA60, ee::s03::f_0019ba60__generated, 0x0019_ba60, K_PLAIN),
        (oc1127_fake_set_equip_row, ee::s03::OVERRIDE_0019B7E0, ee::s03::f_0019b7e0__generated, 0x0019_b7e0, K_PLAIN),
    );

    /// A virtual-method slot `{delta: u16, fake}` at `vt + off` (the vtable
    /// layout the compiled `jalr` reads: halfword this-adjust, then the target).
    pub(crate) fn oc1127_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, n: u32) {
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * n);
    }

    /// Hook the callees (once per process), a scrambled machine with the gate
    /// open, a random bool/pointer mask and the entry `pc` set.
    pub(crate) fn oc1127_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc1127_install();
        let mut r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC1127_CTRL, OC1127_MAGIC);
        let mask = r.next_u32();
        c.bus.write32(OC1127_MASK, mask);
        c.pc = entry;
        r
    }

    const OC1127_SELF: u32 = DATA + 0x20_0000;
    const OC1127_W: u32 = DATA + 0x21_0000;
    const OC1127_VT_W: u32 = DATA + 0x22_0000;
    const OC1127_OBJ: u32 = DATA + 0x23_0000;
    const OC1127_VT_OBJ: u32 = DATA + 0x24_0000;
    const OC1127_PARENT: u32 = DATA + 0x25_0000;
    const OC1127_VT_PARENT: u32 = DATA + 0x26_0000;
    const OC1127_KB: u32 = DATA + 0x27_0000;
    const OC1127_SESSION: u32 = DATA + 0x28_0000;
    const OC1127_NAME: u32 = DATA + 0x2d_0000;
    const OC1127_FINISH: u32 = DATA + 0x2c_0000;

    /// `cFEStateCharEquip_OnWidgetCreated(self, widget)`: the widget's name
    /// hash picks which of the 24 `StrHash` comparisons matches (seed / 40 % 2
    /// is the buy/equip flag, seed / 80 the number of items), covering every
    /// arm including the two six-entry loops and no match at all.
    #[test]
    fn cfe_state_char_equip_on_widget_created_matches() {
        fakes::check_reaching(
            ee::s03::f_00199458__generated,
            cfe_state_char_equip_on_widget_created,
            720,
            &[0x0019_9498, 0x0019_9548, 0x0019_95bc, 0x0019_95e4, 0x0019_9630, 0x0019_9728, 0x0019_9758, 0x0019_9808, 0x0019_97c0, 0x0019_9828],
            &|c, seed| {
                let mut r = oc1127_machine(c, seed, 0x9945, 24, 0x0019_9458);
                c.g[A0][0] = sx(OC1127_SELF);
                c.g[A1][0] = sx(OC1127_W);
                let hash = r.next_u32() | 1;
                c.bus.write32(OC1127_W + 0x38, hash);
                c.bus.write32(OC1127_HASHNAME, hash);
                c.bus.write32(OC1127_HTARGET, seed % 40);
                c.bus.write32(OC1127_W + 8, OC1127_VT_W);
                oc1127_slot(c, &mut r, OC1127_VT_W, 0x48, 0);
                c.bus.write32(OC1127_SELF + 0xc8, (seed / 40) % 2);
                c.bus.write32(OC1127_SELF + 0xae4, r.next_u32() % 2);
                c.bus.write32(OC1127_SELF + 0x958, (seed / 80) % 9);
            },
        );
    }

    /// The jump table of the message switch (`msg - 1` indexes it) as the
    /// game image holds it at 0x4615c0.
    const OC1127_ACTION_TABLE: [u32; 13] = [
        0x001a_450c, 0x001a_4550, 0x001a_4630, 0x001a_4630, 0x001a_4428, 0x001a_438c,
        0x001a_4630, 0x001a_43b4, 0x001a_4630, 0x001a_4630, 0x001a_4630, 0x001a_4630,
        0x001a_44ac,
    ];

    /// `cFEStateNetMessenger_OnWidgetAction(self, widget, msg)`: every message
    /// (0..14; seed % 15), the `StrHash` call that matches the widget in the
    /// 15-row loops (seed / 15 % 17), a negative or valid buddy index, and
    /// the session / roster-entry / found-widget answers from the mask.
    #[test]
    fn cfe_state_net_messenger_on_widget_action_matches() {
        fakes::check_reaching(
            ee::s03::f_001a4330__generated,
            cfe_state_net_messenger_on_widget_action,
            600,
            &[0x001a_43ac, 0x001a_43e4, 0x001a_440c, 0x001a_448c, 0x001a_44a4, 0x001a_44b4, 0x001a_44c8, 0x001a_4504, 0x001a_4604, 0x001a_4628, 0x001a_4640, 0x001a_45cc, 0x001a_45f4],
            &|c, seed| {
                let mut r = oc1127_machine(c, seed, 0x4330, 40, 0x001a_4330);
                for (i, t) in OC1127_ACTION_TABLE.iter().enumerate() {
                    c.bus.write32(0x0046_15c0 + 4 * i as u32, *t);
                }
                c.g[A0][0] = sx(OC1127_SELF);
                c.g[A1][0] = sx(OC1127_W);
                c.g[A2][0] = (seed % 15) as u64;
                c.f[20] = r.next_u32();
                let hash = r.next_u32() | 1;
                c.bus.write32(OC1127_W + 0x38, hash);
                c.bus.write32(OC1127_HASHNAME, hash);
                c.bus.write32(OC1127_HTARGET, (seed / 15) % 17);
                let idx = if (seed / 255) % 2 == 0 { (seed % 7) as u32 } else { 0xffff_fffe };
                c.bus.write32(OC1127_W + 0x18, idx);
                c.bus.write32(OC1127_W + 8, OC1127_VT_W);
                oc1127_slot(c, &mut r, OC1127_VT_W, 0x58, 0);
                c.bus.write32(OC1127_SELF + 0x20, OC1127_PARENT);
                c.bus.write32(OC1127_PARENT + 8, OC1127_VT_PARENT);
                oc1127_slot(c, &mut r, OC1127_VT_PARENT, 0x88, 1);
                c.bus.write32(OC1127_SELF + 0x69c, OC1127_KB);
                c.bus.write32(OC1127_SELF + 0x40, DATA + 0x29_0000);
                c.bus.write32(OC1127_SELF + 0x6e8, DATA + 0x2a_0000);
                c.bus.write32(OC1127_SELF + 0x6ec, DATA + 0x2a_0000 + 4 * (r.next_u32() % 9));
                c.bus.write32(OC1127_SELF + 0x6f4, r.next_u32() % 20);
                c.bus.write32(OC1127_PTRWORD, OC1127_OBJ);
                c.bus.write32(OC1127_OBJ + 8, OC1127_VT_OBJ);
                oc1127_slot(c, &mut r, OC1127_VT_OBJ, 0x48, 2);
                let session = if seed % 3 == 0 { 0 } else { OC1127_SESSION };
                c.bus.write32(GAME_GP.wrapping_add(0x238), session);
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x2b_0000);
            },
        );
    }

    /// `cFEStateNetMessenger_AddNewBuddies(self)`: no session (created), a
    /// session that is or is not connected / logged in, roster counts 0..3,
    /// names that are missing, temporary or already listed, and a full or
    /// roomy name vector.
    #[test]
    fn cfe_state_net_messenger_add_new_buddies_matches() {
        fakes::check_reaching(
            ee::s03::f_001a54d8__generated,
            cfe_state_net_messenger_add_new_buddies,
            400,
            &[0x001a_5504, 0x001a_55f4, 0x001a_5610, 0x001a_561c, 0x001a_5658],
            &|c, seed| {
                let mut r = oc1127_machine(c, seed, 0x54d8, 24, 0x001a_54d8);
                c.g[A0][0] = sx(OC1127_SELF);
                let session = if seed % 5 == 0 { 0 } else { OC1127_SESSION };
                c.bus.write32(GAME_GP.wrapping_add(0x238), session);
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x2b_0000);
                c.bus.write32(OC1127_SESSION + 0xe8, (seed % 7 != 0) as u32);
                c.bus.write32(OC1127_COUNTWORD, (seed / 5) % 4);
                c.bus.write32(OC1127_PTRWORD, OC1127_NAME);
                let finish = if seed % 11 == 0 { 0 } else { OC1127_FINISH };
                c.bus.write32(OC1127_SELF + 0x6ec, finish);
                let cap = if (seed / 2) % 2 == 0 { finish } else { finish + 0x40 };
                c.bus.write32(OC1127_SELF + 0x6f0, cap);
                c.bus.write32(OC1127_SELF + 0x6e4, r.next_u32());
            },
        );
    }
}
