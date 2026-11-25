//! Staging module oc654 (wave 234): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc654`.

#![allow(unused_imports)]

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0026_fa78,
        replay_update_camera_input,
        "Replay_UpdateCameraInput",
    ),
    (
        0x0027_0c58,
        replay_find_weakest_highlight,
        "Replay_FindWeakestHighlight",
    ),
];

/// `int ReplayStream_Decompress(ReplayStream* self, u8** pp, u8* src)`
/// (0x26eab8): decode one compressed frame: header, body, then rewind the
/// read pointer.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn replay_stream_decompress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        call_ptr(c, 0x003e_6574, 0x0026_eae0)?;
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = c.g[SP][0] | 4;
        c.g[A2][0] = 4;
        call_ptr(c, 0x003e_6574, 0x0026_eaf0)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        let v = c.g[A0][0];
        Addr(r32(c, S0)).sw(c, v);
        c.g[A2][0] = Addr(r32(c, S1).wrapping_add(4)).lw(c);
        call_ptr(c, 0x002c_85a0, 0x0026_eb08)?;
        c.g[V1][0] = Addr(sp(c).wrapping_add(4)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
        let v = c.g[A1][0];
        Addr(r32(c, S0)).sw(c, v);
        c.g[V1][0] = Addr(r32(c, S1).wrapping_add(4)).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        let v = c.g[V1][0];
        Addr(r32(c, S1).wrapping_add(8)).sw(c, v);
        Callee {
            addr: 0x0026_e9b0,
            f: ee::s07::f_0026e9b0,
        }
        .call(c, 0x0026_eb2c)?;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_RestoreResumeSnapshot(Replay* self)` (0x26f850): restore the
/// resume snapshot (`+0x3d0`), then free it and clear the pointer.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn replay_restore_resume_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x3d0)).lw(c);
        if c.g[A1][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 7;
            Callee {
                addr: 0x0026_db88,
                f: ee::s07::f_0026db88,
            }
            .call(c, 0x0026_f874)?;
            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x3d0)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0027_0238,
                f: ee::s07::f_00270238,
            }
            .call(c, 0x0026_f880)?;
            Addr(r32(c, S0).wrapping_add(0x3d0)).sw(c, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_BeginPlayback(Replay* self, int resume, int instant, int
/// camera)` (0x26f8a0): start a playback from the state it is in: save a
/// resume snapshot if needed, seek to the start, step one frame and enable
/// the replay cameras (the instant-replay ones when `instant`).
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn replay_begin_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
        c.g[V0][0] = ((c.g[V0][0] < 2) as u64) ^ 1;
        let skip = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A3][0];
        'b_26f968: {
            'b_26f964: {
                'b_26f960: {
                    if skip {
                        break 'b_26f960;
                    }
                    'b_26f8f4: {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x608)).lw(c);
                        if c.g[V0][0] != 0 {
                            c.g[A0][0] = c.g[S0][0];
                            break 'b_26f8f4;
                        }
                        c.cycles += 3;
                        Callee {
                            addr: 0x0026_f7f8,
                            f: ee::s07::f_0026f7f8,
                        }
                        .call(c, 0x0026_f8f0)?;
                        c.g[A0][0] = c.g[S0][0];
                    }
                    'b_26f940: {
                        'b_26f938: {
                            c.cycles += 13;
                            c.g[A1][0] = 7;
                            Callee {
                                addr: 0x0026_f4a0,
                                f: ee::s07::f_0026f4a0,
                            }
                            .call(c, 0x0026_f8fc)?;
                            Addr(r32(c, S0).wrapping_add(0x618)).sw(c, 0);
                            c.g[A1][0] = c.g[S2][0];
                            Addr(r32(c, S0).wrapping_add(0x61c)).sw(c, 0);
                            c.g[A0][0] = c.g[S0][0];
                            let v = c.g[S1][0];
                            Addr(r32(c, S0).wrapping_add(0x620)).sw(c, v);
                            Callee {
                                addr: 0x0027_02f8,
                                f: ee::s07::f_002702f8,
                            }
                            .call(c, 0x0026_f914)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 1;
                            Callee {
                                addr: 0x0027_03f0,
                                f: ee::s07::f_002703f0,
                            }
                            .call(c, 0x0026_f920)?;
                            if c.g[S1][0] == 0 {
                                break 'b_26f938;
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x0026_ef30,
                                f: ee::s07::f_0026ef30,
                            }
                            .call(c, 0x0026_f930)?;
                            break 'b_26f940;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x0026_ee68,
                            f: ee::s07::f_0026ee68,
                        }
                        .call(c, 0x0026_f940)?;
                    }
                    c.cycles += 2;
                    let s1_set = c.g[S1][0] != 0;
                    frame.restore(c, S0, 0x40);
                    if s1_set {
                        break 'b_26f964;
                    }
                    c.cycles += 2;
                    let s3_set = c.g[S3][0] != 0;
                    frame.restore(c, S1, 0x30);
                    if s3_set {
                        break 'b_26f968;
                    }
                    c.cycles += 4;
                    Callee {
                        addr: 0x0028_b180,
                        f: ee::s07::f_0028b180,
                    }
                    .call(c, 0x0026_f958)?;
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x0028_f200,
                        f: ee::s07::f_0028f200,
                    }
                    .call(c, 0x0026_f960)?;
                    break 'b_26f960;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x40);
            }
            c.cycles += 1;
            frame.restore(c, S1, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_UpdateCameraInput(Replay* self)` (0x26fa78): while the
/// replay is in state 3, poll the camera controller at index `+0x28` and
/// reset it on the button presses of the replay pad.
pub fn replay_update_camera_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x30);
        c.g[V0][0] = Addr(r32(c, S0)).lw(c) ^ 3;
        'b_26fb74: {
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
                break 'b_26fb74;
            }
            'b_26fad4: {
                c.cycles += 10;
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x28)).lw(c);
                c.g[A1][0] = 0x6c;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xb0)).lw(c);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0032_0c48, 0x0026_fac0)?;
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                if none {
                    break 'b_26fad4;
                }
                c.cycles += 3;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
                call_ptr(c, 0x0022_e840, 0x0026_fad4)?;
            }
            c.cycles += 9;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x630)).lw(c);
            c.g[V1][0] = 0x44_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5438));
            c.g[A1][0] = 0xb;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            if c.g[A0][0] != c.g[A1][0] {
                frame.restore(c, S0, 0x20);
                break 'b_26fb74;
            }
            'b_26fb60: {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                abi::set_fpr(c, 20, FpuF32::ZERO);
                c.g[A1][0] = 0x31;
                call_ptr(c, 0x0032_0bf0, 0x0026_fb08)?;
                let eq = c_eq_f0_f20(c);
                c.g[A0][0] = c.g[S1][0];
                if !eq {
                    break 'b_26fb60;
                }
                c.cycles += 6;
                c.g[A1][0] = 0x30;
                call_ptr(c, 0x0032_0bf0, 0x0026_fb20)?;
                let eq = c_eq_f0_f20(c);
                c.g[A0][0] = c.g[S1][0];
                if !eq {
                    break 'b_26fb60;
                }
                c.cycles += 6;
                c.g[A1][0] = 0x32;
                call_ptr(c, 0x0032_0bf0, 0x0026_fb38)?;
                let eq = c_eq_f0_f20(c);
                c.g[A0][0] = c.g[S1][0];
                if !eq {
                    break 'b_26fb60;
                }
                c.cycles += 6;
                c.g[A1][0] = 0x33;
                call_ptr(c, 0x0032_0bf0, 0x0026_fb50)?;
                let eq = c_eq_f0_f20(c);
                frame.restore(c, S0, 0x20);
                if eq {
                    break 'b_26fb74;
                }
                break 'b_26fb60;
            }
            c.cycles += 5;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
            call_ptr(c, 0x0022_e840, 0x0026_fb70)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_fpr(c, 20, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_StepFrame(Replay* self, int step)` (0x2703f0): advance the
/// replay by `step` frames (`step != 0`), re-finding the current snapshot and
/// walking the run list; past the end it restores the last snapshot or sets
/// the state to 3. Re-enters itself through the generated alias.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn replay_step_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        let no_step = c.g[S1][0] == 0;
        frame.save_ra(c, 0);
        'b_27041c: {
            if no_step {
                break 'b_27041c;
            }
            c.cycles += 3;
            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x484)).lw(c);
            Callee {
                addr: 0x0026_e448,
                f: ee::s07::f_0026e448,
            }
            .call(c, 0x0027_0418)?;
            let v = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(0x488)).sw(c, v);
        }
        'b_27043c: {
            c.cycles += 3;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x488)).lw(c);
            if c.g[V1][0] == 0 {
                break 'b_27043c;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x3d0)).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                break 'b_27043c;
            }
            c.cycles += 2;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
            let v = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(0x488)).sw(c, v);
        }
        'b_270520: {
            'b_27048c: {
                c.cycles += 2;
                let flag_zero = c.g[S1][0] == 0;
                c.g[S1][0] = sx(r32(c, S0).wrapping_add(0x48c));
                if flag_zero {
                    break 'b_27048c;
                }
                c.cycles += 5;
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x488)).lw(c);
                c.g[S2][0] = 1;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x30)).lw(c);
                let v = c.g[V0][0];
                Addr(r32(c, S0).wrapping_add(0x484)).sw(c, v);
                'l_270458: loop {
                    'b_270470: {
                        c.cycles += 3;
                        c.g[A0][0] = Addr(r32(c, S1)).lw(c);
                        if c.g[A0][0] == 0 {
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                            break 'b_270470;
                        }
                        c.cycles += 3;
                        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x484)).lw(c);
                        Callee {
                            addr: 0x0026_d4d8,
                            f: ee::s07::f_0026d4d8,
                        }
                        .call(c, 0x0027_046c)?;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    }
                    c.cycles += 2;
                    let go_on = (c.g[S2][0] as i64) >= 0;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                    if go_on {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_0458);
                        }
                        continue 'l_270458;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = Addr(r32(c, A0).wrapping_add(0x488)).lw(c);
                    Callee {
                        addr: 0x0026_db88,
                        f: ee::s07::f_0026db88,
                    }
                    .call(c, 0x0027_0484)?;
                    frame.restore(c, S0, 0x30);
                    break 'b_270520;
                }
            }
            'b_270518: {
                c.cycles += 5;
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x484)).lw(c);
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x3cc)).lw(c);
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let below = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                if below {
                    break 'b_270518;
                }
                'b_270504: {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x61c)).lw(c);
                    if c.g[V0][0] == 0 {
                        let v = c.g[A0][0];
                        Addr(r32(c, S0).wrapping_add(0x484)).sw(c, v);
                        break 'b_270504;
                    }
                    c.cycles += 22;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    Callee {
                        addr: 0x0027_02f8,
                        f: ee::s07::f_002702f8,
                    }
                    .call(c, 0x0027_04b8)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 1;
                    Callee {
                        addr: 0x0027_03f0,
                        f: ee::s07::f_002703f0,
                    }
                    .call(c, 0x0027_04c4)?;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[S1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                    c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa8)).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0016_20d0, 0x0027_04e0)?;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x14)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0027_04f4)?;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0022_e840, 0x0027_04fc)?;
                    frame.restore(c, S0, 0x30);
                    break 'b_270520;
                }
                c.cycles += 5;
                c.g[A1][0] = 3;
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0026_f4a0,
                    f: ee::s07::f_0026f4a0,
                }
                .call(c, 0x0027_0510)?;
                frame.restore(c, S0, 0x30);
                break 'b_270520;
            }
            c.cycles += 2;
            let v = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(0x484)).sw(c, v);
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_OnRiderEvent(Replay* self, int rider, int kind)`
/// (0x2709d8): a rider event during a replay recording. Ignored while the
/// replay is seeking or rewinding; otherwise the highlight of the rider is
/// opened (kind 1) or the rider's empty hook runs (kind 0).
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn replay_on_rider_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        'b_270a9c: {
            'b_270a98: {
                c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x61c)).lw(c);
                let busy = c.g[V0][0] != 0;
                c.g[S2][0] = c.g[A2][0];
                if busy {
                    break 'b_270a98;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x620)).lw(c);
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'b_270a9c;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x610)).lw(c);
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'b_270a9c;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'b_270a9c;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0027_0730,
                    f: ee::s07::f_00270730,
                }
                .call(c, 0x0027_0a38)?;
                c.g[A3][0] = c.g[V0][0];
                let rider_missing = (c.g[A3][0] as i64) < 0;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                if rider_missing {
                    break 'b_270a98;
                }
                'b_270a88: {
                    'b_270a70: {
                        c.cycles += 6;
                        c.g[A1][0] = sx((c.g[S1][0] as u32) << 2);
                        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x84)).lw(c);
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        let kind_zero = c.g[S2][0] == 0;
                        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x28)).lw(c);
                        if kind_zero {
                            break 'b_270a70;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        if c.g[S2][0] == c.g[V0][0] {
                            c.g[A2][0] = Addr(r32(c, A0).wrapping_add(0x790)).lw(c);
                            break 'b_270a88;
                        }
                        c.cycles += 2;
                        frame.restore(c, S0, 0x30);
                        break 'b_270a9c;
                    }
                    c.cycles += 6;
                    c.g[A1][0] = Addr(r32(c, A0).wrapping_add(0x790)).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0xfc));
                    Callee {
                        addr: 0x0027_05a8,
                        f: ee::s07::f_002705a8,
                    }
                    .call(c, 0x0027_0a80)?;
                    frame.restore(c, S0, 0x30);
                    break 'b_270a9c;
                }
                c.cycles += 4;
                c.g[A1][0] = c.g[A3][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0xfc));
                Callee {
                    addr: 0x0027_0b88,
                    f: ee::s07::f_00270b88,
                }
                .call(c, 0x0027_0a98)?;
                break 'b_270a98;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `Highlight* Replay_FindWeakestHighlight(Replay* self, int i, Highlight*
/// best, int positive)` (0x270c58): scan the highlight list for the entry
/// with the weakest value in column `i` (`positive`: the strongest). Leaf: no
/// frame, no calls. Returns `best` (or the element found) in v0.
pub fn replay_find_weakest_highlight(c: &mut Cpu) {
    'b_270ce0: {
        c.cycles += 3;
        c.g[T0][0] = Addr(r32(c, A0).wrapping_add(0x3b0)).lw(c);
        let empty = c.g[T0][0] == 0;
        c.g[T1][0] = c.g[A2][0];
        if empty {
            break 'b_270ce0;
        }
        c.cycles += 1;
        c.g[V0][0] = Addr(r32(c, T0).wrapping_add(0x1e)).lbu(c);
        'l_270c68: loop {
            'b_270cd8: {
                c.cycles += 4;
                c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> ((c.g[A1][0] as u32) & 31)) as u32);
                c.g[V0][0] &= 1;
                if c.g[V0][0] == 0 {
                    c.g[T0][0] = Addr(r32(c, T0).wrapping_add(0x14)).lw(c);
                    break 'b_270cd8;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, T0).wrapping_add(0x1f)).lbu(c);
                if c.g[V0][0] != 0 {
                    c.g[T0][0] = Addr(r32(c, T0).wrapping_add(0x14)).lw(c);
                    break 'b_270cd8;
                }
                'b_270cd4: {
                    'b_270cc4: {
                        'b_270ca8: {
                            c.cycles += 2;
                            let positive = c.g[A3][0] == 0;
                            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                            if positive {
                                break 'b_270ca8;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = sx(r32(c, T0).wrapping_add(c.g[V1][0] as u32));
                            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x20)).lw(c);
                            c.g[V1][0] = sx(r32(c, A2).wrapping_add(c.g[V1][0] as u32));
                            if (c.g[A0][0] as i64) <= 0 {
                                break 'b_270cd4;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x20)).lw(c);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                            break 'b_270cc4;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, T0).wrapping_add(c.g[V1][0] as u32));
                        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x20)).lw(c);
                        if (c.g[A0][0] as i64) >= 0 {
                            c.g[T0][0] = Addr(r32(c, T0).wrapping_add(0x14)).lw(c);
                            break 'b_270cd8;
                        }
                        c.cycles += 3;
                        c.g[V1][0] = sx(r32(c, A2).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x20)).lw(c);
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[T0][0] = Addr(r32(c, T0).wrapping_add(0x14)).lw(c);
                        break 'b_270cd8;
                    }
                    c.cycles += 2;
                    c.g[T1][0] = c.g[T0][0];
                    c.g[A2][0] = c.g[T1][0];
                }
                c.cycles += 1;
                c.g[T0][0] = Addr(r32(c, T0).wrapping_add(0x14)).lw(c);
            }
            c.cycles += 2;
            if c.g[T0][0] != 0 {
                c.g[V0][0] = Addr(r32(c, T0).wrapping_add(0x1e)).lbu(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_0c68);
                }
                continue 'l_270c68;
            }
            break 'b_270ce0;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T1][0];
    abi::ret(c);
}

/// `int Replay_LoadSaved(Replay* self, ReplaySave* save)` (0x271758): rebuild
/// the replay's snapshot list from a saved replay. Kept as a port without a
/// test: its callees allocate and deserialise through the real heap and file
/// code, which a fixture cannot bound yet.
#[allow(dead_code)]
pub fn replay_load_saved(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        c.g[S4][0] = c.g[SP][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
        c.g[A2][0] = Addr(r32(c, S3).wrapping_add(0x18)).lw(c);
        let a1 = c.g[A1][0];
        Addr(sp(c)).sw(c, a1);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0xc)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x7c)).lw(c);
        let mismatch = c.g[A2][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        'b_271880: {
            if mismatch {
                break 'b_271880;
            }
            'b_271838: {
                c.cycles += 22;
                c.g[V0][0] = 1;
                c.g[A0][0] = sx(r32(c, S2).wrapping_add(0x3b0));
                Addr(r32(c, S2).wrapping_add(0x608)).sw(c, 1);
                c.g[S5][0] = c.g[A0][0];
                c.g[S1][0] = 0;
                c.g[V1][0] = Addr(r32(c, S3).wrapping_add(8)).lw(c);
                Addr(r32(c, S2).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0xc)).lw(c);
                c.g[V0][0] = (0 < c.g[V0][0]) as u64;
                Addr(r32(c, S2).wrapping_add(0x610)).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr(r32(c, S3).wrapping_add(0x10)).lw(c);
                Addr(r32(c, S2).wrapping_add(0x3c8)).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x14)).lw(c);
                Addr(r32(c, S2).wrapping_add(0x3d0)).sw(c, 0);
                Addr(r32(c, S2).wrapping_add(0x484)).sw(c, 0);
                Addr(r32(c, S2).wrapping_add(0x3cc)).sw(c, c.g[V0][0]);
                Callee {
                    addr: 0x0026_e5c0,
                    f: ee::s07::f_0026e5c0,
                }
                .call(c, 0x0027_17e8)?;
                c.g[V0][0] = sx(r32(c, S3).wrapping_add(0x28));
                Addr(sp(c)).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr(r32(c, S3).wrapping_add(4)).lw(c);
                if (c.g[V1][0] as i64) <= 0 {
                    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x18)).lw(c);
                    break 'b_271838;
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[S2][0];
                'l_271800: loop {
                    c.cycles += 13;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    Callee {
                        addr: 0x0027_0130,
                        f: ee::s07::f_00270130,
                    }
                    .call(c, 0x0027_1808)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0026_d678,
                        f: ee::s07::f_0026d678,
                    }
                    .call(c, 0x0027_1818)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0026_e5d0,
                        f: ee::s07::f_0026e5d0,
                    }
                    .call(c, 0x0027_1824)?;
                    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(4)).lw(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.g[S2][0];
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_1800);
                        }
                        continue 'l_271800;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x18)).lw(c);
                    break 'b_271838;
                }
            }
            'b_271870: {
                c.cycles += 2;
                let none = (c.g[V0][0] as i64) <= 0;
                c.g[S1][0] = 0;
                if none {
                    break 'b_271870;
                }
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S2).wrapping_add(0x48c));
                'l_271848: loop {
                    'b_271860: {
                        c.cycles += 3;
                        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                        if c.g[A0][0] == 0 {
                            c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x18)).lw(c);
                            break 'b_271860;
                        }
                        c.cycles += 3;
                        c.g[A1][0] = c.g[S4][0];
                        Callee {
                            addr: 0x0026_d558,
                            f: ee::s07::f_0026d558,
                        }
                        .call(c, 0x0027_185c)?;
                        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x18)).lw(c);
                    }
                    c.cycles += 4;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let again = c.g[V0][0] != 0;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                    if again {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_1848);
                        }
                        continue 'l_271848;
                    }
                    break 'b_271870;
                }
            }
            c.cycles += 4;
            c.g[V0][0] = 1;
            c.g[V1][0] = 8;
            Addr(r32(c, S2).wrapping_add(0x618)).sw(c, c.g[V0][0]);
            Addr(r32(c, S2)).sw(c, c.g[V1][0]);
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `c.c.eq.s f0, f20` followed by the FCC latch: the compare result.
fn c_eq_f0_f20(c: &mut Cpu) -> bool {
    let eq = FpuF32(c.f[0]).eq_s(FpuF32(c.f[20]));
    abi::set_fcc(c, eq);
    eq
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, DATA};

    /// The entry return address the fixtures give every call.
    const RET: u64 = crate::difftest::RA as u64;





    #[test]
    fn replay_find_weakest_highlight_matches() {
        const SELF: u32 = DATA + 0x30_2000;
        const LIST: u32 = DATA + 0x30_2100;
        const BEST: u32 = DATA + 0x30_2800;
        check(
            ee::s07::f_00270c58__generated,
            replay_find_weakest_highlight,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x27_0c58, 0);
                c.pc = 0x0027_0c58;
                c.g[RA][0] = RET;
                let n = seed % 5;
                for k in 0..n {
                    let e = LIST + 0x40 * k;
                    let next = if k + 1 < n { e + 0x40 } else { 0 };
                    c.bus.write32(e + 0x14, next);
                    c.bus.write8(e + 0x1e, r.next_u32() as u8);
                    c.bus.write8(e + 0x1f, (r.next_u32() % 3) as u8 & 1);
                    c.bus.write32(e + 0x20, r.next_u32());
                }
                c.bus.write32(SELF + 0x3b0, if n == 0 { 0 } else { LIST });
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = (seed % 32) as u64;
                c.g[A2][0] = BEST as u64;
                c.g[A3][0] = (seed % 2) as u64;
            },
        );
    }



    #[test]
    fn replay_update_camera_input_matches() {
        const THIS: u32 = DATA + 0x30_4000;
        const WORLD: u32 = DATA + 0x30_5000;
        const CAM: u32 = DATA + 0x30_5800;
        const OBJ: u32 = DATA + 0x30_6000;
        check(
            ee::s07::f_0026fa78__generated,
            replay_update_camera_input,
            40,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x26_fa78, 0);
                c.pc = 0x0026_fa78;
                c.g[RA][0] = RET;
                c.bus.write32(THIS, if seed % 4 == 0 { 2 } else { 3 });
                c.bus.write32(THIS + 0x28, seed % 4);
                c.bus.write32(THIS + 0x630, seed % 2);
                c.bus.write32(GAME_GP_848, WORLD);
                c.bus.write32(WORLD + 0x84, CAM);
                c.bus.write32(CAM + 0x84, OBJ);
                c.bus.write32(WORLD + 4 * (seed % 4) + 0xb0, OBJ);
                c.bus.write32(0x0044_5438 + 4 * (seed % 2), if seed % 3 == 0 { 0xb } else { 0 });
                c.bus.write32(OBJ + 0x84, r.next_u32());
                c.g[A0][0] = THIS as u64;
            },
        );
    }





    /// The gp-relative world pointer (`GAME_GP - 0x848`).
    const GAME_GP_848: u32 = crate::difftest::GAME_GP - 0x848;
}
