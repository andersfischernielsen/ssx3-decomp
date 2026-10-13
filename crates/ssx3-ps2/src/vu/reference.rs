//! Reference micro-mode interpreter: the *test oracle* for the static
//! recompiler. Not used at runtime (compiled only under `cfg(test)`); install
//! it explicitly with `vu.fallback = Some(reference::run_program)`.

use super::decode::{decode_lower, decode_upper, pair_reads};
use super::exec::{exec_lower, exec_upper};
use super::{Vu, VuExit};
use crate::Qword;

/// Interpret from `start` until the E bit ends the program; same contract
/// as a translated [`super::ProgramFn`].
pub fn run_program(vu: &mut Vu, start: u32, kick: &mut dyn FnMut(&[Qword])) -> VuExit {
    let mask = vu.code.len() as u32 - 1;
    let mut pc = start & mask;
    let mut npc = (pc + 8) & mask;
    let mut ending = false;
    loop {
        let safe = !ending && npc == ((pc + 8) & mask);
        if safe && vu.out_of_budget(1) {
            return VuExit::Budget(pc);
        }
        let lo = u32::from_le_bytes(vu.code[pc as usize..pc as usize + 4].try_into().unwrap());
        let hi = u32::from_le_bytes(
            vu.code[pc as usize + 4..pc as usize + 8]
                .try_into()
                .unwrap(),
        );
        let upper = decode_upper(hi);
        let ibit = hi & 0x8000_0000 != 0;
        let lower = if ibit { None } else { Some(decode_lower(lo)) };
        let reads = pair_reads(&upper, lower.as_ref());
        vu.begin_pair(&reads);
        if ibit {
            vu.loi(lo);
        }
        exec_upper(vu, &upper);
        let taken = match lower {
            Some(l) => exec_lower(vu, &l, pc, kick),
            None => None,
        };
        vu.end_pair();
        pc = npc;
        npc = (npc + 8) & mask;
        if let Some(t) = taken {
            npc = t;
        }
        if ending {
            vu.end_program(pc);
            return VuExit::End;
        }
        if hi & 0x4000_0000 != 0 {
            ending = true;
        }
    }
}
