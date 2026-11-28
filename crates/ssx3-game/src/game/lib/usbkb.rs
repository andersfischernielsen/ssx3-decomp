//! `src/lib/usbkb.cpp`: hand-written USB keyboard (`libusbkb_*`, `sceUsb*`) replacements.

use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_c028, sce_usb_kb_init, "sceUsbKbInit"),
    (0x0040_c220, sce_usb_kb_end, "sceUsbKbEnd"),
    (0x0040_c2e8, sce_usb_kb_get_info, "sceUsbKbGetInfo"),
    (0x0040_c390, sce_usb_kb_read, "sceUsbKbRead"),
    (0x0040_c470, sce_usb_kb_get_location, "sceUsbKbGetLocation"),
    (
        0x0040_c550,
        sce_usb_kb_set_led_status,
        "sceUsbKbSetLEDStatus",
    ),
    (0x0040_c628, sce_usb_kb_set_led_mode, "sceUsbKbSetLEDMode"),
    (0x0040_c728, sce_usb_kb_set_repeat, "sceUsbKbSetRepeat"),
    (0x0040_c770, sce_usb_kb_set_code_type, "sceUsbKbSetCodeType"),
    (
        0x0040_c7c0,
        sce_usb_kb_set_arrangement,
        "sceUsbKbSetArrangement",
    ),
    (0x0040_c810, sce_usb_kb_sync, "sceUsbKbSync"),
    (0x0040_c8c0, libusbkb_sync_result, "libusbkb_SyncResult"),
    (0x0040_c980, libusbkb_read_callback, "libusbkb_ReadCallback"),
    (0x0040_d010, libusbkb_reset_key_buf, "libusbkb_ResetKeyBuf"),
    (0x0040_d040, libusbkb_key_buf_push, "libusbkb_KeyBufPush"),
    (0x0040_d0b0, libusbkb_key_buf_pop, "libusbkb_KeyBufPop"),
    (
        0x0040_d118,
        libusbkb_convert_key_codes,
        "libusbkb_ConvertKeyCodes",
    ),
    (0x0040_d268, sce_usb_kb_cnv_raw_code, "sceUsbKbCnvRawCode"),
    (
        0x0040_d3c0,
        libusbkb_cnv_keypad_code,
        "libusbkb_CnvKeypadCode",
    ),
    (0x0040_d430, sce_usb_kb_set_read_mode, "sceUsbKbSetReadMode"),
    (0x0040_d4a0, sce_usb_kb_clear_rbuf, "sceUsbKbClearRbuf"),
    (
        0x0040_cf48,
        libusbkb_set_led_status_callback_0040cf48,
        "libusbkb_SetLEDStatusCallback",
    ),
    (
        0x0040_cf68,
        libusbkb_set_led_mode_callback_0040cf68,
        "libusbkb_SetLEDModeCallback",
    ),
];

// ---------------------------------------------------------------------------
// Shared direct-call targets (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `sceUsbKbGetInfo` (`0x0040_c2e8`).
    pub(crate) const KB_GET_INFO: Callee = Callee {
        addr: 0x0040_c2e8,
        f: ee::s15::sceUsbKbGetInfo,
    };

    /// `sceUsbKbSync` (`0x0040_c810`).
    pub(crate) const KB_SYNC: Callee = Callee {
        addr: 0x0040_c810,
        f: ee::s15::sceUsbKbSync,
    };

    /// `libusbkb_SyncResult` (`0x0040_c8c0`).
    pub(crate) const KB_SYNC_RESULT: Callee = Callee {
        addr: 0x0040_c8c0,
        f: ee::s15::libusbkb_SyncResult,
    };

    /// `sceUsbKbCnvRawCode` (`0x0040_d268`).
    pub(crate) const CNV_RAW_CODE: Callee = Callee {
        addr: 0x0040_d268,
        f: ee::s15::sceUsbKbCnvRawCode,
    };

    /// `libusbkb_CnvKeypadCode` (`0x0040_d3c0`).
    pub(crate) const CNV_KEYPAD_CODE: Callee = Callee {
        addr: 0x0040_d3c0,
        f: ee::s15::libusbkb_CnvKeypadCode,
    };

    /// `AddIntcHandler` (`0x0042_3a80`).
    pub(crate) const ADD_INTC_HANDLER: Callee = Callee {
        addr: 0x0042_3a80,
        f: ee::s15::f_00423a80,
    };

    pub(super) use crate::game::lib::dev::callee::REMOVE_INTC_HANDLER;

    pub(super) use crate::game::lib::kernel::callee::CREATE_SEMA;

    pub(super) use crate::game::lib::sif::callee::DELETE_SEMA;

    pub(super) use crate::game::lib::mc::callee::SIGNAL_SEMA;

    pub(crate) use crate::game::lib::gcc::WAIT_SEMA;

    pub(super) use crate::game::lib::dev::callee::DISABLE_INTC;

    /// `EnableIntc` (`0x0042_48e8`).
    pub(crate) const ENABLE_INTC: Callee = Callee {
        addr: 0x0042_48e8,
        f: ee::s15::EnableIntc,
    };

    pub(super) use crate::game::lib::sif::callee::PRINTF;

    pub(super) use crate::game::lib::sif::callee::INIT_RPC;

    pub(super) use crate::game::lib::sif::callee::BIND_RPC;

    pub(super) use crate::game::lib::sif::callee::CALL_RPC;

    pub(super) use crate::game::lib::cdvd::callee::CHECK_STAT;

    pub(super) use crate::game::lib::cdvd::callee::DINTR;

    pub(super) use crate::game::lib::sif::callee::EINTR;

    /// `libusbkb_ConvertKeyCodes` (`0x0040_d118`).
    pub(crate) const CONVERT_KEY_CODES: Callee = Callee {
        addr: 0x0040_d118,
        f: ee::s15::libusbkb_ConvertKeyCodes,
    };

    /// `libusbkb_KeyBufPop` (`0x0040_d0b0`).
    pub(crate) const KEY_BUF_POP: Callee = Callee {
        addr: 0x0040_d0b0,
        f: ee::s15::libusbkb_KeyBufPop,
    };

    /// `libusbkb_KeyBufPush` (`0x0040_d040`).
    pub(crate) const KEY_BUF_PUSH: Callee = Callee {
        addr: 0x0040_d040,
        f: ee::s15::libusbkb_KeyBufPush,
    };

    /// `libusbkb_ResetKeyBuf` (`0x0040_d010`).
    pub(crate) const RESET_KEY_BUF: Callee = Callee {
        addr: 0x0040_d010,
        f: ee::s15::libusbkb_ResetKeyBuf,
    };
}

/// Keyboard arrangement (selects the conversion table).
pub(crate) const USBKB_ARRANGEMENT: u32 = 0x8;

/// Pointer to the per-keyboard state array (stride [`USBKB_STRIDE`]).
pub(crate) const USBKB_BASE: u32 = 0x0052_9004;

/// Callbacks `libusbkb_SyncResult` calls for a finished request, by request
/// kind (one word each).
pub(crate) const USBKB_CB_TABLE: u32 = 0x0045_3420;

/// RPC client block shared by the driver calls.
pub(crate) const USBKB_CLIENT: u32 = 0x0052_90a0;

/// Code type: 0 = copy raw codes, 1 = convert through the tables.
pub(crate) const USBKB_CODE_TYPE: u32 = 0x4;

/// Keyboard count (max index for the per-keyboard state array).
pub(crate) const USBKB_COUNT: u32 = 0x0052_9000;

/// RPC end callback issued with every driver call.
pub(crate) const USBKB_END_CALLBACK: u32 = 0x0040_c8b0;

/// First valid keyboard state (keyboards live at 0x5291d0 after init).
pub(crate) const USBKB_FIRST_STATE: u32 = 0x0052_91d0;

/// Keypad code table (byte per mode, then 0x15 `{byte, code}` pairs).
pub(crate) const USBKB_KEYPAD_TABLE: u32 = 0x0049_8fc8;

/// Read mode.
pub(crate) const USBKB_READ_MODE: u32 = 0xc;

/// Key buffer read index.
pub(crate) const USBKB_RIDX: u32 = 0x30;

/// Driver semaphore id (-1 while down).
pub(crate) const USBKB_SEMA: u32 = 0x0045_3438;

/// Send buffer for the driver RPCs (first byte: keyboard index).
pub(crate) const USBKB_SEND: u32 = 0x0052_9100;

/// Second send buffer (per-call meaning; passed as `t1`).
pub(crate) const USBKB_SEND2: u32 = 0x0052_9140;

/// Key buffer write index.
pub(crate) const USBKB_WIDX: u32 = 0x2c;

/// `int libusbkb_CnvKeypadCode(int mode, int code)` (0x40d3c0): look
/// `code & 0xff` up in the keypad table at `0x498fc8` (row `mode & 1`: 21
/// four-byte entries of raw byte plus mapped halfword). Returns the matching
/// entry's mapped code with bit 0x4000 set (a match on entry 0 included), or
/// `code & 0xff` when no entry matches.
pub fn libusbkb_cnv_keypad_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[V0][0] = 0x54;
        c.g[T0][0] = c.g[A0][0] & 1;
        c.mult(0, A0, T0, V0);
        c.g[T1][0] = 0x4a0000;
        c.g[A2][0] = sx(USBKB_KEYPAD_TABLE);
        c.g[A1][0] = c.g[A1][0] & 0xff;
        c.g[T2][0] = c.g[A1][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
        let direct = c.g[A1][0] == c.g[V1][0];
        c.g[A3][0] = 0;
        // Set when the scan runs past the last entry: the original then
        // breaks out past the table load below, returning the input code
        // masked to a byte (V1 keeps the 0x54 stride constant from the last
        // loop test).
        let mut exhausted = false;
        if !direct {
            c.cycles += 1;
            c.g[A3][0] = sx(1);
            loop {
                c.cycles += 3;
                c.g[V0][0] = (c.g[A3][0] < 0x15) as u64;
                let done = c.g[V0][0] == 0;
                c.g[V1][0] = 0x54;
                if done {
                    exhausted = true;
                    break;
                }
                c.cycles += 8;
                c.g[A0][0] = sx((c.g[A3][0] as u32) << 2);
                c.mult(0, V1, T0, V1);
                c.g[A2][0] = sx(USBKB_KEYPAD_TABLE);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                let miss = c.g[T2][0] != c.g[V1][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                if miss {
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_d3f0);
                    }
                    continue;
                } else {
                    break;
                }
            }
        }
        if !exhausted {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(2)).lhu(c);
            c.g[A1][0] = c.g[V1][0] | 0x4000;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A1][0];
        ret(c);
        Ok(())
    })
}

/// `int libusbkb_ConvertKeyCodes(int idx, int mods, int flags, int count,
/// u16 *src, u16 *dst)` (0x40d118): convert `count` raw codes from `src`
/// to `dst` for keyboard `idx`. In raw mode (code type 0) the codes are
/// copied until `count` or a zero code; in convert mode (1) each goes
/// through `sceUsbKbCnvRawCode` with `idx`'s arrangement. Returns the
/// converted count, or 0 for a bad mode or empty input.
pub fn libusbkb_convert_key_codes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        c.g[V0][0] = 0x38;
        c.g[V1][0] = 0x530000;
        c.mult(0, A0, A0, V0);
        let frame = Frame::open(c, 0x90);
        c.g[T2][0] = Addr(USBKB_BASE).lw(c);
        c.sd(frame.at(c, 0x70), c.g[S7][0]);
        c.sd(frame.at(c, 0x60), c.g[S6][0]);
        c.g[S7][0] = c.g[A1][0] & 0xff;
        c.sd(frame.at(c, 0x50), c.g[S5][0]);
        c.g[S6][0] = c.g[A2][0] & 0xff;
        c.sd(frame.at(c, 0x40), c.g[S4][0]);
        c.g[S5][0] = c.g[T1][0];
        c.sd(frame.at(c, 0x30), c.g[S3][0]);
        c.g[S4][0] = c.g[A3][0];
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.g[S3][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sd(frame.at(c, 0x80), c.g[RA][0]);
        c.g[S2][0] = c.g[T0][0];
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[V1][0] = Addr((c.g[S3][0] as u32).wrapping_add(USBKB_CODE_TYPE)).lw(c);
        let copy = c.g[V1][0] == 0;
        c.g[A1][0] = 0;
        // Bad code types return with the ra reloaded here, skipping the
        // shared tail's reload.
        let mut reload_ra = true;
        if !copy {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let convert = c.g[V1][0] == c.g[V0][0];
            c.g[RA][0] = c.ld(frame.at(c, 0x80));
            reload_ra = convert;
            if convert {
                c.cycles += 2;
                let more = (c.g[S4][0] as i64) > 0;
                c.g[S1][0] = 0;
                if more {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[S2][0] as u32).lhu(c);
                    let first = c.g[V0][0] == 0;
                    c.g[S0][0] = 0;
                    if !first {
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        loop {
                            c.cycles += 10;
                            c.g[A0][0] =
                                Addr((c.g[S3][0] as u32).wrapping_add(USBKB_ARRANGEMENT)).lw(c);
                            c.g[A3][0] = Addr(c.g[V0][0] as u32).lhu(c);
                            c.g[A1][0] = c.g[S7][0];
                            c.g[A2][0] = c.g[S6][0];
                            c.g[RA][0] = sx(0x0040_d208);
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                            callee::CNV_RAW_CODE.call(c, 0x0040_d208)?;
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                            c.g[A0][0] = ((c.g[S1][0] as i64) < (c.g[S4][0] as i64)) as u64;
                            let done = c.g[A0][0] == 0;
                            Addr(c.g[V1][0] as u32).sh(c, c.g[V0][0]);
                            if done {
                                break;
                            }
                            c.cycles += 6;
                            c.g[V0][0] = sx((c.g[S1][0] as u32) << 1);
                            c.g[S0][0] = c.g[V0][0];
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
                            let more_codes = c.g[V0][0] != 0;
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            if more_codes {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0040_d1f0);
                                }
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[A1][0] = c.g[S1][0];
            }
        } else {
            c.cycles += 2;
            let more = (c.g[S4][0] as i64) > 0;
            c.g[S1][0] = 0;
            // Set when the copy loop stops on a zero code: the original then
            // skips the `+1; A1 = S1` below, going straight to the common tail.
            let mut zero_end = false;
            if more {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lhu(c);
                let first = c.g[V0][0] == 0;
                c.g[S0][0] = 0;
                if !first {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    loop {
                        c.cycles += 5;
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S4][0] as i64)) as u64;
                        let done = c.g[V0][0] == 0;
                        Addr(c.g[A0][0] as u32).sh(c, c.g[V1][0]);
                        if done {
                            break;
                        }
                        c.cycles += 5;
                        c.g[S0][0] = sx((c.g[S1][0] as u32) << 1);
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
                        let more_codes = c.g[V1][0] != 0;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                        if more_codes {
                            if event_due(c) {
                                deliver_event_here(c, 0x0040_d1a8);
                            }
                            continue;
                        } else {
                            c.cycles += 2;
                            c.g[A1][0] = c.g[S1][0];
                            zero_end = true;
                            break;
                        }
                    }
                }
            }
            if !zero_end {
                c.cycles += 1;
            }
            c.g[A1][0] = c.g[S1][0];
        }
        c.cycles += 2;
        if reload_ra {
            c.g[RA][0] = c.ld(frame.at(c, 0x80));
        }
        c.g[V0][0] = c.g[A1][0];
        c.cycles += 10;
        c.g[S7][0] = c.ld(frame.at(c, 0x70));
        c.g[S6][0] = c.ld(frame.at(c, 0x60));
        c.g[S5][0] = c.ld(frame.at(c, 0x50));
        c.g[S4][0] = c.ld(frame.at(c, 0x40));
        c.g[S3][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int libusbkb_KeyBufPop(int idx, u16 *out)` (0x40d0b0): pop the
/// oldest buffered key of keyboard `idx` into `*out`. Returns -1 when
/// empty, else 0.
pub fn libusbkb_key_buf_pop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = 0x38;
        c.g[V1][0] = 0x530000;
        c.mult(0, A0, V0, A0);
        c.g[A2][0] = Addr(USBKB_BASE).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x24)).lw(c);
        if (c.g[V0][0] as i64) > 0 {
            c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(USBKB_RIDX)).lw(c);
            c.cycles += 10;
            c.g[V0][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x22)).lhu(c);
            Addr(c.g[A1][0] as u32).sh(c, c.g[V1][0]);
            Addr((c.g[A2][0] as u32).wrapping_add(USBKB_RIDX)).sw(c, c.g[A0][0]);
            c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x20)).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < 4) as u64;
            if c.g[V0][0] == 0 {
                Addr((c.g[A3][0] as u32).wrapping_add(0x20)).sw(c, 0);
            }
            c.cycles += 5;
            c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x24)).lw(c);
            c.g[V0][0] = 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let ra = c.g[RA][0] as u32;
            Addr((c.g[A3][0] as u32).wrapping_add(0x24)).sw(c, c.g[V1][0]);
            c.pc = ra;
        } else {
            c.cycles += 2;
            let ra = c.g[RA][0] as u32;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            c.pc = ra;
        }
        Ok(())
    })
}

/// `int libusbkb_KeyBufPush(int idx, int code)` (0x40d040): buffer `code`
/// on keyboard `idx`, resetting the write index when it wraps.
/// Returns -1 when full, else 0.
pub fn libusbkb_key_buf_push(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V0][0] = 0x38;
        c.g[V1][0] = 0x530000;
        c.mult(0, A0, V0, A0);
        c.g[A2][0] = Addr(USBKB_BASE).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x24)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 4) as u64;
        let room = c.g[V0][0] != 0;
        c.g[A1][0] &= 0xffff;
        if room {
            c.cycles += 10;
            c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(USBKB_WIDX)).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            Addr((c.g[V0][0] as u32).wrapping_add(0x22)).sh(c, c.g[A1][0]);
            Addr((c.g[A2][0] as u32).wrapping_add(USBKB_WIDX)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1c)).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < 4) as u64;
            if c.g[V0][0] == 0 {
                Addr((c.g[A0][0] as u32).wrapping_add(0x1c)).sw(c, 0);
            }
            c.cycles += 5;
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x24)).lw(c);
            c.g[V0][0] = 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            let ra = c.g[RA][0] as u32;
            Addr((c.g[A0][0] as u32).wrapping_add(0x24)).sw(c, c.g[V1][0]);
            c.pc = ra;
        } else {
            c.cycles += 2;
            let ra = c.g[RA][0] as u32;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            c.pc = ra;
        }
        Ok(())
    })
}

/// `int libusbkb_ReadCallback()` (0x0040c980): USB keyboard read completion.
///
/// Folds the latest HID report (bytes at 0x529140: modifiers, key count,
/// reserved, key codes) into the cooked output struct at `[0x529098]`, using
/// the per-device state at `[0x529004] + [0x529094]`. Changed keys go through
/// `libusbkb_KeyBufPush`/`Pop`/`ResetKeyBuf` and the keymap tables
/// (`libusbkb_ConvertKeyCodes`); the last cooked report is mirrored to
/// 0x529008. Returns -1 when no report arrived (`[0x529141] == 0xff`);
/// every other path returns 0 (the generated body zeroes `v0` after the
/// dispatch block).
pub fn libusbkb_read_callback(c: &mut Cpu) {
    guest(c, |c| {
        // The original saves the S-regs (and ra) with `sd`, not `sq`:
        // keep the 64-bit saves so the untouched upper stack bytes match
        // the generated body.
        let frame = Frame::open(c, 0x1a0);
        // Outer block: every `break 'b_40cecc` below lands on the epilogue.
        'b_40cecc: {
            c.cycles += 24;
            c.g[A1][0] = 0x38;
            c.sd(frame.at(c, 0x170), c.g[S5][0]);
            c.g[A2][0] = 0x530000;
            c.sd(frame.at(c, 0x160), c.g[S4][0]);
            c.g[S5][0] = 0x530000;
            c.sd(frame.at(c, 0x150), c.g[S3][0]);
            c.g[S4][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6ec0));
            c.sd(frame.at(c, 0x140), c.g[S2][0]);
            c.g[S3][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x7000));
            c.sd(frame.at(c, 0x120), c.g[S0][0]);
            c.g[A0][0] = 0xff;
            frame.save_ra(c, 0x190);
            c.sd(frame.at(c, 0x180), c.g[S6][0]);
            c.sd(frame.at(c, 0x130), c.g[S1][0]);
            // S1 = output struct, V0 = device index, V1 = state-array base.
            c.g[S1][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x98)).lw(c);
            c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x94)).lw(c);
            c.g[V1][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x4)).lw(c);
            c.mult(0, 2, 2, 5);
            // A1 = report key count; S0 = this device's state entry,
            // S2 = its key-buffer header.
            c.g[A1][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x1)).lbu(c);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
            let no_report = c.g[A1][0] == c.g[A0][0];
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if no_report {
                break 'b_40cecc;
            }
            // Inner block: `break 'b_40cec8` below skips the V0 = 0 tail and
            // returns whatever V0 holds (pop result / mirror word / key byte).
            'b_40cec8: {
                'b_40caa0: {
                    c.cycles += 4;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = 0x1;
                    if c.g[V0][0] != c.g[A0][0] {
                        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
                        break 'b_40caa0;
                    }
                    // Fresh report: copy the header into the output struct.
                    c.cycles += 7;
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_sub(0x6ec0)).lbu(c);
                    Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
                    c.g[V1][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x1)).lbu(c);
                    Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V1][0]);
                    c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x2)).lbu(c);
                    let count_empty = c.g[V1][0] == 0u64;
                    Addr((c.g[S1][0] as u32).wrapping_add(0x4)).sw(c, c.g[V0][0]);
                    if count_empty {
                        break 'b_40cec8;
                    }
                    // Compact the nonzero key bytes into the output slots.
                    'b_40ca64: {
                        {
                            c.cycles += 3;
                            c.g[A3][0] = 0u64;
                            let skip = c.g[V1][0] == 0u64;
                            c.g[A2][0] = 0u64;
                            if !skip {
                                c.cycles += 4;
                                c.g[A0][0] = c.g[V1][0];
                                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.g[V1][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[S4][0] as u32));
                                'l_40ca28: loop {
                                    {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            Addr((c.g[V1][0] as u32).wrapping_add(0x4)).lbu(c);
                                        let zero = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = sx((c.g[A3][0] as u32) << 1);
                                        if !zero {
                                            c.cycles += 4;
                                            c.g[V1][0] = Addr(c.g[V1][0] as u32).off(0x4).lbu(c);
                                            c.g[V0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                            Addr(c.g[V0][0] as u32).sh(c, c.g[V1][0]);
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                    let more = c.g[V0][0] != 0u64;
                                    c.g[V1][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[S4][0] as u32));
                                    if more {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0040_ca28);
                                        }
                                        continue 'l_40ca28;
                                    } else {
                                        c.cycles += 2;
                                        Addr((c.g[S1][0] as u32).wrapping_add(0x8))
                                            .sw(c, c.g[A3][0]);
                                        break 'b_40ca64;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                        Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, c.g[A3][0]);
                    }
                    {
                        c.cycles += 2;
                        let any = c.g[A3][0] != 0u64;
                        c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x7000));
                        if !any {
                            c.cycles += 4;
                            c.g[V0][0] = 0x1;
                            Addr((c.g[S1][0] as u32).wrapping_add(0xc)).sh(c, 0u64);
                            Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                            c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x7000));
                        }
                    }
                    c.cycles += 9;
                    c.g[T0][0] = c.g[S0][0];
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x94)).lw(c);
                    c.g[T1][0] = c.g[T0][0];
                    c.g[A3][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x8)).lw(c);
                    c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lbu(c);
                    c.g[A2][0] = Addr(c.g[S1][0] as u32).lbu(c);
                    callee::CONVERT_KEY_CODES.call(c, 0x0040_ca98)?;
                    c.g[V0][0] = 0u64;
                    break 'b_40cecc;
                }
                // Stale report: drive the per-device buffer state machine.
                'b_40cb6c: {
                    'b_40cb68: {
                        'b_40cb2c: {
                            'b_40cac0: {
                                c.cycles += 2;
                                let is_one = c.g[V1][0] == c.g[A0][0];
                                c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                if is_one {
                                    break 'b_40cac0;
                                }
                                c.cycles += 2;
                                let below_two = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x2;
                                if below_two {
                                    break 'b_40cb6c;
                                }
                                c.cycles += 2;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x4)).lw(c);
                                    break 'b_40cb2c;
                                }
                                c.cycles += 2;
                                break 'b_40cb6c;
                            }
                            c.cycles += 5;
                            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x4)).lw(c);
                            c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x8)).lw(c);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let refill = c.g[V0][0] != 0u64;
                            if refill {
                                break 'b_40cb6c;
                            }
                            // The buffered count ran dry: reset the key
                            // buffer and push the pending key.
                            {
                                c.cycles += 7;
                                Addr((c.g[S2][0] as u32).wrapping_add(0x4)).sw(c, 0u64);
                                c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x94)).lw(c);
                                callee::RESET_KEY_BUF.call(c, 0x0040_cae0)?;
                                c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x8)).lw(c);
                                c.g[V1][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                let push_zero = c.g[V1][0] != 0u64;
                                c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x94)).lw(c);
                                if !push_zero {
                                    c.cycles += 3;
                                    c.g[A1][0] = 0u64;
                                    callee::KEY_BUF_PUSH.call(c, 0x0040_caf8)?;
                                    c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x94)).lw(c);
                                }
                            }
                            {
                                c.cycles += 6;
                                c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x10)).lhu(c);
                                callee::KEY_BUF_PUSH.call(c, 0x0040_cb04)?;
                                c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
                                c.g[V1][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                let push_zero = c.g[V1][0] != 0u64;
                                c.g[V0][0] = 0x2;
                                if !push_zero {
                                    c.cycles += 4;
                                    c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x94)).lw(c);
                                    c.g[A1][0] = 0u64;
                                    callee::KEY_BUF_PUSH.call(c, 0x0040_cb20)?;
                                    c.g[V0][0] = 0x2;
                                }
                            }
                            c.cycles += 2;
                            // Delay-slot store: runs on the way out.
                            Addr((c.g[S0][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                            break 'b_40cb68;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        let push_pending = c.g[V0][0] != 0u64;
                        if push_pending {
                            break 'b_40cb6c;
                        }
                        // Second key pending: push it, then maybe a zero.
                        c.cycles += 8;
                        Addr((c.g[S2][0] as u32).wrapping_add(0x4)).sw(c, 0u64);
                        c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x10)).lhu(c);
                        c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x94)).lw(c);
                        callee::KEY_BUF_PUSH.call(c, 0x0040_cb4c)?;
                        c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
                        c.g[V1][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                        if c.g[V1][0] != 0u64 {
                            c.g[A1][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x1)).lbu(c);
                            break 'b_40cb6c;
                        }
                        c.cycles += 3;
                        c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x94)).lw(c);
                        c.g[A1][0] = 0u64;
                        callee::KEY_BUF_PUSH.call(c, 0x0040_cb68)?;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x1)).lbu(c);
                }
                // Empty report: serve the oldest buffered key, if any.
                {
                    c.cycles += 2;
                    if c.g[A1][0] != 0u64 {
                        c.g[V1][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x4)).lbu(c);
                    } else {
                        c.cycles += 6;
                        c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x7000));
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x94)).lw(c);
                        callee::KEY_BUF_POP.call(c, 0x0040_cb84)?;
                        if c.g[V0][0] != 0u64 {
                            Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                            break 'b_40cec8;
                        }
                        c.cycles += 9;
                        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x110)).lhu(c);
                        c.g[V0][0] = 0x1;
                        Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                        Addr((c.g[S1][0] as u32).wrapping_add(0xc)).sh(c, c.g[V1][0]);
                        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
                        Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
                        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                        // Delay-slot store: runs on the way out.
                        Addr((c.g[S1][0] as u32).wrapping_add(0x4)).sw(c, c.g[V1][0]);
                        break 'b_40cec8;
                    }
                }
                // Nonempty report: check the first key byte.
                {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V1][0] = Addr(c.g[S4][0] as u32).lbu(c);
                    } else {
                        c.cycles += 4;
                        Addr(c.g[S2][0] as u32).sw(c, 0u64);
                        c.g[V0][0] = 0u64;
                        // Delay-slot store: runs on the way out.
                        Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
                        break 'b_40cecc;
                    }
                }
                // Filter the report keys into the stack buffer, dropping
                // zeroes, and remember the count at sp+8.
                'b_40cc38: {
                    {
                        c.cycles += 8;
                        c.g[A3][0] = 0u64;
                        c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x1)).lbu(c);
                        c.g[A2][0] = 0u64;
                        c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x2)).lbu(c);
                        Addr(c.g[SP][0] as u32).sw(c, c.g[V1][0]);
                        Addr((c.g[SP][0] as u32).wrapping_add(0x4)).sw(c, c.g[V0][0]);
                        let count_empty = c.g[A0][0] == 0u64;
                        Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, c.g[A0][0]);
                        if !count_empty {
                            c.cycles += 3;
                            c.g[S6][0] = 0x530000;
                            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S4][0] as u32));
                            'l_40cbf8: loop {
                                {
                                    c.cycles += 3;
                                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4)).lbu(c);
                                    let zero = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx((c.g[A3][0] as u32) << 1);
                                    if !zero {
                                        c.cycles += 5;
                                        c.g[V1][0] =
                                            Addr((c.g[V1][0] as u32).wrapping_add(0x4)).lbu(c);
                                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc));
                                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        Addr(c.g[V0][0] as u32).sh(c, c.g[V1][0]);
                                    }
                                }
                                c.cycles += 4;
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                let more = c.g[V0][0] != 0u64;
                                c.g[V1][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[S4][0] as u32));
                                if more {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0040_cbf8);
                                    }
                                    continue 'l_40cbf8;
                                } else {
                                    c.cycles += 2;
                                    // Delay-slot store: runs on the way out.
                                    Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, c.g[A3][0]);
                                    break 'b_40cc38;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[S6][0] = 0x530000;
                    // Delay-slot store: runs on the way out.
                    Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sw(c, c.g[A3][0]);
                }
                // Reconcile the filtered keys with the buffered state.
                'b_40ce9c: {
                    'b_40ce98: {
                        'b_40cc84: {
                            'b_40cc5c: {
                                c.cycles += 6;
                                c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x7000));
                                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x94)).lw(c);
                                callee::RESET_KEY_BUF.call(c, 0x0040_cc44)?;
                                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0xc)).lhu(c);
                                let key0_empty = c.g[V1][0] == 0u64;
                                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x4)).lw(c);
                                if key0_empty {
                                    break 'b_40cc5c;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[V0][0] =
                                        Addr((c.g[S0][0] as u32).wrapping_add(0x14)).lhu(c);
                                    break 'b_40cc84;
                                }
                            }
                            // No usable key yet: report a single zero key.
                            c.cycles += 10;
                            Addr((c.g[S1][0] as u32).wrapping_add(0xc)).sh(c, 0u64);
                            c.g[V0][0] = 0x1;
                            Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                            Addr((c.g[S2][0] as u32).wrapping_add(0x10)).sh(c, 0u64);
                            c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                            Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
                            c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x4)).lw(c);
                            Addr((c.g[S1][0] as u32).wrapping_add(0x4)).sw(c, c.g[V1][0]);
                            // Delay-slot store: runs on the way out.
                            Addr(c.g[S2][0] as u32).sw(c, 0u64);
                            break 'b_40ce98;
                        }
                        // Diff the filtered keys against the last report:
                        // new keys are copied to the output, T3 counts them.
                        'b_40cda8: {
                            'b_40cd64: {
                                c.cycles += 2;
                                let last_empty = c.g[V0][0] == 0u64;
                                c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x8)).lw(c);
                                if last_empty {
                                    // No last report: copy the whole
                                    // filtered list below.
                                    break 'b_40cd64;
                                }
                                'b_40cd58: {
                                    c.cycles += 4;
                                    c.g[T3][0] = 0u64;
                                    c.g[A2][0] = 0u64;
                                    let none = (c.g[A0][0] as i64) <= 0;
                                    c.g[T4][0] = c.g[A0][0];
                                    if none {
                                        break 'b_40cd58;
                                    }
                                    c.cycles += 3;
                                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x7000));
                                    'l_40cca8: loop {
                                        'b_40cd14: {
                                            'b_40cd10: {
                                                {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = Addr(
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    )
                                                    .lw(c);
                                                    let no_table = (c.g[V0][0] as i64) <= 0;
                                                    c.g[A3][0] = 0u64;
                                                    if !no_table {
                                                        c.cycles += 8;
                                                        c.g[A1][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[T1][0] = sx((c.g[A2][0] as u32) << 1);
                                                        c.g[T2][0] = c.g[A1][0];
                                                        c.g[V0][0] = Addr(
                                                            (c.g[V1][0] as u32).wrapping_add(0x14),
                                                        )
                                                        .lhu(c);
                                                        c.g[V1][0] = sx((c.g[T2][0] as u32)
                                                            .wrapping_add(c.g[T1][0] as u32));
                                                        c.g[A0][0] = Addr(c.g[V1][0] as u32).lhu(c);
                                                        let kept = c.g[A0][0] == c.g[V0][0];
                                                        c.g[T0][0] =
                                                            sx((c.g[A2][0] as u32)
                                                                .wrapping_add(0x1));
                                                        if !kept {
                                                            c.cycles += 1;
                                                            c.g[A1][0] = sx((c.g[S5][0] as u32)
                                                                .wrapping_sub(0x7000));
                                                            'l_40ccd8: loop {
                                                                c.cycles += 5;
                                                                c.g[A3][0] = sx((c.g[A3][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                c.g[V0][0] = Addr(
                                                                    (c.g[A1][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                )
                                                                .lw(c);
                                                                c.g[V0][0] = ((c.g[A3][0] as i64)
                                                                    < (c.g[V0][0] as i64))
                                                                    as u64;
                                                                let exhausted = c.g[V0][0] == 0u64;
                                                                c.g[V1][0] =
                                                                    sx((c.g[A3][0] as u32) << 1);
                                                                if exhausted {
                                                                    break 'b_40cd10;
                                                                }
                                                                c.cycles += 6;
                                                                c.g[A0][0] = sx((c.g[T2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[T1][0] as u32,
                                                                    ));
                                                                c.g[V1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V1][0] as u32,
                                                                    ));
                                                                c.g[A1][0] =
                                                                    Addr(c.g[A0][0] as u32).lhu(c);
                                                                c.g[V0][0] = Addr(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x14),
                                                                )
                                                                .lhu(c);
                                                                let miss = c.g[A1][0] != c.g[V0][0];
                                                                c.g[A1][0] = sx((c.g[S5][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x7000));
                                                                if miss {
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c,
                                                                            0x0040_ccd8,
                                                                        );
                                                                    }
                                                                    continue 'l_40ccd8;
                                                                }
                                                                c.cycles += 2;
                                                                c.g[V0][0] = sx((c.g[S5][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x7000));
                                                                break 'b_40cd14;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[T0][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                sx((c.g[S5][0] as u32).wrapping_sub(0x7000));
                                        }
                                        {
                                            c.cycles += 3;
                                            c.g[V1][0] =
                                                Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lw(c);
                                            if c.g[A3][0] != c.g[V1][0] {
                                                c.g[A2][0] = c.g[T0][0];
                                            } else {
                                                c.cycles += 9;
                                                c.g[A0][0] = sx((c.g[A2][0] as u32) << 1);
                                                c.g[V0][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0xc));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                c.g[V1][0] = sx((c.g[T3][0] as u32) << 1);
                                                c.g[A0][0] = Addr(c.g[V0][0] as u32).lhu(c);
                                                c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                c.g[T3][0] =
                                                    sx((c.g[T3][0] as u32).wrapping_add(0x1));
                                                Addr(c.g[V1][0] as u32).sh(c, c.g[A0][0]);
                                                c.g[A2][0] = c.g[T0][0];
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[A2][0] as i64) < (c.g[T4][0] as i64)) as u64;
                                        let more = c.g[V0][0] != 0u64;
                                        c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x7000));
                                        if more {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0040_cca8);
                                            }
                                            continue 'l_40cca8;
                                        }
                                        c.cycles += 2;
                                        // Delay-slot store: runs on the way
                                        // out.
                                        Addr((c.g[S1][0] as u32).wrapping_add(0x8))
                                            .sw(c, c.g[T3][0]);
                                        break 'b_40cda8;
                                    }
                                }
                                // No filtered keys (or no table): report an
                                // empty new-key list.
                                c.cycles += 3;
                                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                                // Delay-slot store: runs on the way out.
                                Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, c.g[T3][0]);
                                break 'b_40cda8;
                            }
                            // No last report: copy the filtered keys as-is.
                            {
                                c.cycles += 3;
                                c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x8)).lw(c);
                                let none = (c.g[A3][0] as i64) <= 0;
                                c.g[A2][0] = 0u64;
                                if !none {
                                    c.cycles += 2;
                                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc));
                                    'l_40cd78: loop {
                                        c.cycles += 8;
                                        c.g[V0][0] = sx((c.g[A2][0] as u32) << 1);
                                        c.g[V1][0] =
                                            sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                        c.g[A0][0] = Addr(c.g[V1][0] as u32).lhu(c);
                                        c.g[V0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[V1][0] =
                                            ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                                        let more = c.g[V1][0] != 0u64;
                                        Addr(c.g[V0][0] as u32).sh(c, c.g[A0][0]);
                                        if more {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0040_cd78);
                                            }
                                            continue 'l_40cd78;
                                        }
                                        c.cycles += 2;
                                        // Delay-slot store: runs on the way
                                        // out.
                                        Addr((c.g[S1][0] as u32).wrapping_add(0x8))
                                            .sw(c, c.g[A2][0]);
                                        break 'b_40cda8;
                                    }
                                }
                                c.cycles += 2;
                                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                                // Delay-slot store: runs on the way out.
                                Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, c.g[A2][0]);
                                break 'b_40cda8;
                            }
                        }
                        // Convert the new keys and remember the last one.
                        {
                            c.cycles += 15;
                            c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                            c.g[S3][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x7000));
                            c.g[A3][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x8)).lw(c);
                            c.g[T0][0] = c.g[S0][0];
                            Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
                            c.g[T1][0] = c.g[S0][0];
                            c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x4)).lw(c);
                            c.g[A2][0] = Addr(c.g[S1][0] as u32).lbu(c);
                            Addr((c.g[S1][0] as u32).wrapping_add(0x4)).sw(c, c.g[V0][0]);
                            c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x94)).lw(c);
                            c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lbu(c);
                            callee::CONVERT_KEY_CODES.call(c, 0x0040_cdd8)?;
                            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x8)).lw(c);
                            let empty = (c.g[V0][0] as i64) <= 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            if !empty {
                                c.cycles += 6;
                                c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x8)).lw(c);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                c.g[V0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
                                let stalled = (c.g[A0][0] as i64) <= 0;
                                Addr((c.g[S2][0] as u32).wrapping_add(0x10)).sh(c, c.g[V1][0]);
                                if stalled {
                                    break 'b_40ce98;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
                                let settled = (c.g[V0][0] as i64) <= 0;
                                c.g[V0][0] = 0x1;
                                if settled {
                                    break 'b_40ce98;
                                }
                                c.cycles += 2;
                                // Delay-slot store: runs on the way out.
                                Addr(c.g[S2][0] as u32).sw(c, c.g[V0][0]);
                                break 'b_40ce98;
                            }
                        }
                        // Convert the whole filtered list and look for the
                        // last buffered key in it.
                        'b_40ce70: {
                            c.cycles += 11;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                            c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x4)).lbu(c);
                            c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x94)).lw(c);
                            c.g[T0][0] = c.g[SP][0] | 0xc;
                            c.g[A2][0] = Addr(c.g[SP][0] as u32).lbu(c);
                            c.g[T1][0] = c.g[S0][0];
                            c.g[A3][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x8)).lw(c);
                            callee::CONVERT_KEY_CODES.call(c, 0x0040_ce30)?;
                            c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x8)).lw(c);
                            let empty = (c.g[A0][0] as i64) <= 0;
                            c.g[A2][0] = 0u64;
                            if !empty {
                                c.cycles += 4;
                                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x90)).lhu(c);
                                c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x10)).lhu(c);
                                let same = c.g[V0][0] == c.g[V1][0];
                                c.g[A1][0] = c.g[V1][0];
                                if !same {
                                    c.cycles += 3;
                                    c.g[T1][0] = c.g[S0][0];
                                    c.g[V1][0] = c.g[A0][0];
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                    'l_40ce58: loop {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                        let done = c.g[V0][0] == 0u64;
                                        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2));
                                        if done {
                                            break 'b_40ce70;
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = Addr(c.g[T1][0] as u32).lhu(c);
                                        if c.g[V0][0] != c.g[A1][0] {
                                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0040_ce58);
                                            }
                                            continue 'l_40ce58;
                                        }
                                        break 'b_40ce70;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let found_early = c.g[A2][0] != c.g[A0][0];
                        c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6ff8));
                        if found_early {
                            break 'b_40ce9c;
                        }
                        // The new keys are all consumed: report a single
                        // zero key and the saved header.
                        c.cycles += 8;
                        Addr(c.g[S2][0] as u32).sw(c, 0u64);
                        c.g[V0][0] = 0x1;
                        Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, c.g[V0][0]);
                        Addr((c.g[S1][0] as u32).wrapping_add(0xc)).sh(c, 0u64);
                        c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                        Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
                        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x4)).lw(c);
                        Addr((c.g[S1][0] as u32).wrapping_add(0x4)).sw(c, c.g[V1][0]);
                        break 'b_40ce98;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6ff8));
                }
                // Mirror the cooked report (0x88 bytes) to 0x529008. This is
                // the tail of 'b_40ce9c: it runs after every exit above.
                c.cycles += 3;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = 0u64;
                'l_40cea8: loop {
                    c.cycles += 8;
                    c.g[V1][0] = Addr(c.g[A0][0] as u32).lbu(c);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[A2][0] < 0x88u64) as u64;
                    Addr(c.g[A1][0] as u32).sb(c, c.g[V1][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    let more = c.g[V0][0] != 0u64;
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0040_cea8);
                        }
                        continue 'l_40cea8;
                    }
                    break 'b_40cec8;
                }
            }
            // Every `'b_40cec8` exit (pop hit/miss, first-key shortcut,
            // filter, converts, mirror loop) flows through the generated
            // `cycles += 1; v0 = 0` here, so all of them return 0.
            c.cycles += 1;
            c.g[V0][0] = 0u64;
        }
        // Epilogue: the generated body restores the S-regs with `ld`.
        c.cycles += 10;
        c.g[RA][0] = c.ld(frame.at(c, 0x190));
        c.g[S6][0] = c.ld(frame.at(c, 0x180));
        c.g[S5][0] = c.ld(frame.at(c, 0x170));
        c.g[S4][0] = c.ld(frame.at(c, 0x160));
        c.g[S3][0] = c.ld(frame.at(c, 0x150));
        c.g[S2][0] = c.ld(frame.at(c, 0x140));
        c.g[S1][0] = c.ld(frame.at(c, 0x130));
        c.g[S0][0] = c.ld(frame.at(c, 0x120));
        frame.close(c);
        Ok(())
    })
}

/// `void libusbkb_ResetKeyBuf(int idx)` (0x40d010): clear keyboard
/// `idx`'s buffer indices and counts.
pub fn libusbkb_reset_key_buf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V0][0] = 0x38;
        c.g[A1][0] = 0x530000;
        c.mult(0, A0, V0, A0);
        c.g[V1][0] = Addr(USBKB_BASE).lw(c);
        c.g[V0][0] = 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        Addr((c.g[V1][0] as u32).wrapping_add(4)).sw(c, 0);
        Addr((c.g[V1][0] as u32).wrapping_add(0x24)).sw(c, 0);
        Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).sw(c, 0);
        let ra = c.g[RA][0] as u32;
        Addr((c.g[V1][0] as u32).wrapping_add(0x20)).sw(c, 0);
        c.pc = ra;
        Ok(())
    })
}

/// `int libusbkb_SyncResult(void)` (0x40c8c0): run the completion
/// callback for the pending RPC result id (returning what it returns)
/// and clear the result. Returns 0 when no result is pending.
pub fn libusbkb_sync_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x530000;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[S0][0] = sx(USBKB_COUNT);
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x90)).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[V0][0] = (c.g[V0][0] < 5) as u64;
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !idle {
            c.cycles += 8;
            c.g[V0][0] = 0x450000;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx(USBKB_CB_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0040_c904);
            call_ptr(c, target, 0x0040_c904)?;
            c.g[A0][0] = c.g[V0][0];
        }
        c.cycles += 7;
        Addr((c.g[S0][0] as u32).wrapping_add(0x98)).sw(c, 0);
        c.g[V0][0] = c.g[A0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x90)).sw(c, 0);
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceUsbKbClearRbuf(int idx)` (0x40d4a0): clear keyboard `idx`'s key
/// buffer over RPC. Returns -1 while the driver is down, -5 for a bad
/// index, -2 when the RPC fails, else 0.
pub fn sce_usb_kb_clear_rbuf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x450000;
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[A0][0] = Addr(USBKB_SEMA).lw(c);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        let down = c.g[A0][0] == c.g[V0][0];
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        if !down {
            c.cycles += 5;
            c.g[V1][0] = 0x530000;
            c.g[V0][0] = Addr(USBKB_COUNT).lw(c);
            c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
            let known = c.g[V0][0] != 0;
            c.g[S2][0] = sx(USBKB_COUNT);
            if !known {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_fff5;
            } else {
                c.cycles += 19;
                c.g[S0][0] = 0x530000;
                Addr(USBKB_SEND).sb(c, c.g[S1][0]);
                c.g[RA][0] = sx(0x0040_d4f4);
                c.g[S0][0] = sx(USBKB_SEND);
                callee::WAIT_SEMA.call(c, 0x0040_d4f4)?;
                c.g[A0][0] = 0x530000;
                c.g[T1][0] = 0x530000;
                c.g[T3][0] = 0x410000;
                c.g[A0][0] = sx(USBKB_CLIENT);
                c.g[A3][0] = c.g[S0][0];
                c.g[T1][0] = sx(USBKB_SEND2);
                c.g[T3][0] = sx(USBKB_END_CALLBACK);
                Addr(c.g[SP][0] as u32).sw(c, 0);
                c.g[A1][0] = 6;
                c.g[A2][0] = 1;
                c.g[T0][0] = 0x10;
                c.g[RA][0] = sx(0x0040_d528);
                c.g[T2][0] = 0;
                callee::CALL_RPC.call(c, 0x0040_d528)?;
                let ok = c.g[V0][0] == 0;
                c.g[A0][0] = 0x4a0000;
                if !ok {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0040_d53c);
                    c.g[A0][0] = sx(0x0049_82f8);
                    callee::PRINTF.call(c, 0x0040_d53c)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                } else {
                    c.cycles += 11;
                    c.g[A0][0] = 0x38;
                    c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(4)).lw(c);
                    c.mult(0, A0, S1, A0);
                    c.g[V0][0] = 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let entry = Addr(c.g[V1][0] as u32);
                    entry.off(0x34).sw(c, 0);
                    entry.off(0x10).sw(c, 0);
                    entry.off(0x14).sw(c, 0);
                    entry.off(0x20).sh(c, 0);
                    entry.off(0x2c).sw(c, 0);
                    entry.off(0x30).sw(c, 0);
                }
            }
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x40);
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceUsbKbCnvRawCode(int arr, int mods, int flags, int raw)`
/// (0x40d268): map raw key code `raw` (low 16 bits) through the 256-halfword
/// table for arrangement `arr` (0: `0x4983c8`, 1: `0x4987c8`, 2: `0x498bc8`;
/// the shifted half at `+0x200` when `mods & 0x22`). A 0x4000 entry goes
/// through `libusbkb_CnvKeypadCode(flags & 0xff, raw & 0xff)`; a 0x8000
/// result becomes `raw | 0x8000`. With the caps bit (`flags & 2`) letters
/// come back uppercase, or lowercase while shifted. Returns 0 for `raw` 0 and
/// `raw` itself for codes >= 0x100; any other `arr` prints the
/// bad-arrangement message (`0x4983b0`) and reads through a null table.
pub fn sce_usb_kb_cnv_raw_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 1;
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0x30), c.g[S3][0]);
        c.g[S1][0] = c.g[A1][0] & 0x22;
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.g[S3][0] = c.g[A2][0];
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S2][0] = 0;
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[S0][0] = c.g[A3][0] & 0xffff;
        let first = c.g[A0][0] == c.g[V0][0];
        c.g[S1][0] = (0 < c.g[S1][0]) as u64;
        if first {
            // Arrangement 1 (breaks out of the dispatch to the 0x4987c8
            // table: 0x4a0000 - 0x7838).
            c.cycles += 5;
            c.g[V0][0] = 0x4a0000;
            c.g[V1][0] = sx((c.g[S1][0] as u32) << 9);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7838));
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 2;
                let second = c.g[A0][0] == c.g[V0][0];
                c.g[A0][0] = 0x4a0000;
                if second {
                    // Arrangement 2 (0x4a0000 - 0x7438 = 0x498bc8 table).
                    c.cycles += 5;
                    c.g[V0][0] = 0x4a0000;
                    c.g[V1][0] = sx((c.g[S1][0] as u32) << 9);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7438));
                    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_d30c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7c50));
                    // The shared debug-print block.
                    c.cycles += 2;
                    callee::PRINTF.call(c, 0x0040_d30c)?;
                }
            } else {
                c.cycles += 2;
                let is_zero = c.g[A0][0] == 0;
                c.g[A0][0] = 0x4a0000;
                if is_zero {
                    // Arrangement 0 (0x4a0000 - 0x7c38 = 0x4983c8 table):
                    // skips the arrangement-2 check below and falls out of
                    // the dispatch here.
                    c.cycles += 5;
                    c.g[V0][0] = 0x4a0000;
                    c.g[V1][0] = sx((c.g[S1][0] as u32) << 9);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7c38));
                    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                } else {
                    // Negative arrangement: prints the same bad-arrangement
                    // message (`0x4983b0`) as the arrangement > 2 arm.
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_d30c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7c50));
                    // The shared debug-print block.
                    c.cycles += 2;
                    callee::PRINTF.call(c, 0x0040_d30c)?;
                }
            }
        }
        c.cycles += 2;
        let empty = c.g[S0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 3;
            c.g[V0][0] = (c.g[S0][0] < 0x100) as u64;
            let mapped = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S0][0] as u32) << 1);
            if !mapped {
                c.cycles += 2;
                c.g[V0][0] = c.g[S0][0];
            } else {
                c.cycles += 5;
                c.g[V1][0] = 0x4000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A1][0] = Addr(c.g[V0][0] as u32).lhu(c);
                let keypad = c.g[A1][0] == c.g[V1][0];
                c.g[V0][0] = 0x8000;
                if keypad {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S3][0] & 0xff;
                    c.g[RA][0] = sx(0x0040_d348);
                    c.g[A1][0] = c.g[S0][0] & 0xff;
                    callee::CNV_KEYPAD_CODE.call(c, 0x0040_d348)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0x8000;
                }
                c.cycles += 2;
                if c.g[A1][0] == c.g[V0][0] {
                    c.g[A1][0] = c.g[S0][0] | 0x8000;
                }
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x61));
                c.g[V0][0] &= 0xffff;
                c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                let lower = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[S3][0] & 2;
                if !lower {
                    c.cycles += 2;
                    let caps_off = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x20));
                    if !caps_off {
                        c.cycles += 1;
                        c.g[A1][0] = c.g[V0][0] & 0xffff;
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x41));
                c.g[V0][0] &= 0xffff;
                c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                let upper = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[S3][0] & 2;
                // Set for uppercase input with caps off: the original then
                // breaks straight to the epilogue, skipping the `+1; V0 = A1`
                // below (V0 already holds A1 here).
                let mut caps_skip = false;
                if !upper {
                    c.cycles += 2;
                    let caps_off = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[A1][0];
                    if !caps_off {
                        c.cycles += 2;
                        let unshifted = c.g[S1][0] == 0;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if !unshifted {
                            c.cycles += 1;
                            c.g[A1][0] = c.g[V0][0] & 0xffff;
                        }
                    } else {
                        caps_skip = true;
                    }
                }
                if !caps_skip {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[A1][0];
                }
            }
        }
        c.cycles += 7;
        c.g[RA][0] = c.ld(frame.at(c, 0x40));
        c.g[S3][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceUsbKbEnd(void)` (0x40c220): shut the driver down: with interrupts
/// disabled, remove the INTC 3 handler (`*0x52909c`) and disable INTC 3 when
/// that leaves none (remove result 0), re-enabling interrupts when they were
/// on; then delete the sema (`*0x453438`, reset to -1) when one is held.
/// Returns 0, or -1 when the handler removal or the sema delete fails (each
/// prints).
pub fn sce_usb_kb_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        c.g[RA][0] = sx(0x0040_c23c);
        c.g[S2][0] = 0;
        callee::DINTR.call(c, 0x0040_c23c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = 3;
        c.g[V0][0] = 0x530000;
        c.g[RA][0] = sx(0x0040_c250);
        c.g[A1][0] = Addr(0x0052_909c).lw(c);
        callee::REMOVE_INTC_HANDLER.call(c, 0x0040_c250)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0040_c264);
            c.g[A0][0] = 3;
            callee::DISABLE_INTC.call(c, 0x0040_c264)?;
        }
        c.cycles += 2;
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x0040_c274);
            callee::EINTR.call(c, 0x0040_c274)?;
        }
        c.cycles += 2;
        if (c.g[S0][0] as i64) < 0 {
            c.g[S1][0] = 0x450000;
            c.cycles += 5;
            c.g[A0][0] = 0x4a0000;
            c.g[S2][0] = 1;
            c.g[A0][0] = sx(0x0049_8278);
            callee::PRINTF.call(c, 0x0040_c28c)?;
            c.g[S1][0] = 0x450000;
        } else {
            c.g[S1][0] = 0x450000;
        }
        c.cycles += 4;
        c.g[S0][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x3438)).lw(c);
        let missing = c.g[A0][0] == c.g[S0][0];
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if !missing {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0040_c2a8);
            callee::DELETE_SEMA.call(c, 0x0040_c2a8)?;
            if (c.g[V0][0] as i64) >= 0 {
                Addr((c.g[S1][0] as u32).wrapping_add(0x3438)).sw(c, c.g[S0][0]);
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            } else {
                // Delete failed: the original breaks straight past the
                // `+1; V0 = -1` of the success path (V0 is already -1 here).
                c.cycles += 6;
                c.g[A0][0] = 0x4a0000;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[RA][0] = sx(0x0040_c2c0);
                c.g[A0][0] = sx(0x0049_82a8);
                callee::PRINTF.call(c, 0x0040_c2c0)?;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            }
        }
        c.cycles += 7;
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        if c.g[S2][0] == 0 {
            c.g[V0][0] = 0;
        }
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceUsbKbGetInfo(int *out)` (0x40c2e8): fetch the keyboard
/// connection info over RPC. Returns -1 while down, -2 when the RPC
/// fails, else 0.
pub fn sce_usb_kb_get_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0x450000;
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.g[S0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[A0][0] = Addr(USBKB_SEMA).lw(c);
        let down = c.g[A0][0] == c.g[V0][0];
        frame.restore_ra(c, 0x20);
        if !down {
            c.cycles += 18;
            c.g[RA][0] = sx(0x0040_c314);
            callee::WAIT_SEMA.call(c, 0x0040_c314)?;
            c.g[A0][0] = 0x530000;
            c.g[A3][0] = 0x530000;
            c.g[T1][0] = 0x530000;
            c.g[T3][0] = 0x410000;
            c.g[A0][0] = sx(USBKB_CLIENT);
            c.g[A3][0] = sx(USBKB_SEND);
            c.g[T1][0] = sx(USBKB_SEND2);
            c.g[T3][0] = sx(USBKB_END_CALLBACK);
            Addr(c.g[SP][0] as u32).sw(c, 0);
            c.g[A1][0] = 1;
            c.g[A2][0] = 1;
            c.g[T0][0] = 0x10;
            c.g[RA][0] = sx(0x0040_c34c);
            c.g[T2][0] = 0x90;
            callee::CALL_RPC.call(c, 0x0040_c34c)?;
            let ok = c.g[V0][0] == 0;
            c.g[A0][0] = 0x4a0000;
            if !ok {
                c.cycles += 5;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0040_c360);
                c.g[A0][0] = sx(0x0049_82d0);
                callee::PRINTF.call(c, 0x0040_c360)?;
                c.g[V0][0] = 0xffff_ffff_ffff_fffe;
            } else {
                c.cycles += 6;
                c.g[V1][0] = sx(USBKB_COUNT);
                c.g[A0][0] = 1;
                c.g[V1][0] = sx(USBKB_COUNT);
                c.g[V0][0] = 0;
                Addr((c.g[V1][0] as u32).wrapping_add(0x98)).sw(c, c.g[S0][0]);
                Addr((c.g[V1][0] as u32).wrapping_add(0x90)).sw(c, c.g[A0][0]);
            }
            c.cycles += 1;
            frame.restore_ra(c, 0x20);
        }
        c.cycles += 3;
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceUsbKbGetLocation(int idx, int *out)` (0x40c470): fetch
/// keyboard `idx`'s location over RPC. Returns -1 while down, -5 for a
/// bad index, -2 when the RPC fails, else 0.
pub fn sce_usb_kb_get_location(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x450000;
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[A0][0] = Addr(USBKB_SEMA).lw(c);
        c.g[S3][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x50), c.g[RA][0]);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        let down = c.g[A0][0] == c.g[V0][0];
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        if !down {
            c.cycles += 5;
            c.g[V1][0] = 0x530000;
            c.g[V0][0] = Addr(USBKB_COUNT).lw(c);
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let known = c.g[V0][0] != 0;
            c.g[S2][0] = sx(USBKB_COUNT);
            if !known {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_fff5;
            } else {
                c.cycles += 19;
                c.g[S0][0] = 0x530000;
                Addr(USBKB_SEND).sb(c, c.g[S1][0]);
                c.g[RA][0] = sx(0x0040_c4cc);
                c.g[S0][0] = sx(USBKB_SEND);
                callee::WAIT_SEMA.call(c, 0x0040_c4cc)?;
                c.g[A0][0] = 0x530000;
                c.g[T1][0] = 0x530000;
                c.g[T3][0] = 0x410000;
                c.g[A0][0] = sx(USBKB_CLIENT);
                c.g[A3][0] = c.g[S0][0];
                c.g[T1][0] = sx(USBKB_SEND2);
                c.g[T3][0] = sx(USBKB_END_CALLBACK);
                Addr(c.g[SP][0] as u32).sw(c, 0);
                c.g[A1][0] = 3;
                c.g[A2][0] = 1;
                c.g[T0][0] = 0x10;
                c.g[RA][0] = sx(0x0040_c500);
                c.g[T2][0] = 0x10;
                callee::CALL_RPC.call(c, 0x0040_c500)?;
                let ok = c.g[V0][0] == 0;
                c.g[A0][0] = 0x4a0000;
                if !ok {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0040_c514);
                    c.g[A0][0] = sx(0x0049_8320);
                    callee::PRINTF.call(c, 0x0040_c514)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = 3;
                    Addr((c.g[S2][0] as u32).wrapping_add(0x98)).sw(c, c.g[S3][0]);
                    Addr((c.g[S2][0] as u32).wrapping_add(0x90)).sw(c, c.g[V0][0]);
                    Addr((c.g[S2][0] as u32).wrapping_add(0x94)).sw(c, c.g[S1][0]);
                    c.g[V0][0] = 0;
                }
            }
        }
        c.cycles += 7;
        frame.restore_ra(c, 0x50);
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `sceUsbKbInit`'s entry and the heads of its loops, where an unwinding
/// event leaves it and the trampoline re-enters it.
const KB_INIT_ENTRY: u32 = 0x0040_c028;
/// Bind retry: reload the spin counter.
const KB_INIT_RETRY: u32 = 0x0040_c050;
/// The delay spin before the next bind.
const KB_INIT_SPIN: u32 = 0x0040_c060;
/// The per-keyboard state reset.
const KB_INIT_STATES: u32 = 0x0040_c150;
const KB_INIT_END: u32 = 0x0040_c220;

/// `int sceUsbKbInit(unsigned int *count_out)` (0x40c028): start the
/// driver: init the RPC, bind (spinning 0x10000 passes and binding again
/// while the server word is still clear), create the sema, fetch the
/// keyboard count (1..127, also stored to `*count_out`), reset each
/// keyboard's state entry and install and enable the interrupt handler.
/// Returns 0 on success, -1 on any failure (the sema is deleted again
/// after it was created). Resumable at the loop heads.
pub fn sce_usb_kb_init(c: &mut Cpu) {
    guest(c, |c| {
        let mut at = c.pc;
        if !matches!(
            at,
            KB_INIT_ENTRY | KB_INIT_RETRY | KB_INIT_SPIN | KB_INIT_STATES
        ) {
            c.jump_out(at, KB_INIT_ENTRY, KB_INIT_END);
            return Ok(());
        }
        let frame = Frame { size: 0x100 };
        'exit: {
            let reset_states = at == KB_INIT_STATES
                || 'setup: {
                    if at == KB_INIT_ENTRY {
                        c.cycles += 10;
                        let frame = Frame::open(c, 0x100);
                        c.sd(frame.at(c, 0xe0), c.g[S2][0]);
                        c.g[S2][0] = c.g[A0][0];
                        c.sd(frame.at(c, 0xd0), c.g[S1][0]);
                        c.sd(frame.at(c, 0xc0), c.g[S0][0]);
                        frame.save_ra(c, 0xf0);
                        c.g[A0][0] = 0;
                        callee::INIT_RPC.call(c, 0x0040_c048)?;
                        c.g[S1][0] = 0x530000;
                    }
                    loop {
                        if at == KB_INIT_RETRY {
                            c.cycles += 4;
                            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                            c.g[A2][0] = 0x10000;
                            at = KB_INIT_SPIN;
                        }
                        if at == KB_INIT_SPIN {
                            loop {
                                c.cycles += 7;
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                                if c.g[A2][0] == c.g[V0][0] {
                                    break;
                                }
                                if back_edge_unwinds(c, KB_INIT_SPIN) {
                                    return Ok(());
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x6f60));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0xffff_ffff_8000_0211;
                        c.g[A2][0] = 0;
                        callee::BIND_RPC.call(c, 0x0040_c094)?;
                        let failed = (c.g[V0][0] as i64) < 0;
                        c.g[A0][0] = 0x4a0000;
                        if failed {
                            c.cycles += 4;
                            c.g[A0][0] = sx(0x0049_8240);
                            callee::PRINTF.call(c, 0x0040_c1f4)?;
                            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                            break 'exit;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x24)).lw(c);
                        let bound = c.g[V0][0] != 0;
                        c.g[V0][0] = 1;
                        if bound {
                            break;
                        }
                        if back_edge_unwinds(c, KB_INIT_RETRY) {
                            return Ok(());
                        }
                        at = KB_INIT_RETRY;
                    }
                    c.cycles += 8;
                    c.g[A0][0] = sx(frame.at(c, 0x90));
                    Addr(frame.at(c, 0x94)).sw(c, c.g[V0][0]);
                    c.g[S1][0] = 0x450000;
                    Addr(frame.at(c, 0x98)).sw(c, c.g[V0][0]);
                    c.g[S0][0] = 0xffff_ffff_ffff_ffff;
                    callee::CREATE_SEMA.call(c, 0x0040_c0c0)?;
                    let created = c.g[V0][0] != c.g[S0][0];
                    Addr(USBKB_SEMA).sw(c, c.g[V0][0]);
                    if !created {
                        c.cycles += 5;
                        c.g[A0][0] = sx(0x0049_8258);
                        callee::PRINTF.call(c, 0x0040_c0d4)?;
                        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                        break 'exit;
                    }
                    'failed: {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[SP][0];
                        callee::KB_GET_INFO.call(c, 0x0040_c0e4)?;
                        let failed = c.g[V0][0] != 0;
                        c.g[A0][0] = 0;
                        if failed {
                            break 'failed;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = sx(frame.at(c, 0xb0));
                        callee::KB_SYNC.call(c, 0x0040_c0f4)?;
                        c.g[V1][0] = Addr(frame.at(c, 0xb0)).lw(c);
                        let failed = c.g[V1][0] != 0;
                        c.g[A0][0] = Addr(frame.at(c, 0)).lw(c);
                        if failed {
                            break 'failed;
                        }
                        c.cycles += 2;
                        let none = (c.g[A0][0] as i64) <= 0;
                        c.g[V0][0] = ((c.g[A0][0] as i64) < 128) as u64;
                        if none {
                            break 'failed;
                        }
                        c.cycles += 2;
                        let too_many = c.g[V0][0] == 0;
                        c.g[A3][0] = 0x530000;
                        if too_many {
                            break 'failed;
                        }
                        c.cycles += 7;
                        c.g[V0][0] = sx(USBKB_FIRST_STATE);
                        c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7000));
                        Addr((c.g[A3][0] as u32).wrapping_sub(0x7000)).sw(c, c.g[A0][0]);
                        Addr(c.g[S2][0] as u32).sw(c, c.g[A0][0]);
                        Addr((c.g[V1][0] as u32).wrapping_add(4)).sw(c, c.g[V0][0]);
                        // The state pointer is a nonzero constant: the
                        // branch back to the failure exit never runs.
                        c.cycles += 2;
                        let none = c.g[A0][0] == 0;
                        c.g[A2][0] = 0;
                        c.cycles += 2;
                        c.g[T0][0] = 0x410000;
                        if none {
                            c.g[S0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7000));
                            break 'setup false;
                        }
                        c.g[V1][0] = 0x38;
                        break 'setup true;
                    }
                    // GetInfo/Sync/count failure: drop the sema.
                    c.cycles += 5;
                    c.g[A0][0] = Addr(USBKB_SEMA).lw(c);
                    callee::DELETE_SEMA.call(c, 0x0040_c134)?;
                    Addr(USBKB_SEMA).sw(c, c.g[S0][0]);
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    break 'exit;
                };
            if reset_states {
                loop {
                    c.cycles += 22;
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7000));
                    c.mult(0, V1, A2, V1);
                    c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(4)).lw(c);
                    c.g[A0][0] = 1;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    let entry = Addr(c.g[V1][0] as u32);
                    entry.off(8).sw(c, c.g[A0][0]);
                    entry.off(0x34).sw(c, 0);
                    entry.off(0).sw(c, c.g[A0][0]);
                    entry.off(4).sw(c, c.g[A0][0]);
                    entry.off(0xc).sw(c, 0);
                    entry.off(0x10).sw(c, 0);
                    entry.off(0x14).sw(c, 0);
                    entry.off(0x18).sw(c, 0);
                    entry.off(0x1c).sw(c, 0);
                    entry.off(0x2c).sw(c, 0);
                    entry.off(0x30).sw(c, 0);
                    entry.off(0x20).sh(c, 0);
                    c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
                    c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V1][0] = 0x38;
                    if !more {
                        break;
                    }
                    if back_edge_unwinds(c, KB_INIT_STATES) {
                        return Ok(());
                    }
                }
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7000));
            }
            c.cycles += 7;
            c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x3078));
            Addr((c.g[S0][0] as u32).wrapping_add(0x10)).sw(c, 0);
            c.g[A0][0] = 3;
            c.g[A2][0] = 0xffff_ffff_ffff_ffff;
            callee::ADD_INTC_HANDLER.call(c, 0x0040_c1cc)?;
            let added = (c.g[V0][0] as i64) >= 0;
            Addr((c.g[S0][0] as u32).wrapping_add(0x9c)).sw(c, c.g[V0][0]);
            if added {
                c.cycles += 3;
                c.g[A0][0] = 3;
                callee::ENABLE_INTC.call(c, 0x0040_c204)?;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 6;
                c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x3438)).lw(c);
                callee::DELETE_SEMA.call(c, 0x0040_c1dc)?;
                c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                Addr((c.g[S1][0] as u32).wrapping_add(0x3438)).sw(c, c.g[V1][0]);
            }
        }
        c.cycles += 6;
        frame.restore_ra(c, 0xf0);
        c.g[S2][0] = c.ld(frame.at(c, 0xe0));
        c.g[S1][0] = c.ld(frame.at(c, 0xd0));
        c.g[S0][0] = c.ld(frame.at(c, 0xc0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceUsbKbRead(int idx, int *out)` (0x40c390): read keyboard
/// `idx`'s key data over RPC. Returns -1 while down, -5 for a bad
/// index, -2 when the RPC fails, else 0.
pub fn sce_usb_kb_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x450000;
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[A0][0] = Addr(USBKB_SEMA).lw(c);
        c.g[S3][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x50), c.g[RA][0]);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        let down = c.g[A0][0] == c.g[V0][0];
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        if !down {
            c.cycles += 5;
            c.g[V1][0] = 0x530000;
            c.g[V0][0] = Addr(USBKB_COUNT).lw(c);
            c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
            let known = c.g[V0][0] != 0;
            c.g[S2][0] = sx(USBKB_COUNT);
            if !known {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_fff5;
            } else {
                c.cycles += 19;
                c.g[S0][0] = 0x530000;
                Addr(USBKB_SEND).sb(c, c.g[S1][0]);
                c.g[RA][0] = sx(0x0040_c3ec);
                c.g[S0][0] = sx(USBKB_SEND);
                callee::WAIT_SEMA.call(c, 0x0040_c3ec)?;
                c.g[A0][0] = 0x530000;
                c.g[T1][0] = 0x530000;
                c.g[T3][0] = 0x410000;
                c.g[A0][0] = sx(USBKB_CLIENT);
                c.g[A3][0] = c.g[S0][0];
                c.g[T1][0] = sx(USBKB_SEND2);
                c.g[T3][0] = sx(USBKB_END_CALLBACK);
                Addr(c.g[SP][0] as u32).sw(c, 0);
                c.g[A1][0] = 2;
                c.g[A2][0] = 1;
                c.g[T0][0] = 0x10;
                c.g[RA][0] = sx(0x0040_c420);
                c.g[T2][0] = 0x50;
                callee::CALL_RPC.call(c, 0x0040_c420)?;
                let ok = c.g[V0][0] == 0;
                c.g[A0][0] = 0x4a0000;
                if !ok {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0040_c434);
                    c.g[A0][0] = sx(0x0049_82f8);
                    callee::PRINTF.call(c, 0x0040_c434)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = 2;
                    Addr((c.g[S2][0] as u32).wrapping_add(0x98)).sw(c, c.g[S3][0]);
                    Addr((c.g[S2][0] as u32).wrapping_add(0x90)).sw(c, c.g[V0][0]);
                    Addr((c.g[S2][0] as u32).wrapping_add(0x94)).sw(c, c.g[S1][0]);
                    c.g[V0][0] = 0;
                }
            }
        }
        c.cycles += 7;
        frame.restore_ra(c, 0x50);
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceUsbKbSetArrangement(int idx, int arr)` (0x40c7c0): set
/// keyboard `idx`'s arrangement (0..2). Returns -5 for a bad index, -4
/// for a bad arrangement, else 0.
pub fn sce_usb_kb_set_arrangement(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[V1][0] = 0x530000;
        c.g[A2][0] = c.g[A0][0];
        c.g[V0][0] = Addr(USBKB_COUNT).lw(c);
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let known = c.g[V0][0] != 0;
        c.g[A0][0] = sx(USBKB_COUNT);
        if known {
            c.cycles += 3;
            c.g[V0][0] = (c.g[A1][0] < 3) as u64;
            let arr_ok = c.g[V0][0] != 0;
            c.g[V1][0] = 0x38;
            if arr_ok {
                c.cycles += 6;
                c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
                c.mult(0, V1, A2, V1);
                c.g[V0][0] = 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let ra = c.g[RA][0] as u32;
                Addr((c.g[V1][0] as u32).wrapping_add(USBKB_ARRANGEMENT)).sw(c, c.g[A1][0]);
                c.pc = ra;
            } else {
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[V0][0] = 0xffff_ffff_ffff_fff4;
                c.pc = ra;
            }
        } else {
            c.cycles += 2;
            let ra = c.g[RA][0] as u32;
            c.g[V0][0] = 0xffff_ffff_ffff_fff5;
            c.pc = ra;
        }
        Ok(())
    })
}

/// `int sceUsbKbSetCodeType(int idx, int type)` (0x40c770): set keyboard
/// `idx`'s code type (0 = raw, 1 = converted). Returns -5 for a bad
/// index, -4 for a bad type, else 0.
pub fn sce_usb_kb_set_code_type(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[V1][0] = 0x530000;
        c.g[A2][0] = c.g[A0][0];
        c.g[V0][0] = Addr(USBKB_COUNT).lw(c);
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let known = c.g[V0][0] != 0;
        c.g[A0][0] = sx(USBKB_COUNT);
        if known {
            c.cycles += 3;
            c.g[V0][0] = (c.g[A1][0] < 2) as u64;
            let type_ok = c.g[V0][0] != 0;
            c.g[V1][0] = 0x38;
            if type_ok {
                c.cycles += 6;
                c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
                c.mult(0, V1, A2, V1);
                c.g[V0][0] = 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let ra = c.g[RA][0] as u32;
                Addr((c.g[V1][0] as u32).wrapping_add(USBKB_CODE_TYPE)).sw(c, c.g[A1][0]);
                c.pc = ra;
            } else {
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[V0][0] = 0xffff_ffff_ffff_fff4;
                c.pc = ra;
            }
        } else {
            c.cycles += 2;
            let ra = c.g[RA][0] as u32;
            c.g[V0][0] = 0xffff_ffff_ffff_fff5;
            c.pc = ra;
        }
        Ok(())
    })
}

/// `int sceUsbKbSetLEDMode(int idx, int mode)` (0x40c628): set keyboard
/// `idx`'s LED mode (0..2) over RPC, recording it in the entry head.
/// Returns -1 while down, -5 for a bad index, -4 for a bad mode, -2
/// when the RPC fails, else 0.
pub fn sce_usb_kb_set_led_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x450000;
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[A2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[A0][0] = Addr(USBKB_SEMA).lw(c);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        let down = c.g[A0][0] == c.g[V0][0];
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        if !down {
            c.cycles += 5;
            c.g[V1][0] = 0x530000;
            c.g[V0][0] = Addr(USBKB_COUNT).lw(c);
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let known = c.g[V0][0] != 0;
            c.g[S2][0] = sx(USBKB_COUNT);
            if !known {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_fff5;
            } else {
                c.cycles += 3;
                c.g[V0][0] = (c.g[A2][0] < 3) as u64;
                let mode_ok = c.g[V0][0] != 0;
                c.g[V0][0] = 0x38;
                if !mode_ok {
                    c.cycles += 2;
                    c.g[V0][0] = 0xffff_ffff_ffff_fff4;
                } else {
                    c.cycles += 24;
                    c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_add(4)).lw(c);
                    c.mult(0, V0, S1, V0);
                    c.g[V1][0] = 0x530000;
                    c.g[S0][0] = sx(USBKB_SEND);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    Addr(c.g[V0][0] as u32).sw(c, c.g[A2][0]);
                    Addr(USBKB_SEND).sb(c, c.g[S1][0]);
                    c.g[RA][0] = sx(0x0040_c6a8);
                    Addr((c.g[S0][0] as u32).wrapping_add(1)).sb(c, c.g[A2][0]);
                    callee::WAIT_SEMA.call(c, 0x0040_c6a8)?;
                    c.g[A0][0] = 0x530000;
                    c.g[T1][0] = 0x530000;
                    c.g[T3][0] = 0x410000;
                    c.g[A0][0] = sx(USBKB_CLIENT);
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T1][0] = sx(USBKB_SEND2);
                    c.g[T3][0] = sx(USBKB_END_CALLBACK);
                    Addr(c.g[SP][0] as u32).sw(c, 0);
                    c.g[A1][0] = 5;
                    c.g[A2][0] = 1;
                    c.g[T0][0] = 0x10;
                    c.g[RA][0] = sx(0x0040_c6dc);
                    c.g[T2][0] = 0x10;
                    callee::CALL_RPC.call(c, 0x0040_c6dc)?;
                    let ok = c.g[V0][0] == 0;
                    c.g[A0][0] = 0x4a0000;
                    if !ok {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x0040_c6f0);
                        c.g[A0][0] = sx(0x0049_8380);
                        callee::PRINTF.call(c, 0x0040_c6f0)?;
                        c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = 5;
                        Addr((c.g[S2][0] as u32).wrapping_add(0x94)).sw(c, c.g[S1][0]);
                        Addr((c.g[S2][0] as u32).wrapping_add(0x90)).sw(c, c.g[V0][0]);
                        Addr((c.g[S2][0] as u32).wrapping_add(0x98)).sw(c, 0);
                        c.g[V0][0] = 0;
                    }
                }
            }
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x40);
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceUsbKbSetLEDStatus(int idx, int status)` (0x40c550): set
/// keyboard `idx`'s LED status (low byte) over RPC. Returns -1 while
/// down, -5 for a bad index, -2 when the RPC fails, else 0.
pub fn sce_usb_kb_set_led_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x450000;
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[A1][0] &= 0xff;
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[A0][0] = Addr(USBKB_SEMA).lw(c);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        let down = c.g[A0][0] == c.g[V0][0];
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        if !down {
            c.cycles += 5;
            c.g[V1][0] = 0x530000;
            c.g[V0][0] = Addr(USBKB_COUNT).lw(c);
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let known = c.g[V0][0] != 0;
            c.g[S2][0] = sx(USBKB_COUNT);
            if !known {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_fff5;
            } else {
                c.cycles += 20;
                c.g[V0][0] = 0x530000;
                c.g[S0][0] = sx(USBKB_SEND);
                Addr(USBKB_SEND).sb(c, c.g[S1][0]);
                c.g[RA][0] = sx(0x0040_c5ac);
                Addr((c.g[S0][0] as u32).wrapping_add(1)).sb(c, c.g[A1][0]);
                callee::WAIT_SEMA.call(c, 0x0040_c5ac)?;
                c.g[A0][0] = 0x530000;
                c.g[T1][0] = 0x530000;
                c.g[T3][0] = 0x410000;
                c.g[A0][0] = sx(USBKB_CLIENT);
                c.g[A3][0] = c.g[S0][0];
                c.g[T1][0] = sx(USBKB_SEND2);
                c.g[T3][0] = sx(USBKB_END_CALLBACK);
                Addr(c.g[SP][0] as u32).sw(c, 0);
                c.g[A1][0] = 4;
                c.g[A2][0] = 1;
                c.g[T0][0] = 0x10;
                c.g[RA][0] = sx(0x0040_c5e0);
                c.g[T2][0] = 0x10;
                callee::CALL_RPC.call(c, 0x0040_c5e0)?;
                let ok = c.g[V0][0] == 0;
                c.g[A0][0] = 0x4a0000;
                if !ok {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0040_c5f4);
                    c.g[A0][0] = sx(0x0049_8350);
                    callee::PRINTF.call(c, 0x0040_c5f4)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = 4;
                    Addr((c.g[S2][0] as u32).wrapping_add(0x94)).sw(c, c.g[S1][0]);
                    Addr((c.g[S2][0] as u32).wrapping_add(0x90)).sw(c, c.g[V0][0]);
                    Addr((c.g[S2][0] as u32).wrapping_add(0x98)).sw(c, 0);
                    c.g[V0][0] = 0;
                }
            }
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x40);
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceUsbKbSetReadMode(int idx, int mode)` (0x40d430): set keyboard
/// `idx`'s read mode (0/1). A change also clears the pending buffer state
/// and the shared 0x529010 word. Returns -5 for a bad index, -4 for a
/// bad mode, else 0.
pub fn sce_usb_kb_set_read_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[V1][0] = 0x530000;
        c.g[A2][0] = c.g[A0][0];
        c.g[V0][0] = Addr(USBKB_COUNT).lw(c);
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let known = c.g[V0][0] != 0;
        c.g[A3][0] = sx(USBKB_COUNT);
        if !known {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_fff5;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (c.g[A1][0] < 2) as u64;
            let mode_ok = c.g[V0][0] != 0;
            c.g[V0][0] = 0x38;
            if !mode_ok {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_fff4;
            } else {
                c.cycles += 6;
                c.g[A0][0] = Addr((c.g[A3][0] as u32).wrapping_add(4)).lw(c);
                c.mult(0, V0, A2, V0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(USBKB_READ_MODE)).lw(c);
                if c.g[A1][0] == c.g[V1][0] {
                    Addr((c.g[V0][0] as u32).wrapping_add(USBKB_READ_MODE)).sw(c, c.g[A1][0]);
                } else {
                    c.cycles += 6;
                    Addr((c.g[V0][0] as u32).wrapping_add(0x14)).sw(c, 0);
                    Addr((c.g[V0][0] as u32).wrapping_add(USBKB_WIDX)).sw(c, 0);
                    Addr((c.g[V0][0] as u32).wrapping_add(USBKB_RIDX)).sw(c, 0);
                    Addr((c.g[A3][0] as u32).wrapping_add(0x10)).sw(c, 0);
                    Addr((c.g[V0][0] as u32).wrapping_add(0x20)).sh(c, 0);
                    Addr((c.g[V0][0] as u32).wrapping_add(USBKB_READ_MODE)).sw(c, c.g[A1][0]);
                }
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        ret(c);
        Ok(())
    })
}

/// `int sceUsbKbSetRepeat(int idx, int delay, int rate)` (0x40c728):
/// set keyboard `idx`'s repeat delay/rate (both non-negative).
/// Returns -5 for a bad delay, -4 for a bad rate, else 0.
pub fn sce_usb_kb_set_repeat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let delay_ok = (c.g[A1][0] as i64) >= 0;
        c.g[V0][0] = 0xffff_ffff_ffff_fff5;
        if !delay_ok {
            c.cycles += 2;
            let ra = c.g[RA][0] as u32;
            c.pc = ra;
        } else {
            c.cycles += 2;
            let rate_ok = (c.g[A2][0] as i64) >= 0;
            c.g[V1][0] = 0x38;
            if rate_ok {
                c.cycles += 9;
                c.g[V0][0] = 0x530000;
                c.mult(0, V1, A0, V1);
                c.g[A0][0] = Addr(USBKB_BASE).lw(c);
                c.g[V0][0] = 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).sw(c, c.g[A2][0]);
                Addr((c.g[V1][0] as u32).wrapping_add(0x18)).sw(c, c.g[A1][0]);
                let ra = c.g[RA][0] as u32;
                Addr((c.g[V1][0] as u32).wrapping_add(0x14)).sw(c, 0);
                c.pc = ra;
            } else {
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[V0][0] = 0xffff_ffff_ffff_fff4;
                c.pc = ra;
            }
        }
        Ok(())
    })
}

/// `int sceUsbKbSync(int mode, int *out)` (0x40c810): mode 0 waits for
/// the pending RPC and stores its result in `*out`; mode 1 polls the RPC
/// state (1 while busy, else the pending result is stored and 0
/// returned); other modes fail. Returns -1 while the driver is down.
pub fn sce_usb_kb_sync(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.g[S1][0] = 0x450000;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[A0][0] = Addr(USBKB_SEMA).lw(c);
        c.g[S0][0] = c.g[A1][0];
        let down = c.g[A0][0] == c.g[V0][0];
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        // Set on a bad mode: RA is restored in-branch and the common `+1`
        // below is skipped.
        let mut bad_mode = false;
        if !down {
            c.cycles += 2;
            let blocking = c.g[V1][0] == 0;
            if blocking {
                c.cycles += 9;
                c.g[RA][0] = sx(0x0040_c85c);
                callee::WAIT_SEMA.call(c, 0x0040_c85c)?;
                c.g[RA][0] = sx(0x0040_c864);
                callee::KB_SYNC_RESULT.call(c, 0x0040_c864)?;
                Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                c.g[RA][0] = sx(0x0040_c870);
                c.g[A0][0] = Addr(USBKB_SEMA).lw(c);
                callee::SIGNAL_SEMA.call(c, 0x0040_c870)?;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 1;
                let poll = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0xffff_ffff_ffff_fff5;
                if poll {
                    c.cycles += 5;
                    c.g[A0][0] = sx(USBKB_CLIENT);
                    c.g[RA][0] = sx(0x0040_c884);
                    callee::CHECK_STAT.call(c, 0x0040_c884)?;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = 1;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0040_c894);
                        callee::KB_SYNC_RESULT.call(c, 0x0040_c894)?;
                        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                        c.g[V0][0] = 0;
                    }
                } else {
                    // Bad mode: the original charges `+2` with the in-branch
                    // RA restore, then skips the `+1` below (straight to the
                    // `+4` epilogue).
                    c.cycles += 2;
                    frame.restore_ra(c, 0x20);
                    bad_mode = true;
                }
            }
        }
        if !bad_mode {
            c.cycles += 1;
            frame.restore_ra(c, 0x20);
        }
        c.cycles += 4;
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `libusbkb_SetLEDStatusCallback` (0x0040_cf48): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn libusbkb_set_led_status_callback_0040cf48(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x530000;
    c.g[A1][0] = 0x530000;
    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x6ebf));
    c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_sub(0x6eff));
    c.g[V0][0] = 0xffffffffffffffff;
    c.g[V1][0] = c.g[V1][0] ^ c.g[A0][0];
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0u64;
    }
    ret(c);
}

/// `libusbkb_SetLEDModeCallback` (0x0040_cf68): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn libusbkb_set_led_mode_callback_0040cf68(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x530000;
    c.g[A1][0] = 0x530000;
    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x6ebf));
    c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_sub(0x6eff));
    c.g[V0][0] = 0xffffffffffffffff;
    c.g[V1][0] = c.g[V1][0] ^ c.g[A0][0];
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0u64;
    }
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::require_paths;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::game::lib::sif::tests::SIF_RPC_POOL_COUNT;
    use ssx3_ee_rt::Cpu;

    /// Last RPC result id (read by `libusbkb_SyncResult`).
    pub(crate) const USBKB_RPC_ID: u32 = 0x0052_9090;

    /// Buffered key codes (halfwords).
    pub(crate) const USBKB_KEYS: u32 = 0x22;

    /// Buffered key count.
    pub(crate) const USBKB_NKEYS: u32 = 0x34;

    /// Per-keyboard state stride.
    pub(crate) const USBKB_STRIDE: u32 = 0x38;

    /// CnvKeypadCode: first-byte match, scan hit and scan miss (the miss
    /// scans all 0x15 rows). Fully covered (leaf + bounded event loop).
    #[test]
    fn libusbkb_cnv_keypad_code_matches() {
        let usbkb_cnv_keypad_code_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_d3c0, 4);
            c.pc = 0x0040_d3c0;
            c.g[A0][0] = (seed % 4) as u64;
            c.g[A1][0] = [0x53, 0x54, 0x2f, 0x2a, 0x2d, 0x00, 0xff, 0x80][seed as usize % 8] as u64;
        };
        check(
            ee::s15::f_0040d3c0__generated,
            libusbkb_cnv_keypad_code,
            200,
            usbkb_cnv_keypad_code_call,
        );
        require_paths(
            ee::s15::f_0040d3c0__generated,
            200,
            &[16, 28, 237],
            usbkb_cnv_keypad_code_call,
        );
    }

    /// ConvertKeyCodes: copy and convert paths, zero termination,
    /// empty input/count and the bad-mode return. Fully covered.
    #[test]
    fn libusbkb_convert_key_codes_matches() {
        let usbkb_convert_key_codes_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = kb_setup(c, seed, 0x0040_d118, 4);
            c.pc = 0x0040_d118;
            let base = c.bus.read32(USBKB_BASE);
            for i in 0..4 {
                let e = base + i * USBKB_STRIDE;
                c.bus
                    .write32(e + USBKB_CODE_TYPE, [0, 1, 2][seed as usize % 3]);
                c.bus.write32(e + USBKB_ARRANGEMENT, (seed % 4) as u32);
            }
            c.g[A0][0] = (seed % 5) as u64;
            c.g[A1][0] = [0, 2, 0x22][seed as usize % 3] as u64;
            c.g[A2][0] = [0, 2][seed as usize % 2] as u64;
            let count = [0, 1, 3, 8][seed as usize % 4];
            c.g[A3][0] = count as u64;
            let (src, dst) = (DATA + 0x2000, DATA + 0x2100);
            c.g[T0][0] = src as u64;
            c.g[T1][0] = dst as u64;
            let codes: [u16; 8] = match seed % 4 {
                0 => [0; 8],
                1 => [4, 5, 6, 0x53, 0x2c, 0, 0, 0],
                2 => [0x1e, 0x27, 4, 5, 6, 7, 8, 9],
                _ => [r.next_u32() as u16; 8],
            };
            for (i, &code) in codes.iter().enumerate() {
                c.bus.write16(src + 2 * i as u32, code);
                c.bus.write16(dst + 2 * i as u32, 0x5555);
            }
        };
        check(
            ee::s15::f_0040d118__generated,
            libusbkb_convert_key_codes,
            150,
            usbkb_convert_key_codes_call,
        );
        require_paths(
            ee::s15::f_0040d118__generated,
            150,
            &[38, 41, 49, 69, 108, 119, 246, 6975],
            usbkb_convert_key_codes_call,
        );
    }

    /// KeyBufPop: empty (-1), plain pops and the read-index wrap.
    /// Fully covered (leaf).
    #[test]
    fn libusbkb_key_buf_pop_matches() {
        let usbkb_key_buf_pop_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_d0b0, 4);
            c.pc = 0x0040_d0b0;
            let base = c.bus.read32(USBKB_BASE);
            let e = base;
            let nkeys = [0, 1, 3, 4][seed as usize % 4];
            c.bus.write32(e + USBKB_NKEYS, nkeys);
            c.bus.write32(e + USBKB_RIDX, (seed % 4) as u32);
            for i in 0..4 {
                c.bus.write16(e + USBKB_KEYS + 2 * i, 0x100 + i as u16);
            }
            c.g[A0][0] = 0;
            c.g[A1][0] = (DATA + 0x3000) as u64;
        };
        check(
            ee::s15::f_0040d0b0__generated,
            libusbkb_key_buf_pop,
            200,
            usbkb_key_buf_pop_call,
        );
        require_paths(
            ee::s15::f_0040d0b0__generated,
            200,
            &[11, 24],
            usbkb_key_buf_pop_call,
        );
    }

    /// KeyBufPush: full (-1), plain pushes and the write-index wrap.
    /// Fully covered (leaf).
    #[test]
    fn libusbkb_key_buf_push_matches() {
        let usbkb_key_buf_push_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_d040, 4);
            c.pc = 0x0040_d040;
            let base = c.bus.read32(USBKB_BASE);
            let e = base;
            c.bus
                .write32(e + USBKB_NKEYS, [0, 3, 4, 5][seed as usize % 4]);
            c.bus.write32(e + USBKB_WIDX, [0, 1, 3][seed as usize % 3]);
            c.g[A0][0] = 0;
            c.g[A1][0] = (0x40 + seed % 0x30) as u64;
        };
        check(
            ee::s15::f_0040d040__generated,
            libusbkb_key_buf_push,
            200,
            usbkb_key_buf_push_call,
        );
        require_paths(
            ee::s15::f_0040d040__generated,
            200,
            &[12, 25],
            usbkb_key_buf_push_call,
        );
    }

    /// The USB keyboard read callback folds the HID report into the cooked
    /// output: fresh reports go through the key buffer and the keymap,
    /// stale reports refill from it, empty ones pop the oldest buffered
    /// key. Seeds cover the no-report early return, the fresh-report copy
    /// (empty and keyed), every buffer-submode dispatch arm, the pop hit
    /// and miss, the first-key shortcut, the filter (all-zero and mixed),
    /// the last-report diff (with and without the search table), and all
    /// three convert modes.
    #[test]
    fn libusbkb_read_callback_matches() {
        let usbkb_read_callback_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            lrb_fixture(c, seed);
        };
        check_lohi(
            ee::s15::f_0040c980__generated,
            libusbkb_read_callback,
            240,
            usbkb_read_callback_call,
        );
        require_paths(
            ee::s15::f_0040c980__generated,
            240,
            &[
                34, 46, 54, 55, 57, 62, 65, 66, 68, 90, 107, 117, 120, 121, 124, 125, 127, 128,
                132, 134, 136, 140, 143, 146, 147, 152, 153, 154, 174, 179, 184, 188, 191, 195,
                209, 254, 261, 264, 344, 479, 1188, 1196, 1198, 1208, 1211, 1221, 1231, 1236, 1239,
                1241, 1244, 1251, 1254, 1264, 1270, 1275, 1280, 1283, 1293, 1308, 1321, 1329, 1336,
                1349, 1351, 1359, 1360, 1369, 1384, 1390, 1415, 1416, 1423, 1425, 1498, 1524, 1554,
                1575, 1586, 1587, 1601, 1675,
            ],
            usbkb_read_callback_call,
        );
    }

    /// ResetKeyBuf: pre-filled state is cleared. Fully covered (leaf).
    #[test]
    fn libusbkb_reset_key_buf_matches() {
        let usbkb_reset_key_buf_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_d010, 4);
            c.pc = 0x0040_d010;
            let base = c.bus.read32(USBKB_BASE);
            let idx = (seed % 5) as u32;
            c.g[A0][0] = idx as u64;
            let e = base + idx * USBKB_STRIDE;
            for off in [0x14, 0x2c, 0x30, 0x34] {
                c.bus.write32(e + off, 0x1111_1111);
            }
        };
        check(
            ee::s15::f_0040d010__generated,
            libusbkb_reset_key_buf,
            60,
            usbkb_reset_key_buf_call,
        );
        require_paths(
            ee::s15::f_0040d010__generated,
            60,
            &[12],
            usbkb_reset_key_buf_call,
        );
    }

    /// SyncResult: no result (0) and the callback path for ids 1..=5
    /// (the callback is the real `0x1406c0` leaf, returning 0); ids
    /// 6..=7 also return 0 with no call. Fully covered.
    #[test]
    fn libusbkb_sync_result_matches() {
        let usbkb_sync_result_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_c8c0, 4);
            c.pc = 0x0040_c8c0;
            let id = (seed % 8) as u32;
            c.bus.write32(USBKB_RPC_ID, id);
            if (1..=5).contains(&id) {
                c.bus.write32(USBKB_CB_TABLE + 4 * id, 0x0014_06c0);
            }
        };
        check(
            ee::s15::f_0040c8c0__generated,
            libusbkb_sync_result,
            120,
            usbkb_sync_result_call,
        );
        require_paths(
            ee::s15::f_0040c8c0__generated,
            120,
            &[17, 27],
            usbkb_sync_result_call,
        );
    }

    /// ClearRbuf: down (-1), bad index (-5), the RPC-failure path (-2) and,
    /// with the server answering (`seed / 7` odd), the buffer-state clears
    /// and 0.
    #[test]
    fn sce_usb_kb_clear_rbuf_matches() {
        let usb_kb_clear_rbuf_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_d4a0, 4);
            if seed / 7 % 2 == 1 {
                kb_answered(c, seed, 0x0040_d4a0, DATA + 0x3000);
            }
            c.pc = 0x0040_d4a0;
            c.bus
                .write32(USBKB_SEMA, if seed % 3 == 0 { 0xffff_ffff } else { 0x1234 });
            c.g[A0][0] = [0, 1, 3, 4, 5][seed as usize % 5] as u64;
        };
        check(
            ee::s15::f_0040d4a0__generated,
            sce_usb_kb_clear_rbuf,
            200,
            usb_kb_clear_rbuf_call,
        );
        require_paths(
            ee::s15::f_0040d4a0__generated,
            200,
            &[16, 23, 206, 319],
            usb_kb_clear_rbuf_call,
        );
    }

    /// CnvRawCode: all three tables, the keypad (0x4000) and undefined
    /// (0x8000) paths, case conversion both ways, raw passthrough
    /// (>= 0x100) and the bad-arrangement prints (arr 3 prints the
    /// format string; negative arrangements print an empty format, as
    /// unmapped reads return 0). Seeds from 200 on pair every modifier
    /// and flag word with raw 4 ('a'/'A') and 0x2f ('['/'{' in
    /// arrangement 0) in arrangements 0 and 1.
    #[test]
    fn sce_usb_kb_cnv_raw_code_matches() {
        let usb_kb_cnv_raw_code_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_d268, 4);
            c.pc = 0x0040_d268;
            let (arr, mods, flags, raw) = match seed.checked_sub(200) {
                None => (seed % 6, seed % 4, seed % 4, seed % 14),
                Some(k) => (k % 2, k / 2 % 4, k / 8 % 4, [2, 14][(k / 32 % 2) as usize]),
            };
            c.g[A0][0] =
                [0u64, 1, 2, 3, 0xffff_ffff_ffff_ffff, 0xffff_ffff_ffff_fffe][arr as usize];
            c.g[A1][0] = [0, 2, 0x20, 0x22][mods as usize] as u64;
            c.g[A2][0] = [0, 2, 3, 1][flags as usize] as u64;
            c.g[A3][0] = [
                0, 1, 4, 0x1e, 0x27, 0x28, 0x29, 0x2c, 0x53, 0x54, 0x5d, 0x64, 0x100, 0x1ff, 0x2f,
            ][raw as usize] as u64;
        };
        check(
            ee::s15::f_0040d268__generated,
            sce_usb_kb_cnv_raw_code,
            264,
            usb_kb_cnv_raw_code_call,
        );
        require_paths(
            ee::s15::f_0040d268__generated,
            264,
            &[32, 37, 48, 53, 55, 58, 74, 81, 185, 846, 851, 867],
            usb_kb_cnv_raw_code_call,
        );
    }

    /// End: interrupt already off/on (skips/runs `EIntr`), the sema
    /// missing (0), deleted (a real sema, created in the fixture) or
    /// undeletable (-1, prints). `RemoveIntcHandler` always returns 0,
    /// so the skip-`DisableIntc` path is unreachable with the real
    /// kernel. Fully covered apart from that dead branch.
    #[test]
    fn sce_usb_kb_end_matches() {
        let usb_kb_end_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_c220, 4);
            c.pc = 0x0040_c220;
            if seed % 2 == 0 {
                c.cop0[12] |= 0x1_0000;
            } else {
                c.cop0[12] &= !0x1_0000;
            }
            match seed / 2 % 3 {
                0 => c.bus.write32(USBKB_SEMA, 0xffff_ffff),
                1 => {
                    // A real sema: the delete succeeds and clears the word.
                    c.bus.write32(DATA + 0x5000, 0);
                    c.bus.write32(DATA + 0x5004, 1);
                    c.bus.write32(DATA + 0x5008, 1);
                    c.bus.write32(DATA + 0x500c, 0);
                    c.bus.write32(DATA + 0x5010, 0);
                    c.bus.write32(DATA + 0x5014, 0);
                    c.g[A0][0] = (DATA + 0x5000) as u64;
                    ee::s15::CreateSema(c);
                    let id = c.g[V0][0] as u32;
                    c.bus.write32(USBKB_SEMA, id);
                    c.g[A0][0] = 0;
                }
                _ => c.bus.write32(USBKB_SEMA, 0x1234),
            }
        };
        check(
            ee::s15::f_0040c220__generated,
            sce_usb_kb_end,
            120,
            usb_kb_end_call,
        );
        require_paths(
            ee::s15::f_0040c220__generated,
            120,
            &[69, 70, 84, 85, 1561, 1592],
            usb_kb_end_call,
        );
    }

    /// GetInfo: down (-1), the RPC-failure path (-2) and, with the server
    /// answering (`seed / 7` odd), the pending-request stores and 0.
    #[test]
    fn sce_usb_kb_get_info_matches() {
        let usb_kb_get_info_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_c2e8, 4);
            if seed / 7 % 2 == 1 {
                kb_answered(c, seed, 0x0040_c2e8, DATA + 0x3000);
            }
            c.pc = 0x0040_c2e8;
            c.bus
                .write32(USBKB_SEMA, if seed % 2 == 0 { 0xffff_ffff } else { 0x1234 });
            c.g[A0][0] = (DATA + 0x3000) as u64;
        };
        check(
            ee::s15::f_0040c2e8__generated,
            sce_usb_kb_get_info,
            120,
            usb_kb_get_info_call,
        );
        require_paths(
            ee::s15::f_0040c2e8__generated,
            120,
            &[12, 199, 345],
            usb_kb_get_info_call,
        );
    }

    /// GetLocation: down (-1), bad index (-5, signed) and the RPC-failure
    /// path (-2); a negative index passes the bound check and still
    /// fails the RPC. With the server answering (`seed / 7` odd) the
    /// pending-request stores and 0.
    #[test]
    fn sce_usb_kb_get_location_matches() {
        let usb_kb_get_location_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_c470, 4);
            if seed / 7 % 2 == 1 {
                kb_answered(c, seed, 0x0040_c470, DATA + 0x3000);
            }
            c.pc = 0x0040_c470;
            c.bus
                .write32(USBKB_SEMA, if seed % 3 == 0 { 0xffff_ffff } else { 0x1234 });
            c.g[A0][0] = [0u64, 1, 3, 4, 0xffff_ffff_ffff_ffff][seed as usize % 5];
            c.g[A1][0] = (DATA + 0x3000) as u64;
        };
        check(
            ee::s15::f_0040c470__generated,
            sce_usb_kb_get_location,
            120,
            usb_kb_get_location_call,
        );
        require_paths(
            ee::s15::f_0040c470__generated,
            120,
            &[19, 26, 208, 342],
            usb_kb_get_location_call,
        );
    }

    /// Gate word for [`fake_add_intc_handler`] (in the machine's RAM).
    const ADD_HANDLER_GATE: u32 = DATA + 0x6200;
    const ADD_HANDLER_MAGIC: u32 = 0x4b42_4948;

    /// `AddIntcHandler` with its gate set: fails (-1); the kernel's never
    /// does.
    fn fake_add_intc_handler(c: &mut Cpu) {
        if c.bus.read32(ADD_HANDLER_GATE) != ADD_HANDLER_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_add_intc_handler),
                ee::s15::f_00423a80__generated,
            );
        }
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.cycles += 5;
        crate::abi::ret(c);
    }

    /// `sceUsbKbInit` state entered at `head`: inside its 0x100 frame (saved
    /// `ra` returning to the test), the bind client in s1, `count_out` in s2,
    /// and for the state reset `count` keyboards with the loop at `index`
    /// and their state entries pre-filled. The bind fails (no free packets):
    /// past it the real `sceSifBindRpc` waits for an IOP reply.
    fn kb_init_at(c: &mut Cpu, r: &mut Rng, head: u32, count: u32, index: u32) {
        c.bus.write32(0x0045_5180, 1);
        c.bus.write32(SIF_RPC_POOL_COUNT, 0);
        // Empty bind/sema failure formats: the real `scePrintf` returns at
        // once (a long print would take the events meant for the loops).
        c.bus.write8(0x0049_8240, 0);
        c.bus.write8(0x0049_8258, 0);
        let count_out = DATA + 0x3004;
        c.bus.write32(count_out, 0x5a5a_5a5a);
        c.pc = head;
        if head == KB_INIT_ENTRY {
            c.g[A0][0] = count_out as u64;
            return;
        }
        let s = crate::difftest::SP - 0x100;
        c.g[SP][0] = s as u64;
        crate::difftest::fill(c, r, s, 0x40);
        c.bus.write64(s + 0xf0, crate::difftest::RA as u64);
        c.g[S2][0] = count_out as u64;
        if head == KB_INIT_STATES {
            let states = DATA + 0x1004;
            crate::difftest::fill(c, r, states, 0x38 * count / 4);
            c.bus.write32(USBKB_COUNT, count);
            c.bus.write32(USBKB_BASE, states);
            c.bus.write32(s, count);
            c.g[S1][0] = 0x450000;
            c.g[A3][0] = 0x530000;
            c.g[A2][0] = index as u64;
            c.g[V1][0] = 0x38;
            c.g[T0][0] = 0x410000;
        } else {
            c.g[S1][0] = 0x530000;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            c.g[A2][0] = index as u64;
        }
    }

    /// Init: from the entry (the bind fails and prints, -1), the bind retry
    /// (a full 0x10000-pass spin, then the bind), the spin with a few passes
    /// left, and the state-reset loop for 1..6 keyboards from any index,
    /// followed by the handler install (the kernel's, or a failing fake:
    /// the sema is deleted, -1). Past the bind nothing is reached from the
    /// entry (it waits for an IOP reply).
    #[test]
    fn sce_usb_kb_init_matches() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00423A80),
            fake_add_intc_handler,
            crate::difftest::fakes::prev_cell(fake_add_intc_handler),
        );
        let init_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = kb_setup(c, seed, 0x0040_c028, 4);
            let head =
                [KB_INIT_ENTRY, KB_INIT_RETRY, KB_INIT_SPIN, KB_INIT_STATES][seed as usize % 4];
            let count = 1 + seed / 4 % 6;
            let index = seed / 24 % count;
            kb_init_at(
                c,
                &mut r,
                head,
                count,
                if head == KB_INIT_SPIN {
                    seed / 4 % 5
                } else {
                    index
                },
            );
            if seed / 48 % 2 == 1 {
                c.bus.write32(ADD_HANDLER_GATE, ADD_HANDLER_MAGIC);
            }
        };
        check(
            ee::s15::f_0040c028__generated,
            sce_usb_kb_init,
            96,
            init_call,
        );
        require_paths(
            ee::s15::f_0040c028__generated,
            96,
            &[
                52, 69, 74, 91, 96, 113, 118, 135, 156, 157, 163, 170, 177, 179, 184, 196, 458912,
            ],
            init_call,
        );
    }
    /// Gate block for the `sceUsbKbGetInfo` / `sceUsbKbSync` fakes: the
    /// keyboard count and GetInfo's result, and Sync's result word.
    const KB_RPC_GATE: u32 = DATA + 0x6210;
    const KB_RPC_MAGIC: u32 = 0x4b42_5243;
    const KB_INFO_COUNT: u32 = DATA + 0x6214;
    const KB_INFO_RESULT: u32 = DATA + 0x6218;
    const KB_SYNC_RESULT: u32 = DATA + 0x621c;

    /// `sceUsbKbGetInfo(info)` with its gate set: the keyboard count to
    /// `info[0]`, the set result in v0 (the real one needs the IOP's reply).
    fn fake_kb_get_info(c: &mut Cpu) {
        if c.bus.read32(KB_RPC_GATE) != KB_RPC_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_kb_get_info),
                ee::s15::f_0040c2e8__generated,
            );
        }
        let info = c.g[A0][0] as u32;
        let count = c.bus.read32(KB_INFO_COUNT);
        c.bus.write32(info, count);
        c.g[V0][0] = sx(c.bus.read32(KB_INFO_RESULT));
        c.cycles += 9;
        crate::abi::ret(c);
    }

    /// `sceUsbKbSync(mode, result)` with its gate set: the set result word to
    /// `*result`, 0 in v0.
    fn fake_kb_sync(c: &mut Cpu) {
        if c.bus.read32(KB_RPC_GATE) != KB_RPC_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_kb_sync),
                ee::s15::f_0040c810__generated,
            );
        }
        let result = c.g[A1][0] as u32;
        let word = c.bus.read32(KB_SYNC_RESULT);
        c.bus.write32(result, word);
        c.g[V0][0] = 0;
        c.cycles += 7;
        crate::abi::ret(c);
    }

    /// Init from the entry with the bind answered (at once, or after one
    /// silent wait: the full 0x10000-pass spin and a second bind), then
    /// GetInfo failing or not, Sync's result word zero or not, 0, 1, 3, 127
    /// or 128 keyboards, and the handler install succeeding or failing.
    #[test]
    fn sce_usb_kb_init_bound_matches() {
        for (slot, fake) in [
            (
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0040C2E8),
                fake_kb_get_info as fn(&mut Cpu),
            ),
            (
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0040C810),
                fake_kb_sync,
            ),
            (
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00423A80),
                fake_add_intc_handler,
            ),
        ] {
            crate::difftest::fakes::hook(slot, fake, crate::difftest::fakes::prev_cell(fake));
        }
        let bound_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = kb_setup(c, seed, 0x0040_c028, 4);
            kb_init_at(c, &mut r, KB_INIT_ENTRY, 1, 0);
            crate::game::lib::cdvd::tests::rpc_answered(c, USBKB_CLIENT, seed % 2);
            // A running guest thread: the bind's semaphore syscalls
            // reschedule, and with no thread the machine goes idle.
            let args = (c.g[A0][0], c.g[RA]);
            ssx3_ps2::kernel::start_main_thread(c, KB_INIT_ENTRY, 0, 0);
            (c.g[A0][0], c.g[RA]) = args;
            c.pc = KB_INIT_ENTRY;
            c.bus.write32(KB_RPC_GATE, KB_RPC_MAGIC);
            c.bus.write32(KB_INFO_RESULT, seed / 2 % 2);
            c.bus.write32(KB_SYNC_RESULT, seed / 4 % 2 * 0x31);
            c.bus
                .write32(KB_INFO_COUNT, [0, 1, 3, 127, 128][(seed / 8 % 5) as usize]);
            if seed / 40 % 2 == 1 {
                c.bus.write32(ADD_HANDLER_GATE, ADD_HANDLER_MAGIC);
            }
        };
        check(
            ee::s15::f_0040c028__generated,
            sce_usb_kb_init,
            80,
            bound_call,
        );
        require_paths(
            ee::s15::f_0040c028__generated,
            80,
            &[317, 329, 331, 333, 381, 424, 425, 468, 3153, 3196, 20147],
            bound_call,
        );
    }

    driven!(sce_usb_kb_init_gen_driven, ee::s15::f_0040c028__generated);
    driven!(sce_usb_kb_init_hand_driven, sce_usb_kb_init);

    /// Init with live events: the spin (0x40c060, entered there, from the
    /// bind retry 0x40c050, or from the entry after a first bind answered
    /// without a server) and the state reset (0x40c150) run long enough for
    /// the event, so unwinds re-enter at those heads. The retry's own back
    /// edge comes right after the bind's syscalls, which take the event.
    #[test]
    fn sce_usb_kb_init_events() {
        for (slot, fake) in [
            (
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0040C2E8),
                fake_kb_get_info as fn(&mut Cpu),
            ),
            (
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0040C810),
                fake_kb_sync,
            ),
        ] {
            crate::difftest::fakes::hook(slot, fake, crate::difftest::fakes::prev_cell(fake));
        }
        check_events(
            sce_usb_kb_init_gen_driven,
            sce_usb_kb_init_hand_driven,
            120,
            &|c, seed| {
                let mut r = kb_setup(c, seed, 0x0040_c028, 4);
                let head = [KB_INIT_RETRY, KB_INIT_SPIN, KB_INIT_STATES, KB_INIT_ENTRY]
                    [(seed / 3 % 4) as usize];
                let (count, index) = match head {
                    KB_INIT_STATES => (60 + seed / 12 % 10, seed / 12 % 3),
                    _ => (1, 40 + seed / 12 % 10 * 30),
                };
                kb_init_at(c, &mut r, head, count, index);
                if head == KB_INIT_ENTRY {
                    crate::game::lib::cdvd::tests::rpc_answered(c, USBKB_CLIENT, 1);
                    c.bus.write32(KB_RPC_GATE, KB_RPC_MAGIC);
                    c.bus.write32(KB_INFO_RESULT, 0);
                    c.bus.write32(KB_SYNC_RESULT, 0);
                    c.bus.write32(KB_INFO_COUNT, 2);
                }
                schedule(c, &mut r, seed, head)
            },
        );
    }

    /// Read: down (-1), bad index (-5, unsigned) and the RPC-failure
    /// path (-2); with the server answering (`seed / 7` odd), the
    /// pending-request stores and 0.
    #[test]
    fn sce_usb_kb_read_matches() {
        let usb_kb_read_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_c390, 4);
            if seed / 7 % 2 == 1 {
                kb_answered(c, seed, 0x0040_c390, DATA + 0x3000);
            }
            c.pc = 0x0040_c390;
            c.bus
                .write32(USBKB_SEMA, if seed % 3 == 0 { 0xffff_ffff } else { 0x1234 });
            c.g[A0][0] = [0u64, 1, 3, 4, 0xffff_ffff_ffff_ffff][seed as usize % 5];
            c.g[A1][0] = (DATA + 0x3000) as u64;
        };
        check(
            ee::s15::f_0040c390__generated,
            sce_usb_kb_read,
            120,
            usb_kb_read_call,
        );
        require_paths(
            ee::s15::f_0040c390__generated,
            120,
            &[19, 26, 208, 349],
            usb_kb_read_call,
        );
    }

    /// SetArrangement: bad index (-5), bad arrangement (-4) and stores.
    /// Fully covered (leaf).
    #[test]
    fn sce_usb_kb_set_arrangement_matches() {
        let usb_kb_set_arrangement_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_c7c0, 4);
            c.pc = 0x0040_c7c0;
            c.g[A0][0] = [0, 1, 3, 4, 7][seed as usize % 5] as u64;
            c.g[A1][0] = [0, 1, 2, 3, 4][seed as usize / 5 % 5] as u64;
        };
        check(
            ee::s15::f_0040c7c0__generated,
            sce_usb_kb_set_arrangement,
            120,
            usb_kb_set_arrangement_call,
        );
        require_paths(
            ee::s15::f_0040c7c0__generated,
            120,
            &[8, 11, 15],
            usb_kb_set_arrangement_call,
        );
    }

    /// SetCodeType: bad index (-5), bad type (-4) and stores.
    /// Fully covered (leaf).
    #[test]
    fn sce_usb_kb_set_code_type_matches() {
        let usb_kb_set_code_type_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_c770, 4);
            c.pc = 0x0040_c770;
            c.g[A0][0] = [0, 1, 3, 4, 7][seed as usize % 5] as u64;
            c.g[A1][0] = [0, 1, 2, 3][seed as usize / 5 % 4] as u64;
        };
        check(
            ee::s15::f_0040c770__generated,
            sce_usb_kb_set_code_type,
            120,
            usb_kb_set_code_type_call,
        );
        require_paths(
            ee::s15::f_0040c770__generated,
            120,
            &[8, 11, 15],
            usb_kb_set_code_type_call,
        );
    }

    /// SetLEDMode: down (-1), bad index (-5), bad mode (-4) and the
    /// RPC-failure path (-2); with the server answering (`seed / 7` odd),
    /// the pending-request stores and 0.
    #[test]
    fn sce_usb_kb_set_led_mode_matches() {
        let usb_kb_set_led_mode_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_c628, 4);
            if seed / 7 % 2 == 1 {
                kb_answered(c, seed, 0x0040_c628, DATA + 0x3000);
            }
            c.pc = 0x0040_c628;
            c.bus
                .write32(USBKB_SEMA, if seed % 3 == 0 { 0xffff_ffff } else { 0x1234 });
            c.g[A0][0] = [0, 1, 3, 4][seed as usize % 4] as u64;
            c.g[A1][0] = [0, 1, 2, 3][seed as usize / 4 % 4] as u64;
        };
        check(
            ee::s15::f_0040c628__generated,
            sce_usb_kb_set_led_mode,
            120,
            usb_kb_set_led_mode_call,
        );
        require_paths(
            ee::s15::f_0040c628__generated,
            120,
            &[17, 24, 27, 215, 348],
            usb_kb_set_led_mode_call,
        );
    }

    /// SetLEDStatus: down (-1), bad index (-5) and the RPC-failure path
    /// (-2); with the server answering (`seed / 7` odd), the
    /// pending-request stores and 0.
    #[test]
    fn sce_usb_kb_set_led_status_matches() {
        let usb_kb_set_led_status_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_c550, 4);
            if seed / 7 % 2 == 1 {
                kb_answered(c, seed, 0x0040_c550, DATA + 0x3000);
            }
            c.pc = 0x0040_c550;
            c.bus
                .write32(USBKB_SEMA, if seed % 3 == 0 { 0xffff_ffff } else { 0x1234 });
            c.g[A0][0] = [0, 1, 3, 4][seed as usize % 4] as u64;
            c.g[A1][0] = (seed % 0x140) as u64;
        };
        check(
            ee::s15::f_0040c550__generated,
            sce_usb_kb_set_led_status,
            120,
            usb_kb_set_led_status_call,
        );
        require_paths(
            ee::s15::f_0040c550__generated,
            120,
            &[17, 24, 208, 341],
            usb_kb_set_led_status_call,
        );
    }

    /// SetReadMode: bad index (-5), bad mode (-4), same-mode and
    /// mode-change stores. Fully covered (leaf).
    #[test]
    fn sce_usb_kb_set_read_mode_matches() {
        let usb_kb_set_read_mode_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_d430, 4);
            c.pc = 0x0040_d430;
            let idx = [0u64, 1, 3, 4, 5, 0xffff_ffff][seed as usize % 6];
            c.g[A0][0] = idx as u64;
            let mode = (seed % 4) as u64;
            c.g[A1][0] = mode;
            let base = c.bus.read32(USBKB_BASE);
            let e = base.wrapping_add((idx as u32).wrapping_mul(USBKB_STRIDE));
            c.bus.write32(
                e + USBKB_READ_MODE,
                if seed % 2 == 0 {
                    mode as u32
                } else {
                    (mode ^ 1) as u32
                },
            );
        };
        check(
            ee::s15::f_0040d430__generated,
            sce_usb_kb_set_read_mode,
            200,
            usb_kb_set_read_mode_call,
        );
        require_paths(
            ee::s15::f_0040d430__generated,
            200,
            &[8, 11, 17, 23],
            usb_kb_set_read_mode_call,
        );
    }

    /// SetRepeat: bad delay (-5), bad rate (-4) and stores.
    /// Fully covered (leaf).
    #[test]
    fn sce_usb_kb_set_repeat_matches() {
        let usb_kb_set_repeat_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_c728, 4);
            c.pc = 0x0040_c728;
            c.g[A0][0] = (seed % 3) as u64;
            c.g[A1][0] = [0xffff_ffff_ffff_ffffu64, 0, 5][seed as usize % 3];
            c.g[A2][0] = [0xffff_ffff_ffff_ffffu64, 0, 7][seed as usize / 3 % 3];
        };
        check(
            ee::s15::f_0040c728__generated,
            sce_usb_kb_set_repeat,
            120,
            usb_kb_set_repeat_call,
        );
        require_paths(
            ee::s15::f_0040c728__generated,
            120,
            &[4, 6, 13],
            usb_kb_set_repeat_call,
        );
    }

    /// Sync: down (-1), blocking wait, poll-busy (1), poll-idle (0) and
    /// bad mode (-5). The real sema/syscall/RPC callees run; the pool is
    /// empty so the RPC state stays idle. Fully covered.
    #[test]
    fn sce_usb_kb_sync_matches() {
        let usb_kb_sync_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            kb_setup(c, seed, 0x0040_c810, 4);
            c.pc = 0x0040_c810;
            c.bus
                .write32(USBKB_SEMA, if seed % 4 == 0 { 0xffff_ffff } else { 0x1234 });
            c.g[A0][0] = (seed % 4) as u64;
            c.g[A1][0] = (DATA + 0x3000) as u64;
            c.bus.write32(USBKB_RPC_ID, 0);
            if seed / 4 % 2 == 0 {
                let pkt = DATA + 0x4000;
                c.bus.write32(USBKB_CLIENT, pkt);
                c.bus.write32(USBKB_CLIENT + 4, 0x7777);
                c.bus.write32(pkt + 0x18, 0x7777);
                c.bus.write32(pkt + 0x10, 1);
            } else {
                c.bus.write32(USBKB_CLIENT, 0);
            }
        };
        check(
            ee::s15::f_0040c810__generated,
            sce_usb_kb_sync,
            200,
            usb_kb_sync_call,
        );
        require_paths(
            ee::s15::f_0040c810__generated,
            200,
            &[15, 21, 38, 51],
            usb_kb_sync_call,
        );
    }

    /// Common fixture: ELF image (format strings, key tables), scrambled
    /// registers, game-like gp, the keyboard array and an empty RPC pool.
    pub(crate) fn kb_setup(c: &mut Cpu, seed: u32, salt: u64, count: u32) -> Rng {
        let mut r = Rng(0x0040_c028 ^ (seed as u64) << 12 ^ salt);
        elf_image(c);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        kb_array(c, count);
        r
    }

    pub(crate) fn lrb_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x51ab_0000 ^ seed as u64);
        scramble(c, &mut r);
        c.pc = 0x0040_c980;
        // Globals the callback reads.
        c.bus.write32(LRB_ST + 0x98, LRB_OUT);
        c.bus.write32(LRB_ST + 0x94, 0);
        c.bus.write32(LRB_ST + 0x4, LRB_DEV);
        // HID report: modifiers, count (0xff = no report), reserved, keys.
        let count = match seed % 9 {
            0 => 0xff,
            1 => 0,
            _ => 1 + seed % 5,
        };
        c.bus.write8(LRB_REP, (seed % 4) as u8);
        c.bus.write8(LRB_REP + 1, count as u8);
        c.bus.write8(LRB_REP + 2, (seed % 3) as u8);
        for k in 0..6u32 {
            c.bus.write8(LRB_REP + 4 + k, lrb_key(seed, k));
        }
        // Per-device state: fresh-report mark, buffer submode, refill
        // counts, second/last keys.
        c.bus
            .write32(LRB_DEV + 0xc, if seed % 4 == 0 { 1 } else { 2 + seed % 2 });
        c.bus
            .write32(LRB_DEV + 0x10, [0, 1, 2, 3, 0xffff_ffff][seed as usize % 5]);
        c.bus.write32(LRB_DEV + 0x14, (seed / 5 % 4) as u32);
        c.bus.write32(LRB_DEV + 0x18, (seed / 7 % 4) as u32);
        c.bus.write32(LRB_DEV + 0x1c, (seed / 11 % 4) as u32);
        c.bus.write32(LRB_DEV + 0x20, (seed % 8) as u32);
        // ConvertKeyCodes mode and the raw-code table select: 0, 1 or 2
        // (anything else would reach scePrintf).
        c.bus.write32(LRB_DEV + 0x4, (seed / 13 % 3) as u32);
        c.bus.write32(LRB_DEV + 0x8, (seed / 17 % 3) as u32);
        // Search-table size (bounds the inner table scan) and entries: one
        // seed in four matches the first filtered key.
        c.bus.write32(LRB_ST + 0x10, (seed / 19 % 3) as u32);
        let t0 = if seed % 2 == 0 {
            lrb_key(seed, 0) as u16
        } else {
            0x77
        };
        c.bus.write16(LRB_ST + 0x14, t0);
        c.bus.write16(LRB_ST + 0x16, (0x10 + seed % 0x40) as u16);
        c.bus.write16(LRB_ST + 0x18, (0x20 + seed % 0x30) as u16);
    }

    /// Keyboard array at `DATA + 0x1000`: `count` entries, zeroed state.
    /// Returns the base address. Also forces the RPC pool empty so the
    /// real `sceSifCallRpc` fails fast (-1) instead of talking to the IOP.
    pub(crate) fn kb_array(c: &mut Cpu, count: u32) -> u32 {
        let base = DATA + 0x1000;
        c.bus.write32(USBKB_COUNT, count);
        c.bus.write32(USBKB_BASE, base);
        c.bus.write32(SIF_RPC_POOL_COUNT, 0);
        for i in 0..8 {
            let e = base + i * USBKB_STRIDE;
            for off in [0, 4, 8, 0xc, 0x14, 0x18, 0x1c, 0x2c, 0x30, 0x34] {
                c.bus.write32(e + off, 0);
            }
        }
        base
    }

    /// The keyboard server answers (`rpc_answered` on the keyboard client,
    /// on a running guest thread), with the words the success arms store
    /// pre-filled: the pending-request words at `+0x90..+0x9c`, the
    /// buffer-state words of the first four entries and 0x40 bytes at
    /// `out`. Call before setting `pc` and the arguments.
    fn kb_answered(c: &mut Cpu, seed: u32, entry: u32, out: u32) {
        let mut r = Rng(entry as u64 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
        crate::game::lib::cdvd::tests::rpc_answered(c, USBKB_CLIENT, 0);
        let ra = c.g[RA];
        ssx3_ps2::kernel::start_main_thread(c, entry, 0, 0);
        c.g[RA] = ra;
        crate::difftest::fill(c, &mut r, USBKB_COUNT + 0x90, 4);
        crate::difftest::fill(c, &mut r, out, 0x10);
        for i in 0..4 {
            let e = DATA + 0x1000 + i * USBKB_STRIDE;
            for off in [0x10, 0x14, 0x20, 0x2c, 0x30, 0x34] {
                c.bus.write32(e + off, r.next_u32() | 1);
            }
        }
    }

    /// Cooked output struct and per-device state live in test RAM; the HID
    /// report and the libusbkb globals stay at their game addresses.
    pub(crate) const LRB_OUT: u32 = DATA + 0x2000;

    pub(crate) const LRB_DEV: u32 = DATA + 0x1000;

    /// libusbkb state (0x529000) and the current HID report (0x529140).
    pub(crate) const LRB_ST: u32 = 0x0052_9000;

    pub(crate) const LRB_REP: u32 = 0x0052_9140;

    /// Report key byte `k`: zeroed on some seeds (filter skip), a small raw
    /// code otherwise, `1` on some seeds (the "already buffered" mark the
    /// first-key check looks for).
    pub(crate) fn lrb_key(seed: u32, k: u32) -> u8 {
        if seed / 3 % 5 == 0 {
            0
        } else if k == 0 && seed / 5 % 4 == 0 {
            1
        } else if k as usize == seed as usize % 6 {
            0
        } else {
            (0x10 + (seed + k) % 0x40) as u8
        }
    }

    driven!(
        libusbkb_cnv_keypad_code_gen_driven,
        ee::s15::f_0040d3c0__generated
    );
    driven!(
        libusbkb_cnv_keypad_code_hand_driven,
        libusbkb_cnv_keypad_code
    );

    /// Keypad lookups with live events: codes found late in the table or
    /// not in it at all keep the row scan going long enough for the event
    /// to land in it.
    #[test]
    fn libusbkb_cnv_keypad_code_events() {
        // Raw codes in row 5 or later (0x58, 0x5c, 0x63, 0x8c) or in no row
        // (0x2a, 0x00, 0xff, 0x04): at least four passes of the scan.
        const SCANNED: [u8; 8] = [0x58, 0x5c, 0x63, 0x8c, 0x2a, 0x00, 0xff, 0x04];
        check_events(
            libusbkb_cnv_keypad_code_gen_driven,
            libusbkb_cnv_keypad_code_hand_driven,
            96,
            &|c, seed| {
                let mut r = kb_setup(c, seed, 0x0040_d3c0, 4);
                // Mode (the low bit picks the row) and the code, with junk
                // above the byte the function masks off.
                c.g[A0][0] = (seed / 3 % 4) as u64;
                c.g[A1][0] = 0x3100 | SCANNED[(seed / 12 % 8) as usize] as u64;
                schedule(c, &mut r, seed, 0x0040_d3c0)
            },
        );
    }

    driven!(
        libusbkb_convert_key_codes_gen_driven,
        ee::s15::f_0040d118__generated
    );
    driven!(
        libusbkb_convert_key_codes_hand_driven,
        libusbkb_convert_key_codes
    );

    /// Key-code conversion with live events: 3..8 codes for keyboard 2,
    /// copied raw or converted through the keymap (every arrangement,
    /// shifted or not, caps on or off), all nonzero or ending at a zero
    /// after three.
    #[test]
    fn libusbkb_convert_key_codes_events() {
        // Raw copies: any nonzero code.
        const RAW: [u16; 8] = [
            0x1234, 0x0041, 0x8001, 0x00ff, 0x7e7e, 0x0102, 0xfffe, 0x0303,
        ];
        // Conversions: a letter first (no keypad scan before the first
        // back edge), then digits, a keypad code (0x59, 0x63) and a code
        // past the table (0x150).
        const KEYMAP: [u16; 8] = [0x04, 0x1e, 0x2c, 0x59, 0x0b, 0x150, 0x27, 0x63];
        check_events(
            libusbkb_convert_key_codes_gen_driven,
            libusbkb_convert_key_codes_hand_driven,
            216,
            &|c, seed| {
                let mut r = kb_setup(c, seed, 0x0040_d118, 4);
                let convert = seed / 3 % 2 == 1;
                let count = 3 + seed / 6 % 6;
                let zero_after_three = seed / 36 % 2 == 1;
                let arrangement = seed / 72 % 3;
                let idx = 2;
                let state = c.bus.read32(USBKB_BASE) + idx * USBKB_STRIDE;
                c.bus.write32(state + USBKB_CODE_TYPE, convert as u32);
                c.bus.write32(state + USBKB_ARRANGEMENT, arrangement);
                c.g[A0][0] = idx as u64;
                // Modifiers (shift bits 0x22) and flags (caps bit 2).
                c.g[A1][0] = [0u64, 0x22, 0x02][arrangement as usize];
                c.g[A2][0] = [0u64, 2, 2][arrangement as usize];
                c.g[A3][0] = count as u64;
                let (src, dst) = (DATA + 0x2046, DATA + 0x2132);
                c.g[T0][0] = src as u64;
                c.g[T1][0] = dst as u64;
                let codes = if convert { KEYMAP } else { RAW };
                for i in 0..8u32 {
                    let code = if i == 3 && zero_after_three {
                        0
                    } else {
                        codes[i as usize]
                    };
                    c.bus.write16(src + 2 * i, code);
                    c.bus.write16(dst + 2 * i, 0x5555);
                }
                schedule(c, &mut r, seed, 0x0040_d118)
            },
        );
    }

    /// Keys of the reports in the read-callback event fixtures: none is 0,
    /// the "already buffered" 1, or a key of the last report's table.
    const REPORT_KEYS: [u8; 8] = [0x04, 0x1e, 0x2c, 0x10, 0x27, 0x0b, 0x3a, 0x15];

    /// Reserved byte of those reports (the last report's matches it unless
    /// a fixture changes it).
    const REPORT_RESERVED: u32 = 0x06;

    /// Keyboard state array of the read-callback event fixtures, the state
    /// of keyboard 1 (the one read) and the cooked output.
    const REPORT_STATES: u32 = DATA + 0x1004;
    const REPORT_KB: u32 = REPORT_STATES + USBKB_STRIDE;
    const REPORT_OUT: u32 = DATA + 0x2014;

    /// The read callback for keyboard 1 with a report holding `keys`: a
    /// stale report (not the fresh-copy mark), buffer submode 0 with counts
    /// of 1, an empty key buffer, `code_type` for the conversion (and as the
    /// arrangement), and a last report with the same reserved byte and one
    /// key (0x77) that is not in this report. The state array, the output,
    /// the last report and the callback's frame are pre-filled.
    fn read_callback_report(c: &mut Cpu, r: &mut Rng, keys: &[u8], code_type: u32) {
        scramble(c, r);
        crate::difftest::fill(c, r, REPORT_STATES, 2 * USBKB_STRIDE / 4);
        crate::difftest::fill(c, r, REPORT_OUT, 0x40 / 4);
        crate::difftest::fill(c, r, crate::difftest::SP - 0x1a0, 0x1a0 / 4);
        crate::difftest::fill(c, r, LRB_ST + 0x8, 0x88 / 4);
        c.bus.write32(LRB_ST, 2);
        c.bus.write32(LRB_ST + 0x4, REPORT_STATES);
        c.bus.write32(LRB_ST + 0x94, 1);
        c.bus.write32(LRB_ST + 0x98, REPORT_OUT);
        for (off, v) in [
            (USBKB_CODE_TYPE, code_type),
            (USBKB_ARRANGEMENT, code_type),
            (0xc, 2),
            (0x10, 0),
            (0x14, 1),
            (0x18, 1),
            (0x1c, 1),
            (USBKB_WIDX, 0),
            (USBKB_RIDX, 0),
            (USBKB_NKEYS, 0),
        ] {
            c.bus.write32(REPORT_KB + off, v);
        }
        c.bus.write16(REPORT_KB + 0x20, 0x5a3c);
        // The report: modifiers, key count, reserved byte, keys.
        crate::difftest::fill(c, r, LRB_REP, 4);
        c.bus.write8(LRB_REP, 0x02);
        c.bus.write8(LRB_REP + 1, keys.len() as u8);
        c.bus.write8(LRB_REP + 2, REPORT_RESERVED as u8);
        for (k, &key) in keys.iter().enumerate() {
            c.bus.write8(LRB_REP + 4 + k as u32, key);
        }
        // The last report: reserved byte, key count, keys (three more
        // entries for the fixtures that raise the count).
        c.bus.write32(LRB_ST + 0xc, REPORT_RESERVED);
        c.bus.write32(LRB_ST + 0x10, 1);
        for (i, &key) in [0x77u16, 0x66, 0x55, 0x44].iter().enumerate() {
            c.bus.write16(LRB_ST + 0x14 + 2 * i as u32, key);
        }
    }

    driven!(
        libusbkb_read_callback_gen_driven,
        ee::s15::f_0040c980__generated
    );
    driven!(libusbkb_read_callback_hand_driven, libusbkb_read_callback);

    /// The keyboard read callback with live events, the first event landing
    /// on each of its loops in turn: the fresh-report copy, the report's key
    /// filter, the diff against the last report (per key, and its scan of
    /// the last report's keys), the plain copy when the last report had no
    /// keys, the search for the held key among the converted keys, and the
    /// save of this report as the last one. The bands whose loop follows an
    /// earlier loop start the event late enough to pass it.
    #[test]
    fn libusbkb_read_callback_events() {
        check_events(
            libusbkb_read_callback_gen_driven,
            libusbkb_read_callback_hand_driven,
            252,
            &|c, seed| {
                let mut r = Rng(0x0040_c980 ^ (seed as u64) << 12);
                let band = seed / 3 % 7;
                let a = seed / 21 % 4;
                let b = seed / 84 % 3;
                let keys = |n: u32| REPORT_KEYS[..n as usize].to_vec();
                // Cycles after the entry at which the event falls due (when
                // the band's loop runs after another loop).
                let mut due: Option<u32> = None;
                match band {
                    // Fresh report (0x40ca28): 2..5 keys, one of them zero
                    // on some seeds.
                    0 => {
                        let mut k = keys(2 + a);
                        if b == 0 {
                            k[1] = 0;
                        }
                        read_callback_report(c, &mut r, &k, b);
                        c.bus.write32(REPORT_KB + 0xc, 1);
                    }
                    // Key filter (0x40cbf8): 2..5 keys after buffer submode
                    // 0, 1 or 2.
                    1 => {
                        read_callback_report(c, &mut r, &keys(2 + a), b);
                        c.bus.write32(REPORT_KB + 0x10, b);
                    }
                    // Diff per key (0x40cca8): 3..6 new keys; due past the
                    // filter (its last pass ends ~36 + 12n cycles in, the
                    // first diff pass ~113 + 12n).
                    2 => {
                        let n = 3 + a;
                        read_callback_report(c, &mut r, &keys(n), b);
                        due = Some(12 * n + 60 + 20 * b);
                    }
                    // Scan of the last report's keys (0x40ccd8): one new key
                    // against 2..4 last keys.
                    3 => {
                        read_callback_report(c, &mut r, &[REPORT_KEYS[a as usize]], b);
                        c.bus.write32(LRB_ST + 0x10, 2 + b);
                    }
                    // Plain copy, last report empty (0x40cd78): 4..7 keys;
                    // due past the filter (first copy pass ~86 + 12n).
                    4 => {
                        let n = 4 + a;
                        read_callback_report(c, &mut r, &keys(n), b);
                        c.bus.write16(LRB_ST + 0x14, 0);
                        due = Some(12 * n + 50 + 15 * b);
                    }
                    // Held-key search (0x40ce58): 2..5 copies of the last
                    // report's key, so nothing is new; the conversion is the
                    // bad mode (no conversion loop), leaving the pre-filled
                    // frame as the converted keys. Due past the filter and
                    // the diff (~61 + 31n), before the first search pass
                    // (~197 + 31n).
                    5 => {
                        let n = 2 + a;
                        read_callback_report(c, &mut r, &vec![0x2c; n as usize], 2);
                        c.bus.write16(LRB_ST + 0x14, 0x2c);
                        due = Some(31 * n + 100 + 30 * b);
                    }
                    // Save as the last report (0x40cea8): one key, a
                    // different reserved byte (no diff).
                    _ => {
                        read_callback_report(c, &mut r, &[REPORT_KEYS[a as usize]], b);
                        c.bus.write32(LRB_ST + 0xc, REPORT_RESERVED ^ 1);
                    }
                }
                let mode = schedule(c, &mut r, seed, 0x0040_c980);
                if let Some(cycles) = due {
                    if mode != crate::difftest::events::Events::Off {
                        c.next_event = c.cycles + cycles as u64;
                    }
                }
                mode
            },
        );
    }

    use crate::difftest::fakes::machine;

    /// `libusbkb_SetLEDStatusCallback` (0x0040_cf48) against the generated body.
    #[test]
    fn libusbkb_set_led_status_callback_0040cf48_matches() {
        check(
            ee::s15::f_0040cf48__generated,
            libusbkb_set_led_status_callback_0040cf48,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x40cf48, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `libusbkb_SetLEDModeCallback` (0x0040_cf68) against the generated body.
    #[test]
    fn libusbkb_set_led_mode_callback_0040cf68_matches() {
        check(
            ee::s15::f_0040cf68__generated,
            libusbkb_set_led_mode_callback_0040cf68,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x40cf68, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
