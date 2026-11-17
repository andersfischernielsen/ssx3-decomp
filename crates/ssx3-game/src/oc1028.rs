//! Staging module for the DNAS relay's ELF helpers.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

const DNAS_RELAY_BASE: u32 = 0x0045_0000;
const DNAS_INIT_SLOT: u32 = 0x152c;
const DNAS_UPDATE_SLOT: u32 = 0x1530;
const DNAS_EXIT_SLOT: u32 = 0x1534;
const ELF_MAGIC: u64 = 0x464c_457f;
const ELF_SECTION_OFFSET: u32 = 0x20;
const ELF_SECTION_SIZE: u64 = 0x28;
const ELF_SPECIAL_SECTION_END: u64 = 0xfeff;
const ELF_SECTION_ABS: u64 = 0xfff1;
/// `".text"`, `".symtab"` and `".strtab"`: the sections `DnasRel_ElfOpen` records.
const DNAS_TEXT_NAME: u32 = 0x0049_75f0;
const DNAS_SYMTAB_NAME: u32 = 0x0049_75f8;
const DNAS_STRTAB_NAME: u32 = 0x0049_7600;
/// `".rel.text"`, `".data"`, `".rel.data"`, `".rodata"`, `".rel.rodata"`.
const DNAS_TEXT_RELOCS: u32 = 0x0049_7608;
const DNAS_DATA_NAME: u32 = 0x0049_7618;
const DNAS_DATA_RELOCS: u32 = 0x0049_7620;
const DNAS_RODATA_NAME: u32 = 0x0049_7630;
const DNAS_RODATA_RELOCS: u32 = 0x0049_7638;
const ELF_RELAY_TYPE: u64 = 0x0008_0001;
const ELF_SECTION_COUNT: u32 = 0x30;
const ELF_SECTION_HEADER_SIZE: u32 = 0x32;
const ELF_SECTION_NAME: u32 = 0;
const ELF_STRING_TABLE_OFFSET: u32 = 0x10;
/// Section header words: file offset (`sh_offset`) and size in bytes (`sh_size`).
const REL_SECTION_FILE_OFFSET: u32 = 0x10;
const REL_SECTION_BYTES: u32 = 0x14;
/// One `Elf32_Rel` entry: `r_offset`, `r_info` (symbol << 8 | type).
const REL_ENTRY_SIZE: u32 = 8;
const REL_TYPE_32: u64 = 2;
const REL_TYPE_26: u64 = 4;
const REL_TYPE_HI16: u64 = 5;
const REL_TYPE_LO16: u64 = 6;
/// Waiting HI16 entries: at most 31 {word index, original word} pairs from
/// `sp + 0x100`; the entry count lives at `sp + 0x200`.
const REL_PENDING_LIMIT: u64 = 31;
const REL_PENDING_OFFSET: u32 = 0x100;
const REL_COUNT_SLOT: u32 = 0x200;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003f_d100, dnas_rel_call_update, "DnasRel_CallUpdate"),
    (0x003f_d128, dnas_rel_call_exit, "DnasRel_CallExit"),
    (0x003f_d150, dnas_rel_run, "DnasRel_Run"),
    (0x003f_d1d8, dnas_rel_is_elf, "DnasRel_IsElf"),
    (
        0x003f_d1f0,
        dnas_rel_get_section_header,
        "DnasRel_GetSectionHeader",
    ),
    (0x003f_d210, dnas_rel_find_section, "DnasRel_FindSection"),
    (
        0x003f_d2b8,
        dnas_rel_get_section_addr,
        "DnasRel_GetSectionAddr",
    ),
    (
        0x003f_d2f8,
        dnas_rel_relocate_section,
        "DnasRel_RelocateSection",
    ),
    (0x003f_d558, dnas_rel_elf_open, "DnasRel_ElfOpen"),
    (0x003f_d660, dnas_rel_relocate_all, "DnasRel_RelocateAll"),
    (0x003f_d6e0, dnas_rel_get_symbol, "DnasRel_GetSymbol"),
    (0x003f_d758, dnas_rel_find_symbol, "DnasRel_FindSymbol"),
];

fn dnas_relay_call(c: &mut Cpu, slot: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V1][0] = DNAS_RELAY_BASE as u64;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(DNAS_RELAY_BASE + slot).lw(c);
        frame.save_ra(c, 0);
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Calls the update entry in the loaded DNAS relay.
pub fn dnas_rel_call_update(c: &mut Cpu) {
    dnas_relay_call(c, DNAS_UPDATE_SLOT, 0x003f_d118);
}

/// Calls the exit entry in the loaded DNAS relay.
pub fn dnas_rel_call_exit(c: &mut Cpu) {
    dnas_relay_call(c, DNAS_EXIT_SLOT, 0x003f_d140);
}

/// Runs the loaded DNAS relay around the caller's update callback.
pub fn dnas_rel_run(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = DNAS_RELAY_BASE as u64;
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A3][0];
        c.g[V1][0] = Addr(DNAS_RELAY_BASE + DNAS_INIT_SLOT).lw(c);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        c.g[A1][0] = 0x0102_0000;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 1;
        frame.save_ra(c, 0);
        let init = c.g[V1][0] as u32;
        c.g[S3][0] = DNAS_RELAY_BASE as u64;
        call_ptr(c, init, 0x003f_d190)?;
        loop {
            c.cycles += 7;
            let user_update = c.g[S1][0] as u32;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, user_update, 0x003f_d198)?;
            c.g[V1][0] = Addr((c.g[S3][0] as u32).wrapping_add(DNAS_UPDATE_SLOT)).lw(c);
            let relay_update = c.g[V1][0] as u32;
            c.g[A0][0] = (c.g[V0][0] < 1) as u64;
            call_ptr(c, relay_update, 0x003f_d1a4)?;
            let repeat = c.g[V0][0] == c.g[S2][0];
            c.g[V1][0] = DNAS_RELAY_BASE as u64;
            if repeat {
                if event_due(c) {
                    deliver_event_here(c, 0x003f_d190);
                }
                continue;
            }
            c.cycles += 11;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(DNAS_EXIT_SLOT)).lw(c);
            let relay_exit = c.g[V0][0] as u32;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, relay_exit, 0x003f_d1b8)?;
            frame.restore(c, S0, 0x50);
            frame.restore(c, S1, 0x40);
            frame.restore(c, S2, 0x30);
            frame.restore(c, S3, 0x20);
            frame.restore(c, S4, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    });
}

/// Reports whether the given buffer begins with an ELF file signature.
pub fn dnas_rel_is_elf(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V1][0] = ELF_MAGIC;
    c.g[V0][0] = ((c.g[V0][0] ^ c.g[V1][0]) < 1) as u64;
    abi::ret(c);
}

/// Locates an ELF section header by index in the relay image.
pub fn dnas_rel_get_section_header(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V1][0] = ELF_SECTION_SIZE;
    c.mult(0, 5, 5, 3);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(ELF_SECTION_OFFSET)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}

/// Returns a relay section's address, including the ELF absolute-section case.
pub fn dnas_rel_get_section_addr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = ELF_SPECIAL_SECTION_END;
        let special = c.g[V0][0] < c.g[A1][0];
        frame.save_ra(c, 0);
        if special {
            c.cycles += 3;
            c.g[V0][0] = c.g[A1][0] ^ ELF_SECTION_ABS;
            c.g[V1][0] = 0xffff_ffff_fff0_0000;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = 0;
            }
        } else {
            c.cycles += 4;
            Callee {
                addr: 0x003f_d1f0,
                f: ee::s15::f_003fd1f0,
            }
            .call(c, 0x003f_d2d4)?;
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lw(c);
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    });
}

mod callee {
    use super::*;

    pub(crate) const DNAS_REL_RELOCATE_SECTION: Callee = Callee {
        addr: 0x003f_d2f8,
        f: ee::s15::DnasRel_RelocateSection,
    };
    pub(crate) const DNAS_REL_GET_SECTION_ADDR: Callee = Callee {
        addr: 0x003f_d2b8,
        f: ee::s15::DnasRel_GetSectionAddr,
    };
    pub(crate) const STRCPY: Callee = Callee {
        addr: 0x0041_66f4,
        f: ee::s15::strcpy,
    };
    pub(crate) const DNAS_REL_IS_ELF: Callee = Callee {
        addr: 0x003f_d1d8,
        f: ee::s15::DnasRel_IsElf,
    };
    pub(crate) const DNAS_REL_FIND_SECTION: Callee = Callee {
        addr: 0x003f_d210,
        f: ee::s15::DnasRel_FindSection,
    };
    pub(crate) const STRCMP: Callee = Callee {
        addr: 0x0041_65a8,
        f: ee::s15::strcmp,
    };
    pub(crate) const DNAS_REL_GET_SYMBOL: Callee = Callee {
        addr: 0x003f_d6e0,
        f: ee::s15::DnasRel_GetSymbol,
    };
}

/// Finds a named section header in the DNAS relay ELF image.
pub fn dnas_rel_find_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        c.g[A2][0] = ELF_SECTION_SIZE;
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S4][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x40);
        c.g[S1][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(ELF_SECTION_HEADER_SIZE)).lhu(c);
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(ELF_SECTION_OFFSET)).lw(c);
        c.mult(0, 2, 2, 6);
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(ELF_SECTION_COUNT)).lhu(c);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        let no_sections = c.g[A0][0] == 0;
        c.g[S3][0] = Addr((c.g[V0][0] as u32).wrapping_add(ELF_STRING_TABLE_OFFSET)).lw(c);
        if no_sections {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 1;
            loop {
                c.cycles += 7;
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(ELF_SECTION_NAME)).lw(c);
                c.g[A1][0] = c.g[S4][0];
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                callee::STRCMP.call(c, 0x003f_d274)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    break;
                }
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(ELF_SECTION_COUNT)).lhu(c);
                c.cycles += 4;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(ELF_SECTION_SIZE as u32));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_d260);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Opens a DNAS relay ELF image and records its text section, symbol table and string table.
pub fn dnas_rel_elf_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        c.g[S4][0] = 0;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S3][0];
        callee::DNAS_REL_IS_ELF.call(c, 0x003f_d58c)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0x80000;
            c.cycles += 1;
            c.g[V0][0] = c.g[S4][0];
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x10)).lw(c);
            c.g[V0][0] = ELF_RELAY_TYPE;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = c.g[S4][0];
            } else {
                c.cycles += 19;
                c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[S1][0]);
                c.g[A1][0] = DNAS_TEXT_NAME as u64;
                c.sw(c.g[S0][0] as u32, c.g[S3][0]);
                c.g[A0][0] = c.g[S0][0];
                c.g[S4][0] = 1;
                callee::DNAS_REL_FIND_SECTION.call(c, 0x003f_d5c0)?;
                c.g[A1][0] = DNAS_SYMTAB_NAME as u64;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[V0][0];
                callee::DNAS_REL_FIND_SECTION.call(c, 0x003f_d5d4)?;
                c.g[A1][0] = DNAS_STRTAB_NAME as u64;
                c.g[A0][0] = c.g[S0][0];
                c.g[S2][0] = c.g[V0][0];
                callee::DNAS_REL_FIND_SECTION.call(c, 0x003f_d5e8)?;
                let text_missing = c.g[S1][0] == 0;
                c.g[A0][0] = c.g[V0][0];
                if !text_missing {
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V0][0]);
                    c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x14)).lw(c);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                }
                c.cycles += 2;
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c);
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                }
                c.cycles += 2;
                c.g[V0][0] = c.g[S4][0];
                if c.g[S2][0] != 0 {
                    c.cycles += 6;
                    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x14)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x10)).lw(c);
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S4][0];
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void DnasRel_RelocateSection(DnasRel* rel, const char* section, const char* rel_section, const u32* remap)`
/// (0x3fd2f8): applies the MIPS relocation entries of the relay section named `rel_section` to the
/// section named `section` (nothing when either is missing). Each entry's symbol address comes from
/// `DnasRel_GetSymbol`; an address inside the window starting at `remap[1]` (`remap[0]` bytes long)
/// is replaced by the word at the same offset in the table at `remap[2]`. R_MIPS_32 adds the address
/// to the word, R_MIPS_26 adds it shifted right by 2 (unmasked); R_MIPS_HI16 entries wait in a list
/// of up to 31 on the stack until the next R_MIPS_LO16, which patches its own low half and the high
/// half of every waiting word (with the carry from bit 15), then empties the list. Other types are
/// skipped.
pub fn dnas_rel_relocate_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x2b0);
        frame.save(c, FP, 0x220);
        frame.save(c, S0, 0x2a0);
        c.g[FP][0] = c.g[A0][0];
        frame.save(c, S7, 0x230);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S1, 0x290);
        frame.save(c, S2, 0x280);
        frame.save(c, S3, 0x270);
        frame.save(c, S4, 0x260);
        frame.save(c, S5, 0x250);
        frame.save(c, S6, 0x240);
        frame.save_ra(c, 0x210);
        c.g[S7][0] = c.g[A3][0];
        callee::DNAS_REL_FIND_SECTION.call(c, 0x003f_d334)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[FP][0];
        callee::DNAS_REL_FIND_SECTION.call(c, 0x003f_d344)?;
        c.g[V1][0] = c.g[V0][0];
        'done: {
            if c.g[S1][0] == 0 {
                break 'done;
            }
            c.cycles += 2;
            c.g[S6][0] = 0;
            if c.g[V1][0] == 0 {
                break 'done;
            }
            // s2 = the relocation entries, s5 = the section they patch.
            c.cycles += 10;
            c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(REL_SECTION_FILE_OFFSET)).lw(c);
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(REL_SECTION_BYTES)).lw(c);
            c.g[S3][0] = 0;
            c.g[V0][0] = Addr(c.g[FP][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(REL_SECTION_FILE_OFFSET)).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 3);
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            let count_slot = (c.g[SP][0] as u32).wrapping_add(REL_COUNT_SLOT);
            c.sw(count_slot, c.g[V1][0]);
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            if c.g[V1][0] == 0 {
                break 'done;
            }
            c.cycles += 1;
            loop {
                // s4 = word index, s1 = type, s0 = the word as it is now.
                c.cycles += 16;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                c.g[A0][0] = c.g[FP][0];
                c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_add(4)).lw(c);
                c.g[A2][0] = c.g[SP][0];
                c.g[S4][0] = sx((c.g[V0][0] as u32) >> 2);
                c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                c.g[S1][0] = c.g[A1][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 8);
                c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
                callee::DNAS_REL_GET_SYMBOL.call(c, 0x003f_d3ac)?;

                // a2 = the symbol address, through the remap window.
                c.g[A0][0] = Addr((c.g[S7][0] as u32).wrapping_add(4)).lw(c);
                c.g[A2][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[A2][0] < c.g[A0][0]) as u64;
                let below = c.g[V0][0] != 0;
                c.g[V0][0] = REL_TYPE_26;
                if !below {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S7][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
                    let inside = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    if inside {
                        c.cycles += 3;
                        c.g[V1][0] = Addr((c.g[S7][0] as u32).wrapping_add(8)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = REL_TYPE_26;
                }

                // Dispatch on the type (t1 = next index, t0 = next entry).
                c.cycles += 2;
                let is_26 = c.g[S1][0] == c.g[V0][0];
                c.g[V0][0] = (c.g[S1][0] < 5) as u64;
                if is_26 {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[A2][0] as u32) >> 2);
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                    c.g[T1][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                    c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(REL_ENTRY_SIZE));
                } else {
                    c.cycles += 2;
                    let low_type = c.g[V0][0] != 0;
                    c.g[V0][0] = REL_TYPE_32;
                    if low_type {
                        c.cycles += 2;
                        let is_32 = c.g[S1][0] == c.g[V0][0];
                        c.g[T1][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                        if is_32 {
                            c.cycles += 6;
                            c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                            c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                        } else {
                            c.cycles += 2;
                        }
                        c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(REL_ENTRY_SIZE));
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = REL_TYPE_HI16;
                        let is_hi16 = c.g[S1][0] == c.g[V0][0];
                        c.g[V0][0] = REL_TYPE_LO16;
                        if is_hi16 {
                            c.cycles += 4;
                            c.g[T1][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                            c.g[V0][0] = (c.g[S6][0] < REL_PENDING_LIMIT) as u64;
                            c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(REL_ENTRY_SIZE));
                            if c.g[V0][0] != 0 {
                                c.cycles += 7;
                                c.g[V0][0] = sx((c.g[S6][0] as u32) << 3);
                                c.g[V1][0] =
                                    sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(1));
                                c.g[A0][0] = c.g[V1][0];
                                c.sw(
                                    (c.g[V1][0] as u32).wrapping_add(REL_PENDING_OFFSET),
                                    c.g[S4][0],
                                );
                                c.sw(
                                    (c.g[A0][0] as u32).wrapping_add(REL_PENDING_OFFSET + 4),
                                    c.g[S0][0],
                                );
                            }
                        } else {
                            c.cycles += 2;
                            let is_lo16 = c.g[S1][0] == c.g[V0][0];
                            c.g[T1][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                            if is_lo16 {
                                dnas_rel_relocate_lo16(c);
                            } else {
                                c.cycles += 2;
                                c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(REL_ENTRY_SIZE));
                            }
                        }
                    }
                }

                c.cycles += 5;
                let count_slot = (c.g[SP][0] as u32).wrapping_add(REL_COUNT_SLOT);
                c.g[V1][0] = Addr(count_slot).lw(c);
                c.g[S3][0] = c.g[T1][0];
                c.g[V0][0] = (c.g[S3][0] < c.g[V1][0]) as u64;
                c.g[S2][0] = c.g[T0][0];
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_d380);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x2a0);
        frame.restore(c, S1, 0x290);
        frame.restore(c, S2, 0x280);
        frame.restore(c, S3, 0x270);
        frame.restore(c, S4, 0x260);
        frame.restore(c, S5, 0x250);
        frame.restore(c, S6, 0x240);
        frame.restore(c, S7, 0x230);
        frame.restore(c, FP, 0x220);
        frame.restore_ra(c, 0x210);
        frame.close(c);
        Ok(())
    });
}

/// The R_MIPS_LO16 arm of `DnasRel_RelocateSection`: adds the symbol address (a2) plus the
/// addend made of the first waiting HI16 word's low half and this word's signed low half,
/// stores the low half of the sum into this word, the rounded high half into every waiting
/// word, and empties the waiting list (s6).
fn dnas_rel_relocate_lo16(c: &mut Cpu) {
    c.cycles += 18;
    let first_pending = (c.g[SP][0] as u32).wrapping_add(REL_PENDING_OFFSET + 4);
    c.g[V1][0] = Addr(first_pending).lhu(c);
    c.g[V0][0] = sx((c.g[S0][0] as u32) << 16);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
    c.g[A1][0] = 0xffff_ffff_ffff_0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
    c.g[A1][0] &= c.g[S0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[S4][0] as u32) << 2);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S5][0] as u32));
    c.g[V0][0] = c.g[A2][0] & 0xffff;
    c.g[V1][0] = sx(((c.g[A2][0] as u32) as i32 >> 15) as u32);
    c.g[A1][0] |= c.g[V0][0];
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(REL_ENTRY_SIZE));
    c.g[V1][0] &= 1;
    c.g[A3][0] = 0;
    if c.g[S6][0] != 0 {
        // a2 = high half rounded by bit 15; a1 walks the saved words.
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 16);
        c.g[T2][0] = 0xffff_ffff_ffff_0000;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(REL_PENDING_OFFSET + 4));
        loop {
            c.cycles += 11;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_sub(4)).lw(c);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = (c.g[A3][0] < c.g[S6][0]) as u64;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            c.g[V1][0] &= c.g[T2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
            c.g[V1][0] |= c.g[A2][0];
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
            if c.g[A0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_d4e0);
            }
        }
    }
    c.cycles += 1;
    c.g[S6][0] = 0;
}

/// Relocates the relay's text, data and read-only data sections in order.
pub fn dnas_rel_relocate_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = DNAS_TEXT_RELOCS as u64;
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = DNAS_TEXT_NAME as u64;
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[S1][0];
        callee::DNAS_REL_RELOCATE_SECTION.call(c, 0x003f_d690)?;

        c.g[A1][0] = DNAS_DATA_NAME as u64;
        c.g[A2][0] = DNAS_DATA_RELOCS as u64;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        callee::DNAS_REL_RELOCATE_SECTION.call(c, 0x003f_d6ac)?;

        c.g[A1][0] = DNAS_RODATA_NAME as u64;
        c.g[A2][0] = DNAS_RODATA_RELOCS as u64;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        callee::DNAS_REL_RELOCATE_SECTION.call(c, 0x003f_d6c8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Copies a relay symbol's name and resolves its section-relative address.
pub fn dnas_rel_get_symbol(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
        let no_symbols = c.g[A1][0] == 0;
        c.g[A0][0] = c.g[A2][0];
        if !no_symbols {
            c.cycles += 3;
            c.g[A3][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lw(c);
            let no_names = c.g[A3][0] == 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
            if !no_names {
                c.cycles += 11;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                callee::STRCPY.call(c, 0x003f_d724)?;
                c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xe)).lhu(c);
                c.g[A0][0] = c.g[S1][0];
                callee::DNAS_REL_GET_SECTION_ADDR.call(c, 0x003f_d730)?;
                c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Finds a relay symbol by name and returns its section-relative address.
pub fn dnas_rel_find_symbol(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x60 };
        let mut at_increment = c.pc == 0x003f_d7a0;
        if !at_increment && c.pc != 0x003f_d758 {
            c.jump_out(c.pc, 0x003f_d758, 0x003f_d80c);
            return Ok(());
        }
        if !at_increment {
            c.cycles += 12;
            let _ = Frame::open(c, 0x60);
            frame.save(c, S1, 0x40);
            frame.save(c, S3, 0x20);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S4, 0x10);
            c.g[S3][0] = 0;
            frame.save(c, S0, 0x50);
            frame.save(c, S2, 0x30);
            frame.save_ra(c, 0);
            c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
        }
        let no_symbols = !at_increment && c.g[A0][0] == 0;
        if !at_increment {
            c.g[S4][0] = c.g[A1][0];
        }
        if !no_symbols {
            if !at_increment {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lw(c);
                c.g[S0][0] = c.g[A0][0];
            }
            let no_names = !at_increment && c.g[V0][0] == 0;
            if !no_names {
                if !at_increment {
                    c.cycles += 2;
                    c.g[S2][0] = 0;
                }
                loop {
                    if at_increment {
                        c.cycles += 1;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x14)).lw(c);
                    c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                    let exhausted = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S4][0];
                    if exhausted {
                        break;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lw(c);
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    callee::STRCMP.call(c, 0x003f_d7c4)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 7;
                        c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xe)).lhu(c);
                        c.g[A0][0] = c.g[S1][0];
                        callee::DNAS_REL_GET_SECTION_ADDR.call(c, 0x003f_d7d8)?;
                        c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
                        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        break;
                    }
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    if back_edge_unwinds(c, 0x003f_d7a0) {
                        return Ok(());
                    }
                    at_increment = true;
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, fakes, scramble, Rng, DATA};

    /// The first special section index (`SHN_LORESERVE`).
    const ELF_SPECIAL_SECTION_MIN: u64 = 0xff00;

    fn call_fixture(c: &mut Cpu, seed: u32, slot: u32) {
        let _r = fakes::machine(c, seed, 0x003f_d100, 1);
        c.bus
            .write32(DNAS_RELAY_BASE + slot, fakes::FAKE + (seed % 3) * 0x10);
        c.bus.write32(
            fakes::LOG_UNWIND_AT,
            if seed % 3 == 0 { 0 } else { u32::MAX },
        );
    }

    #[test]
    fn dnas_rel_call_update_matches() {
        fakes::covered(
            ee::s15::f_003fd100__generated,
            dnas_rel_call_update,
            96,
            &|c, seed| {
                call_fixture(c, seed, DNAS_UPDATE_SLOT);
            },
        );
    }

    #[test]
    fn dnas_rel_call_exit_matches() {
        fakes::covered(
            ee::s15::f_003fd128__generated,
            dnas_rel_call_exit,
            96,
            &|c, seed| {
                call_fixture(c, seed, DNAS_EXIT_SLOT);
            },
        );
    }

    #[test]
    fn dnas_rel_run_matches() {
        fakes::covered(
            ee::s15::f_003fd150__generated,
            dnas_rel_run,
            96,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_d150, 7);
                c.g[A1][0] = sx(DATA + 0x200);
                c.g[A2][0] = if seed % 3 == 1 {
                    0x003f_d1d8
                } else {
                    fakes::FAKE + 0x40
                } as u64;
                c.g[A3][0] = sx(DATA);
                c.bus.write32(DATA, 0);
                c.bus
                    .write32(DNAS_RELAY_BASE + DNAS_INIT_SLOT, fakes::FAKE + 0x10);
                c.bus
                    .write32(DNAS_RELAY_BASE + DNAS_UPDATE_SLOT, fakes::FAKE + 0x20);
                c.bus
                    .write32(DNAS_RELAY_BASE + DNAS_EXIT_SLOT, fakes::FAKE + 0x30);
                if seed % 3 == 1 {
                    c.bus.write32(fakes::LOG_RESULTS + 4, 2);
                } else if seed % 3 == 2 {
                    c.bus.write32(fakes::LOG_RESULTS + 8, 1);
                    c.bus.write32(fakes::LOG_RESULTS + 20, 2);
                } else {
                    c.bus.write32(fakes::LOG_RESULTS + 8, 2);
                }
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed % 7 == 0 { 0 } else { u32::MAX },
                );
            },
        );
    }

    #[test]
    fn dnas_rel_is_elf_matches() {
        check(
            ee::s15::f_003fd1d8__generated,
            dnas_rel_is_elf,
            96,
            &|c, seed| {
                let mut r = Rng(0x003f_d1d8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let word = match seed % 3 {
                    0 => ELF_MAGIC as u32,
                    1 => 0,
                    _ => 0xffff_ffff,
                };
                c.bus.write32(DATA, word);
            },
        );
    }

    #[test]
    fn dnas_rel_get_section_header_matches() {
        check_lohi(
            ee::s15::f_003fd1f0__generated,
            dnas_rel_get_section_header,
            96,
            &|c, seed| {
                let mut r = Rng(0x003f_d1f0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx((seed % 5) * 4);
                c.bus.write32(DATA, DATA + 0x100);
                c.bus.write32(DATA + 0x120, (seed % 7) * 0x28);
            },
        );
    }

    #[test]
    fn dnas_rel_get_section_addr_matches() {
        check(
            ee::s15::f_003fd2b8__generated,
            dnas_rel_get_section_addr,
            96,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003f_d2b8, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    1 => ELF_SECTION_ABS,
                    _ => ELF_SPECIAL_SECTION_MIN,
                };
                c.bus.write32(DATA, DATA + 0x100);
                c.bus.write32(DATA + 0x120, 0x40);
                c.bus.write32(DATA + 0x150, fakes::ptr(&mut r));
            },
        );
    }

    #[test]
    fn dnas_rel_relocate_all_matches() {
        check(
            ee::s15::f_003fd660__generated,
            dnas_rel_relocate_all,
            16,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003f_d660, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(fakes::ptr(&mut r));
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus.write16(DATA + 0x1030, 0);
                c.bus.write16(DATA + 0x1032, 0);
                c.bus.write32(DATA + 0x1020, 0x100);
            },
        );
    }

    #[test]
    fn dnas_rel_get_symbol_matches() {
        check(
            ee::s15::f_003fd6e0__generated,
            dnas_rel_get_symbol,
            96,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003f_d6e0, 2);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0;
                c.g[A2][0] = sx(DATA + 0x400);
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus
                    .write32(DATA + 0x10, if seed % 3 == 0 { 0 } else { DATA + 0x500 });
                c.bus
                    .write32(DATA + 0x20, if seed % 3 == 1 { 0 } else { DATA + 0x700 });
                c.bus.write32(DATA + 0x500, 0);
                c.bus.write32(DATA + 0x504, fakes::ptr(&mut r));
                c.bus.write16(DATA + 0x50e, 0);
                c.bus.write32(DATA + 0x1000 + 0x20, 0x100);
                c.bus.write32(DATA + 0x1000 + 0x100 + 0x10, 0x200);
                c.bus.write32(DATA + 0x700, 0x0063_6261);
            },
        );
    }

    /// Writes the NUL-terminated `text` at `addr`.
    fn dnas_rel_put_cstr(c: &mut Cpu, addr: u32, text: &[u8]) {
        for (i, &b) in text.iter().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + text.len() as u32, 0);
    }

    /// The image has four sections ("x" names them); `.text`, `.symtab` and `.strtab` are each
    /// present or renamed by their own seed digit, so every found/missing pair of arms runs.
    /// The names compared against are the game's own strings (`elf_image`).
    #[test]
    fn dnas_rel_elf_open_matches() {
        check(
            ee::s15::f_003fd558__generated,
            dnas_rel_elf_open,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003f_d558, 0);
                crate::difftest::elf_image(c);
                let image = DATA + 0x1000;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(image);
                c.g[A2][0] = sx(fakes::ptr(&mut r));
                crate::difftest::fill(c, &mut r, DATA, 0xc);
                c.bus
                    .write32(image, if seed % 5 == 0 { 0 } else { ELF_MAGIC as u32 });
                c.bus.write32(
                    image + 0x10,
                    if seed % 5 == 1 {
                        0
                    } else {
                        ELF_RELAY_TYPE as u32
                    },
                );
                c.bus.write32(image + 0x20, 0x100);
                c.bus.write16(image + 0x30, 4);
                c.bus.write16(image + 0x32, 0);
                let lead = |present: bool| if present { b'.' } else { b'-' };
                let names = image + 0x40;
                dnas_rel_put_cstr(c, names, b"x");
                dnas_rel_put_cstr(c, names + 2, b".text");
                c.bus.write8(names + 2, lead(seed / 5 % 2 == 0));
                dnas_rel_put_cstr(c, names + 8, b".symtab");
                c.bus.write8(names + 8, lead(seed / 10 % 2 == 0));
                dnas_rel_put_cstr(c, names + 16, b".strtab");
                c.bus.write8(names + 16, lead(seed / 20 % 2 == 0));
                let headers = image + 0x100;
                let sections = [
                    (0, 0x40, 0x18),
                    (2, 0x200 + (seed % 3) * 0x10, 0x80 + seed),
                    (8, 0x300, 0x30 + seed % 16),
                    (16, 0x380, 0x20),
                ];
                for (i, (name, offset, size)) in sections.into_iter().enumerate() {
                    let h = headers + i as u32 * ELF_SECTION_SIZE as u32;
                    c.bus.write32(h + ELF_SECTION_NAME, name);
                    c.bus.write32(h + 0x10, offset);
                    c.bus.write32(h + 0x14, size);
                }
            },
        );
    }

    #[test]
    fn dnas_rel_find_section_matches() {
        check_lohi(
            ee::s15::f_003fd210__generated,
            dnas_rel_find_section,
            96,
            &|c, seed| {
                let mut r = Rng(0x003f_d210 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x500);
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus.write32(DATA + 0x1020, 0x100);
                c.bus.write16(
                    DATA + 0x1030,
                    if seed % 4 == 0 {
                        0
                    } else if seed % 4 == 1 {
                        1
                    } else {
                        2
                    },
                );
                c.bus.write16(DATA + 0x1032, 0);
                c.bus.write32(DATA + 0x1100, 0);
                c.bus.write32(DATA + 0x1110, 0x300);
                c.bus.write32(DATA + 0x1128, 8);
                c.bus.write32(DATA + 0x1300, 0x0000_0061);
                c.bus.write32(DATA + 0x1308, 0x0000_0062);
                c.bus.write32(
                    DATA + 0x500,
                    match seed % 4 {
                        1 => 0x61,
                        2 => 0x62,
                        _ => 0x63,
                    },
                );
            },
        );
    }

    #[test]
    fn dnas_rel_find_symbol_matches() {
        check_lohi(
            ee::s15::f_003fd758__generated,
            dnas_rel_find_symbol,
            96,
            &|c, seed| {
                let mut r = Rng(0x003f_d758 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x500);
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus
                    .write32(DATA + 0x10, if seed % 6 == 0 { 0 } else { DATA + 0x200 });
                c.bus
                    .write32(DATA + 0x14, if seed % 6 == 2 { 0 } else { 2 });
                c.bus
                    .write32(DATA + 0x20, if seed % 6 == 1 { 0 } else { DATA + 0x400 });
                c.bus.write32(DATA + 0x200, 0);
                c.bus.write32(DATA + 0x204, 0x20);
                c.bus.write16(DATA + 0x20e, ELF_SECTION_ABS as u16);
                c.bus.write32(DATA + 0x210, 6);
                c.bus.write32(DATA + 0x214, 0x40);
                c.bus.write16(DATA + 0x21e, ELF_SECTION_ABS as u16);
                c.bus.write32(DATA + 0x400, 0x6870_6c61);
                c.bus.write32(DATA + 0x404, 0x0000_0061);
                c.bus.write32(DATA + 0x406, 0x6174_6562);
                c.bus.write8(DATA + 0x40a, 0);
                c.bus.write32(
                    DATA + 0x500,
                    match seed % 6 {
                        3 => 0x6870_6c61,
                        4 => 0x6174_6562,
                        _ => 0x6d6d_6167,
                    },
                );
                c.bus
                    .write32(DATA + 0x504, if seed % 6 == 3 { 0x61 } else { 0 });
            },
        );
    }

    crate::difftest::events::driven!(driven_dnas_rel_find_symbol, dnas_rel_find_symbol);
    crate::difftest::events::driven!(
        driven_dnas_rel_find_symbol_generated,
        ee::s15::f_003fd758__generated
    );

    /// Both sides re-entered at the 0x3fd7a0 head after an unwind. The symbol names are empty
    /// and odd-aligned, so `strcmp` returns before its own back edge and the first due event
    /// always lands on this function's head (an unwind inside `strcmp` would resume there,
    /// which `drive` cannot follow).
    #[test]
    fn dnas_rel_find_symbol_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            driven_dnas_rel_find_symbol_generated,
            driven_dnas_rel_find_symbol,
            36,
            &|c, seed| {
                let mut r = Rng(0x003f_d758 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x500);
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus.write32(DATA + 0x10, DATA + 0x200);
                c.bus.write32(DATA + 0x14, 2 + seed / 3 % 4);
                c.bus.write32(DATA + 0x20, DATA + 0x400);
                for i in 0..6 {
                    c.bus.write32(DATA + 0x200 + i * 0x10, 1);
                }
                c.bus.write8(DATA + 0x400, 0x7a);
                c.bus.write8(DATA + 0x401, 0);
                c.bus.write32(DATA + 0x500, 0x6d6d_6167);
                c.bus.write8(DATA + 0x504, 0x61);
                c.bus.write8(DATA + 0x505, 0);
                schedule(c, &mut r, seed, 0x003f_d758)
            },
        );
    }

    /// Relay layout for `dnas_rel_relocate_section`: the relay at `DATA` (image, symbol table,
    /// string table), its ELF image with sections "x" (section names), "t" (the patched words)
    /// and "r" (the entries), the symbols and the remap window.
    const DNAS_RELOC_IMAGE: u32 = DATA + 0x1000;
    const DNAS_RELOC_WORDS: u32 = DNAS_RELOC_IMAGE + 0x400;
    const DNAS_RELOC_ENTRIES: u32 = DNAS_RELOC_IMAGE + 0x800;
    const DNAS_RELOC_SYMTAB: u32 = DATA + 0x2000;
    const DNAS_RELOC_STRTAB: u32 = DATA + 0x2400;
    const DNAS_RELOC_REMAP: u32 = DATA + 0x3000;
    const DNAS_RELOC_REMAP_WORDS: u32 = DATA + 0x3100;
    const DNAS_RELOC_NAMES: u32 = DATA + 0x500;

    /// One relocation fixture. `entries` are (type, symbol) pairs patching words 0, 1, ...;
    /// `extra` bytes after the entries test the size's `>> 3`; `named` gives symbols 1, 3 and 5
    /// a one-letter name (otherwise every name is empty, so `strcpy` makes no back edge).
    /// Symbols: 0 below the remap window, 1 inside it (`inside` words in), 2 at its end,
    /// 3 relative to section 1 (below), 4 absolute (below), 5 far above.
    #[allow(clippy::too_many_arguments)]
    fn dnas_rel_relocate_section_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        entries: &[(u32, u32)],
        extra: u32,
        target_found: bool,
        rel_found: bool,
        inside: u32,
        named: bool,
    ) {
        c.pc = 0x003f_d2f8;
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DNAS_RELOC_NAMES + 1);
        c.g[A2][0] = sx(DNAS_RELOC_NAMES + 3);
        c.g[A3][0] = sx(DNAS_RELOC_REMAP);
        // Odd addresses: `strcmp` takes its byte path.
        c.bus
            .write8(DNAS_RELOC_NAMES + 1, if target_found { b't' } else { b'q' });
        c.bus.write8(DNAS_RELOC_NAMES + 2, 0);
        c.bus
            .write8(DNAS_RELOC_NAMES + 3, if rel_found { b'r' } else { b'q' });
        c.bus.write8(DNAS_RELOC_NAMES + 4, 0);
        c.bus.write32(DATA, DNAS_RELOC_IMAGE);
        c.bus.write32(DATA + 0x10, DNAS_RELOC_SYMTAB);
        c.bus.write32(DATA + 0x14, 6);
        c.bus.write32(DATA + 0x20, DNAS_RELOC_STRTAB);
        c.bus.write32(DNAS_RELOC_IMAGE + 0x20, 0x100);
        c.bus.write16(DNAS_RELOC_IMAGE + 0x30, 3);
        c.bus.write16(DNAS_RELOC_IMAGE + 0x32, 0);
        for (i, &b) in b"x\0t\0r\0".iter().enumerate() {
            c.bus.write8(DNAS_RELOC_IMAGE + 0x41 + i as u32, b);
        }
        let headers = DNAS_RELOC_IMAGE + 0x100;
        let sections = [
            (0, 0x41, 6),
            (2, DNAS_RELOC_WORDS - DNAS_RELOC_IMAGE, 0x100),
            (
                4,
                DNAS_RELOC_ENTRIES - DNAS_RELOC_IMAGE,
                entries.len() as u32 * REL_ENTRY_SIZE + extra,
            ),
        ];
        for (i, (name, offset, size)) in sections.into_iter().enumerate() {
            let h = headers + i as u32 * ELF_SECTION_SIZE as u32;
            c.bus.write32(h + ELF_SECTION_NAME, name);
            c.bus.write32(h + REL_SECTION_FILE_OFFSET, offset);
            c.bus.write32(h + REL_SECTION_BYTES, size);
        }
        for (i, &(kind, sym)) in entries.iter().enumerate() {
            let i = i as u32;
            // The low two bits of r_offset are dropped (word index).
            c.bus.write32(DNAS_RELOC_ENTRIES + i * 8, i * 4 + i % 4);
            c.bus
                .write32(DNAS_RELOC_ENTRIES + i * 8 + 4, sym << 8 | kind);
        }
        crate::difftest::fill(c, r, DNAS_RELOC_WORDS, 0x40);
        let name = if named { 3 } else { 1 };
        let symbols = [
            (1, 0x5f0, 0xff00),
            (name, 0x600 + inside * 4, 0xff00),
            (1, 0x640, 0xff00),
            (name, 0x10, 1),
            (1, 0x8000, ELF_SECTION_ABS as u16),
            (name, 0x7000, 0xff00),
        ];
        for (i, (name, value, section)) in symbols.into_iter().enumerate() {
            let sym = DNAS_RELOC_SYMTAB + i as u32 * 0x10;
            c.bus.write32(sym, name);
            c.bus.write32(sym + 4, value);
            c.bus.write16(sym + 0xe, section);
        }
        for (i, &b) in b"z\0zk\0".iter().enumerate() {
            c.bus.write8(DNAS_RELOC_STRTAB + i as u32, b);
        }
        c.bus.write32(DNAS_RELOC_REMAP, 0x40);
        c.bus
            .write32(DNAS_RELOC_REMAP + 4, DNAS_RELOC_IMAGE + 0x600);
        c.bus.write32(DNAS_RELOC_REMAP + 8, DNAS_RELOC_REMAP_WORDS);
        crate::difftest::fill(c, r, DNAS_RELOC_REMAP_WORDS, 0x10);
        // The frame: name buffer, waiting list (read stale by a LO16 with none waiting).
        crate::difftest::fill(c, r, crate::difftest::SP - 0x2b0, 0x6c);
    }

    /// Types in the order the entries come (rotated per seed): HI16 pairs and a triple
    /// before LO16s, a LO16 with nothing waiting, R_MIPS_32/26 and skipped types 0, 1, 3, 7, 0xff.
    const DNAS_RELOC_TYPE_CYCLE: [u32; 18] =
        [5, 2, 5, 6, 4, 6, 0, 5, 5, 5, 6, 7, 3, 2, 0xff, 1, 4, 6];

    /// `seed % 7`: 0 section missing, 1 relocation section missing, 2 no whole entry,
    /// 3 a full waiting list (34 HI16s, the last 3 dropped), else the type cycle rotated by
    /// `seed / 7 % 18`. Symbol `(i + seed % 6) % 6` for entry `i`; `seed / 2 % 16` words into
    /// the remap window for symbol 1; `seed % 8` extra bytes.
    /// `check`, and both bodies must make the same sequence of logged calls (`check` lets a
    /// different sequence pass unless `SSX3_DIFF_STRICT=1`; a callee reached through its
    /// `__generated` body instead of its slot shows up only there).
    fn dnas_rel_relocate_section_check(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = crate::difftest::compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(
                crate::difftest::strict_ok(&r.soft),
                "seed {seed} (scratch): {:#?}",
                r.soft
            );
            let calls: Vec<&String> = r.soft.iter().filter(|m| m.starts_with("calls")).collect();
            assert!(calls.is_empty(), "seed {seed} (call sequence): {calls:#?}");
        }
    }

    #[test]
    fn dnas_rel_relocate_section_matches() {
        dnas_rel_relocate_section_check(
            ee::s15::f_003fd2f8__generated,
            dnas_rel_relocate_section,
            96,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003f_d2f8, 0);
                let mode = seed % 7;
                let sym = |i: u32| (i + seed % 6) % 6;
                let entries: Vec<(u32, u32)> = match mode {
                    2 => Vec::new(),
                    3 => (0..36)
                        .map(|i| {
                            (
                                if i < 34 {
                                    5
                                } else if i == 34 {
                                    6
                                } else {
                                    2
                                },
                                sym(i),
                            )
                        })
                        .collect(),
                    _ => (0..18)
                        .map(|i| {
                            (
                                DNAS_RELOC_TYPE_CYCLE[((i + seed / 7) % 18) as usize],
                                sym(i),
                            )
                        })
                        .collect(),
                };
                dnas_rel_relocate_section_fixture(
                    c,
                    &mut r,
                    &entries,
                    seed % 8,
                    mode != 0,
                    mode != 1,
                    seed / 2 % 16,
                    true,
                );
            },
        );
    }

    /// Events at the two back edges (0x3fd380 per entry, 0x3fd4e0 per waiting HI16): twelve
    /// HI16s then a LO16 and an R_MIPS_32, all on symbol 0 with empty names. The event is
    /// made due after both section lookups (~221 cycles, so never inside `strcmp`):
    /// `seed / 3 % 2 == 0` spreads it over the entries (outer head), `== 1` puts it in the
    /// LO16 entry (~1349..1573 cycles: inner head). Plain functions on both sides: the loops
    /// never unwind (`deliver_event_here`).
    #[test]
    fn dnas_rel_relocate_section_events() {
        use crate::difftest::events::{check_events, schedule, Events};
        check_events(
            ee::s15::f_003fd2f8__generated,
            dnas_rel_relocate_section,
            90,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003f_d2f8, 0);
                let entries: Vec<(u32, u32)> = (0..14)
                    .map(|i| {
                        (
                            if i < 12 {
                                5
                            } else if i == 12 {
                                6
                            } else {
                                2
                            },
                            0,
                        )
                    })
                    .collect();
                dnas_rel_relocate_section_fixture(c, &mut r, &entries, 0, true, true, 0, false);
                let mode = schedule(c, &mut r, seed, 0x003f_d2f8);
                if mode != Events::Off {
                    let step = seed / 6 % 13;
                    let due = if seed / 3 % 2 == 0 {
                        230 + step * 90
                    } else {
                        1380 + step * 14
                    };
                    c.next_event = c.cycles + due as u64;
                }
                mode
            },
        );
    }
}
