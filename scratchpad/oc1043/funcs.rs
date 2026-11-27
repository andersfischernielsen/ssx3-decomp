/// `int PATH_PlatGetTrackStatus(int slot, int* elapsed_ms, int* remaining_ms)`
/// (0x2aec58): reads one of the 24 sound-bank monitor slots (`D_004D3E98[slot]`),
/// locks the sound system and refreshes the slot's stream state (in-memory
/// streams: position from the stream header; streaming ones: each sub-request's
/// state), stores the two out values, and returns 0, -1 (no slot) or -2 (no
/// stream handle). Straight transcription of the generated body (labels kept).
#[allow(unused_labels, unused_assignments, unused_mut, unreachable_code, unused_variables)]
pub fn path_plat_get_track_status(c: &mut Cpu) {
    guest(c, |c| {
        'b_2aef80: {
        'b_2aeca8: {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
        c.g[V0][0] = 0x4d0000;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S4][0] = c.g[A1][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[S2][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[RA][0]);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[S0][0] = c.lw((c.g[A0][0] as u32));
        c.sw((c.g[S4][0] as u32), 0u64);
        let t = c.g[S0][0] != 0u64;
        c.sw((c.g[S2][0] as u32), 0u64);
        if t {
        break 'b_2aeca8;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = 0xffffffffffffffff;
        if t {
        break 'b_2aef80;
        } else {
        break 'b_2aeca8;
        }
        }
        }
        'b_2aecc4: {
        c.cycles += 5;
        c.g[V1][0] = 0x1;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
        c.g[S5][0] = 0xfffffffffffffff8;
        let t = (c.g[V0][0] as i64) >= 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V1][0]);
        if t {
        break 'b_2aecc4;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = 0xfffffffffffffffe;
        if t {
        break 'b_2aef80;
        } else {
        break 'b_2aecc4;
        }
        }
        }
        'b_2aef6c: {
        'b_2aef68: {
        'b_2aeddc: {
        c.cycles += 5;
        c.g[RA][0] = sx(0x002aeccc);
        c.sb((c.g[S0][0] as u32).wrapping_add(0x65), c.g[V1][0]);
        call_ptr(c, 0x003b58a0, 0x002aeccc)?;
        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x64));
        if c.g[V0][0] == 0u64 {
        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
        break 'b_2aeddc;
        } else {
        c.cycles += 3;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        if (c.g[A1][0] as i64) < 0 {
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[S4][0]);
        break 'b_2aef6c;
        } else {
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        let t = (c.g[V1][0] as i64) < 0;
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        if t {
        break 'b_2aef68;
        } else {
        'b_2aedac: {
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = sx(0x002aed04);
        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        call_ptr(c, 0x003b6670, 0x002aed04)?;
        c.g[S5][0] = c.g[V0][0];
        if (c.g[S5][0] as i64) < 0 {
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        break 'b_2aedac;
        } else {
        'b_2aed3c: {
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[RA][0] = sx(0x002aed1c);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        call_ptr(c, 0x003b8ad8, 0x002aed1c)?;
        c.g[V1][0] = c.g[V0][0];
        let t = (c.g[V1][0] as i64) >= 0;
        c.sw((c.g[S2][0] as u32), c.g[V1][0]);
        if t {
        break 'b_2aed3c;
        } else {
        c.cycles += 5;
        c.sw((c.g[S2][0] as u32), 0u64);
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        let t = true;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        if t {
        break 'b_2aef68;
        } else {
        break 'b_2aed3c;
        }
        }
        }
        'b_2aed7c: {
        'b_2aed60: {
        c.cycles += 5;
        c.g[V0][0] = 0xa;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.mult(0, 2, 3, 2);
        let t = (c.g[A0][0] as i64) < 0;
        c.sw((c.g[S2][0] as u32), c.g[V0][0]);
        if t {
        break 'b_2aed60;
        } else {
        c.cycles += 4;
        c.mtc1(4, 2);
        c.cvt_s_w(2, 2);
        let t = true;
        c.g[V0][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x20));
        if t {
        break 'b_2aed7c;
        } else {
        break 'b_2aed60;
        }
        }
        }
        c.cycles += 7;
        c.g[V1][0] = sx((c.g[A0][0] as u32) >> 1);
        c.g[V0][0] = c.g[A0][0] & 0x1;
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.mtc1(2, 2);
        c.cvt_s_w(2, 2);
        c.add_s(2, 2, 2);
        c.g[V0][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x20));
        break 'b_2aed7c;
        }
        c.cycles += 12;
        c.g[AT][0] = 0x447a0000;
        c.mtc1(1, 1);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.div_s(0, 2, 0);
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
        c.sw((c.g[S4][0] as u32), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        break 'b_2aedac;
        }
        }
        c.cycles += 12;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
        c.g[RA][0] = sx(0x002aedc8);
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
        call_ptr(c, 0x003b86c0, 0x002aedc8)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        let t = true;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
        if t {
        break 'b_2aef68;
        } else {
        break 'b_2aeddc;
        }
        }
        }
        }
        }
        'b_2aee00: {
        c.cycles += 9;
        c.g[RA][0] = sx(0x002aede4);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003b7b48, 0x002aede4)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
        c.g[S5][0] = c.g[V0][0];
        c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x66));
        c.g[V0][0] = c.lw((c.g[SP][0] as u32));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] == 0u64 {
        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
        break 'b_2aee00;
        } else {
        break 'b_2aee00;
        }
        }
        'b_2aef5c: {
        c.cycles += 3;
        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x66));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 0u64;
        if t {
        break 'b_2aef5c;
        } else {
        c.cycles += 1;
        c.g[S1][0] = 0u64;
        'l_2aee10: loop {
        'b_2aef4c: {
        c.cycles += 8;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        if (c.g[A0][0] as i64) < 0 {
        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x66));
        break 'b_2aef4c;
        } else {
        'b_2aef48: {
        'b_2aef3c: {
        'b_2aeeb8: {
        'b_2aee70: {
        c.cycles += 6;
        c.g[RA][0] = sx(0x002aee38);
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x003b7c40, 0x002aee38)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = 0x1;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
        if t {
        break 'b_2aee70;
        } else {
        'b_2aee60: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x2;
        if t {
        break 'b_2aee60;
        } else {
        c.cycles += 2;
        let t = c.g[V1][0] == 0u64;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        if t {
        break 'b_2aeeb8;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        if t {
        break 'b_2aef3c;
        } else {
        break 'b_2aee60;
        }
        }
        }
        }
        c.cycles += 2;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        if t {
        break 'b_2aeeb8;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        if t {
        break 'b_2aef3c;
        } else {
        break 'b_2aee70;
        }
        }
        }
        }
        c.cycles += 18;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[A0][0]);
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
        let t = true;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
        if t {
        break 'b_2aef48;
        } else {
        break 'b_2aeeb8;
        }
        }
        'b_2aeee8: {
        'b_2aeee4: {
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        c.sw((c.g[S4][0] as u32), c.g[V0][0]);
        let t = c.g[S4][0] != 0u64;
        c.sw((c.g[S2][0] as u32), c.g[V1][0]);
        if t {
        break 'b_2aeee4;
        } else {
        c.cycles += 2;
        if c.g[S2][0] != 0u64 {
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        break 'b_2aeee8;
        } else {
        c.cycles += 5;
        c.sw((c.g[S2][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.sw((c.g[S2][0] as u32), c.g[V1][0]);
        break 'b_2aeee4;
        }
        }
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        break 'b_2aeee8;
        }
        'b_2aef28: {
        c.cycles += 16;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = c.lw((c.g[S2][0] as u32));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[A0][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0u64 {
        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
        break 'b_2aef28;
        } else {
        break 'b_2aef28;
        }
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        let t = true;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
        if t {
        break 'b_2aef48;
        } else {
        break 'b_2aef3c;
        }
        }
        c.cycles += 3;
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        break 'b_2aef48;
        }
        c.cycles += 1;
        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x66));
        break 'b_2aef4c;
        }
        }
        c.cycles += 4;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        if t {
        if event_due(c) { deliver_event_here(c, 0x002aee10); }
continue 'l_2aee10;
        } else {
        break 'b_2aef5c;
        }
        }
        }
        }
        c.cycles += 3;
        c.g[RA][0] = sx(0x002aef64);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
        call_ptr(c, 0x003b7f18, 0x002aef64)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        break 'b_2aef68;
        }
        c.cycles += 1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[S4][0]);
        break 'b_2aef6c;
        }
        c.cycles += 5;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[S5][0]);
        c.g[RA][0] = sx(0x002aef7c);
        c.sb((c.g[S0][0] as u32).wrapping_add(0x65), 0u64);
        call_ptr(c, 0x003b58d8, 0x002aef7c)?;
        c.g[V0][0] = 0u64;
        break 'b_2aef80;
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.pc = t; return Ok(());
    })
}

/// `int MusicPlayer_CreateStream(SNDPLAYOPTS* opts, a1, a2, a3, t0)` (0x2b1780): pushes a
/// new bank-monitor instance on the music system's bank manager (default play
/// options, zeroed per-instance fields, delay 100/90/50, max volume 127, close on
/// end), copies `*opts` into its level slot, creates stream 2 and returns its
/// handle. Straight-line transcription of the generated body.
#[allow(unused_labels, unused_assignments, unused_mut, unreachable_code, unused_variables)]
pub fn music_player_create_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 188;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6fc));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = 0x18;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.g[A3][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S4][0] = c.g[T0][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S5][0] = c.g[A2][0];
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S6][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S7][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x8;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.g[S0][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.mult(0, 3, 3, 18);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
        c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
        c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
        c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(2, (c.g[V1][0] as u32));
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
        c.mult(0, 4, 4, 18);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
        c.g[V1][0] = c.lb((c.g[A0][0] as u32));
        c.sw((c.g[A3][0] as u32), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[RA][0] = sx(0x002b1898);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x00416210, 0x002b1898)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = 0x64;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
        c.g[T3][0] = 0x5a;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[T4][0] = 0x32;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[T5][0] = 0x7f;
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[T6][0] = 0x1;
        c.g[A3][0] = c.g[S3][0];
        c.g[T0][0] = c.g[S4][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[T1][0] = c.g[S6][0];
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
        c.g[T2][0] = c.g[S5][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A2][0] = 0u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A1][0] = 0x2;
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[A0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T3][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[T4][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[T5][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), c.g[T6][0]);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6fc));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f8));
        c.mult(0, 2, 2, 18);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.ldl(11, (c.g[S7][0] as u32).wrapping_add(0x7));
        c.ldr(11, (c.g[S7][0] as u32));
        c.ldl(12, (c.g[S7][0] as u32).wrapping_add(0xf));
        c.ldr(12, (c.g[S7][0] as u32).wrapping_add(0x8));
        c.ldl(2, (c.g[S7][0] as u32).wrapping_add(0x17));
        c.ldr(2, (c.g[S7][0] as u32).wrapping_add(0x10));
        c.sdl(11, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(11, (c.g[V1][0] as u32));
        c.sdl(12, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.sdr(12, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x17));
        c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6fc));
        c.g[RA][0] = sx(0x002b1a20);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
        Callee { addr: 0x002aa870, f: ee::s08::StreamArray_CreateStreamEx }.call(c, 0x002b1a20)?;
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6fc));
        c.g[A1][0] = 0x2;
        c.g[V1][0] = c.lw((c.g[A2][0] as u32));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
        c.g[RA][0] = sx(0x002b1a44);
        c.g[A0][0] = c.lw((c.g[A2][0] as u32));
        Callee { addr: 0x002ab128, f: ee::s08::StreamArray_GetStreamHandle }.call(c, 0x002b1a44)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t; return Ok(());
    })
}

/// `void MusicPlayer_dtor(MusicPlayer* self, int flags)` (0x2b3398): resets the vtables of
/// the stream system's bank manager and sound system (rebased on `self` when
/// `flags == 0`), destroys the song configurations (virtual dtor, flag 3), frees
/// the MPF file and song instance, clears `gMusicSys`, then by `flags` runs the
/// sub-object destructors (bit 1) and `operator_delete` (bit 0). Transcription of
/// the generated body (labels kept).
#[allow(unused_labels, unused_assignments, unused_mut, unreachable_code, unused_variables)]
pub fn music_player_dtor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2b34bc: {
        c.cycles += 29;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
        c.g[V0][0] = 0x480000;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x38f8));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[A3][0] = 0x480000;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3928));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[A0][0] = 0x480000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b48));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5440), c.g[V0][0]);
        c.g[T0][0] = 0x480000;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.g[T5][0] = sx((c.g[T0][0] as u32).wrapping_add(0x3948));
        c.g[T6][0] = c.g[A1][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xab0), c.g[A0][0]);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        let t = c.g[T6][0] != 0u64;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[T5][0]);
        if t {
        break 'b_2b34bc;
        } else {
        c.cycles += 44;
        c.g[A1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x3928));
        c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
        c.sd((c.g[SP][0] as u32), c.g[A1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.g[T1][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x3948));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
        c.g[T2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x8));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[SP][0]);
        c.g[T3][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x10));
        c.g[A3][0] = c.lw((c.g[S2][0] as u32));
        c.g[T0][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = c.lw((c.g[A3][0] as u32));
        c.g[A2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
        c.g[V1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5448));
        c.g[A1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T1][0]);
        c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[T2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
        c.g[A0][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[T4][0]);
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5450));
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A0][0]);
        break 'b_2b34bc;
        }
        }
        'b_2b350c: {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
        c.g[S4][0] = c.g[T6][0] & 0x1;
        c.g[S1][0] = 0u64;
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = c.g[T6][0] & 0x2;
        if t {
        break 'b_2b350c;
        } else {
        c.cycles += 2;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
        'l_2b34d8: loop {
        'b_2b34f8: {
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32));
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = 0x3;
        if t {
        break 'b_2b34f8;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x002b34f8);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x002b34f8)?;
        break 'b_2b34f8;
        }
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        if t {
        if event_due(c) { deliver_event_here(c, 0x002b34d8); }
continue 'l_2b34d8;
        } else {
        break 'b_2b350c;
        }
        }
        }
        }
        'b_2b3524: {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5434));
        if c.g[A0][0] == 0u64 {
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x408));
        break 'b_2b3524;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x002b3520);
        call_ptr(c, 0x00317e98, 0x002b3520)?;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x408));
        break 'b_2b3524;
        }
        }
        'b_2b3534: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;

        if t {
        break 'b_2b3534;
        } else {
        c.cycles += 2;
        c.g[RA][0] = sx(0x002b3534);
        c.g[A1][0] = 0x3;
        Callee { addr: 0x002b1f78, f: ee::s08::SongInstance_dtor }.call(c, 0x002b3534)?;
        break 'b_2b3534;
        }
        }
        'b_2b356c: {
        c.cycles += 2;
        let t = c.g[S3][0] == 0u64;
        c.sw((c.g[GP][0] as u32).wrapping_add(0x6fc), 0u64);
        if t {
        break 'b_2b356c;
        } else {
        c.cycles += 12;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32));
        c.g[RA][0] = sx(0x002b3548);
        c.g[A1][0] = 0u64;
        Callee { addr: 0x002aa648, f: ee::s08::StreamArray_dtor }.call(c, 0x002b3548)?;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x002b3558);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
        call_ptr(c, 0x0028bb10, 0x002b3558)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.g[A1][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
        c.g[RA][0] = sx(0x002b356c);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
        Callee { addr: 0x002adee8, f: ee::s08::SoundManagerRef_dtor }.call(c, 0x002b356c)?;
        break 'b_2b356c;
        }
        }
        'b_2b3580: {
        c.cycles += 2;
        let t = c.g[S4][0] == 0u64;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        if t {
        break 'b_2b3580;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x002b357c);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x00317e50, 0x002b357c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        break 'b_2b3580;
        }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t; return Ok(());
    })
}

/// `void QuadTree_Compact(QuadTree* self, void** out, int* size, ctx)` (0x2c1da8,
/// `cQuadTree_exportTree`): allocates the node map and the output block, flattens the
/// tree into it, renumbers each node's children and byte-swaps the header and nodes
/// through the context's fix functions. Transcription of the generated body (labels
/// kept).
#[allow(unused_labels, unused_assignments, unused_mut, unreachable_code, unused_variables)]
pub fn quad_tree_compact(c: &mut Cpu) {
    guest(c, |c| {
        'b_2c1e54: {
        c.cycles += 33;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S0][0] = c.g[A2][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S1][0] = c.g[A3][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = c.g[A1][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A1][0] = 0x480000;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S5][0] = c.g[T2][0];
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S6][0] = c.g[A0][0];
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S7][0] = c.g[T1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[FP][0] = c.g[T0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5fb0));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28));
        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x002c1e1c);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        call_ptr(c, 0x00317e30, 0x002c1e1c)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S2][0] = c.g[V0][0];
        if t {
        break 'b_2c1e54;
        } else {
        c.cycles += 3;
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V0][0] = c.g[S4][0];
        'l_2c1e38: loop {
        c.cycles += 7;
        c.sw((c.g[V0][0] as u32), c.g[V1][0]);
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
        let t = c.g[S2][0] != 0u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        if t {
        if event_due(c) { deliver_event_here(c, 0x002c1e38); }
continue 'l_2c1e38;
        } else {
        break 'b_2c1e54;
        }
        }
        }
        }
        'b_2c1ff4: {
        c.cycles += 48;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
        c.g[A1][0] = 0x480000;
        c.g[A3][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5fc8));
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x002c1e70);
        c.g[S2][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002c1e70)?;
        c.sw((c.g[S3][0] as u32), c.g[V0][0]);
        c.g[A1][0] = 0xffffffffdead0000;
        c.g[A1][0] = c.g[A1][0] | 0xbeef;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002c1e88);
        c.g[A2][0] = c.lw((c.g[S0][0] as u32));
        call_ptr(c, 0x00416210, 0x002c1e88)?;
        c.g[S3][0] = c.lw((c.g[S3][0] as u32));
        c.g[A0][0] = c.g[S6][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = c.g[S4][0];
        c.ldl(2, (c.g[S6][0] as u32).wrapping_add(0x7));
        c.ldr(2, (c.g[S6][0] as u32));
        c.ldl(3, (c.g[S6][0] as u32).wrapping_add(0xf));
        c.ldr(3, (c.g[S6][0] as u32).wrapping_add(0x8));
        c.ldl(5, (c.g[S6][0] as u32).wrapping_add(0x17));
        c.ldr(5, (c.g[S6][0] as u32).wrapping_add(0x10));
        c.ldl(8, (c.g[S6][0] as u32).wrapping_add(0x1f));
        c.ldr(8, (c.g[S6][0] as u32).wrapping_add(0x18));
        c.sdl(2, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(2, (c.g[S3][0] as u32));
        c.sdl(3, (c.g[S3][0] as u32).wrapping_add(0xf));
        c.sdr(3, (c.g[S3][0] as u32).wrapping_add(0x8));
        c.sdl(5, (c.g[S3][0] as u32).wrapping_add(0x17));
        c.sdr(5, (c.g[S3][0] as u32).wrapping_add(0x10));
        c.sdl(8, (c.g[S3][0] as u32).wrapping_add(0x1f));
        c.sdr(8, (c.g[S3][0] as u32).wrapping_add(0x18));
        c.ldl(2, (c.g[S6][0] as u32).wrapping_add(0x27));
        c.ldr(2, (c.g[S6][0] as u32).wrapping_add(0x20));
        c.sdl(2, (c.g[S3][0] as u32).wrapping_add(0x27));
        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x28));
        c.sdr(2, (c.g[S3][0] as u32).wrapping_add(0x20));
        c.sw((c.g[SP][0] as u32), 0u64);
        c.g[A1][0] = c.g[S0][0];
        c.g[T0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
        c.g[T0][0] = sx((c.g[T0][0] as u32) << 3);
        c.g[RA][0] = sx(0x002c1f08);
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
        Callee { addr: 0x002c2088, f: ee::s08::QuadTree_FlattenNode }.call(c, 0x002c1f08)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32));
        if (c.g[V0][0] as i64) <= 0 {
        c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
        break 'b_2c1ff4;
        } else {
        c.cycles += 1;
        'l_2c1f18: loop {
        'b_2c1fdc: {
        'b_2c1fbc: {
        c.cycles += 4;
        c.g[T1][0] = c.lhu((c.g[S0][0] as u32));
        c.g[V0][0] = c.g[T1][0] & 0x1;
        let t = c.g[V0][0] == 0u64;
        c.g[T1][0] = sx((c.g[T1][0] as u32) >> 1);
        if t {
        break 'b_2c1fbc;
        } else {
        c.cycles += 37;
        c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[T1][0] = sx((c.g[T1][0] as u32) << 2);
        c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x6));
        c.g[A0][0] = sx((c.g[A0][0] as u32) >> 1);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
        c.g[A2][0] = c.lhu((c.g[A0][0] as u32));
        c.g[A1][0] = c.lhu((c.g[T1][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.lhu((c.g[V0][0] as u32));
        c.g[RA][0] = sx(0x002c1f78);
        c.g[T0][0] = c.lhu((c.g[V1][0] as u32));
        Callee { addr: 0x002c16b8, f: ee::s08::QuadTreeNode_SetChildren }.call(c, 0x002c1f78)?;
        c.g[A0][0] = c.lhu((c.g[S0][0] as u32));
        let t = c.g[S5][0] as u32;
        c.g[RA][0] = sx(0x002c1f84);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, t, 0x002c1f84)?;
        c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
        c.g[A1][0] = c.g[S1][0];
        let t = c.g[S5][0] as u32;
        c.g[RA][0] = sx(0x002c1f94);
        c.sh((c.g[S0][0] as u32), c.g[V0][0]);
        call_ptr(c, t, 0x002c1f94)?;
        c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = c.g[S1][0];
        let t = c.g[S5][0] as u32;
        c.g[RA][0] = sx(0x002c1fa4);
        c.sh((c.g[S0][0] as u32).wrapping_add(0x2), c.g[V0][0]);
        call_ptr(c, t, 0x002c1fa4)?;
        c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x6));
        c.g[A1][0] = c.g[S1][0];
        let t = c.g[S5][0] as u32;
        c.g[RA][0] = sx(0x002c1fb4);
        c.sh((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        call_ptr(c, t, 0x002c1fb4)?;
        let t = true;
        c.sh((c.g[S0][0] as u32).wrapping_add(0x6), c.g[V0][0]);
        if t {
        break 'b_2c1fdc;
        } else {
        break 'b_2c1fbc;
        }
        }
        }
        c.cycles += 8;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
        let t = c.g[S7][0] as u32;
        c.g[RA][0] = sx(0x002c1fc8);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, t, 0x002c1fc8)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = c.g[S1][0];
        let t = c.g[S7][0] as u32;
        c.g[RA][0] = sx(0x002c1fd8);
        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
        call_ptr(c, t, 0x002c1fd8)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        break 'b_2c1fdc;
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32));
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
        if t {
        if event_due(c) { deliver_event_here(c, 0x002c1f18); }
continue 'l_2c1f18;
        } else {
        c.cycles += 1;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
        break 'b_2c1ff4;
        }
        }
        }
        }
        c.cycles += 37;
        c.g[A1][0] = c.g[S1][0];
        c.sw((c.g[S3][0] as u32).wrapping_add(0x24), 0u64);
        let t = c.g[S7][0] as u32;
        c.g[RA][0] = sx(0x002c2004);
        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1c));
        call_ptr(c, t, 0x002c2004)?;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        c.g[A1][0] = c.g[S1][0];
        let t = c.g[S7][0] as u32;
        c.g[RA][0] = sx(0x002c2014);
        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x18));
        call_ptr(c, t, 0x002c2014)?;
        c.lwc1(12, (c.g[S3][0] as u32));
        c.g[A0][0] = c.g[S1][0];
        let t = c.g[FP][0] as u32;
        c.g[RA][0] = sx(0x002c2024);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        call_ptr(c, t, 0x002c2024)?;
        c.lwc1(12, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[S1][0];
        let t = c.g[FP][0] as u32;
        c.g[RA][0] = sx(0x002c2034);
        c.swc1(0, (c.g[S3][0] as u32));
        call_ptr(c, t, 0x002c2034)?;
        c.lwc1(12, (c.g[S3][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.g[S1][0];
        let t = c.g[FP][0] as u32;
        c.g[RA][0] = sx(0x002c2044);
        c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
        call_ptr(c, t, 0x002c2044)?;
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = c.g[S1][0];
        let t = c.g[S7][0] as u32;
        c.g[RA][0] = sx(0x002c2054);
        c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x8));
        call_ptr(c, t, 0x002c2054)?;
        c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t; return Ok(());
    })
}

/// `void Avalanche_WriteHistoryFile(bool direct, const char* name)` (0x2d83b8,
/// `cAvalanche::SaveData`): when the avalanche list exists, opens the course's
/// history file, writes the magic, the size and then every avalanche's nodes,
/// collisions and ground collisions, closes it and clears `cAvalanche_m_Save`.
/// Transcription of the generated body (labels kept).
#[allow(unused_labels, unused_assignments, unused_mut, unreachable_code, unused_variables)]
pub fn avalanche_write_history_file(c: &mut Cpu) {
    guest(c, |c| {
        'b_2d87ac: {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x180));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[S2][0] = c.g[A1][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.g[S4][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
        let t = c.g[V0][0] == 0u64;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
        if t {
        break 'b_2d87ac;
        } else {
        'b_2d87a8: {
        c.cycles += 25;
        c.g[RA][0] = sx(0x002d83f0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), 0u64);
        call_ptr(c, 0x0014dc80, 0x002d83f0)?;
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x002d83fc);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014dd58, 0x002d83fc)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[S0][0] = 0x4a0000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3af0));
        c.g[RA][0] = sx(0x002d8410);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00144be0, 0x002d8410)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x002d841c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x00144c48, 0x002d841c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x002d8430);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x00417828, 0x002d8430)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x26;
        c.g[A2][0] = 0x64;
        c.g[RA][0] = sx(0x002d8444);
        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x104));
        call_ptr(c, 0x003debf0, 0x002d8444)?;
        let t = c.g[V0][0] == 0u64;
        c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
        if t {
        break 'b_2d87a8;
        } else {
        'b_2d84f4: {
        c.cycles += 3;
        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x108));
        let t = c.g[S1][0] == 0u64;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10c));
        if t {
        break 'b_2d84f4;
        } else {
        c.cycles += 2;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        'l_2d8460: loop {
        'b_2d8494: {
        c.cycles += 2;
        if c.g[A0][0] == 0u64 {
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
        break 'b_2d8494;
        } else {
        c.cycles += 2;
        c.g[A1][0] = 0xa;
        'l_2d8470: loop {
        c.cycles += 8;
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x100));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xf4));
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.mult(0, 2, 2, 5);
        let t = c.g[A0][0] != 0u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        if t {
        if event_due(c) { deliver_event_here(c, 0x002d8470); }
continue 'l_2d8470;
        } else {
        c.cycles += 1;
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
        break 'b_2d8494;
        }
        }
        }
        }
        'b_2d84bc: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
        if t {
        break 'b_2d84bc;
        } else {
        c.cycles += 1;
        'l_2d84a0: loop {
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        let t = c.g[V0][0] != 0u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
        if t {
        if event_due(c) { deliver_event_here(c, 0x002d84a0); }
continue 'l_2d84a0;
        } else {
        break 'b_2d84bc;
        }
        }
        }
        }
        'b_2d84e8: {
        c.cycles += 3;
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xe));
        if c.g[V0][0] == 0u64 {
        c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        break 'b_2d84e8;
        } else {
        'l_2d84c8: loop {
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        let t = c.g[V0][0] != 0u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
        if t {
        if event_due(c) { deliver_event_here(c, 0x002d84c8); }
continue 'l_2d84c8;
        } else {
        c.cycles += 1;
        c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        break 'b_2d84e8;
        }
        }
        }
        }
        c.cycles += 2;
        if c.g[S1][0] != 0u64 {
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        if event_due(c) { deliver_event_here(c, 0x002d8460); }
continue 'l_2d8460;
        } else {
        c.cycles += 1;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
        break 'b_2d84f4;
        }
        }
        }
        }
        'b_2d879c: {
        c.cycles += 20;
        c.g[V0][0] = 0x2be0000;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        c.g[V0][0] = c.g[V0][0] | 0xef00;
        c.g[A0][0] = c.g[S4][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x108), c.g[V0][0]);
        c.g[A2][0] = 0u64;
        c.g[T0][0] = 0x4;
        c.g[RA][0] = sx(0x002d8518);
        c.g[T1][0] = 0x1;
        Callee { addr: 0x002d82a0, f: ee::s09::Avalanche_WriteData }.call(c, 0x002d8518)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = 0x4;
        c.g[T0][0] = 0x4;
        c.g[RA][0] = sx(0x002d8534);
        c.g[T1][0] = 0x1;
        Callee { addr: 0x002d82a0, f: ee::s09::Avalanche_WriteData }.call(c, 0x002d8534)?;
        c.g[V1][0] = 0x8;
        c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
        let t = c.g[S1][0] == 0u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
        if t {
        break 'b_2d879c;
        } else {
        c.cycles += 3;
        c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        'l_2d8550: loop {
        'b_2d8630: {
        c.cycles += 2;
        if c.g[S0][0] == 0u64 {
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
        break 'b_2d8630;
        } else {
        c.cycles += 2;
        c.g[S2][0] = c.g[S5][0];
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        'l_2d8560: loop {
        c.cycles += 51;
        c.g[V0][0] = 0xbeef;
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[A0][0] = c.g[S4][0];
        c.sh((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = 0x2;
        c.g[RA][0] = sx(0x002d8580);
        c.g[T1][0] = 0x1;
        Callee { addr: 0x002d82a0, f: ee::s09::Avalanche_WriteData }.call(c, 0x002d8580)?;
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[V1][0] = 0xa;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A3][0] = c.g[S2][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[A2][0]);
        c.g[T0][0] = 0x2;
        c.g[T1][0] = 0x1;
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x100));
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.mult(0, 2, 2, 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[RA][0] = sx(0x002d85c0);
        c.sh((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
        Callee { addr: 0x002d82a0, f: ee::s09::Avalanche_WriteData }.call(c, 0x002d85c0)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe0));
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        c.g[A0][0] = c.g[S4][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[T0][0] = 0x4;
        c.g[A2][0] = c.g[V1][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
        c.g[RA][0] = sx(0x002d85e8);
        c.g[T1][0] = 0x3;
        Callee { addr: 0x002d82a0, f: ee::s09::Avalanche_WriteData }.call(c, 0x002d85e8)?;
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[A0][0] = 0x1;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        c.g[T0][0] = 0xa;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[A2][0]);
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x100));
        c.cvt_w_s(1, 0);
        c.mfc1(9, 1);
        c.g[RA][0] = sx(0x002d8614);
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xfc));
        Callee { addr: 0x002d82a0, f: ee::s09::Avalanche_WriteData }.call(c, 0x002d8614)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf4));
        let t = c.g[S0][0] != 0u64;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        if t {
        if event_due(c) { deliver_event_here(c, 0x002d8560); }
continue 'l_2d8560;
        } else {
        c.cycles += 1;
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
        break 'b_2d8630;
        }
        }
        }
        }
        'b_2d8714: {
        c.cycles += 26;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        c.g[A3][0] = c.g[S5][0];
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[T0][0] = 0x2;
        c.sh((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
        c.g[RA][0] = sx(0x002d8650);
        c.g[T1][0] = 0x1;
        Callee { addr: 0x002d82a0, f: ee::s09::Avalanche_WriteData }.call(c, 0x002d8650)?;
        c.g[S3][0] = 0u64;
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        c.g[A3][0] = c.g[S5][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[T0][0] = 0x2;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[A2][0]);
        c.g[T1][0] = 0x1;
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xe));
        c.g[RA][0] = sx(0x002d8680);
        c.sh((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
        Callee { addr: 0x002d82a0, f: ee::s09::Avalanche_WriteData }.call(c, 0x002d8680)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
        if c.g[V0][0] == 0u64 {
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xe));
        break 'b_2d8714;
        } else {
        c.cycles += 4;
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        'l_2d86a8: loop {
        c.cycles += 26;
        c.g[A3][0] = c.g[S0][0];
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[A0][0] = c.g[S4][0];
        c.g[T0][0] = 0x4;
        c.g[RA][0] = sx(0x002d86c0);
        c.g[T1][0] = 0x1;
        Callee { addr: 0x002d82a0, f: ee::s09::Avalanche_WriteData }.call(c, 0x002d86c0)?;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        c.g[T0][0] = 0x4;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[T1][0] = 0x1;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[A2][0]);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A3][0] = c.lw((c.g[S2][0] as u32));
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
        c.g[RA][0] = sx(0x002d86f4);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x78));
        Callee { addr: 0x002d82a0, f: ee::s09::Avalanche_WriteData }.call(c, 0x002d86f4)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        if t {
        if event_due(c) { deliver_event_here(c, 0x002d86a8); }
continue 'l_2d86a8;
        } else {
        c.cycles += 1;
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xe));
        break 'b_2d8714;
        }
        }
        }
        }
        'b_2d8790: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.g[S3][0] = 0u64;
        if t {
        break 'b_2d8790;
        } else {
        c.cycles += 3;
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x112));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x110));
        'l_2d8728: loop {
        c.cycles += 26;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        c.g[A3][0] = c.g[S2][0];
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[A0][0] = c.g[S4][0];
        c.g[T0][0] = 0x2;
        c.g[RA][0] = sx(0x002d8744);
        c.g[T1][0] = 0x1;
        Callee { addr: 0x002d82a0, f: ee::s09::Avalanche_WriteData }.call(c, 0x002d8744)?;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        c.g[A0][0] = c.g[S4][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[T0][0] = 0x2;
        c.g[A2][0] = c.g[V1][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
        c.g[T1][0] = 0x1;
        c.g[RA][0] = sx(0x002d8774);
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
        Callee { addr: 0x002d82a0, f: ee::s09::Avalanche_WriteData }.call(c, 0x002d8774)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xe));
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        if t {
        if event_due(c) { deliver_event_here(c, 0x002d8728); }
continue 'l_2d8728;
        } else {
        break 'b_2d8790;
        }
        }
        }
        }
        c.cycles += 3;
        c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        if c.g[S1][0] != 0u64 {
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        if event_due(c) { deliver_event_here(c, 0x002d8550); }
continue 'l_2d8550;
        } else {
        break 'b_2d879c;
        }
        }
        }
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
        c.g[RA][0] = sx(0x002d87a8);
        c.g[A1][0] = 0x64;
        call_ptr(c, 0x003deca0, 0x002d87a8)?;
        break 'b_2d87a8;
        }
        }
        c.cycles += 1;
        c.sw((c.g[GP][0] as u32).wrapping_add(0x958), 0u64);
        break 'b_2d87ac;
        }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x180));
        c.pc = t; return Ok(());
    })
}
