//! Staging module oc855 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc855`.

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
        0x0019_86d0,
        cfestate_request_line_show_song_info,
        "cFEStateRequestLine_ShowSongInfo",
    ),
    (
        0x0019_8a88,
        cfestate_request_line_get_song_title,
        "cFEStateRequestLine_GetSongTitle",
    ),
    (
        0x0019_8ab8,
        cfestate_request_line_get_song_artist,
        "cFEStateRequestLine_GetSongArtist",
    ),
    (
        0x0019_8ae8,
        cfestate_request_line_get_song_price,
        "cFEStateRequestLine_GetSongPrice",
    ),
    (0x0019_8af0, oc855_fe_format_money, "FE_FormatMoney"),
    (
        0x0019_8db8,
        cfestate_char_equip_ctor,
        "cFEStateCharEquip_ctor",
    ),
];

/// `cFEStateRequestLine::ShowSongInfo(this, idx)` (0x1986d0): show the
/// highlighted song's "title - artist" line in the info text widget
/// (`this+0x1d4`) and, when song `idx` is not owned yet, its price in the
/// price widget (`this+0x1d8`); owned songs clear the price and hide the
/// widget at `this+0x1dc`.
pub fn cfestate_request_line_show_song_info(c: &mut Cpu) {
    /// Widget pointers of the request line.
    const INFO_TEXT: u32 = 0x1d4;
    const PRICE_TEXT: u32 = 0x1d8;
    const PRICE_BOX: u32 = 0x1dc;
    const OWNED_MASK: u32 = 0x160;
    const SHOWN_FLAG: u32 = 0x16c;
    const GET_ARTIST: Callee = Callee {
        addr: 0x0019_8ab8,
        f: ee::s03::cFEStateRequestLine_GetSongArtist,
    };
    const GET_PRICE: Callee = Callee {
        addr: 0x0019_8ae8,
        f: ee::s03::cFEStateRequestLine_GetSongPrice,
    };
    const FORMAT_MONEY: Callee = Callee {
        addr: 0x0019_8af0,
        f: ee::s03::FE_FormatMoney,
    };

    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 19, 0x20);
        frame.save(c, 20, 0x10);
        let this = c.g[A0][0];
        c.g[S3][0] = this;
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(INFO_TEXT)).lw(c);
        c.g[S4][0] = c.g[A1][0];
        'text_done: {
            if c.g[V0][0] != 0 {
                c.cycles += 47;
                let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[V0][0] = world;
                c.g[A0][0] = 0x460000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x770));
                c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8c)).lw(c);
                c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lh(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x0031_7670, 0x0019_871c)?;
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x24)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                let method = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, method, 0x0019_872c)?;
                c.g[A1][0] = c.g[S4][0];
                c.g[S2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                GET_ARTIST.call(c, 0x0019_873c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x003d_bca0, 0x0019_8748)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0041_6810, 0x0019_8754)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = 0x460000;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[A3][0] = 0;
                c.g[A0][0] = sx((c.g[S0][0] as u32) << 1);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x688));
                c.g[A2][0] = 0x100;
                call_ptr(c, 0x0031_7e30, 0x0019_8774)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x002c_26d0, 0x0019_8788)?;
                let widget = Addr(r32(c, S3).wrapping_add(INFO_TEXT)).lw(c);
                c.g[A2][0] = widget;
                c.g[A1][0] = 1;
                let vt = Addr((widget as u32).wrapping_add(8)).lw(c);
                c.g[V0][0] = vt;
                c.g[A0][0] = Addr((vt as u32).wrapping_add(0x48)).lh(c);
                c.g[V1][0] = Addr((vt as u32).wrapping_add(0x4c)).lw(c);
                let method = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0019_87a4)?;
                c.g[A0][0] = Addr(r32(c, S3).wrapping_add(INFO_TEXT)).lw(c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a_0e90, 0x0019_87b0)?;
                if c.g[S0][0] == 0 {
                    c.g[A0][0] = Addr(r32(c, S3).wrapping_add(PRICE_TEXT)).lw(c);
                    break 'text_done;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0031_7e98, 0x0019_87c0)?;
            }
            c.cycles += 1;
            c.g[A0][0] = Addr(r32(c, S3).wrapping_add(PRICE_TEXT)).lw(c);
        }
        // `a0` = the price widget.
        'price_done: {
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                frame.restore(c, 16, 0x50);
                break 'price_done;
            }
            'hide_box: {
                'price: {
                    'show_box: {
                        c.cycles += 7;
                        c.g[A2][0] = Addr(r32(c, S3).wrapping_add(OWNED_MASK)).ld(c);
                        c.g[V0][0] = c.g[A2][0] >> (c.g[S4][0] & 63);
                        c.g[V0][0] &= 1;
                        c.g[V0][0] <<= 32;
                        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                        c.g[A1][0] = 0x4a0000;
                        if c.g[V0][0] != 0 {
                            // Song owned: blank the price.
                            c.cycles += 5;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                            call_ptr(c, 0x003a_0c50, 0x0019_87f0)?;
                            c.g[A2][0] = Addr(r32(c, S3).wrapping_add(PRICE_BOX)).lw(c);
                            c.g[A1][0] = 0;
                            break 'show_box;
                        }
                        c.cycles += 3;
                        c.g[A1][0] = 0;
                        c.g[A0][0] = 0;
                        // Population count of the 64-bit owned mask.
                        loop {
                            c.cycles += 7;
                            c.g[V0][0] = c.g[A2][0] >> (c.g[A0][0] & 63);
                            c.g[V0][0] &= 1;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                            c.g[V1][0] = ((c.g[A0][0] as i64) < 64) as u64;
                            let more = c.g[V1][0] != 0;
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0019_8808);
                                }
                                continue;
                            }
                            break;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 6) as u64;
                        let few = c.g[V0][0] != 0;
                        c.g[A0][0] = 0x460000;
                        if !few {
                            break 'price;
                        }
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x780));
                        call_ptr(c, 0x0031_7670, 0x0019_8838)?;
                        c.g[A0][0] = Addr(r32(c, S3).wrapping_add(PRICE_TEXT)).lw(c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a_1000, 0x0019_8844)?;
                        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(SHOWN_FLAG)).lw(c);
                        c.g[A1][0] = 1;
                        if c.g[V0][0] == 0 {
                            break 'hide_box;
                        }
                        c.cycles += 1;
                        c.g[A2][0] = Addr(r32(c, S3).wrapping_add(PRICE_BOX)).lw(c);
                        break 'show_box;
                    }
                    // Show/hide the box widget through its vtable.
                    c.cycles += 7;
                    let widget = c.g[A2][0] as u32;
                    let vt = Addr(widget.wrapping_add(8)).lw(c);
                    c.g[V0][0] = vt;
                    c.g[A0][0] = Addr((vt as u32).wrapping_add(0x48)).lh(c);
                    c.g[V1][0] = Addr((vt as u32).wrapping_add(0x4c)).lw(c);
                    let method = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0019_8868)?;
                    frame.restore(c, 16, 0x50);
                    break 'price_done;
                }
                // Fewer than six songs owned: price line.
                c.cycles += 11;
                c.g[A1][0] = c.g[S4][0];
                c.g[A0][0] = c.g[S3][0];
                GET_PRICE.call(c, 0x0019_887c)?;
                c.g[A0][0] = c.g[V0][0];
                FORMAT_MONEY.call(c, 0x0019_8884)?;
                c.g[A0][0] = Addr(r32(c, S3).wrapping_add(PRICE_TEXT)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0c50, 0x0019_8890)?;
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(SHOWN_FLAG)).lw(c);
                c.g[A1][0] = 1;
                if c.g[V0][0] != 0 {
                    c.cycles += 6;
                    let widget = Addr(r32(c, S3).wrapping_add(PRICE_BOX)).lw(c);
                    c.g[A2][0] = widget;
                    let vt = Addr((widget as u32).wrapping_add(8)).lw(c);
                    c.g[V0][0] = vt;
                    c.g[A0][0] = Addr((vt as u32).wrapping_add(0x48)).lh(c);
                    c.g[V1][0] = Addr((vt as u32).wrapping_add(0x4c)).lw(c);
                    let method = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0019_88b4)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of `GetSongTitle`/`GetSongArtist`: `MusicPlayer_GetInstance`
/// (0x28b180) and then the music player's text getter `getter` on
/// `player + 0x118` with the song index.
fn oc855_song_text(c: &mut Cpu, getter: u32, ret_player: u32, ret_getter: u32) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0028_b180, ret_player)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
        call_ptr(c, getter, ret_getter)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `const char* cFEStateRequestLine::GetSongTitle(this, song)` (0x198a88):
/// the music player's title of song `song`.
pub fn cfestate_request_line_get_song_title(c: &mut Cpu) {
    oc855_song_text(c, 0x002b_40f0, 0x0019_8a9c, 0x0019_8aa8);
}

/// `const char* cFEStateRequestLine::GetSongArtist(this, song)` (0x198ab8):
/// the music player's artist of song `song`.
pub fn cfestate_request_line_get_song_artist(c: &mut Cpu) {
    oc855_song_text(c, 0x002b_4120, 0x0019_8acc, 0x0019_8ad8);
}

/// `int cFEStateRequestLine::GetSongPrice(this, song)` (0x198ae8): every
/// song costs 5000.
pub fn cfestate_request_line_get_song_price(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 5000;
    ret(c);
}

/// `char* FE_FormatMoney(int amount)` (0x198af0): format `amount` with the
/// locale's currency settings into the shared buffer at 0x4c6310 and return
/// it: the optional sign and `$` prefix, the digits grouped in threes with
/// the locale's separator, and the trailing `$` for locales that put the
/// symbol last.
pub fn oc855_fe_format_money(c: &mut Cpu) {
    /// Size of the locale record (at 0x535610) copied to the stack, less the
    /// last 8 bytes.
    const LOCALE_SIZE: u32 = 0x280;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x2f0);
        frame.save(c, 17, 0x2d0);
        frame.save(c, 18, 0x2c0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x2b0);
        c.g[S2][0] = 0x20;
        frame.save(c, 16, 0x2e0);
        frame.save(c, 20, 0x2a0);
        frame.save_ra(c, 0x290);
        c.g[S3][0] = 1;
        call_ptr(c, 0x0014_dc80, 0x0019_8b1c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_dd58, 0x0019_8b28)?;

        // Copy the locale record to the stack (0x20 bytes per round). The
        // record's address is a constant that is 8-byte aligned, so the
        // generated body's unaligned (`ldl`/`ldr`) copy loop is never taken
        // and is not reproduced here.
        c.g[V1][0] = 0x530000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
        c.g[V0][0] = c.g[V1][0] & 7;
        c.g[A0][0] = c.g[SP][0];
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(LOCALE_SIZE));
        c.g[S4][0] = 0x4c0000;
        c.g[A1][0] = 0x4a0000;
        loop {
            c.cycles += 11;
            let src = c.g[V1][0] as u32;
            let dst = c.g[A0][0] as u32;
            for (i, reg) in [A2, A3, T0, T1].into_iter().enumerate() {
                c.g[reg][0] = Addr(src.wrapping_add(8 * i as u32)).ld(c);
            }
            for (i, reg) in [A2, A3, T0, T1].into_iter().enumerate() {
                let v = c.g[reg][0];
                Addr(dst.wrapping_add(8 * i as u32)).sd(c, v);
            }
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0019_8ba8);
                }
                continue;
            }
            break;
        }
        // The last 8 bytes.
        c.cycles += 10;
        let v = Addr(c.g[V1][0] as u32).read_u64_unaligned(c);
        c.g[V0][0] = v;
        Addr(c.g[A0][0] as u32).write_u64_unaligned(c, v);
        c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
        c.g[V1][0] = 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 22);
        c.g[A0][0] = c.g[V0][0] & 7;
        // Currency format 0..3: where the symbol goes and which separator.
        let known = 'format: {
            if c.g[A0][0] == c.g[V1][0] {
                c.g[S3][0] = 0;
                break 'format true;
            }
            c.cycles += 2;
            c.g[V0][0] = 2;
            if c.g[A0][0] == 0 {
                c.cycles += 2;
                c.g[S2][0] = 0x2c;
                break 'format true;
            }
            c.cycles += 2;
            let is2 = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if is2 {
                c.cycles += 2;
                c.g[S2][0] = 0x2e;
                break 'format true;
            }
            c.cycles += 2;
            let is3 = c.g[A0][0] == c.g[V0][0];
            c.g[S0][0] = 0;
            if is3 {
                c.cycles += 1;
                c.g[S3][0] = 0;
                break 'format true;
            }
            c.cycles += 2;
            false
        };
        if known {
            c.cycles += 1;
            c.g[S0][0] = 0;
        }
        // Leading "$ " when the symbol goes first.
        c.cycles += 2;
        c.g[V1][0] = 0x24;
        if c.g[S3][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6310));
            c.g[A0][0] = 0x20;
            c.sb((c.g[S4][0] as u32).wrapping_add(0x6310), c.g[V1][0]);
            c.sb((c.g[V0][0] as u32).wrapping_add(1), c.g[A0][0]);
            c.g[S0][0] = 2;
        }
        // Sign.
        c.cycles += 2;
        if (c.g[S1][0] as i64) >= 0 {
            c.g[A0][0] = c.g[SP][0];
        } else {
            c.cycles += 10;
            c.g[V0][0] = c.g[S0][0];
            c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6310));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = 0x2d;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sb(c.g[V1][0] as u32, c.g[A0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[S1][0] = sx(0u32.wrapping_sub(c.g[S1][0] as u32));
            c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[A0][0] = c.g[SP][0];
        }
        // Decimal digits of |amount| onto the stack buffer, then copy them
        // to the output with a separator before every group of three.
        c.cycles += 9;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0041_7828, 0x0019_8c8c)?;
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0041_6810, 0x0019_8c94)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
        c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
        let empty = c.g[A3][0] == 0;
        c.g[A0][0] = 0;
        if !empty {
            c.cycles += 9;
            c.g[V0][0] = 0x4c0000;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6310));
            c.g[T1][0] = 3;
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[T3][0] = 0;
            c.g[T0][0] = 0x0100_0000;
            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
            c.g[T2][0] = 0x0100_0000;
            loop {
                'digit_done: {
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lbu(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 24);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 24);
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[S0][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                    let last = c.g[V0][0] == 0;
                    c.sb(c.g[A1][0] as u32, c.g[A0][0]);
                    'group: {
                        if !last {
                            c.cycles += 3;
                            c.div(0, 2, 9);
                            c.cycles += 3;
                            c.g[V0][0] = c.hi[0];
                            let mid_group = c.g[V0][0] != 0;
                            c.g[V0][0] = c.g[T0][0];
                            if mid_group {
                                break 'digit_done;
                            }
                            // A group boundary: insert the separator.
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                            c.sb(c.g[A1][0] as u32, c.g[S2][0]);
                            c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                            break 'group;
                        }
                        break 'group;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[T0][0];
                }
                c.cycles += 4;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                let more = c.g[A0][0] != c.g[A3][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_8cc8);
                    }
                    continue;
                }
                break;
            }
        }
        // Trailing " $" when the symbol goes last.
        c.cycles += 2;
        let symbol_first = c.g[S3][0] != 0;
        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6310));
        if !symbol_first {
            c.cycles += 16;
            c.g[V1][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6310));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[A0][0] = 0x20;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.sb(c.g[V1][0] as u32, c.g[A0][0]);
            c.g[V0][0] = c.g[S0][0];
            c.g[V1][0] = 0x24;
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sb(c.g[A1][0] as u32, c.g[V1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6310));
        }
        c.cycles += 10;
        frame.restore(c, 17, 0x2d0);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore(c, 18, 0x2c0);
        frame.restore(c, 16, 0x2e0);
        frame.restore(c, 19, 0x2b0);
        frame.restore(c, 20, 0x2a0);
        frame.restore_ra(c, 0x290);
        c.sb(c.g[V1][0] as u32, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCharEquip::cFEStateCharEquip(this, a1, player, flag, t0)`
/// (0x198db8): construct the character-equipment screen state: the camera
/// constants, the item-list bookkeeping fields, the five sound/anim handles
/// of the UI, the previewed character (`FEModule_RequestPreviewChar`) and
/// the item list (`BuildItemList`, `BoltFile_Acquire`).
pub fn cfestate_char_equip_ctor(c: &mut Cpu) {
    if c.pc != 0x0019_8db8 {
        return ee::s03::f_00198db8__entries(c);
    }
    guest(c, |c| {
        c.cycles += 54;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        c.g[S0][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x20);
        frame.save(c, 20, 0x10);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, 18, 0x30);
        c.g[S4][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0039_e2a0, 0x0019_8dec)?;
        // Camera constants (floats built through `at`).
        c.g[V0][0] = 0x470000;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 6);
        c.g[AT][0] = 0xffff_ffff_c2d2_0000;
        c.mtc1(1, 5);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ab8));
        c.g[AT][0] = 0xffff_ffff_c2c8_0000;
        c.mtc1(1, 4);
        c.g[V1][0] = 0xe;
        c.g[AT][0] = 0xffff_ffff_c2a0_0000;
        c.mtc1(1, 3);
        c.g[A1][0] = c.g[S0][0];
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x59b0));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x59ac));
        c.g[AT][0] = 0xffff_ffff_c368_0000;
        c.mtc1(1, 2);
        let this = |off: u32, c: &Cpu| (c.g[S1][0] as u32).wrapping_add(off);
        let a = this(8, c);
        c.sw(a, c.g[V0][0]);
        let a = this(0xac0, c);
        c.swc1(5, a);
        let a = this(0xac4, c);
        c.swc1(4, a);
        let a = this(0xac8, c);
        c.swc1(3, a);
        let a = this(0xad0, c);
        c.swc1(0, a);
        let a = this(0xad4, c);
        c.swc1(1, a);
        let a = this(0xad8, c);
        c.swc1(2, a);
        let a = this(0xadc, c);
        c.swc1(6, a);
        let a = this(0xc, c);
        c.sw(a, c.g[V1][0]);
        let a = this(0xacc, c);
        c.swc1(6, a);
        let a = this(0x44, c);
        c.sb(a, c.g[S0][0]);
        let owner = Addr(this(0x10, c)).lw(c);
        c.g[V0][0] = owner;
        c.g[A0][0] = Addr(owner as u32).lw(c);
        call_ptr(c, 0x001a_1cd0, 0x0019_8e64)?;
        let a = this(0x15, c);
        c.sb(a, c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x0019_8e6c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0019_8e78)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = Addr(this(0x44, c)).lb(c);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0014_7488, 0x0019_8e88)?;
        let a = this(0xbc, c);
        c.sw(a, c.g[V0][0]);
        if c.g[S3][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let a = this(0xc8, c);
            c.sw(a, c.g[V0][0]);
        } else {
            c.cycles += 1;
            let a = this(0xc8, c);
            c.sw(a, 0);
        }
        c.cycles += 38;
        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = 1;
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1f20)).lw(c);
        c.g[A0][0] = sx(this(0xd0, c));
        for off in [0x958, 0xa60] {
            let a = this(off, c);
            c.sw(a, 0);
        }
        c.g[A2][0] = 0x50;
        let a = this(0x960, c);
        c.sb(a, 0);
        for off in [
            0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x78, 0x7c, 0xab0,
        ] {
            let a = this(off, c);
            c.sw(a, 0);
        }
        let a = this(0xa74, c);
        c.sw(a, c.g[V0][0]);
        let a = this(0xae0, c);
        c.sw(a, c.g[V1][0]);
        for off in [0x95c, 0xa64, 0xa68, 0xa6c, 0xa70] {
            let a = this(off, c);
            c.sw(a, c.g[V0][0]);
        }
        call_ptr(c, 0x003e_6448, 0x0019_8f18)?;
        c.g[A2][0] = 0xffff_ffff_fffe_0000;
        let a = this(0x120, c);
        c.sw(a, 0);
        c.g[A2][0] |= 0x7961;
        c.g[A0][0] = sx(this(0xa78, c));
        c.g[V1][0] = sx(this(0xa4, c));
        c.g[V0][0] = sx(this(0x8c, c));
        c.g[A1][0] = 5;
        // Six slots of three parallel arrays (+0x8c, +0xa4, +0xa78).
        loop {
            c.cycles += 8;
            Addr(c.g[V0][0] as u32).sw(c, 0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            Addr(c.g[V1][0] as u32).sw(c, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            let fill = c.g[A2][0];
            Addr(c.g[A0][0] as u32).sw(c, fill);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            let more = (c.g[A1][0] as i64) >= 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0019_8f38);
                }
                continue;
            }
            break;
        }
        c.cycles += 69;
        for off in [0x80, 0x84, 0x88] {
            let a = this(off, c);
            c.sw(a, 0);
        }
        // Four handles looked up by name in the UI at `owner + 0x58`:
        // (name string offset in 0x460000.., field).
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7a8));
        c.g[S0][0] = Addr(this(0x10, c)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0019_8f74)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_8380, 0x0019_8f80)?;
        for (delta, field, ret_name, ret_lookup) in [
            (0x7b8u32, 0xa90u32, 0x0019_8f94u32, 0x0019_8fa0u32),
            (0u32.wrapping_sub(0x25b8), 0xa94, 0x0019_8fb4, 0x0019_8fc0),
            (0u32.wrapping_sub(0x25c8), 0xa98, 0x0019_8fd4, 0x0019_8fe0),
        ] {
            c.g[A0][0] = 0x460000;
            c.g[S0][0] = Addr(this(0x10, c)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(delta));
            let a = this(field, c);
            c.sw(a, c.g[V0][0]);
            call_ptr(c, 0x0031_7670, ret_name)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_8380, ret_lookup)?;
        }
        let a = this(0xa9c, c);
        c.sw(a, c.g[V0][0]);
        c.g[A3][0] = 1;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        let a = this(0xaa0, c);
        c.sw(a, 0);
        c.g[A2][0] = Addr(this(0xbc, c)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x7c)).lw(c);
        c.g[A1][0] = Addr(this(0x44, c)).lb(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        call_ptr(c, 0x001a_0508, 0x0019_9004)?;
        call_ptr(c, 0x0014_dc80, 0x0019_900c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x0019_9018)?;
        c.g[A2][0] = Addr(this(0xbc, c)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(this(0x44, c)).lb(c);
        call_ptr(c, 0x0015_0928, 0x0019_9028)?;
        let a = this(0xc0, c);
        c.sw(a, c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x0019_9030)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 9;
        call_ptr(c, 0x0014_dd58, 0x0019_903c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(this(0x44, c)).lb(c);
        c.g[A2][0] = Addr(this(0xbc, c)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_aea8, 0x0019_9050)?;
        c.g[A1][0] = Addr(this(0x44, c)).lb(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = Addr(this(0xbc, c)).lw(c);
        call_ptr(c, 0x0014_b478, 0x0019_9060)?;
        let a = this(0xc4, c);
        c.sw(a, c.g[V0][0]);
        let a = this(0xae4, c);
        c.sw(a, c.g[S4][0]);
        // Item-list mode `v0` (0xcc): -1 when both flags are zero, else
        // derived through a 5-entry jump table on the player's slot kind.
        'mode: {
            if c.g[S3][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                if c.g[S4][0] == 0 {
                    break 'mode;
                }
            }
            c.cycles += 9;
            c.g[A0][0] = Addr(this(0x44, c)).lb(c);
            call_ptr(c, 0x0014_a130, 0x0019_907c)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_6d98, 0x0019_9088)?;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x11));
            c.g[V0][0] = (c.g[V1][0] < 5) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[V0][0] = 0x460000;
            if out_of_range {
                c.cycles += 1;
                c.g[V0][0] = 1;
                break 'mode;
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7d0));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            let (extra, mode) = match target {
                0x0019_90b0 | 0x0019_90b8 => (2, 1),
                0x0019_90c0 | 0x0019_90c8 => (2, 2),
                0x0019_90d0 => (2, 3),
                _ => {
                    c.jump_out(target, 0x0019_8db8, 0x0019_9144);
                    return Ok(());
                }
            };
            c.cycles += extra;
            c.g[V0][0] = mode;
        }
        c.cycles += 26;
        let a = this(0xcc, c);
        c.sw(a, c.g[V0][0]);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1f20)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0019_b180, 0x0019_90ec)?;
        c.g[A1][0] = Addr(this(0xbc, c)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_b700, 0x0019_90f8)?;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = Addr(this(0xbc, c)).lw(c);
        c.g[A3][0] = 9;
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x7c)).lw(c);
        c.g[T0][0] = 1;
        c.g[T1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
        call_ptr(c, 0x0019_da10, 0x0019_911c)?;
        let a = this(0xa64, c);
        c.sw(a, c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_UNWIND_AT};
    use crate::difftest::{check, check_lohi, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the fakes: they only act in machines that hold the magic.
    const OC855_CTRL: u32 = DATA + 0x20_0000;
    const OC855_MAGIC: u32 = 0x0855_f4e5;
    /// Mailbox words (forced results) of the fakes, one per callee address.
    const OC855_MAIL: u32 = DATA + 0x30_0000;

    fn oc855_mail(addr: u32) -> u32 {
        OC855_MAIL + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0`.
    fn oc855_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc855_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    fn oc855_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc855_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// the forced result of its mailbox (0 = the fake's pointer).
    fn oc855_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc855_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc855_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC855_CTRL) != OC855_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc855_fake_call(c, $addr);
                }
            )*
            fn oc855_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc855_fakes!(
        (
            oc855_fake_28b180,
            OC855_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc855_fake_2b40f0,
            OC855_PREV_2B40F0,
            ee::s08::OVERRIDE_002B40F0,
            ee::s08::f_002b40f0__generated,
            0x002b_40f0
        ),
        (
            oc855_fake_2b4120,
            OC855_PREV_2B4120,
            ee::s08::OVERRIDE_002B4120,
            ee::s08::f_002b4120__generated,
            0x002b_4120
        ),
        (
            oc855_fake_317670,
            OC855_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc855_fake_3dbca0,
            OC855_PREV_3DBCA0,
            ee::s14::OVERRIDE_003DBCA0,
            ee::s14::f_003dbca0__generated,
            0x003d_bca0
        ),
        (
            oc855_fake_416810,
            OC855_PREV_416810,
            ee::s15::OVERRIDE_00416810,
            ee::s15::f_00416810__generated,
            0x0041_6810
        ),
        (
            oc855_fake_317e30,
            OC855_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc855_fake_2c26d0,
            OC855_PREV_2C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0
        ),
        (
            oc855_fake_3a0e90,
            OC855_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90
        ),
        (
            oc855_fake_317e98,
            OC855_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc855_fake_3a0c50,
            OC855_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc855_fake_3a1000,
            OC855_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc855_fake_14dc80,
            OC855_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc855_fake_14dd58,
            OC855_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc855_fake_417828,
            OC855_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc855_fake_39e2a0,
            OC855_PREV_39E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039_e2a0
        ),
        (
            oc855_fake_1a1cd0,
            OC855_PREV_1A1CD0,
            ee::s03::OVERRIDE_001A1CD0,
            ee::s03::f_001a1cd0__generated,
            0x001a_1cd0
        ),
        (
            oc855_fake_147488,
            OC855_PREV_147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488
        ),
        (
            oc855_fake_3e6448,
            OC855_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc855_fake_398380,
            OC855_PREV_398380,
            ee::s13::OVERRIDE_00398380,
            ee::s13::f_00398380__generated,
            0x0039_8380
        ),
        (
            oc855_fake_1a0508,
            OC855_PREV_1A0508,
            ee::s03::OVERRIDE_001A0508,
            ee::s03::f_001a0508__generated,
            0x001a_0508
        ),
        (
            oc855_fake_150928,
            OC855_PREV_150928,
            ee::s01::OVERRIDE_00150928,
            ee::s01::f_00150928__generated,
            0x0015_0928
        ),
        (
            oc855_fake_14aea8,
            OC855_PREV_14AEA8,
            ee::s01::OVERRIDE_0014AEA8,
            ee::s01::f_0014aea8__generated,
            0x0014_aea8
        ),
        (
            oc855_fake_14b478,
            OC855_PREV_14B478,
            ee::s01::OVERRIDE_0014B478,
            ee::s01::f_0014b478__generated,
            0x0014_b478
        ),
        (
            oc855_fake_14a130,
            OC855_PREV_14A130,
            ee::s01::OVERRIDE_0014A130,
            ee::s01::f_0014a130__generated,
            0x0014_a130
        ),
        (
            oc855_fake_146d98,
            OC855_PREV_146D98,
            ee::s01::OVERRIDE_00146D98,
            ee::s01::f_00146d98__generated,
            0x0014_6d98
        ),
        (
            oc855_fake_14b700,
            OC855_PREV_14B700,
            ee::s01::OVERRIDE_0014B700,
            ee::s01::f_0014b700__generated,
            0x0014_b700
        ),
        (
            oc855_fake_19da10,
            OC855_PREV_19DA10,
            ee::s03::OVERRIDE_0019DA10,
            ee::s03::f_0019da10__generated,
            0x0019_da10
        ),
        (
            oc855_fake_14bdb8,
            OC855_PREV_14BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8
        ),
        (
            oc855_fake_14ad28,
            OC855_PREV_14AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014_ad28
        ),
        (
            oc855_fake_14d7e8,
            OC855_PREV_14D7E8,
            ee::s01::OVERRIDE_0014D7E8,
            ee::s01::f_0014d7e8__generated,
            0x0014_d7e8
        ),
        (
            oc855_fake_418ef8,
            OC855_PREV_418EF8,
            ee::s15::OVERRIDE_00418EF8,
            ee::s15::f_00418ef8__generated,
            0x0041_8ef8
        ),
    );

    /// A machine with the fakes on, `calls` fake calls to unwind within.
    fn oc855_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc855_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC855_CTRL, OC855_MAGIC);
        r
    }

    fn oc855_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    #[test]
    fn cfestate_request_line_get_song_title_matches() {
        check(
            ssx3_ee::s03::f_00198a88__generated,
            cfestate_request_line_get_song_title,
            24,
            &|c, seed| {
                let mut r = oc855_base(c, seed, 0x198a88, 2);
                c.pc = 0x0019_8a88;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn cfestate_request_line_get_song_artist_matches() {
        check(
            ssx3_ee::s03::f_00198ab8__generated,
            cfestate_request_line_get_song_artist,
            24,
            &|c, seed| {
                let mut r = oc855_base(c, seed, 0x198ab8, 2);
                c.pc = 0x0019_8ab8;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn cfestate_request_line_get_song_price_matches() {
        check(
            ssx3_ee::s03::f_00198ae8__generated,
            cfestate_request_line_get_song_price,
            8,
            &|c, seed| {
                let mut r = oc855_base(c, seed, 0x198ae8, 0);
                c.pc = 0x0019_8ae8;
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn oc855_fe_format_money_matches() {
        check_lohi(
            ssx3_ee::s03::f_00198af0__generated,
            oc855_fe_format_money,
            96,
            &|c, seed| {
                let mut r = oc855_base(c, seed, 0x198af0, 4);
                c.pc = 0x0019_8af0;
                // The locale record: random bytes, the currency format in
                // bits 22..24 of its first word.
                let fmt = seed % 8;
                for i in 0..0xa2 {
                    oc855_w(c, 0x0053_5610 + 4 * i, r.next_u32());
                }
                let flags = r.next_u32() & !(7 << 22) | fmt << 22;
                oc855_w(c, 0x0053_5610, flags);
                // The digit count `strlen` reports (0 = empty string).
                match seed / 8 % 6 {
                    0 => oc855_force_zero(c, 0x0041_6810),
                    1 => oc855_force(c, 0x0041_6810, 1),
                    2 => oc855_force(c, 0x0041_6810, 3),
                    3 => oc855_force(c, 0x0041_6810, 4),
                    4 => oc855_force(c, 0x0041_6810, 7),
                    _ => oc855_force(c, 0x0041_6810, 10),
                }
                let amount = match seed % 3 {
                    0 => r.next_u32() % 100_000,
                    1 => 0u32.wrapping_sub(r.next_u32() % 100_000),
                    _ => r.next_u32(),
                };
                c.g[A0][0] = if seed % 5 == 0 {
                    (r.next_u32() as u64) << 32 | amount as u64
                } else {
                    sx(amount)
                };
            },
        );
    }

    #[test]
    fn cfestate_request_line_show_song_info_matches() {
        check(
            ssx3_ee::s03::f_001986d0__generated,
            cfestate_request_line_show_song_info,
            128,
            &|c, seed| {
                let mut r = oc855_base(c, seed, 0x1986d0, 14);
                c.pc = 0x0019_86d0;
                let this = DATA + 0x1000;
                let info = DATA + 0x2000;
                let price = DATA + 0x2400;
                let boxw = DATA + 0x2800;
                let vt = DATA + 0x3000;
                let world = DATA + 0x4000;
                let obj = DATA + 0x4400;
                let sub = DATA + 0x4800;
                let idx = r.next_u32() % 64;
                oc855_w(c, this + 0x1d4, if seed % 7 == 0 { 0 } else { info });
                oc855_w(c, this + 0x1d8, if seed % 5 == 1 { 0 } else { price });
                oc855_w(c, this + 0x1dc, boxw);
                oc855_w(c, this + 0x16c, (seed / 3 % 2) * 5);
                // The owned-songs mask: song `idx` owned, few, or many owned.
                let rand64 = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                let mask: u64 = match seed % 4 {
                    0 => 1 << idx,
                    1 => 0,
                    2 => !(1u64 << idx),
                    _ => rand64 & !(1u64 << idx),
                };
                c.bus.write64(this + 0x160, mask);
                for w in [info, price, boxw] {
                    oc855_w(c, w + 8, vt);
                }
                c.bus
                    .write16(vt + 0x48, (r.next_u32() as i16 as i32 & !3) as u16);
                oc855_w(c, vt + 0x4c, FAKE);
                oc855_w(c, GAME_GP.wrapping_sub(0x848), world);
                oc855_w(c, world + 0x8c, obj);
                oc855_w(c, obj + 4, sub);
                c.bus
                    .write16(sub + 0x20, (r.next_u32() as i16 as i32 & !3) as u16);
                oc855_w(c, sub + 0x24, FAKE + 0x10);
                // The money locale (format and digits) behind FE_FormatMoney.
                for i in 0..0xa2 {
                    oc855_w(c, 0x0053_5610 + 4 * i, r.next_u32());
                }
                let flags = r.next_u32() & !(7 << 22) | (seed / 4 % 4) << 22;
                oc855_w(c, 0x0053_5610, flags);
                // Fake call order: 0 text height, 1 vtable, 2-3 artist, 4
                // length, 5 strlen, 6 alloc, 7 format, 8 vtable, 9 set text.
                if seed % 6 == 2 {
                    oc855_force_zero(c, 0x0031_7e30);
                } else {
                    oc855_force(c, 0x0031_7e30, 0x00a0_0000 + 0x40 * (seed % 16));
                }
                oc855_force(c, 0x0041_6810, 5 + seed % 3);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(idx);
            },
        );
    }

    #[test]
    fn cfestate_char_equip_ctor_matches() {
        check(
            ssx3_ee::s03::f_00198db8__generated,
            cfestate_char_equip_ctor,
            96,
            &|c, seed| {
                let mut r = oc855_base(c, seed, 0x198db8, 0);
                c.pc = 0x0019_8db8;
                let this = DATA + 0x1000;
                let owner = DATA + 0x2000;
                let world = DATA + 0x4000;
                let flag3 = seed % 3 != 0;
                let flag4 = (seed / 3) % 2 == 1;
                oc855_w(c, this + 0x10, owner);
                oc855_w(c, owner, DATA + 0x2400);
                oc855_w(c, GAME_GP.wrapping_sub(0x848), world);
                oc855_w(c, world + 0x7c, DATA + 0x5000);
                oc855_w(c, GAME_GP.wrapping_sub(0x1f20), r.next_u32());
                oc855_w(c, GAME_GP.wrapping_sub(0x59b0), r.next_u32());
                oc855_w(c, GAME_GP.wrapping_sub(0x59ac), r.next_u32());
                // The 5-entry jump table of the item-list mode.
                let table = [
                    0x0019_90b0,
                    0x0019_90b8,
                    0x0019_90c0,
                    0x0019_90c8,
                    0x0019_90d0,
                ];
                for (i, t) in table.into_iter().enumerate() {
                    oc855_w(c, 0x0046_07d0 + 4 * i as u32, t);
                }
                // `146d98` result: the slot kind (0x11..0x15 select a table
                // entry, anything else the default).
                let mut kind = 0x0f + (seed / 2) % 9;
                if seed % 16 == 5 {
                    // A corrupt entry: the dispatcher leaves through `jump_out`.
                    let bad = seed / 16 % 5;
                    oc855_w(c, 0x0046_07d0 + 4 * bad, 0x0019_9000);
                    kind = 0x11 + bad;
                }
                oc855_force(c, 0x0014_6d98, kind);
                // BuildItemList (real) finds an empty list.
                oc855_force_zero(c, 0x0014_d7e8);
                let mode_called = flag3 || flag4;
                let first_in_build = 22 + if mode_called { 2 } else { 0 };
                let unwind = match seed % 4 {
                    // Unwind somewhere in the constructor.
                    0 => r.next_u32() % first_in_build,
                    // Skip BuildItemList's list loops when `this+0xc8` = 0.
                    _ if !flag3 => first_in_build,
                    _ => u32::MAX,
                };
                c.bus.write32(LOG_UNWIND_AT, unwind);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32() & 0x7f);
                c.g[A3][0] = flag3 as u64;
                c.g[T0][0] = flag4 as u64;
            },
        );
    }
}
