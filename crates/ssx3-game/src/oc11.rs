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
    (0x0022_9b90, crowd_man2d_clear_slot, "CrowdMan2d_ClearSlot"),
    (0x0022_d640, section_man_start_event_switch, "SectionMan_StartEventSwitch"),
    (0x0022_e078, section_man_is_section_loaded, "SectionMan_IsSectionLoaded"),
    (0x0022_e0c8, section_man_get_world_section_index, "SectionMan_GetWorldSectionIndex"),
    (0x0022_e278, section_man_pop_free_node, "SectionMan_PopFreeNode"),
    (0x0023_0698, game_module_get_transition_man, "GameModule_GetTransitionMan"),
    (0x0023_06a8, game_module_get_splash_manager, "GameModule_GetSplashManager"),
    (0x0023_1cd0, app_module_ctor, "AppModule_ctor"),
    (0x0023_1cf0, app_module_try_exit, "AppModule_TryExit"),
    (0x0023_2328, pre_start_screen_ctor, "PreStartScreen_ctor"),
    (0x0023_2720, c_pre_game_load_screen_ctor, "cPreGameLoadScreen_ctor"),
    (0x0023_2738, c_pre_fe_load_screen_ctor, "cPreFELoadScreen_ctor"),
    (0x0023_3c10, game_state_finish_ctor, "GameStateFinish_ctor"),
    (0x0023_4188, game_state_pre_race_load_ctor, "GameStatePreRaceLoad_ctor"),
    (0x0023_4ef8, game_state_reset_ctor, "GameStateReset_ctor"),
    (0x0023_6208, game_state_event_select_ctor, "GameStateEventSelect_ctor"),
    (0x0023_6868, game_state_event_switch_load_ctor, "GameStateEventSwitchLoad_ctor"),
    (0x0023_7060, input_sync_ctor, "InputSync_ctor"),
    (0x0023_c850, mc_manager_exists, "MCManager_Exists"),
    (0x0023_c898, mc_manager_is_idle, "MCManager_IsIdle"),
];

// ---------------------------------------------------------------------------
// Crowd / section manager.

/// `void CrowdMan2d_ClearSlot(CrowdMan2d *self, int slot)` (0x229b90): store
/// all-ones in the 32-bit slot at `self + slot*4 + 8`; leaves that address in
/// `a1` and all-ones in `v0`.
pub fn crowd_man2d_clear_slot(c: &mut Cpu) {
    const SLOT0: u32 = 0x8;
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] |= 0xffff; // -> all ones
    Addr((c.g[A1][0] as u32).wrapping_add(SLOT0)).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void SectionMan_StartEventSwitch(SectionMan *man, uint event)` (0x22d640):
/// if `event` is non-zero and no switch is already pending (`+0x1b4 == 0`),
/// record it at `+0x1b8` and set the pending word at `+0x1b4` to 1. `v0` is 1
/// on the non-zero path, untouched otherwise.
pub fn section_man_start_event_switch(c: &mut Cpu) {
    const PENDING: u32 = 0x1b4;
    const EVENT: u32 = 0x1b8;
    let man = Addr(abi::arg(c, 0));
    let event = c.g[A1][0];
    c.cycles += 2;
    if event != 0 {
        c.cycles += 3;
        let pending = man.off(PENDING).lw(c);
        c.g[V0][0] = 1;
        if pending == 0 {
            c.cycles += 2;
            man.off(EVENT).sw(c, event);
            man.off(PENDING).sw(c, c.g[V0][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool SectionMan_IsSectionLoaded(int index)` (0x22e078): true when the
/// load-state word of section `index` (record base 0x442168 + index*16, field
/// `+8`) is 1 or 2.
pub fn section_man_is_section_loaded(c: &mut Cpu) {
    const SECTIONS: u32 = 0x44_2168;
    const STATE: u32 = 0x8;
    c.cycles += 8;
    let idx = abi::arg(c, 1);
    c.g[A1][0] = sx(idx << 4);
    let base = SECTIONS.wrapping_add(idx << 4);
    c.g[V1][0] = sx(base);
    let state = Addr(base).off(STATE).lw(c);
    c.g[V0][0] = (sx((state as u32).wrapping_sub(1)) < 2) as u64;
    abi::ret(c);
}

/// `int SectionMan_GetWorldSectionIndex(int index)` (0x22e0c8): the world
/// section word of record `index` (record base 0x442168 + index*16, `+4`).
pub fn section_man_get_world_section_index(c: &mut Cpu) {
    const SECTIONS: u32 = 0x44_2168;
    const WORLD: u32 = 0x4;
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
    let base = SECTIONS.wrapping_add(c.g[A1][0] as u32);
    c.g[V1][0] = sx(base);
    c.g[V0][0] = Addr(base).off(WORLD).lw(c);
    abi::ret(c);
}

/// `void *SectionMan_PopFreeNode(SectionMan *man)` (0x22e278): pop the free
/// list head at `+0x198`, returning it and making its `next` (`+4`) the new
/// head.
pub fn section_man_pop_free_node(c: &mut Cpu) {
    const FREE_HEAD: u32 = 0x198;
    let man = Addr(abi::arg(c, 0));
    c.cycles += 4;
    let head = man.off(FREE_HEAD).lw(c);
    c.g[V0][0] = head;
    let next = Addr(head as u32).off(0x4).lw(c);
    c.g[V1][0] = next;
    man.off(FREE_HEAD).sw(c, next);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Game module.

/// `void *GameModule_GetTransitionMan(GameModule *self, int index)` (0x230698):
/// `self->transitions[index]` (word at `self + index*4 + 0x5c`).
pub fn game_module_get_transition_man(c: &mut Cpu) {
    const TRANSITIONS: u32 = 0x5c;
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    let slot = (c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32);
    c.g[A0][0] = sx(slot);
    c.g[V0][0] = Addr(slot).off(TRANSITIONS).lw(c);
    abi::ret(c);
}

/// `void *GameModule_GetSplashManager(GameModule *self, int index)` (0x2306a8):
/// `self->splash[index]` (word at `self + index*4 + 0x68`).
pub fn game_module_get_splash_manager(c: &mut Cpu) {
    const SPLASH: u32 = 0x68;
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    let slot = (c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32);
    c.g[A0][0] = sx(slot);
    c.g[V0][0] = Addr(slot).off(SPLASH).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// App module.

/// `AppModule *AppModule_ctor(AppModule *self)` (0x231cd0): install vtable
/// 0x47d980, zero the flag at `+8`, copy the float at `gp-0x51b8` into `+4`,
/// and return `self`.
pub fn app_module_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2680;
    const FADE: u32 = 0x4;
    const FLAG: u32 = 0x8;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 8;
    let fade = Addr((c.g[GP][0] as u32).wrapping_sub(0x51b8)).read_fpu(c);
    c.f[0] = fade.0;
    let vtable = sx(VTABLE);
    c.g[V0][0] = vtable;
    this.off(FLAG).sw(c, 0);
    this.sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.off(FADE).write_fpu(c, fade);
    abi::ret(c);
}

/// `bool AppModule_TryExit(AppModule *self)` (0x231cf0): mark the flag at `+8`
/// and report whether the fade value at `+4` has reached the threshold at
/// `gp-0x51b4` (the FPU condition is left set either way).
pub fn app_module_try_exit(c: &mut Cpu) {
    const FADE: u32 = 0x4;
    const FLAG: u32 = 0x8;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 7;
    let threshold = Addr((c.g[GP][0] as u32).wrapping_sub(0x51b4)).read_fpu(c); // f1
    c.f[1] = threshold.0;
    c.g[V0][0] = 1;
    let current = this.off(FADE).read_fpu(c); // f0
    c.f[0] = current.0;
    let done = threshold.le(current);
    abi::set_fcc(c, done);
    this.off(FLAG).sw(c, c.g[V0][0]);
    if !done {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Simple screen/state constructors: install a vtable and zero some fields.

/// `PreStartScreen *PreStartScreen_ctor(PreStartScreen *self)` (0x232328):
/// vtable 0x47d938, zero `+4` and `+8`, return `self`.
pub fn pre_start_screen_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x26c8;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 7;
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    this.sw(c, vtable);
    this.off(0x8).sw(c, 0);
    abi::ret(c);
}

/// `cPreGameLoadScreen *cPreGameLoadScreen_ctor(cPreGameLoadScreen *self)`
/// (0x232720): vtable 0x47d8a8, zero `+4`, return `self`.
pub fn c_pre_game_load_screen_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2758;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 6;
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    this.sw(c, vtable);
    abi::ret(c);
}

/// `cPreFELoadScreen *cPreFELoadScreen_ctor(cPreFELoadScreen *self)`
/// (0x232738): vtable 0x47d860, zero `+4`, return `self`.
pub fn c_pre_fe_load_screen_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x27a0;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 6;
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    this.sw(c, vtable);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Game-state constructors. Each installs two vtables back to back (only the
// second survives at the same slot), clears the global game-state pointer at
// `gp-0x488`, and fills a small set of state words.

/// `GameStateFinish *GameStateFinish_ctor(GameStateFinish *self)` (0x233c10):
/// state words `+0=7`, `+4=0`, `+0xc=0x47d5e0`, `+0x10=2`, `+0x14=0`.
pub fn game_state_finish_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2a20;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 16;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[A1][0] = 7;
    c.g[A2][0] = 2;
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 7);
    this.off(0x10).sw(c, 2);
    this.off(0x4).sw(c, 0);
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `GameStatePreRaceLoad *GameStatePreRaceLoad_ctor(GameStatePreRaceLoad *self)`
/// (0x234188): state words `+0=2`, `+4=0`, `+8=0`, `+0xc=0x47d530`,
/// `+0x10=0`, `+0x14=0`, `+0x1c=4`.
pub fn game_state_pre_race_load_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2ad0;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 18;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[A1][0] = 2;
    c.g[A2][0] = 4;
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 2);
    this.off(0x1c).sw(c, 4);
    this.off(0x4).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    this.off(0x8).sw(c, 0);
    abi::ret(c);
}

/// `GameStateReset *GameStateReset_ctor(GameStateReset *self)` (0x234ef8):
/// state words `+0=4`, `+4=0`, `+0xc=0x47d428`, `+0x10=5`, `+0x14=0`,
/// `+0x18=0`, `+0x1c=0`.
pub fn game_state_reset_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2bd8;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 18;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[A1][0] = 4;
    c.g[A2][0] = 5;
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 4);
    this.off(0x10).sw(c, 5);
    this.off(0x4).sw(c, 0);
    this.off(0x14).sw(c, 0);
    this.off(0x18).sw(c, 0);
    this.off(0x1c).sw(c, 0);
    abi::ret(c);
}

/// `GameStateEventSelect *GameStateEventSelect_ctor(GameStateEventSelect *self)`
/// (0x236208): state words `+0=4`, `+4=0`, `+8=0`, `+0xc=0x47d320`,
/// `+0x14=0`, `+0x18=0x17`.
pub fn game_state_event_select_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2ce0;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 17;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[A1][0] = 4;
    c.g[A2][0] = 0x17;
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 4);
    this.off(0x18).sw(c, 0x17);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `GameStateEventSwitchLoad *GameStateEventSwitchLoad_ctor(
/// GameStateEventSwitchLoad *self)` (0x236868): state words `+0=0xa`, `+4=0`,
/// `+0xc=0x47d2c8`.
pub fn game_state_event_switch_load_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2d38;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 13;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[V0][0] = 0xa;
    this.sw(c, 0xa);
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Input sync.

/// `InputSync *InputSync_ctor(InputSync *self, uint a1, int divisor, uint a3)`
/// (0x237060): `self+0x10 = 0`, `self+0x8 = divisor`, `self+0 = a1*divisor`,
/// `self+4 = (60/divisor)*divisor` (both products signed 32-bit, low word);
/// the divide and multiplies also leave LO/HI set.
pub fn input_sync_ctor(c: &mut Cpu) {
    const B: u32 = 0x4;
    const C: u32 = 0x8;
    const D: u32 = 0xc;
    const E: u32 = 0x10;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 13;
    // v1 is pre-loaded with the constant 60, the dividend of the `div`.
    c.g[V1][0] = 0x3c;
    this.off(B).sw(c, c.g[A1][0]); // a1 (overwritten at the end)
    let (q, _) = abi::div(c, 0x3c, c.g[A2][0] as u32); // div v1, a2
    this.off(C).sw(c, c.g[A2][0]);
    this.off(D).sw(c, 0);
    c.g[V0][0] = c.g[A0][0];
    this.off(E).sw(c, 0);
    c.g[V1][0] = sx(q);
    // Two scratch multiplies: `mult a1*a2` leaves the low word in a3, then
    // `mult v1*a2` leaves its low word back in v1 (and LO/HI).
    let a3 = abi::mult(c, c.g[A1][0] as u32, c.g[A2][0] as u32);
    c.g[A3][0] = sx(a3);
    let v = abi::mult(c, q, c.g[A2][0] as u32);
    c.g[V1][0] = sx(v);
    this.sw(c, c.g[A3][0]);
    c.cycles += 2;
    this.off(B).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Memory-card manager.

/// `bool MCManager_Exists(void)` (0x23c850): whether the global manager pointer
/// at `gp-0x47c` is non-zero.
pub fn mc_manager_exists(c: &mut Cpu) {
    c.cycles += 3;
    let p = Addr((c.g[GP][0] as u32).wrapping_sub(0x47c)).lw(c);
    c.g[V0][0] = (0u64 < p) as u64;
    abi::ret(c);
}

/// `bool MCManager_IsIdle(MCManager *self)` (0x23c898): true when the state
/// word at `+0x130` is exactly 1.
pub fn mc_manager_is_idle(c: &mut Cpu) {
    c.cycles += 4;
    let state = Addr(abi::arg(c, 0)).off(0x130).lw(c);
    c.g[V0][0] = ((state ^ 1) < 1) as u64;
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, check_paths, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn crowd_man2d_clear_slot_matches() {
        check(ssx3_ee::s05::f_00229b90, crowd_man2d_clear_slot, 64, &|c, seed| {
            let mut r = Rng(0x0c11_0001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 8) as u64;
        });
    }

    #[test]
    fn section_man_start_event_switch_matches() {
        check_paths(ssx3_ee::s05::f_0022d640, section_man_start_event_switch, 300, &[4, 7, 9], &|c, seed| {
            let mut r = Rng(0x0c11_0002 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let event = if seed % 3 == 0 {
                0
            } else {
                ((r.next_u32() as u64) << 32) | r.next_u32() as u64
            };
            c.g[A1][0] = event;
            c.bus.write32(DATA + 0x1b4, if seed % 2 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn section_man_is_section_loaded_matches() {
        check(ssx3_ee::s05::f_0022e078, section_man_is_section_loaded, 200, &|c, seed| {
            let mut r = Rng(0x0c11_0003 ^ seed as u64);
            scramble(c, &mut r);
            let idx = (seed % 16) as u32;
            c.g[A1][0] = idx as u64;
            let base = 0x44_2168u32.wrapping_add(idx << 4);
            let state = match seed % 6 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => 0xffff_ffff,
                _ => r.next_u32(),
            };
            c.bus.write32(base + 0x8, state);
        });
    }

    #[test]
    fn section_man_get_world_section_index_matches() {
        check(ssx3_ee::s05::f_0022e0c8, section_man_get_world_section_index, 100, &|c, seed| {
            let mut r = Rng(0x0c11_0004 ^ seed as u64);
            scramble(c, &mut r);
            let idx = (seed % 16) as u32;
            c.g[A1][0] = idx as u64;
            let base = 0x44_2168u32.wrapping_add(idx << 4);
            c.bus.write32(base + 0x4, r.next_u32());
        });
    }

    #[test]
    fn section_man_pop_free_node_matches() {
        check(ssx3_ee::s05::f_0022e278, section_man_pop_free_node, 200, &|c, seed| {
            let mut r = Rng(0x0c11_0005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            if seed % 3 == 0 {
                c.bus.write32(DATA + 0x198, 0);
            } else {
                let head = DATA + 0x100;
                c.bus.write32(DATA + 0x198, head);
                c.bus.write32(head + 0x4, r.next_u32());
            }
        });
    }

    #[test]
    fn game_module_get_transition_man_matches() {
        check(ssx3_ee::s06::f_00230698, game_module_get_transition_man, 100, &|c, seed| {
            let mut r = Rng(0x0c11_0006 ^ seed as u64);
            scramble(c, &mut r);
            let idx = (seed % 16) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = idx as u64;
            c.bus.write32(DATA + idx * 4 + 0x5c, r.next_u32());
        });
    }

    #[test]
    fn game_module_get_splash_manager_matches() {
        check(ssx3_ee::s06::f_002306a8, game_module_get_splash_manager, 100, &|c, seed| {
            let mut r = Rng(0x0c11_0007 ^ seed as u64);
            scramble(c, &mut r);
            let idx = (seed % 16) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = idx as u64;
            c.bus.write32(DATA + idx * 4 + 0x68, r.next_u32());
        });
    }

    #[test]
    fn app_module_ctor_matches() {
        check(ssx3_ee::s06::f_00231cd0, app_module_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_0008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x51b8), r.next_u32());
            for i in 0..3 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn app_module_try_exit_matches() {
        check_paths(ssx3_ee::s06::f_00231cf0, app_module_try_exit, 300, &[9, 10], &|c, seed| {
            let mut r = Rng(0x0c11_0009 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            let threshold = match seed % 4 {
                0 => 0x3f80_0000,
                1 => 0xc000_0000,
                2 => 0,
                _ => r.edge_f32(),
            };
            let current = if seed % 2 == 0 { 0x3f00_0000 } else { 0x4000_0000 };
            c.bus.write32(GAME_GP.wrapping_sub(0x51b4), threshold);
            c.bus.write32(DATA + 0x4, current);
        });
    }

    #[test]
    fn pre_start_screen_ctor_matches() {
        check(ssx3_ee::s06::f_00232328, pre_start_screen_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_000a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..3 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn c_pre_game_load_screen_ctor_matches() {
        check(ssx3_ee::s06::f_00232720, c_pre_game_load_screen_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_000b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..2 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn c_pre_fe_load_screen_ctor_matches() {
        check(ssx3_ee::s06::f_00232738, c_pre_fe_load_screen_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_000c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..2 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn game_state_finish_ctor_matches() {
        check(ssx3_ee::s06::f_00233c10, game_state_finish_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_000d ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
            for i in 0..8 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn game_state_pre_race_load_ctor_matches() {
        check(ssx3_ee::s06::f_00234188, game_state_pre_race_load_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_000e ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
            for i in 0..8 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn game_state_reset_ctor_matches() {
        check(ssx3_ee::s06::f_00234ef8, game_state_reset_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_000f ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
            for i in 0..8 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn game_state_event_select_ctor_matches() {
        check(ssx3_ee::s06::f_00236208, game_state_event_select_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_0010 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
            for i in 0..8 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn game_state_event_switch_load_ctor_matches() {
        check(ssx3_ee::s06::f_00236868, game_state_event_switch_load_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_0011 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
            for i in 0..4 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn input_sync_ctor_matches() {
        check_lohi(ssx3_ee::s06::f_00237060, input_sync_ctor, 300, &|c, seed| {
            let mut r = Rng(0x0c11_0012 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = match seed % 5 {
                0 => 0,
                1 => 1,
                2 => (r.next_u32() & 0xffff) as u64,
                3 => 0xffff_ffff,
                _ => ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
            };
            c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        });
    }

    #[test]
    fn mc_manager_exists_matches() {
        check(ssx3_ee::s06::f_0023c850, mc_manager_exists, 200, &|c, seed| {
            let mut r = Rng(0x0c11_0013 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let v = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => 0xffff_ffff,
                _ => r.next_u32(),
            };
            c.bus.write32(GAME_GP.wrapping_sub(0x47c), v);
        });
    }

    #[test]
    fn mc_manager_is_idle_matches() {
        check(ssx3_ee::s06::f_0023c898, mc_manager_is_idle, 200, &|c, seed| {
            let mut r = Rng(0x0c11_0014 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let v = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => 2,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + 0x130, v);
        });
    }
}
