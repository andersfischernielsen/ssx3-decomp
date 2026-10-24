//! Recursive-descent disassembler.
//!
//! Ghidra's boundary map is authoritative and is what [`crate::program::Program`]
//! decodes against. This module reconstructs instruction boundaries from the
//! bytes alone by following control flow, which both cross-checks Ghidra and
//! gives us a fallback for code Ghidra has not seen.
//!
//! The recursion seeds from a function entry and follows:
//!   * fall-through after ordinary instructions,
//!   * both the taken target and the fall-through of branches (after their delay
//!     slot),
//!   * the target of `j`/`jal` and the return path of `jal`/`jalr`.
//!
//! Indirect `jr` targets are not followed (that is exactly the jump-table case
//! that makes a linear sweep wrong); `eret` ends a path.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use ssx3_core::elf::Elf;

use crate::r5900::{decode, Instruction, Operands};

/// Instructions discovered by recursive descent, keyed by address.
pub struct Disasm {
    pub instructions: BTreeMap<u32, Instruction>,
}

impl Disasm {
    pub fn addresses(&self) -> impl Iterator<Item = u32> + '_ {
        self.instructions.keys().copied()
    }
    pub fn len(&self) -> usize {
        self.instructions.len()
    }
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}

/// How a path terminates.
enum Term {
    None,
    /// Branch to `target`; `fall_through` says whether pc+8 is also reachable.
    Branch {
        target: u32,
        fall_through: bool,
    },
    /// Jump to `target` (None = indirect); `fall_through` for call returns.
    Jump {
        target: Option<u32>,
        fall_through: bool,
    },
    Stop,
}

fn classify(inst: &Instruction, pc: u32) -> Term {
    use Operands::*;
    let btarget = |offset: i16| {
        pc.wrapping_add(4)
            .wrapping_add((offset as i32 as u32).wrapping_mul(4))
    };
    let jtarget = |t: u32| (pc.wrapping_add(4) & 0xF000_0000) | (t << 2);

    match (inst.mnemonic, inst.operands) {
        // `beq $0,$0,off` is an unconditional branch (`b`) with no fall-through.
        ("beq" | "beql", Branch { rs, rt, offset }) => Term::Branch {
            target: btarget(offset),
            fall_through: !(rs == 0 && rt == 0),
        },
        ("bne" | "bnel", Branch { offset, .. }) => Term::Branch {
            target: btarget(offset),
            fall_through: true,
        },
        (
            "blez" | "bgtz" | "blezl" | "bgtzl" | "bltz" | "bgez" | "bltzl" | "bgezl" | "bltzal"
            | "bgezal" | "bltzall" | "bgezall",
            Branch1 { offset, .. },
        ) => Term::Branch {
            target: btarget(offset),
            fall_through: true,
        },
        ("bc1f" | "bc1t", Cop1Branch { offset }) => Term::Branch {
            target: btarget(offset),
            fall_through: true,
        },
        ("j", Jump { target }) => Term::Jump {
            target: Some(jtarget(target)),
            fall_through: false,
        },
        ("jal", Jump { target }) => Term::Jump {
            target: Some(jtarget(target)),
            fall_through: true,
        },
        // Indirect jumps end the path we can follow.
        ("jr", Rs { .. }) => Term::Jump {
            target: Option::None,
            fall_through: false,
        },
        ("jalr", RdRs { .. }) => Term::Jump {
            target: Option::None,
            fall_through: true,
        },
        ("eret", _) => Term::Stop,
        _ => Term::None,
    }
}

/// Recursively disassemble starting at `entry`, visiting at most `max` instructions.
pub fn disassemble(elf: &Elf, entry: u32, max: usize) -> Disasm {
    let mut instructions: BTreeMap<u32, Instruction> = BTreeMap::new();
    let mut work: VecDeque<u32> = VecDeque::new();
    work.push_back(entry);

    let mut visited_starts: BTreeSet<u32> = BTreeSet::new();

    while let Some(start) = work.pop_front() {
        if !visited_starts.insert(start) || instructions.contains_key(&start) {
            continue;
        }
        let mut pc = start;
        let mut pending: Option<Term> = None;

        loop {
            if instructions.len() >= max {
                return Disasm { instructions };
            }
            if pending.is_none() && instructions.contains_key(&pc) {
                break;
            }
            let Some(word) = elf.read_u32(pc) else { break };
            let inst = decode(word);
            instructions.insert(pc, inst);

            if let Some(t) = pending.take() {
                // `pc` was the delay slot of an earlier control transfer.
                match t {
                    Term::Branch {
                        target,
                        fall_through,
                    } => {
                        work.push_back(target);
                        if fall_through {
                            work.push_back(pc.wrapping_add(4));
                        }
                    }
                    Term::Jump {
                        target,
                        fall_through,
                    } => {
                        if let Some(target) = target {
                            work.push_back(target);
                        }
                        if fall_through {
                            work.push_back(pc.wrapping_add(4));
                        }
                    }
                    Term::None | Term::Stop => {}
                }
                break;
            }

            match classify(&inst, pc) {
                Term::None => pc = pc.wrapping_add(4),
                Term::Stop => break,
                Term::Branch { .. } | Term::Jump { .. } => {
                    // The next word is this transfer's delay slot; handle it,
                    // then apply the terminator.
                    pending = Some(classify(&inst, pc));
                    pc = pc.wrapping_add(4);
                }
            }
        }
    }

    Disasm { instructions }
}

/// Result of comparing our boundaries with Ghidra's runs.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Coverage {
    /// Instructions both agree on.
    pub matched: usize,
    /// Instructions we found that Ghidra did not.
    pub extra: usize,
    /// Instructions Ghidra found that we missed.
    pub missing: usize,
}

impl Coverage {
    pub fn is_exact(&self) -> bool {
        self.extra == 0 && self.missing == 0
    }
}

/// Compare a [`Disasm`] with Ghidra's instruction runs.
pub fn compare(disasm: &Disasm, runs: &[(u32, u32)]) -> Coverage {
    let mut gh: BTreeSet<u32> = BTreeSet::new();
    for &(lo, hi) in runs {
        gh.extend((lo..hi).step_by(4));
    }
    let ours: BTreeSet<u32> = disasm.instructions.keys().copied().collect();
    Coverage {
        matched: ours.intersection(&gh).count(),
        extra: ours.difference(&gh).count(),
        missing: gh.difference(&ours).count(),
    }
}
