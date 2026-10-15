//! Wire formats between the EE SND library and its IOP driver
//! (`docs/investigation/phase3-sound.md` §1, §2, §4, §5).

/// RPC server id `'SND'`.
pub const SND_RPC_ID: u32 = 0x0053_4E44;
/// SNDDRV version the EE checks in the init block (8.3.3).
pub const SND_VERSION: u32 = 0x0008_0303;
/// Size of the status block sent to the EE every tick.
pub const STATUS_LEN: usize = 0x240;
/// Silent self-loop the driver uploads at init; every voice idles there.
pub const SAFE_LOOP_ADDR: u32 = 0x5000;
/// A voice whose next address is below this has finished (status pos 0).
pub const SAFE_LOOP_END: u32 = 0x5040;
/// Size of the command block (tag 3 payload).
pub const COMMAND_LEN: usize = 0x2a0;
pub const SPU_VOICES: usize = 48;
pub const IOP_VOICES: usize = 16;

fn u16_at(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn u32_at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

/// RPC fno 0 payload (0x14 bytes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitParams {
    pub version: u32,
    /// EE address the status block is written to.
    pub status_ee: u32,
    /// EE address for DTS PCM (0 unless DTS).
    pub dts_ee: u32,
    pub ee_rate: u16,
    pub iop_rate: u16,
    pub channels: u8,
    pub aux: u8,
    pub iop_voices: u8,
    pub spu_fx_to_mixer: u8,
}

impl InitParams {
    pub fn parse(b: &[u8]) -> Option<Self> {
        (b.len() >= 0x14).then(|| Self {
            version: u32_at(b, 0),
            status_ee: u32_at(b, 4),
            dts_ee: u32_at(b, 8),
            ee_rate: u16_at(b, 0xC),
            iop_rate: u16_at(b, 0xE),
            channels: b[0x10],
            aux: b[0x11],
            iop_voices: b[0x12],
            spu_fx_to_mixer: b[0x13],
        })
    }

    /// EE PCM frames per tick (`b39c`: rate * 512 / 48000).
    pub fn ee_frames(&self) -> usize {
        self.ee_rate as usize * 512 / 48000
    }

    /// IOP mixer frames per tick (`b39e`).
    pub fn iop_frames(&self) -> usize {
        self.iop_rate as usize * 512 / 48000
    }
}

/// One SPU voice record of the command block (8 bytes).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SpuVoiceCmd {
    /// Bits 0..22: SPU byte start address (0 = stop); bits 23..31: start
    /// counter, so a restart of the same address still changes the word.
    pub word: u32,
    /// SPU pitch (0x1000 = 48 kHz).
    pub pitch: u16,
    pub vol_l: i8,
    pub vol_r: i8,
}

/// One IOP software voice record (0x10 bytes).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct IopVoiceCmd {
    /// Bits 0..22: IOP address of the sample header (0 = stop).
    pub word: u32,
    pub pitch: u16,
    /// Dry gain per output channel (0 = R, 1 = L).
    pub dry: [i8; 6],
    pub wet: i8,
}

/// The command block (tag 3 payload).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandBlock {
    pub wet_bits: u64,
    pub dry_bits: u64,
    pub rev_mode: i8,
    pub rev_delay: i8,
    pub rev_feedback: i8,
    pub resample_quality: u8,
    pub evol: [i16; 2],
    pub spu: [SpuVoiceCmd; SPU_VOICES],
    pub iop: [IopVoiceCmd; IOP_VOICES],
}

impl CommandBlock {
    pub fn parse(b: &[u8]) -> Option<Self> {
        if b.len() < COMMAND_LEN {
            return None;
        }
        let mut spu = [SpuVoiceCmd::default(); SPU_VOICES];
        for (v, s) in spu.iter_mut().enumerate() {
            let o = 0x20 + v * 8;
            *s = SpuVoiceCmd { word: u32_at(b, o), pitch: u16_at(b, o + 4), vol_l: b[o + 6] as i8, vol_r: b[o + 7] as i8 };
        }
        let mut iop = [IopVoiceCmd::default(); IOP_VOICES];
        for (i, s) in iop.iter_mut().enumerate() {
            let o = 0x1a0 + i * 0x10;
            let mut dry = [0i8; 6];
            for (k, d) in dry.iter_mut().enumerate() {
                *d = b[o + 6 + k] as i8;
            }
            *s = IopVoiceCmd { word: u32_at(b, o), pitch: u16_at(b, o + 4), dry, wet: b[o + 0xC] as i8 };
        }
        Some(Self {
            wet_bits: u32_at(b, 0) as u64 | (u32_at(b, 4) as u64) << 32,
            dry_bits: u32_at(b, 8) as u64 | (u32_at(b, 0xC) as u64) << 32,
            rev_mode: b[0x10] as i8,
            rev_delay: b[0x11] as i8,
            rev_feedback: b[0x12] as i8,
            resample_quality: b[0x13],
            evol: [u16_at(b, 0x14) as i16, u16_at(b, 0x16) as i16],
            spu,
            iop,
        })
    }
}

/// What a tag buffer carries for one tick.
#[derive(Clone, Debug, Default)]
pub struct TagBuffer {
    /// Tag 0 / tag 5 sequence (the EE's `SND_Service` tick count).
    pub seq: u32,
    pub command: Option<CommandBlock>,
    /// Tag 1: planar s16 EE PCM, plane-major (plane 0 = R, plane 1 = L).
    pub pcm: Vec<i16>,
    /// Tag 2: EE wet buffer (absent in SSX 3).
    pub ee_wet: Option<Vec<u8>>,
}

/// Why a tag buffer was not used.
#[derive(Debug, PartialEq, Eq)]
pub enum TagError {
    /// Header (tag 0) and trailer (tag 5) sequence differ: the EE was still
    /// writing it.
    Torn,
    Malformed(usize),
}

/// Parse a tag buffer as `SNDIOP_copytagbuf` + `SNDIOP_processtagbuf` do.
pub fn parse_tags(b: &[u8]) -> Result<TagBuffer, TagError> {
    let mut out = TagBuffer::default();
    let mut o = 0usize;
    let mut trailer = None;
    loop {
        if o + 0x10 > b.len() {
            return Err(TagError::Malformed(o));
        }
        match u32_at(b, o) {
            0 => {
                out.seq = u32_at(b, o + 4);
                o += 0x10;
            }
            1 => {
                let bytes = u16_at(b, o + 4) as usize;
                let d = o + 0x10;
                if d + bytes > b.len() {
                    return Err(TagError::Malformed(o));
                }
                out.pcm = b[d..d + bytes].chunks(2).map(|h| i16::from_le_bytes([h[0], h[1]])).collect();
                o = d + bytes;
            }
            2 => {
                // Size known only from the init block; SSX 3 never sends it.
                return Err(TagError::Malformed(o));
            }
            3 => {
                let size = u32_at(b, o + 4) as usize;
                out.command = CommandBlock::parse(&b[o + 8..]);
                o += 8 + size;
            }
            4 => o += 0x10 + 0x800,
            5 => {
                trailer = Some(u32_at(b, o + 4));
                o += 0x10;
            }
            6 => break,
            _ => return Err(TagError::Malformed(o)),
        }
    }
    if trailer != Some(out.seq) {
        return Err(TagError::Torn);
    }
    Ok(out)
}

/// The status block (`ioptoee`), 0x240 bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusBlock {
    pub seq: u32,
    /// Per SPU voice: (echo of the command word, position).
    pub spu: [(u32, u32); SPU_VOICES],
    /// Per IOP voice: (echo, decoder frame or -1).
    pub iop: [(u32, i32); IOP_VOICES],
}

impl Default for StatusBlock {
    fn default() -> Self {
        Self { seq: 0, spu: [(0, 0); SPU_VOICES], iop: [(0, -1); IOP_VOICES] }
    }
}

impl StatusBlock {
    pub fn to_bytes(&self) -> [u8; STATUS_LEN] {
        let mut b = [0u8; STATUS_LEN];
        b[0..4].copy_from_slice(&self.seq.to_le_bytes());
        for (v, (e, p)) in self.spu.iter().enumerate() {
            b[0x3c + v * 8..0x40 + v * 8].copy_from_slice(&e.to_le_bytes());
            b[0x40 + v * 8..0x44 + v * 8].copy_from_slice(&p.to_le_bytes());
        }
        for (i, (e, f)) in self.iop.iter().enumerate() {
            b[0x1bc + i * 8..0x1c0 + i * 8].copy_from_slice(&e.to_le_bytes());
            b[0x1c0 + i * 8..0x1c4 + i * 8].copy_from_slice(&f.to_le_bytes());
        }
        b[0x23c..0x240].copy_from_slice(&self.seq.to_le_bytes());
        b
    }
}

/// SIF user command 0, type 1: upload one piece of sample data to SPU RAM.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UploadCmd {
    /// Token sent back with the completion (0 = none).
    pub token: u32,
    /// IOP staging address of the data.
    pub iop_src: u32,
    /// SPU byte address.
    pub spu_dst: u32,
    /// Bytes, rounded up to 0x40.
    pub size: u32,
    pub prio: u16,
}

impl UploadCmd {
    /// From the command packet (0x30 bytes, header included).
    pub fn parse(p: &[u8]) -> Option<Self> {
        (p.len() >= 0x24 && u32_at(p, 0x10) == 1).then(|| Self {
            token: u32_at(p, 0x14),
            iop_src: u32_at(p, 0x18),
            spu_dst: u32_at(p, 0x1c),
            size: (u16_at(p, 0x20) as u32 + 0x3f) & !0x3f,
            prio: u16_at(p, 0x22),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tagbuf(seq: u32, trailer: u32) -> Vec<u8> {
        let mut b = vec![0u8; 0x8f0];
        b[4..8].copy_from_slice(&seq.to_le_bytes());
        b[8..12].copy_from_slice(&0x8f0u32.to_le_bytes());
        b[0x10..0x14].copy_from_slice(&3u32.to_le_bytes());
        b[0x14..0x18].copy_from_slice(&0x2a8u32.to_le_bytes());
        b[0x18 + 0x20..0x18 + 0x24].copy_from_slice(&0x0080_5040u32.to_le_bytes());
        b[0x2c0..0x2c4].copy_from_slice(&1u32.to_le_bytes());
        b[0x2c4..0x2c6].copy_from_slice(&0x600u16.to_le_bytes());
        b[0x2d0..0x2d2].copy_from_slice(&123i16.to_le_bytes());
        b[0x8d0..0x8d4].copy_from_slice(&5u32.to_le_bytes());
        b[0x8d4..0x8d8].copy_from_slice(&trailer.to_le_bytes());
        b[0x8e0..0x8e4].copy_from_slice(&6u32.to_le_bytes());
        b
    }

    #[test]
    fn parses_the_ssx3_layout() {
        let t = parse_tags(&tagbuf(7, 7)).unwrap();
        assert_eq!(t.seq, 7);
        assert_eq!(t.pcm.len(), 0x300);
        assert_eq!(t.pcm[0], 123);
        assert_eq!(t.command.unwrap().spu[0].word, 0x0080_5040);
    }

    #[test]
    fn torn_buffer_is_rejected() {
        assert_eq!(parse_tags(&tagbuf(7, 6)).unwrap_err(), TagError::Torn);
    }

    #[test]
    fn status_layout() {
        let mut s = StatusBlock { seq: 3, ..Default::default() };
        s.spu[1] = (0xAA, 0x5040);
        s.iop[0] = (0xBB, -1);
        let b = s.to_bytes();
        assert_eq!(u32_at(&b, 0x44), 0xAA);
        assert_eq!(u32_at(&b, 0x48), 0x5040);
        assert_eq!(u32_at(&b, 0x1bc), 0xBB);
        assert_eq!(u32_at(&b, 0x1c0), u32::MAX);
        assert_eq!(u32_at(&b, 0x23c), 3);
    }
}
