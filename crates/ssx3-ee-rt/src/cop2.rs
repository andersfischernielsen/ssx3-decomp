//! EE-side COP2 (VU0 macro mode) entry points used by recompiled code.
//!
//! Register moves are exact. Arithmetic macro ops are lowered by the
//! recompiler to per-op calls on [`Cpu::vu0`] (see [`crate::vu0`]); the VU
//! itself lives in the hardware model.

use crate::cpu::{sx, Cpu};

impl Cpu {
    pub fn qmfc2(&mut self, rt: usize, fd: usize) {
        if rt == 0 {
            return;
        }
        let r = self.bus.plat.vf(fd);
        self.g[rt] = [
            r[0] as u64 | (r[1] as u64) << 32,
            r[2] as u64 | (r[3] as u64) << 32,
        ];
    }
    pub fn qmtc2(&mut self, rt: usize, fd: usize) {
        if fd == 0 {
            return;
        }
        let g = self.g[rt];
        self.bus.plat.set_vf(
            fd,
            [
                g[0] as u32,
                (g[0] >> 32) as u32,
                g[1] as u32,
                (g[1] >> 32) as u32,
            ],
        );
    }
    pub fn cfc2(&mut self, rt: usize, id: usize) {
        let v = self.bus.plat.read_ctrl(id as u32);
        if rt != 0 {
            self.g[rt][0] = sx(v);
        }
    }
    pub fn ctc2(&mut self, rt: usize, id: usize) {
        let v = self.g[rt][0] as u32;
        self.bus.plat.write_ctrl(id as u32, v);
    }
    pub fn vu0_macro(&mut self, word: u32) {
        self.bus.plat.macro_op(word);
    }
    /// `vcallms`: run a VU0 microprogram to completion.
    pub fn vcallms(&mut self, addr: u32) {
        self.bus.plat.vcallms(addr);
    }
    pub fn vcallmsr(&mut self) {
        let a = self.bus.plat.read_ctrl(27) * 8;
        self.vcallms(a);
    }
    pub fn ei(&mut self) {
        self.cop0[12] |= 0x1_0000;
        self.next_event = self.cycles;
    }
    pub fn di(&mut self) {
        self.cop0[12] &= !0x1_0000;
    }
}
