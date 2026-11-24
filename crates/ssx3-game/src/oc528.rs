//! Staging module oc528 (wave wave231): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc528`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0025_01e8, avalanche_menu_ctor, "AvalancheMenu_ctor")];

/// `addiu rt, rs, imm`: `rs + imm`, sign-extended to 64 bits.
fn avalanche_ctor_addiu(c: &Cpu, rs: usize, imm: i32) -> u64 {
    sx((c.g[rs][0] as u32).wrapping_add(imm as u32))
}

/// The 32-bit address `rs + off`.
fn avalanche_ctor_at(c: &Cpu, rs: usize, off: u32) -> u32 {
    (c.g[rs][0] as u32).wrapping_add(off)
}

/// `cAvalancheMenu::cAvalancheMenu(this)` (0x2501e8): the menu's base
/// constructor, its vtable, then the title, the spacer and one item per
/// tuning value (int or float, each with its variable and range), and
/// finally every item added to the page in order. Returns `this`.
/// Stack: 0x130 bytes; the item addresses are kept in 0x4-byte slots at
/// `sp+0x0..` (title at slot 0, then one slot per item).
pub fn avalanche_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 508;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x100);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0xf0);
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 304);
        frame.save(c, S2, 0xe0);
        frame.save(c, S3, 0xd0);
        frame.save(c, S4, 0xc0);
        c.g[S2][0] = avalanche_ctor_addiu(c, S0, 1812);
        frame.save(c, S5, 0xb0);
        c.g[S4][0] = avalanche_ctor_addiu(c, S0, 1760);
        frame.save(c, S6, 0xa0);
        c.g[S5][0] = avalanche_ctor_addiu(c, S0, 1556);
        frame.save(c, S7, 0x90);
        c.g[S6][0] = avalanche_ctor_addiu(c, S0, 1660);
        frame.save(c, FP, 0x80);
        c.g[S7][0] = avalanche_ctor_addiu(c, S0, 1712);
        frame.save_ra(c, 0x70);
        c.g[FP][0] = avalanche_ctor_addiu(c, S0, 1452);
        frame.save_fpr(c, 23, 0x128);
        c.g[S3][0] = avalanche_ctor_addiu(c, S0, 1864);
        frame.save_fpr(c, 22, 0x120);
        c.g[S1][0] = avalanche_ctor_addiu(c, S0, 1916);
        frame.save_fpr(c, 21, 0x118);
        frame.save_fpr(c, 20, 0x110);
        c.sw(frame.at(c, 0), c.g[V0][0]);
        call_ptr(c, 0x002caa58, 0x00250254)?; // cMenu_cMenu (MenuSysPage_ctor)
        c.mtc1(0, 20);
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 332);
        c.g[A1][0] = 0x480000;
        c.sw(frame.at(c, 0x4), c.g[V0][0]);
        c.g[A1][0] = avalanche_ctor_addiu(c, A1, -640);
        c.g[V0][0] = 0x480000;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 21);
        c.g[V0][0] = avalanche_ctor_addiu(c, V0, 64);
        c.g[AT][0] = 0x447a0000;
        c.mtc1(1, 22);
        c.sw(avalanche_ctor_at(c, S0, 0x12c), c.g[V0][0]);
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 23);
        c.g[A0][0] = c.lw(frame.at(c, 0));
        call_ptr(c, 0x002cd2a0, 0x00250294)?; // cNullMenuItem ctor: TitleItem
        c.g[A0][0] = c.lw(frame.at(c, 0x4));
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 356);
        c.sw(frame.at(c, 0x8), c.g[V0][0]);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002cd350, 0x002502a8)?; // cSpaceMenuItem ctor: SpaceItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 404);
        c.g[A0][0] = c.lw(frame.at(c, 0x8));
        c.sw(frame.at(c, 0xc), c.g[V0][0]);
        c.g[T1][0] = 0x4a0000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 452);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, 11760);
        c.sw(frame.at(c, 0x10), c.g[V0][0]);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2368);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x002502d8)?; // cIntMenuItem ctor: EnableItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 504);
        c.g[A0][0] = c.lw(frame.at(c, 0xc));
        c.sw(frame.at(c, 0x14), c.g[V0][0]);
        c.g[T1][0] = 0x4a0000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 552);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, 11768);
        c.sw(frame.at(c, 0x18), c.g[V0][0]);
        c.g[T0][0] = 0x1;
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2372);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x00250308)?; // cIntMenuItem ctor: RecalcItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 600);
        c.g[A0][0] = c.lw(frame.at(c, 0x10));
        c.sw(frame.at(c, 0x1c), c.g[V0][0]);
        c.g[A3][0] = 0x480000;
        c.g[AT][0] = 0x40800000;
        c.mtc1(1, 13);
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 648);
        c.sw(frame.at(c, 0x20), c.g[V0][0]);
        c.mov_s(12, 20);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -624);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2376);
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002cdce8, 0x0025033c)?; // cFloatMenuItem ctor: BrightnessItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 696);
        c.g[A0][0] = c.lw(frame.at(c, 0x14));
        c.sw(frame.at(c, 0x24), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 744);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -608);
        c.sw(frame.at(c, 0x28), c.g[V0][0]);
        c.g[A1][0] = 0x3;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2380);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x0025036c)?; // cIntMenuItem ctor: UseLightMapColourItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 796);
        c.g[A0][0] = c.lw(frame.at(c, 0x18));
        c.sw(frame.at(c, 0x2c), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 844);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -584);
        c.sw(frame.at(c, 0x30), c.g[V0][0]);
        c.g[A1][0] = 0x4;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2384);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x0025039c)?; // cIntMenuItem ctor: UseEndColourItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 892);
        c.g[A0][0] = c.lw(frame.at(c, 0x1c));
        c.sw(frame.at(c, 0x34), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 940);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -568);
        c.sw(frame.at(c, 0x38), c.g[V0][0]);
        c.g[A1][0] = 0x5;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2388);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x002503cc)?; // cIntMenuItem ctor: SquareLerpItem
        c.g[A0][0] = c.lw(frame.at(c, 0x20));
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 988);
        c.g[T1][0] = 0x4a0000;
        c.sw(frame.at(c, 0x3c), c.g[V0][0]);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, 11776);
        c.g[A1][0] = 0x6;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2392);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x002503f4)?; // cIntMenuItem ctor: SaveItem
        c.g[A0][0] = c.lw(frame.at(c, 0x24));
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1040);
        c.g[T1][0] = 0x4a0000;
        c.sw(frame.at(c, 0x40), c.g[V0][0]);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, 11784);
        c.g[T0][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2396);
        c.g[A1][0] = 0x7;
        call_ptr(c, 0x002cd8f8, 0x0025041c)?; // cIntMenuItem ctor: LoadItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1092);
        c.g[A0][0] = c.lw(frame.at(c, 0x28));
        c.sw(frame.at(c, 0x44), c.g[V0][0]);
        c.g[A3][0] = 0x480000;
        c.g[AT][0] = 0x42f00000;
        c.mtc1(1, 13);
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1144);
        c.sw(frame.at(c, 0x48), c.g[V0][0]);
        c.mov_s(12, 20);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -544);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2400);
        c.g[A1][0] = 0x8;
        call_ptr(c, 0x002cdce8, 0x00250450)?; // cFloatMenuItem ctor: MinImpactVelocityItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1196);
        c.g[A0][0] = c.lw(frame.at(c, 0x2c));
        c.sw(frame.at(c, 0x4c), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1248);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -520);
        c.sw(frame.at(c, 0x50), c.g[V0][0]);
        c.g[A1][0] = 0x9;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2404);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x00250480)?; // cIntMenuItem ctor: RenderEmittersItem
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1300);
        c.g[A0][0] = c.lw(frame.at(c, 0x30));
        c.sw(frame.at(c, 0x54), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1348);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -504);
        c.sw(frame.at(c, 0x58), c.g[V0][0]);
        c.g[A1][0] = 0xa;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2408);
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x3f;
        call_ptr(c, 0x002cd8f8, 0x002504b0)?; // cIntMenuItem ctor: RenderLinesItem
        c.g[A0][0] = c.lw(frame.at(c, 0x34));
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1400);
        c.g[T1][0] = 0x480000;
        c.sw(frame.at(c, 0x5c), c.g[V0][0]);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -488);
        c.g[A1][0] = 0xb;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2412);
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x002504d8)?; // cIntMenuItem ctor: TriggerItem
        c.g[A0][0] = c.lw(frame.at(c, 0x38));
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1504);
        c.g[T1][0] = 0x4a0000;
        c.sw(frame.at(c, 0x60), c.g[V0][0]);
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, 11792);
        c.g[T0][0] = 0x1;
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2416);
        c.g[A1][0] = 0xc;
        call_ptr(c, 0x002cd8f8, 0x00250500)?; // cIntMenuItem ctor: UpdateItem
        c.g[A0][0] = c.lw(frame.at(c, 0x3c));
        c.g[V0][0] = avalanche_ctor_addiu(c, S0, 1608);
        c.g[A3][0] = 0x480000;
        c.sw(frame.at(c, 0x64), c.g[V0][0]);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -464);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2420);
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x002cdce8, 0x00250528)?; // cFloatMenuItem ctor: UpdateTimeStepItem
        c.g[A0][0] = c.lw(frame.at(c, 0x40));
        c.g[A3][0] = 0x480000;
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -440);
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2424);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002cdce8, 0x00250548)?; // cFloatMenuItem ctor: RotationalVelocityScaleItem
        c.g[A0][0] = c.lw(frame.at(c, 0x44));
        c.g[A3][0] = 0x480000;
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -408);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2428);
        c.g[A1][0] = 0xf;
        call_ptr(c, 0x002cdce8, 0x00250568)?; // cFloatMenuItem ctor: RandomBounceChanceItem
        c.g[A0][0] = c.lw(frame.at(c, 0x48));
        c.g[A3][0] = 0x480000;
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -384);
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2432);
        c.g[A1][0] = 0x10;
        call_ptr(c, 0x002cdce8, 0x00250588)?; // cFloatMenuItem ctor: RandomBounceVelocityScaleXYItem
        c.g[A0][0] = c.lw(frame.at(c, 0x4c));
        c.g[A3][0] = 0x480000;
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -352);
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2436);
        c.g[A1][0] = 0x11;
        call_ptr(c, 0x002cdce8, 0x002505a8)?; // cFloatMenuItem ctor: RandomBounceVelocityScaleZItem
        c.g[A0][0] = c.lw(frame.at(c, 0x50));
        c.g[A3][0] = 0x480000;
        c.mov_s(12, 21);
        c.mov_s(13, 22);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -320);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2440);
        c.g[A1][0] = 0x12;
        call_ptr(c, 0x002cdce8, 0x002505c8)?; // cFloatMenuItem ctor: RandomBounceVelocityScaleItem
        c.g[A0][0] = c.lw(frame.at(c, 0x54));
        c.g[T1][0] = 0x480000;
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -288);
        c.g[T0][0] = 0x3e8;
        c.g[A3][0] = 0u64;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2444);
        c.g[A1][0] = 0x13;
        call_ptr(c, 0x002cd8f8, 0x002505e8)?; // cIntMenuItem ctor: AvalancheIdItem
        c.g[A0][0] = c.lw(frame.at(c, 0x58));
        c.g[A3][0] = 0x4a0000;
        c.g[AT][0] = 0x42fe0000;
        c.mtc1(1, 13);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, 11800);
        c.mov_s(12, 20);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2448);
        c.g[A1][0] = 0x14;
        call_ptr(c, 0x002cdce8, 0x0025060c)?; // cFloatMenuItem ctor: GravityItem
        c.g[A0][0] = c.lw(frame.at(c, 0x5c));
        c.g[A3][0] = 0x480000;
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -272);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2452);
        c.g[A1][0] = 0x15;
        call_ptr(c, 0x002cdce8, 0x0025062c)?; // cFloatMenuItem ctor: VelocityDampingItem
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[FP][0];
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -248);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2456);
        c.g[A1][0] = 0x16;
        call_ptr(c, 0x002cdce8, 0x0025064c)?; // cFloatMenuItem ctor: NormalVelocityScaleItem
        c.g[A0][0] = c.lw(frame.at(c, 0x60));
        c.g[A3][0] = 0x480000;
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -224);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2460);
        c.g[A1][0] = 0x17;
        call_ptr(c, 0x002cdce8, 0x0025066c)?; // cFloatMenuItem ctor: ForwardVelocityScaleItem
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[S5][0];
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -200);
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2464);
        c.g[A1][0] = 0x18;
        call_ptr(c, 0x002cdce8, 0x0025068c)?; // cFloatMenuItem ctor: NormalToForwardScaleItem
        c.g[A0][0] = c.lw(frame.at(c, 0x64));
        c.g[A3][0] = 0x480000;
        c.g[AT][0] = 0x42ee0000;
        c.mtc1(1, 13);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -176);
        c.mov_s(12, 21);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2468);
        c.g[A1][0] = 0x19;
        call_ptr(c, 0x002cdce8, 0x002506b0)?; // cFloatMenuItem ctor: MaxVelocityItem
        c.g[A3][0] = 0x4a0000;
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A0][0] = c.g[S6][0];
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, 11808);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2472);
        c.g[A1][0] = 0x1a;
        call_ptr(c, 0x002cdce8, 0x002506d0)?; // cFloatMenuItem ctor: DampingItem
        c.g[T1][0] = 0x480000;
        c.g[T0][0] = 0x400;
        c.g[T1][0] = avalanche_ctor_addiu(c, T1, -160);
        c.g[A0][0] = c.g[S7][0];
        c.g[A3][0] = 0x1;
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2476);
        c.g[A1][0] = 0x1b;
        call_ptr(c, 0x002cd8f8, 0x002506f0)?; // cIntMenuItem ctor: NumParticlesItem
        c.g[A3][0] = 0x480000;
        c.mov_s(12, 21);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -144);
        c.g[A0][0] = c.g[S4][0];
        c.mov_s(13, 22);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2480);
        c.g[A1][0] = 0x1c;
        call_ptr(c, 0x002cdce8, 0x00250710)?; // cFloatMenuItem ctor: DefaultSizeItem
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -128);
        c.mov_s(12, 20);
        c.mov_s(13, 23);
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2484);
        c.g[A1][0] = 0x1d;
        call_ptr(c, 0x002cdce8, 0x00250730)?; // cFloatMenuItem ctor: PatchColourScaleItem
        c.g[A3][0] = 0x480000;
        c.mov_s(12, 20);
        c.mov_s(13, 23);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, -104);
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2488);
        c.g[A1][0] = 0x1e;
        call_ptr(c, 0x002cdce8, 0x00250750)?; // cFloatMenuItem ctor: PatchColourScale2Item
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 12);
        c.g[A3][0] = 0x4a0000;
        c.mov_s(13, 22);
        c.g[A3][0] = avalanche_ctor_addiu(c, A3, 11816);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = avalanche_ctor_addiu(c, GP, 2492);
        c.g[A1][0] = 0x1f;
        call_ptr(c, 0x002cdce8, 0x00250774)?; // cFloatMenuItem ctor: SeedItem
        c.g[A1][0] = c.lw(frame.at(c, 0));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250784)?; // cMenu_addItem: TitleItem
        c.g[A1][0] = c.lw(frame.at(c, 0x4));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250794)?; // cMenu_addItem: SpaceItem
        c.g[A1][0] = c.lw(frame.at(c, 0x8));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002507a4)?; // cMenu_addItem: EnableItem
        c.g[A1][0] = c.lw(frame.at(c, 0xc));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002507b4)?; // cMenu_addItem: RecalcItem
        c.g[A1][0] = c.lw(frame.at(c, 0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002507c4)?; // cMenu_addItem: BrightnessItem
        c.g[A1][0] = c.lw(frame.at(c, 0x14));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002507d4)?; // cMenu_addItem: UseLightMapColourItem
        c.g[A1][0] = c.lw(frame.at(c, 0x18));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002507e4)?; // cMenu_addItem: UseEndColourItem
        c.g[A1][0] = c.lw(frame.at(c, 0x1c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002507f4)?; // cMenu_addItem: SquareLerpItem
        c.g[A1][0] = c.lw(frame.at(c, 0x20));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250804)?; // cMenu_addItem: SaveItem
        c.g[A1][0] = c.lw(frame.at(c, 0x24));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250814)?; // cMenu_addItem: LoadItem
        c.g[A1][0] = c.lw(frame.at(c, 0x28));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250824)?; // cMenu_addItem: MinImpactVelocityItem
        c.g[A1][0] = c.lw(frame.at(c, 0x2c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250834)?; // cMenu_addItem: RenderEmittersItem
        c.g[A1][0] = c.lw(frame.at(c, 0x30));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250844)?; // cMenu_addItem: RenderLinesItem
        c.g[A1][0] = c.lw(frame.at(c, 0x34));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250854)?; // cMenu_addItem: TriggerItem
        c.g[A1][0] = c.lw(frame.at(c, 0x38));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250864)?; // cMenu_addItem: UpdateItem
        c.g[A1][0] = c.lw(frame.at(c, 0x3c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250874)?; // cMenu_addItem: UpdateTimeStepItem
        c.g[A1][0] = c.lw(frame.at(c, 0x40));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250884)?; // cMenu_addItem: RotationalVelocityScaleItem
        c.g[A1][0] = c.lw(frame.at(c, 0x44));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250894)?; // cMenu_addItem: RandomBounceChanceItem
        c.g[A1][0] = c.lw(frame.at(c, 0x48));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002508a4)?; // cMenu_addItem: RandomBounceVelocityScaleXYItem
        c.g[A1][0] = c.lw(frame.at(c, 0x4c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002508b4)?; // cMenu_addItem: RandomBounceVelocityScaleZItem
        c.g[A1][0] = c.lw(frame.at(c, 0x50));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002508c4)?; // cMenu_addItem: RandomBounceVelocityScaleItem
        c.g[A1][0] = c.lw(frame.at(c, 0x54));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002508d4)?; // cMenu_addItem: AvalancheIdItem
        c.g[A1][0] = c.lw(frame.at(c, 0x58));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002508e4)?; // cMenu_addItem: GravityItem
        c.g[A1][0] = c.lw(frame.at(c, 0x5c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002508f4)?; // cMenu_addItem: VelocityDampingItem
        c.g[A1][0] = c.g[FP][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250904)?; // cMenu_addItem: NormalVelocityScaleItem
        c.g[A1][0] = c.lw(frame.at(c, 0x60));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250914)?; // cMenu_addItem: ForwardVelocityScaleItem
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250924)?; // cMenu_addItem: NormalToForwardScaleItem
        c.g[A1][0] = c.lw(frame.at(c, 0x64));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250934)?; // cMenu_addItem: MaxVelocityItem
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250944)?; // cMenu_addItem: DampingItem
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250954)?; // cMenu_addItem: NumParticlesItem
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250964)?; // cMenu_addItem: DefaultSizeItem
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250974)?; // cMenu_addItem: PatchColourScaleItem
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250984)?; // cMenu_addItem: PatchColourScale2Item
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00250994)?; // cMenu_addItem: SeedItem
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S0, 0x100);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S7, 0x90);
        frame.restore(c, FP, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 23, 0x128);
        frame.restore_fpr(c, 22, 0x120);
        frame.restore_fpr(c, 21, 0x118);
        frame.restore_fpr(c, 20, 0x110);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, Rng, DATA};

    /// The menu lives at `DATA`; the ctor writes the page fields and every
    /// item, the rest of the object is random junk.
    #[test]
    fn avalanche_menu_ctor_matches() {
        check(
            ssx3_ee::s06::f_002501e8__generated,
            avalanche_menu_ctor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2501_e8, 0);
                fill(c, &mut r, DATA, 0x240);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
            },
        );
    }
}
