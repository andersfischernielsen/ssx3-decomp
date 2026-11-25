//! Staging module oc643 (wave 234): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc643`.

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
///
/// Empty on purpose: `SSXApp_BootLoad` (0x2282d0) calls
/// `InputMap_LoadFromFile` (0x226b60), a 1496-line file parser with no
/// override slot, so its differential test cannot be bounded. The port is
/// kept (see `ssxapp_boot_load`) until that parser is ported.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

/// `lui 0x480000`: the string and global pool that the generated code
/// reaches with `0x480000 - off`.
const HI_DATA: u32 = 0x0048_0000;
/// `lui 0x4a0000`: the second string pool (`D_004A29xx`).
const HI_STR2: u32 = 0x004a_0000;
/// `lui 0x440000`: the texture-name table at `+0x2100`.
const HI_TEX: u32 = 0x0044_0000;
/// `lui 0x530000`: the BE option struct at `+0x5610` (copied by `Cfg_82D0()`).
const HI_BEOPT: u32 = 0x0053_0000;
/// Offset of the BE option struct from `HI_BEOPT` (the `Cfg_82D0()` source).
const BEOPT_OFF: u32 = 0x5610;

/// SSXApp fields the load writes or reads.
const SELF_BE: u32 = 0x78;
const SELF_FONTS: u32 = 0x88;
const SELF_ANIM_DB: u32 = 0x90;
const SELF_ANIM_MAP: u32 = 0x94;
const SELF_PADS: u32 = 0xa8;
const SELF_PLAYERS: u32 = 0xb0;
const SELF_INPUT_MAPS: u32 = 0xb8;
const SELF_CMDLINE_TRACK: u32 = 0xcc;
const SELF_BIG_TEX: u32 = 0xec;
const SELF_BIG_RWRD: u32 = 0x118;

/// The SSXApp pointer as the register holds it (read after every call).
#[inline(always)]
fn boot_self(c: &Cpu) -> u32 {
    c.g[S4][0] as u32
}

/// `if (BXFILE_exists(name)) FILESYS_addbigsync(name, 0, 100, &self->field)`:
/// one of the four generated blocks that open the audio big files. `pre` is
/// the cycles charged at the block head, `name_lo` the string at
/// `0x480000 - name_lo`, `field` the self field the handle goes to.
fn boot_big_if_exists(
    c: &mut Cpu,
    pre: u64,
    name_lo: u32,
    field: u32,
    ret_exists: u32,
    ret_add: u32,
) -> Flow {
    c.cycles += pre;
    c.g[V0][0] = HI_DATA as u64;
    c.g[S0][0] = sx(HI_DATA.wrapping_sub(name_lo));
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0031_7f38, ret_exists)?; // BXFILE_exists
    let found = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S0][0];
    if found {
        c.cycles += 4;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x64;
        c.g[A3][0] = sx(boot_self(c).wrapping_add(field));
        call_ptr(c, 0x003d_ed50, ret_add)?; // FILESYS_addbigsync
    }
    Ok(())
}

/// The inline copy of the 0x288-byte BE option snapshot (`Cfg_82D0()` by
/// value) into the stack buffer at SP, as the generated code does it. `src`
/// is the source register; its address is advanced in 32-byte steps. Aligned
/// sources (`src & 7 == 0`) use `ld`/`sd` with the registers `al`; unaligned
/// ones use `ldl`/`ldr`/`sdl`/`sdr` with `ua`. The 8-byte tail is copied by
/// the next block (see [`boot_cfg_tail`]). `heads` are the two loop heads for
/// the event checks.
fn boot_cfg_copy(c: &mut Cpu, src: usize, ua: [usize; 4], al: [usize; 4], heads: (u32, u32)) {
    c.g[V0][0] = c.g[src][0] & 0x7;
    let aligned = c.g[V0][0] == 0;
    c.g[V1][0] = c.g[SP][0];
    if !aligned {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[src][0] as u32).wrapping_add(0x280));
        loop {
            c.cycles += 19;
            let s = c.g[src][0] as u32;
            let d = c.g[V1][0] as u32;
            for k in 0..4 {
                let off = (8 * k) as u32;
                c.ldl(ua[k], s.wrapping_add(off + 7));
                c.ldr(ua[k], s.wrapping_add(off));
            }
            for k in 0..4 {
                let off = (8 * k) as u32;
                c.sdl(ua[k], d.wrapping_add(off + 7));
                c.sdr(ua[k], d.wrapping_add(off));
            }
            c.g[src][0] = sx(s.wrapping_add(0x20));
            let more = c.g[src][0] != c.g[V0][0];
            c.g[V1][0] = sx(d.wrapping_add(0x20));
            if more {
                if event_due(c) {
                    deliver_event_here(c, heads.0);
                }
                continue;
            }
            c.cycles += 2;
            return;
        }
    }
    c.cycles += 1;
    c.g[V0][0] = sx((c.g[src][0] as u32).wrapping_add(0x280));
    loop {
        c.cycles += 11;
        let s = c.g[src][0] as u32;
        let d = c.g[V1][0] as u32;
        for k in 0..4 {
            c.g[al[k]][0] = c.ld(s.wrapping_add((8 * k) as u32));
        }
        for k in 0..4 {
            c.sd(d.wrapping_add((8 * k) as u32), c.g[al[k]][0]);
        }
        c.g[src][0] = sx(s.wrapping_add(0x20));
        let more = c.g[src][0] != c.g[V0][0];
        c.g[V1][0] = sx(d.wrapping_add(0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, heads.1);
            }
            continue;
        }
        return;
    }
}

/// The 8-byte tail of a [`boot_cfg_copy`] (`ldl`/`ldr` from the end of the
/// source into `reg`, then `sdl`/`sdr` to SP+0x280). The caller charges the
/// block's cycles.
fn boot_cfg_tail(c: &mut Cpu, src: usize, reg: usize) {
    let s = c.g[src][0] as u32;
    let d = c.g[V1][0] as u32;
    c.ldl(reg, s.wrapping_add(7));
    c.ldr(reg, s);
    c.sdl(reg, d.wrapping_add(7));
    c.sdr(reg, d);
}

/// `cSSXApp_initload(SSXApp* self)` (0x2282d0, `SSXApp_BootLoad`): the game's
/// one-time load. Starts the net modules (`initOnline`), opens the big files
/// that exist, the scripts, the 11 character textures and `rwrdps2.big`,
/// creates the ScriptEngine, loads `input.map` and `input2.map` into the
/// input maps and gives both players a cInput on the first map, loads the
/// fonts, applies the display options and the locale, builds the animation
/// data, starts the audio with the saved volumes, inits the world triggers
/// and copies a non-empty command-line track.
///
/// `Cfg_82D0()` by value is the inline copy of the BE option struct, five
/// times: each `cSSXAudio_*` argument is read from the stack copy.
#[allow(dead_code)]
pub fn ssxapp_boot_load(c: &mut Cpu) {
    guest(c, ssxapp_boot_load_body);
}

#[allow(dead_code)]
fn ssxapp_boot_load_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x0022_82d0);
    c.cycles += 19;
    let frame = Frame::open(c, 0x330);
    frame.save(c, 20, 0x2e0);
    frame.save(c, 16, 0x320);
    frame.save(c, 17, 0x310);
    frame.save(c, 18, 0x300);
    frame.save(c, 19, 0x2f0);
    frame.save(c, 21, 0x2d0);
    frame.save(c, 22, 0x2c0);
    frame.save(c, 23, 0x2b0);
    frame.save(c, 30, 0x2a0);
    frame.save_ra(c, 0x290);

    // initOnline(): net modules.
    c.g[S4][0] = c.g[A0][0];
    Callee {
        addr: 0x0022_8ee0,
        f: ee::s05::f_00228ee0,
    }
    .call(c, 0x0022_8304)?;

    // The four audio big files (music, music2, audio, speech).
    boot_big_if_exists(c, 0, 0x57d8, 0xd8, 0x0022_8314, 0x0022_832c)?;
    boot_big_if_exists(c, 6, 0x57c0, 0xdc, 0x0022_833c, 0x0022_8354)?;
    boot_big_if_exists(c, 6, 0x57a8, 0xd4, 0x0022_8364, 0x0022_837c)?;
    boot_big_if_exists(c, 6, 0x5790, 0xe0, 0x0022_838c, 0x0022_83a4)?;

    // func_00256188(); scdat.big; D_004A29C8 (mdlps2.big).
    c.cycles += 21;
    c.g[RA][0] = sx(0x0022_83ac);
    c.g[S6][0] = sx(boot_self(c).wrapping_add(SELF_BIG_RWRD));
    call_ptr(c, 0x0025_6188, 0x0022_83ac)?;
    c.g[A0][0] = HI_DATA as u64;
    c.g[A1][0] = 0;
    c.g[A0][0] = sx(HI_DATA.wrapping_sub(0x5778));
    c.g[A2][0] = 0x64;
    c.g[A3][0] = sx(boot_self(c).wrapping_add(0xe4));
    c.g[S3][0] = sx(boot_self(c).wrapping_add(SELF_PLAYERS));
    call_ptr(c, 0x003d_ed50, 0x0022_83c8)?;
    c.g[S5][0] = sx(boot_self(c).wrapping_add(SELF_PADS));
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x728));
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x64;
    c.g[A3][0] = sx(boot_self(c).wrapping_add(0xe8));
    call_ptr(c, 0x003d_ed50, 0x0022_83e0)?;

    // The 11 character textures (D_00442100[i] into fEC[i]).
    c.g[FP][0] = sx(boot_self(c).wrapping_add(SELF_FONTS));
    c.g[V1][0] = HI_TEX as u64;
    c.g[S7][0] = HI_BEOPT as u64;
    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2100));
    c.g[S0][0] = sx(boot_self(c).wrapping_add(SELF_BIG_TEX));
    c.g[S1][0] = 0xa;
    loop {
        c.cycles += 9;
        c.g[A0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x64;
        call_ptr(c, 0x003d_ed50, 0x0022_840c)?;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
        let more = (c.g[S1][0] as i64) >= 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0022_83f8);
            }
            continue;
        }
        break;
    }

    // rwrdps2.big (D_004A29CC into f118).
    c.cycles += 29;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x724));
    c.g[A1][0] = 0;
    c.g[A3][0] = c.g[S6][0];
    c.g[A2][0] = 0x64;
    call_ptr(c, 0x003d_ed50, 0x0022_8430)?;

    // ScriptEngine: alloc, construct, InitOnce.
    c.g[S2][0] = c.g[S5][0];
    c.g[A1][0] = sx(HI_DATA.wrapping_sub(0x5730));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    c.g[A0][0] = 0xa50;
    c.g[S5][0] = 0x1;
    call_ptr(c, 0x0031_7d70, 0x0022_8450)?; // cMemMan_alloc
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0027_7f20, 0x0022_8458)?; // cSSXScriptEngine ctor
    c.g[A0][0] = c.g[V0][0];
    c.sw((c.g[GP][0] as u32).wrapping_sub(0x84c), c.g[V0][0]); // ScriptEngine = sm
    call_ptr(c, 0x0027_8210, 0x0022_8464)?; // cSSXScriptEngine_InitOnce

    // input.map and input2.map into the two input maps.
    c.g[A1][0] = HI_DATA as u64;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = sx(HI_DATA.wrapping_sub(0x5720));
    Callee {
        addr: 0x0022_6b60,
        f: ee::s05::f_00226b60,
    }
    .call(c, 0x0022_8474)?; // cSSXApp_loadInputMap
    c.g[A1][0] = HI_DATA as u64;
    c.sw(boot_self(c).wrapping_add(SELF_INPUT_MAPS), c.g[V0][0]);
    c.g[A1][0] = sx(HI_DATA.wrapping_sub(0x5708));
    c.g[A0][0] = c.g[S4][0];
    Callee {
        addr: 0x0022_6b60,
        f: ee::s05::f_00226b60,
    }
    .call(c, 0x0022_8488)?;
    c.sw(boot_self(c).wrapping_add(SELF_INPUT_MAPS + 4), c.g[V0][0]);

    // Two cInputs (setPad, setMap) on fB0[j], j = 0, 1.
    loop {
        c.cycles += 19;
        c.g[V0][0] = HI_STR2 as u64;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x8;
        c.g[A1][0] = sx(HI_STR2.wrapping_add(0x29d0));
        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(1));
        call_ptr(c, 0x0031_7d70, 0x0022_84ac)?; // cMemMan_alloc (str_Input)
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(c.g[S2][0] as u32);
        c.g[S1][0] = c.lw(boot_self(c).wrapping_add(SELF_INPUT_MAPS));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0024_4400, 0x0022_84c0)?; // cInput_setPad
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0024_4408, 0x0022_84d0)?; // cInput_setMap
        c.sw(c.g[S3][0] as u32, c.g[S0][0]);
        let more = (c.g[S5][0] as i64) >= 0;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0022_8490);
            }
            continue;
        }
        break;
    }

    // Font interface: alloc 0x44, zero the four entries and the counters.
    c.cycles += 11;
    c.g[A1][0] = HI_DATA as u64;
    c.g[A0][0] = 0x44;
    c.g[A1][0] = sx(HI_DATA.wrapping_sub(0x56f0));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x0022_84f4)?; // cMemMan_alloc (cUIFontInterface)
    c.sw(c.g[V0][0] as u32, 0);
    c.g[A1][0] = u64::MAX;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
    c.g[A0][0] = 0x3;
    loop {
        c.cycles += 7;
        c.sw(c.g[V1][0] as u32, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        c.sw((c.g[V1][0] as u32).wrapping_add(4), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(8), 0);
        let more = c.g[A0][0] != c.g[A1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0022_8508);
            }
            continue;
        }
        break;
    }

    // Font interface fields, then the two font entries.
    c.cycles += 78;
    c.sw(c.g[FP][0] as u32, c.g[V0][0]); // self->fonts = new interface
    c.g[A0][0] = sx(HI_STR2.wrapping_add(0x29d8));
    c.sb((c.g[V0][0] as u32).wrapping_add(0x34), 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x38), 0);
    c.g[S0][0] = HI_DATA as u64;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), 0);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x56d8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x40), 0);
    c.g[S1][0] = HI_DATA as u64;
    call_ptr(c, 0x0031_7670, 0x0022_8550)?; // name the interface (str_cUIFontInterface)
    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x56c0));
    c.g[A0][0] = c.lw(boot_self(c).wrapping_add(SELF_FONTS));
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0039_84b0, 0x0022_8568)?; // cUIFontInterface_addFontEntry(fefont)
    c.g[S2][0] = sx((c.g[S7][0] as u32).wrapping_add(BEOPT_OFF));
    c.g[A0][0] = sx(HI_STR2.wrapping_add(0x29e0));
    call_ptr(c, 0x0031_7670, 0x0022_8578)?;
    c.g[A0][0] = c.lw(boot_self(c).wrapping_add(SELF_FONTS));
    c.g[A1][0] = 1;
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[V0][0];
    call_ptr(c, 0x0039_84b0, 0x0022_858c)?; // cUIFontInterface_addFontEntry(hudfont)
    c.g[A0][0] = c.lw(boot_self(c).wrapping_add(SELF_FONTS));
    call_ptr(c, 0x0039_8568, 0x0022_8594)?; // cUIFontInterface_loadFonts
    call_ptr(c, 0x0014_dc80, 0x0022_859c)?; // cBE_getBE
    c.sw(boot_self(c).wrapping_add(SELF_BE), c.g[V0][0]);

    // BE options, the locale, the animation data.
    c.g[A0][0] = c.g[S4][0];
    Callee {
        addr: 0x0022_8c08,
        f: ee::s05::f_00228c08,
    }
    .call(c, 0x0022_85a8)?; // SSXApp_ApplyDisplayOptions
    c.g[A0][0] = c.g[S4][0];
    Callee {
        addr: 0x0022_8a78,
        f: ee::s05::f_00228a78,
    }
    .call(c, 0x0022_85b0)?; // SSXApp_LoadLocale
    call_ptr(c, 0x0031_1870, 0x0022_85b8)?; // cRiderAnimBase_initAIAnimEventMap
    c.g[A1][0] = HI_DATA as u64;
    c.g[A2][0] = 0;
    c.g[A1][0] = sx(HI_DATA.wrapping_sub(0x56a8));
    c.g[A3][0] = 0;
    c.g[A0][0] = 0x1848;
    call_ptr(c, 0x0031_7d70, 0x0022_85d0)?; // cMemMan_alloc (GameAnimMap)
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0031_4d98, 0x0022_85d8)?; // cGameAnimMap ctor
    c.g[A0][0] = c.g[V0][0];
    c.sw(boot_self(c).wrapping_add(SELF_ANIM_MAP), c.g[V0][0]);
    call_ptr(c, 0x0031_4dd8, 0x0022_85e4)?; // cGameAnimMap_initMap
    c.g[A1][0] = HI_DATA as u64;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    c.g[A1][0] = sx(HI_DATA.wrapping_sub(0x5698));
    c.g[A0][0] = 0x3fc;
    call_ptr(c, 0x0031_7d70, 0x0022_85fc)?; // cMemMan_alloc (AnimDataBase)
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0031_1048, 0x0022_8604)?; // cAnimDataBase ctor
    c.sw(boot_self(c).wrapping_add(SELF_ANIM_DB), c.g[V0][0]);
    call_ptr(c, 0x0014_dc80, 0x0022_860c)?; // cBE_getBE
    c.g[A1][0] = 4;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dd58, 0x0022_8618)?; // cBE_getInterface(be, 4)

    // Audio: SSXAUDIO_Init with the saved output mode, then the volumes.
    call_ptr(c, 0x0028_b180, 0x0022_8620)?; // cAudioInterface_bxAudio
    c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(BEOPT_OFF));
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 12);
    c.g[A1][0] = c.g[A1][0] & 0x3;
    call_ptr(c, 0x0028_9d08, 0x0022_8634)?; // cSSXAudio_BEAudioModeToAudioMode
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0028_4bb0, 0x0022_863c)?; // SSXAUDIO_Init
    call_ptr(c, 0x0031_a6b8, 0x0022_8644)?; // func_0031A6B8
    call_ptr(c, 0x0028_b180, 0x0022_864c)?; // cAudioInterface_bxAudio
    c.g[A0][0] = c.g[V0][0];

    // Cfg_82D0().sfxVol: copy A, its tail, then func_00287410.
    boot_cfg_copy(
        c,
        18,
        [5, 6, 7, 8],
        [9, 5, 6, 7],
        (0x0022_8660, 0x0022_86b8),
    );
    c.cycles += 14;
    boot_cfg_tail(c, 18, 8);
    c.g[A1][0] = c.lw(sp(c)) & 0xf;
    call_ptr(c, 0x0028_7410, 0x0022_8700)?; // func_00287410 (sfxVol)

    // Cfg_82D0().speechVol: copy B, its tail, then cSSXAudio_SetSFXVolume.
    call_ptr(c, 0x0028_b180, 0x0022_8708)?; // cAudioInterface_bxAudio
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(HI_BEOPT.wrapping_add(BEOPT_OFF));
    boot_cfg_copy(c, 5, [6, 7, 8, 9], [6, 7, 8, 9], (0x0022_8720, 0x0022_8778));
    c.cycles += 15;
    boot_cfg_tail(c, 5, 2);
    c.g[A1][0] = sx((c.lw(sp(c)) as u32) >> 8);
    c.g[A1][0] = c.g[A1][0] & 0xf;
    call_ptr(c, 0x0028_7488, 0x0022_87c4)?; // cSSXAudio_SetSFXVolume (speechVol)

    // Cfg_82D0().musicVol: copy C, its tail, then func_00287520.
    call_ptr(c, 0x0028_b180, 0x0022_87cc)?; // cAudioInterface_bxAudio
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(HI_BEOPT.wrapping_add(BEOPT_OFF));
    boot_cfg_copy(c, 5, [9, 6, 7, 8], [9, 6, 7, 8], (0x0022_87e4, 0x0022_883c));
    c.cycles += 14;
    boot_cfg_tail(c, 5, 9);
    c.g[A1][0] = c.lbu(sp(c)) >> 4;
    call_ptr(c, 0x0028_7520, 0x0022_8884)?; // func_00287520 (musicVol)

    // Cfg_82D0().iUseDJSpeech: copy D, its tail, then cSSXAudio_SetDJSpeech.
    call_ptr(c, 0x0028_b180, 0x0022_888c)?; // cAudioInterface_bxAudio
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(HI_BEOPT.wrapping_add(BEOPT_OFF));
    boot_cfg_copy(c, 5, [8, 9, 6, 7], [8, 9, 6, 7], (0x0022_88a4, 0x0022_88fc));
    c.cycles += 14;
    boot_cfg_tail(c, 5, 8);
    c.g[A1][0] = sx(((c.lw(sp(c)) as u32) as i32 >> 17) as u32);
    c.g[A1][0] = c.g[A1][0] & 0x1;
    call_ptr(c, 0x0028_7558, 0x0022_8948)?; // cSSXAudio_SetDJSpeech

    // Cfg_82D0().iUseArcadeSFX: copy E, its tail, then cSSXAudio_SetArcadeSFX.
    c.g[S2][0] = sx(HI_BEOPT.wrapping_add(BEOPT_OFF));
    call_ptr(c, 0x0028_b180, 0x0022_8950)?; // cAudioInterface_bxAudio
    c.g[A0][0] = c.g[V0][0];
    boot_cfg_copy(
        c,
        18,
        [7, 8, 9, 5],
        [6, 7, 8, 9],
        (0x0022_8964, 0x0022_89bc),
    );
    c.cycles += 15;
    boot_cfg_tail(c, 18, 2);
    c.g[A1][0] = c.lh(sp(c).wrapping_add(0x2));
    c.g[A1][0] = c.g[A1][0] & 0x1;
    call_ptr(c, 0x0028_75d0, 0x0022_8a04)?; // cSSXAudio_SetArcadeSFX

    // WORLDTRIGGERMANAGER_Init, then the command-line track.
    call_ptr(c, 0x002b_4b08, 0x0022_8a0c)?; // WORLDTRIGGERMANAGER_Init
    c.g[A1][0] = HI_STR2 as u64;
    c.g[A0][0] = c.lw(boot_self(c).wrapping_add(SELF_CMDLINE_TRACK));
    c.g[A1][0] = sx(HI_STR2.wrapping_add(0x29e8));
    call_ptr(c, 0x0041_65a8, 0x0022_8a1c)?; // strcmp(fCC, D_004A29E8)
    let same = c.g[V0][0] == 0;
    frame.restore(c, 16, 0x320);
    if !same {
        c.cycles += 9;
        call_ptr(c, 0x0014_dc80, 0x0022_8a2c)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0022_8a38)?; // cBE_getInterface(be, 0)
        c.g[A0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1bc8));
        c.g[A1][0] = c.lw(boot_self(c).wrapping_add(SELF_CMDLINE_TRACK));
        call_ptr(c, 0x0031_84e8, 0x0022_8a44)?; // cBXString_operator=
        frame.restore(c, 16, 0x320);
    }

    // Epilogue.
    c.cycles += 11;
    frame.restore(c, 17, 0x310);
    frame.restore(c, 18, 0x300);
    frame.restore(c, 19, 0x2f0);
    frame.restore(c, 20, 0x2e0);
    frame.restore(c, 21, 0x2d0);
    frame.restore(c, 22, 0x2c0);
    frame.restore(c, 23, 0x2b0);
    frame.restore(c, 30, 0x2a0);
    frame.restore_ra(c, 0x290);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
