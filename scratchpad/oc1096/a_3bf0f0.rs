/// `SNDENC_EncodeFrame` (0x003bf0f0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sndenc_encode_frame(c: &mut Cpu) {
    if c.pc != 0x003bf0f0 {
        return ee::s13::f_003bf0f0__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003bf0f0;
    loop {
        match pc {
            0x003bf0f0 => {
                c.cycles += 14;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.g[T3][0] = c.g[A2][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S6][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = c.g[S1][0] & 0x1;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[V0][0] == 0u64; 
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t { pc = 0x003bf130; continue; }
                pc = 0x003bf128; continue;
            }
            0x003bf128 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0xfffffffffffffffd;
                if t { pc = 0x003bf3b8; continue; }
                pc = 0x003bf130; continue;
            }
            0x003bf130 => {
                c.cycles += 14;
                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x1));
                c.g[A0][0] = 0x70000000;
                c.g[S0][0] = c.g[V0][0] | 0xf;
                c.g[V1][0] = 0x510000;
                c.g[T3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.g[T3][0] & c.g[A0][0];
                c.sw((c.g[V1][0] as u32).wrapping_sub(0x5394), c.g[T3][0]);
                c.g[V0][0] = c.g[V0][0] ^ c.g[A0][0];
                c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                c.sb((c.g[T3][0] as u32).wrapping_add(0xa3c), c.g[V0][0]);
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x5394));
                c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xa3c));
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = c.g[V1][0];
                if t { pc = 0x003bf24c; continue; }
                pc = 0x003bf168; continue;
            }
            0x003bf168 => {
                c.cycles += 8;
                c.g[V0][0] = 0x510000;
                c.g[T3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa41));
                c.sw((c.g[A1][0] as u32).wrapping_add(0xa24), c.g[T3][0]);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5398));
                c.g[A0][0] = 0xe0;
                c.g[V1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
                let t = c.g[V1][0] != c.g[A0][0]; 
                c.g[S4][0] = c.g[V0][0];
                if t { pc = 0x003bf1a8; continue; }
                pc = 0x003bf188; continue;
            }
            0x003bf188 => {
                c.cycles += 8;
                c.g[A0][0] = 0x450000;
                c.g[A1][0] = c.g[T3][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3500));
                c.g[A2][0] = 0x40;
                c.g[RA][0] = sx(0x003bf1a0);
                c.g[A3][0] = 0u64;
                Callee { addr: 0x003bc968, f: ee::s13::SNDENC_DmaToSpr }.call(c, 0x003bf1a0)?;
                let t = true; 
                c.g[T3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe41));
                if t { pc = 0x003bf1ec; continue; }
                pc = 0x003bf1a8; continue;
            }
            0x003bf1a8 => {
                c.cycles += 3;
                c.g[V0][0] = 0x160;
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x450000;
                if t { pc = 0x003bf1d4; continue; }
                pc = 0x003bf1b4; continue;
            }
            0x003bf1b4 => {
                c.cycles += 8;
                c.g[A0][0] = 0x450000;
                c.g[A1][0] = c.g[T3][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3100));
                c.g[A2][0] = 0x60;
                c.g[RA][0] = sx(0x003bf1cc);
                c.g[A3][0] = 0u64;
                Callee { addr: 0x003bc968, f: ee::s13::SNDENC_DmaToSpr }.call(c, 0x003bf1cc)?;
                let t = true; 
                c.g[T3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1041));
                if t { pc = 0x003bf1ec; continue; }
                pc = 0x003bf1d4; continue;
            }
            0x003bf1d4 => {
                c.cycles += 6;
                c.g[A1][0] = c.g[T3][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2b00));
                c.g[A2][0] = 0x80;
                c.g[RA][0] = sx(0x003bf1e8);
                c.g[A3][0] = 0u64;
                Callee { addr: 0x003bc968, f: ee::s13::SNDENC_DmaToSpr }.call(c, 0x003bf1e8)?;
                c.g[T3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1241));
                pc = 0x003bf1ec; continue;
            }
            0x003bf1ec => {
                c.cycles += 24;
                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
                c.g[A0][0] = 0x450000;
                c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3680));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa20), c.g[T3][0]);
                c.g[A2][0] = 0x12;
                c.g[A3][0] = 0x10000;
                c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x200));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[T0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = sx((c.g[T3][0] as u32).wrapping_add(0x30));
                c.g[T2][0] = sx((c.g[T3][0] as u32).wrapping_add(0x60));
                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(0xc0));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa38), c.g[V0][0]);
                c.g[A1][0] = c.g[T3][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa2c), c.g[T0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa30), c.g[T1][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa34), c.g[T2][0]);
                c.g[RA][0] = sx(0x003bf244);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa28), c.g[T3][0]);
                Callee { addr: 0x003bc968, f: ee::s13::SNDENC_DmaToSpr }.call(c, 0x003bf244)?;
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                if t { pc = 0x003bf2e8; continue; }
                pc = 0x003bf24c; continue;
            }
            0x003bf24c => {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa41));
                c.g[V1][0] = 0x510000;
                c.sw((c.g[A1][0] as u32).wrapping_add(0xa20), c.g[V0][0]);
                c.g[S4][0] = c.g[V1][0];
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x5398));
                c.g[A0][0] = 0xe0;
                c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x4));
                
                if c.g[V1][0] != c.g[A0][0] { c.g[V0][0] = 0x160; pc = 0x003bf27c; continue; }
                pc = 0x003bf270; continue;
            }
            0x003bf270 => {
                c.cycles += 3;
                c.g[V0][0] = 0x450000;
                let t = true; 
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3500));
                if t { pc = 0x003bf2a0; continue; }
                pc = 0x003bf27c; continue;
            }
            0x003bf27c => {
                c.cycles += 2;
                
                if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = 0x1e0; pc = 0x003bf290; continue; }
                pc = 0x003bf284; continue;
            }
            0x003bf284 => {
                c.cycles += 3;
                c.g[V0][0] = 0x450000;
                let t = true; 
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3100));
                if t { pc = 0x003bf2a0; continue; }
                pc = 0x003bf290; continue;
            }
            0x003bf290 => {
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
                if t { pc = 0x003bf2a8; continue; }
                pc = 0x003bf298; continue;
            }
            0x003bf298 => {
                c.cycles += 2;
                c.g[V0][0] = 0x450000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2b00));
                pc = 0x003bf2a0; continue;
            }
            0x003bf2a0 => {
                c.cycles += 2;
                c.sw((c.g[A1][0] as u32).wrapping_add(0xa24), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
                pc = 0x003bf2a8; continue;
            }
            0x003bf2a8 => {
                c.cycles += 16;
                c.g[A0][0] = 0x450000;
                c.g[A1][0] = 0x450000;
                c.g[A2][0] = 0x450000;
                c.g[A3][0] = 0x450000;
                c.g[V0][0] = 0x450000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3680));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3670));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3650));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x3620));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x35c0));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa28), c.g[A0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa38), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa2c), c.g[A1][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa30), c.g[A2][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa34), c.g[A3][0]);
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                pc = 0x003bf2e8; continue;
            }
            0x003bf2e8 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                
                if c.g[V0][0] == c.g[S1][0] { c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398)); pc = 0x003bf304; continue; }
                pc = 0x003bf2f4; continue;
            }
            0x003bf2f4 => {
                c.cycles += 4;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                c.g[RA][0] = sx(0x003bf300);
                c.sh((c.g[V1][0] as u32).wrapping_add(0x6), 0u64);
                Callee { addr: 0x003bcba0, f: ee::s13::SNDENC_WriteHeader }.call(c, 0x003bf300)?;
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                pc = 0x003bf304; continue;
            }
            0x003bf304 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bf388; continue; }
                pc = 0x003bf310; continue;
            }
            0x003bf310 => {
                c.cycles += 4;
                c.g[S2][0] = c.g[S4][0];
                c.g[S3][0] = 0x1;
                c.g[S1][0] = c.g[S6][0];
                pc = 0x003bf320; continue;
            }
            0x003bf320 => {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                c.g[V0][0] = c.g[A0][0] & 0xf;
                let t = c.g[V0][0] != 0u64; 
                c.g[A2][0] = sx((c.g[S0][0] as u32) << 2);
                if t { if back_edge_unwinds(c, 0x003bf128) { return Ok(()); } pc = 0x003bf128; continue; }
                pc = 0x003bf330; continue;
            }
            0x003bf330 => {
                c.cycles += 10;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[RA][0] = sx(0x003bf348);
                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                Callee { addr: 0x003bd220, f: ee::s13::SNDENC_AnalysisFilterbank }.call(c, 0x003bf348)?;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
                let t = c.g[V1][0] != c.g[S3][0]; 
                
                if t { pc = 0x003bf368; continue; }
                pc = 0x003bf358; continue;
            }
            0x003bf358 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003bf360);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x003bd5b8, f: ee::s13::SNDENC_EncodeChannelMode1 }.call(c, 0x003bf360)?;
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                if t { pc = 0x003bf374; continue; }
                pc = 0x003bf368; continue;
            }
            0x003bf368 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003bf370);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x003bde08, f: ee::s13::SNDENC_EncodeChannelMode2 }.call(c, 0x003bf370)?;
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                pc = 0x003bf374; continue;
            }
            0x003bf374 => {
                c.cycles += 5;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                if t { if back_edge_unwinds(c, 0x003bf320) { return Ok(()); } pc = 0x003bf320; continue; }
                pc = 0x003bf388; continue;
            }
            0x003bf388 => {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                c.g[A0][0] = 0x1;
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
                let t = c.g[V1][0] != c.g[A0][0]; 
                
                if t { pc = 0x003bf3ac; continue; }
                pc = 0x003bf39c; continue;
            }
            0x003bf39c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003bf3a4);
                Callee { addr: 0x003bd9c0, f: ee::s13::SNDENC_PackHeaderMode1 }.call(c, 0x003bf3a4)?;
                let t = true; 
                c.g[V0][0] = 0u64;
                if t { pc = 0x003bf3b8; continue; }
                pc = 0x003bf3ac; continue;
            }
            0x003bf3ac => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003bf3b4);
                c.g[A0][0] = c.g[S6][0];
                Callee { addr: 0x003be6e0, f: ee::s13::SNDENC_PackHeaderMode2 }.call(c, 0x003bf3b4)?;
                c.g[V0][0] = 0u64;
                pc = 0x003bf3b8; continue;
            }
            0x003bf3b8 => {
                c.cycles += 10;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003bf0f0, 0x003bf3e0); return Ok(()); }
        }
    }
    });
}
