//! Vector Units (VU0 and VU1): register file, micro-mode interpreter and the
//! COP2 macro-mode operations the EE issues.
//!
//! INTERFACE CONTRACT:
//! - `Vu::new(index)`: index 0 (4 KiB code / 4 KiB data) or 1 (16 KiB / 16 KiB).
//! - fields `vf: [[u32; 4]; 32]` (raw float bits, x,y,z,w), `vi: [u16; 16]`,
//!   `code`, `data` (byte vectors), `tpc`, `running`.
//! - `macro_op(word)`: execute one EE COP2 "CO" instruction (bit 25 set),
//!   except VCALLMS/VCALLMSR which the EE handles by calling `start(pc)`.
//! - `read_ctrl(reg)` / `write_ctrl(reg, val)`: cfc2/ctc2 control registers
//!   (0..15 = VI, 16 status, 17 MAC, 18 clip, 20 R, 21 I, 22 Q, 26 TPC, ...).
//! - `start(pc)` then `run(max_cycles, kick)`: run micro code until the E bit
//!   ends it (returns true) or the budget expires (returns false, still running).
//!   `kick(&[Qword])` receives each XGKICK GIF packet (VU1 only).

use crate::Qword;

pub struct Vu {
    pub index: usize,
    pub vf: [[u32; 4]; 32],
    pub vi: [u16; 16],
    pub code: Vec<u8>,
    pub data: Vec<u8>,
    pub tpc: u32,
    pub running: bool,
}

impl Vu {
    pub fn new(index: usize) -> Self {
        let size = if index == 0 { 4096 } else { 16384 };
        Self {
            index,
            vf: [[0; 4]; 32],
            vi: [0; 16],
            code: vec![0; size],
            data: vec![0; size],
            tpc: 0,
            running: false,
        }
    }
    pub fn macro_op(&mut self, _word: u32) {}
    pub fn read_ctrl(&self, reg: u32) -> u32 {
        if reg < 16 { self.vi[reg as usize] as u32 } else { 0 }
    }
    pub fn write_ctrl(&mut self, reg: u32, val: u32) {
        if (1..16).contains(&reg) {
            self.vi[reg as usize] = val as u16;
        }
    }
    pub fn start(&mut self, pc: u32) {
        self.tpc = pc;
        self.running = true;
    }
    pub fn run(&mut self, _max_cycles: u64, _kick: &mut dyn FnMut(&[Qword])) -> bool {
        self.running = false;
        true
    }
}
