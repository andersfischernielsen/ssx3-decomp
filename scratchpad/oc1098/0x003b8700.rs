/// `SND_SetFxEnable` (0x003b8700): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn snd_set_fx_enable(c: &mut Cpu) {
    if c.pc != 0x003b8700 {
        return ee::s13::f_003b8700__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003b8700;
    loop {
        match pc {
            0x003b8700 => {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x003b8720);
                c.g[S1][0] = c.g[A1][0];
                Callee { addr: 0x003ba6e8, f: ee::s13::SND_HandleToChannel }.call(c, 0x003b8720)?;
                c.g[S0][0] = c.g[V0][0];
                let t = (c.g[S0][0] as i64) < 0; 
                c.g[V1][0] = 0xffffffffffffffff;
                if t { pc = 0x003b876c; continue; }
                pc = 0x003b872c; continue;
            }
            0x003b872c => {
                c.cycles += 5;
                c.g[V0][0] = 0x510000;
                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
                c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                let t = true; 
                c.g[S2][0] = 0x8c;
                if t { pc = 0x003b8758; continue; }
                pc = 0x003b8740; continue;
            }
            0x003b8740 => {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1f0));
                c.mult(0, 2, 2, 18);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sb((c.g[V0][0] as u32).wrapping_add(0x5f), c.g[S1][0]);
                c.g[RA][0] = sx(0x003b8758);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                Callee { addr: 0x003c2ec8, f: ee::s13::SND_ChannelApplyFxEnable }.call(c, 0x003b8758)?;
                pc = 0x003b8758; continue;
            }
            0x003b8758 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003b8764);
                c.g[A1][0] = c.g[SP][0];
                Callee { addr: 0x003baa00, f: ee::s13::SND_NextHandleChannel }.call(c, 0x003b8764)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                if t { if back_edge_unwinds(c, 0x003b8740) { return Ok(()); } pc = 0x003b8740; continue; }
                pc = 0x003b876c; continue;
            }
            0x003b876c => {
                c.cycles += 8;
                c.g[V0][0] = c.g[S0][0];
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003b8700, 0x003b878c); return Ok(()); }
        }
    }
    });
}
