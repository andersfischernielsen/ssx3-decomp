//! Staging module oc1156 (wave wave273): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1156`.

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
    (0x003c_90c0, snd_dsp_node_connect, "SND_DspNode_Connect"),
    (0x003c_9938, snd_dec_utk_free_stream, "SND_DecUtk_FreeStream"),
    (0x003c_9960, snd_dec_utk_init_stream, "SND_DecUtk_InitStream"),
    (0x003c_9a00, snd_dec_pcm16_decode_mem, "SND_DecPcm16_DecodeMem"),
    (0x003c_9b10, snd_dec_pcm16_decode_loop, "SND_DecPcm16_DecodeLoop"),
    (0x003c_9c08, snd_dec_pcm16_decode_stream, "SND_DecPcm16_DecodeStream"),
    (0x003c_9d28, snd_dec_pcm16_init_stream, "SND_DecPcm16_InitStream"),
    (0x003c_9f58, snd_dec_eaxa_decode_mem, "SND_DecEaxa_DecodeMem"),
];

/// Address `off` past the register `reg` (the `off(reg)` operand).
#[inline(always)]
fn snd_reg_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// Links a source output to a destination input (the success exit of
/// `SND_DspNode_Connect`): `source->prev` (+`prev`) points at `dest`,
/// `dest->requester` (+0x1a) is set to `requester`, and `dest->next`
/// (+`next`) points back at `source`. Returns 0.
fn snd_dsp_node_link(c: &mut Cpu, source: u64, dest: u64, prev: u32, next: u32, requester: u64) {
    Addr((source as u32).wrapping_add(prev)).sw(c, dest);
    c.g[V0][0] = 0;
    Addr((dest as u32).wrapping_add(0x1a)).sb(c, requester);
    Addr((dest as u32).wrapping_add(next)).sw(c, source);
    ret(c);
}

/// `SND_DspNode_Connect` (0x3c90c0): `SFILTER_connect(source, dest, outSlot,
/// inSlot)`. Links the source's output `outSlot` to the destination's input
/// `inSlot` in the first free pairing (out 1 / in 1, out 1 / in 2, out 2 / in 1,
/// out 2 / in 2). The destination's input links are `pfnnext` (+0x8) and
/// `pfnnext2` (+0xc); the source's output links are `pfnprev1` (+0x10) and
/// `pfnprev2` (+0x14). Returns 0 when linked, -1 when every pairing is taken.
pub fn snd_dsp_node_connect(c: &mut Cpu) {
    let source = c.g[A0][0];
    let dest = c.g[A1][0];
    let out_slot = c.g[A2][0];
    let in_slot = c.g[A3][0];
    let dest32 = dest as u32;
    c.cycles += 3;
    if out_slot == 1 {
        c.cycles += 2;
        if in_slot == 1 {
            c.cycles += 3;
            if Addr(dest32.wrapping_add(0x08)).lw(c) == 0 {
                c.cycles += 5;
                return snd_dsp_node_link(c, source, dest, 0x10, 0x08, in_slot);
            }
        }
        c.cycles += 4;
        if in_slot == 2 {
            c.cycles += 3;
            if Addr(dest32.wrapping_add(0x0c)).lw(c) == 0 {
                c.cycles += 5;
                return snd_dsp_node_link(c, source, dest, 0x10, 0x0c, out_slot);
            }
        }
        c.cycles += 4;
        c.g[V0][0] = u64::MAX;
        return ret(c);
    }
    c.cycles += 2;
    if out_slot == 2 {
        c.cycles += 2;
        if in_slot == 1 {
            c.cycles += 3;
            if Addr(dest32.wrapping_add(0x08)).lw(c) == 0 {
                c.cycles += 5;
                return snd_dsp_node_link(c, source, dest, 0x14, 0x08, out_slot);
            }
        }
        c.cycles += 4;
        if in_slot == 2 {
            c.cycles += 3;
            if Addr(dest32.wrapping_add(0x0c)).lw(c) == 0 {
                c.cycles += 4;
                return snd_dsp_node_link(c, source, dest, 0x14, 0x0c, in_slot);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    ret(c);
}

/// `SND_DecUtk_FreeStream` (0x3c9938): `SFILTER_unpackmtpfrestore`. Frees the
/// MTPF decoder at `node->pDecoder` (+0x34) with `SND_UtkFree` when it is set.
pub fn snd_dec_utk_free_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(snd_reg_at(c, A0, 0x34)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            Callee {
                addr: 0x003c_d1d0,
                f: ee::s14::f_003cd1d0,
            }
            .call(c, 0x003c_9954)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `SND_DecUtk_InitStream` (0x3c9960): `SFILTER_unpackmtpfinit(node, source)`.
/// Fills the MTPF stream node from its source voice: the decode and restore
/// entries (+0x0, +0x4), the packet handle (+0x24) and sample channel (+0x38)
/// of the voice, a fresh 0xd68-byte MTPF decoder (+0x34), and zeroed frame
/// counters. `source->totalframes` (+0x24) and `platformver` (+0xc) are copied.
pub fn snd_dec_utk_init_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x003d_0000;
        frame.save(c, 16, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6910));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let vtable = c.g[V0][0];
        Addr(snd_reg_at(c, S0, 0)).sw(c, vtable);
        c.g[A0][0] = Addr(snd_reg_at(c, S1, 0x1c)).lw(c);
        Callee {
            addr: 0x003c_e0e0,
            f: ee::s14::f_003ce0e0,
        }
        .call(c, 0x003c_998c)?;
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x003c_e378,
            f: ee::s14::f_003ce378,
        }
        .call(c, 0x003c_9994)?;
        let handle = c.g[V0][0];
        Addr(snd_reg_at(c, S0, 0x24)).sw(c, handle);
        c.g[A0][0] = Addr(snd_reg_at(c, S1, 0x1c)).lw(c);
        Callee {
            addr: 0x003c_e118,
            f: ee::s14::f_003ce118,
        }
        .call(c, 0x003c_99a0)?;
        let channel = c.g[V0][0];
        Addr(snd_reg_at(c, S0, 0x38)).sb(c, channel);
        c.g[V1][0] = 0x003d_0000;
        Addr(snd_reg_at(c, S0, 0x1c)).sw(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x66c8));
        Addr(snd_reg_at(c, S0, 0x2c)).sh(c, 0);
        c.g[A0][0] = 0xd68;
        c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x24)).lw(c);
        let total = c.g[V0][0];
        Addr(snd_reg_at(c, S0, 0x30)).sw(c, total);
        c.g[A1][0] = Addr(snd_reg_at(c, S1, 0x0c)).lw(c);
        let restore = c.g[V1][0];
        Addr(snd_reg_at(c, S0, 0x04)).sw(c, restore);
        let platform = c.g[A1][0];
        Addr(snd_reg_at(c, S0, 0x20)).sw(c, platform);
        Callee {
            addr: 0x003c_d1a8,
            f: ee::s14::f_003cd1a8,
        }
        .call(c, 0x003c_99d0)?;
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x003c_ddb8,
            f: ee::s14::f_003cddb8,
        }
        .call(c, 0x003c_99d8)?;
        let decoder = c.g[V0][0];
        Addr(snd_reg_at(c, S0, 0x34)).sw(c, decoder);
        Addr(snd_reg_at(c, S0, 0x28)).sw(c, 0);
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SND_DecPcm16_InitStream` (0x3c9d28): `SFILTER_unpackpfinit(node, source)`.
/// Starts a PCM16 frame-stream reader on the source voice: the block-fetch
/// entry (+0x0), the packet handle (+0x28) and sample channel (+0x31), an
/// unknown total (-1), no sample yet, and the source's 16-bit flag (+0x30).
pub fn snd_dec_pcm16_init_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x003d_0000;
        frame.save(c, 16, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63f8));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let vtable = c.g[V0][0];
        Addr(snd_reg_at(c, S0, 0)).sw(c, vtable);
        c.g[A0][0] = Addr(snd_reg_at(c, S1, 0x1c)).lw(c);
        Callee {
            addr: 0x003c_e0e0,
            f: ee::s14::f_003ce0e0,
        }
        .call(c, 0x003c_9d54)?;
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x003c_e378,
            f: ee::s14::f_003ce378,
        }
        .call(c, 0x003c_9d5c)?;
        let handle = c.g[V0][0];
        Addr(snd_reg_at(c, S0, 0x28)).sw(c, handle);
        c.g[A0][0] = Addr(snd_reg_at(c, S1, 0x1c)).lw(c);
        Callee {
            addr: 0x003c_e118,
            f: ee::s14::f_003ce118,
        }
        .call(c, 0x003c_9d68)?;
        c.g[V1][0] = u64::MAX;
        let channel = c.g[V0][0];
        Addr(snd_reg_at(c, S0, 0x31)).sb(c, channel);
        let unknown = c.g[V1][0];
        Addr(snd_reg_at(c, S0, 0x20)).sw(c, unknown);
        Addr(snd_reg_at(c, S0, 0x1c)).sw(c, 0);
        Addr(snd_reg_at(c, S0, 0x24)).sw(c, 0);
        frame.restore_ra(c, 0);
        c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x18)).lbu(c);
        Addr(snd_reg_at(c, S0, 0x2c)).sw(c, 0);
        let bits16 = c.g[V0][0];
        Addr(snd_reg_at(c, S0, 0x30)).sb(c, bits16);
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `SND_DecPcm16_DecodeMem` (0x3c9a00): `SFILTER_unpackf(node, n, _, buf)`.
/// Decodes `n` frames of a PCM16 sample stream into `buf` (floats) from the
/// current frame (+0x20) up to the total (+0x24). Past the total the rest of
/// `buf` is zeroed. Returns 1, or -1 when the stream is already finished.
pub fn snd_dec_pcm16_decode_mem(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 16, 0x30);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(snd_reg_at(c, A0, 0x20)).lw(c);
        c.g[A2][0] = Addr(snd_reg_at(c, A0, 0x24)).lw(c);
        c.g[V0][0] = (c.g[A1][0] < c.g[A2][0]) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[S2][0] = c.g[A3][0];
        if !in_range {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[V1][0] = (c.g[V0][0] < c.g[A2][0]) as u64;
            let reaches_total = c.g[V1][0] == 0;
            let new_frame = c.g[V0][0];
            Addr(snd_reg_at(c, A0, 0x20)).sw(c, new_frame);
            if !reaches_total {
                c.cycles += 3;
                c.g[V0][0] = Addr(snd_reg_at(c, A0, 0x28)).lw(c);
                let no_bits = c.g[V0][0] == 0;
                c.g[A2][0] = c.g[S2][0];
                if no_bits {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(snd_reg_at(c, A0, 0x1c)).lw(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 1);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    Callee {
                        addr: 0x003c_9e50,
                        f: ee::s14::f_003c9e50,
                    }
                    .call(c, 0x003c_9a64)?;
                    c.g[V0][0] = 1;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(snd_reg_at(c, A0, 0x28)).lw(c);
                let no_bits = c.g[V0][0] == 0;
                c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                if !no_bits {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(snd_reg_at(c, A0, 0x1c)).lw(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 1);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    Callee {
                        addr: 0x003c_9e50,
                        f: ee::s14::f_003c9e50,
                    }
                    .call(c, 0x003c_9a90)?;
                }
                c.cycles += 6;
                c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                c.g[A1][0] = 0;
                call_ptr(c, 0x0041_6210, 0x003c_9aa8)?;
                c.cycles += 1;
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SND_DecPcm16_DecodeLoop` (0x3c9b10): `SFILTER_unpacklf(node, n, _, buf)`.
/// Decodes `n` frames of a looped PCM16 sample (the loop runs from
/// `sustainstart` (+0x24) to `sustainend` (+0x28); the frame is +0x20, the
/// 16-bit flag +0x2c, the samples +0x1c). Each run of frames goes through
/// `SND_Pcm16ToFloat`. Returns 1.
pub fn snd_dec_pcm16_decode_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 16, 0x40);
        c.g[S3][0] = c.g[A3][0];
        let empty = (c.g[S2][0] as i64) <= 0;
        frame.save_ra(c, 0);
        if !empty {
            c.cycles += 2;
            c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x28)).lw(c);
            loop {
                // One run: up to the loop end, then the wrap to sustainstart.
                c.cycles += 7;
                c.g[A1][0] = Addr(snd_reg_at(c, S1, 0x20)).lw(c);
                c.g[A0][0] = Addr(snd_reg_at(c, S1, 0x2c)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[S0][0] as i64)) as u64;
                let no_bits = c.g[A0][0] == 0;
                if c.g[V1][0] != 0 {
                    c.g[S0][0] = c.g[S2][0];
                }
                if !no_bits {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x1c)).lw(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 1);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    Callee {
                        addr: 0x003c_9e50,
                        f: ee::s14::f_003c9e50,
                    }
                    .call(c, 0x003c_9b74)?;
                }
                c.cycles += 9;
                c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x20)).lw(c);
                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[A0][0] = Addr(snd_reg_at(c, S1, 0x28)).lw(c);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                c.g[A0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                let not_wrapped = c.g[A0][0] == 0;
                let advanced = c.g[V0][0];
                Addr(snd_reg_at(c, S1, 0x20)).sw(c, advanced);
                if !not_wrapped {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x24)).lw(c);
                    let start = c.g[V0][0];
                    Addr(snd_reg_at(c, S1, 0x20)).sw(c, start);
                }
                // Back edge.
                c.cycles += 2;
                if (c.g[S2][0] as i64) <= 0 {
                    break;
                }
                c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x28)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003c_9b40);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x40);
        c.g[V0][0] = 1;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SND_DecPcm16_DecodeStream` (0x3c9c08): `SFILTER_unpackpf(node, n, _, buf)`.
/// Decodes `n` frames of a streamed PCM16 sample. The node fetches its next
/// sample block with `SNDPKTPLAYI_get` (the block length goes to its frame
/// total, +0x20), frees the previous frame count with `SNDPKTPLAYI_freeframes`
/// first, and counts the frames it has handed out in +0x2c. Returns that
/// count, or the count after zeroing `buf` once the stream has no sample left.
pub fn snd_dec_pcm16_decode_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 16, 0x50);
        frame.save_ra(c, 0x10);
        c.g[A2][0] = Addr(snd_reg_at(c, S1, 0x2c)).lw(c);
        let handed_out = c.g[A2][0] != 0;
        c.g[S3][0] = c.g[A3][0];
        if handed_out {
            c.cycles += 4;
            c.g[A0][0] = Addr(snd_reg_at(c, S1, 0x28)).lw(c);
            c.g[A1][0] = Addr(snd_reg_at(c, S1, 0x31)).lbu(c);
            call_ptr(c, 0x003c_7010, 0x003c_9c40)?;
            Addr(snd_reg_at(c, S1, 0x2c)).sw(c, 0);
        }
        c.cycles += 2;
        let mut no_sample = false;
        if (c.g[S2][0] as i64) <= 0 {
            c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x2c)).lw(c);
        } else {
            c.cycles += 1;
            loop {
                // Fetch a block when the current one is used up.
                c.cycles += 5;
                c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x24)).lw(c);
                c.g[V1][0] = Addr(snd_reg_at(c, S1, 0x20)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x20)).lw(c);
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = Addr(snd_reg_at(c, S1, 0x28)).lw(c);
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A1][0] = Addr(snd_reg_at(c, S1, 0x31)).lbu(c);
                    c.g[A3][0] = sx(sp(c).wrapping_add(4));
                    call_ptr(c, 0x003c_6de0, 0x003c_9c78)?;
                    let sample = c.g[V0][0];
                    Addr(snd_reg_at(c, S1, 0x1c)).sw(c, sample);
                    if sample == 0 {
                        // The stream has no sample left: zero the rest of buf.
                        c.cycles += 3;
                        c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x2c)).lw(c);
                        let prev = c.g[V0][0] == 0;
                        c.g[A2][0] = sx((c.g[S2][0] as u32) << 2);
                        if !prev {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = 0;
                            call_ptr(c, 0x0041_6210, 0x003c_9c98)?;
                            c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x2c)).lw(c);
                        }
                        no_sample = true;
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                    Addr(snd_reg_at(c, S1, 0x24)).sw(c, 0);
                    let block_len = c.g[V0][0];
                    Addr(snd_reg_at(c, S1, 0x20)).sw(c, block_len);
                    c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x20)).lw(c);
                }
                // Decode the rest of the current block.
                c.cycles += 6;
                c.g[A1][0] = Addr(snd_reg_at(c, S1, 0x24)).lw(c);
                c.g[A0][0] = Addr(snd_reg_at(c, S1, 0x30)).lbu(c);
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[S0][0] as i64)) as u64;
                let no_bits = c.g[A0][0] == 0;
                if c.g[V1][0] != 0 {
                    c.g[S0][0] = c.g[S2][0];
                }
                if !no_bits {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x1c)).lw(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 1);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    Callee {
                        addr: 0x003c_9e50,
                        f: ee::s14::f_003c9e50,
                    }
                    .call(c, 0x003c_9ce0)?;
                }
                c.cycles += 10;
                c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x24)).lw(c);
                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[A0][0] = Addr(snd_reg_at(c, S1, 0x2c)).lw(c);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                let cur = c.g[V0][0];
                Addr(snd_reg_at(c, S1, 0x24)).sw(c, cur);
                let more = (c.g[S2][0] as i64) > 0;
                let handed = c.g[A0][0];
                Addr(snd_reg_at(c, S1, 0x2c)).sw(c, handed);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_9c50);
                    }
                    continue;
                }
                break;
            }
            if !no_sample {
                c.cycles += 1;
                c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x2c)).lw(c);
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `SND_DecEaxa_DecodeMem` (0x3c9f58): `SFILTER_unpackxaf(node, count, _, buffer)`.
/// Decodes up to `count` frames of an EAXA stream with `SND_EaxaDecode` into
/// `buffer` (the cursor lives on the stack), counts them in +0x28, and zeroes
/// the rest of the buffer. Returns the frames delivered, or -1 when the stream
/// is already at its total (+0x24).
pub fn snd_dec_eaxa_decode_mem(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x28)).lw(c);
        c.g[V1][0] = Addr(snd_reg_at(c, S1, 0x24)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        let cursor_init = c.g[A3][0];
        Addr(c.g[SP][0] as u32).sw(c, cursor_init);
        if more {
            c.cycles += 14;
            c.g[A0][0] = Addr(snd_reg_at(c, S1, 0x1c)).lw(c);
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S2][0];
            Callee {
                addr: 0x003c_cf90,
                f: ee::s14::f_003ccf90,
            }
            .call(c, 0x003c_9fa0)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = Addr(c.g[SP][0] as u32).lw(c);
            c.g[V0][0] = Addr(snd_reg_at(c, S1, 0x28)).lw(c);
            c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            let counted = c.g[V0][0];
            Addr(snd_reg_at(c, S1, 0x28)).sw(c, counted);
            let all_read = c.g[A1][0] == 0;
            let zero_from = c.g[A0][0];
            Addr(c.g[SP][0] as u32).sw(c, zero_from);
            if !all_read {
                c.cycles += 5;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                c.g[A1][0] = 0;
                c.g[A2][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                call_ptr(c, 0x0041_6210, 0x003c_9fdc)?;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::{check, fakes, fill, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (in the machine's RAM): the
    /// fakes answer only while it holds `OC1156_MAGIC`.
    pub(crate) const OC1156_CTRL: u32 = DATA + 0x0020_0000;
    pub(crate) const OC1156_MAGIC: u32 = 0x1156_c0de;
    /// `SNDPKTPLAYI_get`'s sample pointer (0: no block) and block length.
    pub(crate) const OC1156_GET_PTR: u32 = OC1156_CTRL + 0x10;
    pub(crate) const OC1156_GET_LEN: u32 = OC1156_CTRL + 0x14;
    /// `SND_EaxaDecode`'s frame count.
    pub(crate) const OC1156_EAXA_N: u32 = OC1156_CTRL + 0x18;
    /// Node, source and destination structs, sample data and the output buffer.
    pub(crate) const OC1156_NODE: u32 = OC1156_CTRL + 0x1000;
    pub(crate) const OC1156_SOURCE: u32 = OC1156_CTRL + 0x1200;
    pub(crate) const OC1156_DEST: u32 = OC1156_CTRL + 0x1400;
    pub(crate) const OC1156_SAMPLES: u32 = OC1156_CTRL + 0x2000;
    pub(crate) const OC1156_BUF: u32 = OC1156_CTRL + 0x4000;

    /// Scrambled machine with the gate open (the fakes answer).
    pub(crate) fn oc1156_gated_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(OC1156_CTRL, OC1156_MAGIC);
        r
    }

    /// A callee fake that logs the call and returns (gate on), or runs the
    /// body it replaced (gate off).
    macro_rules! oc1156_fake {
        ($name:ident, $target:expr, $generated:path) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1156_CTRL) != OC1156_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $generated);
                }
                fakes::fake_call(c, $target);
            }
        };
    }

    oc1156_fake!(
        oc1156_fake_utk_free,
        0x003c_d1d0,
        ee::s14::f_003cd1d0__generated
    );
    oc1156_fake!(
        oc1156_fake_utk_alloc,
        0x003c_d1a8,
        ee::s14::f_003cd1a8__generated
    );
    oc1156_fake!(
        oc1156_fake_utk_reset,
        0x003c_ddb8,
        ee::s14::f_003cddb8__generated
    );
    oc1156_fake!(
        oc1156_fake_master_channel,
        0x003c_e0e0,
        ee::s14::f_003ce0e0__generated
    );
    oc1156_fake!(
        oc1156_fake_find_channel,
        0x003c_e378,
        ee::s14::f_003ce378__generated
    );
    oc1156_fake!(
        oc1156_fake_sub_index,
        0x003c_e118,
        ee::s14::f_003ce118__generated
    );
    oc1156_fake!(
        oc1156_fake_freeframes,
        0x003c_7010,
        ee::s13::f_003c7010__generated
    );

    /// `SNDPKTPLAYI_get`: the block pointer and length come from the
    /// mailboxes; the length is written through the `blockLen` pointer in a2.
    pub(crate) fn oc1156_fake_get(c: &mut Cpu) {
        if c.bus.read32(OC1156_CTRL) != OC1156_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1156_fake_get),
                ee::s13::f_003c6de0__generated,
            );
        }
        let len_at = c.g[A2][0] as u32;
        fakes::fake_call(c, 0x003c_6de0);
        let len = c.bus.read32(OC1156_GET_LEN);
        c.bus.write32(len_at, len);
        c.g[V0][0] = c.bus.read32(OC1156_GET_PTR) as u64;
    }

    /// `SND_EaxaDecode`: returns the count from its mailbox.
    pub(crate) fn oc1156_fake_eaxa(c: &mut Cpu) {
        if c.bus.read32(OC1156_CTRL) != OC1156_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1156_fake_eaxa),
                ee::s14::f_003ccf90__generated,
            );
        }
        fakes::fake_call(c, 0x003c_cf90);
        c.g[V0][0] = c.bus.read32(OC1156_EAXA_N) as u64;
    }

    pub(crate) fn oc1156_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003CD1D0),
            oc1156_fake_utk_free,
            fakes::prev_cell(oc1156_fake_utk_free),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003CD1A8),
            oc1156_fake_utk_alloc,
            fakes::prev_cell(oc1156_fake_utk_alloc),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003CDDB8),
            oc1156_fake_utk_reset,
            fakes::prev_cell(oc1156_fake_utk_reset),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003CE0E0),
            oc1156_fake_master_channel,
            fakes::prev_cell(oc1156_fake_master_channel),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003CE378),
            oc1156_fake_find_channel,
            fakes::prev_cell(oc1156_fake_find_channel),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003CE118),
            oc1156_fake_sub_index,
            fakes::prev_cell(oc1156_fake_sub_index),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C7010),
            oc1156_fake_freeframes,
            fakes::prev_cell(oc1156_fake_freeframes),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C6DE0),
            oc1156_fake_get,
            fakes::prev_cell(oc1156_fake_get),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003CCF90),
            oc1156_fake_eaxa,
            fakes::prev_cell(oc1156_fake_eaxa),
        );
    }

    /// Node and source fields the voice-based init functions read.
    pub(crate) fn oc1156_voice_setup(c: &mut Cpu, r: &mut Rng) {
        c.g[A0][0] = sx(OC1156_NODE);
        c.g[A1][0] = sx(OC1156_SOURCE);
        c.bus.write32(OC1156_SOURCE + 0x1c, r.next_u32());
        c.bus.write32(OC1156_SOURCE + 0x0c, r.next_u32());
        c.bus.write32(OC1156_SOURCE + 0x24, r.next_u32());
        c.bus.write8(OC1156_SOURCE + 0x18, r.next_u32() as u8);
    }

    #[test]
    fn snd_dsp_node_connect_matches() {
        check(
            ee::s14::f_003c90c0__generated,
            snd_dsp_node_connect,
            300,
            &|c, seed| {
                let mut r = Rng(0x1156_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OC1156_SOURCE);
                c.g[A1][0] = sx(OC1156_DEST);
                c.g[A2][0] = [1u64, 2, 0, 3][(seed % 4) as usize];
                c.g[A3][0] = [1u64, 2, 0, 3][((seed / 4) % 4) as usize];
                c.bus.write32(
                    OC1156_DEST + 0x08,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
                c.bus.write32(
                    OC1156_DEST + 0x0c,
                    if seed % 2 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn snd_dec_utk_free_stream_matches() {
        oc1156_install_fakes();
        check(
            ee::s14::f_003c9938__generated,
            snd_dec_utk_free_stream,
            200,
            &|c, seed| {
                let mut r = oc1156_gated_machine(c, seed, 0x1156_0002);
                c.g[A0][0] = sx(OC1156_NODE);
                let decoder = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() | 0x8
                };
                c.bus.write32(OC1156_NODE + 0x34, decoder);
            },
        );
    }

    #[test]
    fn snd_dec_utk_init_stream_matches() {
        oc1156_install_fakes();
        check(
            ee::s14::f_003c9960__generated,
            snd_dec_utk_init_stream,
            100,
            &|c, seed| {
                let mut r = oc1156_gated_machine(c, seed, 0x1156_0003);
                oc1156_voice_setup(c, &mut r);
            },
        );
    }

    #[test]
    fn snd_dec_pcm16_init_stream_matches() {
        oc1156_install_fakes();
        check(
            ee::s14::f_003c9d28__generated,
            snd_dec_pcm16_init_stream,
            100,
            &|c, seed| {
                let mut r = oc1156_gated_machine(c, seed, 0x1156_0004);
                oc1156_voice_setup(c, &mut r);
            },
        );
    }

    /// Frame counters, sample pointer and 16-bit flag of a PCM16 stream node.
    pub(crate) fn oc1156_pcm_node(c: &mut Cpu, pos: u32, total: u32, bits16: u32) {
        c.bus.write32(OC1156_NODE + 0x20, pos);
        c.bus.write32(OC1156_NODE + 0x24, total);
        c.bus.write32(OC1156_NODE + 0x1c, OC1156_SAMPLES);
        c.bus.write32(OC1156_NODE + 0x28, bits16);
    }

    #[test]
    fn snd_dec_pcm16_decode_mem_matches() {
        check(
            ee::s14::f_003c9a00__generated,
            snd_dec_pcm16_decode_mem,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1156_0005, 0);
                fill(c, &mut r, OC1156_SAMPLES, 64);
                let cap = 6 + seed % 7;
                let pos = (seed / 7) % 14;
                let n = 1 + (seed / 3) % 7;
                c.g[A0][0] = sx(OC1156_NODE);
                c.g[A1][0] = n as u64;
                c.g[A3][0] = sx(OC1156_BUF);
                oc1156_pcm_node(c, pos, cap, (seed / 2) % 2);
            },
        );
    }

    /// Setup of the looped PCM16 decoder: a loop of 4-9 frames, the current
    /// frame inside it, a count from -2 to 22 (the empty and wrapped cases).
    fn oc1156_loop_setup(c: &mut Cpu, r: &mut Rng, seed: u32) {
        fill(c, r, OC1156_SAMPLES, 64);
        let start = seed % 5;
        let end = start + 3 + (seed / 5) % 6;
        let cur = start + (seed / 30) % (end - start + 1);
        let n = (seed % 25) as i64 - 2;
        c.g[A0][0] = sx(OC1156_NODE);
        c.g[A1][0] = n as u64;
        c.g[A3][0] = sx(OC1156_BUF);
        c.bus.write32(OC1156_NODE + 0x20, cur);
        c.bus.write32(OC1156_NODE + 0x24, start);
        c.bus.write32(OC1156_NODE + 0x28, end);
        c.bus.write32(OC1156_NODE + 0x1c, OC1156_SAMPLES);
        c.bus.write32(OC1156_NODE + 0x2c, (seed / 7) % 2);
    }

    #[test]
    fn snd_dec_pcm16_decode_loop_matches() {
        check(
            ee::s14::f_003c9b10__generated,
            snd_dec_pcm16_decode_loop,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1156_0006, 0);
                oc1156_loop_setup(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn snd_dec_pcm16_decode_loop_events() {
        check_events(
            ee::s14::f_003c9b10__generated,
            snd_dec_pcm16_decode_loop,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1156_0007, 0);
                let ev = schedule(c, &mut r, seed, 0x003c_9b10);
                oc1156_loop_setup(c, &mut r, seed);
                ev
            },
        );
    }

    /// Setup of the streamed PCM16 decoder: a block that may be used up (the
    /// fetch path), a fetch that may return no block, and a count of -2..10.
    fn oc1156_stream_setup(c: &mut Cpu, r: &mut Rng, seed: u32) {
        oc1156_install_fakes();
        c.bus.write32(OC1156_CTRL, OC1156_MAGIC);
        fill(c, r, OC1156_SAMPLES, 64);
        let total = seed % 6;
        let cur = if total == 0 { 0 } else { (seed / 6) % (total + 1) };
        let prev = if seed % 4 == 0 { 0 } else { 1 + (seed / 4) % 5 };
        let n = (seed % 13) as i64 - 2;
        c.bus.write32(OC1156_GET_PTR, if (seed / 12) % 3 == 0 { 0 } else { OC1156_SAMPLES });
        c.bus.write32(OC1156_GET_LEN, 1 + (seed / 36) % 8);
        c.g[A0][0] = sx(OC1156_NODE);
        c.g[A1][0] = n as u64;
        c.g[A3][0] = sx(OC1156_BUF);
        c.bus.write32(OC1156_NODE + 0x1c, OC1156_SAMPLES);
        c.bus.write32(OC1156_NODE + 0x20, total);
        c.bus.write32(OC1156_NODE + 0x24, cur);
        c.bus.write32(OC1156_NODE + 0x28, r.next_u32());
        c.bus.write32(OC1156_NODE + 0x2c, prev);
        c.bus.write8(OC1156_NODE + 0x30, ((seed / 3) % 2) as u8);
        c.bus.write8(OC1156_NODE + 0x31, (seed % 3) as u8);
    }

    #[test]
    fn snd_dec_pcm16_decode_stream_matches() {
        oc1156_install_fakes();
        check(
            ee::s14::f_003c9c08__generated,
            snd_dec_pcm16_decode_stream,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1156_0008, 0);
                oc1156_stream_setup(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn snd_dec_pcm16_decode_stream_events() {
        oc1156_install_fakes();
        check_events(
            ee::s14::f_003c9c08__generated,
            snd_dec_pcm16_decode_stream,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1156_0009, 0);
                let ev = schedule(c, &mut r, seed, 0x003c_9c08);
                oc1156_stream_setup(c, &mut r, seed);
                ev
            },
        );
    }

    #[test]
    fn snd_dec_eaxa_decode_mem_matches() {
        oc1156_install_fakes();
        check(
            ee::s14::f_003c9f58__generated,
            snd_dec_eaxa_decode_mem,
            300,
            &|c, seed| {
                let mut r = oc1156_gated_machine(c, seed, 0x1156_000a);
                let count = 1 + seed % 6;
                let n = (seed / 6) % (count + 2);
                c.bus.write32(OC1156_EAXA_N, n);
                c.g[A0][0] = sx(OC1156_NODE);
                c.g[A1][0] = count as u64;
                c.g[A3][0] = sx(OC1156_BUF);
                c.bus.write32(OC1156_NODE + 0x1c, r.next_u32() | 0x100);
                c.bus.write32(OC1156_NODE + 0x24, 4);
                c.bus.write32(OC1156_NODE + 0x28, (seed / 12) % 6);
            },
        );
    }
}
