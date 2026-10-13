//! EE-side COP2 (VU0 macro mode) entry points used by recompiled code.
//!
//! Register moves are exact. Arithmetic macro ops currently go through
//! `Vu::macro_op(word)`; the recompiler will switch to per-op calls once the
//! VU library exposes them (see `docs/investigation/vu-vif.md`).

use crate::cpu::{sx, Cpu};

impl Cpu {
    pub fn qmfc2(&mut self, rt: usize, fd: usize) {
        if rt == 0 {
            return;
        }
        let r = self.bus.hw.vu0.vf[fd];
        self.g[rt] = [r[0] as u64 | (r[1] as u64) << 32, r[2] as u64 | (r[3] as u64) << 32];
    }
    pub fn qmtc2(&mut self, rt: usize, fd: usize) {
        if fd == 0 {
            return;
        }
        let g = self.g[rt];
        self.bus.hw.vu0.vf[fd] = [g[0] as u32, (g[0] >> 32) as u32, g[1] as u32, (g[1] >> 32) as u32];
    }
    pub fn cfc2(&mut self, rt: usize, id: usize) {
        let v = self.bus.hw.vu0.read_ctrl(id as u32);
        if rt != 0 {
            self.g[rt][0] = sx(v);
        }
    }
    pub fn ctc2(&mut self, rt: usize, id: usize) {
        let v = self.g[rt][0] as u32;
        self.bus.hw.vu0.write_ctrl(id as u32, v);
    }
    pub fn vu0_macro(&mut self, word: u32) {
        self.bus.hw.vu0.macro_op(word);
    }
    /// `vcallms`: run a VU0 microprogram to completion.
    pub fn vcallms(&mut self, addr: u32) {
        self.bus.hw.vu0.start(addr);
        self.bus.hw.vu0.run(1_000_000, &mut |_| {});
    }
    pub fn vcallmsr(&mut self) {
        let a = self.bus.hw.vu0.read_ctrl(27) * 8;
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
