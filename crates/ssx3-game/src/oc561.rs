//! Staging module oc561 (wave wave232): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc561`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_0d08, input_map_free, "InputMap_Free"),
    (0x0032_0d50, input_map_parse_file, "InputMap_ParseFile"),
    (
        0x0032_1010,
        input_map_eval_float_args,
        "InputMap_EvalFloatArgs",
    ),
    (
        0x0032_1188,
        input_map_eval_bool_args,
        "InputMap_EvalBoolArgs",
    ),
    (
        0x0032_1590,
        input_parser_skip_statement,
        "InputParser_SkipStatement",
    ),
    (
        0x0032_6a20,
        input_preproc_empty_stub,
        "InputPreproc_EmptyStub",
    ),
    (
        0x0032_6ae0,
        pad_queue_stop_all_actuators,
        "PadQueue_StopAllActuators",
    ),
    (
        0x0032_9938,
        rider_sphere_tree_ctor_copy,
        "RiderSphereTree_ctor_copy",
    ),
    (0x0032_b620, rle_decode, "RLE_Decode"),
];

/// `EA_MemFree(void*)` (0x317e98).
pub(crate) const IM_EA_MEM_FREE: Callee = Callee {
    addr: 0x0031_7e98,
    f: ee::s10::f_00317e98,
};

/// `InputMap_CreateParser(map, buffer)` (0x320db0).
pub(crate) const IM_CREATE_PARSER: Callee = Callee {
    addr: 0x0032_0db0,
    f: ee::s10::f_00320db0,
};

/// `InputParser_NextToken(parser)` (0x321af0).
pub(crate) const IM_NEXT_TOKEN: Callee = Callee {
    addr: 0x0032_1af0,
    f: ee::s10::f_00321af0,
};

/// `InputVM_Init(vm, pad)` (0x325250).
pub(crate) const IM_VM_INIT: Callee = Callee {
    addr: 0x0032_5250,
    f: ee::s10::f_00325250,
};

/// `InputVM_Execute(vm, pad, ...)` (0x325450).
pub(crate) const IM_VM_EXECUTE: Callee = Callee {
    addr: 0x0032_5450,
    f: ee::s10::f_00325450,
};

/// `PadDevice_StopActuators(device)` (0x326cf0).
pub(crate) const PADDEV_STOP_ACTUATORS: Callee = Callee {
    addr: 0x0032_6cf0,
    f: ee::s10::f_00326cf0,
};

/// `RiderSphereTree_Assign(tree)` (0x329a28).
pub(crate) const RIDER_SPHERE_TREE_ASSIGN: Callee = Callee {
    addr: 0x0032_9a28,
    f: ee::s10::f_00329a28,
};

/// `void InputMap_Free(InputMap* map)` (0x320d08): frees the two buffers
/// the map owns (`+4` and `+0x14`), each only when set.
pub fn input_map_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let map = c.g[S0][0] as u32;
        c.g[A0][0] = Addr(map.wrapping_add(4)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            IM_EA_MEM_FREE.call(c, 0x0032_0d2c)?;
        }
        c.g[A0][0] = Addr(map.wrapping_add(0x14)).lw(c);
        c.cycles += 2;
        frame.restore(c, S0, 0x10);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            IM_EA_MEM_FREE.call(c, 0x0032_0d40)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void InputMap_ParseFile(InputMap* map, const char* path)` (0x320d50):
/// reads the file size (into the stack word at sp), then creates the
/// parser from the loaded buffer; a null buffer is not freed.
pub fn input_map_parse_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0x100;
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        // EAFile_LoadFileGetSizeEx(path, &size, 0x100), through the table.
        call_ptr(c, 0x003e_1a10, 0x0032_0d74)?;
        c.g[S1][0] = c.g[V0][0];
        let size_addr = sp(c);
        c.g[A2][0] = Addr(size_addr).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        IM_CREATE_PARSER.call(c, 0x0032_0d88)?;
        let no_buffer = c.g[S1][0] == 0;
        frame.restore(c, S0, 0x30);
        if !no_buffer {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            IM_EA_MEM_FREE.call(c, 0x0032_0d98)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// Shared body of `cInputMap_getValue1` and `cInputMap_getState1`
/// (0x321010 / 0x321188): spills the eight float arguments (f12..f19) to a
/// stack array, initialises the VM, looks up the state's operand table and
/// runs the VM. `prologue` is the cycle charge of the first block, the two
/// return addresses are the VM calls' return points.
fn oc561_eval_args(c: &mut Cpu, prologue: u64, init_ret: u32, exec_ret: u32) -> Flow {
    c.cycles += prologue;
    let frame = Frame::open(c, 0x220);
    frame.save(c, S3, 0x1a0);
    frame.save(c, S0, 0x1d0);
    c.g[S3][0] = c.g[A0][0];
    frame.save(c, S1, 0x1c0);
    c.g[S0][0] = c.g[A2][0];
    frame.save(c, S2, 0x1b0);
    c.g[A0][0] = c.g[SP][0];
    frame.save_fpr(c, 27, 0x218);
    c.g[S2][0] = c.g[A3][0];
    frame.save_fpr(c, 26, 0x210);
    c.mov_s(27, 19);
    frame.save_fpr(c, 25, 0x208);
    c.mov_s(26, 18);
    frame.save_fpr(c, 24, 0x200);
    c.mov_s(25, 17);
    frame.save_fpr(c, 23, 0x1f8);
    c.mov_s(24, 16);
    frame.save_fpr(c, 22, 0x1f0);
    c.mov_s(23, 15);
    frame.save_fpr(c, 21, 0x1e8);
    c.mov_s(22, 14);
    frame.save_fpr(c, 20, 0x1e0);
    c.mov_s(21, 13);
    c.mov_s(20, 12);
    frame.save_ra(c, 0x190);
    c.g[S1][0] = c.g[A1][0];
    IM_VM_INIT.call(c, init_ret)?;
    let table = (c.g[S3][0] as u32).wrapping_add(4);
    c.g[V1][0] = Addr(table).lw(c);
    c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
    let stack_word = sp(c);
    let modifier = c.g[S2][0];
    Addr(stack_word).sw(c, modifier);
    c.g[A1][0] = c.g[S1][0];
    frame.save_fpr(c, 20, 0x10c);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    frame.save_fpr(c, 21, 0x110);
    c.g[A0][0] = c.g[SP][0];
    frame.save_fpr(c, 22, 0x114);
    frame.save_fpr(c, 23, 0x118);
    frame.save_fpr(c, 24, 0x11c);
    frame.save_fpr(c, 25, 0x120);
    frame.save_fpr(c, 26, 0x124);
    frame.save_fpr(c, 27, 0x128);
    c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x14)).lw(c);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    IM_VM_EXECUTE.call(c, exec_ret)?;
    Ok(())
}

/// Restores what [`oc561_eval_args`] saved and returns.
fn oc561_eval_restore(c: &mut Cpu) {
    let frame = Frame { size: 0x220 };
    frame.restore(c, S0, 0x1d0);
    frame.restore(c, S1, 0x1c0);
    frame.restore(c, S2, 0x1b0);
    frame.restore(c, S3, 0x1a0);
    frame.restore_ra(c, 0x190);
    frame.restore_fpr(c, 27, 0x218);
    frame.restore_fpr(c, 26, 0x210);
    frame.restore_fpr(c, 25, 0x208);
    frame.restore_fpr(c, 24, 0x200);
    frame.restore_fpr(c, 23, 0x1f8);
    frame.restore_fpr(c, 22, 0x1f0);
    frame.restore_fpr(c, 21, 0x1e8);
    frame.restore_fpr(c, 20, 0x1e0);
    frame.close(c);
}

/// `float cInputMap_getValue1(InputMap* map, InputPad* pad, int state,
/// int modifier)` (0x321010): runs the state's VM and returns its float
/// result (left in f0 by the VM).
pub fn input_map_eval_float_args(c: &mut Cpu) {
    guest(c, |c| {
        oc561_eval_args(c, 62, 0x0032_1080, 0x0032_10cc)?;
        oc561_eval_restore(c);
        Ok(())
    });
}

/// `bool cInputMap_getState1(InputMap* map, InputPad* pad, int state,
/// int modifier)` (0x321188): as the value variant, returning whether the
/// VM's float result is non-zero.
pub fn input_map_eval_bool_args(c: &mut Cpu) {
    guest(c, |c| {
        oc561_eval_args(c, 52, 0x0032_11f8, 0x0032_1244)?;
        c.mtc1(0, 1);
        c.c_eq_s(0, 1);
        let nonzero = !c.fcc();
        c.g[V0][0] = 0x1;
        if !nonzero {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 15;
        oc561_eval_restore(c);
        Ok(())
    });
}

/// `void cInputMapParser_skipPastSemicolon(InputMapParser* self)` (0x321590):
/// reads tokens until the token type at `+0x98` is 5 (the end marker) or 0.
pub fn input_parser_skip_statement(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 5;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let parser = c.g[S0][0] as u32;
        c.g[V1][0] = Addr(parser.wrapping_add(0x98)).lw(c);
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 2;
            let empty = c.g[V1][0] == 0;
            c.g[S1][0] = 5;
            if !empty {
                c.cycles += 1;
                loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    IM_NEXT_TOKEN.call(c, 0x0032_15c8)?;
                    let token_at = (c.g[S0][0] as u32).wrapping_add(0x98);
                    c.g[V0][0] = Addr(token_at).lw(c);
                    if c.g[V0][0] == c.g[S1][0] {
                        break;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_15c0);
                    }
                }
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        IM_NEXT_TOKEN.call(c, 0x0032_15e4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void InputPreproc_EmptyStub(void)` (0x326a20): does nothing.
pub fn input_preproc_empty_stub(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void PadQueue_StopAllActuators(PadQueue* q)` (0x326ae0): stops every
/// pad device in the queue (`count` at `+0x2ee8`, the device pointers from
/// `+0x2eec`), checking events at each back edge.
pub fn pad_queue_stop_all_actuators(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let queue = c.g[S2][0] as u32;
        c.g[V0][0] = Addr(queue.wrapping_add(0x2ee8)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[S0][0] = sx(queue.wrapping_add(0x2eec));
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            loop {
                c.cycles += 7;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                PADDEV_STOP_ACTUATORS.call(c, 0x0032_6b1c)?;
                let count_at = (c.g[S2][0] as u32).wrapping_add(0x2ee8);
                c.g[V0][0] = Addr(count_at).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0032_6b10);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `RiderSphereTree* RiderSphereTree_ctor_copy(RiderSphereTree* t)`
/// (0x329938): sets the tree up by `RiderSphereTree_Assign` and returns it.
pub fn rider_sphere_tree_ctor_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x13;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = u64::MAX;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[S0][0];
        RIDER_SPHERE_TREE_ASSIGN.call(c, 0x0032_9958)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `int RLE_Decode(u8* dst, const s8* src)` (0x32b620): decodes the
/// signed-header run-length stream at `src` into `dst`. A header `h >= 0`
/// repeats the next byte `h + 1` times; `h < 0` copies the next `-h`
/// bytes; a zero header ends the stream. Returns the bytes written.
pub fn rle_decode(c: &mut Cpu) {
    c.cycles += 2;
    let has_src = c.g[A1][0] != 0;
    c.g[A2][0] = c.g[A0][0];
    if !has_src {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    c.cycles += 2;
    c.g[A3][0] = Addr(c.g[A1][0] as u32).lb(c);
    loop {
        c.cycles += 2;
        let repeat = (c.g[A3][0] as i64) >= 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        if repeat {
            c.cycles += 2;
            let end = c.g[A3][0] == 0;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            if end {
                break;
            }
            c.cycles += 1;
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lbu(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                let value = c.g[V0][0];
                Addr(c.g[A2][0] as u32).sb(c, value);
                let more = c.g[V1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_b678);
                    }
                    continue;
                }
                c.cycles += 1;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                break;
            }
        } else {
            c.cycles += 2;
            c.g[V1][0] = sx(0u32.wrapping_sub(c.g[A3][0] as u32));
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lbu(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                let value = c.g[V0][0];
                Addr(c.g[A2][0] as u32).sb(c, value);
                let more = c.g[V1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_b648);
                    }
                    continue;
                }
                c.cycles += 2;
                break;
            }
        }
        c.cycles += 2;
        if c.g[A3][0] != 0 {
            c.g[A3][0] = Addr(c.g[A1][0] as u32).lb(c);
            if event_due(c) {
                deliver_event_here(c, 0x0032_b638);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// The fakes below only log while this word is set at `OC561_GATE`.
    const OC561_GATE: u32 = DATA + 0x20_0000;
    const OC561_MAGIC: u32 = 0x0561_0561;
    /// Result cells the setups write: file-size call's v0 and size, the
    /// VM exec call's f0, and the token script read by the next-token fake.
    const OC561_FILE_V0: u32 = DATA + 0x20_0010;
    const OC561_FILE_SIZE: u32 = DATA + 0x20_0014;
    const OC561_EXEC_F0: u32 = DATA + 0x20_0018;
    const OC561_TOK_IDX: u32 = DATA + 0x20_0020;
    const OC561_TOKS: u32 = DATA + 0x20_0040;

    fn oc561_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC561_GATE) == OC561_MAGIC
    }

    /// Scrambled registers, the gate on, and no fake unwinds.
    fn oc561_setup(c: &mut Cpu, r: &mut Rng) {
        scramble(c, r);
        c.bus.write32(OC561_GATE, OC561_MAGIC);
        c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
    }

    fn oc561_fake_mem_free(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_mem_free),
                ee::s10::f_00317e98__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7e98, ra);
    }

    fn oc561_fake_create_parser(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_create_parser),
                ee::s10::f_00320db0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_0db0, ra);
    }

    fn oc561_fake_file_size(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_file_size),
                ee::s14::f_003e1a10__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let size_out = c.g[A1][0] as u32;
        let size = c.bus.read32(OC561_FILE_SIZE);
        c.bus.write32(size_out, size);
        fakes::fake(c, 0x003e_1a10, ra);
        c.g[V0][0] = c.bus.read32(OC561_FILE_V0) as u64;
    }

    fn oc561_fake_next_token(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_next_token),
                ee::s10::f_00321af0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let parser = c.g[A0][0] as u32;
        let k = c.bus.read32(OC561_TOK_IDX);
        let token = c.bus.read32(OC561_TOKS + 4 * k.min(7));
        c.bus.write32(OC561_TOK_IDX, k + 1);
        fakes::fake(c, 0x0032_1af0, ra);
        c.bus.write32(parser.wrapping_add(0x98), token);
    }

    fn oc561_fake_vm_init(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_vm_init),
                ee::s10::f_00325250__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_5250, ra);
    }

    fn oc561_fake_vm_execute(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_vm_execute),
                ee::s10::f_00325450__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_5450, ra);
        c.f[0] = c.bus.read32(OC561_EXEC_F0);
    }

    fn oc561_fake_pad_stop(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_pad_stop),
                ee::s10::f_00326cf0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_6cf0, ra);
    }

    fn oc561_fake_assign(c: &mut Cpu) {
        if !oc561_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc561_fake_assign),
                ee::s10::f_00329a28__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0032_9a28, ra);
    }

    /// Puts the gated fakes into their override slots (once per process).
    fn oc561_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc561_fake_mem_free,
            fakes::prev_cell(oc561_fake_mem_free),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320DB0),
            oc561_fake_create_parser,
            fakes::prev_cell(oc561_fake_create_parser),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E1A10),
            oc561_fake_file_size,
            fakes::prev_cell(oc561_fake_file_size),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00321AF0),
            oc561_fake_next_token,
            fakes::prev_cell(oc561_fake_next_token),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00325250),
            oc561_fake_vm_init,
            fakes::prev_cell(oc561_fake_vm_init),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00325450),
            oc561_fake_vm_execute,
            fakes::prev_cell(oc561_fake_vm_execute),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00326CF0),
            oc561_fake_pad_stop,
            fakes::prev_cell(oc561_fake_pad_stop),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00329A28),
            oc561_fake_assign,
            fakes::prev_cell(oc561_fake_assign),
        );
    }

    /// The map at DATA, its operand table at DATA+0x500, its base at
    /// DATA+0x600, and the float arguments f12..f19 (shared by both
    /// `InputMap_Eval*Args` tests).
    fn oc561_eval_setup(c: &mut Cpu, r: &mut Rng, seed: u32) {
        oc561_setup(c, r);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x1000);
        c.g[A2][0] = (seed % 4) as u64;
        c.g[A3][0] = (seed % 5) as u64;
        c.bus.write32(DATA + 4, DATA + 0x500);
        c.bus.write32(DATA + 0x14, DATA + 0x600);
        for k in 0..4u32 {
            c.bus.write32(DATA + 0x500 + 4 * k, r.next_u32() % 8);
        }
        for i in 0..8usize {
            c.f[12 + i] = r.f32(100.0);
        }
        c.bus
            .write32(OC561_EXEC_F0, if seed % 2 == 0 { 0 } else { 0x3f80_0000 });
    }

    #[test]
    fn input_map_free_matches() {
        oc561_install_fakes();
        check(ee::s10::f_00320d08__generated, input_map_free, 96, &|c, seed| {
            let mut r = Rng(0x0032_0d08 ^ seed as u64);
            oc561_setup(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus
                .write32(DATA + 4, if seed % 3 == 0 { 0 } else { DATA + 0x400 });
            c.bus
                .write32(DATA + 0x14, if seed % 2 == 0 { 0 } else { DATA + 0x800 });
        });
    }

    #[test]
    fn input_map_parse_file_matches() {
        oc561_install_fakes();
        check(ee::s10::f_00320d50__generated, input_map_parse_file, 96, &|c, seed| {
            let mut r = Rng(0x0032_0d50 ^ seed as u64);
            oc561_setup(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            c.bus.write32(OC561_FILE_SIZE, DATA + 0x2000 + 16 * seed);
            c.bus
                .write32(OC561_FILE_V0, if seed % 3 == 0 { 0 } else { DATA + 0x3000 });
        });
    }

    #[test]
    fn input_map_eval_float_args_matches() {
        oc561_install_fakes();
        check(
            ee::s10::f_00321010__generated,
            input_map_eval_float_args,
            96,
            &|c, seed| {
                let mut r = Rng(0x0032_1010 ^ seed as u64);
                oc561_eval_setup(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn input_map_eval_bool_args_matches() {
        oc561_install_fakes();
        check(
            ee::s10::f_00321188__generated,
            input_map_eval_bool_args,
            96,
            &|c, seed| {
                let mut r = Rng(0x0032_1188 ^ seed as u64);
                oc561_eval_setup(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn input_parser_skip_statement_matches() {
        oc561_install_fakes();
        check(
            ee::s10::f_00321590__generated,
            input_parser_skip_statement,
            96,
            &|c, seed| {
                let mut r = Rng(0x0032_1590 ^ seed as u64);
                oc561_setup(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let first = [5u32, 0, 3, 7][(seed % 4) as usize];
                c.bus.write32(DATA + 0x98, first);
                c.bus.write32(OC561_TOK_IDX, 0);
                for i in 0..7u32 {
                    c.bus.write32(OC561_TOKS + 4 * i, r.next_u32() % 4);
                }
                // The script always ends with the end marker (5).
                c.bus.write32(OC561_TOKS + 4 * 7, 5);
            },
        );
    }

    #[test]
    fn input_preproc_empty_stub_matches() {
        check(
            ee::s10::f_00326a20__generated,
            input_preproc_empty_stub,
            8,
            &|c, seed| {
                let mut r = Rng(0x0032_6a20 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn pad_queue_stop_all_actuators_matches() {
        oc561_install_fakes();
        check(
            ee::s10::f_00326ae0__generated,
            pad_queue_stop_all_actuators,
            96,
            &|c, seed| {
                let mut r = Rng(0x0032_6ae0 ^ seed as u64);
                oc561_setup(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let count = match seed % 6 {
                    5 => (-2i32) as u32,
                    k => k,
                };
                c.bus.write32(DATA + 0x2ee8, count);
                for i in 0..6u32 {
                    c.bus.write32(DATA + 0x2eec + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn rider_sphere_tree_ctor_copy_matches() {
        oc561_install_fakes();
        check(
            ee::s10::f_00329938__generated,
            rider_sphere_tree_ctor_copy,
            16,
            &|c, seed| {
                let mut r = Rng(0x0032_9938 ^ seed as u64);
                oc561_setup(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x40 * seed);
            },
        );
    }

    #[test]
    fn rle_decode_matches() {
        check(ee::s10::f_0032b620__generated, rle_decode, 200, &|c, seed| {
            let mut r = Rng(0x0032_b620 ^ seed as u64);
            scramble(c, &mut r);
            let src = DATA + 0x1000;
            let mut at = src;
            if seed % 6 == 0 {
                c.g[A1][0] = 0;
            } else {
                c.g[A1][0] = sx(src);
                let segments = 1 + r.next_u32() % 6;
                for _ in 0..segments {
                    if r.next_u32() & 1 == 0 {
                        // Repeat run: header 0..127, then the byte to repeat.
                        c.bus.write8(at, (1 + r.next_u32() % 40) as u8);
                        c.bus.write8(at + 1, r.next_u32() as u8);
                        at += 2;
                    } else {
                        // Literal run: header -n, then n bytes.
                        let n = 1 + r.next_u32() % 20;
                        c.bus.write8(at, (256 - n) as u8);
                        at += 1;
                        for _ in 0..n {
                            c.bus.write8(at, r.next_u32() as u8);
                            at += 1;
                        }
                    }
                }
                c.bus.write8(at, 0);
            }
            c.g[A0][0] = sx(DATA + 0x20000);
        });
    }
}
