//! The card's error-correcting code: 3 bytes per 128-byte chunk of a page,
//! four chunks per 512-byte page, kept in the page's 16 spare bytes
//! (12 ECC bytes, then 4 zero bytes). A Hamming code: column parities of
//! the bits (in bits 0-2 and 4-6), line parities of the byte indices.
//! Checked against every written page of a card MCMAN wrote.

const fn parity(b: u8) -> bool {
    b.count_ones() & 1 != 0
}

const fn column_mask(b: u8) -> u8 {
    let mut m = 0;
    if parity(b & 0x55) {
        m |= 0x01;
    }
    if parity(b & 0x33) {
        m |= 0x02;
    }
    if parity(b & 0x0f) {
        m |= 0x04;
    }
    if parity(b & 0xaa) {
        m |= 0x10;
    }
    if parity(b & 0xcc) {
        m |= 0x20;
    }
    if parity(b & 0xf0) {
        m |= 0x40;
    }
    m
}

const COLUMN: [u8; 256] = {
    let mut t = [0; 256];
    let mut i = 0;
    while i < 256 {
        t[i] = column_mask(i as u8);
        i += 1;
    }
    t
};

/// ECC of one 128-byte chunk.
pub fn chunk(s: &[u8]) -> [u8; 3] {
    let (mut col, mut l0, mut l1) = (0x77u8, 0x7fu8, 0x7fu8);
    for (i, &b) in s.iter().enumerate() {
        col ^= COLUMN[b as usize];
        if parity(b) {
            l0 ^= !(i as u8);
            l1 ^= i as u8;
        }
    }
    [col, l0 & 0x7f, l1]
}

/// The 16 spare bytes of a 512-byte page.
pub fn spare(page: &[u8]) -> [u8; 16] {
    let mut out = [0; 16];
    for (k, c) in page.chunks(128).enumerate().take(4) {
        out[k * 3..k * 3 + 3].copy_from_slice(&chunk(c));
    }
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn zero_chunk() {
        assert_eq!(super::chunk(&[0; 128]), [0x77, 0x7f, 0x7f]);
    }
}
