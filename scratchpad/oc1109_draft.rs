/// `SPCH_BuildSentencePart` (0x003d8e58): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn spch_build_sentence_part(c: &mut Cpu) {
    if c.pc != 0x003d8e58 {
        return ee::s14::f_003d8e58__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003d8e58;
    loop {
        match pc {
            0x003d8e58 => {
                c.cycles += 26;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
                c.g[T4][0] = c.g[A0][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[T5][0] = 0x1;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(31, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[S4][0] = c.g[A1][0];
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[A1][0] = 0u64;
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lbu((c.g[S4][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.lbu((c.g[S4][0] as u32).wrapping_add(0x6));
                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
                c.g[A0][0] = c.lbu((c.g[S4][0] as u32).wrapping_add(0x5));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = c.g[V0][0] & 0x7c;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                let t = c.g[A0][0] == 0u64; 
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                if t { pc = 0x003d8ee4; continue; }
                pc = 0x003d8ec0; continue;
            }
            0x003d8ec0 => {
                c.cycles += 2;
                c.g[V1][0] = c.g[S5][0];
                pc = 0x003d8ec8; continue;
            }
            0x003d8ec8 => {
                c.cycles += 7;
                c.sw((c.g[V1][0] as u32), 0u64);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lbu((c.g[S4][0] as u32).wrapping_add(0x5));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                if t { if back_edge_unwinds(c, 0x003d8ec8) { return Ok(()); } pc = 0x003d8ec8; continue; }
                pc = 0x003d8ee4; continue;
            }
            0x003d8ee4 => {
                c.cycles += 3;
                c.g[S3][0] = 0xb;
                c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add(0x5f));
                pc = 0x003d8ef0; continue;
            }
            0x003d8ef0 => {
                c.cycles += 7;
                c.sb((c.g[V0][0] as u32), 0u64);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                let t = (c.g[S3][0] as i64) >= 0; 
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
                if t { if back_edge_unwinds(c, 0x003d8ef0) { return Ok(()); } pc = 0x003d8ef0; continue; }
                pc = 0x003d8f0c; continue;
            }
            0x003d8f0c => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[S4][0] as u32).wrapping_add(0x4));
                c.g[S7][0] = sx((c.g[V0][0] as u32) >> 2);
                c.g[S6][0] = 0x1;
                pc = 0x003d8f18; continue;
            }
            0x003d8f18 => {
                c.cycles += 2;
                let t = c.g[S7][0] == 0u64; 
                c.g[S3][0] = 0u64;
                if t { pc = 0x003d9048; continue; }
                pc = 0x003d8f20; continue;
            }
            0x003d8f20 => {
                c.cycles += 2;
                c.g[FP][0] = sx((c.g[S4][0] as u32).wrapping_add(0x8));
                c.g[S1][0] = c.g[T4][0];
                pc = 0x003d8f28; continue;
            }
            0x003d8f28 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x7));
                
                if c.g[V0][0] != 0u64 { c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1)); pc = 0x003d903c; continue; }
                pc = 0x003d8f34; continue;
            }
            0x003d8f34 => {
                c.cycles += 22;
                c.g[V0][0] = 0x1;
                c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                c.sb((c.g[S1][0] as u32).wrapping_add(0x7), c.g[V0][0]);
                c.g[A2][0] = c.g[SP][0];
                c.g[V0][0] = c.lh((c.g[S1][0] as u32));
                c.g[V1][0] = c.lbu((c.g[A0][0] as u32));
                c.g[A3][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x6));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1734));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A3][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[S2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                c.g[A1][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x62));
                c.sq(12, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = sx(0x003d8f80);
                c.sq(13, (c.g[SP][0] as u32).wrapping_add(0x20));
                Callee { addr: 0x003db838, f: ee::s14::SPCH_DecodeSampleEntry }.call(c, 0x003d8f80)?;
                c.lq(12, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] == 0u64; 
                c.lq(13, (c.g[SP][0] as u32).wrapping_add(0x20));
                if t { pc = 0x003d9000; continue; }
                pc = 0x003d8f8c; continue;
            }
            0x003d8f8c => {
                c.cycles += 2;
                let t = (c.g[S2][0] as i64) <= 0; 
                c.g[A3][0] = 0u64;
                if t { pc = 0x003d9038; continue; }
                pc = 0x003d8f94; continue;
            }
            0x003d8f94 => {
                c.cycles += 7;
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = 0xff;
                c.g[T1][0] = c.g[S1][0];
                c.g[T3][0] = 0xfe;
                c.g[T2][0] = 0x1;
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc));
                pc = 0x003d8fb0; continue;
            }
            0x003d8fb0 => {
                c.cycles += 3;
                c.g[A0][0] = c.lbu((c.g[A2][0] as u32).wrapping_add(0x8));
                let t = c.g[A0][0] == c.g[T0][0]; 
                c.g[V0][0] = c.g[A0][0] & 0x80;
                if t { pc = 0x003d9024; continue; }
                pc = 0x003d8fbc; continue;
            }
            0x003d8fbc => {
                c.cycles += 2;
                
                if c.g[V0][0] == 0u64 { c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1)); pc = 0x003d9028; continue; }
                pc = 0x003d8fc4; continue;
            }
            0x003d8fc4 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[A2][0] as u32));
                let t = c.g[V0][0] != c.g[T0][0]; 
                c.g[A0][0] = c.g[A0][0] & 0x7f;
                if t { pc = 0x003d9008; continue; }
                pc = 0x003d8fd0; continue;
            }
            0x003d8fd0 => {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                
                if c.g[V1][0] != 0u64 { c.g[V0][0] = c.lbu((c.g[A1][0] as u32)); pc = 0x003d8ff0; continue; }
                pc = 0x003d8fe4; continue;
            }
            0x003d8fe4 => {
                c.cycles += 3;
                c.sb((c.g[T1][0] as u32).wrapping_add(0x7), 0u64);
                let t = true; 
                c.g[S6][0] = 0u64;
                if t { pc = 0x003d9024; continue; }
                pc = 0x003d8ff0; continue;
            }
            0x003d8ff0 => {
                c.cycles += 4;
                c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> ((c.g[V0][0] as u32) & 31)) as u32);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                
                if c.g[V0][0] != 0u64 { c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1)); pc = 0x003d9028; continue; }
                pc = 0x003d9000; continue;
            }
            0x003d9000 => {
                c.cycles += 2;
                let t = true; 
                c.g[T5][0] = 0u64;
                if t { pc = 0x003d9050; continue; }
                pc = 0x003d9008; continue;
            }
            0x003d9008 => {
                c.cycles += 2;
                
                if c.g[V0][0] != c.g[T3][0] { c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1)); pc = 0x003d9028; continue; }
                pc = 0x003d9010; continue;
            }
            0x003d9010 => {
                c.cycles += 5;
                c.g[V1][0] = c.lbu((c.g[A1][0] as u32));
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                c.g[V1][0] = sx((c.g[T2][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                pc = 0x003d9024; continue;
            }
            0x003d9024 => {
                c.cycles += 1;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                pc = 0x003d9028; continue;
            }
            0x003d9028 => {
                c.cycles += 4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                if t { if back_edge_unwinds(c, 0x003d8fb0) { return Ok(()); } pc = 0x003d8fb0; continue; }
                pc = 0x003d9038; continue;
            }
            0x003d9038 => {
                c.cycles += 1;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                pc = 0x003d903c; continue;
            }
            0x003d903c => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S7][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                if t { if back_edge_unwinds(c, 0x003d8f28) { return Ok(()); } pc = 0x003d8f28; continue; }
                pc = 0x003d9048; continue;
            }
            0x003d9048 => {
                c.cycles += 2;
                let t = c.g[S6][0] == 0u64; 
                c.g[S6][0] = 0x1;
                if t { if back_edge_unwinds(c, 0x003d8f18) { return Ok(()); } pc = 0x003d8f18; continue; }
                pc = 0x003d9050; continue;
            }
            0x003d9050 => {
                c.cycles += 13;
                c.lq(31, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[V0][0] = c.g[T5][0];
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003d8e58, 0x003d9084); return Ok(()); }
        }
    }
    });
}
