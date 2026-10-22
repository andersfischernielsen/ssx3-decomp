//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! - **`_mbtowc_r`** (0x00418958): newlib's multibyte-to-wide-character
//!   decoder. It reads the reent's locale name (`*(r + 0x34)`) and compares it
//!   with the three Japanese locales the game's newlib carries
//!   (`"C-SJIS"`, `"C-EUCJP"`, `"C-JIS"`, at 0x49a200/0x49a208/0x49a210):
//!
//!   - short/unknown locale or a null locale name: one byte is returned as
//!     the wide character (`(0 < *s)`, 0 written for a null string);
//!   - `C-SJIS`: a lead byte 0x81..0x9f or 0xe0..0xef takes a trail byte
//!     0x40..0x7e or 0x80..0xfc and yields `lead << 8 | trail`, 2 bytes;
//!   - `C-EUCJP`: a lead byte 0xa1..0xfe takes a trail 0xa1..0xfe, same
//!     packing, 2 bytes;
//!   - `C-JIS`: ISO-2022-JP. The first byte of a sequence is classified
//!     (ESC, `$`, `(`, `@`, `B`, `J`, `)`, other) and a two-dimensional table
//!     at 0x454af8 indexed by the mbstate row and the class selects the
//!     action: reset the state and return 0, return one byte, store a
//!     two-byte character, or advance to the next input byte and keep
//!     scanning.
//!
//!   Like the original, the replacement is a `pc` dispatch: the function has a
//!   runtime jump table (0x49a220, read from the loaded image) into its
//!   ISO-2022-JP handlers, and four event checks whose resume points are
//!   block labels inside the function (the trampoline re-enters the function
//!   at the saved `pc`), so the label set *is* the interface. Everything else
//!   is the original's register and cycle sequence, block by block.
//!
//! Calls: `strlen`/`strcmp` through the generated aliases of this shard (the
//! game's newlib, not the host library), with the original's return addresses
//! and the unwind check after each.

use crate::abi;
use crate::call::{guest, Callee};
use crate::events::back_edge_unwinds;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0041_8958, mbtowc_r, "_mbtowc_r")];

mod callee {
    use super::{ee, Callee};

    /// `size_t strlen(const char*)` (`a0` in, `v0` out).
    pub const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::strlen,
    };
    /// `int strcmp(const char*, const char*)` (`a0`, `a1` in, `v0` out).
    pub const STRCMP: Callee = Callee {
        addr: 0x0041_65a8,
        f: ee::s15::strcmp,
    };
}

// ---------------------------------------------------------------------------
// The game image's locale names and C-JIS tables.

/// `"C-SJIS"`.
pub const LOCALE_SJIS: u32 = 0x0049_a200;
/// `"C-EUCJP"`.
pub const LOCALE_EUCJP: u32 = 0x0049_a208;
/// `"C-JIS"`.
pub const LOCALE_JIS: u32 = 0x0049_a210;
/// The `strcmp` second argument the original materialises for `C-SJIS`
/// (`0x4a0000 - 0x5e00`); the other two are derived from it.
const STR_BASE: u32 = 0x004a_0000;

/// `[state][class]` action index table (`0x450000 + 0x4af8`, indexed at
/// `+0x24` by the class and by `state * 9 * 4`).
pub const JIS_ACTIONS: u32 = 0x0045_4af8;
/// `[state][class]` next-state table (`0x450000 + 0x4948`, same indexing).
pub const JIS_STATES: u32 = 0x0045_4948;

// ---------------------------------------------------------------------------
// Block labels. The generated body dispatches on `pc`, so these are the
// names the arms answer to (the original jumps between them with a computed
// jump, and the event checks resume at `MINUS1` and `JIS_NEXT_BYTE`).

mod l {
    pub const ENTRY: u32 = 0x0041_8958;
    /// `n == 0`: -1.
    pub const N_ZERO: u32 = 0x0041_8990;
    /// Fetch the locale name; a null name is a single-byte locale.
    pub const LOCALE: u32 = 0x0041_8998;
    /// `strlen(name) < 2` is a single-byte locale.
    pub const LOCALE_LEN: u32 = 0x0041_89a4;
    /// `strcmp(name, "C-SJIS")`.
    pub const CMP_SJIS: u32 = 0x0041_89b8;
    /// `C-SJIS`: *s is not NUL; return 0 for a null string.
    pub const SJIS_HEAD: u32 = 0x0041_89cc;
    /// Lead byte 0x81..0x9f?
    pub const SJIS_LEAD_HI: u32 = 0x0041_89d8;
    /// Lead byte 0xe0..0xef?
    pub const SJIS_LEAD_LO: u32 = 0x0041_89e8;
    /// Load the trail byte, reject an incomplete sequence.
    pub const SJIS_TRAIL: u32 = 0x0041_89f8;
    /// Return -1 (invalid or incomplete).
    pub const MINUS1: u32 = 0x0041_8a00;
    /// Trail byte 0x40..0x7e?
    pub const SJIS_TRAIL1: u32 = 0x0041_8a08;
    /// Trail byte 0x80..0xfc?
    pub const SJIS_TRAIL2: u32 = 0x0041_8a18;
    /// Return -1 (invalid trail).
    pub const MINUS1_TRAIL: u32 = 0x0041_8a28;
    /// `strcmp(name, "C-EUCJP")`.
    pub const CMP_EUCJP: u32 = 0x0041_8a30;
    /// `C-EUCJP`: *s is not NUL; return 0 for a null string.
    pub const EUC_HEAD: u32 = 0x0041_8a44;
    /// Lead byte 0xa1..0xfe? (else single byte.)
    pub const EUC_LEAD: u32 = 0x0041_8a4c;
    /// Reject an incomplete sequence, else classify the trail byte.
    pub const EUC_TRAIL: u32 = 0x0041_8a5c;
    /// Trail byte 0xa1..0xfe?
    pub const EUC_TRAIL_CLASS: u32 = 0x0041_8a64;
    /// Load the lead byte for the two-byte pack.
    pub const EUC_LEAD_BYTE: u32 = 0x0041_8a74;
    /// `*pwc = lead << 8 | trail`, return 2.
    pub const TWO_BYTES: u32 = 0x0041_8a78;
    /// `strcmp(name, "C-JIS")`.
    pub const CMP_JIS: u32 = 0x0041_8a90;
    /// `C-JIS`: a null string resets the state and returns 1.
    pub const JIS_STATE: u32 = 0x0041_8aa4;
    /// Reset the mbstate, return 1.
    pub const JIS_RESET: u32 = 0x0041_8aac;
    /// Start scanning at offset 0; `t1` selects the state row.
    pub const JIS_SETUP: u32 = 0x0041_8ab8;
    /// `a3 = s + offset`.
    pub const JIS_NEXT_BYTE: u32 = 0x0041_8acc;
    /// Classify the current input byte.
    pub const JIS_CLASSIFY: u32 = 0x0041_8ad0;
    /// `c < ')'`?
    pub const JIS_LT_41: u32 = 0x0041_8ae0;
    /// ESC?
    pub const JIS_ESC: u32 = 0x0041_8aec;
    /// `c < 0x1c`?
    pub const JIS_LT_28: u32 = 0x0041_8af4;
    /// NUL?
    pub const JIS_NUL: u32 = 0x0041_8afc;
    /// Class 7 or 8 depending on `0x5d < c - 0x21`.
    pub const JIS_CLASS_5D_A: u32 = 0x0041_8b04;
    /// `$`?
    pub const JIS_DOLLAR: u32 = 0x0041_8b0c;
    /// Class 7 or 8 (second copy).
    pub const JIS_CLASS_5D_B: u32 = 0x0041_8b14;
    /// `c < 'C'`, `B`?
    pub const JIS_GE_41: u32 = 0x0041_8b1c;
    /// `@`?
    pub const JIS_GE_43: u32 = 0x0041_8b28;
    /// `@` class 3 or 7/8.
    pub const JIS_AT: u32 = 0x0041_8b30;
    /// Class 7 or 8 (third copy).
    pub const JIS_CLASS_5D_C: u32 = 0x0041_8b38;
    /// `J`?
    pub const JIS_GE_67: u32 = 0x0041_8b40;
    /// Class 7 or 8 (fourth copy).
    pub const JIS_CLASS_5D_D: u32 = 0x0041_8b4c;
    /// Class 0 (ESC).
    pub const CLASS_ESC: u32 = 0x0041_8b54;
    /// Class 1 (`$`).
    pub const CLASS_DOLLAR: u32 = 0x0041_8b5c;
    /// Class 3 (`@`).
    pub const CLASS_AT: u32 = 0x0041_8b64;
    /// Class 2 (`(`).
    pub const CLASS_LPAREN: u32 = 0x0041_8b6c;
    /// Class 4 (`B`).
    pub const CLASS_B: u32 = 0x0041_8b74;
    /// Class 5 (`J`).
    pub const CLASS_J: u32 = 0x0041_8b7c;
    /// Class 6 (anything else).
    pub const CLASS_OTHER: u32 = 0x0041_8b84;
    /// Class 7 or 8 (shared tail).
    pub const CLASS_5D: u32 = 0x0041_8b8c;
    /// Look the class up in the two `C-JIS` tables.
    pub const JIS_TABLE: u32 = 0x0041_8b9c;
    /// Jump through the handler table.
    pub const JIS_DISPATCH: u32 = 0x0041_8bd4;
    /// Reset the state, `*pwc = 0`, return the scan offset.
    pub const JIS_RESET_ZERO: u32 = 0x0041_8bf0;
    /// Reset the state, return one byte.
    pub const JIS_ONE_BYTE: u32 = 0x0041_8c00;
    /// Reset the state, store two bytes, return `offset + 1`.
    pub const JIS_TWO_BYTES: u32 = 0x0041_8c14;
    /// Reset the state to 1, store two bytes, return 2.
    pub const JIS_TWO_BYTES_STATE1: u32 = 0x0041_8c20;
    /// `*pwc = b0 << 8 | b1`, return `v0`.
    pub const STORE_TWO: u32 = 0x0041_8c30;
    /// Advance to the next input byte.
    pub const JIS_ADVANCE: u32 = 0x0041_8c48;
    /// Loop tail: more input bytes?
    pub const JIS_LOOP: u32 = 0x0041_8c4c;
    /// Return -1 (no handler accepted the sequence).
    pub const MINUS1_SCAN: u32 = 0x0041_8c5c;
    /// Single-byte locale: a null string returns 0.
    pub const SINGLE_BYTE: u32 = 0x0041_8c64;
    /// `*pwc = *s; return (0 < *s)`.
    pub const SINGLE_BYTE_COPY: u32 = 0x0041_8c6c;
    /// Return 0.
    pub const RETURN_ZERO: u32 = 0x0041_8c80;
    /// Restore the frame and return.
    pub const EPILOGUE: u32 = 0x0041_8c84;
}

/// The original's entry and end (for the out-of-range `pc` hook).
const START: u32 = l::ENTRY;
const END: u32 = 0x0041_8ca4;

#[inline(always)]
fn a(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `int _mbtowc_r(_reent* r, wchar_t* pwc, const char* s, size_t n,
///                mbstate_t* state)` (0x418958): decode the next character of
/// `s` under the reent's Japanese locale, write it to `pwc` and return its
/// number of bytes (-1 invalid, 0 for a null string or a reset).
pub fn mbtowc_r(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                l::ENTRY => {
                    c.cycles += 14;
                    c.g[SP][0] = sx(a(c, SP).wrapping_sub(0x70));
                    c.sq(19, a(c, SP).wrapping_add(0x30)); // save s3
                    c.sq(16, a(c, SP).wrapping_add(0x60)); // save s0
                    c.g[S3][0] = c.g[SP][0]; // the wide-character sink (pwc)
                    c.sq(17, a(c, SP).wrapping_add(0x50)); // save s1
                    c.g[S0][0] = c.g[A2][0]; // s0 = s
                    c.sq(18, a(c, SP).wrapping_add(0x40)); // save s2
                    c.g[S1][0] = c.g[A0][0]; // s1 = reent
                    c.sq(20, a(c, SP).wrapping_add(0x20)); // save s4
                    c.g[S2][0] = c.g[T0][0]; // s2 = state
                    c.sd(a(c, SP).wrapping_add(0x10), c.g[RA][0]);
                    c.g[S4][0] = c.g[A3][0]; // s4 = n
                    let null_string = c.g[S0][0] == 0;
                    if c.g[A1][0] != 0 {
                        c.g[S3][0] = c.g[A1][0];
                    }
                    pc = if null_string { l::LOCALE } else { l::N_ZERO };
                }

                l::N_ZERO => {
                    c.cycles += 2;
                    let none = c.g[S4][0] == 0;
                    c.g[V0][0] = sx(0xffff_ffff);
                    pc = if none { l::EPILOGUE } else { l::LOCALE };
                }

                l::LOCALE => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(a(c, S1).wrapping_add(0x34));
                    let null_name = c.g[A0][0] == 0;
                    pc = if null_name {
                        l::SINGLE_BYTE
                    } else {
                        l::LOCALE_LEN
                    };
                }

                l::LOCALE_LEN => {
                    c.cycles += 5;
                    callee::STRLEN.call(c, 0x0041_89ac)?;
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let short = c.g[V0][0] != 0;
                    c.g[A1][0] = sx(STR_BASE);
                    pc = if short { l::SINGLE_BYTE } else { l::CMP_SJIS };
                }

                l::CMP_SJIS => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(a(c, S1).wrapping_add(0x34));
                    c.g[A1][0] = sx(a(c, A1).wrapping_sub(0x5e00));
                    callee::STRCMP.call(c, 0x0041_89c4)?;
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = sx(STR_BASE);
                        pc = l::CMP_EUCJP;
                    } else {
                        pc = l::SJIS_HEAD;
                    }
                }

                l::SJIS_HEAD => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lbu(a(c, S0));
                    let null_string = c.g[S0][0] == 0;
                    c.g[V0][0] = 0;
                    pc = if null_string {
                        l::EPILOGUE
                    } else {
                        l::SJIS_LEAD_HI
                    };
                }

                l::SJIS_LEAD_HI => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(a(c, V1).wrapping_sub(0x81));
                    c.g[V0][0] = (c.g[V0][0] < 0x1fu64) as u64;
                    let lead = c.g[V0][0] != 0;
                    c.g[V0][0] = (c.g[S4][0] < 0x2u64) as u64;
                    pc = if lead { l::SJIS_TRAIL } else { l::SJIS_LEAD_LO };
                }

                l::SJIS_LEAD_LO => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(a(c, V1).wrapping_sub(0xe0));
                    c.g[V0][0] = (c.g[V0][0] < 0x10u64) as u64;
                    let single = c.g[V0][0] == 0;
                    c.g[V0][0] = (c.g[S4][0] < 0x2u64) as u64;
                    pc = if single {
                        l::SINGLE_BYTE
                    } else {
                        l::SJIS_TRAIL
                    };
                }

                l::SJIS_TRAIL => {
                    c.cycles += 2;
                    // `v0` is `n < 2`: with a trail byte available the second
                    // byte is classified, otherwise the sequence is incomplete.
                    let have_trail = c.g[V0][0] == 0;
                    c.g[V1][0] = c.lbu(a(c, S0).wrapping_add(0x1));
                    pc = if have_trail {
                        l::SJIS_TRAIL1
                    } else {
                        l::MINUS1
                    };
                }

                l::MINUS1 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx(0xffff_ffff);
                    pc = l::EPILOGUE;
                }

                l::SJIS_TRAIL1 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(a(c, V1).wrapping_sub(0x40));
                    c.g[V0][0] = (c.g[V0][0] < 0x3fu64) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.lbu(a(c, S0));
                        pc = l::TWO_BYTES;
                    } else {
                        pc = l::SJIS_TRAIL2;
                    }
                }

                l::SJIS_TRAIL2 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(a(c, V1).wrapping_sub(0x80));
                    c.g[V0][0] = (c.g[V0][0] < 0x7du64) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.lbu(a(c, S0));
                        pc = l::TWO_BYTES;
                    } else {
                        pc = l::MINUS1_TRAIL;
                    }
                }

                l::MINUS1_TRAIL => {
                    c.cycles += 2;
                    c.g[V0][0] = sx(0xffff_ffff);
                    pc = l::EPILOGUE;
                }

                l::CMP_EUCJP => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(a(c, S1).wrapping_add(0x34));
                    c.g[A1][0] = sx(a(c, A1).wrapping_sub(0x5df8));
                    callee::STRCMP.call(c, 0x0041_8a3c)?;
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = sx(STR_BASE);
                        pc = l::CMP_JIS;
                    } else {
                        pc = l::EUC_HEAD;
                    }
                }

                l::EUC_HEAD => {
                    c.cycles += 2;
                    let null_string = c.g[S0][0] == 0;
                    c.g[V0][0] = c.lbu(a(c, S0));
                    pc = if null_string {
                        l::RETURN_ZERO
                    } else {
                        l::EUC_LEAD
                    };
                }

                l::EUC_LEAD => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(a(c, V0).wrapping_sub(0xa1));
                    c.g[V0][0] = (c.g[V0][0] < 0x5eu64) as u64;
                    let single = c.g[V0][0] == 0;
                    c.g[V0][0] = (c.g[S4][0] < 0x2u64) as u64;
                    pc = if single { l::SINGLE_BYTE } else { l::EUC_TRAIL };
                }

                l::EUC_TRAIL => {
                    c.cycles += 2;
                    let incomplete = c.g[V0][0] != 0;
                    c.g[V1][0] = c.lbu(a(c, S0).wrapping_add(0x1));
                    if incomplete {
                        // A single check before the forward jump to `MINUS1`.
                        if c.cycles >= c.next_event {
                            c.pc = l::MINUS1;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = l::MINUS1;
                    } else {
                        pc = l::EUC_TRAIL_CLASS;
                    }
                }

                l::EUC_TRAIL_CLASS => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(a(c, V1).wrapping_sub(0xa1));
                    c.g[V0][0] = (c.g[V0][0] < 0x5eu64) as u64;
                    let invalid = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(0xffff_ffff);
                    pc = if invalid {
                        l::EPILOGUE
                    } else {
                        l::EUC_LEAD_BYTE
                    };
                }

                l::EUC_LEAD_BYTE => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lbu(a(c, S0));
                    pc = l::TWO_BYTES;
                }

                l::TWO_BYTES => {
                    c.cycles += 6;
                    c.g[V0][0] = 2;
                    c.g[V1][0] = c.lbu(a(c, S0).wrapping_add(0x1));
                    c.g[A0][0] = sx(a(c, A0) << 8);
                    c.g[V1][0] = sx(a(c, V1).wrapping_add(a(c, A0)));
                    c.sh(a(c, S3), c.g[V1][0]);
                    pc = l::EPILOGUE;
                }

                l::CMP_JIS => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(a(c, S1).wrapping_add(0x34));
                    c.g[A1][0] = sx(a(c, A1).wrapping_sub(0x5df0));
                    callee::STRCMP.call(c, 0x0041_8a9c)?;
                    let other = c.g[V0][0] != 0;
                    pc = if other { l::SINGLE_BYTE } else { l::JIS_STATE };
                }

                l::JIS_STATE => {
                    c.cycles += 2;
                    if c.g[S0][0] != 0 {
                        c.g[V0][0] = c.lw(a(c, S2));
                        pc = l::JIS_SETUP;
                    } else {
                        pc = l::JIS_RESET;
                    }
                }

                l::JIS_RESET => {
                    c.cycles += 3;
                    c.sw(a(c, S2), 0);
                    c.g[V0][0] = 1;
                    pc = l::EPILOGUE;
                }

                l::JIS_SETUP => {
                    c.cycles += 5;
                    c.g[T1][0] = 3; // table row: 3 for a state in progress
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A2][0] = 0;
                    let none = c.g[S4][0] == 0;
                    if c.g[V0][0] == 0 {
                        c.g[T1][0] = 0; // row 0 for the initial state
                    }
                    if none {
                        // A single check before the forward jump to `MINUS1`.
                        if c.cycles >= c.next_event {
                            c.pc = l::MINUS1;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = l::MINUS1;
                    } else {
                        pc = l::JIS_NEXT_BYTE;
                    }
                }

                l::JIS_NEXT_BYTE => {
                    c.cycles += 1;
                    c.g[V0][0] = sx(a(c, S0).wrapping_add(a(c, A2)));
                    pc = l::JIS_CLASSIFY;
                }

                l::JIS_CLASSIFY => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x28; // '('
                    c.g[A0][0] = c.lbu(a(c, V0));
                    let lparen = c.g[A0][0] == c.g[V1][0];
                    c.g[T0][0] = c.g[V0][0];
                    pc = if lparen {
                        l::CLASS_LPAREN
                    } else {
                        l::JIS_LT_41
                    };
                }

                l::JIS_LT_41 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 41i64) as u64;
                    let ge = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x1b; // ESC
                    pc = if ge { l::JIS_GE_41 } else { l::JIS_ESC };
                }

                l::JIS_ESC => {
                    c.cycles += 2;
                    let esc = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 28i64) as u64;
                    pc = if esc { l::CLASS_ESC } else { l::JIS_LT_28 };
                }

                l::JIS_LT_28 => {
                    c.cycles += 2;
                    let lt = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x24; // '$'
                    pc = if lt { l::JIS_DOLLAR } else { l::JIS_NUL };
                }

                l::JIS_NUL => {
                    c.cycles += 2;
                    let nul = c.g[A0][0] == 0;
                    c.g[V0][0] = sx(a(c, A0).wrapping_sub(0x21));
                    pc = if nul {
                        l::CLASS_OTHER
                    } else {
                        l::JIS_CLASS_5D_A
                    };
                }

                l::JIS_CLASS_5D_A => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x5d; // ']'
                    pc = l::CLASS_5D;
                }

                l::JIS_DOLLAR => {
                    c.cycles += 2;
                    let dollar = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = sx(a(c, A0).wrapping_sub(0x21));
                    pc = if dollar {
                        l::CLASS_DOLLAR
                    } else {
                        l::JIS_CLASS_5D_B
                    };
                }

                l::JIS_CLASS_5D_B => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x5d; // ']'
                    pc = l::CLASS_5D;
                }

                l::JIS_GE_41 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x42; // 'B'
                    let is_b = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 67i64) as u64;
                    pc = if is_b { l::CLASS_B } else { l::JIS_GE_43 };
                }

                l::JIS_GE_43 => {
                    c.cycles += 2;
                    let lt = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x40; // '@'
                    pc = if lt { l::JIS_GE_67 } else { l::JIS_AT };
                }

                l::JIS_AT => {
                    c.cycles += 2;
                    let at = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = sx(a(c, A0).wrapping_sub(0x21));
                    pc = if at { l::CLASS_AT } else { l::JIS_CLASS_5D_C };
                }

                l::JIS_CLASS_5D_C => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x5d; // ']'
                    pc = l::CLASS_5D;
                }

                l::JIS_GE_67 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4a; // 'J'
                    let is_j = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = sx(a(c, A0).wrapping_sub(0x21));
                    pc = if is_j { l::CLASS_J } else { l::JIS_CLASS_5D_D };
                }

                l::JIS_CLASS_5D_D => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x5d; // ']'
                    pc = l::CLASS_5D;
                }

                l::CLASS_ESC => {
                    c.cycles += 2;
                    c.g[A0][0] = 0;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_DOLLAR => {
                    c.cycles += 2;
                    c.g[A0][0] = 1;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_AT => {
                    c.cycles += 2;
                    c.g[A0][0] = 3;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_LPAREN => {
                    c.cycles += 2;
                    c.g[A0][0] = 2;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_B => {
                    c.cycles += 2;
                    c.g[A0][0] = 4;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_J => {
                    c.cycles += 2;
                    c.g[A0][0] = 5;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_OTHER => {
                    c.cycles += 2;
                    c.g[A0][0] = 6;
                    pc = l::JIS_TABLE;
                }

                l::CLASS_5D => {
                    c.cycles += 4;
                    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                    c.g[A0][0] = 8;
                    c.g[V0][0] = 7;
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = c.g[V0][0];
                    }
                    pc = l::JIS_TABLE;
                }

                l::JIS_TABLE => {
                    c.cycles += 14;
                    c.g[A1][0] = 0x24; // one table row: 9 classes * 4 bytes
                    c.g[A0][0] = sx(a(c, A0) << 2);
                    // `mult a1, t1, a1`: the row offset (0 or 3 rows).
                    c.mult(0, A1, T1, A1);
                    c.g[V1][0] = sx(JIS_ACTIONS);
                    c.g[V0][0] = sx(JIS_STATES);
                    c.g[A0][0] = sx(a(c, A0).wrapping_add(a(c, A1)));
                    c.g[V1][0] = sx(a(c, A0).wrapping_add(a(c, V1)));
                    c.g[V1][0] = c.lw(a(c, V1)); // the action index
                    c.g[A0][0] = sx(a(c, A0).wrapping_add(a(c, V0)));
                    c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                    let none = c.g[V0][0] == 0;
                    c.g[T1][0] = c.lw(a(c, A0)); // the next state
                    if none {
                        // A single check before the forward jump to `MINUS1`.
                        if c.cycles >= c.next_event {
                            c.pc = l::MINUS1;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = l::MINUS1;
                    } else {
                        pc = l::JIS_DISPATCH;
                    }
                }

                l::JIS_DISPATCH => {
                    c.cycles += 7;
                    c.g[V0][0] = sx(STR_BASE);
                    c.g[V1][0] = sx(a(c, V1) << 2);
                    c.g[V0][0] = sx(a(c, V0).wrapping_sub(0x5de0));
                    c.g[V1][0] = sx(a(c, V1).wrapping_add(a(c, V0)));
                    c.g[A0][0] = c.lw(a(c, V1));
                    pc = a(c, A0);
                }

                l::JIS_RESET_ZERO => {
                    c.cycles += 4;
                    c.sw(a(c, S2), 0);
                    c.g[V0][0] = c.g[A2][0];
                    c.sh(a(c, S3), 0);
                    pc = l::EPILOGUE;
                }

                l::JIS_ONE_BYTE => {
                    c.cycles += 5;
                    c.sw(a(c, S2), 0);
                    c.g[V0][0] = sx(a(c, A2).wrapping_add(0x1));
                    c.g[V1][0] = c.lbu(a(c, A3));
                    c.sh(a(c, S3), c.g[V1][0]);
                    pc = l::EPILOGUE;
                }

                l::JIS_TWO_BYTES => {
                    c.cycles += 3;
                    c.sw(a(c, S2), 0);
                    c.g[V0][0] = sx(a(c, A2).wrapping_add(0x1));
                    pc = l::STORE_TWO;
                }

                l::JIS_TWO_BYTES_STATE1 => {
                    c.cycles += 4;
                    c.g[V1][0] = 1;
                    c.g[V0][0] = sx(a(c, A3).wrapping_sub(a(c, S0)));
                    c.sw(a(c, S2), c.g[V1][0]);
                    c.g[V0][0] = sx(a(c, V0).wrapping_add(0x2));
                    pc = l::STORE_TWO;
                }

                l::STORE_TWO => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lbu(a(c, A3));
                    c.g[A0][0] = c.lbu(a(c, A3).wrapping_add(0x1));
                    c.g[V1][0] = sx(a(c, V1) << 8);
                    c.g[A0][0] = sx(a(c, A0).wrapping_add(a(c, V1)));
                    c.sh(a(c, S3), c.g[A0][0]);
                    pc = l::EPILOGUE;
                }

                l::JIS_ADVANCE => {
                    c.cycles += 1;
                    c.g[A3][0] = sx(a(c, T0).wrapping_add(0x1));
                    pc = l::JIS_LOOP;
                }

                l::JIS_LOOP => {
                    c.cycles += 4;
                    c.g[A2][0] = sx(a(c, A2).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[A2][0] < c.g[S4][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(a(c, S0).wrapping_add(a(c, A2)));
                    if more {
                        // The loop's back edge: an unwind resumes the function
                        // at the classification head.
                        if back_edge_unwinds(c, l::JIS_CLASSIFY) {
                            return Ok(());
                        }
                        pc = l::JIS_CLASSIFY;
                    } else {
                        pc = l::MINUS1_SCAN;
                    }
                }

                l::MINUS1_SCAN => {
                    c.cycles += 2;
                    c.g[V0][0] = sx(0xffff_ffff);
                    pc = l::EPILOGUE;
                }

                l::SINGLE_BYTE => {
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        c.g[V0][0] = 0;
                        pc = l::EPILOGUE;
                    } else {
                        pc = l::SINGLE_BYTE_COPY;
                    }
                }

                l::SINGLE_BYTE_COPY => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lbu(a(c, S0));
                    c.sh(a(c, S3), c.g[V0][0]);
                    c.g[V0][0] = c.lbu(a(c, S0));
                    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                    pc = l::EPILOGUE;
                }

                l::RETURN_ZERO => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    pc = l::EPILOGUE;
                }

                l::EPILOGUE => {
                    c.cycles += 8;
                    let sp = a(c, SP);
                    c.lq(16, sp.wrapping_add(0x60));
                    c.lq(17, sp.wrapping_add(0x50));
                    c.lq(18, sp.wrapping_add(0x40));
                    c.lq(19, sp.wrapping_add(0x30));
                    c.lq(20, sp.wrapping_add(0x20));
                    c.g[RA][0] = c.ld(sp.wrapping_add(0x10));
                    c.g[SP][0] = sx(sp.wrapping_add(0x70));
                    abi::ret(c);
                    return Ok(());
                }

                _ => {
                    c.jump_out(pc, START, END);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{compare, elf_image, seed_count, DATA};

    const SEEDS: u32 = 300;

    const REENT: u32 = DATA + 0x1000;
    const STATE: u32 = DATA + 0x1100;
    const STR: u32 = DATA + 0x1200;
    const PWC: u32 = DATA + 0x1300;
    /// `""` and `"XY"`: the short/unknown locale names.
    const SHORT: u32 = DATA + 0x1400;
    const OTHER: u32 = DATA + 0x1410;

    /// Hard, soft and LO/HI must all match: the original keeps every
    /// intermediate in guest registers (and runs one `mult`), so a caller
    /// could observe them.
    fn check_strict(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seed_count(seeds) {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} (scratch): {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
        }
    }

    /// Bytes that land in every class of the three decoders: NUL, a plain
    /// ASCII letter, ESC, `$`, `(`, `)`, `@`, `B`, `J`, `]`, an SJIS lead, a
    /// high SJIS lead, an EUC lead/trail, and the ends of both trail ranges.
    const BYTES: [u8; 16] = [
        0x00, 0x41, 0x1b, 0x24, 0x28, 0x29, 0x40, 0x42, 0x4a, 0x5d, 0x81, 0x9f, 0xa1, 0xe0, 0xef,
        0xfc,
    ];

    fn setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x4189_58, 0);
        // The locale names and the two C-JIS tables are the game's own data.
        elf_image(c);
        c.bus.write8(SHORT, 0);
        c.bus.write8(OTHER, b'X');
        c.bus.write8(OTHER + 1, b'Y');
        c.bus.write8(OTHER + 2, 0);

        let case = seed % 7;
        let locale = match case {
            0 => 0,     // no locale name: single byte
            1 => SHORT, // strlen < 2: single byte
            2 => LOCALE_SJIS,
            3 => LOCALE_EUCJP,
            4 => LOCALE_JIS,
            5 => OTHER, // none of the three: single byte
            _ => LOCALE_SJIS,
        };
        c.bus.write32(REENT + 0x34, locale);

        // Input bytes: the two selected ones then zeros (so a two-byte pack
        // reads well-defined data). The strides 5 and 7 walk all of `BYTES`
        // for every case within the seed range.
        let b0 = BYTES[(seed as usize * 5 + case as usize) % BYTES.len()];
        let b1 = BYTES[(seed as usize * 7 + case as usize) % BYTES.len()];
        for i in 0..8u32 {
            c.bus.write8(STR + i, 0);
        }
        c.bus.write8(STR, b0);
        c.bus.write8(STR + 1, b1);

        // The mbstate rows: 0 (initial) and a state in progress.
        c.bus
            .write32(STATE, [0u32, 1, 0xff, 0][(seed as usize / 7) % 4]);

        c.g[A0][0] = REENT as u64;
        c.g[A1][0] = if seed % 5 == 0 { 0 } else { PWC as u64 };
        c.g[A2][0] = if seed % 3 == 0 { 0 } else { STR as u64 };
        c.g[A3][0] = (seed % 4) as u64;
        c.g[T0][0] = STATE as u64;
        c.pc = START;
    }

    #[test]
    fn mbtowc_r_matches() {
        check_strict(ssx3_ee::s15::f_00418958__generated, mbtowc_r, SEEDS, &|c, seed| {
            setup(c, seed)
        });
    }
}
