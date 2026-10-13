//! VIF0/VIF1: the command stream in front of each VU (UNPACK, MPG, MSCAL,
//! DIRECT, STROW/STCOL/STMASK, ...).
//!
//! INTERFACE CONTRACT:
//! - `Vif::new(index)`.
//! - `write(data, vu, gs)`: consume whole quadwords from DMA (or the FIFO);
//!   command state persists across calls. `vu` is the VU this VIF feeds. For
//!   VIF1, DIRECT/DIRECTHL go to `gs.gif_write(2, ..)` and the VU's XGKICK goes
//!   to `gs.gif_write(1, ..)`.
//! - `read_reg(off)` / `write_reg(off, val)`: VIFn registers (`off` relative to
//!   `0x1000_3800` for VIF0, `0x1000_3C00` for VIF1).
//! - `stalled()`: an interrupt bit (i) or MARK stalled the stream.
//! - `take_irq()`: true once per VIF interrupt (INTC VIF0/VIF1 bit).

use crate::gs::Gs;
use crate::vu::Vu;
use crate::Qword;

pub struct Vif {
    pub index: usize,
}

impl Vif {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
    pub fn write(&mut self, _data: &[Qword], _vu: &mut Vu, _gs: &mut Gs) {}
    pub fn read_reg(&self, _off: u32) -> u32 {
        0
    }
    pub fn write_reg(&mut self, _off: u32, _val: u32) {}
    pub fn stalled(&self) -> bool {
        false
    }
    pub fn take_irq(&mut self) -> bool {
        false
    }
}
