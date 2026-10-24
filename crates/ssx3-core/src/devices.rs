//! Models of the Emotion Engine's on-chip devices that live in the
//! `0x1000_0000` register window: the four timers, the DMA controller
//! (DMAC) and its ten channels, and the interrupt controller (INTC).
//!
//! Offsets passed to [`Devices::read32`] / [`Devices::write32`] are **byte
//! offsets from [`REGISTERS_IO_BASE`]** (`0x1000_0000`), so an access to the
//! T0 counter is offset `0x00`, and GIF-DMA's `CHCR` is offset `0x0A000`.
//! Offsets that this model does not know about return `None` (reads) or
//! `false` (writes) so the runtime can log them instead of silently
//! swallowing state.
//!
//! The layout is transcribed from `docs/ps2-bible.md` ("PS2 EE Hardware
//! Register Map"), which in turn cites the EE User's Manual. Only the pieces
//! the game actually touches are modelled; the rest of the window (VIF, GIF,
//! IPU, SIF, ...) is handled elsewhere or left for later.
//!
//! This module is pure Rust with no dependencies, matching the rest of
//! `ssx3-core`.

/// Base of the EE hardware register window.
pub const REGISTERS_IO_BASE: u32 = crate::memory_map::device::REGISTERS_IO_BASE;
/// Inclusive last byte of the EE hardware register window.
pub const REGISTERS_IO_END: u32 = crate::memory_map::device::REGISTERS_IO_END;

// ---------------------------------------------------------------------------
// Timers
// ---------------------------------------------------------------------------

/// Base address (as an offset from [`REGISTERS_IO_BASE`]) of each timer.
///
/// Timers 2 and 3 have no `HOLD` register; reads/writes to it simply miss and
/// are reported as unhandled.
pub const TIMER_BASES: [u32; 4] = [
    0x0000_0000, // T0
    0x0000_0800, // T1
    0x0000_1000, // T2
    0x0000_1800, // T3
];

/// Register offsets inside one timer, relative to its base in [`TIMER_BASES`].
pub mod timer_reg {
    /// Counter value (16-bit; the EE fixes the upper bits to zero).
    pub const COUNT: u32 = 0x00;
    /// Mode: clock source, gate, zero-return, count-up enable and flags.
    pub const MODE: u32 = 0x10;
    /// Compare value (16-bit).
    pub const COMP: u32 = 0x20;
    /// Hold register (timers 0 and 1 only).
    pub const HOLD: u32 = 0x30;
}

/// Bit fields of a timer's `MODE` register (`docs/ps2-bible.md`).
pub mod timer_mode {
    /// Count-up enable: `0` stops the counter, `1` starts/restarts it.
    pub const CUE: u32 = 1 << 7;
    /// Compare-interrupt enable.
    pub const CMPE: u32 = 1 << 8;
    /// Overflow-interrupt enable.
    pub const OVFE: u32 = 1 << 9;
    /// Equal flag: set when the counter matches `COMP`. Sticky in this model.
    pub const EQUF: u32 = 1 << 10;
    /// Overflow flag: set when the 16-bit counter wraps. Sticky in this model.
    pub const OVFF: u32 = 1 << 11;
    /// Zero return: clear the counter when it reaches `COMP`.
    pub const ZRET: u32 = 1 << 6;
}

/// The 16-bit mask applied to the counter and compare/hold registers.
const TIMER_MASK: u32 = 0xffff;

/// One EE timer.
///
/// The register values are stored as `u32` because the CPU accesses them a
/// word at a time; `COUNT`, `COMP` and `HOLD` are masked to 16 bits on write,
/// matching the hardware.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Timer {
    /// `Tn_COUNT`: the counter.
    pub count: u32,
    /// `Tn_MODE`: control and flag bits (see [`timer_mode`]).
    pub mode: u32,
    /// `Tn_COMP`: compare value.
    pub comp: u32,
    /// `Tn_HOLD`: hold value (only meaningful for timers 0 and 1).
    pub hold: u32,
}

impl Timer {
    /// A stopped timer with all registers zero.
    pub const fn new() -> Self {
        Self {
            count: 0,
            mode: 0,
            comp: 0,
            hold: 0,
        }
    }

    /// Whether the counter is currently running (`MODE.CUE` set).
    pub const fn enabled(&self) -> bool {
        self.mode & timer_mode::CUE != 0
    }

    /// Read a timer register by its offset relative to the timer base.
    ///
    /// Returns `None` for offsets that are not a timer register, so callers
    /// can distinguish "reads as zero" from "not modelled".
    pub fn read_offset(&self, off: u32) -> Option<u32> {
        Some(match off {
            timer_reg::COUNT => self.count,
            timer_reg::MODE => self.mode,
            timer_reg::COMP => self.comp,
            timer_reg::HOLD => self.hold,
            _ => return None,
        })
    }

    /// Write a timer register by its offset relative to the timer base.
    ///
    /// `COUNT`, `COMP` and `HOLD` are 16-bit and are masked. Returns whether
    /// the offset named a timer register.
    pub fn write_offset(&mut self, off: u32, val: u32) -> bool {
        match off {
            // A write to COUNT loads the counter.
            timer_reg::COUNT => self.count = val & TIMER_MASK,
            timer_reg::MODE => self.mode = val,
            timer_reg::COMP => self.comp = val & TIMER_MASK,
            timer_reg::HOLD => self.hold = val & TIMER_MASK,
            _ => return false,
        }
        true
    }

    /// Advance a running counter by `cycles` timer ticks.
    ///
    /// * Does nothing while `MODE.CUE` is clear.
    /// * Wraps at 16 bits, setting `MODE.OVFF` when it does.
    /// * When the counter reaches `COMP` (and `COMP` is non-zero) it sets
    ///   `MODE.EQUF`. With `MODE.ZRET` set the counter then reloads to zero
    ///   (`COMP` is the reload point); otherwise it keeps counting past the
    ///   match.
    pub fn tick(&mut self, cycles: u32) {
        if !self.enabled() {
            return;
        }

        let mut n = self.count.wrapping_add(cycles) as u64;
        if n > TIMER_MASK as u64 {
            self.mode |= timer_mode::OVFF;
            n &= TIMER_MASK as u64;
        }

        if self.comp != 0 && n >= self.comp as u64 {
            self.mode |= timer_mode::EQUF;
            if self.mode & timer_mode::ZRET != 0 {
                n = 0;
            }
        }

        self.count = n as u32 & TIMER_MASK;
    }
}

// ---------------------------------------------------------------------------
// DMA controller
// ---------------------------------------------------------------------------

/// Base offset of each of the ten DMA channels (offset from
/// [`REGISTERS_IO_BASE`]).
pub const DMA_CHANNEL_BASES: [u32; 10] = [
    0x0000_8000, // 0: VIF0
    0x0000_9000, // 1: VIF1
    0x0000_a000, // 2: GIF
    0x0000_b000, // 3: fromIPU
    0x0000_b400, // 4: toIPU
    0x0000_c000, // 5: SIF0
    0x0000_c400, // 6: SIF1
    0x0000_c800, // 7: SIF2
    0x0000_d000, // 8: fromSPR
    0x0000_d400, // 9: toSPR
];

/// Per-channel DMA register offsets, relative to a channel base in
/// [`DMA_CHANNEL_BASES`].
pub mod dma_reg {
    /// `Dn_CHCR`: channel control (bit 8 starts a transfer).
    pub const CHCR: u32 = 0x00;
    /// `Dn_MADR`: memory address.
    pub const MADR: u32 = 0x10;
    /// `Dn_QWC`: quadword count.
    pub const QWC: u32 = 0x20;
    /// `Dn_TADR`: tag address (chain mode).
    pub const TADR: u32 = 0x30;
    /// `Dn_ASR`: address stack (`ASR0`; `ASR1` at `+0x50` is not modelled).
    pub const ASR: u32 = 0x40;
}

/// `D_CTRL`: DMAC master enable.
pub const DMA_CTRL: u32 = 0x0000_e000;
/// `D_STAT`: DMAC interrupt status.
pub const DMA_STAT: u32 = 0x0000_e010;
/// `D_PCR`: DMAC channel priority.
pub const DPCR: u32 = 0x0000_e020;

/// One DMA channel's registers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DmaChannel {
    /// `Dn_CHCR`: channel control.
    pub chcr: u32,
    /// `Dn_MADR`: source/destination memory address.
    pub madr: u32,
    /// `Dn_QWC`: number of quadwords.
    pub qwc: u32,
    /// `Dn_TADR`: tag list address.
    pub tadr: u32,
    /// `Dn_ASR`: address stack pointer.
    pub asr: u32,
}

impl DmaChannel {
    /// A channel with every register cleared.
    pub const fn new() -> Self {
        Self {
            chcr: 0,
            madr: 0,
            qwc: 0,
            tadr: 0,
            asr: 0,
        }
    }

    /// Read a channel register by its offset relative to the channel base.
    pub fn read_offset(&self, off: u32) -> Option<u32> {
        Some(match off {
            dma_reg::CHCR => self.chcr,
            dma_reg::MADR => self.madr,
            dma_reg::QWC => self.qwc,
            dma_reg::TADR => self.tadr,
            dma_reg::ASR => self.asr,
            _ => return None,
        })
    }

    /// Write a channel register by its offset relative to the channel base.
    ///
    /// Returns whether the offset named a modelled register.
    pub fn write_offset(&mut self, off: u32, val: u32) -> bool {
        match off {
            dma_reg::CHCR => self.chcr = val,
            dma_reg::MADR => self.madr = val,
            dma_reg::QWC => self.qwc = val,
            dma_reg::TADR => self.tadr = val,
            dma_reg::ASR => self.asr = val,
            _ => return false,
        }
        true
    }
}

// ---------------------------------------------------------------------------
// Interrupt controller
// ---------------------------------------------------------------------------

/// `INTC_STAT`: source status bits.
pub const INTC_STAT: u32 = 0x0000_f000;
/// `INTC_MASK`: per-source enable mask.
pub const INTC_MASK: u32 = 0x0000_f010;

/// The EE interrupt controller.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Intc {
    /// `INTC_STAT`: pending interrupt sources.
    pub stat: u32,
    /// `INTC_MASK`: enabled sources.
    pub mask: u32,
}

impl Intc {
    /// An interrupt controller with nothing pending and everything masked.
    pub const fn new() -> Self {
        Self { stat: 0, mask: 0 }
    }

    /// Sources that are both pending and enabled.
    pub const fn pending(&self) -> u32 {
        self.stat & self.mask
    }
}

// ---------------------------------------------------------------------------
// The whole window
// ---------------------------------------------------------------------------

/// The modelled portion of the EE `0x1000_0000` hardware window.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Devices {
    /// The four timers, T0..T3.
    pub timers: [Timer; 4],
    /// The ten DMA channels, in channel-index order.
    pub dma: [DmaChannel; 10],
    /// `D_CTRL`: DMAC master enable.
    pub d_ctrl: u32,
    /// `D_STAT`: DMAC interrupt status.
    pub d_stat: u32,
    /// `D_PCR`: DMAC channel priority.
    pub dpcr: u32,
    /// The interrupt controller.
    pub intc: Intc,
}

impl Devices {
    /// A freshly reset device block: timers stopped, DMA idle, no interrupts.
    pub const fn new() -> Self {
        Self {
            timers: [Timer::new(); 4],
            dma: [DmaChannel::new(); 10],
            d_ctrl: 0,
            d_stat: 0,
            dpcr: 0,
            intc: Intc::new(),
        }
    }

    /// Read a 32-bit register at `off`, a byte offset from [`REGISTERS_IO_BASE`].
    ///
    /// Returns `None` when the offset is not modelled so the caller can log
    /// the access.
    pub fn read32(&self, off: u32) -> Option<u32> {
        for (i, &base) in TIMER_BASES.iter().enumerate() {
            if let Some(rel) = off.checked_sub(base) {
                if let Some(v) = self.timers[i].read_offset(rel) {
                    return Some(v);
                }
            }
        }
        for (i, &base) in DMA_CHANNEL_BASES.iter().enumerate() {
            if let Some(rel) = off.checked_sub(base) {
                if let Some(v) = self.dma[i].read_offset(rel) {
                    return Some(v);
                }
            }
        }
        match off {
            DMA_CTRL => Some(self.d_ctrl),
            DMA_STAT => Some(self.d_stat),
            DPCR => Some(self.dpcr),
            INTC_STAT => Some(self.intc.stat),
            INTC_MASK => Some(self.intc.mask),
            _ => None,
        }
    }

    /// Write a 32-bit register at `off`, a byte offset from [`REGISTERS_IO_BASE`].
    ///
    /// Returns whether the offset was handled. DMA and INTC registers store
    /// their value directly; the caller is expected to clear `INTC_STAT` bits
    /// explicitly when acknowledging an interrupt.
    pub fn write32(&mut self, off: u32, val: u32) -> bool {
        for (i, &base) in TIMER_BASES.iter().enumerate() {
            if let Some(rel) = off.checked_sub(base) {
                if self.timers[i].write_offset(rel, val) {
                    return true;
                }
            }
        }
        for (i, &base) in DMA_CHANNEL_BASES.iter().enumerate() {
            if let Some(rel) = off.checked_sub(base) {
                if self.dma[i].write_offset(rel, val) {
                    return true;
                }
            }
        }
        match off {
            DMA_CTRL => self.d_ctrl = val,
            DMA_STAT => self.d_stat = val,
            DPCR => self.dpcr = val,
            INTC_STAT => self.intc.stat = val,
            INTC_MASK => self.intc.mask = val,
            _ => return false,
        }
        true
    }

    /// Advance every running timer by `cycles` ticks.
    pub fn timers_tick(&mut self, cycles: u32) {
        for t in &mut self.timers {
            t.tick(cycles);
        }
    }

    /// Counter value of timer `i`, or `0` if `i` is out of range.
    pub fn timer_count(&self, i: usize) -> u32 {
        self.timers.get(i).map_or(0, |t| t.count)
    }

    /// Interrupts that are both pending and unmasked (`STAT & MASK`).
    pub const fn intc_pending(&self) -> u32 {
        self.intc.pending()
    }
}

// ---------------------------------------------------------------------------
// Names
// ---------------------------------------------------------------------------

/// Readable names for the timer registers, indexed by `[timer][register]`.
const TIMER_REG_NAMES: [[&str; 4]; 4] = [
    ["T0_COUNT", "T0_MODE", "T0_COMP", "T0_HOLD"],
    ["T1_COUNT", "T1_MODE", "T1_COMP", "T1_HOLD"],
    ["T2_COUNT", "T2_MODE", "T2_COMP", "T2_HOLD"],
    ["T3_COUNT", "T3_MODE", "T3_COMP", "T3_HOLD"],
];

/// Readable names for the DMA channel registers, indexed by `[channel][register]`.
const DMA_REG_NAMES: [[&str; 5]; 10] = [
    [
        "D0_VIF0_CHCR",
        "D0_VIF0_MADR",
        "D0_VIF0_QWC",
        "D0_VIF0_TADR",
        "D0_VIF0_ASR",
    ],
    [
        "D1_VIF1_CHCR",
        "D1_VIF1_MADR",
        "D1_VIF1_QWC",
        "D1_VIF1_TADR",
        "D1_VIF1_ASR",
    ],
    [
        "D2_GIF_CHCR",
        "D2_GIF_MADR",
        "D2_GIF_QWC",
        "D2_GIF_TADR",
        "D2_GIF_ASR",
    ],
    [
        "D3_IPUF_CHCR",
        "D3_IPUF_MADR",
        "D3_IPUF_QWC",
        "D3_IPUF_TADR",
        "D3_IPUF_ASR",
    ],
    [
        "D4_IPUT_CHCR",
        "D4_IPUT_MADR",
        "D4_IPUT_QWC",
        "D4_IPUT_TADR",
        "D4_IPUT_ASR",
    ],
    [
        "D5_SIF0_CHCR",
        "D5_SIF0_MADR",
        "D5_SIF0_QWC",
        "D5_SIF0_TADR",
        "D5_SIF0_ASR",
    ],
    [
        "D6_SIF1_CHCR",
        "D6_SIF1_MADR",
        "D6_SIF1_QWC",
        "D6_SIF1_TADR",
        "D6_SIF1_ASR",
    ],
    [
        "D7_SIF2_CHCR",
        "D7_SIF2_MADR",
        "D7_SIF2_QWC",
        "D7_SIF2_TADR",
        "D7_SIF2_ASR",
    ],
    [
        "D8_SPRF_CHCR",
        "D8_SPRF_MADR",
        "D8_SPRF_QWC",
        "D8_SPRF_TADR",
        "D8_SPRF_ASR",
    ],
    [
        "D9_SPRT_CHCR",
        "D9_SPRT_MADR",
        "D9_SPRT_QWC",
        "D9_SPRT_TADR",
        "D9_SPRT_ASR",
    ],
];

/// Map a timer-relative offset to its index in [`TIMER_REG_NAMES`].
fn timer_reg_index(off: u32) -> Option<usize> {
    Some(match off {
        timer_reg::COUNT => 0,
        timer_reg::MODE => 1,
        timer_reg::COMP => 2,
        timer_reg::HOLD => 3,
        _ => return None,
    })
}

/// Map a channel-relative offset to its index in [`DMA_REG_NAMES`].
fn dma_reg_index(off: u32) -> Option<usize> {
    Some(match off {
        dma_reg::CHCR => 0,
        dma_reg::MADR => 1,
        dma_reg::QWC => 2,
        dma_reg::TADR => 3,
        dma_reg::ASR => 4,
        _ => return None,
    })
}

/// Readable name for a well-known register at byte offset `off` from
/// [`REGISTERS_IO_BASE`], or `None` if the offset is not recognised.
///
/// This is intended for logging IO accesses, so it covers every register
/// [`Devices`] models.
pub fn register_name(off: u32) -> Option<&'static str> {
    for (i, &base) in TIMER_BASES.iter().enumerate() {
        if let Some(rel) = off.checked_sub(base) {
            if let Some(reg) = timer_reg_index(rel) {
                return Some(TIMER_REG_NAMES[i][reg]);
            }
        }
    }
    for (i, &base) in DMA_CHANNEL_BASES.iter().enumerate() {
        if let Some(rel) = off.checked_sub(base) {
            if let Some(reg) = dma_reg_index(rel) {
                return Some(DMA_REG_NAMES[i][reg]);
            }
        }
    }
    Some(match off {
        DMA_CTRL => "D_CTRL",
        DMA_STAT => "D_STAT",
        DPCR => "D_PCR",
        INTC_STAT => "INTC_STAT",
        INTC_MASK => "INTC_MASK",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_register_roundtrip_and_masking() {
        let mut t = Timer::new();
        assert!(t.write_offset(timer_reg::COUNT, 0x1_2345));
        assert_eq!(t.read_offset(timer_reg::COUNT), Some(0x2345));
        assert!(t.write_offset(timer_reg::MODE, timer_mode::CUE));
        assert_eq!(t.read_offset(timer_reg::MODE), Some(timer_mode::CUE));
        assert!(t.write_offset(timer_reg::COMP, 0x1234));
        assert_eq!(t.read_offset(timer_reg::COMP), Some(0x1234));
        assert!(t.write_offset(timer_reg::HOLD, 0xabcd));
        assert_eq!(t.read_offset(timer_reg::HOLD), Some(0xabcd));
        // A gap inside the timer block is not a register.
        assert_eq!(t.read_offset(0x08), None);
        assert!(!t.write_offset(0x08, 1));
    }

    #[test]
    fn timer_counts_and_reloads() {
        let mut d = Devices::new();
        // T0: count up, zero-return at the compare value.
        assert!(d.write32(
            TIMER_BASES[0] + timer_reg::MODE,
            timer_mode::CUE | timer_mode::ZRET
        ));
        assert!(d.write32(TIMER_BASES[0] + timer_reg::COMP, 10));

        d.timers_tick(4);
        assert_eq!(d.timer_count(0), 4);
        assert_eq!(d.timers[0].mode & timer_mode::EQUF, 0);

        // Reaching the compare value sets the equal flag and reloads to zero.
        d.timers_tick(6);
        assert_eq!(d.timer_count(0), 0);
        assert_ne!(d.timers[0].mode & timer_mode::EQUF, 0);

        // T1 was never enabled, so it must not have moved.
        assert_eq!(d.timer_count(1), 0);
    }

    #[test]
    fn timer_overflow_sets_flag() {
        let mut d = Devices::new();
        d.timers[2].write_offset(timer_reg::MODE, timer_mode::CUE);
        d.timers[2].write_offset(timer_reg::COUNT, 0xffff);
        d.timers_tick(1);
        assert_eq!(d.timer_count(2), 0);
        assert_ne!(d.timers[2].mode & timer_mode::OVFF, 0);
    }

    #[test]
    fn stopped_timer_does_not_count() {
        let mut d = Devices::new();
        d.write32(TIMER_BASES[3] + timer_reg::COUNT, 5);
        d.timers_tick(100);
        assert_eq!(d.timer_count(3), 5);
    }

    #[test]
    fn dma_register_roundtrip() {
        let mut d = Devices::new();
        // Channel 2 is the GIF path; exercise each modelled register.
        let base = DMA_CHANNEL_BASES[2];
        let cases = [
            (dma_reg::CHCR, 0x0000_0104),
            (dma_reg::MADR, 0x0010_0000),
            (dma_reg::QWC, 0x0000_0003),
            (dma_reg::TADR, 0x0020_0000),
            (dma_reg::ASR, 0x0000_0001),
        ];
        for (reg, val) in cases {
            assert!(
                d.write32(base + reg, val),
                "write {reg:#x} should be handled"
            );
            assert_eq!(d.read32(base + reg), Some(val));
        }
        assert_eq!(d.dma[2].madr, 0x0010_0000);

        // Global DMAC registers store and return their value.
        for (reg, val) in [(DMA_CTRL, 1u32), (DMA_STAT, 0xf), (DPCR, 0x0005_0005)] {
            assert!(d.write32(reg, val));
            assert_eq!(d.read32(reg), Some(val));
        }

        // A gap between modelled channel registers is unhandled.
        assert_eq!(d.read32(base + 0x08), None);
        assert!(!d.write32(base + 0x08, 0));
    }

    #[test]
    fn intc_pending_is_stat_and_mask() {
        let mut d = Devices::new();
        assert_eq!(d.intc_pending(), 0);
        assert!(d.write32(INTC_STAT, 0b1010));
        assert!(d.write32(INTC_MASK, 0b1100));
        assert_eq!(d.read32(INTC_STAT), Some(0b1010));
        assert_eq!(d.read32(INTC_MASK), Some(0b1100));
        assert_eq!(d.intc_pending(), 0b1000);
        assert_eq!(d.intc.stat & d.intc.mask, 0b1000);
    }

    #[test]
    fn unknown_offsets_are_unhandled() {
        let mut d = Devices::new();
        assert_eq!(d.read32(0xdead_beef), None);
        assert!(!d.write32(0xdead_beef, 1));
        // Recognised windows with an unmodelled inner offset also miss.
        assert_eq!(d.read32(0x1ffc), None);
        assert!(!d.write32(0x1ffc, 1));
    }

    #[test]
    fn register_names_cover_the_window() {
        assert_eq!(
            register_name(TIMER_BASES[0] + timer_reg::COUNT),
            Some("T0_COUNT")
        );
        assert_eq!(
            register_name(TIMER_BASES[2] + timer_reg::MODE),
            Some("T2_MODE")
        );
        assert_eq!(
            register_name(DMA_CHANNEL_BASES[2] + dma_reg::CHCR),
            Some("D2_GIF_CHCR")
        );
        assert_eq!(
            register_name(DMA_CHANNEL_BASES[9] + dma_reg::ASR),
            Some("D9_SPRT_ASR")
        );
        assert_eq!(register_name(DMA_CTRL), Some("D_CTRL"));
        assert_eq!(register_name(DMA_STAT), Some("D_STAT"));
        assert_eq!(register_name(DPCR), Some("D_PCR"));
        assert_eq!(register_name(INTC_STAT), Some("INTC_STAT"));
        assert_eq!(register_name(INTC_MASK), Some("INTC_MASK"));
        assert_eq!(register_name(0x08), None);
    }
}
