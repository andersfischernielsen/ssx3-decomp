/// `SNDENC_WriteHeader` (0x003bcba0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sndenc_write_header(c: &mut Cpu) {
    if c.pc != 0x003bcba0 {
        return ee::s13::f_003bcba0__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003bcba0;
    loop {
        match pc {
            0x003bcba0 => {
                c.cycles += 31;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.g[A0][0] = 0xfffffffff8720000;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = c.g[A0][0] | 0x4e1f;
                c.g[S0][0] = 0x510000;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5398));
                c.g[A1][0] = 0x20;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[S0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[RA][0] = sx(0x003bcbd8);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x6), 0u64);
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcbd8)?;
                c.g[A0][0] = 0xb0000;
                c.g[A1][0] = 0x20;
                c.g[RA][0] = sx(0x003bcbe8);
                c.g[A0][0] = c.g[A0][0] | 0x3ee8;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcbe8)?;
                c.g[A0][0] = 0x7ffe0000;
                c.g[A1][0] = 0x20;
                c.g[RA][0] = sx(0x003bcbf8);
                c.g[A0][0] = c.g[A0][0] | 0x8001;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcbf8)?;
                c.g[A0][0] = 0xfc30000;
                c.g[A1][0] = 0x1c;
                c.g[RA][0] = sx(0x003bcc08);
                c.g[A0][0] = c.g[A0][0] | 0xc7dc;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcc08)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5398));
                c.g[A0][0] = 0x1;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32));
                
                if c.g[V0][0] != c.g[A0][0] { c.g[A0][0] = 0x9; pc = 0x003bcc30; continue; }
                pc = 0x003bcc1c; continue;
            }
            0x003bcc1c => {
                c.cycles += 5;
                c.g[A0][0] = 0x8;
                c.g[RA][0] = sx(0x003bcc28);
                c.g[A1][0] = 0x6;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcc28)?;
                let t = true; 
                c.g[A0][0] = 0x60000;
                if t { pc = 0x003bcc3c; continue; }
                pc = 0x003bcc30; continue;
            }
            0x003bcc30 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003bcc38);
                c.g[A1][0] = 0x6;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcc38)?;
                c.g[A0][0] = 0x60000;
                pc = 0x003bcc3c; continue;
            }
            0x003bcc3c => {
                c.cycles += 15;
                c.g[A1][0] = 0x13;
                c.g[A0][0] = c.g[A0][0] | 0xe000;
                c.g[RA][0] = sx(0x003bcc4c);
                c.g[S0][0] = 0u64;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcc4c)?;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = 0x21c0000;
                c.g[A1][0] = 0x1a;
                c.g[A0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                c.g[RA][0] = sx(0x003bcc68);
                c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcc68)?;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x1));
                let t = c.g[V1][0] == 0u64; 
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                if t { pc = 0x003bcca4; continue; }
                pc = 0x003bcc78; continue;
            }
            0x003bcc78 => {
                c.cycles += 2;
                c.g[A0][0] = 0x16;
                pc = 0x003bcc80; continue;
            }
            0x003bcc80 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bcc88);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcc88)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x16;
                if t { if back_edge_unwinds(c, 0x003bcc80) { return Ok(()); } pc = 0x003bcc80; continue; }
                pc = 0x003bcca0; continue;
            }
            0x003bcca0 => {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                pc = 0x003bcca4; continue;
            }
            0x003bcca4 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bccd8; continue; }
                pc = 0x003bccb0; continue;
            }
            0x003bccb0 => {
                c.cycles += 2;
                c.g[A0][0] = 0x17;
                pc = 0x003bccb8; continue;
            }
            0x003bccb8 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bccc0);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bccc0)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x17;
                if t { if back_edge_unwinds(c, 0x003bccb8) { return Ok(()); } pc = 0x003bccb8; continue; }
                pc = 0x003bccd8; continue;
            }
            0x003bccd8 => {
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcd18; continue; }
                pc = 0x003bcce8; continue;
            }
            0x003bcce8 => {
                c.cycles += 2;
                c.g[A0][0] = 0u64;
                pc = 0x003bccf0; continue;
            }
            0x003bccf0 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bccf8);
                c.g[A1][0] = 0x3;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bccf8)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0u64;
                if t { if back_edge_unwinds(c, 0x003bccf0) { return Ok(()); } pc = 0x003bccf0; continue; }
                pc = 0x003bcd10; continue;
            }
            0x003bcd10 => {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                pc = 0x003bcd18; continue;
            }
            0x003bcd18 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcd48; continue; }
                pc = 0x003bcd20; continue;
            }
            0x003bcd20 => {
                c.cycles += 2;
                c.g[A0][0] = 0x3;
                pc = 0x003bcd28; continue;
            }
            0x003bcd28 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bcd30);
                c.g[A1][0] = 0x2;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcd30)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x3;
                if t { if back_edge_unwinds(c, 0x003bcd28) { return Ok(()); } pc = 0x003bcd28; continue; }
                pc = 0x003bcd48; continue;
            }
            0x003bcd48 => {
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcd88; continue; }
                pc = 0x003bcd58; continue;
            }
            0x003bcd58 => {
                c.cycles += 2;
                c.g[A0][0] = 0x6;
                pc = 0x003bcd60; continue;
            }
            0x003bcd60 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bcd68);
                c.g[A1][0] = 0x3;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcd68)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x6;
                if t { if back_edge_unwinds(c, 0x003bcd60) { return Ok(()); } pc = 0x003bcd60; continue; }
                pc = 0x003bcd80; continue;
            }
            0x003bcd80 => {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                pc = 0x003bcd88; continue;
            }
            0x003bcd88 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcdb8; continue; }
                pc = 0x003bcd90; continue;
            }
            0x003bcd90 => {
                c.cycles += 2;
                c.g[A0][0] = 0x6;
                pc = 0x003bcd98; continue;
            }
            0x003bcd98 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bcda0);
                c.g[A1][0] = 0x3;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcda0)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x6;
                if t { if back_edge_unwinds(c, 0x003bcd98) { return Ok(()); } pc = 0x003bcd98; continue; }
                pc = 0x003bcdb8; continue;
            }
            0x003bcdb8 => {
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcdf0; continue; }
                pc = 0x003bcdc8; continue;
            }
            0x003bcdc8 => {
                c.cycles += 2;
                c.g[A0][0] = 0x1;
                pc = 0x003bcdd0; continue;
            }
            0x003bcdd0 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bcdd8);
                c.g[A1][0] = 0x1;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcdd8)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x1;
                if t { if back_edge_unwinds(c, 0x003bcdd0) { return Ok(()); } pc = 0x003bcdd0; continue; }
                pc = 0x003bcdf0; continue;
            }
            0x003bcdf0 => {
                c.cycles += 2;
                c.g[S1][0] = 0x3;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                pc = 0x003bcdf8; continue;
            }
            0x003bcdf8 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bce28; continue; }
                pc = 0x003bce04; continue;
            }
            0x003bce04 => {
                c.cycles += 1;
                c.g[A0][0] = 0x3;
                pc = 0x003bce08; continue;
            }
            0x003bce08 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bce10);
                c.g[A1][0] = 0x2;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bce10)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x3;
                if t { if back_edge_unwinds(c, 0x003bce08) { return Ok(()); } pc = 0x003bce08; continue; }
                pc = 0x003bce28; continue;
            }
            0x003bce28 => {
                c.cycles += 3;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                let t = (c.g[S1][0] as i64) >= 0; 
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                if t { if back_edge_unwinds(c, 0x003bcdf8) { return Ok(()); } pc = 0x003bcdf8; continue; }
                pc = 0x003bce34; continue;
            }
            0x003bce34 => {
                c.cycles += 1;
                c.g[S1][0] = 0x6;
                pc = 0x003bce38; continue;
            }
            0x003bce38 => {
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bce70; continue; }
                pc = 0x003bce48; continue;
            }
            0x003bce48 => {
                c.cycles += 2;
                c.g[A0][0] = 0x7;
                pc = 0x003bce50; continue;
            }
            0x003bce50 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bce58);
                c.g[A1][0] = 0x3;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bce58)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x7;
                if t { if back_edge_unwinds(c, 0x003bce50) { return Ok(()); } pc = 0x003bce50; continue; }
                pc = 0x003bce70; continue;
            }
            0x003bce70 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 11i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                if t { if back_edge_unwinds(c, 0x003bce38) { return Ok(()); } pc = 0x003bce38; continue; }
                pc = 0x003bce7c; continue;
            }
            0x003bce7c => {
                c.cycles += 7;
                c.g[A0][0] = 0x8;
                c.g[RA][0] = sx(0x003bce88);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bce88)?;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x1));
                let t = c.g[V1][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcec0; continue; }
                pc = 0x003bce98; continue;
            }
            0x003bce98 => {
                c.cycles += 2;
                c.g[A0][0] = 0u64;
                pc = 0x003bcea0; continue;
            }
            0x003bcea0 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bcea8);
                c.g[A1][0] = 0x18;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcea8)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0u64;
                if t { if back_edge_unwinds(c, 0x003bcea0) { return Ok(()); } pc = 0x003bcea0; continue; }
                pc = 0x003bcec0; continue;
            }
            0x003bcec0 => {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[A0][0] = c.g[S2][0];
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcfbc; continue; }
                pc = 0x003bced4; continue;
            }
            0x003bced4 => {
                c.cycles += 1;
                c.g[S3][0] = 0x1;
                pc = 0x003bced8; continue;
            }
            0x003bced8 => {
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32));
                let t = c.g[V0][0] != c.g[S3][0]; 
                c.g[S1][0] = 0u64;
                if t { pc = 0x003bcf38; continue; }
                pc = 0x003bcee8; continue;
            }
            0x003bcee8 => {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = 0xd;
                pc = 0x003bcef0; continue;
            }
            0x003bcef0 => {
                c.cycles += 6;
                c.g[RA][0] = sx(0x003bcef8);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcef8)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 14i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0xd;
                if t { if back_edge_unwinds(c, 0x003bcef0) { return Ok(()); } pc = 0x003bcef0; continue; }
                pc = 0x003bcf08; continue;
            }
            0x003bcf08 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                if t { pc = 0x003bcfa4; continue; }
                pc = 0x003bcf14; continue;
            }
            0x003bcf14 => {
                c.cycles += 1;
                c.g[A0][0] = 0xb;
                pc = 0x003bcf18; continue;
            }
            0x003bcf18 => {
                c.cycles += 6;
                c.g[RA][0] = sx(0x003bcf20);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcf20)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0xb;
                if t { if back_edge_unwinds(c, 0x003bcf18) { return Ok(()); } pc = 0x003bcf18; continue; }
                pc = 0x003bcf30; continue;
            }
            0x003bcf30 => {
                c.cycles += 2;
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                if t { pc = 0x003bcfa4; continue; }
                pc = 0x003bcf38; continue;
            }
            0x003bcf38 => {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = 0xd;
                pc = 0x003bcf40; continue;
            }
            0x003bcf40 => {
                c.cycles += 6;
                c.g[RA][0] = sx(0x003bcf48);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcf48)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 5i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0xd;
                if t { if back_edge_unwinds(c, 0x003bcf40) { return Ok(()); } pc = 0x003bcf40; continue; }
                pc = 0x003bcf58; continue;
            }
            0x003bcf58 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 14i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                if t { pc = 0x003bcf98; continue; }
                pc = 0x003bcf64; continue;
            }
            0x003bcf64 => {
                c.cycles += 1;
                c.g[A0][0] = 0xb;
                pc = 0x003bcf68; continue;
            }
            0x003bcf68 => {
                c.cycles += 6;
                c.g[RA][0] = sx(0x003bcf70);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcf70)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 14i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0xb;
                if t { if back_edge_unwinds(c, 0x003bcf68) { return Ok(()); } pc = 0x003bcf68; continue; }
                pc = 0x003bcf80; continue;
            }
            0x003bcf80 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                if t { pc = 0x003bcf98; continue; }
                pc = 0x003bcf88; continue;
            }
            0x003bcf88 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003bcf90);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcf90)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                pc = 0x003bcf98; continue;
            }
            0x003bcf98 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x8;
                if t { if back_edge_unwinds(c, 0x003bcf88) { return Ok(()); } pc = 0x003bcf88; continue; }
                pc = 0x003bcfa0; continue;
            }
            0x003bcfa0 => {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                pc = 0x003bcfa4; continue;
            }
            0x003bcfa4 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.g[S2][0];
                if t { if back_edge_unwinds(c, 0x003bced8) { return Ok(()); } pc = 0x003bced8; continue; }
                pc = 0x003bcfb4; continue;
            }
            0x003bcfb4 => {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                pc = 0x003bcfbc; continue;
            }
            0x003bcfbc => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcff4; continue; }
                pc = 0x003bcfc4; continue;
            }
            0x003bcfc4 => {
                c.cycles += 1;
                c.g[A0][0] = 0u64;
                pc = 0x003bcfc8; continue;
            }
            0x003bcfc8 => {
                c.cycles += 11;
                c.g[RA][0] = sx(0x003bcfd0);
                c.g[A1][0] = 0x18;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcfd0)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = 0u64;
                c.g[RA][0] = sx(0x003bcfe0);
                c.g[A1][0] = 0x18;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcfe0)?;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0u64;
                if t { if back_edge_unwinds(c, 0x003bcfc8) { return Ok(()); } pc = 0x003bcfc8; continue; }
                pc = 0x003bcff4; continue;
            }
            0x003bcff4 => {
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = 0x1;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32));
                
                if c.g[V0][0] != c.g[V1][0] { c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8)); pc = 0x003bd030; continue; }
                pc = 0x003bd008; continue;
            }
            0x003bd008 => {
                c.cycles += 10;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = 0xff;
                c.sb((c.g[V1][0] as u32).wrapping_add(0xe8), 0u64);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                c.sb((c.g[V1][0] as u32).wrapping_add(0x7cb), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                let t = true; 
                c.sb((c.g[V1][0] as u32).wrapping_add(0x7ca), c.g[A0][0]);
                if t { pc = 0x003bd05c; continue; }
                pc = 0x003bd030; continue;
            }
            0x003bd030 => {
                c.cycles += 11;
                c.g[A1][0] = 0xff;
                c.g[A2][0] = 0xf8;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x7d5));
                c.g[V0][0] = c.g[V0][0] | 0x7;
                c.sb((c.g[A0][0] as u32).wrapping_add(0x7d5), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.sb((c.g[V0][0] as u32).wrapping_add(0x7d4), c.g[A1][0]);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.sb((c.g[V0][0] as u32).wrapping_add(0x7d7), c.g[A2][0]);
                pc = 0x003bd05c; continue;
            }
            0x003bd05c => {
                c.cycles += 7;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003bcba0, 0x003bd078); return Ok(()); }
        }
    }
    });
}
