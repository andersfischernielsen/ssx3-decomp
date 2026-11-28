//! `src/dirtysock/dnasauth.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::events::Resume;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0025_6228, dnasauth_init, "DNASAuth_Init"),
    (0x0025_60b0, dnas_auth_run_blocking, "DNASAuth_RunBlocking"),
    (0x0025_61e0, dnas_auth_dtor, "DNASAuth_dtor"),
    (0x0025_6250, dnas_auth_empty_hook, "DNASAuth_EmptyHook"),
    (0x0025_6258, dnas_auth_start, "DNASAuth_Start"),
    (0x0025_62d0, dnas_auth_begin_auth, "DNASAuth_BeginAuth"),
    (0x0025_63a8, dnas_auth_free_buffers, "DNASAuth_FreeBuffers"),
    (0x0025_63f8, dnas_auth_update, "DNASAuth_Update"),
    (
        0x0025_6460,
        dnas_auth_update_load_overlay,
        "DNASAuth_UpdateLoadOverlay",
    ),
    (
        0x0025_6510,
        dnas_auth_update_load_config,
        "DNASAuth_UpdateLoadConfig",
    ),
    (0x0025_65e0, dnas_auth_update_auth, "DNASAuth_UpdateAuth"),
];

/// `void DNASAuth_Init(DNASAuth *self)` (0x256228): clear the auth state and
/// set the handle fields to -1.
pub fn dnasauth_init(c: &mut Cpu) {
    c.cycles += 9;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    this.sw(c, 0);
    this.off(0x18).sw(c, c.g[V0][0]);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `DNASAuth_EmptyHook` (0x0025_6250): straight-line leaf. Mechanical port of the generated body.
pub fn dnas_auth_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void DNASAuth_Update(DNASAuth* self)` (0x002563f8): runs one step of the DNAS
/// authentication state machine, chosen by the state word at +0: 0 loads the
/// overlay, 1 the config, 2 authenticates (the jump table at 0x480410), other
/// states do nothing. Kept as the generated block machine, so every resume
/// entry (after an unwound callee) also runs natively.
pub fn dnas_auth_update(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002563f8 => {
                    c.cycles += 7;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                    c.g[A1][0] = c.g[A0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00256454;
                        continue;
                    }
                    pc = 0x00256414;
                    continue;
                }
                0x00256414 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x410));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;
                    pc = t;
                    continue;
                }
                0x0025642c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00256434);
                    c.g[A0][0] = c.g[A1][0];
                    call_ptr(c, 0x00256460, 0x00256434)?;
                    let t = true;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        pc = 0x00256458;
                        continue;
                    }
                    pc = 0x0025643c;
                    continue;
                }
                0x0025643c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00256444);
                    c.g[A0][0] = c.g[A1][0];
                    call_ptr(c, 0x00256510, 0x00256444)?;
                    let t = true;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        pc = 0x00256458;
                        continue;
                    }
                    pc = 0x0025644c;
                    continue;
                }
                0x0025644c => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00256454);
                    c.g[A0][0] = c.g[A1][0];
                    call_ptr(c, 0x002565e0, 0x00256454)?;
                    pc = 0x00256454;
                    continue;
                }
                0x00256454 => {
                    c.cycles += 1;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    pc = 0x00256458;
                    continue;
                }
                0x00256458 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002563f8, 0x00256460);
                    return Ok(());
                }
            }
        }
    });
}

/// Resume points of `DNASAuth_RunBlocking` (0x2560b0). The function is a
/// `match pc` dispatcher whose loops return on an unwind request, so every
/// loop head (and each block after a call) is an entry point.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum DnasRunStage {
    /// The entry: allocate, construct, start (0x2560b0).
    Create,
    /// 0x2560f4: one cycle, then the first update loop.
    PadA,
    /// 0x2560f8: the first update loop's body (a loop head).
    UpdateA,
    /// 0x256114: loop while the state is below 3.
    CheckA,
    /// 0x256124: `BeginAuth`, then the second loop's test.
    Begin,
    /// 0x256134: one cycle, then the second update loop.
    PadB,
    /// 0x256138: the second update loop's body (a loop head).
    UpdateB,
    /// 0x256154: loop while the state is not 5.
    CheckB,
    /// 0x256160: free the buffers, then the destructor when `this` is set.
    Free,
    /// 0x256170: the destructor (`DNASAuth_dtor(this, 3)`).
    Dtor,
    /// 0x256178: restore s0/ra and return.
    Epilogue,
}

/// The stage result: the next stage, return, or an unwind (the callee left
/// `pc` elsewhere).
pub(crate) type DnasStep = Result<Resume<DnasRunStage>, Unwound>;

/// `DNASAuth_RunBlocking` (0x2560b0): the blocking authentication loop. It
/// creates the DNAS auth object (`operator new(0xa50)`, the ctor, `Start`),
/// then updates it until its state reaches 3 and then 5, frees the buffers and
/// destroys the object when the object is set.
pub(crate) const DNAS_RUN: u32 = 0x0025_60b0;

pub(crate) const DNAS_RUN_END: u32 = 0x0025_6188;

/// The frame of the entry (`sp -= 0x20`).
pub(crate) const DNAS_RUN_FRAME: u32 = 0x20;

/// The loop heads with an event check (the back edges).
pub(crate) const DNAS_UPDATE_A: u32 = 0x0025_60f8;

pub(crate) const DNAS_UPDATE_B: u32 = 0x0025_6138;

pub fn dnas_auth_run_blocking(c: &mut Cpu) {
    let mut stage = match c.pc {
        DNAS_RUN => DnasRunStage::Create,
        0x0025_60f4 => DnasRunStage::PadA,
        DNAS_UPDATE_A => DnasRunStage::UpdateA,
        0x0025_6114 => DnasRunStage::CheckA,
        0x0025_6124 => DnasRunStage::Begin,
        0x0025_6134 => DnasRunStage::PadB,
        DNAS_UPDATE_B => DnasRunStage::UpdateB,
        0x0025_6154 => DnasRunStage::CheckB,
        0x0025_6160 => DnasRunStage::Free,
        0x0025_6170 => DnasRunStage::Dtor,
        0x0025_6178 => DnasRunStage::Epilogue,
        pc => {
            c.jump_out(pc, DNAS_RUN, DNAS_RUN_END);
            return;
        }
    };
    loop {
        let step = match stage {
            DnasRunStage::Create => dnas_run_create(c),
            DnasRunStage::PadA => dnas_run_pad_a(c),
            DnasRunStage::UpdateA => dnas_run_update_a(c),
            DnasRunStage::CheckA => dnas_run_check_a(c),
            DnasRunStage::Begin => dnas_run_begin(c),
            DnasRunStage::PadB => dnas_run_pad_b(c),
            DnasRunStage::UpdateB => dnas_run_update_b(c),
            DnasRunStage::CheckB => dnas_run_check_b(c),
            DnasRunStage::Free => dnas_run_free(c),
            DnasRunStage::Dtor => dnas_run_dtor(c),
            DnasRunStage::Epilogue => dnas_run_epilogue(c),
        };
        match step {
            Ok(Resume::Go(next)) => stage = next,
            Ok(Resume::Done) | Ok(Resume::Unwind) | Err(Unwound) => return,
        }
    }
}

/// Entry: `this = new(0xa50)` (0x317d70), `cFEStateAuthDNASCreate_ctor(this)`,
/// `DNASAuth_Start(this, 0)`, then the state word is read.
pub(crate) fn dnas_run_create(c: &mut Cpu) -> DnasStep {
    c.cycles += 17;
    let frame = Frame::open(c, DNAS_RUN_FRAME);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = 0x004a_2ec8;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[A0][0] = 0xa50;
    call_ptr(c, 0x0031_7d70, 0x0025_60d4)?;
    c.g[A0][0] = c.g[V0][0];
    Callee {
        addr: 0x0025_61b8,
        f: ee::s06::f_002561b8,
    }
    .call(c, 0x0025_60dc)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[S0][0];
    Callee {
        addr: 0x0025_6258,
        f: ee::s06::f_00256258,
    }
    .call(c, 0x0025_60ec)?;
    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
    Ok(Resume::Go(DnasRunStage::CheckA))
}

pub(crate) fn dnas_run_pad_a(c: &mut Cpu) -> DnasStep {
    c.cycles += 1;
    Ok(Resume::Go(DnasRunStage::UpdateA))
}

/// The first update: `DNASAuth_Update(this)`, two calls, then the state.
pub(crate) fn dnas_run_update_a(c: &mut Cpu) -> DnasStep {
    c.cycles += 7;
    c.g[A0][0] = c.g[S0][0];
    Callee {
        addr: 0x0025_63f8,
        f: ee::s06::f_002563f8,
    }
    .call(c, 0x0025_6100)?;
    c.g[A0][0] = 0;
    call_ptr(c, 0x003e_5928, 0x0025_6108)?;
    c.g[A0][0] = 1;
    call_ptr(c, 0x003e_52b0, 0x0025_6110)?;
    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
    Ok(Resume::Go(DnasRunStage::CheckA))
}

/// Loop while `state < 3` (signed); the back edge checks events.
pub(crate) fn dnas_run_check_a(c: &mut Cpu) -> DnasStep {
    c.cycles += 4;
    c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
    c.g[V0][0] ^= 1;
    if c.g[V0][0] == 0 {
        if back_edge_unwinds(c, DNAS_UPDATE_A) {
            return Ok(Resume::Unwind);
        }
        return Ok(Resume::Go(DnasRunStage::UpdateA));
    }
    Ok(Resume::Go(DnasRunStage::Begin))
}

pub(crate) fn dnas_run_begin(c: &mut Cpu) -> DnasStep {
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    Callee {
        addr: 0x0025_62d0,
        f: ee::s06::f_002562d0,
    }
    .call(c, 0x0025_612c)?;
    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
    Ok(Resume::Go(DnasRunStage::CheckB))
}

pub(crate) fn dnas_run_pad_b(c: &mut Cpu) -> DnasStep {
    c.cycles += 1;
    Ok(Resume::Go(DnasRunStage::UpdateB))
}

/// The second update: as the first one, with the returns of its own.
pub(crate) fn dnas_run_update_b(c: &mut Cpu) -> DnasStep {
    c.cycles += 7;
    c.g[A0][0] = c.g[S0][0];
    Callee {
        addr: 0x0025_63f8,
        f: ee::s06::f_002563f8,
    }
    .call(c, 0x0025_6140)?;
    c.g[A0][0] = 0;
    call_ptr(c, 0x003e_5928, 0x0025_6148)?;
    c.g[A0][0] = 1;
    call_ptr(c, 0x003e_52b0, 0x0025_6150)?;
    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
    Ok(Resume::Go(DnasRunStage::CheckB))
}

/// Loop while `state != 5` (the xor with 5 is zero then); the back edge
/// checks events.
pub(crate) fn dnas_run_check_b(c: &mut Cpu) -> DnasStep {
    c.cycles += 3;
    c.g[V0][0] ^= 5;
    if c.g[V0][0] != 0 {
        if back_edge_unwinds(c, DNAS_UPDATE_B) {
            return Ok(Resume::Unwind);
        }
        return Ok(Resume::Go(DnasRunStage::UpdateB));
    }
    Ok(Resume::Go(DnasRunStage::Free))
}

/// `DNASAuth_FreeBuffers(this)`; the destructor runs only when `this` is set
/// (`a0` holds `this` in both cases).
pub(crate) fn dnas_run_free(c: &mut Cpu) -> DnasStep {
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    Callee {
        addr: 0x0025_63a8,
        f: ee::s06::f_002563a8,
    }
    .call(c, 0x0025_6168)?;
    c.g[A0][0] = c.g[S0][0];
    if c.g[S0][0] == 0 {
        return Ok(Resume::Go(DnasRunStage::Epilogue));
    }
    Ok(Resume::Go(DnasRunStage::Dtor))
}

pub(crate) fn dnas_run_dtor(c: &mut Cpu) -> DnasStep {
    c.cycles += 2;
    c.g[A1][0] = 3;
    Callee {
        addr: 0x0025_61e0,
        f: ee::s06::f_002561e0,
    }
    .call(c, 0x0025_6178)?;
    Ok(Resume::Go(DnasRunStage::Epilogue))
}

pub(crate) fn dnas_run_epilogue(c: &mut Cpu) -> DnasStep {
    c.cycles += 4;
    let frame = Frame {
        size: DNAS_RUN_FRAME,
    };
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(Resume::Done)
}

/// The address `off` bytes past the pointer in register `reg` (read after a
/// call: callee-saved registers may only be trusted from the register).
pub(crate) fn oc1545_field(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `void DNASAuth::~DNASAuth(this, flags)` (0x2561e0): runs the hook that
/// does nothing, then frees `this` when bit 0 of `flags` is set.
pub fn dnas_auth_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Callee {
            addr: 0x0025_6250,
            f: ee::s06::f_00256250,
        }
        .call(c, 0x0025_61fc)?;
        c.g[S0][0] &= 0x1;
        let free = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if free {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0025_6210)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void DNASAuth_Start(DNASAuth* self, int arg)` (0x256258): records `arg`,
/// clears the two buffer pointers, allocates the 0x195000-byte block
/// (`operator_new`) and loads the DNAS data file into +0x18.
pub fn dnas_auth_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x0400_0000;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let arg = c.g[A1][0];
        Addr(oc1545_field(c, S0, 0x20)).sw(c, arg);
        c.g[A0][0] = 0x19_0000;
        Addr(oc1545_field(c, S0, 0x14)).sw(c, 0);
        c.g[A1][0] = 0x4a_0000;
        Addr(oc1545_field(c, S0, 0x1c)).sw(c, 0);
        c.g[A0][0] |= 0x5000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2ee0));
        let one = c.g[V0][0];
        Addr(oc1545_field(c, S0, 0)).sw(c, one);
        c.g[A2][0] |= 0x100;
        call_ptr(c, 0x0031_7e30, 0x0025_629c)?;
        c.g[A0][0] = 0x48_0000;
        c.g[A2][0] = 0x19_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3e8));
        c.g[A1][0] = c.g[V0][0];
        let block = c.g[V0][0];
        Addr(oc1545_field(c, S0, 0x4)).sw(c, block);
        c.g[A2][0] |= 0x5000;
        call_ptr(c, 0x003d_f748, 0x0025_62b8)?;
        let file = c.g[V0][0];
        Addr(oc1545_field(c, S0, 0x18)).sw(c, file);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void DNASAuth::BeginAuth(this)` (0x2562d0): clears the 0xa0-byte request
/// block at +0x24, copies the two header words and the MAC address into it,
/// and starts the DNAS release with `DnasRel_CallInit`.
pub fn dnas_auth_begin_auth(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0xa0;
        frame.save(c, S0, 0x30);
        c.g[A1][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx(oc1545_field(c, S0, 0x98));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(oc1545_field(c, S0, 0x24));
        Addr(oc1545_field(c, S0, 0x1c)).sw(c, 0);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x003e_6448, 0x0025_6304)?;
        c.g[V1][0] = Addr(oc1545_field(c, S0, 0xc)).lw(c);
        c.g[A0][0] = 8;
        c.g[V0][0] = Addr(oc1545_field(c, S0, 0x10)).lw(c);
        let v1 = c.g[V1][0];
        Addr(oc1545_field(c, S0, 0x6c)).sw(c, v1);
        let v0 = c.g[V0][0];
        Addr(oc1545_field(c, S0, 0x70)).sw(c, v0);
        Addr(oc1545_field(c, S0, 0x90)).sw(c, 0);
        Addr(oc1545_field(c, S0, 0x8c)).sw(c, 0);
        c.ldl(2, (c.g[GP][0] as u32).wrapping_sub(0x211));
        c.ldr(2, (c.g[GP][0] as u32).wrapping_sub(0x218));
        c.sdl(2, oc1545_field(c, S0, 0x7b));
        c.sdr(2, oc1545_field(c, S0, 0x74));
        let eight = c.g[A0][0];
        Addr(oc1545_field(c, S0, 0x84)).sw(c, eight);
        call_ptr(c, 0x003f_3f78, 0x0025_6338)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0041_66f4, 0x0025_6344)?;
        c.g[V1][0] = 0x48_0000;
        c.g[A2][0] = 1;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3f8));
        c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(7, c.g[A1][0] as u32);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lh(c);
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xa)).lb(c);
        c.sdl(7, oc1545_field(c, S0, 0xaf));
        c.sdr(7, oc1545_field(c, S0, 0xa8));
        let half = c.g[V0][0];
        Addr(oc1545_field(c, S0, 0xb0)).sh(c, half);
        let byte = c.g[A0][0];
        Addr(oc1545_field(c, S0, 0xb2)).sb(c, byte);
        c.g[A1][0] = 4;
        Addr(oc1545_field(c, S0, 0xb8)).sw(c, 0);
        c.g[V0][0] = Addr(oc1545_field(c, S0, 0x20)).lw(c);
        c.g[A0][0] = c.g[S2][0];
        let len = c.g[A1][0];
        Addr(oc1545_field(c, S0, 0)).sw(c, len);
        let v0 = c.g[V0][0];
        Addr(oc1545_field(c, S0, 0xbc)).sw(c, v0);
        let flag = c.g[A2][0];
        Addr(oc1545_field(c, S0, 0xc0)).sw(c, flag);
        call_ptr(c, 0x003f_d0d8, 0x0025_6390)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void DNASAuth::FreeBuffers(this)` (0x2563a8): frees the buffer at +0xc
/// and then the one at +4, clearing each pointer that was freed.
pub fn dnas_auth_free_buffers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(oc1545_field(c, S0, 0xc)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = Addr(oc1545_field(c, S0, 0x4)).lw(c);
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x0025_63cc)?;
            Addr(oc1545_field(c, S0, 0xc)).sw(c, 0);
            c.g[A0][0] = Addr(oc1545_field(c, S0, 0x4)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x0025_63e4)?;
            Addr(oc1545_field(c, S0, 0x4)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void DNASAuth::UpdateLoadOverlay(this)` (0x256460): once the load at +0x18
/// is done, reads its size into +8 and, if there is data, loads the config
/// buffer (+0xc) with `DnasRel_Load`'s result checked and allocates it.
pub fn dnas_auth_update_load_overlay(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(oc1545_field(c, S0, 0x18)).lw(c);
        call_ptr(c, 0x003d_f980, 0x0025_6478)?;
        c.g[V1][0] = 1;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 8;
            c.g[A0][0] = Addr(oc1545_field(c, S0, 0x18)).lw(c);
            c.g[A1][0] = 0;
            Addr(oc1545_field(c, S0, 0x8)).sw(c, 0);
            c.g[A2][0] = sx(oc1545_field(c, S0, 0x8));
            call_ptr(c, 0x003d_f9d8, 0x0025_6498)?;
            c.g[A0][0] = Addr(oc1545_field(c, S0, 0x4)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = Addr(oc1545_field(c, S0, 0x8)).lw(c);
                if (c.g[A1][0] as i64) > 0 {
                    c.cycles += 4;
                    call_ptr(c, 0x003f_cfd8, 0x0025_64b8)?;
                    let failed = (c.g[V0][0] as i64) < 0;
                    c.g[A1][0] = 0x48_0000;
                    if !failed {
                        c.cycles += 15;
                        c.g[A2][0] = 0x400_0000;
                        c.g[A0][0] = 0x1_0000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x428));
                        c.g[A2][0] |= 0x100;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7e30, 0x0025_64d8)?;
                        c.g[A0][0] = 0x48_0000;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x438));
                        let buf = c.g[V0][0];
                        Addr(oc1545_field(c, S0, 0xc)).sw(c, buf);
                        c.g[A2][0] = 0x1_0000;
                        call_ptr(c, 0x003d_f748, 0x0025_64f0)?;
                        c.g[V1][0] = 2;
                        let file = c.g[V0][0];
                        Addr(oc1545_field(c, S0, 0x18)).sw(c, file);
                        let state = c.g[V1][0];
                        Addr(oc1545_field(c, S0, 0)).sw(c, state);
                    }
                    c.cycles += 1;
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void DNASAuth::UpdateLoadConfig(this)` (0x256510): once the config load
/// at +0x18 is done, reads its entry count into +0x10 and XORs that many bytes
/// of the buffer at +0xc with a stack-derived key (a byte loop with event
/// checks at its back edge).
pub fn dnas_auth_update_load_config(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(oc1545_field(c, S0, 0x18)).lw(c);
        call_ptr(c, 0x003d_f980, 0x0025_6528)?;
        c.g[V1][0] = 1;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 8;
            c.g[A0][0] = Addr(oc1545_field(c, S0, 0x18)).lw(c);
            c.g[A1][0] = 0;
            Addr(oc1545_field(c, S0, 0x10)).sw(c, 0);
            c.g[A2][0] = sx(oc1545_field(c, S0, 0x10));
            call_ptr(c, 0x003d_f9d8, 0x0025_6548)?;
            c.g[A0][0] = Addr(oc1545_field(c, S0, 0xc)).lw(c);
            if c.g[A0][0] != 0 {
                'loop_block: {
                    c.cycles += 3;
                    c.g[A3][0] = Addr(oc1545_field(c, S0, 0x10)).lw(c);
                    let empty = (c.g[A3][0] as i64) <= 0;
                    c.g[V0][0] = 3;
                    if empty {
                        break 'loop_block;
                    }
                    c.cycles += 10;
                    c.g[V1][0] = 0x4a_0000;
                    let mode = c.g[V0][0];
                    Addr(oc1545_field(c, S0, 0)).sw(c, mode);
                    c.g[A2][0] = c.g[A0][0];
                    c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2ee8));
                    c.lwl(2, (c.g[T1][0] as u32).wrapping_add(0x3));
                    c.lwr(2, c.g[T1][0] as u32);
                    c.swl(2, sp(c).wrapping_add(0x3));
                    c.swr(2, sp(c));
                    let none = (c.g[A3][0] as i64) <= 0;
                    c.g[A1][0] = 0;
                    if none {
                        break 'loop_block;
                    }
                    c.cycles += 2;
                    c.g[A3][0] = u64::MAX;
                    loop {
                        c.cycles += 16;
                        c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3));
                        if c.g[V1][0] != 0 {
                            c.g[V0][0] = c.g[A1][0];
                        }
                        c.g[A0][0] = Addr(c.g[A2][0] as u32).lbu(c);
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[V1][0] = sx(sp(c).wrapping_add(c.g[V0][0] as u32));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lbu(c);
                        c.g[A0][0] ^= c.g[V0][0];
                        let key = c.g[A0][0];
                        Addr(c.g[A2][0] as u32).sb(c, key);
                        c.g[V0][0] = Addr(oc1545_field(c, S0, 0x10)).lw(c);
                        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        if !more {
                            break 'loop_block;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0025_6590);
                        }
                    }
                }
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void DNASAuth::UpdateAuth(this)` (0x2565e0): runs the release's update
/// step; once it reports done, clears the three strings of the release, calls
/// `DnasRel_CallExit`, copies three strings and sets the state to 5.
pub fn dnas_auth_update_auth(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(oc1545_field(c, S3, 0x14)).lw(c);
        call_ptr(c, 0x003f_d100, 0x0025_6604)?;
        if c.g[V0][0] == 0 {
            c.cycles += 24;
            c.g[S0][0] = sx(oc1545_field(c, S3, 0xc4));
            c.g[A2][0] = 0x38c;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e_6448, 0x0025_6620)?;
            c.g[S1][0] = sx(oc1545_field(c, S3, 0x650));
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = sx(oc1545_field(c, S3, 0x850));
            call_ptr(c, 0x003f_d128, 0x0025_6630)?;
            Addr(oc1545_field(c, S3, 0x450)).sh(c, 0);
            c.g[S0][0] = c.g[V0][0];
            Addr(oc1545_field(c, S3, 0x650)).sh(c, 0);
            c.g[A0][0] = sx(oc1545_field(c, S3, 0x450));
            Addr(oc1545_field(c, S3, 0x850)).sh(c, 0);
            c.g[A1][0] = sx(oc1545_field(c, S3, 0xcc));
            call_ptr(c, 0x003d_be20, 0x0025_664c)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx(oc1545_field(c, S3, 0x10c));
            call_ptr(c, 0x003d_be20, 0x0025_6658)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx(oc1545_field(c, S3, 0x20c));
            call_ptr(c, 0x003d_be20, 0x0025_6664)?;
            if c.g[S0][0] != 0 {
                Addr(oc1545_field(c, S3, 0x1c)).sw(c, 0);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                let ready = c.g[V0][0];
                Addr(oc1545_field(c, S3, 0x1c)).sw(c, ready);
            }
            c.cycles += 3;
            c.g[V0][0] = 5;
            let stage = c.g[V0][0];
            Addr(oc1545_field(c, S3, 0)).sw(c, stage);
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    #[test]
    fn dnasauth_init_matches() {
        check(ssx3_ee::s06::f_00256228, dnasauth_init, 100, &|c, seed| {
            let mut r = Rng(0x2562_2800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for off in [0u32, 4, 8, 0xc, 0x10, 0x14, 0x18] {
                c.bus.write32(DATA + off, r.next_u32());
            }
        });
    }

    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::GAME_GP;
    use std::ptr::addr_of_mut;

    /// `DNASAuth_EmptyHook` (0x0025_6250) against the generated body.
    #[test]
    fn dnas_auth_empty_hook_matches() {
        check(
            ee::s06::f_00256250__generated,
            dnas_auth_empty_hook,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x256250, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// Gate word of this module's callee fakes (own address and magic in the
    /// machine's RAM, so other tests' machines run the real bodies). The
    /// results the fakes return sit after it: word `16 + 4 * index` holds
    /// `1 + value` (0 = leave the fakes' default).
    pub(crate) const OC1527_CTRL: u32 = DATA + 0x2e_0000;

    pub(crate) const OC1527_MAGIC: u32 = 0x1527_f00d;

    /// The logged call; the result comes from the target's mailbox word.
    pub(crate) fn oc1527_fake(c: &mut Cpu, target: u32, idx: u32) {
        fakes::fake_call(c, target);
        let w = c.bus.read32(OC1527_CTRL + 0x10 + 4 * idx);
        if w != 0 {
            c.g[V0][0] = (w - 1) as u64;
        }
    }

    /// Forces the result of the callee with fake index `idx`.
    pub(crate) fn oc1527_result(c: &mut Cpu, idx: u32, value: u32) {
        c.bus.write32(OC1527_CTRL + 0x10 + 4 * idx, value + 1);
    }

    /// Hooked fakes, scrambled registers, the game image, the gate on.
    pub(crate) fn oc1527_machine(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        calls: u32,
    ) -> crate::difftest::Rng {
        oc1527_hook();
        let r = machine(c, seed, salt, calls);
        elf_image(c);
        c.bus.write32(OC1527_CTRL, OC1527_MAGIC);
        r
    }

    pub(crate) fn oc1527_fake_00256460(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1527_fake_00256460),
                ee::s06::f_00256460__generated,
            );
        }
        oc1527_fake(c, 0x00256460, 0);
    }

    pub(crate) fn oc1527_fake_00256510(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1527_fake_00256510),
                ee::s06::f_00256510__generated,
            );
        }
        oc1527_fake(c, 0x00256510, 1);
    }

    pub(crate) fn oc1527_fake_002565e0(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1527_fake_002565e0),
                ee::s06::f_002565e0__generated,
            );
        }
        oc1527_fake(c, 0x002565e0, 2);
    }

    pub(crate) fn oc1527_fake_0040baa8(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1527_fake_0040baa8),
                ee::s15::f_0040baa8__generated,
            );
        }
        oc1527_fake(c, 0x0040baa8, 3);
    }

    pub(crate) fn oc1527_fake_0040b9e8(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1527_fake_0040b9e8),
                ee::s15::f_0040b9e8__generated,
            );
        }
        oc1527_fake(c, 0x0040b9e8, 4);
    }

    pub(crate) fn oc1527_fake_0040bb38(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1527_fake_0040bb38),
                ee::s15::f_0040bb38__generated,
            );
        }
        oc1527_fake(c, 0x0040bb38, 5);
    }

    pub(crate) fn oc1527_fake_0040bc08(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1527_fake_0040bc08),
                ee::s15::f_0040bc08__generated,
            );
        }
        oc1527_fake(c, 0x0040bc08, 6);
    }

    pub(crate) fn oc1527_fake_00402520(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1527_fake_00402520),
                ee::s15::f_00402520__generated,
            );
        }
        oc1527_fake(c, 0x00402520, 7);
    }

    pub(crate) fn oc1527_fake_0040bfe0(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1527_fake_0040bfe0),
                ee::s15::f_0040bfe0__generated,
            );
        }
        oc1527_fake(c, 0x0040bfe0, 8);
    }

    /// Puts every fake in its override slot (once per process).
    pub(crate) fn oc1527_hook() {
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00256460),
            oc1527_fake_00256460,
            fakes::prev_cell(oc1527_fake_00256460),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00256510),
            oc1527_fake_00256510,
            fakes::prev_cell(oc1527_fake_00256510),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002565E0),
            oc1527_fake_002565e0,
            fakes::prev_cell(oc1527_fake_002565e0),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0040BAA8),
            oc1527_fake_0040baa8,
            fakes::prev_cell(oc1527_fake_0040baa8),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0040B9E8),
            oc1527_fake_0040b9e8,
            fakes::prev_cell(oc1527_fake_0040b9e8),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0040BB38),
            oc1527_fake_0040bb38,
            fakes::prev_cell(oc1527_fake_0040bb38),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0040BC08),
            oc1527_fake_0040bc08,
            fakes::prev_cell(oc1527_fake_0040bc08),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402520),
            oc1527_fake_00402520,
            fakes::prev_cell(oc1527_fake_00402520),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0040BFE0),
            oc1527_fake_0040bfe0,
            fakes::prev_cell(oc1527_fake_0040bfe0),
        );
    }

    /// `DNASAuth_Update`: every state (0, 1, 2 through the jump table, 3..5
    /// no-op arms of the table, 6+ and negative skip it), the start and each
    /// resume entry, and an unwind at each call.
    #[test]
    fn dnas_auth_update_matches() {
        const RESUMES: [u32; 6] = [
            0x0025_6414,
            0x0025_642c,
            0x0025_643c,
            0x0025_644c,
            0x0025_6454,
            0x0025_6458,
        ];
        check(
            ee::s06::f_002563f8__generated,
            dnas_auth_update,
            240,
            &|c, seed| {
                let _ = oc1527_machine(c, seed, 0x2563f8, 2);
                let obj = DATA + 0x1000;
                let state = match seed % 9 {
                    8 => 0xffff_fff0,
                    n => n,
                };
                c.bus.write32(obj, state);
                c.g[A0][0] = obj as u64;
                c.pc = 0x0025_63f8;
                if seed % 5 == 4 {
                    c.pc = RESUMES[(seed / 5) as usize % RESUMES.len()];
                    c.g[A1][0] = obj as u64;
                    c.g[V1][0] = sx(state);
                    c.g[V0][0] = 0x0048_0000;
                }
            },
        );
    }

    /// The paths of the state machine: the cycle totals differ per state.
    #[test]
    fn dnas_auth_update_fixture_covers_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..60 {
            let mut c = crate::difftest::fresh();
            let _ = oc1527_machine(&mut c, seed, 0x2563f8, 0);
            let obj = DATA + 0x1000;
            c.bus.write32(obj, seed % 9);
            c.g[A0][0] = obj as u64;
            c.pc = 0x0025_63f8;
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s06::f_002563f8__generated);
            seen.insert(c.cycles - before);
        }
        assert!(
            seen.len() >= 4,
            "fixture reaches too few paths: {}",
            seen.len()
        );
    }

    /// The world the constructor reads at the end.
    pub(crate) const OC1527_WORLD: u32 = DATA + 0x1800;

    /// Seeds the constructor fixture: `this` at DATA+0x1000, every callee's
    /// result chosen by mixed radices of the seed, resume starts for some seeds.
    pub(crate) fn oc1527_ctor_setup(c: &mut Cpu, seed: u32, calls: u32) {
        const RESUMES: [u32; 13] = [
            0x002c_74bc,
            0x002c_74c8,
            0x002c_74cc,
            0x002c_7500,
            0x002c_7508,
            0x002c_755c,
            0x002c_7560,
            0x002c_7578,
            0x002c_7590,
            0x002c_75b4,
            0x002c_75b8,
            0x002c_75c8,
            0x002c_768c,
        ];
        let mut r = oc1527_machine(c, seed, 0x2c7478, calls);
        let this = DATA + 0x1000;
        c.g[A0][0] = this as u64;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC1527_WORLD);
        c.bus.write32(OC1527_WORLD + 0x18, r.next_u32());
        // 0x40baa8 (video flag), 0x40b9e8 (region 0..9), 0x40bb38 (language
        // 0..3), 0x40bc08 (aspect 0..2), 0x402520 (date query), 0x40bfe0.
        oc1527_result(c, 3, if seed % 3 == 0 { 0 } else { 5 });
        oc1527_result(c, 4, (seed / 3) % 10);
        oc1527_result(c, 5, (seed / 5) % 4);
        oc1527_result(c, 6, (seed / 7) % 3);
        oc1527_result(c, 7, (seed / 11) % 2);
        c.pc = 0x002c_7478;
        if seed % 7 == 6 {
            c.pc = RESUMES[(seed / 7) as usize % RESUMES.len()];
            c.g[S5][0] = this as u64;
            c.g[S0][0] = 1;
        }
    }

    /// The constructor fixture reaches many distinct paths.
    #[test]
    fn c_ps2_console_config_ctor_fixture_covers_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..400 {
            if seed % 7 == 6 {
                continue;
            }
            let mut c = crate::difftest::fresh();
            oc1527_ctor_setup(&mut c, seed, 0);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s08::f_002c7478__generated);
            seen.insert(c.cycles - before);
        }
        assert!(
            seen.len() >= 12,
            "fixture reaches too few paths: {}",
            seen.len()
        );
    }

    pub(crate) const MAGIC: u32 = 0x1544_c0de;

    // One gate word per fake, each in its own machine's RAM.
    pub(crate) const G_DELETE: u32 = DATA + 0x21_0000;

    pub(crate) const G_STRCPY: u32 = DATA + 0x21_0040;

    pub(crate) const G_STRCPY_A: u32 = DATA + 0x21_0080;

    pub(crate) const G_LOAD_ELF: u32 = DATA + 0x21_00c0;

    pub(crate) const G_ALLOC: u32 = DATA + 0x21_0100;

    pub(crate) const G_PLAY_START: u32 = DATA + 0x21_0140;

    pub(crate) const G_PLAY_E5928: u32 = DATA + 0x21_0180;

    pub(crate) const G_PLAY_E5398: u32 = DATA + 0x21_01c0;

    pub(crate) const G_PLAY_780: u32 = DATA + 0x21_0200;

    pub(crate) const G_PLAY_860: u32 = DATA + 0x21_0240;

    pub(crate) const G_PLAY_6C8: u32 = DATA + 0x21_0280;

    /// Zero answers left for `0x3ae780` (decremented per call).
    pub(crate) const CTR_PLAY_780: u32 = DATA + 0x21_1000;

    /// Non-zero: `0x3ae6c8` answers zero.
    pub(crate) const ZERO_PLAY_6C8: u32 = DATA + 0x21_1004;

    pub(crate) const G_DNAS_NEW: u32 = DATA + 0x21_2000;

    pub(crate) const G_DNAS_INIT: u32 = DATA + 0x21_2040;

    pub(crate) const G_DNAS_START: u32 = DATA + 0x21_2080;

    pub(crate) const G_DNAS_UPDATE: u32 = DATA + 0x21_20c0;

    pub(crate) const G_DNAS_E5928: u32 = DATA + 0x21_2100;

    pub(crate) const G_DNAS_E52B0: u32 = DATA + 0x21_2140;

    pub(crate) const G_DNAS_BEGIN: u32 = DATA + 0x21_2180;

    pub(crate) const G_DNAS_FREE: u32 = DATA + 0x21_21c0;

    pub(crate) const G_DNAS_DTOR: u32 = DATA + 0x21_2200;

    /// The DNAS auth object (its first word is the state).
    pub(crate) const DNAS_OBJ: u32 = DATA + 0x5000;

    /// Gated fakes: each logs its call when its gate word is `MAGIC` and runs
    /// the body it replaced otherwise.
    macro_rules! gated_fakes {
        ($( ($fake:ident, $gate:expr, $target:expr, $slot:path, $generated:path) ),* $(,)?) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32($gate) != MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $target, ra);
                }
            )*

            /// Hooks the fakes above (once per process).
            fn install_fakes() {
                $(
                    fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
                )*
            }
        };
    }

    gated_fakes! {
        (oc1544_fake_delete, G_DELETE, 0x0031_7e50, ee::s10::OVERRIDE_00317E50, ee::s10::f_00317e50__generated),
        (oc1544_fake_strcpy, G_STRCPY, 0x002c_2508, ee::s08::OVERRIDE_002C2508, ee::s08::f_002c2508__generated),
        (oc1544_fake_strcpy_a, G_STRCPY_A, 0x002c_2540, ee::s08::OVERRIDE_002C2540, ee::s08::f_002c2540__generated),
        (oc1544_fake_load_elf, G_LOAD_ELF, 0x0031_b008, ee::s10::OVERRIDE_0031B008, ee::s10::f_0031b008__generated),
        (oc1544_fake_alloc, G_ALLOC, 0x0025_2f60, ee::s06::OVERRIDE_00252F60, ee::s06::f_00252f60__generated),
        (oc1544_fake_play_start, G_PLAY_START, 0x0025_36c8, ee::s06::OVERRIDE_002536C8, ee::s06::f_002536c8__generated),
        (oc1544_fake_play_e5928, G_PLAY_E5928, 0x003e_5928, ee::s14::OVERRIDE_003E5928, ee::s14::f_003e5928__generated),
        (oc1544_fake_play_e5398, G_PLAY_E5398, 0x003e_5398, ee::s14::OVERRIDE_003E5398, ee::s14::f_003e5398__generated),
        (oc1544_fake_play_860, G_PLAY_860, 0x003a_e860, ee::s13::OVERRIDE_003AE860, ee::s13::f_003ae860__generated),
        (oc1544_fake_dnas_new, G_DNAS_NEW, 0x0031_7d70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated),
        (oc1544_fake_dnas_init, G_DNAS_INIT, 0x0025_6228, ee::s06::OVERRIDE_00256228, ee::s06::f_00256228__generated),
        (oc1544_fake_dnas_start, G_DNAS_START, 0x0025_6258, ee::s06::OVERRIDE_00256258, ee::s06::f_00256258__generated),
        (oc1544_fake_dnas_e5928, G_DNAS_E5928, 0x003e_5928, ee::s14::OVERRIDE_003E5928, ee::s14::f_003e5928__generated),
        (oc1544_fake_dnas_e52b0, G_DNAS_E52B0, 0x003e_52b0, ee::s14::OVERRIDE_003E52B0, ee::s14::f_003e52b0__generated),
        (oc1544_fake_dnas_begin, G_DNAS_BEGIN, 0x0025_62d0, ee::s06::OVERRIDE_002562D0, ee::s06::f_002562d0__generated),
        (oc1544_fake_dnas_free, G_DNAS_FREE, 0x0025_63a8, ee::s06::OVERRIDE_002563A8, ee::s06::f_002563a8__generated),
        (oc1544_fake_dnas_dtor, G_DNAS_DTOR, 0x0025_61e0, ee::s06::OVERRIDE_002561E0, ee::s06::f_002561e0__generated),
    }

    /// `0x3ae780` answers zero for the first `CTR_PLAY_780` calls.
    pub(crate) fn oc1544_fake_play_780(c: &mut Cpu) {
        if c.bus.read32(G_PLAY_780) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1544_fake_play_780),
                ee::s13::f_003ae780__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let left = c.bus.read32(CTR_PLAY_780);
        if left > 0 {
            c.bus.write32(CTR_PLAY_780, left - 1);
        }
        fakes::fake(c, 0x003a_e780, ra);
        if left > 0 {
            c.g[V0][0] = 0;
        }
    }

    /// `0x3ae6c8` answers zero when `ZERO_PLAY_6C8` is set.
    pub(crate) fn oc1544_fake_play_6c8(c: &mut Cpu) {
        if c.bus.read32(G_PLAY_6C8) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1544_fake_play_6c8),
                ee::s13::f_003ae6c8__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let zero = c.bus.read32(ZERO_PLAY_6C8) != 0;
        fakes::fake(c, 0x003a_e6c8, ra);
        if zero {
            c.g[V0][0] = 0;
        }
    }

    /// `DNASAuth_Update(this)`: the state word at `this` advances by one.
    pub(crate) fn oc1544_fake_dnas_update(c: &mut Cpu) {
        if c.bus.read32(G_DNAS_UPDATE) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1544_fake_dnas_update),
                ee::s06::f_002563f8__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let state = c.g[A0][0] as u32;
        fakes::fake(c, 0x0025_63f8, ra);
        let next = c.bus.read32(state).wrapping_add(1);
        c.bus.write32(state, next);
    }

    pub(crate) fn install_custom_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003AE780),
            oc1544_fake_play_780,
            fakes::prev_cell(oc1544_fake_play_780),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003AE6C8),
            oc1544_fake_play_6c8,
            fakes::prev_cell(oc1544_fake_play_6c8),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002563F8),
            oc1544_fake_dnas_update,
            fakes::prev_cell(oc1544_fake_dnas_update),
        );
    }

    pub(crate) fn install_all() {
        install_fakes();
        install_custom_fakes();
    }

    pub(crate) fn open_gates(c: &mut Cpu, gates: &[u32]) {
        for g in gates {
            c.bus.write32(*g, MAGIC);
        }
    }

    driven!(dnas_run_generated, ee::s06::f_002560b0__generated);

    driven!(dnas_run_hand, dnas_auth_run_blocking);

    #[test]
    fn dnas_auth_run_blocking_matches() {
        install_all();
        check_events(dnas_run_generated, dnas_run_hand, 400, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x2560_b0, 0);
            open_gates(
                c,
                &[
                    G_DNAS_NEW,
                    G_DNAS_INIT,
                    G_DNAS_START,
                    G_DNAS_UPDATE,
                    G_DNAS_E5928,
                    G_DNAS_E52B0,
                    G_DNAS_BEGIN,
                    G_DNAS_FREE,
                    G_DNAS_DTOR,
                ],
            );
            // The first logged call is `operator new`: it returns the object.
            c.bus.write32(fakes::LOG_RESULTS, DNAS_OBJ);
            // Initial state 0..=5: the two loops run for the states below 3
            // and below 5 and end at 5.
            c.bus.write32(DNAS_OBJ, seed % 6);
            schedule(c, &mut r, seed, DNAS_RUN)
        });
    }

    /// Gate word of this module's callee fakes (own address and magic in the
    /// machine's RAM, so other tests' machines run the real bodies). Mailbox
    /// `i` is a flag word at `+0x10 + 8i` and a value word at `+0x14 + 8i`:
    /// a set flag makes the fake return the value (sign-extended).
    pub(crate) const OC1545_CTRL: u32 = DATA + 0x2d_0000;

    pub(crate) const OC1545_MAGIC: u32 = 0x1545_f00d;

    pub(crate) const OC1545_OBJ: u32 = DATA + 0x1000;

    pub(crate) const OC1545_BUF: u32 = DATA + 0x1200;

    /// Mailbox indices of the callee fakes.
    pub(crate) const MB_NEW: u32 = 0;

    pub(crate) const MB_IS_DONE: u32 = 5;

    /// `AsyncFile_GetResult`: its value is written to the pointer in `a2`.
    pub(crate) const MB_RESULT: u32 = 6;

    pub(crate) const MB_CALL_UPDATE: u32 = 12;

    pub(crate) const MB_CALL_EXIT: u32 = 13;

    pub(crate) const MB_LOAD_DNAS: u32 = 14;

    /// The logged call; a set mailbox flag makes it return the mailbox value.
    pub(crate) fn oc1545_fake(c: &mut Cpu, target: u32, idx: u32) {
        fakes::fake_call(c, target);
        if c.bus.read32(OC1545_CTRL + 0x10 + 8 * idx) != 0 {
            c.g[V0][0] = sx(c.bus.read32(OC1545_CTRL + 0x14 + 8 * idx));
        }
    }

    /// Forces the result of the callee with mailbox `idx`.
    pub(crate) fn oc1545_result(c: &mut Cpu, idx: u32, value: u32) {
        c.bus.write32(OC1545_CTRL + 0x10 + 8 * idx, 1);
        c.bus.write32(OC1545_CTRL + 0x14 + 8 * idx, value);
    }

    pub(crate) fn oc1545_fake_317e30(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_317e30),
                ee::s10::f_00317e30__generated,
            );
        }
        oc1545_fake(c, 0x0031_7e30, MB_NEW);
    }

    pub(crate) fn oc1545_fake_317e50(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_317e50),
                ee::s10::f_00317e50__generated,
            );
        }
        oc1545_fake(c, 0x0031_7e50, 1);
    }

    pub(crate) fn oc1545_fake_317e98(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_317e98),
                ee::s10::f_00317e98__generated,
            );
        }
        oc1545_fake(c, 0x0031_7e98, 2);
    }

    pub(crate) fn oc1545_fake_318260(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_318260),
                ee::s10::f_00318260__generated,
            );
        }
        oc1545_fake(c, 0x0031_8260, 3);
    }

    pub(crate) fn oc1545_fake_3df748(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_3df748),
                ee::s14::f_003df748__generated,
            );
        }
        oc1545_fake(c, 0x003d_f748, 4);
    }

    pub(crate) fn oc1545_fake_3df980(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_3df980),
                ee::s14::f_003df980__generated,
            );
        }
        oc1545_fake(c, 0x003d_f980, MB_IS_DONE);
    }

    pub(crate) fn oc1545_fake_3df9d8(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_3df9d8),
                ee::s14::f_003df9d8__generated,
            );
        }
        let out = c.g[A2][0] as u32;
        let value = c.bus.read32(OC1545_CTRL + 0x14 + 8 * MB_RESULT);
        oc1545_fake(c, 0x003d_f9d8, MB_RESULT);
        c.bus.write32(out, value);
    }

    pub(crate) fn oc1545_fake_3e6448(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_3e6448),
                ee::s14::f_003e6448__generated,
            );
        }
        oc1545_fake(c, 0x003e_6448, 7);
    }

    pub(crate) fn oc1545_fake_3f3f78(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_3f3f78),
                ee::s14::f_003f3f78__generated,
            );
        }
        oc1545_fake(c, 0x003f_3f78, 8);
    }

    pub(crate) fn oc1545_fake_3dbe20(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_3dbe20),
                ee::s14::f_003dbe20__generated,
            );
        }
        oc1545_fake(c, 0x003d_be20, 9);
    }

    pub(crate) fn oc1545_fake_4166f4(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_4166f4),
                ee::s15::f_004166f4__generated,
            );
        }
        oc1545_fake(c, 0x0041_66f4, 10);
    }

    pub(crate) fn oc1545_fake_3fd0d8(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_3fd0d8),
                ee::s15::f_003fd0d8__generated,
            );
        }
        oc1545_fake(c, 0x003f_d0d8, 11);
    }

    pub(crate) fn oc1545_fake_3fd100(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_3fd100),
                ee::s15::f_003fd100__generated,
            );
        }
        oc1545_fake(c, 0x003f_d100, MB_CALL_UPDATE);
    }

    pub(crate) fn oc1545_fake_3fd128(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_3fd128),
                ee::s15::f_003fd128__generated,
            );
        }
        oc1545_fake(c, 0x003f_d128, MB_CALL_EXIT);
    }

    pub(crate) fn oc1545_fake_3fcfd8(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1545_fake_3fcfd8),
                ee::s15::f_003fcfd8__generated,
            );
        }
        oc1545_fake(c, 0x003f_cfd8, MB_LOAD_DNAS);
    }

    /// Puts every fake in its override slot (once per process).
    pub(crate) fn oc1545_hook() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1545_fake_317e30,
            fakes::prev_cell(oc1545_fake_317e30),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1545_fake_317e50,
            fakes::prev_cell(oc1545_fake_317e50),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1545_fake_317e98,
            fakes::prev_cell(oc1545_fake_317e98),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            oc1545_fake_318260,
            fakes::prev_cell(oc1545_fake_318260),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF748),
            oc1545_fake_3df748,
            fakes::prev_cell(oc1545_fake_3df748),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF980),
            oc1545_fake_3df980,
            fakes::prev_cell(oc1545_fake_3df980),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF9D8),
            oc1545_fake_3df9d8,
            fakes::prev_cell(oc1545_fake_3df9d8),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6448),
            oc1545_fake_3e6448,
            fakes::prev_cell(oc1545_fake_3e6448),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F3F78),
            oc1545_fake_3f3f78,
            fakes::prev_cell(oc1545_fake_3f3f78),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DBE20),
            oc1545_fake_3dbe20,
            fakes::prev_cell(oc1545_fake_3dbe20),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004166F4),
            oc1545_fake_4166f4,
            fakes::prev_cell(oc1545_fake_4166f4),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_003FD0D8),
            oc1545_fake_3fd0d8,
            fakes::prev_cell(oc1545_fake_3fd0d8),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_003FD100),
            oc1545_fake_3fd100,
            fakes::prev_cell(oc1545_fake_3fd100),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_003FD128),
            oc1545_fake_3fd128,
            fakes::prev_cell(oc1545_fake_3fd128),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_003FCFD8),
            oc1545_fake_3fcfd8,
            fakes::prev_cell(oc1545_fake_3fcfd8),
        );
    }

    /// Hooked fakes, scrambled registers, the gate on.
    pub(crate) fn oc1545_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        oc1545_hook();
        let _ = machine(c, seed, salt, calls);
        c.bus.write32(OC1545_CTRL, OC1545_MAGIC);
    }

    /// `DNASAuth::~DNASAuth`: both flag values, so both paths free or not.
    #[test]
    fn dnas_auth_dtor_matches() {
        check(
            ee::s06::f_002561e0__generated,
            dnas_auth_dtor,
            200,
            &|c, seed| {
                oc1545_machine(c, seed, 0x2561e0, 3);
                c.g[A0][0] = OC1545_OBJ as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    /// `DNASAuth_Start`: the operator_new result set or the default.
    #[test]
    fn dnas_auth_start_matches() {
        check(
            ee::s06::f_00256258__generated,
            dnas_auth_start,
            200,
            &|c, seed| {
                oc1545_machine(c, seed, 0x256258, 3);
                if seed % 3 == 0 {
                    oc1545_result(c, MB_NEW, OC1545_OBJ + 0x2000);
                }
                c.g[A0][0] = OC1545_OBJ as u64;
                c.g[A1][0] = seed as u64 * 0x101;
            },
        );
    }

    /// `DNASAuth_BeginAuth`: random request fields, so every copy is exercised.
    #[test]
    fn dnas_auth_begin_auth_matches() {
        check(
            ee::s06::f_002562d0__generated,
            dnas_auth_begin_auth,
            200,
            &|c, seed| {
                oc1545_machine(c, seed, 0x2562d0, 3);
                let mut r = Rng(0x2562_d0 ^ seed as u64);
                fill(c, &mut r, OC1545_OBJ, 0x80);
                c.g[A0][0] = OC1545_OBJ as u64;
            },
        );
    }

    /// `DNASAuth_FreeBuffers`: each of the two pointers is either set or not.
    #[test]
    fn dnas_auth_free_buffers_matches() {
        check(
            ee::s06::f_002563a8__generated,
            dnas_auth_free_buffers,
            200,
            &|c, seed| {
                oc1545_machine(c, seed, 0x2563a8, 3);
                c.bus.write32(
                    OC1545_OBJ + 0xc,
                    if seed & 1 != 0 { DATA + 0x3000 } else { 0 },
                );
                c.bus.write32(
                    OC1545_OBJ + 0x4,
                    if seed & 2 != 0 { DATA + 0x3100 } else { 0 },
                );
                c.g[A0][0] = OC1545_OBJ as u64;
            },
        );
    }

    /// `DNASAuth_UpdateLoadOverlay`: the load done or not, the size zero or
    /// positive, the buffer set or not, `DnasRel_Load` failing or not.
    #[test]
    fn dnas_auth_update_load_overlay_matches() {
        check(
            ee::s06::f_00256460__generated,
            dnas_auth_update_load_overlay,
            240,
            &|c, seed| {
                oc1545_machine(c, seed, 0x256460, 3);
                let mut r = Rng(0x2564_60 ^ seed as u64);
                fill(c, &mut r, OC1545_OBJ, 0x10);
                c.bus.write32(
                    OC1545_OBJ + 0x4,
                    if seed & 1 != 0 { DATA + 0x3000 } else { 0 },
                );
                oc1545_result(c, MB_IS_DONE, if seed % 4 == 0 { 0 } else { 1 });
                oc1545_result(c, MB_RESULT, seed % 5);
                oc1545_result(c, MB_LOAD_DNAS, if seed % 3 == 0 { 0xffff_ffff } else { 1 });
                c.g[A0][0] = OC1545_OBJ as u64;
            },
        );
    }

    /// `DNASAuth_UpdateLoadConfig`: the entry count 0..11 (the byte loop runs
    /// for the non-zero ones), the buffer set or not.
    #[test]
    fn dnas_auth_update_load_config_matches() {
        check(
            ee::s06::f_00256510__generated,
            dnas_auth_update_load_config,
            240,
            &|c, seed| {
                oc1545_machine(c, seed, 0x256510, 3);
                let mut r = Rng(0x2565_10 ^ seed as u64);
                fill(c, &mut r, OC1545_BUF, 0x20);
                c.bus
                    .write32(OC1545_OBJ + 0xc, if seed & 1 != 0 { OC1545_BUF } else { 0 });
                oc1545_result(c, MB_IS_DONE, if seed % 4 == 0 { 0 } else { 1 });
                oc1545_result(c, MB_RESULT, seed % 12);
                c.g[A0][0] = OC1545_OBJ as u64;
            },
        );
    }

    /// `DNASAuth_UpdateAuth`: the update step busy or done, the exit result
    /// zero or not.
    #[test]
    fn dnas_auth_update_auth_matches() {
        check(
            ee::s06::f_002565e0__generated,
            dnas_auth_update_auth,
            200,
            &|c, seed| {
                oc1545_machine(c, seed, 0x2565e0, 3);
                oc1545_result(c, MB_CALL_UPDATE, if seed % 3 == 0 { 1 } else { 0 });
                oc1545_result(c, MB_CALL_EXIT, seed % 2);
                c.g[A0][0] = OC1545_OBJ as u64;
            },
        );
    }
}
