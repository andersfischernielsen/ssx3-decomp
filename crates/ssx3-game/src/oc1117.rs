//! Staging module oc1117 (wave wave269): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1117`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0040_32a8,
        libmpeg_reset_mpeg1_flag,
        "libmpeg_ResetMpeg1Flag",
    ),
    (
        0x0040_32b8,
        libmpeg_output_last_picture,
        "libmpeg_OutputLastPicture",
    ),
    (
        0x0040_3338,
        libmpeg_init_spr_buffers,
        "libmpeg_InitSprBuffers",
    ),
    (0x0040_3398, libmpeg_reset_ipu, "libmpeg_ResetIpu"),
    (
        0x0040_3460,
        libmpeg_default_error_print,
        "libmpeg_DefaultErrorPrint",
    ),
    (0x0040_3470, libmpeg_errorf, "libmpeg_Errorf"),
    (0x0040_34a8, libmpeg_error, "libmpeg_Error"),
];

/// `int libmpeg_FlushPendingPicture(Mpeg *m)` (0x00403240): when the decoder
/// `d = m->dec` (+0x40) has a picture pending (`d[1]` and `d[2]` set), output it
/// with `libmpeg_OutputLastPicture`, set `m[2] = d[70] - d[43]` and clear
/// `d[1]`. Returns 1 when it flushed, else 0.
#[allow(dead_code)]
pub fn libmpeg_flush_pending_picture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        let a = frame.at(c, 0x10);
        c.sd(a, c.g[S1][0]);
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let a = frame.at(c, 0);
        c.sd(a, c.g[S0][0]);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
        let pending = c.g[V0][0] != 0;
        c.g[A0][0] = 0;
        let mut fall_through = true;
        if pending {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
            let has_picture = c.g[V0][0] != 0;
            frame.restore_ra(c, 0x20);
            fall_through = has_picture;
            if has_picture {
                c.cycles += 8;
                Callee {
                    addr: 0x0040_32b8,
                    f: ee::s15::f_004032b8,
                }
                .call(c, 0x0040_3278)?;
                // The state is re-read from the callee-saved registers after the call.
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
                c.g[A0][0] = 1;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xac).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                let frames = c.g[V0][0];
                Addr(c.g[S1][0] as u32).off(0x8).sw(c, frames);
                Addr(c.g[S0][0] as u32).off(0x4).sw(c, 0);
            }
        }
        if fall_through {
            c.cycles += 1;
            frame.restore_ra(c, 0x20);
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[A0][0];
        let a = frame.at(c, 0x10);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_ResetMpeg1Flag(Mpeg *m)` (0x004032a8): clears `m->mpeg1` (+0x848)
/// and tail-calls `libmpeg_SetIpuMpeg1Bit(1)`.
pub fn libmpeg_reset_mpeg1_flag(c: &mut Cpu) {
    c.cycles += 3;
    Addr(c.g[A0][0] as u32).off(0x848).sw(c, 0);
    c.g[A0][0] = 1;
    c.pc = 0x0040_5b48;
    ee::s15::f_00405b48(c);
}

/// `void libmpeg_OutputLastPicture(Mpeg *dec)` (0x004032b8): the pending
/// picture's output. A non-zero `dec[72]` (+0x120) is an error (`libmpeg_Error`
/// with the message at 0x497cc8) and clears it. Otherwise it outputs a frame
/// (`libmpeg_OutputFrame`, when `dec[93]` (+0x174) is 3) or a field pair
/// (`libmpeg_OutputFieldPair`), then clears `dec[72]`.
pub fn libmpeg_output_last_picture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        let a = frame.at(c, 0);
        c.sd(a, c.g[S0][0]);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let pending_error = Addr(c.g[S0][0] as u32).off(0x120).lw(c);
        c.g[V0][0] = pending_error;
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
        if pending_error != 0 {
            c.cycles += 5;
            c.g[A1][0] = sx(0x0049_7cc8);
            Callee {
                addr: 0x0040_34a8,
                f: ee::s15::f_004034a8,
            }
            .call(c, 0x0040_32e0)?;
            Addr(c.g[S0][0] as u32).off(0x120).sw(c, 0);
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x174).lw(c);
            c.g[V0][0] = 3;
            c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
            if c.g[V1][0] == 3 {
                c.cycles += 6;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x1bc).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0040_86c0,
                    f: ee::s15::f_004086c0,
                }
                .call(c, 0x0040_3308)?;
                Addr(c.g[S0][0] as u32).off(0x120).sw(c, 0);
            } else {
                c.cycles += 5;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x1cc).lw(c);
                c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x1dc).lw(c);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0040_87d0,
                    f: ee::s15::f_004087d0,
                }
                .call(c, 0x0040_3320)?;
                Addr(c.g[S0][0] as u32).off(0x120).sw(c, 0);
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0x10);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_InitSprBuffers(Mpeg *m)` (0x00403338): switches the IPU to
/// MPEG-1 (`libmpeg_SetIpuMpeg1Bit(1)`), then points the four scratchpad
/// buffer slots of `m` at 0x70000000 and its +0x1800/+0x1b00/+0x3300 offsets
/// and clears the word at +0x810.
pub fn libmpeg_init_spr_buffers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        let a = frame.at(c, 0);
        c.sd(a, c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        let a = frame.at(c, 0x10);
        c.sd(a, c.g[S1][0]);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = 1;
        Callee {
            addr: 0x0040_5b48,
            f: ee::s15::f_00405b48,
        }
        .call(c, 0x0040_3354)?;
        c.g[S1][0] = 0x7000_0000;
        c.g[V0][0] = 0x7000_1800;
        c.g[V1][0] = 0x7000_1b00;
        c.g[A0][0] = 0x7000_3300;
        let m = Addr(c.g[S0][0] as u32);
        let base = c.g[S1][0];
        m.off(0x590).sw(c, base);
        let second = c.g[V0][0];
        m.off(0x594).sw(c, second);
        let third = c.g[V1][0];
        m.off(0x6d0).sw(c, third);
        let fourth = c.g[A0][0];
        m.off(0x6d4).sw(c, fourth);
        m.off(0x810).sw(c, 0);
        frame.restore_ra(c, 0x20);
        let a = frame.at(c, 0x10);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_ResetIpu(Mpeg *m)` (0x00403398): sets `m[+0x818] = 1`,
/// clears `m[+0x1b0]`, disables interrupts (`DIntr`, its state in `v0`),
/// resets the IPU and DMA registers (0x1000b000.., 0x1000d400..) and the
/// DMA/IPU control words, re-enables them (`EIntr`) when they were enabled,
/// clears the IPU status registers and tail-calls `sceIpuSync(0, 0)` after
/// writing 0x40000000 to the IPU control register.
pub fn libmpeg_reset_ipu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        let v = c.g[V0][0];
        Addr(c.g[A0][0] as u32).off(0x818).sw(c, v);
        Addr(c.g[A0][0] as u32).off(0x1b0).sw(c, 0);
        Callee {
            addr: 0x0042_c078,
            f: ee::s15::f_0042c078,
        }
        .call(c, 0x0040_33b0)?;
        // `v0` is the interrupt state DIntr returned.
        let intr_on = c.g[V0][0] != 0;
        c.g[A2][0] = 0x1000_f520;
        c.g[T0][0] = 0x1_0000;
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A3][0] = 0x1000_f590;
        c.g[V1][0] = c.g[V1][0] | c.g[T0][0];
        let v = c.g[V1][0];
        Addr(c.g[A3][0] as u32).sw(c, v);
        c.g[A0][0] = 0x1000_b000;
        c.g[A1][0] = 0x1000_b400;
        Addr(c.g[A0][0] as u32).sw(c, 0);
        Addr(c.g[A1][0] as u32).sw(c, 0);
        c.g[V1][0] = 0x1000_d400;
        Addr(c.g[V1][0] as u32).sw(c, 0);
        c.g[A0][0] = 0xffff_ffff_fffe_ffff;
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c) & c.g[A0][0];
        let v = c.g[V1][0];
        Addr(c.g[A3][0] as u32).sw(c, v);
        c.g[V0][0] = 0x1000_0000;
        if intr_on {
            c.cycles += 3;
            Callee {
                addr: 0x0042_c0c0,
                f: ee::s15::f_0042c0c0,
            }
            .call(c, 0x0040_3418)?;
            c.g[V0][0] = 0x1000_0000;
        }
        c.cycles += 17;
        c.g[V0][0] = 0x1000_b020;
        c.g[A0][0] = 0x1000_b420;
        Addr(c.g[V0][0] as u32).sw(c, 0);
        c.g[V1][0] = 0x1000_d420;
        Addr(c.g[A0][0] as u32).sw(c, 0);
        Addr(c.g[V1][0] as u32).sw(c, 0);
        c.g[V0][0] = 0x1000_2010;
        c.g[V1][0] = 0x4000_0000;
        let v = c.g[V1][0];
        Addr(c.g[V0][0] as u32).sw(c, v);
        frame.restore_ra(c, 0);
        c.g[A0][0] = 0;
        c.g[A1][0] = 0;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        c.pc = 0x0040_9638;
        ee::s15::f_00409638(c);
        Ok(())
    })
}

/// `void libmpeg_DefaultErrorPrint(const char *msg)` (0x00403460): prints the
/// message through the format at 0x497ce8 (tail call to `scePrintf`).
pub fn libmpeg_default_error_print(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = c.g[A0][0];
    c.g[A0][0] = sx(0x0049_7ce8);
    c.pc = 0x0042_5c60;
    ee::s15::f_00425c60(c);
}

/// `void libmpeg_Errorf(Mpeg *m, const char *fmt, ...)` (0x00403470): formats
/// the message into a 0x120-byte stack buffer (`sprintf`) and reports it with
/// `libmpeg_Error(m, buf)`.
pub fn libmpeg_errorf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x120);
        let a = frame.at(c, 0x100);
        c.sd(a, c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x110);
        c.g[A0][0] = c.g[SP][0];
        Callee {
            addr: 0x0041_7828,
            f: ee::s15::f_00417828,
        }
        .call(c, 0x0040_3488)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        Callee {
            addr: 0x0040_34a8,
            f: ee::s15::f_004034a8,
        }
        .call(c, 0x0040_3494)?;
        frame.restore_ra(c, 0x110);
        let a = frame.at(c, 0x100);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

/// `void libmpeg_Error(Mpeg *m, const char *msg)` (0x004034a8): with a
/// decoder `m` whose callback table is set (`m[+0x858]`, its `+0xc` non-zero),
/// queues the message through `libmpeg_DispatchCallback` (message in a
/// two-word block on the stack, type 0); otherwise prints it with
/// `libmpeg_DefaultErrorPrint`.
pub fn libmpeg_error(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x858).lw(c);
        let mut dispatched = false;
        if c.g[V1][0] != 0 {
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0xc).lw(c);
                let has_callback = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[V1][0];
                if has_callback {
                    c.cycles += 6;
                    let msg_slot = frame.at(c, 4);
                    let msg = c.g[A1][0];
                    Addr(msg_slot).sw(c, msg);
                    let type_slot = frame.at(c, 0);
                    Addr(type_slot).sw(c, 0);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x0040_2c30,
                        f: ee::s15::f_00402c30,
                    }
                    .call(c, 0x0040_34e0)?;
                    frame.restore_ra(c, 0x10);
                    dispatched = true;
                }
            }
        }
        if !dispatched {
            c.cycles += 3;
            c.g[A0][0] = c.g[A1][0];
            Callee {
                addr: 0x0040_3460,
                f: ee::s15::f_00403460,
            }
            .call(c, 0x0040_34f0)?;
            frame.restore_ra(c, 0x10);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (the magic at `OC1117_CTRL` in the machine's RAM).
    pub(crate) const OC1117_CTRL: u32 = DATA + 0x0b_4000;
    pub(crate) const OC1117_MAGIC: u32 = 0x1117_6e5a;
    /// The MPEG state and its decoder in the test machine.
    pub(crate) const OC1117_MPEG: u32 = DATA + 0x0b_0000;
    pub(crate) const OC1117_DEC: u32 = DATA + 0x0b_2000;

    static OC1117_PREV_ERROR: AtomicUsize = AtomicUsize::new(0);
    static OC1117_PREV_OUTPUT_FRAME: AtomicUsize = AtomicUsize::new(0);
    static OC1117_PREV_FIELD_PAIR: AtomicUsize = AtomicUsize::new(0);
    static OC1117_PREV_DISPATCH: AtomicUsize = AtomicUsize::new(0);
    static OC1117_PREV_PRINTF: AtomicUsize = AtomicUsize::new(0);
    static OC1117_PREV_SPRINTF: AtomicUsize = AtomicUsize::new(0);

    /// A gated fake's answer: return at once, with `v0` zero.
    fn oc1117_leaf(c: &mut Cpu) {
        c.g[V0][0] = 0;
        crate::abi::ret(c);
    }

    macro_rules! oc1117_gated {
        ($name:ident, $prev:ident, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1117_CTRL) != OC1117_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                oc1117_leaf(c)
            }
        };
    }
    oc1117_gated!(
        oc1117_fake_error,
        OC1117_PREV_ERROR,
        ee::s15::f_004034a8__generated
    );
    oc1117_gated!(
        oc1117_fake_output_frame,
        OC1117_PREV_OUTPUT_FRAME,
        ee::s15::f_004086c0__generated
    );
    oc1117_gated!(
        oc1117_fake_field_pair,
        OC1117_PREV_FIELD_PAIR,
        ee::s15::f_004087d0__generated
    );
    oc1117_gated!(
        oc1117_fake_dispatch,
        OC1117_PREV_DISPATCH,
        ee::s15::f_00402c30__generated
    );
    oc1117_gated!(
        oc1117_fake_printf,
        OC1117_PREV_PRINTF,
        ee::s15::f_00425c60__generated
    );
    oc1117_gated!(
        oc1117_fake_sprintf,
        OC1117_PREV_SPRINTF,
        ee::s15::f_00417828__generated
    );

    /// Hooks this module's gated fakes into the override slots (once per process).
    fn oc1117_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004034A8),
            oc1117_fake_error,
            &OC1117_PREV_ERROR,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004086C0),
            oc1117_fake_output_frame,
            &OC1117_PREV_OUTPUT_FRAME,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004087D0),
            oc1117_fake_field_pair,
            &OC1117_PREV_FIELD_PAIR,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00402C30),
            oc1117_fake_dispatch,
            &OC1117_PREV_DISPATCH,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00425C60),
            oc1117_fake_printf,
            &OC1117_PREV_PRINTF,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc1117_fake_sprintf,
            &OC1117_PREV_SPRINTF,
        );
    }

    /// `ResetMpeg1Flag`: clears the flag at +0x848 and runs `SetIpuMpeg1Bit`.
    #[test]
    fn libmpeg_reset_mpeg1_flag_matches() {
        check(
            ee::s15::f_004032a8__generated,
            libmpeg_reset_mpeg1_flag,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x4032a8, 0);
                c.bus
                    .write32(OC1117_MPEG + 0x848, seed.wrapping_mul(0x9e37_79b9) | 1);
                c.g[A0][0] = sx(OC1117_MPEG);
                c.pc = 0x0040_32a8;
            },
        );
    }

    /// `OutputLastPicture`: the error exit (`dec[72]` set), the frame output
    /// (`dec[93] == 3`) and the field-pair output.
    #[test]
    fn libmpeg_output_last_picture_matches() {
        oc1117_install_fakes();
        check(
            ee::s15::f_004032b8__generated,
            libmpeg_output_last_picture,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x4032b8, 0);
                c.bus.write32(OC1117_CTRL, OC1117_MAGIC);
                let d = OC1117_DEC;
                c.bus
                    .write32(d + 0x120, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(d + 0x118, r.next_u32());
                c.bus.write32(d + 0xac, r.next_u32());
                c.bus
                    .write32(d + 0x174, if seed % 2 == 0 { 3 } else { r.next_u32() % 5 });
                c.bus.write32(d + 0x1bc, r.next_u32());
                c.bus.write32(d + 0x1cc, r.next_u32());
                c.bus.write32(d + 0x1dc, r.next_u32());
                c.g[A0][0] = sx(d);
                c.pc = 0x0040_32b8;
            },
        );
    }

    /// `InitSprBuffers`: the IPU switch call, then the scratchpad buffer words.
    #[test]
    fn libmpeg_init_spr_buffers_matches() {
        check(
            ee::s15::f_00403338__generated,
            libmpeg_init_spr_buffers,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x403338, 0);
                c.g[A0][0] = sx(OC1117_DEC);
                c.pc = 0x0040_3338;
            },
        );
    }

    /// `ResetIpu`: with interrupts off (`cop0` status bit 16 clear) or on
    /// (`DIntr` returns 1, `EIntr` runs), then the IPU reset tail.
    #[test]
    fn libmpeg_reset_ipu_matches() {
        check(
            ee::s15::f_00403398__generated,
            libmpeg_reset_ipu,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x403398, 0);
                c.cop0[12] = if seed % 2 == 0 { 0x1_0000 } else { 0 };
                c.g[A0][0] = sx(OC1117_DEC);
                c.pc = 0x0040_3398;
            },
        );
    }

    /// `DefaultErrorPrint`: a tail call to `scePrintf` with the format and the message.
    #[test]
    fn libmpeg_default_error_print_matches() {
        oc1117_install_fakes();
        check(
            ee::s15::f_00403460__generated,
            libmpeg_default_error_print,
            50,
            &|c, seed| {
                let _ = machine(c, seed, 0x403460, 0);
                c.bus.write32(OC1117_CTRL, OC1117_MAGIC);
                c.pc = 0x0040_3460;
            },
        );
    }

    /// `Errorf`: the `sprintf` into the stack buffer, then `libmpeg_Error`
    /// with the decoder and the buffer.
    #[test]
    fn libmpeg_errorf_matches() {
        oc1117_install_fakes();
        check(
            ee::s15::f_00403470__generated,
            libmpeg_errorf,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x403470, 0);
                c.bus.write32(OC1117_CTRL, OC1117_MAGIC);
                c.g[A0][0] = sx(OC1117_DEC);
                c.g[A1][0] = sx(OC1117_MPEG + 0x100);
                c.pc = 0x0040_3470;
            },
        );
    }

    /// `Error`: no decoder, no callback table, a callback (`DispatchCallback`)
    /// and the default print.
    #[test]
    fn libmpeg_error_matches() {
        oc1117_install_fakes();
        check(
            ee::s15::f_004034a8__generated,
            libmpeg_error,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x4034a8, 0);
                c.bus.write32(OC1117_CTRL, OC1117_MAGIC);
                let st = OC1117_MPEG;
                c.g[A0][0] = if seed % 9 == 0 { 0 } else { sx(st) };
                c.bus
                    .write32(st + 0x858, if seed % 4 == 0 { 0 } else { OC1117_DEC });
                c.bus.write32(st + 0xc, if seed % 5 == 0 { 0 } else { 1 });
                c.bus
                    .write32(OC1117_DEC + 0xc, if seed % 3 == 0 { 0 } else { 1 });
                c.g[A1][0] = sx(OC1117_MPEG + 0x200);
                c.pc = 0x0040_34a8;
            },
        );
    }
}
