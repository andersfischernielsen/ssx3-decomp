//! EE memory map and device windows.
//!
//! Values are transcribed directly from the Ghidra program `SLUS_207.72`
//! (`list_segments`), so addresses here match what you see in Ghidra.
//!
//! Physical EE memory is 32 MiB of RDRAM plus a 16 KiB scratchpad. The R5900
//! also exposes cached/uncached aliases of the same RAM via the high bits of
//! the address; [`crate::Memory`] folds those aliases back together.

/// One contiguous, named region of the address space as Ghidra reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Segment {
    pub name: &'static str,
    pub start: u32,
    /// Inclusive last address, matching Ghidra's `end`.
    pub end: u32,
    pub perm: Perm,
}

impl Segment {
    pub const fn len(&self) -> u32 {
        self.end - self.start + 1
    }
    pub const fn contains(&self, addr: u32) -> bool {
        addr >= self.start && addr <= self.end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Perm {
    pub r: bool,
    pub w: bool,
    pub x: bool,
}

const fn perms(r: bool, w: bool, x: bool) -> Perm {
    Perm { r, w, x }
}

pub const R_X: Perm = perms(true, false, true);
pub const RW: Perm = perms(true, true, false);
pub const RWX: Perm = perms(true, true, true);
pub const R: Perm = perms(true, false, false);

/// The EE's own image (`SLUS_207.72`), as loaded by the PS2 loader.
pub const SLUS_207_72: &[Segment] = &[
    Segment {
        name: ".text",
        start: 0x0010_0000,
        end: 0x0042_e01f,
        perm: R_X,
    },
    // Function-sections emitted by the linker (`-ffunction-sections`). The
    // section names still carry C++ symbol names even though the ELF is
    // stripped, which makes them a free naming source for the decompilation.
    Segment {
        name: ".gnu.linkonce.t.*",
        start: 0x0042_e020,
        end: 0x0042_e587,
        perm: R_X,
    },
    Segment {
        name: ".vutext",
        start: 0x0042_e590,
        end: 0x0043_cd6f,
        perm: R_X,
    },
    Segment {
        name: ".data",
        start: 0x0043_ce00,
        end: 0x0045_680b,
        perm: RW,
    },
    Segment {
        name: ".gnu.linkonce.d.*",
        start: 0x0045_6810,
        end: 0x0045_68bf,
        perm: RW,
    },
    Segment {
        name: ".rodata",
        start: 0x0045_6900,
        end: 0x0049_b017,
        perm: R,
    },
    Segment {
        name: ".gcc_except_table",
        start: 0x0049_b080,
        end: 0x0049_b0db,
        perm: RW,
    },
    Segment {
        name: ".lit4",
        start: 0x0049_b100,
        end: 0x004a_0e57,
        perm: RW,
    },
    Segment {
        name: ".sdata",
        start: 0x004a_0e80,
        end: 0x004a_4bf3,
        perm: RW,
    },
    Segment {
        name: ".sbss",
        start: 0x004a_4c00,
        end: 0x004a_5bbb,
        perm: RW,
    },
    Segment {
        name: ".bss",
        start: 0x004a_5c00,
        end: 0x0053_eadb,
        perm: RW,
    },
];

/// Program image base / entry point.
pub const IMAGE_BASE: u32 = 0x0010_0000;
pub const ENTRY_POINT: u32 = 0x0010_0008;

/// Hardware windows the R5900 sees.
pub mod device {
    /// EE timers, DMA, interrupt controller, and friends.
    pub const REGISTERS_IO_BASE: u32 = 0x1000_0000;
    pub const REGISTERS_IO_END: u32 = 0x1000_ffff;

    /// VU0 microcode / data.
    pub const VU0_CODE: u32 = 0x1100_0000;
    pub const VU0_DATA: u32 = 0x1100_4000;
    /// VU1 microcode / data (16 KiB each).
    pub const VU1_CODE: u32 = 0x1100_8000;
    pub const VU1_DATA: u32 = 0x1100_c000;

    /// Graphics Synthesizer privileged registers.
    pub const GS_REGS_BASE: u32 = 0x1200_0000;
    pub const GS_REGS_END: u32 = 0x1200_1fff;

    /// IOP (R3000) RAM, mirrored through the EE's 32 MiB window.
    pub const IOP_RAM_BASE: u32 = 0x1c00_0000;
    pub const IOP_RAM_END: u32 = 0x1dff_ffff;
    pub const IOP_RAM_SIZE: usize = 2 * 1024 * 1024;

    /// EE scratchpad (SPR).
    pub const SCRATCHPAD_BASE: u32 = 0x7000_0000;
    pub const SCRATCHPAD_SIZE: usize = 16 * 1024;

    /// BIOS ROM windows (cached / uncached).
    pub const BIOS_CACHED: u32 = 0x9fc0_0000;
    pub const BIOS_UNCACHED: u32 = 0xbfc0_0000;
    pub const BIOS_SIZE: usize = 4 * 1024 * 1024;
}

/// Base and size of main RDRAM (the "RAM" windows).
pub const MAIN_RAM_BASE: u32 = 0x0000_0000;
pub const MAIN_RAM_SIZE: usize = 32 * 1024 * 1024;
/// 0x2000_0000 is the uncached alias of main RAM.
pub const MAIN_RAM_UNCACHED: u32 = 0x2000_0000;
/// 0x3000_0000 is the uncached, "accelerated" alias of main RAM. Ghidra labels
/// the mapped window from 0x3010_0000 because the first MiB is not defined
/// there, but the alias itself is a full 32 MiB mirror based at 0x3000_0000.
pub const MAIN_RAM_UNCACHED_ACCEL: u32 = 0x3000_0000;
pub const MAIN_RAM_UNCACHED_ACCEL_WINDOW: u32 = 0x3010_0000;
