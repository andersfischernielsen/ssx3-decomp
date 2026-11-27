//! Staging module oc956 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc956`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001e_8200,
    ov_screen_finish_ov2_init,
    "OVScreenFinishOv2_Init",
)];

/// `OVScreenFinishOv2_Init(self)` (0x1e8200; `cOVState_BASSSP_QUAL_FINISHOV_onCreateScreen`
/// in the peak3 fork): builds the race-finish overlay screen. Adds the screen
/// by hash name to the UI engine (`self+0x10`), stores it at `self+0x40` and
/// plays its first frame, clears `self+0x9c`, flags one object (`+0x90 |= 8`),
/// then hides and re-shows the mode objects: the four result banners by the
/// game mode (`*(world+0xc0)`, unless the event type byte `0x535c10` is 6),
/// the four per-player prompts by the interface 0xd value (+0x14) and the
/// event class byte (`0x535c12`), the six rank markers (`sprintf` of a
/// format, visible only for the player's slot `table[0x536730 + 4*player]`,
/// +1), and a "no rank" marker when the slot is 6 or more. It fills the
/// score text object from a localised format string and the player's score
/// (`0x536640`), shows the other object when interface 8 has a non-negative
/// value (+0x18/+0x1c) and finally hides two result objects for a few
/// situations (class 2, a flag at the player's rider +0x480, event types 9
/// to 11, or mode 1 with +0x70 == 3). Ends by calling `0x294f78` with `0xe`.
/// The body keeps the generated block structure (labels are generated block
/// addresses); the cycle charges are the generated ones.
pub fn ov_screen_finish_ov2_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e826c: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x270));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x230));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x250));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x240));
            c.g[A0][0] = 0x470000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x220));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1cc0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x210));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x200));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x1f0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x1e0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x1d0), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x260));
            c.g[RA][0] = sx(0x001e_8240);
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x0031_7670, 0x001e_8240)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001e_8254);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x0039_7cc0, 0x001e_8254)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1e826c;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001e_826c);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039_c870, 0x001e_826c)?;
                break 'b_1e826c;
            }
        }
        'b_1e82a8: {
            c.cycles += 12;
            c.g[RA][0] = sx(0x001e_8274);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0020_a380, 0x001e_8274)?;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x9c), 0u64);
            c.g[A0][0] = 0x470000;
            c.g[RA][0] = sx(0x001e_8284);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x17e8));
            call_ptr(c, 0x0031_7670, 0x001e_8284)?;
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001e_8290);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001e_8290)?;
            c.g[V1][0] = c.g[V0][0];
            let t = c.g[V1][0] == 0u64;

            if t {
                break 'b_1e82a8;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = c.g[V0][0] | 0x8;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x90), c.g[V0][0]);
                break 'b_1e82a8;
            }
        }
        'b_1e83c0: {
            'b_1e83bc: {
                c.cycles += 49;
                c.g[RA][0] = sx(0x001e_82b0);
                c.g[S0][0] = 0x530000;
                call_ptr(c, 0x0014_dc80, 0x001e_82b0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e_82bc);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014_dd58, 0x001e_82bc)?;
                c.g[RA][0] = sx(0x001e_82c4);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5bc8));
                call_ptr(c, 0x0014_dc80, 0x001e_82c4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e_82d0);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014_dd58, 0x001e_82d0)?;
                c.g[RA][0] = sx(0x001e_82d8);
                c.g[S7][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dc80, 0x001e_82d8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e_82e4);
                c.g[A1][0] = 0x8;
                call_ptr(c, 0x0014_dd58, 0x001e_82e4)?;
                c.g[RA][0] = sx(0x001e_82ec);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                call_ptr(c, 0x0014_dc80, 0x001e_82ec)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e_82f8);
                c.g[A1][0] = 0xd;
                call_ptr(c, 0x0014_dd58, 0x001e_82f8)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V1][0] = 0x470000;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x17d8));
                c.g[A0][0] = c.g[S3][0];
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                c.g[A1][0] = c.g[S6][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c4), c.g[V0][0]);
                c.g[RA][0] = sx(0x001e_8320);
                c.g[S1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x48));
                call_ptr(c, 0x0039_e8b8, 0x001e_8320)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S3][0];
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x17c8));
                c.g[RA][0] = sx(0x001e_8334);
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x0039_e8b8, 0x001e_8334)?;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x001e_8344);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17b8));
                call_ptr(c, 0x0039_e8b8, 0x001e_8344)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S3][0];
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x17a8));
                c.g[RA][0] = sx(0x001e_8358);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_e8b8, 0x001e_8358)?;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c4));
                c.g[S4][0] = c.lw((c.g[V0][0] as u32));
                c.g[V0][0] = 0x6;
                let t = c.g[S1][0] == c.g[V0][0];
                c.g[V0][0] = 0x2;
                if t {
                    break 'b_1e83bc;
                } else {
                    'b_1e83b0: {
                        'b_1e83a0: {
                            'b_1e839c: {
                                c.cycles += 2;
                                let t = c.g[S4][0] == c.g[V0][0];
                                c.g[V0][0] = ((c.g[S4][0] as i64) < 3i64) as u64;
                                if t {
                                    break 'b_1e839c;
                                } else {
                                    'b_1e838c: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = 0x3;
                                        if t {
                                            break 'b_1e838c;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[S4][0] == c.g[V0][0];
                                            c.g[A0][0] = c.g[S7][0];
                                            if t {
                                                break 'b_1e83b0;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_1e83c0;
                                                } else {
                                                    break 'b_1e838c;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = (c.g[S4][0] as i64) < 0;
                                    c.g[A1][0] = c.g[S6][0];
                                    if t {
                                        break 'b_1e83bc;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_1e83a0;
                                        } else {
                                            break 'b_1e839c;
                                        }
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A1][0] = c.g[S5][0];
                            break 'b_1e83a0;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e_83a8);
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x0039_e948, 0x001e_83a8)?;
                        let t = true;
                        c.g[A0][0] = c.g[S7][0];
                        if t {
                            break 'b_1e83c0;
                        } else {
                            break 'b_1e83b0;
                        }
                    }
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001e_83bc);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0039_e948, 0x001e_83bc)?;
                    break 'b_1e83bc;
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[S7][0];
            break 'b_1e83c0;
        }
        'b_1e8448: {
            'b_1e8434: {
                'b_1e8430: {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x001e_83c8);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014_6e98, 0x001e_83c8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = c.g[A0][0] != 0u64;
                    c.g[S1][0] = c.lw((c.g[V1][0] as u32));
                    if t {
                        break 'b_1e8430;
                    } else {
                        'b_1e8410: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x3;
                            let t = c.g[S4][0] == c.g[V0][0];
                            c.g[V1][0] = 0x530000;
                            if t {
                                break 'b_1e8410;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                                c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                                c.g[V1][0] = 0x4;
                                let t = c.g[A0][0] == c.g[V1][0];
                                c.g[V0][0] = 0x5;
                                if t {
                                    break 'b_1e8410;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A0][0] != c.g[V0][0];
                                    c.g[A1][0] = 0x470000;
                                    if t {
                                        break 'b_1e8434;
                                    } else {
                                        break 'b_1e8410;
                                    }
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[V0][0] = 0x1;
                        c.g[A1][0] = 0x470000;
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0xfb4), c.g[V0][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1860));
                        c.g[RA][0] = sx(0x001e_8428);
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x0039_e948, 0x001e_8428)?;
                        let t = true;
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_1e8448;
                        } else {
                            break 'b_1e8430;
                        }
                    }
                }
                c.cycles += 1;
                c.g[A1][0] = 0x470000;
                break 'b_1e8434;
            }
            c.cycles += 5;
            c.sw((c.g[GP][0] as u32).wrapping_sub(0xfb4), 0u64);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1860));
            c.g[RA][0] = sx(0x001e_8444);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0039_e8b8, 0x001e_8444)?;
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
            break 'b_1e8448;
        }
        'b_1e849c: {
            'b_1e8490: {
                'b_1e848c: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        break 'b_1e848c;
                    } else {
                        'b_1e8478: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x3;
                            let t = c.g[S4][0] == c.g[V0][0];
                            c.g[A0][0] = 0x530000;
                            if t {
                                break 'b_1e8478;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                                c.g[A0][0] = 0x5;
                                let t = c.g[V1][0] != c.g[A0][0];
                                c.g[A0][0] = c.g[S3][0];
                                if t {
                                    break 'b_1e8490;
                                } else {
                                    c.cycles += 1;
                                    c.g[A1][0] = 0x4a0000;
                                    break 'b_1e8478;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[RA][0] = sx(0x001e_8484);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2180));
                        call_ptr(c, 0x0039_e948, 0x001e_8484)?;
                        let t = true;
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_1e849c;
                        } else {
                            break 'b_1e848c;
                        }
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[S3][0];
                break 'b_1e8490;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x001e_8498);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2180));
            call_ptr(c, 0x0039_e8b8, 0x001e_8498)?;
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
            break 'b_1e849c;
        }
        'b_1e84f0: {
            'b_1e84e0: {
                c.cycles += 3;
                c.g[V0][0] = 0x2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A1][0] = 0x4a0000;
                    break 'b_1e84e0;
                } else {
                    'b_1e84c8: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = c.g[S4][0] == c.g[V0][0];
                        c.g[V1][0] = 0x530000;
                        if t {
                            break 'b_1e84c8;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = 0x5;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                            c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                            if c.g[V1][0] != c.g[A0][0] {
                                c.g[A1][0] = 0x4a0000;
                                break 'b_1e84e0;
                            } else {
                                break 'b_1e84c8;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001e_84d8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2188));
                    call_ptr(c, 0x0039_e948, 0x001e_84d8)?;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                    if t {
                        break 'b_1e84f0;
                    } else {
                        break 'b_1e84e0;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001e_84ec);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2188));
            call_ptr(c, 0x0039_e8b8, 0x001e_84ec)?;
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
            break 'b_1e84f0;
        }
        'b_1e8540: {
            'b_1e8530: {
                c.cycles += 3;
                c.g[V0][0] = 0x3;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A1][0] = 0x4a0000;
                    break 'b_1e8530;
                } else {
                    'b_1e851c: {
                        c.cycles += 2;
                        let t = c.g[S4][0] == c.g[V1][0];
                        c.g[A1][0] = 0x4a0000;
                        if t {
                            break 'b_1e851c;
                        } else {
                            c.cycles += 6;
                            c.g[A0][0] = 0x530000;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                            c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                            c.g[A0][0] = 0x5;
                            let t = c.g[V1][0] != c.g[A0][0];

                            if t {
                                break 'b_1e8530;
                            } else {
                                break 'b_1e851c;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001e_8528);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2190));
                    call_ptr(c, 0x0039_e948, 0x001e_8528)?;
                    let t = true;
                    c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
                    if t {
                        break 'b_1e8540;
                    } else {
                        break 'b_1e8530;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001e_853c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2190));
            call_ptr(c, 0x0039_e8b8, 0x001e_853c)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
            break 'b_1e8540;
        }
        c.cycles += 8;
        c.g[S5][0] = (c.g[S1][0] < 0x6u64) as u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c8), c.g[S0][0]);
        c.g[FP][0] = 0x4a0000;
        c.g[S0][0] = 0x1;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[A0][0] = c.g[SP][0];
        'l_1e8560: loop {
            'b_1e85c0: {
                c.cycles += 17;
                c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2198));
                c.g[RA][0] = sx(0x001e_856c);
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x0041_7828, 0x001e_856c)?;
                c.g[RA][0] = sx(0x001e_8574);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0031_7670, 0x001e_8574)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001e_8580);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001e_8580)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001e_859c);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001e_859c)?;
                if c.g[S0][0] != c.g[S1][0] {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    break 'b_1e85c0;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e_85bc);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e_85bc)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    break 'b_1e85c0;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 7i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[A0][0] = c.g[SP][0];
                if event_due(c) {
                    deliver_event_here(c, 0x001e_8560);
                }
                continue 'l_1e8560;
            } else {
                'b_1e85e8: {
                    c.cycles += 2;
                    if c.g[S5][0] != 0u64 {
                        c.g[A0][0] = 0x470000;
                        break 'b_1e85e8;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[RA][0] = sx(0x001e_85e4);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x21a8));
                        call_ptr(c, 0x0039_e948, 0x001e_85e4)?;
                        c.g[A0][0] = 0x470000;
                        break 'b_1e85e8;
                    }
                }
                'b_1e86a8: {
                    'b_1e86a4: {
                        c.cycles += 43;
                        c.g[RA][0] = sx(0x001e_85f0);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1798));
                        call_ptr(c, 0x0031_7670, 0x001e_85f0)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001e_85fc);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_d860, 0x001e_85fc)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[S2][0] = c.g[V0][0];
                        c.g[A0][0] = 0x470000;
                        c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1768));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x001e_8620);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        call_ptr(c, 0x0031_7670, 0x001e_8620)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001e_8630);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, t, 0x001e_8630)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001e_863c);
                        c.g[A0][0] = c.g[S6][0];
                        call_ptr(c, 0x002c_2508, 0x001e_863c)?;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c8));
                        c.g[V1][0] = 0x530000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                        c.g[A1][0] = c.g[S6][0];
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[A0][0] = c.g[S7][0];
                        c.g[RA][0] = sx(0x001e_865c);
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32));
                        call_ptr(c, 0x002c_26d0, 0x001e_865c)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001e_8668);
                        c.g[A1][0] = c.g[S7][0];
                        call_ptr(c, 0x003a_0e90, 0x001e_8668)?;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x001e_8674);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1788));
                        call_ptr(c, 0x0031_7670, 0x001e_8674)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001e_8680);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_d860, 0x001e_8680)?;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c0));
                        c.g[S2][0] = c.g[V0][0];
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                        let t = (c.g[V1][0] as i64) >= 0;
                        c.g[A0][0] = 0u64;
                        if t {
                            break 'b_1e86a4;
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c0));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                            let t = (c.g[V0][0] as i64) < 0;

                            if t {
                                break 'b_1e86a8;
                            } else {
                                break 'b_1e86a4;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = 0x1;
                    break 'b_1e86a8;
                }
                'b_1e86e8: {
                    'b_1e86cc: {
                        c.cycles += 2;
                        let t = c.g[A0][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_1e86cc;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001e_86c4);
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001e_86c4)?;
                            let t = true;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            if t {
                                break 'b_1e86e8;
                            } else {
                                break 'b_1e86cc;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e_86e4);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e_86e4)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    break 'b_1e86e8;
                }
                'b_1e8774: {
                    'b_1e8758: {
                        'b_1e8754: {
                            c.cycles += 10;
                            c.g[V0][0] = 0x530000;
                            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                            c.g[A1][0] = 0x2;
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                            c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x49));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            let t = c.g[A2][0] == c.g[A1][0];
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                            if t {
                                break 'b_1e8754;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x480));
                                if c.g[V0][0] != 0u64 {
                                    c.g[A1][0] = 0x470000;
                                    break 'b_1e8758;
                                } else {
                                    c.cycles += 4;
                                    c.g[S0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4a));
                                    c.g[V0][0] = 0x9;
                                    let t = c.g[S0][0] == c.g[V0][0];
                                    c.g[V0][0] = 0xa;
                                    if t {
                                        break 'b_1e8754;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S0][0] == c.g[V0][0];
                                        c.g[V0][0] = 0xb;
                                        if t {
                                            break 'b_1e8754;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[S0][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_1e8754;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S4][0] != c.g[V0][0];
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x1c4));
                                                if t {
                                                    break 'b_1e8774;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = 0x3;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[A0][0] as u32).wrapping_add(0x70));
                                                    let t = c.g[V1][0] != c.g[V0][0];

                                                    if t {
                                                        break 'b_1e8774;
                                                    } else {
                                                        break 'b_1e8754;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A1][0] = 0x470000;
                        break 'b_1e8758;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001e_8764);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1778));
                    call_ptr(c, 0x0039_e8b8, 0x001e_8764)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001e_8774);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x21a0));
                    call_ptr(c, 0x0039_e8b8, 0x001e_8774)?;
                    break 'b_1e8774;
                }
                c.cycles += 17;
                c.g[RA][0] = sx(0x001e_877c);
                call_ptr(c, 0x0028_b180, 0x001e_877c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001e_8788);
                c.g[A1][0] = 0xe;
                call_ptr(c, 0x0029_4f78, 0x001e_8788)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x260));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x250));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x240));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x230));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x220));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1d0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x270));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{fresh, run_generated, Rng, DATA, GAME_GP};

    /// Gate word of this module's callee fakes (machine RAM, never
    /// `DATA+0x100000`), and the mailbox of the results they force.
    const OC956_CTRL: u32 = DATA + 0x0020_0000;
    const OC956_MAGIC: u32 = 0x0956_c0de;
    const OC956_BOX: u32 = DATA + 0x0020_0100;
    const OC956_BOX_SCREEN: u32 = 0;
    const OC956_BOX_FIRST_OBJ: u32 = 1;
    const OC956_BOX_OBJ: u32 = 2;
    const OC956_BOX_IF8: u32 = 3;
    const OC956_BOX_IFD: u32 = 4;
    const OC956_BOX_PLAYER: u32 = 5;
    const OC956_BOX_OBJ_CALLS: u32 = 6;

    // Fixture layout (inside DATA..DATA+0xfffff, above the fakes' pointers).
    const OC956_SELF: u32 = DATA + 0x8_0000;
    const OC956_SCREEN: u32 = DATA + 0x8_0200;
    const OC956_OBJ: u32 = DATA + 0x8_0300;
    const OC956_OBJ_VT: u32 = DATA + 0x8_0400;
    const OC956_IF8: u32 = DATA + 0x8_0500;
    const OC956_IFD: u32 = DATA + 0x8_0600;
    const OC956_WORLD: u32 = DATA + 0x8_0700;
    const OC956_GAME: u32 = DATA + 0x8_0800;
    const OC956_RIDERS: u32 = DATA + 0x8_0900;
    const OC956_MODE: u32 = DATA + 0x8_0b00;
    const OC956_LOC: u32 = DATA + 0x8_0c00;
    const OC956_LOC_VT: u32 = DATA + 0x8_0d00;
    const OC956_RIDER: u32 = DATA + 0x8_1000;
    const OC956_G_EVENT: u32 = 0x0053_5bc8;
    const OC956_G_SLOTS: u32 = 0x0053_6730;
    const OC956_G_SCORES: u32 = 0x0053_6640;

    /// A fake call that forces the results the body dereferences: the screen
    /// (397cc0), the object lookups (39d860: the first may be null, the rest
    /// are the shared object), interfaces 8 and 0xd (14dd58) and the player
    /// (146e98). Everything else returns the fake's hash pointer.
    fn oc956_fake_call(c: &mut Cpu, target: u32) {
        let ret = c.g[RA][0] as u32;
        let a1 = c.g[A1][0] as u32;
        fakes::fake_call(c, target);
        if c.pc != ret {
            return;
        }
        let slot = |k: u32| OC956_BOX + 4 * k;
        let v = match target {
            0x0039_7cc0 => Some(c.bus.read32(slot(OC956_BOX_SCREEN))),
            0x0039_d860 => {
                let n = c.bus.read32(slot(OC956_BOX_OBJ_CALLS));
                c.bus.write32(slot(OC956_BOX_OBJ_CALLS), n + 1);
                let k = if n == 0 {
                    OC956_BOX_FIRST_OBJ
                } else {
                    OC956_BOX_OBJ
                };
                Some(c.bus.read32(slot(k)))
            }
            0x0014_dd58 if a1 == 8 => Some(c.bus.read32(slot(OC956_BOX_IF8))),
            0x0014_dd58 if a1 == 0xd => Some(c.bus.read32(slot(OC956_BOX_IFD))),
            0x0014_6e98 => Some(c.bus.read32(slot(OC956_BOX_PLAYER))),
            _ => None,
        };
        if let Some(v) = v {
            c.g[V0][0] = v as u64;
        }
    }

    fn oc956_fake_317670(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_317670),
                ee::s10::f_00317670__generated,
            );
        }
        oc956_fake_call(c, 0x0031_7670);
    }

    fn oc956_fake_397cc0(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_397cc0),
                ee::s13::f_00397cc0__generated,
            );
        }
        oc956_fake_call(c, 0x0039_7cc0);
    }

    fn oc956_fake_39c870(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_39c870),
                ee::s13::f_0039c870__generated,
            );
        }
        oc956_fake_call(c, 0x0039_c870);
    }

    fn oc956_fake_20a380(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_20a380),
                ee::s05::f_0020a380__generated,
            );
        }
        oc956_fake_call(c, 0x0020_a380);
    }

    fn oc956_fake_39d860(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_39d860),
                ee::s13::f_0039d860__generated,
            );
        }
        oc956_fake_call(c, 0x0039_d860);
    }

    fn oc956_fake_14dc80(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_14dc80),
                ee::s01::f_0014dc80__generated,
            );
        }
        oc956_fake_call(c, 0x0014_dc80);
    }

    fn oc956_fake_14dd58(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_14dd58),
                ee::s01::f_0014dd58__generated,
            );
        }
        oc956_fake_call(c, 0x0014_dd58);
    }

    fn oc956_fake_39e8b8(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_39e8b8),
                ee::s13::f_0039e8b8__generated,
            );
        }
        oc956_fake_call(c, 0x0039_e8b8);
    }

    fn oc956_fake_39e948(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_39e948),
                ee::s13::f_0039e948__generated,
            );
        }
        oc956_fake_call(c, 0x0039_e948);
    }

    fn oc956_fake_146e98(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_146e98),
                ee::s01::f_00146e98__generated,
            );
        }
        oc956_fake_call(c, 0x0014_6e98);
    }

    fn oc956_fake_417828(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_417828),
                ee::s15::f_00417828__generated,
            );
        }
        oc956_fake_call(c, 0x0041_7828);
    }

    fn oc956_fake_2c2508(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_2c2508),
                ee::s08::f_002c2508__generated,
            );
        }
        oc956_fake_call(c, 0x002c_2508);
    }

    fn oc956_fake_2c26d0(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_2c26d0),
                ee::s08::f_002c26d0__generated,
            );
        }
        oc956_fake_call(c, 0x002c_26d0);
    }

    fn oc956_fake_3a0e90(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_3a0e90),
                ee::s13::f_003a0e90__generated,
            );
        }
        oc956_fake_call(c, 0x003a_0e90);
    }

    fn oc956_fake_294f78(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_294f78),
                ee::s08::f_00294f78__generated,
            );
        }
        oc956_fake_call(c, 0x0029_4f78);
    }

    fn oc956_fake_28b180(c: &mut Cpu) {
        if c.bus.read32(OC956_CTRL) != OC956_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc956_fake_28b180),
                ee::s07::f_0028b180__generated,
            );
        }
        oc956_fake_call(c, 0x0028_b180);
    }

    fn oc956_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc956_fake_317670,
            fakes::prev_cell(oc956_fake_317670),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_00397CC0),
            oc956_fake_397cc0,
            fakes::prev_cell(oc956_fake_397cc0),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc956_fake_39c870,
            fakes::prev_cell(oc956_fake_39c870),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s05::OVERRIDE_0020A380),
            oc956_fake_20a380,
            fakes::prev_cell(oc956_fake_20a380),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc956_fake_39d860,
            fakes::prev_cell(oc956_fake_39d860),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc956_fake_14dc80,
            fakes::prev_cell(oc956_fake_14dc80),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc956_fake_14dd58,
            fakes::prev_cell(oc956_fake_14dd58),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039E8B8),
            oc956_fake_39e8b8,
            fakes::prev_cell(oc956_fake_39e8b8),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039E948),
            oc956_fake_39e948,
            fakes::prev_cell(oc956_fake_39e948),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_00146E98),
            oc956_fake_146e98,
            fakes::prev_cell(oc956_fake_146e98),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc956_fake_417828,
            fakes::prev_cell(oc956_fake_417828),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s08::OVERRIDE_002C2508),
            oc956_fake_2c2508,
            fakes::prev_cell(oc956_fake_2c2508),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s08::OVERRIDE_002C26D0),
            oc956_fake_2c26d0,
            fakes::prev_cell(oc956_fake_2c26d0),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003A0E90),
            oc956_fake_3a0e90,
            fakes::prev_cell(oc956_fake_3a0e90),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s08::OVERRIDE_00294F78),
            oc956_fake_294f78,
            fakes::prev_cell(oc956_fake_294f78),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            oc956_fake_28b180,
            fakes::prev_cell(oc956_fake_28b180),
        );
    }

    fn oc956_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// The overlay state with random mode, event bytes, interface values,
    /// player slot and score tables; `seed` also picks null first object /
    /// null screen cases. `calls` bounds the call index at which a fake unwinds.
    fn oc956_fix(c: &mut Cpu, seed: u32, calls: u32) {
        oc956_install();
        let mut r: Rng = fakes::machine(c, seed, 0x956, calls);
        oc956_w(c, OC956_CTRL, OC956_MAGIC);
        c.g[A0][0] = OC956_SELF as u64;

        // The world (gp-0x848): game mode, localisation, riders.
        oc956_w(c, GAME_GP.wrapping_sub(0x848), OC956_WORLD);
        oc956_w(c, OC956_WORLD + 0xc0, OC956_MODE);
        oc956_w(c, OC956_WORLD + 0x8c, OC956_LOC);
        oc956_w(c, OC956_WORLD + 0x84, OC956_GAME);
        oc956_w(c, OC956_GAME + 0xc, OC956_RIDERS);
        let mode = r.next_u32() % 5;
        oc956_w(c, OC956_MODE, mode);
        let m70 = if r.next_u32() & 1 != 0 {
            3
        } else {
            r.next_u32() % 5
        };
        oc956_w(c, OC956_MODE + 0x70, m70);
        oc956_w(c, OC956_LOC + 4, OC956_LOC_VT);
        c.bus.write16(OC956_LOC_VT + 0x20, 0);
        oc956_w(c, OC956_LOC_VT + 0x24, FAKE + 0x50);
        for i in 0..8u32 {
            let rider = OC956_RIDER + 0x600 * i;
            oc956_w(c, OC956_RIDERS + 4 * i + 0x28, rider);
            let flag = if r.next_u32() % 3 == 0 {
                0
            } else {
                r.next_u32() | 1
            };
            oc956_w(c, rider + 0x480, flag);
        }

        // Event bytes and the per-player tables.
        let ev48 = if r.next_u32() % 3 == 0 {
            6
        } else {
            r.next_u32() % 6
        };
        let b49 = if r.next_u32() % 3 == 0 {
            2
        } else {
            r.next_u32() % 4
        };
        let b4a = [4u8, 5, 9, 10, 11, 0, 3, 5][(r.next_u32() % 8) as usize];
        c.bus.write8(OC956_G_EVENT + 0x48, ev48 as u8);
        c.bus.write8(OC956_G_EVENT + 0x49, b49 as u8);
        c.bus.write8(OC956_G_EVENT + 0x4a, b4a);
        for i in 0..8u32 {
            let slot = r.next_u32() % 9;
            oc956_w(c, OC956_G_SLOTS + 4 * i, slot);
            let score = r.next_u32();
            oc956_w(c, OC956_G_SCORES + 4 * i, score);
        }

        // Interfaces 8 (two ints, either may be negative) and 0xd (+0x14: 0..4).
        let neg = |r: &mut Rng| {
            if r.next_u32() & 1 != 0 {
                r.next_u32() | 0x8000_0000
            } else {
                r.next_u32() & 0x7fff_ffff
            }
        };
        let (n18, n1c) = (neg(&mut r), neg(&mut r));
        oc956_w(c, OC956_IF8 + 0x18, n18);
        oc956_w(c, OC956_IF8 + 0x1c, n1c);
        oc956_w(c, OC956_IFD + 0x14, r.next_u32() % 5);

        // The screen and its objects (one vtable slot +0x4c, delta +0x48).
        oc956_w(c, OC956_OBJ + 8, OC956_OBJ_VT);
        c.bus.write16(OC956_OBJ_VT + 0x48, 0);
        oc956_w(c, OC956_OBJ_VT + 0x4c, FAKE + 0x40);
        oc956_w(c, OC956_OBJ + 0x90, r.next_u32() & 0xf0);

        // Forced results.
        let box_ = |k: u32| OC956_BOX + 4 * k;
        oc956_w(
            c,
            box_(OC956_BOX_SCREEN),
            if seed % 7 == 0 { 0 } else { OC956_SCREEN },
        );
        oc956_w(
            c,
            box_(OC956_BOX_FIRST_OBJ),
            if seed % 6 == 0 { 0 } else { OC956_OBJ },
        );
        oc956_w(c, box_(OC956_BOX_OBJ), OC956_OBJ);
        oc956_w(c, box_(OC956_BOX_IF8), OC956_IF8);
        oc956_w(c, box_(OC956_BOX_IFD), OC956_IFD);
        let player = r.next_u32() % 8;
        oc956_w(c, box_(OC956_BOX_PLAYER), player);
    }

    /// The finish overlay's init matches the generated body: every mode and
    /// event byte combination, the rank markers, and unwinds inside the calls.
    #[test]
    fn ov_screen_finish_ov2_init_matches() {
        fakes::covered(
            ee::s04::f_001e8200__generated,
            ov_screen_finish_ov2_init,
            200,
            &|c, seed| oc956_fix(c, seed, 70),
        );
    }

    /// The fixture reaches many distinct paths (total cycle costs of the
    /// generated body over the seeds, no unwinds).
    #[test]
    fn ov_screen_finish_ov2_init_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..400u32 {
            let mut c = fresh();
            oc956_fix(&mut c, seed, 0);
            let before = c.cycles;
            run_generated(&mut c, ee::s04::f_001e8200__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 20, "only {} paths: {seen:?}", seen.len());
    }
}
