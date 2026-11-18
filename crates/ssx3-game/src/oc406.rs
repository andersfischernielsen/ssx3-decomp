//! Staging module oc406 (wave wave218): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc406`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x00323540,
        input_parser_parse_logical_and,
        "InputParser_ParseLogicalAnd",
    ),
    (
        0x00151ef0,
        be_apply_bolt_dependencies,
        "BE_ApplyBoltDependencies",
    ),
    (
        0x0012bf68,
        rider_gate_sub_handle_input,
        "RiderGateSub_HandleInput",
    ),
    (
        0x0027c9b0,
        script_rider_read_fields,
        "ScriptRider_ReadFields",
    ),
    (
        0x00326150,
        input_preproc_process_text,
        "InputPreproc_ProcessText",
    ),
    (
        0x003233b8,
        input_parser_parse_logical_or,
        "InputParser_ParseLogicalOr",
    ),
    (
        0x0035c5a0,
        rail_modifier_get_world_matrix,
        "RailModifier_GetWorldMatrix",
    ),
    (
        0x002743e8,
        in_seq_channel_update_tracks,
        "InSeq_ChannelUpdateTracks",
    ),
    (0x00274100, in_seq_channel_advance, "InSeq_ChannelAdvance"),
    (
        0x00324ec8,
        input_parser_emit_binary_op,
        "InputParser_EmitBinaryOp",
    ),
    (
        0x00323268,
        input_parser_parse_conditional,
        "InputParser_ParseConditional",
    ),
    (0x00310c48, rider_model_find_bone, "RiderModel_FindBone"),
    (0x00274de0, in_seq_instance_tick, "InSeq_InstanceTick"),
    (
        0x003230e0,
        input_parser_parse_statement,
        "InputParser_ParseStatement",
    ),
    (
        0x0033dbe8,
        cell_collide_column_recursive,
        "Cell_CollideColumnRecursive",
    ),
    (
        0x0030d8b8,
        rider_model_load_variant,
        "RiderModel_LoadVariant",
    ),
    (
        0x00132770,
        rider_railslide_sub_check_airborne,
        "RiderRailslideSub_CheckAirborne",
    ),
    (0x003aaa08, c_ps2_chunk_resolve_v19, "cPS2ChunkResolve_v19"),
    (
        0x00160028,
        camera_wobble_update_target,
        "CameraWobble_UpdateTarget",
    ),
    (
        0x00273330,
        in_seq_event_track_update,
        "InSeq_EventTrackUpdate",
    ),
];

// ---------------------------------------------------------------- helpers

/// `lw` through a register: `*(reg + off)` (sign-extended word).
fn o406_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.lw(a)
}

/// `sw` through a register: `*(reg + off) = val_reg`.
fn o406_sw(c: &mut Cpu, reg: usize, off: u32, val_reg: usize) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    let v = c.g[val_reg][0];
    c.sw(a, v);
}

// ------------------------------------------------- RiderGateSub_HandleInput

/// `void RiderGateSub_HandleInput(Gate *self, int a1)` (0x12bf68): the start
/// gate controller. Feeds the boost input, then runs the sub-state selected
/// by `*self`: 0 charge, 1 launch, 2 wait/push, 3 push; other states do
/// nothing.
pub fn rider_gate_sub_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 0;
        c.g[A0][0] = o406_lw(c, S0, 0x14);
        call_ptr(c, 0x0011_4130, 0x0012_bf90)?;
        let state = o406_lw(c, S0, 0);
        c.g[V1][0] = state;
        c.g[V0][0] = 1;
        c.g[V0][0] = ((state as i64) < 2) as u64;
        if state == 1 {
            // 0x12bfe0: launch.
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0012_c408, 0x0012_bfec)?;
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                // State 2 or more.
                c.g[V0][0] = 2;
                c.cycles += 2;
                c.g[V0][0] = 3;
                if state == 2 {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0012_c0c0, 0x0012_c000)?;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    if state == 3 {
                        c.cycles += 3;
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0012_c130, 0x0012_c010)?;
                    } else {
                        c.cycles += 2;
                    }
                }
            } else {
                // State below 2 (and not 1).
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                if state == 0 {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0012_c230, 0x0012_bfd8)?;
                } else {
                    c.cycles += 2;
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ----------------------------------------------- InputParser_ParseLogicalAnd

/// Is the constant node `node` (kind 4, value at +4) equal to 0.0 on the FPU
/// compare the parser uses (`c.eq.s f1, f0`)? Sets f1/f0/fcc like the original.
fn o406_const_is_zero(c: &mut Cpu, node_reg: usize) -> bool {
    let a = (c.g[node_reg][0] as u32).wrapping_add(4);
    c.lwc1(1, a);
    c.mtc1(0, 0);
    c.c_eq_s(1, 0);
    c.fcc()
}

/// `tParseNode *InputParser_ParseLogicalAnd(InputParser *self)` (0x323540):
/// `a && b && ...` with constant folding (a constant 0 operand makes the
/// whole chain 0, a constant non-zero operand disappears). Node kind 7 is
/// the `&&` node (children +4, +8); token 0xd is `&&`.
pub fn input_parser_parse_logical_and(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S0, 0x70);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        call_ptr(c, 0x0032_36d8, 0x0032_3574)?;
        c.g[S1][0] = c.g[V0][0];
        'body: {
            if c.g[S1][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'body;
            }
            c.g[V1][0] = o406_lw(c, S1, 0);
            c.cycles += 3;
            c.g[V0][0] = 4;
            c.g[S4][0] = 0;
            if c.g[V1][0] == 4 {
                c.cycles += 6;
                if o406_const_is_zero(c, S1) {
                    c.g[S4][0] = 1;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = o406_lw(c, S2, 0x98);
            c.g[V0][0] = 0xd;
            c.g[V0][0] = c.g[S1][0];
            if c.g[V1][0] != 0xd {
                break 'body;
            }
            c.cycles += 5;
            c.mtc1(0, 21);
            c.g[S6][0] = 7;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 20);
            c.g[S5][0] = 0xd;
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0032_1af0, 0x0032_35e8)?;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0032_36d8, 0x0032_35f0)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    c.g[A0][0] = c.g[S2][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_3588);
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0032_1638, 0x0032_3590)?;
                    c.g[V0][0] = 0;
                    break 'body;
                }
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                'iter: {
                    if c.g[S4][0] != 0 {
                        // The chain is already constant 0: drop the operand.
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0032_1638, 0x0032_360c)?;
                        c.g[V0][0] = o406_lw(c, S2, 0x98);
                        break 'iter;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = o406_lw(c, S0, 0);
                    c.g[S3][0] = 4;
                    if c.g[V0][0] != 4 {
                        c.g[V0][0] = o406_lw(c, S1, 0);
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S1][0];
                        if c.g[V0][0] == 4 {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_1638, 0x0032_367c)?;
                            c.g[S1][0] = c.g[S0][0];
                        } else {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_15f8, 0x0032_368c)?;
                            o406_sw(c, V0, 4, S1);
                            o406_sw(c, V0, 8, S0);
                            c.g[S1][0] = c.g[V0][0];
                            o406_sw(c, V0, 0, S6);
                        }
                    } else {
                        c.cycles += 5;
                        let a = (c.g[S0][0] as u32).wrapping_add(4);
                        c.lwc1(0, a);
                        c.c_eq_s(0, 21);
                        let nonzero = !c.fcc();
                        c.g[A1][0] = c.g[S1][0];
                        if nonzero {
                            c.cycles += 6;
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_1638, 0x0032_3658)?;
                            c.g[V0][0] = o406_lw(c, S1, 0);
                            if c.g[V0][0] == c.g[S3][0] {
                                let a = (c.g[S1][0] as u32).wrapping_add(4);
                                c.swc1(20, a);
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = o406_lw(c, S2, 0x98);
                                break 'iter;
                            }
                        } else {
                            // The operand is the constant 0: the chain is 0.
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_1638, 0x0032_3640)?;
                            c.g[S1][0] = c.g[S0][0];
                            c.g[S4][0] = 1;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = o406_lw(c, S2, 0x98);
                }
                c.cycles += 2;
                let more = c.g[V0][0] == c.g[S5][0];
                c.g[V0][0] = c.g[S1][0];
                if !more {
                    break 'body;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_35e0);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ InputParser_ParseLogicalOr

/// `tParseNode *InputParser_ParseLogicalOr(InputParser *self)` (0x3233b8):
/// `a || b || ...` over `ParseLogicalAnd`, with constant folding (a constant
/// non-zero operand makes the chain 1.0, a constant 0 operand disappears).
/// Node kind 6 is the `||` node; token 0xc is `||`.
pub fn input_parser_parse_logical_or(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x78);
        frame.save_fpr(c, 20, 0x70);
        call_ptr(c, 0x0032_3540, 0x0032_33e8)?;
        c.g[S1][0] = c.g[V0][0];
        'body: {
            if c.g[S1][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'body;
            }
            c.g[V1][0] = o406_lw(c, S1, 0);
            c.cycles += 3;
            c.g[V0][0] = 4;
            c.g[S3][0] = 0;
            if c.g[V1][0] == 4 {
                c.cycles += 6;
                if !o406_const_is_zero(c, S1) {
                    c.g[S3][0] = 1;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = o406_lw(c, S2, 0x98);
            c.g[V0][0] = 0xc;
            c.g[V0][0] = c.g[S1][0];
            if c.g[V1][0] != 0xc {
                break 'body;
            }
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 20);
            c.g[S5][0] = 6;
            c.mtc1(0, 21);
            c.g[S4][0] = 0xc;
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0032_1af0, 0x0032_3460)?;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0032_3540, 0x0032_3468)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    c.g[A0][0] = c.g[S2][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_33fc);
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0032_1638, 0x0032_3404)?;
                    c.g[V0][0] = 0;
                    break 'body;
                }
                c.cycles += 2;
                'iter: {
                    // true: go on to the node-combining code at 0x323484
                    // (after the `s3` shortcut).
                    let mut combine = true;
                    if c.g[S3][0] != 0 {
                        // The chain is already constant 1.0.
                        c.cycles += 2;
                        let a = (c.g[S1][0] as u32).wrapping_add(4);
                        c.swc1(20, a);
                        combine = false;
                    } else {
                        c.g[V0][0] = o406_lw(c, S0, 0);
                    }
                    if combine {
                        c.cycles += 3;
                        c.g[V1][0] = 4;
                        if c.g[V0][0] != c.g[V1][0] {
                            c.g[V0][0] = o406_lw(c, S1, 0);
                            c.cycles += 2;
                            c.g[A1][0] = c.g[S1][0];
                            if c.g[V0][0] == c.g[V1][0] {
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0032_1638, 0x0032_34e4)?;
                                c.g[S1][0] = c.g[S0][0];
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0032_15f8, 0x0032_34f4)?;
                                o406_sw(c, V0, 4, S1);
                                o406_sw(c, V0, 8, S0);
                                c.g[S1][0] = c.g[V0][0];
                                o406_sw(c, V0, 0, S5);
                            }
                            c.cycles += 1;
                            c.g[V0][0] = o406_lw(c, S2, 0x98);
                            break 'iter;
                        }
                        c.cycles += 5;
                        let a = (c.g[S0][0] as u32).wrapping_add(4);
                        c.lwc1(0, a);
                        c.c_eq_s(0, 21);
                        let zero = c.fcc();
                        c.g[A1][0] = c.g[S0][0];
                        if !zero {
                            // The operand is a non-zero constant: the chain is 1.0.
                            c.cycles += 7;
                            let a = (c.g[S0][0] as u32).wrapping_add(4);
                            c.swc1(20, a);
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A0][0] = c.g[S2][0];
                            c.g[S1][0] = c.g[S0][0];
                            call_ptr(c, 0x0032_1638, 0x0032_34b8)?;
                            c.g[S3][0] = 1;
                            c.cycles += 1;
                            c.g[V0][0] = o406_lw(c, S2, 0x98);
                            break 'iter;
                        }
                    } else {
                        c.cycles += 1;
                        c.g[A1][0] = c.g[S0][0];
                    }
                    // 0x3234c4: drop the operand `s0`.
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0032_1638, 0x0032_34cc)?;
                    c.g[V0][0] = o406_lw(c, S2, 0x98);
                }
                c.cycles += 2;
                let more = c.g[V0][0] == c.g[S4][0];
                c.g[V0][0] = c.g[S1][0];
                if !more {
                    break 'body;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_3458);
                }
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ InputParser_ParseConditional

/// `tParseNode *InputParser_ParseConditional(InputParser *self)` (0x323268):
/// `cond ? a : b` over `ParseLogicalOr` (token 6 `?`, token 7 `:`), folding
/// a constant condition. Node kind 5 is the select node (children +4 cond,
/// +8 then, +0xc else).
pub fn input_parser_parse_conditional(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0032_33b8, 0x0032_3288)?;
        c.g[S1][0] = c.g[V0][0];
        'body: {
            if c.g[S1][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'body;
            }
            c.g[V1][0] = o406_lw(c, S0, 0x98);
            c.cycles += 3;
            c.g[V0][0] = 6;
            c.g[V0][0] = c.g[S1][0];
            if c.g[V1][0] != 6 {
                break 'body;
            }
            // `?`: the "then" expression.
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0032_1af0, 0x0032_32b0)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0032_3268, 0x0032_32b8)?;
            c.g[S3][0] = c.g[V0][0];
            'fail: {
                // `fail`: free the condition, then the "then" node, return 0.
                if c.g[S3][0] == 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    break 'fail;
                }
                c.g[V1][0] = o406_lw(c, S0, 0x98);
                c.cycles += 3;
                c.g[V0][0] = 7;
                c.g[A1][0] = 0x0049_0000;
                if c.g[V1][0] != 7 {
                    // Missing `:`.
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2058));
                    call_ptr(c, 0x0032_1500, 0x0032_32e8)?;
                    c.g[A1][0] = c.g[S1][0];
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0032_1af0, 0x0032_32f8)?;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0032_3268, 0x0032_3300)?;
                    c.g[S2][0] = c.g[V0][0];
                    if c.g[S2][0] != 0 {
                        // Both branches parsed: fold or build the select node.
                        c.g[V1][0] = o406_lw(c, S1, 0);
                        c.cycles += 3;
                        c.g[V0][0] = 4;
                        if c.g[V1][0] == 4 {
                            c.cycles += 6;
                            let zero = o406_const_is_zero(c, S1);
                            c.g[A0][0] = c.g[S0][0];
                            if !zero {
                                // Constant true: the "then" node.
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S2][0];
                                call_ptr(c, 0x0032_1638, 0x0032_335c)?;
                                c.g[V0][0] = c.g[S3][0];
                                break 'body;
                            }
                            // Constant false: the "else" node.
                            c.cycles += 4;
                            c.g[A1][0] = c.g[S3][0];
                            call_ptr(c, 0x0032_1638, 0x0032_336c)?;
                            c.g[V0][0] = c.g[S2][0];
                            break 'body;
                        }
                        c.cycles += 9;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_15f8, 0x0032_337c)?;
                        c.g[V1][0] = c.g[V0][0];
                        c.g[V0][0] = 5;
                        o406_sw(c, V1, 4, S1);
                        o406_sw(c, V1, 0, V0);
                        o406_sw(c, V1, 8, S3);
                        c.g[V0][0] = c.g[V1][0];
                        o406_sw(c, V1, 0xc, S2);
                        break 'body;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S1][0];
                }
                // 0x323310: free the condition, then the "then" node.
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0032_1638, 0x0032_3318)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
            }
            // 0x323320: free `a1` and return 0.
            c.cycles += 4;
            call_ptr(c, 0x0032_1638, 0x0032_3328)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ InputParser_ParseStatement

/// `void InputParser_ParseStatement(InputParser *self)` (0x3230e0): one
/// `name = expression;` statement of an input map script. The name token
/// (kind 2) must not be a button, a builtin or an existing identifier; it
/// gets the next entry of the variable table (0x44 bytes each from +0x80e8,
/// count at +0x80e4), which is constructed; then `=` (token 3), the
/// expression (`ParseConditional`) and `;` (token 5) follow. Any error
/// skips to the end of the statement.
pub fn input_parser_parse_statement(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 2;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = o406_lw(c, S0, 0x98);
        c.g[A1][0] = 0x0049_0000;
        // The error exits: `'skip` skips the statement (0x32322c),
        // `'finish` is the normal end (0x323254).
        'finish: {
            'accepted: {
                'skip: {
                    'error: {
                        if c.g[V0][0] != c.g[V1][0] {
                            c.cycles += 2;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2130));
                            break 'error;
                        }
                        c.cycles += 6;
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0032_16e8, 0x0032_3120)?;
                        let found = (c.g[V0][0] as i64) >= 0;
                        c.g[A1][0] = 0x0049_0000;
                        if found {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2108));
                            call_ptr(c, 0x0032_1500, 0x0032_3138)?;
                            break 'skip;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0032_1880, 0x0032_314c)?;
                        let found = (c.g[V0][0] as i64) >= 0;
                        c.g[A1][0] = 0x0049_0000;
                        if found {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x20e0));
                            call_ptr(c, 0x0032_1500, 0x0032_3164)?;
                            break 'skip;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0032_1a40, 0x0032_3178)?;
                        let found = c.g[V0][0] != 0;
                        c.g[A1][0] = 0x0049_0000;
                        if found {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x20b8));
                            call_ptr(c, 0x0032_1500, 0x0032_3190)?;
                            break 'skip;
                        }
                        // A new variable: entry `count` of the table.
                        c.cycles += 20;
                        c.g[V0][0] = 0x8000;
                        c.g[A2][0] = 0x44;
                        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = 0x80e8;
                        c.g[V0][0] = o406_lw(c, S2, 0xe4);
                        c.g[A1][0] = c.g[S1][0];
                        c.g[V1][0] = 0x10000;
                        c.mult(0, V0, V0, A2);
                        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        o406_sw(c, V1, 0x8e8, S1);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x0041_66f4, 0x0032_31d0)?;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_1af0, 0x0032_31d8)?;
                        c.g[V1][0] = o406_lw(c, S0, 0x98);
                        c.g[V0][0] = 3;
                        c.g[A1][0] = 0x0049_0000;
                        if c.g[V1][0] != 3 {
                            // Missing `=`.
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2098));
                            break 'error;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_1af0, 0x0032_31fc)?;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_3268, 0x0032_3204)?;
                        o406_sw(c, S1, 0x40, V0);
                        if c.g[V0][0] == 0 {
                            break 'skip;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = o406_lw(c, S0, 0x98);
                        c.g[V0][0] = 5;
                        c.g[A1][0] = 0x0049_0000;
                        if c.g[V1][0] == 5 {
                            break 'accepted;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2078));
                    }
                    // 0x323224: report the error.
                    c.cycles += 2;
                    call_ptr(c, 0x0032_1500, 0x0032_322c)?;
                }
                // 0x32322c: skip the rest of the statement.
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0032_1590, 0x0032_3234)?;
                frame.restore(c, S0, 0x30);
                break 'finish;
            }
            // 0x32323c: the `;` ends the statement; count the variable.
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0032_1af0, 0x0032_3244)?;
            c.g[V0][0] = o406_lw(c, S2, 0xe4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            o406_sw(c, S2, 0xe4, V0);
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ InputParser_EmitBinaryOp

/// `int InputParser_EmitBinaryOp(InputParser *self, Node *node, int *dst,
/// u32 *code, int cap, int op, int a, int b)` (0x324ec8): compile both
/// operands of a binary node (`node+4`, `node+8`) with `EmitNode`, free
/// their temporary registers, pick the destination register, and emit the
/// opcode word (`op` 6 bits, `dst` 6 bits, src 10 bits, arg 10 bits).
/// Returns the number of words written, or -1 after an error.
pub fn input_parser_emit_binary_op(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[T0][0];
        frame.save(c, S2, 0x80);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x70);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x60);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S5, 0x50);
        c.g[S4][0] = c.g[T2][0];
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, FP, 0x20);
        c.g[S7][0] = u64::MAX;
        frame.save_ra(c, 0x10);
        c.g[FP][0] = c.g[T1][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = o406_lw(c, S3, 4);
        o406_sw(c, SP, 0, S7);
        c.g[T1][0] = c.g[S4][0];
        call_ptr(c, 0x0032_4678, 0x0032_4f28)?;
        c.g[S5][0] = c.g[V0][0];
        'body: {
            if (c.g[S5][0] as i64) < 0 {
                c.g[V0][0] = u64::MAX;
                break 'body;
            }
            c.cycles += 14;
            c.g[V0][0] = sx((c.g[S5][0] as u32) << 2);
            c.g[A1][0] = o406_lw(c, S3, 8);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[S5][0] as u32));
            c.g[T0][0] = c.g[S0][0];
            o406_sw(c, SP, 4, S7);
            c.g[T1][0] = c.g[S4][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[SP][0] | 4;
            c.g[A3][0] = c.g[S2][0];
            call_ptr(c, 0x0032_4678, 0x0032_4f60)?;
            c.g[T0][0] = c.g[V0][0];
            if (c.g[T0][0] as i64) < 0 {
                c.g[V0][0] = u64::MAX;
                break 'body;
            }
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
            c.g[A1][0] = o406_lw(c, SP, 4);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = (c.g[A1][0] < 0x40) as u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
            if c.g[V0][0] != 0 {
                c.cycles += 8;
                o406_emit_free_reg(c);
            }
            c.cycles += 4;
            c.g[A1][0] = o406_lw(c, SP, 0);
            c.g[V0][0] = (c.g[A1][0] < 0x40) as u64;
            c.g[A0][0] = 0x10000;
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                o406_emit_free_reg(c);
            }
            'overflow: {
                'alloc_failed: {
                    c.cycles += 3;
                    c.g[V0][0] = o406_lw(c, S6, 0);
                    let have_dst = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = 0x10000;
                    if !have_dst {
                        c.cycles += 4;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V0][0] = o406_lw(c, A0, 0x9e8);
                        let none = (c.g[V0][0] as i64) <= 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                        if !none {
                            c.cycles += 5;
                            c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                            o406_sw(c, A0, 0x9e8, V0);
                            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V0][0] = o406_lw(c, V1, 0x8ec);
                            o406_sw(c, S6, 0, V0);
                        }
                        c.cycles += 3;
                        c.g[V0][0] = o406_lw(c, S6, 0);
                        let have_dst = (c.g[V0][0] as i64) >= 0;
                        c.g[A1][0] = 0x0049_0000;
                        if !have_dst {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1dc0));
                            break 'alloc_failed;
                        }
                    }
                    c.cycles += 2;
                    let no_room = (c.g[S0][0] as i64) <= 0;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffc0;
                    if no_room {
                        c.cycles += 3;
                        c.g[A1][0] = 0x0049_0000;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1dd8));
                        break 'overflow;
                    }
                    // Emit the opcode word.
                    c.cycles += 28;
                    c.g[V1][0] = o406_lw(c, S2, 0);
                    c.g[A0][0] = c.g[FP][0] & 0x3f;
                    c.g[A2][0] = o406_lw(c, SP, 0);
                    c.g[V1][0] &= c.g[V0][0];
                    c.g[A1][0] = 0xffff_ffff_ffff_f03f;
                    c.g[V1][0] |= c.g[A0][0];
                    c.g[A3][0] = 0xffff_ffff_ffc0_0000;
                    o406_sw(c, S2, 0, V1);
                    c.g[A2][0] &= 0x3ff;
                    c.g[V1][0] &= c.g[A1][0];
                    c.g[A3][0] |= 0xfff;
                    c.g[V0][0] = o406_lw(c, S6, 0);
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 12);
                    c.g[A1][0] = o406_lw(c, SP, 4);
                    c.g[A0][0] = 0x3f_0000;
                    c.g[V0][0] &= 0x3f;
                    c.g[A0][0] |= 0xffff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 22);
                    c.g[V1][0] |= c.g[V0][0];
                    c.g[V1][0] &= c.g[A3][0];
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[V1][0] |= c.g[A2][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    c.g[V1][0] &= c.g[A0][0];
                    c.g[V1][0] |= c.g[A1][0];
                    o406_sw(c, S2, 0, V1);
                    break 'body;
                }
                // 0x3250a8-side: both failures end in the parser error.
                break 'overflow;
            }
            c.cycles += 3;
            call_ptr(c, 0x0032_1500, 0x0032_50a4)?;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Return a temporary register (`a1`, below 0x40) to the parser's free list:
/// `list[count++] = a1` (list at +0x108ec, count at +0x109e8), as both
/// operand registers of `EmitBinaryOp` are.
fn o406_emit_free_reg(c: &mut Cpu) {
    c.g[A0][0] = 0x10000;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = o406_lw(c, A0, 0x9e8);
    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    o406_sw(c, V0, 0x8ec, A1);
    o406_sw(c, A0, 0x9e8, V1);
}

// ------------------------------------------------ InputPreproc_ProcessText

/// `int InputPreproc_ProcessText(InputPreproc *self, Cursor *in, a2, a3)`
/// (0x326150): copy one source line to the output. Leading blanks (space,
/// tab, CR) are written through; a `#` starts a directive, `//` a line
/// comment, `/*` a block comment (each handled by its own function); any
/// other character copies the rest of the line, counting it at its
/// newline. Returns 1 at end of input (when the lookahead char at +0xc is
/// negative), else 0.
pub fn input_preproc_process_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A3][0];
        frame.save(c, S6, 0x20);
        c.g[S5][0] = 0xd;
        frame.save(c, S7, 0x10);
        c.g[S6][0] = 9;
        frame.save(c, S1, 0x70);
        c.g[S7][0] = 0x20;
        frame.save_ra(c, 0);
        'epilogue: {
            'to_d8: {
                'to_d4: {
                    // Leading blanks.
                    let mut first = true;
                    loop {
                        if !first {
                            c.cycles += 8;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0032_6360, 0x0032_61a0)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_6478, 0x0032_61ac)?;
                            c.g[V0][0] = o406_lw(c, S0, 0xc);
                            let eof = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = 0;
                            if eof {
                                break 'epilogue;
                            }
                        }
                        first = false;
                        c.cycles += 3;
                        c.g[S1][0] = o406_lw(c, S0, 0xc);
                        c.g[V0][0] = 0;
                        if c.g[S1][0] == c.g[S7][0] {
                            c.cycles += 1;
                            c.g[V0][0] = 1;
                        } else {
                            c.cycles += 2;
                            if c.g[S1][0] == c.g[S6][0] {
                                c.g[V0][0] = 1;
                            } else {
                                c.cycles += 2;
                                if c.g[S1][0] == c.g[S5][0] {
                                    c.cycles += 1;
                                    c.g[V0][0] = 1;
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        let a = (c.g[S0][0] as u32).wrapping_add(0xc);
                        c.g[A1][0] = c.lb(a);
                        if event_due(c) {
                            deliver_event_here(c, 0x0032_6198);
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x23;
                    let not_hash = c.g[S1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2f;
                    if !not_hash {
                        // `#`: a directive.
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x0032_6478, 0x0032_61f8)?;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[A3][0] = c.g[S4][0];
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_66e8, 0x0032_620c)?;
                        c.g[V0][0] = o406_lw(c, S0, 0xc);
                        break 'to_d4;
                    }
                    c.cycles += 2;
                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[S1][0] = 0xa;
                    } else {
                        // `/`: a comment, or a plain slash.
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x0032_6478, 0x0032_6228)?;
                        c.g[V1][0] = o406_lw(c, S0, 0xc);
                        let not_slash = c.g[V1][0] != c.g[S1][0];
                        c.g[V0][0] = 0x2a;
                        if !not_slash {
                            c.cycles += 7;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S3][0];
                            c.g[A3][0] = c.g[S4][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0032_65c0, 0x0032_6248)?;
                            c.g[V0][0] = o406_lw(c, S0, 0xc);
                            break 'to_d4;
                        }
                        c.cycles += 2;
                        let not_star = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = c.g[S0][0];
                        if !not_star {
                            c.cycles += 9;
                            c.g[A1][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_6478, 0x0032_6260)?;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S3][0];
                            c.g[A3][0] = c.g[S4][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0032_64b0, 0x0032_6274)?;
                            c.g[V0][0] = o406_lw(c, S0, 0xc);
                            break 'to_d4;
                        }
                        // A plain `/`: write it, then the rest of the line.
                        c.cycles += 4;
                        c.g[A1][0] = 0x2f;
                        call_ptr(c, 0x0032_6360, 0x0032_6284)?;
                        c.g[S1][0] = 0xa;
                    }
                    // The rest of the line, up to the newline in s1.
                    loop {
                        c.cycles += 3;
                        c.g[V0][0] = o406_lw(c, S0, 0xc);
                        let eof = (c.g[V0][0] as i64) < 0;
                        c.g[V0][0] = !c.g[V0][0];
                        if eof {
                            break 'to_d8;
                        }
                        c.cycles += 6;
                        let a = (c.g[S0][0] as u32).wrapping_add(0xc);
                        c.g[A1][0] = c.lb(a);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0032_6360, 0x0032_62b0)?;
                        c.g[V0][0] = o406_lw(c, S0, 0xc);
                        let more = c.g[V0][0] != c.g[S1][0];
                        c.g[A0][0] = c.g[S0][0];
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0032_6290);
                            }
                            c.cycles += 2;
                            c.g[A1][0] = c.g[S2][0];
                            call_ptr(c, 0x0032_6478, 0x0032_6298)?;
                            continue;
                        }
                        // Newline: count the line, step past it.
                        c.cycles += 6;
                        c.g[V0][0] = o406_lw(c, S2, 0xc);
                        c.g[A1][0] = c.g[S2][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                        o406_sw(c, S2, 0xc, V0);
                        call_ptr(c, 0x0032_6478, 0x0032_62d0)?;
                        c.g[V0][0] = o406_lw(c, S0, 0xc);
                        break 'to_d4;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = !c.g[V0][0];
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ InSeq_ChannelUpdateTracks

/// `bool InSeq_ChannelUpdateTracks(InSeqChannel *self, float frame, float dt)`
/// (0x2743e8): update every track player of the channel (virtual call at
/// vtable +0x1c with the frame, dt and an output flag word), OR the
/// flags together and tell the entity communicator (`self+0x2c`), if any,
/// about them. Returns whether any track reported progress.
pub fn in_seq_channel_update_tracks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0;
        frame.save_fpr(c, 21, 0x68);
        c.g[S3][0] = 0;
        frame.save_fpr(c, 20, 0x60);
        c.mov_s(21, 13);
        frame.save_ra(c, 0x10);
        c.g[V1][0] = o406_lw(c, S0, 0x1c);
        let a = (c.g[V1][0] as u32).wrapping_add(2);
        c.g[V0][0] = c.lbu(a);
        c.mov_s(20, 12);
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            loop {
                c.cycles += 22;
                c.g[V1][0] = o406_lw(c, S0, 0x24);
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                let a = frame.at(c, 0);
                c.sw(a, 0);
                c.mov_s(12, 20);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.mov_s(13, 21);
                c.g[A2][0] = o406_lw(c, V0, 0);
                c.g[A1][0] = c.g[SP][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V1][0] = o406_lw(c, A2, 4);
                let a = (c.g[V1][0] as u32).wrapping_add(0x18);
                c.g[A0][0] = c.lh(a);
                c.g[V0][0] = o406_lw(c, V1, 0x1c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_4468)?;
                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                c.g[A0][0] = o406_lw(c, S0, 0x1c);
                c.g[S3][0] |= c.g[V0][0];
                c.g[V1][0] = c.lw(frame.at(c, 0));
                let a = (c.g[A0][0] as u32).wrapping_add(2);
                c.g[V0][0] = c.lbu(a);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let again = c.g[V0][0] != 0;
                c.g[S2][0] |= c.g[V1][0];
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_4430);
                }
            }
        }
        c.cycles += 3;
        c.g[T1][0] = o406_lw(c, S0, 0x2c);
        c.g[V0][0] = c.g[S3][0];
        if c.g[T1][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            if c.g[S2][0] != 0 {
                c.cycles += 9;
                c.g[V1][0] = o406_lw(c, T1, 0);
                c.g[T0][0] = o406_lw(c, S0, 0x1c);
                c.mov_s(12, 20);
                let a = (c.g[V1][0] as u32).wrapping_add(0x18);
                c.g[A0][0] = c.lh(a);
                c.g[A3][0] = c.g[S2][0];
                c.g[V0][0] = o406_lw(c, V1, 0x1c);
                let a = (c.g[T0][0] as u32).wrapping_add(3);
                c.g[A2][0] = c.lbu(a);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_44c0)?;
            }
            c.cycles += 13;
            c.g[T0][0] = o406_lw(c, S0, 0x2c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = o406_lw(c, S0, 0x1c);
            c.mov_s(12, 20);
            c.g[V1][0] = o406_lw(c, T0, 0);
            c.mov_s(13, 21);
            let a = (c.g[V0][0] as u32).wrapping_add(3);
            c.g[A2][0] = c.lbu(a);
            c.g[A3][0] = c.g[S2][0];
            let a = (c.g[V1][0] as u32).wrapping_add(0x20);
            c.g[A0][0] = c.lh(a);
            c.g[V0][0] = o406_lw(c, V1, 0x24);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_44f0)?;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------------ RiderModel_FindBone

/// `int RiderModel_FindBone(RiderModel *self, int part, const char *name)`
/// (0x310c48): the index of the bone called `name` among the part's bone
/// records (0x50 bytes each from the part's table), or -1. The table entry
/// (0x58 bytes at `self+0xc`) holds the bone array pointer (+0x38), the
/// base index (+4) and the count (+0x44).
pub fn rider_model_find_bone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S0, 0x40);
        c.g[V1][0] = 0x58;
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = o406_lw(c, A0, 0x1c);
        c.g[A3][0] = o406_lw(c, A0, 0xc);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = o406_lw(c, A1, 0);
        c.mult(0, V0, V0, V1);
        c.g[S2][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = o406_lw(c, S2, 0x44);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = o406_lw(c, S2, 0x38);
        'body: {
            if !empty {
                c.cycles += 1;
                c.g[S1][0] = c.g[A0][0];
                loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x0041_65a8, 0x0031_0ca4)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = o406_lw(c, S2, 4);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        break 'body;
                    }
                    c.g[V0][0] = o406_lw(c, S2, 0x44);
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let again = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x50));
                    if !again {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_0c98);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------------- RailModifier_GetWorldMatrix

/// `void RailModifier_GetWorldMatrix(RailModifier *self, Matrix *m)`
/// (0x35c5a0): `m = m * self->localMatrix(+0x50) * nodeMatrix`, i.e. the
/// instance's node matrix is fetched into a stack temporary, the 4x4
/// product of `m` and the modifier's matrix (VU0) is written over `m`
/// (through a second stack copy, as the compiler did).
pub fn rail_modifier_get_world_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 61;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x80);
        c.g[S1][0] = c.g[A1][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = o406_lw(c, S0, 0x40);
        c.g[A1][0] = o406_lw(c, S0, 0x34);
        call_ptr(c, 0x0034_fed8, 0x0035_c5c8)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = sx(frame.at(c, 0x40));
        let m = c.g[S1][0] as u32;
        let local = c.g[S0][0] as u32;
        let tmp = c.g[V0][0] as u32;
        let sp0 = frame.at(c, 0);
        for i in 0..4u32 {
            c.lqc2(4 + i as usize, m.wrapping_add(0x10 * i));
        }
        for i in 0..4u32 {
            c.lqc2(8 + i as usize, local.wrapping_add(0x10 * i));
        }
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        for i in 0..4u32 {
            c.sqc2(12 + i as usize, tmp.wrapping_add(0x10 * i));
        }
        for i in 0..4u32 {
            c.lqc2(1 + i as usize, tmp.wrapping_add(0x10 * i));
        }
        for i in 0..4u32 {
            c.sqc2(1 + i as usize, sp0.wrapping_add(0x10 * i));
        }
        for i in 0..4u32 {
            c.lqc2(1 + i as usize, sp0.wrapping_add(0x10 * i));
        }
        for i in 0..4u32 {
            c.sqc2(1 + i as usize, m.wrapping_add(0x10 * i));
        }
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ----------------------------------------------- CameraWobble_UpdateTarget

/// `void CameraWobble_UpdateTarget(CameraWobble *self, float amount)`
/// (0x160028): `+0x18 = amount`. With `amount == 0` the wobble state is
/// cleared; otherwise, once the timer (+0x20) has passed its limit (+0x28)
/// a new random target pair (+0xc, +0x10, each `rand * 2 - 1` from the
/// mantissa trick `1.0 | rand & 0x7fffff`) is drawn and the timer reset,
/// else the timer advances by the step constant at `gp-0x6c24`.
pub fn camera_wobble_update_target(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        frame.save_fpr(c, 22, 0x50);
        c.mtc1(0, 22);
        frame.save(c, S2, 0x10);
        c.c_eq_s(12, 22);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        let zero = c.fcc();
        let a = (c.g[S2][0] as u32).wrapping_add(0x18);
        c.swc1(12, a);
        if zero {
            c.cycles += 5;
            for off in [0x20u32, 0, 4, 0xc, 0x10] {
                let a = (c.g[S2][0] as u32).wrapping_add(off);
                c.swc1(22, a);
            }
        } else {
            c.cycles += 6;
            let a = (c.g[S2][0] as u32).wrapping_add(0x20);
            c.lwc1(1, a);
            let a = (c.g[S2][0] as u32).wrapping_add(0x28);
            c.lwc1(0, a);
            c.c_lt_s(0, 1);
            let advance = !c.fcc();
            c.g[S0][0] = 0x7f_0000;
            if advance {
                c.cycles += 4;
                let a = (c.g[GP][0] as u32).wrapping_sub(0x6c24);
                c.lwc1(0, a);
                c.add_s(0, 1, 0);
                let a = (c.g[S2][0] as u32).wrapping_add(0x20);
                c.swc1(0, a);
            } else {
                c.cycles += 28;
                let a = (c.g[S2][0] as u32).wrapping_add(0xc);
                c.lwc1(0, a);
                let a = (c.g[S2][0] as u32).wrapping_add(0x10);
                c.lwc1(1, a);
                c.g[S0][0] |= 0xffff;
                let a = c.g[S2][0] as u32;
                c.swc1(0, a);
                c.g[S1][0] = 0x3f80_0000;
                let a = (c.g[S2][0] as u32).wrapping_add(4);
                c.swc1(1, a);
                call_ptr(c, 0x0031_77f0, 0x0016_0090)?;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 20);
                c.g[V0][0] &= c.g[S0][0];
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 21);
                c.g[V0][0] |= c.g[S1][0];
                c.mtc1(2, 1);
                c.sub_s(0, 1, 20);
                c.sub_s(0, 0, 21);
                c.add_s(0, 0, 0);
                let a = (c.g[S2][0] as u32).wrapping_add(0xc);
                c.swc1(0, a);
                call_ptr(c, 0x0031_77f0, 0x0016_00c0)?;
                c.g[V0][0] &= c.g[S0][0];
                let a = (c.g[S2][0] as u32).wrapping_add(0x20);
                c.swc1(22, a);
                c.g[V0][0] |= c.g[S1][0];
                c.mtc1(2, 0);
                c.sub_s(20, 0, 20);
                c.sub_s(20, 20, 21);
                c.add_s(20, 20, 20);
                let a = (c.g[S2][0] as u32).wrapping_add(0x10);
                c.swc1(20, a);
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------- InSeq_EventTrackUpdate

/// `bool InSeq_EventTrackUpdate(EventTrack *self, float time, u32 *flags)`
/// (0x273330): fire every key (`self+0x14`, a key with its time in the
/// half-word at +2) that `time` has reached: set bit 0x10 in `*flags`,
/// fire the key and advance to the next one. Returns whether keys remain.
pub fn in_seq_event_track_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x38);
        frame.save_ra(c, 0);
        c.mov_s(21, 12);
        frame.save_fpr(c, 20, 0x30);
        let a = (c.g[S0][0] as u32).wrapping_add(0x24);
        c.lwc1(0, a);
        c.c_eq_s(0, 21);
        let same_time = c.fcc();
        c.g[S1][0] = c.g[A1][0];
        if !same_time {
            c.cycles += 2;
            o406_sw(c, S0, 0x20, ZERO);
        }
        loop {
            c.cycles += 3;
            c.g[V0][0] = o406_lw(c, S0, 0x14);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x20);
                break;
            }
            c.cycles += 7;
            let a = (c.g[V0][0] as u32).wrapping_add(2);
            c.g[V0][0] = c.lhu(a);
            c.mtc1(V0, 20);
            c.cvt_s_w(20, 20);
            c.c_lt_s(21, 20);
            if !c.fcc() {
                c.g[V0][0] = o406_lw(c, S1, 0);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_3370);
                }
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[V0][0] |= 0x10;
                o406_sw(c, S1, 0, V0);
                c.g[A1][0] = o406_lw(c, S0, 0x14);
                call_ptr(c, 0x0027_3418, 0x0027_3384)?;
                let a = (c.g[S0][0] as u32).wrapping_add(0x24);
                c.swc1(20, a);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0027_3468, 0x0027_3390)?;
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = o406_lw(c, S0, 0x14);
            frame.restore(c, S0, 0x20);
            break;
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ----------------------------------------------------- InSeq_ChannelAdvance

/// `bool InSeq_ChannelAdvance(InSeqChannel *self, float frame, float dt,
/// ..., int step)` (0x274100): advance the channel by `dt`. With a
/// non-zero `step` the time is first cut to the nearest key time any track
/// player (vtable +0x34, returns a frame count) says is ahead, and
/// `UpdateTracks` runs until the whole time is used or a call reports no
/// progress. Returns 1 when the channel is finished (`+0x28 == 0`), else
/// the last `UpdateTracks` result.
pub fn in_seq_channel_advance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x10);
        frame.save_fpr(c, 22, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x38);
        c.mov_s(22, 13);
        frame.save(c, S0, 0x20);
        c.mov_s(21, 12);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 23, 0x48);
        frame.save_fpr(c, 20, 0x30);
        c.g[V0][0] = o406_lw(c, S1, 0x28);
        let running = c.g[V0][0] != 0;
        c.mov_s(13, 14);
        'end: {
            if !running {
                c.cycles += 2;
                c.g[V0][0] = 1;
                break 'end;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            let to_last_update = if c.g[A1][0] == 0 {
                // 0x274200.
                c.cycles += 1;
                c.mov_s(12, 22);
                true
            } else {
                c.cycles += 1;
                c.g[V1][0] = o406_lw(c, S1, 0x1c);
                'step: loop {
                    // 0x274158: the smallest positive key distance of any track.
                    c.cycles += 4;
                    c.mov_s(20, 22);
                    let a = (c.g[V1][0] as u32).wrapping_add(2);
                    c.g[V0][0] = c.lbu(a);
                    c.g[S0][0] = 0;
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.mtc1(0, 23);
                        c.g[V1][0] = o406_lw(c, S1, 0x24);
                        loop {
                            c.cycles += 14;
                            c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[A0][0] = o406_lw(c, V0, 0);
                            c.g[V1][0] = o406_lw(c, A0, 4);
                            let a = (c.g[V1][0] as u32).wrapping_add(0x30);
                            c.g[A1][0] = c.lh(a);
                            c.g[V0][0] = o406_lw(c, V1, 0x34);
                            let target = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            call_ptr(c, target, 0x0027_4190)?;
                            c.mtc1(2, 0);
                            c.cvt_s_w(0, 0);
                            c.c_le_s(23, 0);
                            if !c.fcc() {
                                c.g[V1][0] = o406_lw(c, S1, 0x1c);
                            } else {
                                c.cycles += 4;
                                c.c_lt_s(0, 20);
                                if c.fcc() {
                                    c.mov_s(20, 0);
                                }
                                c.cycles += 1;
                                c.g[V1][0] = o406_lw(c, S1, 0x1c);
                            }
                            c.cycles += 5;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                            let a = (c.g[V1][0] as u32).wrapping_add(2);
                            c.g[V0][0] = c.lbu(a);
                            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                break;
                            }
                            c.g[V1][0] = o406_lw(c, S1, 0x24);
                            if event_due(c) {
                                deliver_event_here(c, 0x0027_4170);
                            }
                        }
                    }
                    // 0x2741d0.
                    c.cycles += 4;
                    c.c_eq_s(20, 22);
                    let nothing_left = c.fcc();
                    c.g[A0][0] = c.g[S1][0];
                    if nothing_left {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_4140);
                        }
                        // 0x274140.
                        c.cycles += 3;
                        c.mov_s(12, 22);
                        c.sub_s(13, 12, 21);
                        break 'step true;
                    }
                    c.cycles += 6;
                    c.sub_s(13, 20, 21);
                    c.mov_s(21, 20);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x0027_43e8, 0x0027_41f0)?;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = o406_lw(c, S1, 0x1c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_4158);
                        }
                        continue 'step;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'step false;
                }
            };
            if to_last_update {
                c.cycles += 2;
                call_ptr(c, 0x0027_43e8, 0x0027_420c)?;
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x48);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ----------------------------------------------- BE_ApplyBoltDependencies

/// `int BE_ApplyBoltDependencies(BE *self, int id, int on)` (0x151ef0):
/// switching bolt `id` on or off pulls its dependency links from the bolt
/// database. Each link (0xc bytes: condition id +6, condition value +5,
/// target +0xa, wanted state +9): skipped unless its condition item matches,
/// then the target item (item index table at `self+0x288`, items 4 words
/// from `self+0x290`) is either `id` itself (set/clear its 0x10 flag) or
/// another bolt that is switched through `BE_SetCharProfileBoltEquipped`.
/// Returns 0 when that call fails, else 1.
pub fn be_apply_bolt_dependencies(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S6, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S5, 0x40);
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S6][0] = c.g[A1][0];
        call_ptr(c, 0x0014_bdb8, 0x0015_1f28)?;
        c.g[S5][0] = c.g[V0][0];
        let a = (c.g[S2][0] as u32).wrapping_add(0xbc0);
        c.g[A1][0] = c.lb(a);
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S5][0];
        let a = frame.at(c, 0);
        c.sw(a, 0);
        c.g[A2][0] = c.g[S6][0];
        c.g[T0][0] = c.g[SP][0];
        call_ptr(c, 0x0014_db40, 0x0015_1f48)?;
        c.g[A0][0] = o406_lw(c, S2, 0x288);
        c.g[V1][0] = sx((c.g[S6][0] as u32) << 1);
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = c.lh(c.g[V1][0] as u32);
        let missing = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if missing {
            c.cycles += 1;
            c.g[S3][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
            c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
        'ret: {
            'done: {
                c.cycles += 3;
                c.g[V0][0] = c.lw(frame.at(c, 0));
                let none = (c.g[V0][0] as i64) <= 0;
                c.g[S4][0] = 0;
                if none {
                    break 'done;
                }
                c.cycles += 2;
                c.g[S7][0] = u64::MAX;
                loop {
                    'tail: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xa));
                        let no_target = c.g[V0][0] == c.g[S7][0];
                        c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xa));
                        if no_target {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(frame.at(c, 0));
                            break 'tail;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(6));
                        let no_cond = c.g[V0][0] == c.g[S7][0];
                        c.g[A1][0] = o406_lw(c, S2, 0x288);
                        if !no_cond {
                            // Is the condition item owned the way the link wants?
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.g[V0][0] = c.lh(c.g[V0][0] as u32);
                            let absent = (c.g[V0][0] as i64) < 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            if absent {
                                c.cycles += 1;
                                c.g[V0][0] = 0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                c.g[V0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            }
                            c.cycles += 7;
                            c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(2));
                            c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(5));
                            c.g[V0][0] &= 0x10;
                            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                            c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
                            let differ = c.g[V0][0] != c.g[V1][0];
                            c.g[V0][0] = c.lw(frame.at(c, 0));
                            if differ {
                                break 'tail;
                            }
                        }
                        // The target item.
                        c.cycles += 15;
                        c.g[V0][0] = sx((c.g[A0][0] as u32) << 16);
                        c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                        c.g[A0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(9));
                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 1);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[A0][0] = (0u64 < c.g[A0][0]) as u64;
                        c.g[V0][0] = c.lh(c.g[V1][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(2));
                        c.g[V1][0] &= 0x10;
                        c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
                        if c.g[V1][0] != c.g[A0][0] {
                            c.g[A1][0] = c.lh(c.g[S1][0] as u32);
                        } else {
                            c.cycles += 2;
                            let owned = c.g[V1][0] != 0;
                            c.g[V0][0] = c.lw(frame.at(c, 0));
                            if owned {
                                break 'tail;
                            }
                            // Not owned, link wants it off: only when the bolt
                            // definition has flag 8.
                            c.cycles += 5;
                            let a = (c.g[S2][0] as u32).wrapping_add(0xbc0);
                            c.g[A1][0] = c.lb(a);
                            c.g[A0][0] = c.g[S5][0];
                            call_ptr(c, 0x0014_d448, 0x0015_2030)?;
                            let found = (c.g[V0][0] as i64) >= 0;
                            c.g[A0][0] = c.g[V0][0];
                            if found {
                                c.cycles += 4;
                                c.g[V0][0] = 0x38;
                                c.g[V1][0] = o406_lw(c, S5, 4);
                                c.mult(0, V0, A0, V0);
                                c.g[V0][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = 0;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = o406_lw(c, V0, 0x34);
                            c.g[V0][0] &= 8;
                            let no_flag = c.g[V0][0] == 0;
                            c.g[V0][0] = c.lw(frame.at(c, 0));
                            if no_flag {
                                break 'tail;
                            }
                            c.cycles += 1;
                            c.g[A1][0] = c.lh(c.g[S1][0] as u32);
                        }
                        // 0x152090: the target is `id` itself, or another bolt.
                        c.cycles += 2;
                        if c.g[A1][0] != c.g[S6][0] {
                            let a = (c.g[S0][0] as u32).wrapping_add(9);
                            c.g[A2][0] = c.lb(a);
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = (0u64 < c.g[A2][0]) as u64;
                            call_ptr(c, 0x0015_1c90, 0x0015_209c)?;
                            let failed = c.g[V0][0] == 0;
                            c.g[V0][0] = 0;
                            if failed {
                                break 'ret;
                            }
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(9));
                            let off = c.g[V0][0] == 0;
                            c.g[V0][0] = c.lhu((c.g[S3][0] as u32).wrapping_add(2));
                            if !off {
                                c.cycles += 3;
                                c.g[V0][0] |= 0x10;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] &= 0xffef;
                            }
                            let a = (c.g[S3][0] as u32).wrapping_add(2);
                            c.sh(a, c.g[V0][0]);
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw(frame.at(c, 0));
                    }
                    c.cycles += 4;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    if !more {
                        break 'done;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0015_1f88);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ ScriptRider_ReadFields

/// How `ScriptRider_ReadFields` stores one track value.
#[derive(Clone, Copy)]
enum O406Store {
    /// `out[n] = *value` (word read through the returned pointer).
    Deref,
    /// `out[n] = value` (the call returns the word itself).
    Direct,
    /// `out[n] = *(float*)value`.
    Float,
}

/// `void ScriptRider_ReadFields(ScriptEntity *self, ScriptRiderFields *out)`
/// (0x27c9b0): read the twelve values of the rider's script channels into
/// `out`: three (value, value) word pairs from tracks 0..2, then six floats
/// from tracks 3..8. Each is one `InSeq_ChannelGetTrack(self, track)` and
/// one virtual read call on the track (vtable offsets +0x20 / +0x28).
pub fn script_rider_read_fields(c: &mut Cpu) {
    // (voff, vcall return, store, out offset, next track, GetTrack return)
    const STEPS: [(u32, u32, O406Store, u32, u32, u32); 12] = [
        (0x20, 0x0027_c9e4, O406Store::Deref, 0x0, 0, 0x0027_c9f8),
        (0x28, 0x0027_ca0c, O406Store::Direct, 0x4, 1, 0x0027_ca1c),
        (0x20, 0x0027_ca30, O406Store::Deref, 0x8, 1, 0x0027_ca44),
        (0x28, 0x0027_ca58, O406Store::Direct, 0xc, 2, 0x0027_ca68),
        (0x20, 0x0027_ca7c, O406Store::Deref, 0x10, 2, 0x0027_ca90),
        (0x28, 0x0027_caa4, O406Store::Direct, 0x14, 3, 0x0027_cab4),
        (0x20, 0x0027_cac8, O406Store::Float, 0x18, 4, 0x0027_cadc),
        (0x20, 0x0027_caf0, O406Store::Float, 0x1c, 5, 0x0027_cb04),
        (0x20, 0x0027_cb18, O406Store::Float, 0x20, 6, 0x0027_cb2c),
        (0x20, 0x0027_cb40, O406Store::Float, 0x24, 7, 0x0027_cb54),
        (0x20, 0x0027_cb68, O406Store::Float, 0x28, 8, 0x0027_cb7c),
        (0x20, 0x0027_cb90, O406Store::Float, 0x2c, 0, 0),
    ];
    guest(c, |c| {
        c.cycles += 127;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0027_42c8, 0x0027_c9d0)?;
        for (n, &(voff, vret, store, dst, next, next_ret)) in STEPS.iter().enumerate() {
            // The virtual read call on the track in v0.
            c.g[V1][0] = o406_lw(c, V0, 4);
            let a = (c.g[V1][0] as u32).wrapping_add(voff);
            c.g[A0][0] = c.lh(a);
            c.g[A1][0] = o406_lw(c, V1, voff + 4);
            let target = c.g[A1][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, vret)?;
            let last = n == STEPS.len() - 1;
            match store {
                O406Store::Deref => {
                    c.g[V1][0] = o406_lw(c, V0, 0);
                    c.g[A1][0] = next as u64;
                    c.g[A0][0] = c.g[S0][0];
                    o406_sw(c, S1, dst, V1);
                }
                O406Store::Direct => {
                    c.g[A1][0] = next as u64;
                    c.g[A0][0] = c.g[S0][0];
                    o406_sw(c, S1, dst, V0);
                }
                O406Store::Float => {
                    c.lwc1(0, c.g[V0][0] as u32);
                    if last {
                        frame.restore(c, S0, 0x20);
                    } else {
                        c.g[A1][0] = next as u64;
                        c.g[A0][0] = c.g[S0][0];
                    }
                    let a = (c.g[S1][0] as u32).wrapping_add(dst);
                    c.swc1(0, a);
                }
            }
            if last {
                break;
            }
            call_ptr(c, 0x0027_42c8, next_ret)?;
        }
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------- InSeq_InstanceTick

/// The two per-channel loops of `InSeq_InstanceTick` that share one shape:
/// `flush` true calls `InSeq_ChannelFlushEvents` on every channel (counter
/// s1, previous result s5, 0x274fc0), false calls `InSeq_ChannelRestart`
/// (counter s4, previous result s1, 0x275078). A channel reports "progress"
/// in s2 when its call returns non-zero or the previous one did.
fn o406_inseq_channel_loop(c: &mut Cpu, flush: bool) -> Flow {
    // `flush`: the FlushEvents loop (counter s1, offset s3, previous s5, flag
    // s2) at 0x274fc0; otherwise the Restart loop (counter s4, offset s3,
    // previous s1, flag s2) at 0x275078.
    let (counter, prev, call, ret, head) = if flush {
        (S1, S5, 0x0027_3d20u32, 0x0027_4f90u32, 0x0027_4f80u32)
    } else {
        (S4, S1, 0x0027_3f68u32, 0x0027_5048u32, 0x0027_5038u32)
    };
    if flush {
        c.cycles += 4;
    } else {
        c.cycles += 5;
    }
    c.g[V1][0] = o406_lw(c, S0, 0x10);
    let a = (c.g[V1][0] as u32).wrapping_add(1);
    c.g[V0][0] = c.lbu(a);
    let none = c.g[V0][0] == 0;
    if !flush {
        c.g[S4][0] = 0;
    }
    c.g[S1][0] = 0;
    if none {
        return Ok(());
    }
    c.cycles += if flush { 1 } else { 2 };
    c.g[S3][0] = 0;
    loop {
        c.cycles += 6;
        c.g[A0][0] = o406_lw(c, S0, 0x18);
        c.g[S2][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
        call_ptr(c, call, ret)?;
        if c.g[V0][0] != 0 {
            c.g[S2][0] = 1;
            c.cycles += 1;
            c.g[V1][0] = o406_lw(c, S0, 0x10);
        } else {
            c.cycles += 2;
            if c.g[prev][0] == 0 {
                c.g[V1][0] = o406_lw(c, S0, 0x10);
            } else {
                c.cycles += 1;
                c.g[S2][0] = 1;
                c.cycles += 1;
                c.g[V1][0] = o406_lw(c, S0, 0x10);
            }
        }
        c.cycles += 6;
        c.g[counter][0] = sx((c.g[counter][0] as u32).wrapping_add(1));
        c.g[prev][0] = c.g[S2][0];
        let a = (c.g[V1][0] as u32).wrapping_add(1);
        c.g[V0][0] = c.lbu(a);
        c.g[V0][0] = ((c.g[counter][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let again = c.g[V0][0] != 0;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3c));
        if !again {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// Call `listener->vtable[off]` (`lh +off` this adjust, `lw +off+4`
/// function) with `a1 = inst` (the instance), through the listener `s4`:
/// the notification the tick sends on a state change.
fn o406_inseq_notify(c: &mut Cpu, voff: u32, ret: u32) -> Flow {
    c.g[V0][0] = o406_lw(c, S4, 0);
    c.g[A1][0] = c.g[S0][0];
    let a = (c.g[V0][0] as u32).wrapping_add(voff);
    c.g[A0][0] = c.lh(a);
    c.g[V1][0] = o406_lw(c, V0, voff + 4);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `bool InSeq_InstanceTick(InSeqInstance *self, ...)` (0x274de0): advance a
/// playing sequence instance (state at +0x20: 0 idle, 1 starting, 2
/// playing, 3 stopping, 4 finished). Tells the listener about the start,
/// runs `InstanceAdvance` over the time left (state 3 and 4 clamp to the
/// length), starts / flushes / restarts the channels, loops the time and
/// finally stops the instance or reports the end to the listener.
pub fn in_seq_instance_tick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        c.g[V0][0] = o406_lw(c, S0, 0x20);
        'exit: {
            if c.g[V0][0] == 0 {
                break 'exit;
            }
            c.cycles += 3;
            c.g[V0][0] = o406_lw(c, S0, 0x2c);
            if (c.g[V0][0] as i64) > 0 {
                break 'exit;
            }
            c.cycles += 7;
            call_ptr(c, 0x0027_4d70, 0x0027_4e30)?;
            c.g[S4][0] = c.g[V0][0];
            c.g[V1][0] = o406_lw(c, S0, 0x20);
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = o406_lw(c, S0, 0x2c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                o406_sw(c, S0, 0x20, V0);
                if c.g[S4][0] != 0 {
                    c.cycles += 6;
                    o406_inseq_notify(c, 8, 0x0027_4e68)?;
                }
                c.cycles += 1;
                c.g[V0][0] = o406_lw(c, S0, 0x2c);
            }
            c.cycles += 2;
            if (c.g[V0][0] as i64) > 0 {
                break 'exit;
            }
            c.cycles += 3;
            c.g[V0][0] = o406_lw(c, S0, 0x20);
            if c.g[V0][0] == 0 {
                break 'exit;
            }
            'stop: {
                'to_j: {
                    'g: {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[S5][0] = 0;
                        call_ptr(c, 0x0027_4c70, 0x0027_4e8c)?;
                        c.mtc1(V0, 20);
                        c.cvt_s_w(20, 20);
                        c.g[V1][0] = o406_lw(c, S0, 0x20);
                        c.mtc1(0, 21);
                        c.g[V0][0] = 4;
                        c.g[S6][0] = 0;
                        if c.g[V1][0] == 4 {
                            c.cycles += 4;
                            let a = (c.g[S0][0] as u32).wrapping_add(0x28);
                            c.lwc1(0, a);
                            c.c_le_s(20, 0);
                            if c.fcc() {
                                break 'stop;
                            }
                        }
                        break 'g;
                    }
                    'finish: {
                        'check: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(3));
                            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                            let other_state = c.g[V0][0] == 0;
                            c.g[V0][0] = 4;
                            if !other_state {
                                // State 3 or 4: advance to the end time.
                                c.cycles += 5;
                                let a = (c.g[S0][0] as u32).wrapping_add(0x24);
                                c.lwc1(14, a);
                                let a = (c.g[S0][0] as u32).wrapping_add(0x28);
                                c.lwc1(13, a);
                                c.mov_s(0, 14);
                                let not_four = c.g[V1][0] != c.g[V0][0];
                                c.add_s(12, 13, 14);
                                if !not_four {
                                    c.cycles += 1;
                                    c.mov_s(12, 20);
                                }
                                c.cycles += 4;
                                c.c_le_s(20, 12);
                                let within = !c.fcc();
                                c.g[V0][0] = 1;
                                if !within {
                                    c.cycles += 4;
                                    c.sub_s(14, 12, 13);
                                    c.g[S6][0] = 1;
                                    c.mov_s(12, 20);
                                    c.sub_s(21, 0, 14);
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S0][0];
                                o406_sw(c, S0, 0x3c, V0);
                                call_ptr(c, 0x0027_51a0, 0x0027_4f0c)?;
                                o406_sw(c, S0, 0x3c, ZERO);
                                break 'finish;
                            }
                            break 'check;
                        }
                        // 0x274f28: any other state starts / flushes the channels.
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_eq_s(20, 0);
                        if c.fcc() {
                            c.g[S6][0] = 1;
                        }
                        c.cycles += 6;
                        c.g[V0][0] = 1;
                        c.g[V1][0] = o406_lw(c, S0, 0x10);
                        o406_sw(c, S0, 0x3c, V0);
                        let a = (c.g[V1][0] as u32).wrapping_add(1);
                        c.g[V0][0] = c.lbu(a);
                        let none = c.g[V0][0] == 0;
                        c.g[S1][0] = 0;
                        if !none {
                            c.cycles += 2;
                            c.g[S2][0] = 0;
                            loop {
                                c.cycles += 9;
                                c.g[A0][0] = o406_lw(c, S0, 0x18);
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                call_ptr(c, 0x0027_3aa8, 0x0027_4f58)?;
                                c.g[V1][0] = o406_lw(c, S0, 0x10);
                                let a = (c.g[V1][0] as u32).wrapping_add(1);
                                c.g[V0][0] = c.lbu(a);
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let again = c.g[V0][0] != 0;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3c));
                                if !again {
                                    break;
                                }
                                if event_due(c) {
                                    deliver_event_here(c, 0x0027_4f48);
                                }
                            }
                        }
                        o406_inseq_channel_loop(c, true)?;
                        c.cycles += 4;
                        c.g[V0][0] = 3;
                        o406_sw(c, S0, 0x3c, ZERO);
                        let no_listener = c.g[S4][0] == 0;
                        o406_sw(c, S0, 0x20, V0);
                        if !no_listener {
                            c.cycles += 8;
                            c.g[V1][0] = o406_lw(c, S4, 0);
                            c.g[A1][0] = c.g[S0][0];
                            c.mtc1(0, 12);
                            let a = (c.g[V1][0] as u32).wrapping_add(0x10);
                            c.g[A0][0] = c.lh(a);
                            c.g[V0][0] = o406_lw(c, V1, 0x14);
                            c.mov_s(13, 12);
                            let target = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x0027_4ff0)?;
                        }
                    }
                    // 0x274ff0.
                    c.cycles += 3;
                    c.g[V0][0] = o406_lw(c, S0, 0x40);
                    let has_flag = c.g[V0][0] != 0;
                    c.g[V0][0] = 4;
                    if has_flag {
                        break 'stop;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = o406_lw(c, S0, 0x20);
                    if c.g[V1][0] == c.g[V0][0] {
                        break 'stop;
                    }
                    c.cycles += 2;
                    if c.g[S6][0] == 0 {
                        let a = (c.g[S0][0] as u32).wrapping_add(0x28);
                        c.lwc1(0, a);
                        break 'to_j;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = o406_lw(c, S0, 0x1c);
                    if c.g[V0][0] == 0 {
                        let a = (c.g[S0][0] as u32).wrapping_add(0x28);
                        c.lwc1(0, a);
                        break 'to_j;
                    }
                    // Loop the sequence: restart the channels and wrap the time.
                    o406_inseq_channel_loop(c, false)?;
                    c.cycles += 4;
                    c.c_lt_s(20, 21);
                    let skip = !c.fcc();
                    o406_sw(c, S0, 0x28, ZERO);
                    if !skip {
                        c.cycles += 2;
                        c.sub_s(21, 21, 20);
                        loop {
                            c.cycles += 5;
                            c.c_lt_s(20, 21);
                            if !c.fcc() {
                                break;
                            }
                            c.sub_s(21, 21, 20);
                            if event_due(c) {
                                deliver_event_here(c, 0x0027_5090);
                            }
                        }
                    }
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_eq_s(21, 0);
                    if c.fcc() {
                        c.g[V0][0] = o406_lw(c, S0, 0x40);
                    } else {
                        c.cycles += 9;
                        let a = (c.g[S0][0] as u32).wrapping_add(0x28);
                        c.lwc1(13, a);
                        c.g[V0][0] = 1;
                        o406_sw(c, S0, 0x3c, V0);
                        c.mov_s(14, 21);
                        c.add_s(12, 13, 21);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0027_51a0, 0x0027_50d4)?;
                        o406_sw(c, S0, 0x3c, ZERO);
                        c.g[V0][0] = o406_lw(c, S0, 0x40);
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        break 'exit;
                    }
                    break 'stop;
                }
                // 0x2750f4: the time is within the length: end, or keep going.
                c.cycles += 6;
                let a = (c.g[S0][0] as u32).wrapping_add(0x24);
                c.lwc1(1, a);
                c.add_s(0, 0, 1);
                c.c_le_s(20, 0);
                if !c.fcc() {
                    break 'exit;
                }
                c.cycles += 3;
                c.g[V1][0] = o406_lw(c, S0, 0x1c);
                let looping = c.g[V1][0] == 0;
                c.g[V0][0] = 4;
                if !looping {
                    c.cycles += 2;
                    let none = c.g[S4][0] == 0;
                    c.g[A1][0] = c.g[S0][0];
                    if !none {
                        c.cycles += 6;
                        c.g[V1][0] = o406_lw(c, S4, 0);
                        let a = (c.g[V1][0] as u32).wrapping_add(0x28);
                        c.g[A0][0] = c.lh(a);
                        c.g[V0][0] = o406_lw(c, V1, 0x2c);
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0027_5134)?;
                        c.g[V1][0] = c.g[V0][0];
                    }
                    c.cycles += 3;
                    c.g[V0][0] = o406_lw(c, S0, 0x20);
                    if c.g[V0][0] == 0 {
                        break 'exit;
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != 0 {
                        break 'exit;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 4;
                }
                // 0x275150: the sequence has ended: state 4 and tell the listener.
                c.cycles += 2;
                let none = c.g[S4][0] == 0;
                o406_sw(c, S0, 0x20, V0);
                if !none {
                    c.cycles += 6;
                    o406_inseq_notify(c, 0x18, 0x0027_5170)?;
                }
                c.cycles += 1;
                break 'exit;
            }
            // 0x2750e4: stop the instance.
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0027_4918, 0x0027_50ec)?;
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------- RiderRailslideSub_CheckAirborne

/// `int RiderRailslideSub_CheckAirborne(RailslideSub *self, State *s)`
/// (0x132770): decide whether the rail-slide state ends. Returns 0 while
/// the rider (`self+8`) is not in state 1, or is on animation 0xe with the
/// 0xb0 flag clear; otherwise tells the rider (0x11fec8) to leave the rail
/// (mode 5 when the turn input `s+4` (6-bit signed) is non-zero, else mode
/// 4) and returns 1.
pub fn rider_railslide_sub_check_airborne(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = o406_lw(c, S0, 8);
        call_ptr(c, 0x0011_fe98, 0x0013_2790)?;
        c.g[V1][0] = 1;
        let not_riding = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        'body: {
            if not_riding {
                break 'body;
            }
            c.cycles += 7;
            c.g[V0][0] = o406_lw(c, S0, 8);
            c.g[A1][0] = 2;
            c.g[A0][0] = o406_lw(c, V0, 0x784);
            call_ptr(c, 0x0031_1ae8, 0x0013_27ac)?;
            c.g[V1][0] = 0xe;
            let other_anim = c.g[V0][0] != c.g[V1][0];
            c.g[A1][0] = 2;
            if !other_anim {
                c.cycles += 8;
                c.g[V0][0] = o406_lw(c, S0, 8);
                c.g[A0][0] = o406_lw(c, V0, 0x784);
                call_ptr(c, 0x0031_1b20, 0x0013_27c4)?;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_46a0, 0x0013_27d0)?;
                let clear = c.g[V0][0] == 0;
                c.g[V0][0] = 0;
                if clear {
                    break 'body;
                }
            }
            c.cycles += 14;
            c.g[A0][0] = o406_lw(c, S0, 8);
            call_ptr(c, 0x0011_5640, 0x0013_27e0)?;
            c.g[V0][0] = o406_lw(c, S1, 4);
            let a = (c.g[GP][0] as u32).wrapping_sub(0x7570);
            c.lwc1(2, a);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 26);
            c.mtc1(0, 1);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(0, 0, 2);
            c.c_eq_s(0, 1);
            let straight = c.fcc();
            c.g[A0][0] = o406_lw(c, S0, 8);
            if !straight {
                c.cycles += 4;
                c.g[A1][0] = 5;
                call_ptr(c, 0x0011_fec8, 0x0013_2818)?;
                c.g[V0][0] = 1;
                break 'body;
            }
            c.cycles += 3;
            c.g[A1][0] = 4;
            call_ptr(c, 0x0011_fec8, 0x0013_2828)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------------------- cPS2ChunkResolve_v19

/// `int cPS2ChunkResolve_v19(Resolver *self, void *base, Chunk *chunk)`
/// (0x3aaa08): resolve a chunk's internal offsets into pointers. The three
/// offsets at +4, +8 and +0xc are rebased; then chunk type 3 (a table of
/// 0x140-byte records at `+8`, count `+2`: each one's own resolver at
/// 0x32c540, then 0x327da8 over the data at `+4`) or type 1 (a table of
/// variable-size records, each with four rebased offsets) is walked.
/// Returns 1.
pub fn c_ps2_chunk_resolve_v19(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x50);
        frame.save(c, S4, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = o406_lw(c, S0, 4);
        c.g[S4][0] = c.g[A1][0];
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            o406_sw(c, S0, 4, V0);
        } else {
            o406_sw(c, S0, 4, ZERO);
        }
        for off in [8u32, 0xc] {
            c.cycles += 4;
            c.g[V1][0] = o406_lw(c, S0, off);
            c.g[V0][0] = u64::MAX;
            let fixed = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            if fixed {
                c.cycles += 1;
                o406_sw(c, S0, off, V0);
            } else {
                c.cycles += 2;
                o406_sw(c, S0, off, ZERO);
            }
        }
        'done: {
            'tail: {
                'not_three: {
                    c.cycles += 4;
                    let a = c.g[S0][0] as u32;
                    c.g[V1][0] = c.lhu(a);
                    c.g[V0][0] = 3;
                    let not_three = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 1;
                    if not_three {
                        break 'not_three;
                    }
                    c.cycles += 9;
                    c.g[S1][0] = o406_lw(c, S0, 8);
                    c.g[S2][0] = 0;
                    o406_sw(c, S1, 4, ZERO);
                    o406_sw(c, S1, 0, ZERO);
                    let a = (c.g[S0][0] as u32).wrapping_add(2);
                    c.g[V1][0] = c.lhu(a);
                    o406_sw(c, S1, 4, V1);
                    c.g[V0][0] = o406_lw(c, S0, 0xc);
                    let empty = c.g[V1][0] == 0;
                    o406_sw(c, S1, 0, V0);
                    if !empty {
                        c.cycles += 1;
                        c.g[S3][0] = 0;
                        loop {
                            c.cycles += 8;
                            c.g[A0][0] = o406_lw(c, S1, 0);
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                            call_ptr(c, 0x0032_c540, 0x003a_aac8)?;
                            c.g[V0][0] = o406_lw(c, S1, 4);
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let again = c.g[V0][0] != 0;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x140));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x003a_aab8);
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V0][0] = o406_lw(c, S1, 4);
                    c.g[S2][0] = 0;
                    let none = (c.g[V0][0] as i64) <= 0;
                    c.g[S0][0] = o406_lw(c, S0, 4);
                    if none {
                        break 'tail;
                    }
                    c.cycles += 2;
                    c.g[S3][0] = 0;
                    loop {
                        c.cycles += 11;
                        c.g[A0][0] = o406_lw(c, S1, 0);
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S4][0];
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                        call_ptr(c, 0x0032_7da8, 0x003a_ab08)?;
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x140));
                        c.g[V1][0] = o406_lw(c, S1, 4);
                        c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        let again = c.g[V1][0] != 0;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        if !again {
                            c.cycles += 2;
                            break 'done;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_aaf0);
                        }
                    }
                }
                // Not type 3.
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    break 'done;
                }
                c.cycles += 10;
                c.g[T0][0] = o406_lw(c, S0, 8);
                c.g[A3][0] = 0;
                o406_sw(c, T0, 4, ZERO);
                o406_sw(c, T0, 0, ZERO);
                let a = (c.g[S0][0] as u32).wrapping_add(2);
                c.g[V1][0] = c.lhu(a);
                o406_sw(c, T0, 4, V1);
                c.g[V0][0] = o406_lw(c, S0, 0xc);
                o406_sw(c, T0, 0, V0);
                let empty = c.g[V1][0] == 0;
                c.g[A2][0] = o406_lw(c, S0, 4);
                if empty {
                    break 'tail;
                }
                c.cycles += 1;
                loop {
                    c.cycles += 23;
                    c.g[V1][0] = o406_lw(c, T0, 0);
                    c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    o406_sw(c, V0, 0, A2);
                    c.g[V1][0] = o406_lw(c, A2, 4);
                    c.g[V0][0] = o406_lw(c, A2, 8);
                    c.g[A0][0] = o406_lw(c, A2, 0xc);
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A1][0] = o406_lw(c, A2, 0x10);
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    o406_sw(c, A2, 8, V0);
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    o406_sw(c, A2, 4, V1);
                    o406_sw(c, A2, 0xc, A0);
                    o406_sw(c, A2, 0x10, A1);
                    let a = c.g[A2][0] as u32;
                    c.g[V0][0] = c.lhu(a);
                    c.g[V1][0] = o406_lw(c, T0, 4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                    c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let again = c.g[V1][0] != 0;
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if !again {
                        break 'tail;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_ab58);
                    }
                }
            }
            c.cycles += 1;
        }
        c.cycles += 8;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------- RiderModel_LoadVariant

/// `void RiderModel_LoadVariant(RiderModel *self, int lod, ModelPart *part,
/// int index)` (0x30d8b8): register a loaded model part under its id
/// (`part+0x52`). If the part table (count +8, 0x58-byte entries at +0xc)
/// has the id already, the entry's LOD record gets `used = 1` and the part
/// and its data (`part+0x60`) filed at `index`. Otherwise a new entry is
/// appended: four LOD ref tables are allocated and cleared, then the same
/// fields plus the bone names, the two other data offsets, the bone count
/// and the two counters are filled in.
pub fn rider_model_load_variant(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S2, 0x80);
        frame.save(c, S4, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S7, 0x30);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, FP, 0x20);
        c.g[S7][0] = sx((c.g[S4][0] as u32).wrapping_add(0x60));
        frame.save(c, S0, 0xa0);
        c.g[FP][0] = c.g[A3][0];
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = o406_lw(c, S2, 8);
        let no_parts = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = 0;
        'exit: {
            if !no_parts {
                c.cycles += 4;
                c.g[T0][0] = sx((c.g[A1][0] as u32) << 3);
                c.g[T2][0] = 1;
                c.g[T1][0] = sx((c.g[FP][0] as u32) << 4);
                c.g[A3][0] = 0;
                loop {
                    c.cycles += 6;
                    c.g[V0][0] = o406_lw(c, S2, 0xc);
                    let a = (c.g[S4][0] as u32).wrapping_add(0x52);
                    c.g[A0][0] = c.lb(a);
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = o406_lw(c, V0, 0);
                    if c.g[V1][0] == c.g[A0][0] {
                        // The part is known: file this lod's ref in it.
                        c.cycles += 15;
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        o406_sw(c, V1, 0x18, T2);
                        c.g[V0][0] = o406_lw(c, S2, 0xc);
                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V1][0] = o406_lw(c, V0, 0x1c);
                        c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        o406_sw(c, V1, 0, S4);
                        c.g[V0][0] = o406_lw(c, S2, 0xc);
                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V1][0] = o406_lw(c, V0, 0x1c);
                        c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        o406_sw(c, V1, 4, S7);
                        break 'exit;
                    }
                    c.g[V0][0] = o406_lw(c, S2, 8);
                    c.cycles += 4;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x58));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_d910);
                    }
                }
            }
            // A new part entry.
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[FP][0] as u32) << 4);
            c.g[S6][0] = sx((c.g[A1][0] as u32) << 3);
            let a = frame.at(c, 0);
            c.sw(a, c.g[V0][0]);
            c.g[S3][0] = 0;
            c.g[S5][0] = 0x58;
            c.g[S0][0] = o406_lw(c, S2, 8);
            loop {
                c.cycles += 28;
                c.g[S1][0] = sx((c.g[S3][0] as u32) << 3);
                c.g[V0][0] = o406_lw(c, S2, 0xc);
                c.g[V1][0] = 0x0049_0000;
                c.mult(0, S0, S0, S5);
                c.g[A0][0] = o406_lw(c, S2, 4);
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6380));
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                call_ptr(c, 0x0031_7e30, 0x0030_d9c8)?;
                o406_sw(c, S0, 0, V0);
                c.g[A1][0] = 0;
                c.g[V0][0] = o406_lw(c, S2, 8);
                c.g[V1][0] = o406_lw(c, S2, 0xc);
                c.mult(0, V0, V0, S5);
                c.g[A2][0] = o406_lw(c, S2, 4);
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[A0][0] = o406_lw(c, V0, 0x1c);
                call_ptr(c, 0x003e_6448, 0x0030_d9f4)?;
                c.g[V0][0] = ((c.g[S3][0] as i64) < 4) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[S0][0] = o406_lw(c, S2, 8);
                if event_due(c) {
                    deliver_event_here(c, 0x0030_d990);
                }
            }
            c.cycles += 79;
            c.g[V1][0] = o406_lw(c, S2, 8);
            c.g[A1][0] = 0x58;
            c.g[V0][0] = o406_lw(c, S2, 0xc);
            c.g[A2][0] = 1;
            c.mult(0, V1, V1, A1);
            let a = (c.g[S4][0] as u32).wrapping_add(0x52);
            c.g[A0][0] = c.lb(a);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            o406_sw(c, V1, 0, A0);
            c.g[V0][0] = o406_lw(c, S2, 8);
            c.g[V1][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V0, V0, A1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
            o406_sw(c, V0, 0x18, A2);
            c.g[V1][0] = o406_lw(c, S2, 8);
            c.g[V0][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V1, V1, A1);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S6][0] as u32));
            c.g[V0][0] = c.lw(frame.at(c, 0));
            c.g[A0][0] = o406_lw(c, V1, 0x1c);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            o406_sw(c, A0, 0, S4);
            c.g[V0][0] = o406_lw(c, S2, 8);
            c.g[V1][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V0, V0, A1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
            c.g[V1][0] = o406_lw(c, V0, 0x1c);
            c.g[V0][0] = c.lw(frame.at(c, 0));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            o406_sw(c, V1, 4, S7);
            c.g[A0][0] = o406_lw(c, S2, 8);
            c.g[V1][0] = o406_lw(c, S2, 0xc);
            c.mult(0, A0, A0, A1);
            c.g[V0][0] = o406_lw(c, S4, 0x18);
            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            o406_sw(c, A0, 0x38, V0);
            c.g[V1][0] = o406_lw(c, S2, 8);
            c.g[A0][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V1, V1, A1);
            c.g[V0][0] = o406_lw(c, S4, 0x2c);
            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            o406_sw(c, V1, 0x40, V0);
            c.g[A0][0] = o406_lw(c, S2, 8);
            c.g[V1][0] = o406_lw(c, S2, 0xc);
            c.mult(0, A0, A0, A1);
            c.g[V0][0] = o406_lw(c, S4, 0x1c);
            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            o406_sw(c, A0, 0x3c, V0);
            c.g[V1][0] = o406_lw(c, S2, 8);
            c.g[V0][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V1, V1, A1);
            let a = (c.g[S4][0] as u32).wrapping_add(0x4a);
            c.g[A0][0] = c.lh(a);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            o406_sw(c, V1, 0x44, A0);
            c.g[V0][0] = o406_lw(c, S2, 8);
            c.g[A0][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V0, V0, A1);
            let a = (c.g[S4][0] as u32).wrapping_add(0x4e);
            c.g[V1][0] = c.lh(a);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            o406_sw(c, V0, 0x48, V1);
            c.g[A0][0] = o406_lw(c, S2, 8);
            c.g[V1][0] = o406_lw(c, S2, 0xc);
            c.mult(0, A0, A0, A1);
            let a = (c.g[S4][0] as u32).wrapping_add(0x50);
            c.g[V0][0] = c.lh(a);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            o406_sw(c, A0, 0x4c, V0);
            c.g[V1][0] = o406_lw(c, S2, 8);
            c.g[V0][0] = o406_lw(c, S2, 0xc);
            c.mult(0, V1, V1, A1);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            o406_sw(c, V1, 0x50, FP);
            c.g[V0][0] = o406_lw(c, S2, 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            o406_sw(c, S2, 8, V0);
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------- Cell_CollideColumnRecursive

/// Recurse into one octree child: `Cell_CollideColumnRecursive(a0 = s6,
/// query s4, inside query s2, ctx s5, node = t0, cell = sp)`.
fn o406_collide_child(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = c.g[S6][0];
    c.g[A1][0] = c.g[S4][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.g[S5][0];
    c.g[T1][0] = c.g[SP][0];
    call_ptr(c, 0x0033_dbe8, ret)
}

/// `void Cell_CollideColumnRecursive(int flag, Query *q, Query *inside,
/// Ctx *ctx, OctNode *node, Cell *cell)` (0x33dbe8): walk the collision
/// octree against the query box. The cell (level +0, x +4, y +8, z +0xc)
/// is classified against the ctx box (float compares of the cell's
/// min/max); a cell the box only partly covers (class 2) collides the node's
/// two object lists (`Cell_CollideInstance`, `Cell_CollideObject`) and, unless
/// at the finest level 11, recurses into the eight children with the
/// sub-cell (level - 1, coordinates doubled and offset per child).
///
/// The generated classification only ever yields 1 (outside) or 2
/// (partial): the original's "fully inside" branch (class 0, which would
/// collide the subtree with `Cell_CollideVolumeSubtree`) is unreachable and
/// not ported.
pub fn cell_collide_column_recursive(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x90);
        let a = (c.g[GP][0] as u32).wrapping_sub(0x2ce0);
        c.lwc1(3, a);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[T1][0];
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S0, 0x80);
        c.g[S6][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V1][0] = o406_lw(c, S3, 4);
        c.g[V0][0] = o406_lw(c, S3, 0);
        c.mtc1(V1, 0);
        c.cvt_s_w(0, 0);
        c.g[A0][0] = o406_lw(c, S5, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7f));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
        let a = c.g[A0][0] as u32;
        c.lwc1(2, a);
        c.sub_s(0, 0, 3);
        c.mtc1(V0, 1);
        c.mul_s(0, 0, 1);
        c.c_lt_s(2, 0);
        let mut outside = c.fcc();
        c.g[S4][0] = c.g[A1][0];
        if !outside {
            c.cycles += 9;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.mtc1(V0, 0);
            c.cvt_s_w(0, 0);
            c.add_s(0, 0, 3);
            c.mul_s(0, 0, 1);
            c.c_lt_s(0, 2);
            let t = c.fcc();
            c.g[V1][0] = 1;
            if !t {
                c.cycles += 10;
                c.g[V0][0] = o406_lw(c, S3, 8);
                let a = (c.g[A0][0] as u32).wrapping_add(4);
                c.lwc1(2, a);
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.sub_s(0, 0, 3);
                c.mul_s(0, 0, 1);
                c.c_lt_s(2, 0);
                let t = c.fcc();
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                if !t {
                    c.cycles += 8;
                    c.mtc1(V0, 0);
                    c.cvt_s_w(0, 0);
                    c.add_s(0, 0, 3);
                    c.mul_s(0, 0, 1);
                    c.c_lt_s(0, 2);
                    let partial = !c.fcc();
                    c.g[V1][0] = 2;
                    outside = !partial;
                }
            }
        }
        if outside {
            c.cycles += 1;
            c.g[V1][0] = 1;
        }
        'exit: {
            // `v1 != 0` always holds here (see the doc comment).
            c.cycles += 2;
            c.g[V0][0] = 2;
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                break 'exit;
            }
            // Partly covered: the node's two object lists.
            c.cycles += 3;
            c.g[S0][0] = o406_lw(c, S1, 0x24);
            if c.g[S0][0] == 0 {
                c.g[S0][0] = o406_lw(c, S1, 0x20);
            } else {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x0033_5960, 0x0033_ddf0)?;
                    c.g[S0][0] = o406_lw(c, S0, 0);
                    let more = c.g[S0][0] != 0;
                    c.g[A1][0] = c.g[S0][0];
                    if !more {
                        c.cycles += 1;
                        c.g[S0][0] = o406_lw(c, S1, 0x20);
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0033_dde8);
                    }
                }
            }
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                c.g[A1][0] = o406_lw(c, S3, 0);
            } else {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x0033_5b90, 0x0033_de18)?;
                    c.g[S0][0] = o406_lw(c, S0, 0);
                    let more = c.g[S0][0] != 0;
                    c.g[A1][0] = c.g[S0][0];
                    if !more {
                        c.cycles += 1;
                        c.g[A1][0] = o406_lw(c, S3, 0);
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0033_de10);
                    }
                }
            }
            // The cell's level: nothing below 11.
            c.cycles += 3;
            c.g[V0][0] = 0xb;
            let finest = c.g[A1][0] == c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            if finest {
                c.cycles += 1;
                break 'exit;
            }
            // The sub-cell on the stack: level - 1, x, y, z doubled.
            c.cycles += 12;
            c.g[A0][0] = o406_lw(c, S3, 0xc);
            c.g[V0][0] = o406_lw(c, S3, 4);
            c.g[V1][0] = o406_lw(c, S3, 8);
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[T0][0] = o406_lw(c, S1, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
            o406_sw(c, SP, 0, A1);
            o406_sw(c, SP, 4, V0);
            o406_sw(c, SP, 8, V1);
            o406_sw(c, SP, 0xc, A0);
            if c.g[T0][0] != 0 {
                o406_collide_child(c, 0x0033_de7c)?;
            }
            // (child offset, sub-cell word, delta, return address)
            const CHILDREN: [(u32, u32, u32, u32); 7] = [
                (0x10, 4, 1, 0x0033_dea8),
                (0x18, 8, 1, 0x0033_ded4),
                (0x8, 4, u32::MAX, 0x0033_df00),
                (0xc, 0xc, 1, 0x0033_df2c),
                (0x1c, 4, 1, 0x0033_df58),
                (0x14, 8, u32::MAX, 0x0033_df84),
                (0x4, 4, u32::MAX, 0x0033_dfb0),
            ];
            for (child, word, delta, ret) in CHILDREN {
                c.cycles += 5;
                c.g[V0][0] = o406_lw(c, SP, word);
                c.g[T0][0] = o406_lw(c, S1, child);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(delta));
                o406_sw(c, SP, word, V0);
                if c.g[T0][0] != 0 {
                    o406_collide_child(c, ret)?;
                }
            }
            c.cycles += 1;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{elf_image, put_vec, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    // ------------------------------------------------------------ fixtures

    /// Gate of the callee fakes (a magic word in the machine's RAM). Above
    /// the fakes' call log (`DATA + 0x10_0000`).
    pub(crate) const O406_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const O406_MAGIC: u32 = 0x4f34_3036;
    /// Index of the next 64-bit fake result, and the results (24).
    pub(crate) const O406_RESULT_IDX: u32 = O406_CTRL + 0x10;
    pub(crate) const O406_RESULTS: u32 = O406_CTRL + 0x20;
    /// Index of the next token kind a `NextToken` fake delivers, and the kinds (32).
    pub(crate) const O406_KIND_IDX: u32 = O406_CTRL + 0x14;
    pub(crate) const O406_KINDS: u32 = O406_CTRL + 0x104;
    /// Index of the next character a `NextChar` fake delivers, and the characters (64).
    pub(crate) const O406_CHAR_IDX: u32 = O406_CTRL + 0x18;
    pub(crate) const O406_CHARS: u32 = O406_CTRL + 0x204;
    /// The InputParser, the expression node pool, the bolt database.
    pub(crate) const O406_PARSER: u32 = DATA + 0x4_0000;
    pub(crate) const O406_NODES: u32 = DATA + 0x5_0000;
    pub(crate) const O406_BOLT_DB: u32 = DATA + 0x1000;
    /// A sequence track (the `GetTrack` fake's result).
    pub(crate) const O406_TRACK: u32 = DATA + 0x400;

    fn o406_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    fn o406_pop64(c: &mut Cpu) -> u64 {
        let i = c.bus.read32(O406_RESULT_IDX);
        c.bus.write32(O406_RESULT_IDX, i + 1);
        if i >= 24 {
            0
        } else {
            c.bus.read64(O406_RESULTS + 8 * i)
        }
    }

    fn o406_pop_kind(c: &mut Cpu) -> u32 {
        let i = c.bus.read32(O406_KIND_IDX);
        c.bus.write32(O406_KIND_IDX, i + 1);
        if i >= 32 {
            0
        } else {
            c.bus.read32(O406_KINDS + 4 * i)
        }
    }

    fn o406_pop_char(c: &mut Cpu) -> u32 {
        let i = c.bus.read32(O406_CHAR_IDX);
        c.bus.write32(O406_CHAR_IDX, i + 1);
        if i >= 64 {
            u32::MAX
        } else {
            c.bus.read32(O406_CHARS + 4 * i)
        }
    }

    /// A gated callee fake: logs the call like `fakes::fake` and, for the
    /// callees whose results the parsers and sequences act on, delivers the
    /// next queued value (the setup of the test fills the queues).
    fn o406_fake_call(c: &mut Cpu, addr: u32, ra: u32) {
        let a0 = c.g[A0][0] as u32;
        let a2 = c.g[A2][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match addr {
            // ParseComparison, Lookup*/FindIdentifier, FindBoltIndex,
            // SetCharProfileBoltEquipped, ChannelGetLength/Listener/Flush, the
            // rider state, an animation id, a flag test.
            0x0032_36d8 | 0x0032_16e8 | 0x0032_1880 | 0x0032_1a40 | 0x0014_d448 | 0x0015_1c90
            | 0x0027_4c70 | 0x0027_4d70 | 0x0027_3d20 | 0x0011_fe98 | 0x0031_1ae8 | 0x0014_46a0 => {
                c.g[V0][0] = o406_pop64(c);
            }
            // NextToken: the parser's token kind.
            0x0032_1af0 => {
                let kind = o406_pop_kind(c);
                c.bus.write32(a0 + 0x98, kind);
            }
            // EmitNode: a word count and the register it chose.
            0x0032_4678 => {
                c.g[V0][0] = o406_pop64(c);
                let reg = o406_pop64(c) as u32;
                c.bus.write32(a2, reg);
            }
            // NextChar: the preprocessor's lookahead character.
            0x0032_6478 => {
                let ch = o406_pop_char(c);
                c.bus.write32(a0 + 0xc, ch);
            }
            // EventTrack_NextKey: the next key.
            0x0027_3468 => {
                let key = o406_pop64(c) as u32;
                c.bus.write32(a0 + 0x14, key);
            }
            0x0014_bdb8 => c.g[V0][0] = O406_BOLT_DB as u64,
            0x0027_42c8 => c.g[V0][0] = O406_TRACK as u64,
            _ => {}
        }
    }

    macro_rules! o406_fakes {
        ($(($name:ident, $addr:expr, $shard:ident, $slot:ident, $gen:ident)),* $(,)?) => {
            $(
                pub(crate) fn $name(c: &mut Cpu) {
                    if c.bus.read32(O406_CTRL) != O406_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), ee::$shard::$gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    o406_fake_call(c, $addr, ra);
                }
            )*

            /// Hooks the gated fakes into the callees' override slots (once
            /// per process; with the gate off they run what the slots held).
            pub(crate) fn o406_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    o406_fakes! {
        (o406_fake_boost, 0x0011_4130, s00, OVERRIDE_00114130, f_00114130__generated),
        (o406_fake_wait_push, 0x0012_c0c0, s00, OVERRIDE_0012C0C0, f_0012c0c0__generated),
        (o406_fake_push, 0x0012_c130, s00, OVERRIDE_0012C130, f_0012c130__generated),
        (o406_fake_charge, 0x0012_c230, s00, OVERRIDE_0012C230, f_0012c230__generated),
        (o406_fake_launch, 0x0012_c408, s00, OVERRIDE_0012C408, f_0012c408__generated),
        (o406_fake_rider_state, 0x0011_fe98, s00, OVERRIDE_0011FE98, f_0011fe98__generated),
        (o406_fake_rider_misc, 0x0011_5640, s00, OVERRIDE_00115640, f_00115640__generated),
        (o406_fake_rider_leave, 0x0011_fec8, s00, OVERRIDE_0011FEC8, f_0011fec8__generated),
        (o406_fake_compare, 0x0032_36d8, s10, OVERRIDE_003236D8, f_003236d8__generated),
        (o406_fake_next_token, 0x0032_1af0, s10, OVERRIDE_00321AF0, f_00321af0__generated),
        (o406_fake_alloc_node, 0x0032_15f8, s10, OVERRIDE_003215F8, f_003215f8__generated),
        (o406_fake_parser_error, 0x0032_1500, s10, OVERRIDE_00321500, f_00321500__generated),
        (o406_fake_lookup_button, 0x0032_16e8, s10, OVERRIDE_003216E8, f_003216e8__generated),
        (o406_fake_lookup_builtin, 0x0032_1880, s10, OVERRIDE_00321880, f_00321880__generated),
        (o406_fake_find_identifier, 0x0032_1a40, s10, OVERRIDE_00321A40, f_00321a40__generated),
        (o406_fake_emit_node, 0x0032_4678, s10, OVERRIDE_00324678, f_00324678__generated),
        (o406_fake_preproc_write, 0x0032_6360, s10, OVERRIDE_00326360, f_00326360__generated),
        (o406_fake_next_char, 0x0032_6478, s10, OVERRIDE_00326478, f_00326478__generated),
        (o406_fake_skip_line_comment, 0x0032_65c0, s10, OVERRIDE_003265C0, f_003265c0__generated),
        (o406_fake_rand, 0x0031_77f0, s10, OVERRIDE_003177F0, f_003177f0__generated),
        (o406_fake_new_named, 0x0031_7e30, s10, OVERRIDE_00317E30, f_00317e30__generated),
        (o406_fake_anim_a, 0x0031_1ae8, s10, OVERRIDE_00311AE8, f_00311ae8__generated),
        (o406_fake_anim_b, 0x0031_1b20, s10, OVERRIDE_00311B20, f_00311b20__generated),
        (o406_fake_chunk_record, 0x0032_c540, s10, OVERRIDE_0032C540, f_0032c540__generated),
        (o406_fake_event_fire_key, 0x0027_3418, s07, OVERRIDE_00273418, f_00273418__generated),
        (o406_fake_event_next_key, 0x0027_3468, s07, OVERRIDE_00273468, f_00273468__generated),
        (o406_fake_flush_events, 0x0027_3d20, s07, OVERRIDE_00273D20, f_00273d20__generated),
        (o406_fake_get_length, 0x0027_4c70, s07, OVERRIDE_00274C70, f_00274c70__generated),
        (o406_fake_get_listener, 0x0027_4d70, s07, OVERRIDE_00274D70, f_00274d70__generated),
        (o406_fake_get_track, 0x0027_42c8, s07, OVERRIDE_002742C8, f_002742c8__generated),
        (o406_fake_node_matrix, 0x0034_fed8, s11, OVERRIDE_0034FED8, f_0034fed8__generated),
        (o406_fake_collide_instance, 0x0033_5960, s11, OVERRIDE_00335960, f_00335960__generated),
        (o406_fake_collide_object, 0x0033_5b90, s11, OVERRIDE_00335B90, f_00335b90__generated),
        (o406_fake_bolts_db, 0x0014_bdb8, s01, OVERRIDE_0014BDB8, f_0014bdb8__generated),
        (o406_fake_find_bolt, 0x0014_d448, s01, OVERRIDE_0014D448, f_0014d448__generated),
        (o406_fake_set_bolt, 0x0015_1c90, s01, OVERRIDE_00151C90, f_00151c90__generated),
        (o406_fake_flags_test, 0x0014_46a0, s01, OVERRIDE_001446A0, f_001446a0__generated),
        (o406_fake_mem_fill, 0x003e_6448, s14, OVERRIDE_003E6448, f_003e6448__generated),
    }

    /// The miss hook of the sequence tests: the track player's tick (vtable
    /// target `FAKE`) answers from the result queue and writes a flag word;
    /// the key-distance call (`FAKE + 0x30`) answers a small count.
    fn o406_miss(c: &mut Cpu, target: u32) {
        let a1 = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        if target == fakes::FAKE {
            c.g[V0][0] = o406_pop64(c);
            let flags = o406_pop64(c) as u32;
            c.bus.write32(a1, flags);
        } else if target == fakes::FAKE + 0x30 {
            c.g[V0][0] = o406_pop64(c);
        }
    }

    /// Scrambled registers, the gate, `pc` at the entry, the fakes.
    fn o406_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        o406_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.pc = entry;
        c.bus.write32(O406_CTRL, O406_MAGIC);
        c.hooks.miss = o406_miss;
        r
    }

    /// Fill the 64-bit result queue.
    fn o406_results(c: &mut Cpu, values: &[u64]) {
        for (i, v) in values.iter().enumerate() {
            c.bus.write64(O406_RESULTS + 8 * i as u32, *v);
        }
    }

    fn o406_kinds(c: &mut Cpu, kinds: &[u32]) {
        for (i, k) in kinds.iter().enumerate() {
            c.bus.write32(O406_KINDS + 4 * i as u32, *k);
        }
    }

    /// A pool of 12 expression nodes at `O406_NODES`: constants (kind 4,
    /// value 0.0 / 1.0 / -3.5 / 0.5) and other leaf kinds (children null).
    fn o406_node_pool(c: &mut Cpu, r: &mut Rng) {
        const VALUES: [u32; 4] = [0, 0x3f80_0000, 0xc060_0000, 0x3f00_0000];
        const KINDS: [u32; 5] = [5, 6, 7, 0x10, 0x21];
        for i in 0..12u32 {
            let n = O406_NODES + 0x40 * i;
            for off in (0..0x40).step_by(4) {
                c.bus.write32(n + off, 0);
            }
            if r.next_u32() % 20 < 11 {
                c.bus.write32(n, 4);
                c.bus.write32(n + 4, VALUES[(r.next_u32() % 4) as usize]);
            } else {
                c.bus.write32(n, KINDS[(r.next_u32() % 5) as usize]);
            }
        }
    }

    /// Queue of node results: the pool in a random order, null now and then.
    fn o406_node_results(c: &mut Cpu, r: &mut Rng, first: usize) {
        let mut values = vec![0u64; first];
        for i in 0..14u32 {
            let k = (r.next_u32() % 12) + i % 3;
            values.push(if r.next_u32() % 8 == 0 {
                0
            } else {
                (O406_NODES + 0x40 * (k % 12)) as u64
            });
        }
        o406_results(c, &values);
    }

    /// An InputParser at `O406_PARSER` holding `kind` as the current token.
    fn o406_parser(c: &mut Cpu, kind: u32) {
        for off in (0..0x200).step_by(4) {
            c.bus.write32(O406_PARSER + off, 0);
        }
        c.bus.write32(O406_PARSER + 0x98, kind);
        c.g[A0][0] = O406_PARSER as u64;
    }

    /// A random token kind out of `kinds`.
    fn o406_pick(r: &mut Rng, kinds: &[u32]) -> u32 {
        kinds[(r.next_u32() as usize) % kinds.len()]
    }

    // ------------------------------------------------ RiderGateSub_HandleInput

    /// The gate's sub-state dispatch: every state (-1, 0, 1, 2, 3, 4) with
    /// the five callees (boost input, charge, launch, wait-push, push) as
    /// fakes; a fake unwinds at a random call.
    #[test]
    fn rider_gate_sub_handle_input_matches() {
        fakes::covered(
            ee::s00::f_0012bf68__generated,
            rider_gate_sub_handle_input,
            140,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x12bf68, 2, 0x0012_bf68);
                let states = [0u32, 1, 2, 3, 4, u32::MAX, 9];
                o406_w(c, DATA, states[(seed % 7) as usize]);
                o406_w(c, DATA + 0x14, r.next_u32());
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    // --------------------------------------- InputParser_ParseLogicalAnd/Or/...

    /// `a && b && ...`: a constant 0 / 1 / other operands, `&&` tokens, a null
    /// operand (error), node kinds; the comparison/NextToken/alloc callees are
    /// fakes fed from queues; `FreeNode` is real.
    #[test]
    fn input_parser_parse_logical_and_matches() {
        fakes::covered(
            ee::s10::f_00323540__generated,
            input_parser_parse_logical_and,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x323540, 8, 0x0032_3540);
                elf_image(c);
                o406_node_pool(c, &mut r);
                o406_node_results(c, &mut r, 0);
                let mut kinds = vec![];
                for _ in 0..6 {
                    kinds.push(o406_pick(&mut r, &[0xd, 0xd, 0xd, 0]));
                }
                o406_kinds(c, &kinds);
                o406_parser(c, o406_pick(&mut r, &[0xd, 0xd, 0xd, 0xc]));
            },
        );
    }

    /// `a || b || ...` over the real `ParseLogicalAnd`.
    #[test]
    fn input_parser_parse_logical_or_matches() {
        fakes::covered(
            ee::s10::f_003233b8__generated,
            input_parser_parse_logical_or,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x3233b8, 8, 0x0032_33b8);
                elf_image(c);
                o406_node_pool(c, &mut r);
                o406_node_results(c, &mut r, 0);
                let mut kinds = vec![];
                for _ in 0..8 {
                    kinds.push(o406_pick(&mut r, &[0xc, 0xc, 0xd, 0]));
                }
                o406_kinds(c, &kinds);
                o406_parser(c, o406_pick(&mut r, &[0xc, 0xc, 0xd, 0xd, 0]));
            },
        );
    }

    /// `c ? a : b` (recursing into the real `ParseConditional`).
    #[test]
    fn input_parser_parse_conditional_matches() {
        fakes::covered(
            ee::s10::f_00323268__generated,
            input_parser_parse_conditional,
            240,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x323268, 10, 0x0032_3268);
                elf_image(c);
                o406_node_pool(c, &mut r);
                o406_node_results(c, &mut r, 0);
                let mut kinds = vec![];
                for _ in 0..10 {
                    kinds.push(o406_pick(&mut r, &[7, 7, 6, 0, 0xc, 0xd, 5]));
                }
                o406_kinds(c, &kinds);
                o406_parser(c, o406_pick(&mut r, &[6, 6, 6, 0, 7]));
            },
        );
    }

    /// `name = expression;`: the name token, the three lookups (button,
    /// builtin, identifier: mostly "not found"), the new variable (real
    /// `strcpy`), `=`, the expression (real parser chain) and `;`, with the
    /// error exits (missing `=`, missing `;`, failed expression, redefined
    /// name).
    #[test]
    fn input_parser_parse_statement_matches() {
        fakes::covered(
            ee::s10::f_003230e0__generated,
            input_parser_parse_statement,
            260,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x3230e0, 10, 0x0032_30e0);
                elf_image(c);
                o406_node_pool(c, &mut r);
                o406_node_results(c, &mut r, 3);
                let lookups = [u64::MAX, u64::MAX, 0, 0, 0, 1];
                let mut head = vec![];
                for i in 0..3 {
                    let pick = if r.next_u32() % 5 == 0 {
                        lookups[(r.next_u32() % 6) as usize]
                    } else if i < 2 {
                        u64::MAX
                    } else {
                        0
                    };
                    head.push(pick);
                }
                for (i, v) in head.iter().enumerate() {
                    c.bus.write64(O406_RESULTS + 8 * i as u32, *v);
                }
                let kinds = [
                    o406_pick(&mut r, &[3, 3, 3, 3, 4]),
                    o406_pick(&mut r, &[5, 5, 5, 5, 0xd, 0xc, 6, 0]),
                    o406_pick(&mut r, &[5, 5, 0, 7]),
                    o406_pick(&mut r, &[5, 0]),
                    0,
                    5,
                ];
                o406_kinds(c, &kinds);
                o406_parser(c, o406_pick(&mut r, &[2, 2, 2, 2, 2, 5]));
                // The token text and the variable table (count at +0x80e4).
                c.bus.write32(O406_PARSER + 0x9c, 0x0000_6261);
                c.bus.write32(O406_PARSER + 0x80e4, r.next_u32() % 4);
            },
        );
    }

    // --------------------------------------------- InputParser_EmitBinaryOp

    /// Compile a binary node: both `EmitNode` calls (fakes: a word count or -1
    /// and the chosen register), the free-register list, the destination
    /// choice, the allocation and overflow errors and the opcode word.
    #[test]
    fn input_parser_emit_binary_op_matches() {
        fakes::covered(
            ee::s10::f_00324ec8__generated,
            input_parser_emit_binary_op,
            260,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x324ec8, 3, 0x0032_4ec8);
                let regs = [u64::MAX, 3, 0x20, 0x3f, 0x40, 0x7f, 1, 2];
                let counts = [u64::MAX, 0, 1, 2, 3, 1, 2];
                let mut values = vec![];
                for _ in 0..2 {
                    values.push(counts[(r.next_u32() % 7) as usize]);
                    values.push(regs[(r.next_u32() % 8) as usize]);
                }
                o406_results(c, &values);
                o406_parser(c, 0);
                let free_count = r.next_u32() % 4;
                // The free list (+0x108ec) and its count (+0x109e8).
                c.bus.write32(O406_PARSER + 0x10000 + 0x9e8, free_count);
                for i in 0..4 {
                    c.bus
                        .write32(O406_PARSER + 0x10000 + 0x8ec + 4 * i, r.next_u32() % 0x40);
                }
                c.g[A1][0] = (O406_NODES) as u64;
                c.g[A2][0] = (DATA + 0x100) as u64;
                let dst = [u32::MAX, 5, 0x30, u32::MAX][(r.next_u32() % 4) as usize];
                c.bus.write32(DATA + 0x100, dst);
                c.g[A3][0] = (DATA + 0x800) as u64;
                c.bus.write32(DATA + 0x800, r.next_u32());
                c.g[T0][0] = (r.next_u32() % 9) as u64;
                c.g[T1][0] = r.next_u32() as u64;
                c.g[T2][0] = r.next_u32() as u64;
            },
        );
    }

    // --------------------------------------------- InputPreproc_ProcessText

    /// One source line: leading blanks, `//` and `/*` comments, a plain `/`
    /// and ordinary text up to the newline (lookahead characters from a
    /// queue; `Write`, `NextChar` and the line-comment skipper are fakes, the
    /// block-comment skipper is real). The `#` directive path is not
    /// reached (the real directive handler reads files).
    #[test]
    fn input_preproc_process_text_matches() {
        fakes::covered(
            ee::s10::f_00326150__generated,
            input_preproc_process_text,
            260,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x326150, 6, 0x0032_6150);
                const ALPHABET: [u32; 9] = [0x20, 9, 0xd, 0x78, 0x2f, 0x2a, 0xa, 0x2f, 0x79];
                let first = ALPHABET[(r.next_u32() % 9) as usize];
                for i in 0..20u32 {
                    let ch = if i == 16 {
                        u32::MAX
                    } else {
                        ALPHABET[(r.next_u32() % 9) as usize]
                    };
                    c.bus.write32(O406_CHARS + 4 * i, ch);
                }
                o406_w(c, DATA + 0xc, if seed % 17 == 0 { u32::MAX } else { first });
                o406_w(c, DATA + 0x100 + 0xc, r.next_u32() % 50);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    // ------------------------------------------------ InSeq channel updates

    /// A channel at `DATA`: header (+0x1c: byte +2 track count, byte +3
    /// type), the track player array (+0x24), a communicator (+0x2c) whose
    /// vtable entries (+0x1c, +0x24) are fakes. Each track player's vtable
    /// (+0x1c tick, +0x34 key distance) points at `FAKE`/`FAKE + 0x30`.
    fn o406_channel(c: &mut Cpu, r: &mut Rng, tracks: u32) {
        let hdr = DATA + 0x100;
        let array = DATA + 0x200;
        let vtbl = DATA + 0x500;
        let comm_vt = DATA + 0x600;
        o406_w(c, DATA + 0x1c, hdr);
        c.bus.write8(hdr + 2, tracks as u8);
        c.bus.write8(hdr + 3, (r.next_u32() % 4) as u8);
        o406_w(c, DATA + 0x24, array);
        for i in 0..tracks {
            let player = DATA + 0x300 + 0x40 * i;
            o406_w(c, array + 4 * i, player);
            o406_w(c, player + 4, vtbl);
        }
        c.bus.write16(vtbl + 0x18, 0x10);
        o406_w(c, vtbl + 0x1c, fakes::FAKE);
        c.bus.write16(vtbl + 0x30, 0x20);
        o406_w(c, vtbl + 0x34, fakes::FAKE + 0x30);
        let comm = if r.next_u32() % 4 == 0 {
            0
        } else {
            DATA + 0x580
        };
        o406_w(c, DATA + 0x2c, comm);
        o406_w(c, DATA + 0x580, comm_vt);
        c.bus.write16(comm_vt + 0x18, 0x8);
        o406_w(c, comm_vt + 0x1c, fakes::FAKE + 0x10);
        c.bus.write16(comm_vt + 0x20, 0x8);
        o406_w(c, comm_vt + 0x24, fakes::FAKE + 0x20);
    }

    /// Tick results the track players give: progress or none, and flag words.
    fn o406_tick_results(c: &mut Cpu, r: &mut Rng) {
        let mut values = vec![];
        for _ in 0..10 {
            values.push([0u64, 1, 1, 5][(r.next_u32() % 4) as usize]);
            values.push([0u64, 0, 1, 0x10][(r.next_u32() % 4) as usize]);
        }
        o406_results(c, &values);
    }

    /// Run every track's tick (vtable call), OR the flags and notify the
    /// communicator (0 to 3 tracks, with and without a communicator).
    #[test]
    fn in_seq_channel_update_tracks_matches() {
        fakes::covered(
            ee::s07::f_002743e8__generated,
            in_seq_channel_update_tracks,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x2743e8, 6, 0x0027_43e8);
                let tracks = r.next_u32() % 4;
                o406_channel(c, &mut r, tracks);
                o406_tick_results(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.f[12] = r.f32(100.0);
                c.f[13] = r.f32(2.0);
            },
        );
    }

    /// Advance a channel: stopped (`+0x28 == 0`), without a step, and with
    /// one: the key distances (fake `FAKE + 0x30`) clamp the time, the real
    /// `UpdateTracks` runs until the time is used or reports no progress.
    #[test]
    fn in_seq_channel_advance_matches() {
        fakes::covered(
            ee::s07::f_00274100__generated,
            in_seq_channel_advance,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x274100, 12, 0x0027_4100);
                let tracks = r.next_u32() % 3;
                o406_channel(c, &mut r, tracks);
                o406_tick_results(c, &mut r);
                // Key distances after the tick answers: small counts.
                let mut values = vec![];
                for i in 0..24 {
                    values.push(if i % 3 == 0 {
                        [0u64, 1, 2, 3, 6][(r.next_u32() % 5) as usize]
                    } else if i % 3 == 1 {
                        r.next_u32() as u64 % 2
                    } else {
                        r.next_u32() as u64 % 0x20
                    });
                }
                o406_results(c, &values);
                o406_w(c, DATA + 0x28, if seed % 7 == 0 { 0 } else { 1 });
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 5 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
                c.f[12] = r.f32(10.0);
                c.f[13] = r.f32(5.0);
                c.f[14] = r.f32(5.0);
            },
        );
    }

    /// Event track: the keys (halfword time at +2) at `DATA + 0x300`.
    fn o406_event_track(c: &mut Cpu, r: &mut Rng) {
        let n = 1 + r.next_u32() % 4;
        let mut t = r.next_u32() % 3;
        let mut next = vec![];
        for i in 0..n {
            let key = DATA + 0x300 + 0x10 * i;
            c.bus.write16(key + 2, t as u16);
            t += r.next_u32() % 3;
            next.push((key + 0x10) as u64);
        }
        // The chain ends (the key after the last is null).
        let last = next.len() - 1;
        next[last] = 0;
        o406_results(c, &next);
        o406_w(c, DATA + 0x14, DATA + 0x300);
        if r.next_u32() % 6 == 0 {
            o406_w(c, DATA + 0x14, 0);
        }
        c.f[12] = ((r.next_u32() % 12) as f32).to_bits();
        let t = c.f[12];
        o406_w(
            c,
            DATA + 0x24,
            if r.next_u32() % 3 == 0 { t } else { r.f32(5.0) },
        );
    }

    /// Fire the keys the time has reached (the fire/advance callees are
    /// fakes; the next-key fake feeds the chain).
    #[test]
    fn in_seq_event_track_update_matches() {
        fakes::covered(
            ee::s07::f_00273330__generated,
            in_seq_event_track_update,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x273330, 6, 0x0027_3330);
                o406_event_track(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x80) as u64;
                o406_w(c, DATA + 0x80, r.next_u32() & !0x10);
            },
        );
    }

    /// `ScriptRider_ReadFields`: the twelve (track lookup, virtual read) pairs.
    #[test]
    fn script_rider_read_fields_matches() {
        fakes::covered(
            ee::s07::f_0027c9b0__generated,
            script_rider_read_fields,
            120,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x27c9b0, 24, 0x0027_c9b0);
                let vt = DATA + 0x500;
                o406_w(c, O406_TRACK + 4, vt);
                for off in [0x20u32, 0x28] {
                    c.bus.write16(vt + off, 0x10);
                    o406_w(c, vt + off + 4, fakes::FAKE + off);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                for off in (0..0x100).step_by(4) {
                    o406_w(c, DATA + 0x2000 + off, r.next_u32());
                }
            },
        );
    }

    // ------------------------------------------------------ InSeq_InstanceTick

    /// An instance at `DATA`: state at +0x20, delay +0x2c, channel table
    /// header +0x10 (byte +1 = channel count) with channels (0x3c each) at
    /// +0x18, flag +0x1c, times +0x24/+0x28, +0x40, listener from the fake.
    #[test]
    fn in_seq_instance_tick_matches() {
        fakes::covered(
            ee::s07::f_00274de0__generated,
            in_seq_instance_tick,
            300,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x274de0, 8, 0x0027_4de0);
                let listener = DATA + 0x700;
                let l_vt = DATA + 0x780;
                o406_w(c, listener, l_vt);
                for off in [8u32, 0x10, 0x18, 0x28] {
                    c.bus.write16(l_vt + off, 0x4);
                    o406_w(c, l_vt + off + 4, fakes::FAKE + 0x10 + off);
                }
                // Results: listener, length, then channel flush answers.
                let mut values = vec![];
                values.push(if r.next_u32() % 4 == 0 {
                    0
                } else {
                    listener as u64
                });
                values.push([4u64, 4, 8, 12][(r.next_u32() % 4) as usize]);
                for _ in 0..6 {
                    values.push(r.next_u32() as u64 % 2);
                }
                o406_results(c, &values);
                let states = [0u32, 1, 2, 3, 4, 2, 3, 1];
                o406_w(c, DATA + 0x20, states[(seed % 8) as usize]);
                o406_w(c, DATA + 0x2c, if seed % 9 == 0 { 3 } else { 0 });
                let hdr = DATA + 0x100;
                o406_w(c, DATA + 0x10, hdr);
                c.bus.write8(hdr + 1, (seed % 3 == 0) as u8);
                o406_w(c, DATA + 0x18, DATA + 0x200);
                o406_w(c, DATA + 0x1c, (seed % 3 == 0) as u32);
                o406_w(c, DATA + 0x24, r.f32(3.0));
                o406_w(c, DATA + 0x28, (r.f32(12.0) & 0x7fff_ffff));
                o406_w(c, DATA + 0x40, (seed % 5 == 0) as u32);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    // -------------------------------------------- RailModifier_GetWorldMatrix

    /// `m = m * local * node` with random matrices (the node-matrix callee is
    /// a fake); VU0 results are compared as registers and memory.
    #[test]
    fn rail_modifier_get_world_matrix_matches() {
        fakes::covered(
            ee::s11::f_0035c5a0__generated,
            rail_modifier_get_world_matrix,
            100,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x35c5a0, 1, 0x0035_c5a0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                for off in [0x34u32, 0x40] {
                    o406_w(c, DATA + off, r.next_u32());
                }
                for row in 0..4u32 {
                    let a = [r.f32(4.0), r.f32(4.0), r.f32(4.0), r.f32(2.0)];
                    let b = [r.f32(4.0), r.f32(4.0), r.f32(4.0), r.f32(2.0)];
                    put_vec(c, DATA + 0x400 + 0x10 * row, a);
                    put_vec(c, DATA + 0x50 + 0x10 * row, b);
                }
            },
        );
    }

    // -------------------------------------------- CameraWobble_UpdateTarget

    /// The wobble target: zero amount (cleared), timer below/above its limit
    /// (step constant at `gp-0x6c24`, or two random draws; the random
    /// callee is a fake).
    #[test]
    fn camera_wobble_update_target_matches() {
        fakes::covered(
            ee::s01::f_00160028__generated,
            camera_wobble_update_target,
            120,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x160028, 2, 0x0016_0028);
                c.g[A0][0] = DATA as u64;
                c.f[12] = if seed % 4 == 0 { 0 } else { r.f32(3.0) };
                for off in [0xcu32, 0x10, 0x20, 0x28] {
                    o406_w(c, DATA + off, r.f32(5.0));
                }
                o406_w(c, GAME_GP - 0x6c24, 0x3e80_0000);
            },
        );
    }

    // ------------------------------------------------- RiderModel_FindBone

    /// Look a bone up by name in a part's bone records (real `strcmp`):
    /// found at some index, not found, or an empty part.
    #[test]
    fn rider_model_find_bone_matches() {
        crate::difftest::check(
            ee::s10::f_00310c48__generated,
            rider_model_find_bone,
            160,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x310c48, 0, 0x0031_0c48);
                let part = (r.next_u32() % 3) as u32;
                let table = DATA + 0x200;
                let bones = DATA + 0x1000;
                o406_w(c, DATA + 0xc, table);
                o406_w(c, DATA + 0x1c, DATA + 0x100);
                o406_w(c, DATA + 0x100 + 4 * part, 1);
                let count = r.next_u32() % 5;
                o406_w(c, table + 0x58 + 0x44, count);
                o406_w(c, table + 0x58 + 0x38, bones);
                o406_w(c, table + 0x58 + 4, r.next_u32() % 100);
                for i in 0..4u32 {
                    let name = [b'b', b'o', b'n', b'e', b'0' + i as u8, 0, 0, 0];
                    for (k, b) in name.iter().enumerate() {
                        c.bus.write8(bones + 0x50 * i + k as u32, *b);
                    }
                }
                let want = r.next_u32() % 5;
                let name = [b'b', b'o', b'n', b'e', b'0' + want as u8, 0, 0, 0];
                for (k, b) in name.iter().enumerate() {
                    c.bus.write8(DATA + 0x800 + k as u32, *b);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = part as u64;
                c.g[A2][0] = (DATA + 0x800) as u64;
            },
        );
    }

    // ------------------------------------- RiderRailslideSub_CheckAirborne

    /// The rail-slide end check: the rider state callee (1 or other), the
    /// animation (0xe or other) with the flag test, and the turn input (zero
    /// or not); the callees are fakes fed from the result queue.
    #[test]
    fn rider_railslide_sub_check_airborne_matches() {
        fakes::covered(
            ee::s01::f_00132770__generated,
            rider_railslide_sub_check_airborne,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x132770, 5, 0x0013_2770);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                o406_w(c, DATA + 8, DATA + 0x200);
                o406_w(c, DATA + 0x200 + 0x784, DATA + 0x300);
                o406_w(
                    c,
                    DATA + 0x104,
                    [0u32, 0, 0x4, 0x8, 0x3f, 0x21][(r.next_u32() % 6) as usize],
                );
                o406_w(c, GAME_GP - 0x7570, 0x3f80_0000);
                // Results: the rider state, the animation, the flag test.
                let state = [1u64, 1, 1, 0, 2][(r.next_u32() % 5) as usize];
                let anim = [0xeu64, 0xe, 5][(r.next_u32() % 3) as usize];
                let flag = [0u64, 1, 1][(r.next_u32() % 3) as usize];
                o406_results(c, &[state, anim, flag]);
            },
        );
    }

    // ---------------------------------------------- cPS2ChunkResolve_v19

    /// Resolve a chunk: the three offsets (-1 or relative), type 3 (the
    /// records walk; one fake callee, one real) and type 1 (the variable
    /// records), type 2 (nothing more).
    #[test]
    fn c_ps2_chunk_resolve_v19_matches() {
        fakes::covered(
            ee::s13::f_003aaa08__generated,
            c_ps2_chunk_resolve_v19,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x3aaa08, 4, 0x003a_aa08);
                let chunk = DATA + 0x1000;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = chunk as u64;
                let off = |r: &mut Rng, v: u32| if r.next_u32() % 3 == 0 { u32::MAX } else { v };
                o406_w(c, chunk + 4, off(&mut r, 0x100));
                o406_w(c, chunk + 8, off(&mut r, 0x200));
                o406_w(c, chunk + 0xc, off(&mut r, 0x300));
                let kind = [1u16, 3, 3, 1, 2, 7][(r.next_u32() % 6) as usize];
                c.bus.write16(chunk, kind);
                c.bus.write16(chunk + 2, (r.next_u32() % 3) as u16);
                // The records (type 3: a table at +0x200 of 0x140-byte
                // entries; type 1: variable records at +0x100, 0x10 bytes
                // apart plus their offsets).
                for i in 0..3u32 {
                    let rec = chunk + 0x100 + 0x40 * i;
                    c.bus.write16(rec, (r.next_u32() % 2) as u16);
                    for k in 1..5u32 {
                        o406_w(c, rec + 4 * k, 0x10 * k);
                    }
                }
            },
        );
    }

    // ------------------------------------------------ RiderModel_LoadVariant

    /// Register a model part: known id (the lod record is filed) or a new
    /// part (four lod tables allocated and cleared through the fakes).
    #[test]
    fn rider_model_load_variant_matches() {
        fakes::covered(
            ee::s10::f_0030d8b8__generated,
            rider_model_load_variant,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x30d8b8, 8, 0x0030_d8b8);
                let this = DATA + 0x5_0000;
                let parts = DATA + 0x5_1000;
                let part = DATA + 0x5_8000;
                let count = r.next_u32() % 4;
                o406_w(c, this + 4, r.next_u32() % 4);
                o406_w(c, this + 8, count);
                o406_w(c, this + 0xc, parts);
                for i in 0..4u32 {
                    let e = parts + 0x58 * i;
                    o406_w(c, e, 1 + r.next_u32() % 3);
                    for lod in 0..4u32 {
                        o406_w(
                            c,
                            e + 0x1c + 8 * lod,
                            DATA + 0x6_0000 + 0x100 * (4 * i + lod),
                        );
                    }
                }
                c.bus.write8(part + 0x52, (1 + r.next_u32() % 3) as u8);
                for off in [0x18u32, 0x1c, 0x2c] {
                    o406_w(c, part + off, r.next_u32() & 0xfff);
                }
                for off in [0x4au32, 0x4e, 0x50] {
                    c.bus.write16(part + off, r.next_u32() as u16);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = part as u64;
                c.g[A3][0] = (r.next_u32() % 3) as u64;
            },
        );
    }

    // ------------------------------------------ BE_ApplyBoltDependencies

    /// Switching a bolt on or off pulls its links (real `FindTable2Entries`
    /// over a small database): conditions, owned/unowned targets, the target
    /// being the bolt itself (flag set/clear) or another bolt switched
    /// through a fake; the failure exit.
    #[test]
    fn be_apply_bolt_dependencies_matches() {
        fakes::covered(
            ee::s01::f_00151ef0__generated,
            be_apply_bolt_dependencies,
            240,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x151ef0, 4, 0x0015_1ef0);
                let this = DATA + 0x2000;
                let index = DATA + 0x3000;
                let links = DATA + 0x1100;
                let kind = (seed % 2) as u32;
                c.bus.write8(this + 0xbc0, kind as u8);
                o406_w(c, this + 0x288, index);
                for id in 0..8u32 {
                    let slot = if r.next_u32() % 6 == 0 {
                        0xffff
                    } else {
                        r.next_u32() % 6
                    };
                    c.bus.write16(index + 2 * id, slot as u16);
                }
                for i in 0..6u32 {
                    c.bus.write16(this + 0x290 + 4 * i, r.next_u32() as u16 % 8);
                    c.bus
                        .write16(this + 0x290 + 4 * i + 2, (r.next_u32() & 0x10) as u16);
                }
                // The database: link table, per-kind start and count.
                o406_w(c, O406_BOLT_DB + 8, links);
                let count = 1 + r.next_u32() % 3;
                o406_w(c, O406_BOLT_DB + 0x20c + 4 * kind, count);
                o406_w(c, O406_BOLT_DB + 0x284 + 4 * kind, r.next_u32() % 2);
                let id = 1 + r.next_u32() % 5;
                let on = r.next_u32() % 2;
                for i in 0..8u32 {
                    let l = links + 0xc * i;
                    c.bus.write8(l + 1, on as u8);
                    c.bus.write16(l + 2, id as u16);
                    c.bus.write8(l + 5, (r.next_u32() % 2) as u8);
                    c.bus.write16(
                        l + 6,
                        [0xffffu32, 1, 2, 3][(r.next_u32() % 4) as usize] as u16,
                    );
                    c.bus.write8(l + 9, (r.next_u32() % 2) as u8);
                    c.bus.write16(
                        l + 0xa,
                        [0xffffu32, 1, 2, 3, id][(r.next_u32() % 5) as usize] as u16,
                    );
                }
                o406_w(c, O406_BOLT_DB + 4, DATA + 0x1400);
                for i in 0..8u32 {
                    o406_w(c, DATA + 0x1400 + 0x38 * i + 0x34, r.next_u32() & 8);
                }
                let mut values = vec![];
                for _ in 0..10 {
                    values.push([u64::MAX, 0, 1, 2][(r.next_u32() % 4) as usize]);
                    values.push((r.next_u32() % 2) as u64);
                }
                o406_results(c, &values);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = id as u64;
                c.g[A2][0] = on as u64;
            },
        );
    }

    // ------------------------------------ Cell_CollideColumnRecursive

    /// Octree column collision: the cell classified against the box
    /// (outside, partly covered), the node's two object lists (fakes) and
    /// the eight children (real recursion).
    #[test]
    fn cell_collide_column_recursive_matches() {
        fakes::covered(
            ee::s11::f_0033dbe8__generated,
            cell_collide_column_recursive,
            240,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x33dbe8, 4, 0x0033_dbe8);
                let node = DATA + 0x1000;
                let cell = DATA + 0x2000;
                let ctx = DATA + 0x3000;
                let floats = DATA + 0x3100;
                let level = 3 + r.next_u32() % 9;
                let (x, y) = (r.next_u32() % 4, r.next_u32() % 4);
                o406_w(c, cell, level);
                o406_w(c, cell + 4, x);
                o406_w(c, cell + 8, y);
                o406_w(c, cell + 0xc, r.next_u32() % 4);
                o406_w(c, GAME_GP - 0x2ce0, 0x3f00_0000);
                o406_w(c, ctx, floats);
                let scale = f32::from_bits((level + 0x7f) << 23);
                let draw = |r: &mut Rng, base: u32| {
                    let u = (r.next_u32() % 3000) as f32 / 1000.0 - 1.0;
                    ((base as f32 + u) * scale).to_bits()
                };
                o406_w(c, floats, draw(&mut r, x));
                o406_w(c, floats + 4, draw(&mut r, y));
                // Object lists and children.
                for off in (0..0x40).step_by(4) {
                    o406_w(c, node + off, 0);
                }
                if r.next_u32() % 2 == 0 {
                    o406_w(c, node + 0x24, DATA + 0x900);
                    o406_w(
                        c,
                        DATA + 0x900,
                        if r.next_u32() % 2 == 0 {
                            DATA + 0x910
                        } else {
                            0
                        },
                    );
                    o406_w(c, DATA + 0x910, 0);
                }
                if r.next_u32() % 2 == 0 {
                    o406_w(c, node + 0x20, DATA + 0x920);
                    o406_w(c, DATA + 0x920, 0);
                }
                for child in 0..8u32 {
                    if r.next_u32() % 2 == 0 {
                        let n = DATA + 0x1400 + 0x40 * child;
                        for off in (0..0x40).step_by(4) {
                            o406_w(c, n + off, 0);
                        }
                        o406_w(c, node + 4 * child, n);
                    }
                }
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = (DATA + 0x4000) as u64;
                c.g[A2][0] = (DATA + 0x4100) as u64;
                c.g[A3][0] = ctx as u64;
                c.g[T0][0] = node as u64;
                c.g[T1][0] = cell as u64;
            },
        );
    }
}
