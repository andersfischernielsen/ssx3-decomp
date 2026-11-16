//! Staging module oc387 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc387`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0032_3bd8,
    input_parser_parse_unary,
    "InputParser_ParseUnary",
)];

// ---------------------------------------------------------------------------
// InputParser_ParseUnary (`cInputMapParser_parseTerm`)

/// Parser fields the term parser reads: the current token kind, its text
/// and numeric value, the expression nodes' free list, and the identifier
/// being defined (a reference to it is recursive).
pub(crate) const PARSE_UNARY_TOKEN: u32 = 0x98;
pub(crate) const PARSE_UNARY_TEXT: u32 = 0x9c;
pub(crate) const PARSE_UNARY_VALUE: u32 = 0xdc;
#[cfg(test)]
pub(crate) const PARSE_UNARY_FREE_NODES: u32 = 0xe0;
#[cfg(test)]
pub(crate) const PARSE_UNARY_IDENT_COUNT: u32 = 0x80e4;
#[cfg(test)]
pub(crate) const PARSE_UNARY_IDENTS: u32 = 0x80e8;
/// `self + 0x10000 + 0x8e8`: the identifier whose definition is parsed.
pub(crate) const PARSE_UNARY_CURRENT_IDENT: u32 = 0x8e8;
/// An identifier entry's expression pointer (entries are 0x44 bytes, the
/// name first).
pub(crate) const PARSE_UNARY_IDENT_EXPR: u32 = 0x40;

/// Expression node kinds the term parser builds (`node[0]`; `node[1]` is
/// the operand or value, `node[2]` the modifier).
const PU_NODE_IDENT: u64 = 1;
const PU_NODE_BUTTON: u64 = 2;
const PU_NODE_CONFIG: u64 = 3;
const PU_NODE_CONST: u64 = 4;
const PU_NODE_NEG: u64 = 0x12;
const PU_NODE_NOT: u64 = 0x13;

/// Token kinds: an identifier, `.`, `)`.
const PU_TOK_NAME: u64 = 2;
const PU_TOK_DOT: u64 = 4;
const PU_TOK_RPAREN: u64 = 0xa;
/// "No modifier" and "unknown modifier" from LookupModifier.
const PU_MOD_NONE: u64 = 5;
const PU_MOD_BAD: u64 = 6;

/// The token switch's jump table (`0x490000 - 0x1f00`) and its targets.
const PU_JUMP_TABLE: u32 = 0x0048_e100;
const PU_CASE_NAME: u32 = 0x0032_3c24;
const PU_CASE_NAME_DOT: u32 = 0x0032_3d64;
const PU_CASE_NUMBER: u32 = 0x0032_3e28;
const PU_CASE_PLUS: u32 = 0x0032_3e4c;
const PU_CASE_MINUS: u32 = 0x0032_3e5c;
const PU_CASE_NOT: u32 = 0x0032_3eac;
const PU_CASE_PAREN: u32 = 0x0032_3f20;
/// `max min pow atan2` (ParseFunction2) then the ten one-argument
/// functions (ParseFunction1), 0x1c bytes apart, node kinds 0x14..0x21.
const PU_CASE_FUNC_FIRST: u32 = 0x0032_3f78;
const PU_CASE_FUNC_LAST: u32 = 0x0032_40e4;
const PU_CASE_DEFAULT: u32 = 0x0032_4100;

/// Error messages.
const PU_MSG_EXPECTED_MODIFIER: u64 = 0x0048_e008;
const PU_MSG_BAD_BUTTON_MODIFIER: u64 = 0x0048_e028;
const PU_MSG_RECURSIVE: u64 = 0x0048_e048;
const PU_MSG_BAD_REF_MODIFIER: u64 = 0x0048_e070;
const PU_MSG_UNEXPECTED_IDENT: u64 = 0x0048_e098;
const PU_MSG_EXPECTED_RPAREN: u64 = 0x0048_e0b8;
const PU_MSG_EXPECTED_TERM: u64 = 0x0048_e0d8;

const PU_NEXT_TOKEN: Callee = Callee {
    addr: 0x0032_1af0,
    f: ee::s10::InputParser_NextToken,
};
const PU_ALLOC_NODE: Callee = Callee {
    addr: 0x0032_15f8,
    f: ee::s10::InputParser_AllocNode,
};
const PU_ERROR: Callee = Callee {
    addr: 0x0032_1500,
    f: ee::s10::InputParser_Error,
};
const PU_LOOKUP_BUTTON: Callee = Callee {
    addr: 0x0032_16e8,
    f: ee::s10::InputParser_LookupButton,
};
const PU_LOOKUP_MODIFIER: Callee = Callee {
    addr: 0x0032_17d0,
    f: ee::s10::InputParser_LookupModifier,
};
const PU_LOOKUP_BUILTIN: Callee = Callee {
    addr: 0x0032_1880,
    f: ee::s10::InputParser_LookupBuiltinName,
};
const PU_FIND_IDENTIFIER: Callee = Callee {
    addr: 0x0032_1a40,
    f: ee::s10::InputParser_FindIdentifier,
};
/// Recursion goes through the generated alias (and so the override slot).
const PU_PARSE_UNARY: Callee = Callee {
    addr: 0x0032_3bd8,
    f: ee::s10::InputParser_ParseUnary,
};
const PU_PARSE_CONDITIONAL: Callee = Callee {
    addr: 0x0032_3268,
    f: ee::s10::InputParser_ParseConditional,
};
const PU_FREE_NODE: Callee = Callee {
    addr: 0x0032_1638,
    f: ee::s10::InputParser_FreeNode,
};
const PU_PARSE_FUNCTION2: Callee = Callee {
    addr: 0x0032_4138,
    f: ee::s10::InputParser_ParseFunction2,
};
const PU_PARSE_FUNCTION1: Callee = Callee {
    addr: 0x0032_4398,
    f: ee::s10::InputParser_ParseFunction1,
};

/// How a term ends: the shared tails of the original.
enum PuExit {
    /// `a0`/`a1` (and `a2`) hold an error call's arguments (0x32410c):
    /// report it, return 0.
    Error,
    /// Return 0 (0x324114).
    Zero,
    /// `v0` holds the result; restore `s0` (0x324118).
    Done,
    /// `v0` holds the result and `s0` is restored (0x32411c).
    Restored,
}

#[inline(always)]
fn pu_r(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// The current token kind (`s1` is the parser).
#[inline(always)]
fn pu_token(c: &mut Cpu) -> u64 {
    let a = pu_r(c, S1).wrapping_add(PARSE_UNARY_TOKEN);
    c.lw(a)
}

/// `a0 = self; NextToken()` returning to `ret`.
#[inline(always)]
fn pu_next_token(c: &mut Cpu, ret: u32) -> Result<(), Unwound> {
    c.g[A0][0] = c.g[S1][0];
    PU_NEXT_TOKEN.call(c, ret)
}

/// `a0 = self; AllocNode()` returning to `ret`; the node is left in `s0`.
#[inline(always)]
fn pu_alloc_node(c: &mut Cpu, ret: u32) -> Result<(), Unwound> {
    c.g[A0][0] = c.g[S1][0];
    PU_ALLOC_NODE.call(c, ret)?;
    c.g[S0][0] = c.g[V0][0];
    Ok(())
}

/// Error arguments `a0 = self, a1 = msg` (0x32410c follows).
#[inline(always)]
fn pu_error_args(c: &mut Cpu, msg: u64) -> PuExit {
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = msg;
    PuExit::Error
}

/// The node tail at 0x323f10: `s0[1] = s2; s0[0] = v0; return s0`.
fn pu_store_node(c: &mut Cpu) -> PuExit {
    c.cycles += 4;
    let node = pu_r(c, S0);
    c.sw(node.wrapping_add(4), c.g[S2][0]);
    c.sw(node, c.g[V0][0]);
    c.g[V0][0] = c.g[S0][0];
    PuExit::Done
}

/// `name` is a button (index in `s3`, name text in `s0`): an optional
/// `.modifier`, then a button node `{2, index, modifier}`.
fn pu_button(c: &mut Cpu) -> Result<PuExit, Unwound> {
    c.cycles += 6;
    c.g[S2][0] = PU_MOD_NONE;
    pu_next_token(c, 0x0032_3c48)?;
    c.g[V1][0] = pu_token(c);
    c.g[V0][0] = PU_TOK_DOT;
    if c.g[V1][0] == c.g[V0][0] {
        c.cycles += 6;
        pu_next_token(c, 0x0032_3c60)?;
        c.g[V1][0] = pu_token(c);
        c.g[V0][0] = PU_TOK_NAME;
        c.g[A1][0] = 0x0049_0000;
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 3;
            return Ok(pu_error_args(c, PU_MSG_EXPECTED_MODIFIER));
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        PU_LOOKUP_MODIFIER.call(c, 0x0032_3c88)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = PU_MOD_BAD;
        c.g[A1][0] = 0x0049_0000;
        if c.g[S2][0] == c.g[V0][0] {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = PU_MSG_BAD_BUTTON_MODIFIER;
            c.g[A2][0] = c.g[S0][0];
            PU_ERROR.call(c, 0x0032_3ca8)?;
            c.g[V0][0] = 0;
            return Ok(PuExit::Done);
        }
        c.cycles += 2;
        pu_next_token(c, 0x0032_3cb8)?;
    }
    c.cycles += 9;
    pu_alloc_node(c, 0x0032_3cc0)?;
    let node = pu_r(c, S0);
    c.g[V0][0] = PU_NODE_BUTTON;
    c.sw(node.wrapping_add(4), c.g[S3][0]);
    c.sw(node, c.g[V0][0]);
    c.g[V0][0] = c.g[S0][0];
    c.sw(node.wrapping_add(8), c.g[S2][0]);
    Ok(PuExit::Done)
}

/// After an identifier reference's `.` (0x323d64; `v1` = the next token):
/// expect a modifier name, then build the reference.
fn pu_ident_dot(c: &mut Cpu) -> Result<PuExit, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = PU_TOK_NAME;
    c.g[A1][0] = 0x0049_0000;
    if c.g[V1][0] != c.g[V0][0] {
        c.cycles += 3;
        return Ok(pu_error_args(c, PU_MSG_EXPECTED_MODIFIER));
    }
    c.cycles += 7;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    PU_LOOKUP_MODIFIER.call(c, 0x0032_3d88)?;
    c.g[S4][0] = c.g[V0][0];
    c.g[V0][0] = PU_MOD_BAD;
    c.g[A1][0] = 0x0049_0000;
    if c.g[S4][0] == c.g[V0][0] {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = PU_MSG_BAD_REF_MODIFIER;
        c.g[A2][0] = c.g[S0][0];
        PU_ERROR.call(c, 0x0032_3da8)?;
        c.g[V0][0] = 0;
        return Ok(PuExit::Done);
    }
    c.cycles += 2;
    pu_next_token(c, 0x0032_3db8)?;
    pu_ident_node(c)
}

/// The reference node (0x323db8; identifier entry in `s3`, modifier in
/// `s4`): a constant definition is copied (`{4, value}`), anything else
/// becomes `{1, entry, modifier}`.
fn pu_ident_node(c: &mut Cpu) -> Result<PuExit, Unwound> {
    c.cycles += 5;
    let expr_at = pu_r(c, S3).wrapping_add(PARSE_UNARY_IDENT_EXPR);
    c.g[V0][0] = c.lw(expr_at);
    c.g[V1][0] = PU_NODE_CONST;
    c.g[S2][0] = c.lw(pu_r(c, V0));
    if c.g[S2][0] == c.g[V1][0] {
        c.cycles += 8;
        pu_alloc_node(c, 0x0032_3dd4)?;
        let node = pu_r(c, S0);
        c.sw(node, c.g[S2][0]);
        let expr_at = pu_r(c, S3).wrapping_add(PARSE_UNARY_IDENT_EXPR);
        c.g[V1][0] = c.lw(expr_at);
        c.lwc1(0, pu_r(c, V1).wrapping_add(4));
        c.swc1(0, node.wrapping_add(4));
        return Ok(PuExit::Done);
    }
    c.cycles += 9;
    pu_alloc_node(c, 0x0032_3df4)?;
    let node = pu_r(c, S0);
    c.g[V0][0] = PU_NODE_IDENT;
    c.sw(node.wrapping_add(4), c.g[S3][0]);
    c.sw(node, c.g[V0][0]);
    c.g[V0][0] = c.g[S0][0];
    c.sw(node.wrapping_add(8), c.g[S4][0]);
    Ok(PuExit::Done)
}

/// An identifier token (0x323c24): a button, a built-in config value, or a
/// defined identifier; anything else is an error.
fn pu_name(c: &mut Cpu) -> Result<PuExit, Unwound> {
    c.cycles += 7;
    c.g[S0][0] = sx(pu_r(c, S1).wrapping_add(PARSE_UNARY_TEXT));
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    PU_LOOKUP_BUTTON.call(c, 0x0032_3c34)?;
    c.g[S3][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    if (c.g[S3][0] as i64) >= 0 {
        return pu_button(c);
    }
    c.cycles += 5;
    c.g[A1][0] = c.g[S0][0];
    PU_LOOKUP_BUILTIN.call(c, 0x0032_3ce4)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    if (c.g[S2][0] as i64) >= 0 {
        c.cycles += 7;
        pu_next_token(c, 0x0032_3cf8)?;
        pu_alloc_node(c, 0x0032_3d00)?;
        c.g[V0][0] = PU_NODE_CONFIG;
        return Ok(pu_store_node(c));
    }
    c.cycles += 5;
    c.g[A1][0] = c.g[S0][0];
    PU_FIND_IDENTIFIER.call(c, 0x0032_3d14)?;
    c.g[S3][0] = c.g[V0][0];
    c.g[A1][0] = 0x0049_0000;
    if c.g[S3][0] == 0 {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = PU_MSG_UNEXPECTED_IDENT;
        c.g[A2][0] = c.g[S0][0];
        PU_ERROR.call(c, 0x0032_3e20)?;
        c.g[V0][0] = 0;
        return Ok(PuExit::Done);
    }
    c.cycles += 7;
    pu_next_token(c, 0x0032_3d28)?;
    c.g[V0][0] = sx(pu_r(c, S1).wrapping_add(0x10000));
    let current = pu_r(c, V0).wrapping_add(PARSE_UNARY_CURRENT_IDENT);
    c.g[V1][0] = c.lw(current);
    if c.g[S3][0] == c.g[V1][0] {
        c.cycles += 4;
        return Ok(pu_error_args(c, PU_MSG_RECURSIVE));
    }
    c.g[V1][0] = pu_token(c);
    c.cycles += 3;
    c.g[V0][0] = PU_TOK_DOT;
    c.g[S4][0] = PU_MOD_NONE;
    if c.g[V1][0] != c.g[V0][0] {
        return pu_ident_node(c);
    }
    c.cycles += 3;
    pu_next_token(c, 0x0032_3d60)?;
    c.g[V1][0] = pu_token(c);
    pu_ident_dot(c)
}

/// `-term` / `!term` (0x323e5c / 0x323eac): parse the operand; a constant
/// is folded (negated, or `x == 0 ? 1 : 0`), anything else wrapped in a
/// `{kind, operand}` node.
fn pu_prefix(c: &mut Cpu, not: bool) -> Result<PuExit, Unwound> {
    let (next_ret, term_ret) = if not {
        (0x0032_3eb4, 0x0032_3ebc)
    } else {
        (0x0032_3e64, 0x0032_3e6c)
    };
    c.cycles += 7;
    pu_next_token(c, next_ret)?;
    c.g[A0][0] = c.g[S1][0];
    PU_PARSE_UNARY.call(c, term_ret)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V0][0] = PU_NODE_CONST;
    if c.g[S2][0] == 0 {
        return Ok(PuExit::Zero);
    }
    c.cycles += 3;
    let term = pu_r(c, S2);
    c.g[V1][0] = c.lw(term);
    if c.g[V1][0] == c.g[V0][0] {
        if not {
            c.cycles += 6;
            c.mtc1(0, 0);
            c.lwc1(1, term.wrapping_add(4));
            c.c_eq_s(1, 0);
            let zero = c.fcc();
            c.mov_s(2, 0);
            if zero {
                c.cycles += 2;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 2);
            }
            c.cycles += 3;
            c.swc1(2, term.wrapping_add(4));
        } else {
            c.cycles += 5;
            c.lwc1(0, term.wrapping_add(4));
            c.neg_s(0, 0);
            c.swc1(0, term.wrapping_add(4));
        }
        c.g[V0][0] = c.g[S2][0];
        return Ok(PuExit::Done);
    }
    if not {
        c.cycles += 4;
        pu_alloc_node(c, 0x0032_3f08)?;
        c.g[V0][0] = PU_NODE_NOT;
    } else {
        c.cycles += 5;
        pu_alloc_node(c, 0x0032_3ea0)?;
        c.g[V0][0] = PU_NODE_NEG;
    }
    Ok(pu_store_node(c))
}

/// One switch case (the jump-table target `case`); `None` loops back to the
/// switch (a unary `+`), `Err` is an unwind.
fn pu_case(c: &mut Cpu, case: u32) -> Result<Option<PuExit>, Unwound> {
    let exit = match case {
        PU_CASE_NAME => pu_name(c)?,
        PU_CASE_NAME_DOT => pu_ident_dot(c)?,
        PU_CASE_NUMBER => {
            // A number: `{4, value}`, then the next token.
            c.cycles += 9;
            pu_alloc_node(c, 0x0032_3e30)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] = PU_NODE_CONST;
            let node = pu_r(c, S0);
            c.sw(node, c.g[V0][0]);
            c.lwc1(0, pu_r(c, S1).wrapping_add(PARSE_UNARY_VALUE));
            c.swc1(0, node.wrapping_add(4));
            pu_token_and_return_s0(c, 0x0032_3f70)?
        }
        PU_CASE_PLUS => {
            c.cycles += 4;
            pu_next_token(c, 0x0032_3e54)?;
            c.g[V0][0] = pu_token(c);
            return Ok(None);
        }
        PU_CASE_MINUS => pu_prefix(c, false)?,
        PU_CASE_NOT => pu_prefix(c, true)?,
        PU_CASE_PAREN => {
            // `( expression )`.
            c.cycles += 9;
            pu_next_token(c, 0x0032_3f28)?;
            c.g[A0][0] = c.g[S1][0];
            PU_PARSE_CONDITIONAL.call(c, 0x0032_3f30)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = pu_token(c);
            c.g[V0][0] = PU_TOK_RPAREN;
            c.g[A1][0] = 0x0049_0000;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 1;
                c.g[A0][0] = c.g[S1][0];
                pu_token_and_return_s0(c, 0x0032_3f70)?
            } else {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = PU_MSG_EXPECTED_RPAREN;
                PU_ERROR.call(c, 0x0032_3f50)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                PU_FREE_NODE.call(c, 0x0032_3f5c)?;
                c.g[V0][0] = 0;
                PuExit::Done
            }
        }
        PU_CASE_FUNC_FIRST..=PU_CASE_FUNC_LAST if (case - PU_CASE_FUNC_FIRST) % 0x1c == 0 => {
            // A function call: the node kind follows the keyword.
            let index = (case - PU_CASE_FUNC_FIRST) / 0x1c;
            let parse = if index < 4 {
                PU_PARSE_FUNCTION2
            } else {
                PU_PARSE_FUNCTION1
            };
            c.cycles += 7;
            pu_next_token(c, case + 8)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x14 + index as u64;
            parse.call(c, case + 0x14)?;
            c.lq(16, sp(c).wrapping_add(0x50));
            PuExit::Restored
        }
        PU_CASE_DEFAULT => pu_default(c),
        _ => unreachable!(),
    };
    Ok(Some(exit))
}

/// `a0 = self` already: next token, return `s0` (0x323f68).
fn pu_token_and_return_s0(c: &mut Cpu, ret: u32) -> Result<PuExit, Unwound> {
    c.cycles += 4;
    PU_NEXT_TOKEN.call(c, ret)?;
    c.g[V0][0] = c.g[S0][0];
    Ok(PuExit::Done)
}

/// "Expected term in expression" (0x324100).
fn pu_default(c: &mut Cpu) -> PuExit {
    c.cycles += 3;
    pu_error_args(c, PU_MSG_EXPECTED_TERM)
}

/// Is `t` one of the switch's targets?
fn pu_is_case(t: u32) -> bool {
    matches!(
        t,
        PU_CASE_NAME
            | PU_CASE_NAME_DOT
            | PU_CASE_NUMBER
            | PU_CASE_PLUS
            | PU_CASE_MINUS
            | PU_CASE_NOT
            | PU_CASE_PAREN
            | PU_CASE_DEFAULT
    ) || ((PU_CASE_FUNC_FIRST..=PU_CASE_FUNC_LAST).contains(&t)
        && (t - PU_CASE_FUNC_FIRST) % 0x1c == 0)
}

/// `ParseNode* InputParser_ParseUnary(InputParser* self)` (0x323bd8,
/// `cInputMapParser_parseTerm`): parse one term of an input-map expression
/// at the current token: a button (`button12.pressed`), a config value
/// (`padtype`, `arg3`, `config7`), a defined identifier (constants are
/// copied), a number, unary `+`/`-`/`!` (constants folded), a parenthesised
/// expression or a built-in function call (`max(a, b)`, `sin(x)`). Returns
/// the node, or 0 after reporting a syntax error.
pub fn input_parser_parse_unary(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = pu_token(c);
        let exit = loop {
            // switch (token - 2), 0x25 cases.
            c.cycles += 4;
            c.g[A0][0] = sx(pu_r(c, V0).wrapping_sub(2));
            c.g[V1][0] = (c.g[A0][0] < 0x25) as u64;
            c.g[V0][0] = 0x0049_0000;
            if c.g[V1][0] == 0 {
                break pu_default(c);
            }
            c.cycles += 6;
            c.g[V1][0] = sx(pu_r(c, A0) << 2);
            c.g[V0][0] = sx(PU_JUMP_TABLE);
            c.g[V1][0] = sx(pu_r(c, V1).wrapping_add(PU_JUMP_TABLE));
            c.g[A0][0] = c.lw(pu_r(c, V1));
            let case = pu_r(c, A0);
            if !pu_is_case(case) {
                c.jump_out(case, 0x0032_3bd8, 0x0032_4138);
                return Ok(());
            }
            match pu_case(c, case)? {
                Some(exit) => break exit,
                None => {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_3bfc);
                    }
                }
            }
        };
        let mut exit = exit;
        if let PuExit::Error = exit {
            c.cycles += 2;
            PU_ERROR.call(c, 0x0032_4114)?;
            exit = PuExit::Zero;
        }
        if let PuExit::Zero = exit {
            c.cycles += 1;
            c.g[V0][0] = 0;
            exit = PuExit::Done;
        }
        if let PuExit::Done = exit {
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------- InputParser_ParseUnary

    /// The parser for the ParseUnary test, its source text, node pool and
    /// identifier expressions.
    const PU_PARSER: u32 = crate::difftest::DATA;
    const PU_SRC: u32 = crate::difftest::DATA + 0x4_0000;
    const PU_NODES: u32 = crate::difftest::DATA + 0x2_0000;
    const PU_EXPRS: u32 = crate::difftest::DATA + 0x3_0000;

    /// Defined identifiers: name, expression kind (4 = a constant).
    const PU_IDENTS: [(&[u8], u32); 4] = [(b"speed", 4), (b"gate", 7), (b"self", 1), (b"tilt", 4)];

    /// Scenarios: the current token (kind, text) and the source after it.
    const PU_SCENARIOS: &[(u32, &[u8], &[u8])] = &[
        // Buttons, plain and with modifiers (good, unknown, missing).
        (2, b"button5", b" + 1"),
        (2, b"button12", b".held;"),
        (2, b"button7", b" . pressed"),
        (2, b"button0", b".released"),
        (2, b"button3", b".bogus"),
        (2, b"button3", b".(1)"),
        // Config values.
        (2, b"padtype", b")"),
        (2, b"config4", b""),
        (2, b"arg2", b"*2"),
        // Identifiers: constant, expression, recursive, modifiers.
        (2, b"speed", b""),
        (2, b"gate", b";"),
        (2, b"self", b""),
        (2, b"gate", b".released"),
        (2, b"tilt", b".repeat"),
        (2, b"gate", b".nope"),
        (2, b"gate", b".7"),
        (2, b"button999", b""),
        (2, b"mystery", b""),
        // Numbers and unary operators.
        (11, b"2.5", b" ;"),
        (20, b"+", b"4"),
        (20, b"+", b"+ + button2"),
        (21, b"-", b"3.25"),
        (21, b"-", b"button4.value"),
        (21, b"-", b"-gate"),
        (21, b"-", b";"),
        (24, b"!", b"0"),
        (24, b"!", b"2"),
        (24, b"!", b"speed"),
        (24, b"!", b"(button1)"),
        (24, b"!", b")"),
        // Parentheses.
        (9, b"(", b"1 + 2)"),
        (9, b"(", b"button3 * 2;"),
        (9, b"(", b"1;"),
        // Not a term.
        (0, b"", b""),
        (1, b"#", b""),
        (3, b"=", b"1"),
        (10, b")", b""),
        (22, b"*", b"2"),
        (39, b"?", b""),
        (0xffff_ffff, b"", b""),
    ];

    fn pu_test_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, &b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// `InputParser_ParseUnary` against the generated body with every
    /// callee real (the tokenizer reads a source text after the current
    /// token): each kind of term, each modifier and syntax error, nested
    /// unary operators (the generated recursion), parenthesised expressions
    /// and every built-in function keyword, and tokens that are no term.
    #[test]
    fn input_parser_parse_unary_matches() {
        use crate::difftest::{check, elf_image, scramble, Rng, GAME_GP};
        let funcs = 14u32;
        let n = PU_SCENARIOS.len() as u32 + funcs + 2;
        check(
            ee::s10::f_00323bd8__generated,
            input_parser_parse_unary,
            200,
            &|c, seed| {
                let mut r = Rng(0x0032_3bd8 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                elf_image(c);
                c.pc = 0x0032_3bd8;
                c.g[A0][0] = sx(PU_PARSER);
                let k = seed % n;
                let (tok, text, rest): (u32, &[u8], &[u8]) = if (k as usize) < PU_SCENARIOS.len() {
                    PU_SCENARIOS[k as usize]
                } else if k - (PU_SCENARIOS.len() as u32) < funcs {
                    // max min pow atan2 take two arguments, the rest one.
                    let f = k - PU_SCENARIOS.len() as u32;
                    let rest: &[u8] = if f < 4 {
                        &b"(1, button2)"[..]
                    } else {
                        &b"(speed)"[..]
                    };
                    (25 + f, &b"f"[..], rest)
                } else if k % 2 == 0 {
                    (25, &b"max"[..], &b"1)"[..])
                } else {
                    (30, &b"sin"[..], &b""[..])
                };
                let p = PU_PARSER;
                // Tokenizer: source, position, file name, line.
                pu_test_str(c, PU_SRC, rest);
                c.bus.write32(p, PU_SRC);
                c.bus.write32(p + 4, rest.len() as u32);
                c.bus.write32(p + 8, PU_SRC);
                pu_test_str(c, p + 0xc, b"keys.map");
                c.bus.write32(p + 0x8c, r.next_u32() % 200);
                c.bus.write32(p + 0x90, 0);
                c.bus.write32(p + 0x94, 0);
                // The current token.
                c.bus.write32(p + PARSE_UNARY_TOKEN, tok);
                pu_test_str(c, p + PARSE_UNARY_TEXT, text);
                let value = if seed % 5 == 0 { 0 } else { r.f32(100.0) };
                c.bus.write32(p + PARSE_UNARY_VALUE, value);
                // Free expression nodes.
                for i in 0..48u32 {
                    let node = PU_NODES + 0x10 * i;
                    c.bus.write32(node, r.next_u32());
                    c.bus
                        .write32(node + 4, if i == 47 { 0 } else { node + 0x10 });
                }
                c.bus.write32(p + PARSE_UNARY_FREE_NODES, PU_NODES);
                // Identifiers and the one being defined ("self").
                c.bus
                    .write32(p + PARSE_UNARY_IDENT_COUNT, PU_IDENTS.len() as u32);
                for (i, (name, kind)) in PU_IDENTS.iter().enumerate() {
                    let e = p + PARSE_UNARY_IDENTS + 0x44 * i as u32;
                    let expr = PU_EXPRS + 0x10 * i as u32;
                    pu_test_str(c, e, name);
                    c.bus.write32(e + PARSE_UNARY_IDENT_EXPR, expr);
                    c.bus.write32(expr, *kind);
                    let v = if seed % 7 == 0 { 0 } else { r.f32(10.0) };
                    c.bus.write32(expr + 4, v);
                    c.bus.write32(expr + 8, r.next_u32());
                }
                let current = p + PARSE_UNARY_IDENTS + 0x44 * 2;
                c.bus
                    .write32(p + 0x1_0000 + PARSE_UNARY_CURRENT_IDENT, current);
            },
        );
    }
}
