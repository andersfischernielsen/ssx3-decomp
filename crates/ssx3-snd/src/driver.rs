//! SNDDRV's per-tick logic in plain Rust (`docs/investigation/phase3-sound.md`).
//!
//! One [`SndDriver::tick`] is one SNDDRV tick: one AutoDMA half of 512 output
//! frames at 48 kHz (every 3,145,728 EE cycles). It does what `threadmain`
//! does after the AutoDMA interrupt:
//! 1. take the tag buffer the EE last sent (`SNDIOP_copytagbuf`);
//! 2. apply the command block to the SPU voices and the IOP software voices
//!    (`SNDIOP_updatevoices`, `SNDIOP_updateiopvoices`), reading each voice's
//!    next address (NAX) for the status block;
//! 3. mix the EE PCM and the IOP voices into the idle AutoDMA half
//!    (`SNDIOP_mix`);
//! 4. build the status block the EE reads.
//!
//! Then it renders the half on the SPU2 model, which the real hardware does
//! while the next tick is pending. The half written in step 3 plays in the
//! *next* tick's period, as on hardware.
//!
//! SPU access goes through LIBSD-style helpers that map to the SPU2 register
//! file (`ssx3_iop::spu2::Spu2::write_reg16` / `read`).

use ssx3_iop::spu2::{Spu2, BATCH};

use crate::iopmix::IopMixer;
use crate::output::Resampler;
use crate::proto::*;

/// `volramplut` (0x3abd8): sweep rate by half the volume distance.
const VOL_RAMP: [u8; 128] = [
    0, 55, 51, 49, 47, 46, 45, 44, 43, 43, 42, 42, 41, 40, 40, 40, 39, 39, 39, 39, 37, 37, 37, 37, 37, 37, 37, 37, 36, 36,
    36, 36, 35, 35, 35, 35, 35, 35, 35, 35, 34, 34, 34, 34, 34, 34, 34, 34, 33, 33, 33, 33, 33, 33, 33, 33, 32, 32, 32, 32,
    32, 32, 32, 32, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28,
];

/// `sndiopsafeloop` (0x3ac58): four silent PS-ADPCM blocks with flags 7
/// (loop start, repeat, end), uploaded to [`SAFE_LOOP_ADDR`].
fn safe_loop() -> [u8; 0x40] {
    let mut b = [0u8; 0x40];
    for blk in 0..4 {
        b[blk * 16 + 1] = 7;
    }
    b
}

/// Output frames per tick (one AutoDMA half).
pub const TICK_FRAMES: usize = 512;
/// EE cycles per tick: 512 frames at 48 kHz on a 294.912 MHz EE.
pub const TICK_EE_CYCLES: u64 = 3_145_728;

// SPU2 register offsets (per core: + 0x400 for core 1).
const VOLL: u32 = 0x0;
const VOLR: u32 = 0x2;
const PITCH: u32 = 0x4;
const ADSR1: u32 = 0x6;
const ADSR2: u32 = 0x8;
const ENVX: u32 = 0xA;
const VOLXL: u32 = 0xC;
const SSA: u32 = 0x0;
const LSAX: u32 = 0x4;
const NAX: u32 = 0x8;
const KON: u32 = 0x1A0;
const KOFF: u32 = 0x1A4;
/// Master volumes: 0x760 + core * 0x28 + {MVOL 0, EVOL 4, AVOL 8, BVOL 0xC}.
const MASTER: u32 = 0x760;

/// `vfix`: an s8 volume as a fixed 15-bit SPU volume (127 → 0x3FFF).
fn vfix(b: i8) -> u16 {
    if b < 0 {
        (b as u8 as u16) << 7
    } else {
        b as u16 * 0x81
    }
}

/// SNDDRV's cache of what it last applied to an SPU voice (`0x3ae18`).
#[derive(Clone, Copy, Default)]
struct SpuCache {
    word: u32,
    pitch: u16,
    vol: [i8; 2],
}

/// Per IOP software voice (`0x3af98`).
#[derive(Clone, Copy, Default)]
struct IopCache {
    word: u32,
    pitch: u16,
    dry: [i8; 6],
    wet: i8,
}

/// What one tick produces.
pub struct TickOutput {
    /// Status block for the EE (`StatusBlock::to_bytes` at the init block's
    /// `status_ee`), followed by SIF command 1 type 0.
    pub status: StatusBlock,
    /// 512 interleaved stereo frames at 48 kHz.
    pub audio: Vec<i16>,
}

pub struct SndDriver {
    pub spu: Spu2,
    pub cfg: Option<InitParams>,
    mix: IopMixer,
    out: Resampler,
    spu_cache: [SpuCache; SPU_VOICES],
    /// Volume sweep still running (`0x3b338`): re-evaluated every tick.
    ramp: [[bool; 2]; SPU_VOICES],
    iop_cache: [IopCache; IOP_VOICES],
    iop_active: [bool; IOP_VOICES],
    status: StatusBlock,
    wet_bits: u64,
    dry_bits: u64,
    quality: u8,
    /// `updatevoices` count (`0x3b2f4`): BVOL opens at the third.
    updates: u32,
    seq: u32,
    buses: [Vec<i32>; 2],
    /// Mixed in the previous tick; plays during this one.
    pending: Vec<[i16; 2]>,
}

impl Default for SndDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl SndDriver {
    pub fn new() -> Self {
        let mut spu = Spu2::new();
        spu.trace = false;
        Self {
            spu,
            cfg: None,
            mix: IopMixer::new(IOP_VOICES, 2),
            out: Resampler::new(),
            spu_cache: [SpuCache::default(); SPU_VOICES],
            ramp: [[false; 2]; SPU_VOICES],
            iop_cache: [IopCache::default(); IOP_VOICES],
            iop_active: [false; IOP_VOICES],
            status: StatusBlock::default(),
            wet_bits: 0,
            dry_bits: 0xFFFF_FFFF_FFFF,
            quality: 0,
            updates: 0,
            seq: 0,
            buses: [vec![0; 512], vec![0; 512]],
            pending: vec![[0, 0]; TICK_FRAMES],
        }
    }

    // ---- LIBSD-style SPU access (voice index 0..47; 24.. = core 1) ----

    fn voice_base(v: usize) -> u32 {
        (v / 24) as u32 * 0x400 + (v % 24) as u32 * 0x10
    }
    fn addr_base(v: usize) -> u32 {
        (v / 24) as u32 * 0x400 + 0x1C0 + (v % 24) as u32 * 12
    }
    fn set_param(&mut self, v: usize, reg: u32, value: u16) {
        self.spu.write_reg16(Self::voice_base(v) + reg, value as u32);
    }
    fn get_param(&mut self, v: usize, reg: u32) -> u16 {
        self.spu.read(Self::voice_base(v) + reg) as u16
    }
    /// `sceSdSetAddr` with a byte address.
    fn set_addr(&mut self, v: usize, reg: u32, byte_addr: u32) {
        let w = byte_addr >> 1;
        let base = Self::addr_base(v) + reg;
        self.spu.write_reg16(base, w >> 16);
        self.spu.write_reg16(base + 2, w & 0xFFFF);
    }
    /// `sceSdGetAddr` (byte address).
    fn get_addr(&mut self, v: usize, reg: u32) -> u32 {
        let base = Self::addr_base(v) + reg;
        let hi = self.spu.read(base) & 0xFFFF;
        let lo = self.spu.read(base + 2) & 0xFFFF;
        (hi << 17) | (lo << 1)
    }
    /// `sceSdSetSwitch` for a 48-voice mask: core 0 = bits 0..23, core 1 =
    /// bits 24..47 (`SNDIOP_keyon` / `keyoff`).
    fn set_switch(&mut self, reg: u32, mask: u64) {
        for core in 0..2u32 {
            let m = (mask >> (core * 24)) as u32 & 0xFF_FFFF;
            if m != 0 {
                self.spu.write_reg16(core * 0x400 + reg, m & 0xFFFF);
                self.spu.write_reg16(core * 0x400 + reg + 2, m >> 16);
            }
        }
    }
    fn set_master(&mut self, core: u32, reg: u32, lr: [u16; 2]) {
        self.spu.write_reg16(MASTER + core * 0x28 + reg, lr[0] as u32);
        self.spu.write_reg16(MASTER + core * 0x28 + reg + 2, lr[1] as u32);
    }

    /// RPC fno 0: `SNDIOP_init` (stereo path).
    pub fn init(&mut self, p: InitParams) {
        self.cfg = Some(p);
        self.mix = IopMixer::new(p.iop_voices as usize, (p.channels + p.aux) as usize);
        self.buses = [vec![0; p.iop_frames().max(1)], vec![0; p.iop_frames().max(1)]];
        for core in 0..2 {
            self.set_master(core, 0x0, [0x3FFF, 0x3FFF]);
            self.set_master(core, 0x4, [0, 0]);
        }
        self.set_master(0, 0xC, [0, 0]);
        // MMIX core 0 / 1 (input dry only); VMIX: the model mixes every voice dry.
        self.spu.write_reg16(0x198, 0x0FC0);
        self.spu.write_reg16(0x400 + 0x198, 0x0FCC);
        self.spu.dma_write(SAFE_LOOP_ADDR, &safe_loop());
        for v in 0..SPU_VOICES {
            self.set_param(v, PITCH, 0);
            self.set_addr(v, SSA, SAFE_LOOP_ADDR);
            self.set_addr(v, LSAX, SAFE_LOOP_ADDR);
            self.set_param(v, ADSR1, 0x000F);
            self.set_param(v, ADSR2, 0x0005);
            self.set_param(v, VOLL, 0);
            self.set_param(v, VOLR, 0);
        }
        self.set_switch(KON, (1u64 << SPU_VOICES) - 1);
    }

    /// SIF user command 0 type 1: one piece of a bank to SPU RAM.
    pub fn upload(&mut self, cmd: &UploadCmd, iop_ram: &[u8]) {
        let src = (cmd.iop_src & 0x1F_FFFF) as usize;
        let end = (src + cmd.size as usize).min(iop_ram.len());
        self.spu.dma_write(cmd.spu_dst, &iop_ram[src..end]);
    }

    /// One SNDDRV tick. `tag` is the EE's last tag buffer (the receive area
    /// in IOP RAM: replayed when the EE has not sent a new one, as SNDDRV
    /// does); `iop_ram` holds the IOP-RAM sample banks.
    pub fn tick(&mut self, tag: &[u8], iop_ram: &[u8]) -> TickOutput {
        if let (Some(cfg), Ok(t)) = (self.cfg, parse_tags(tag)) {
            if let Some(cmd) = &t.command {
                self.update_voices(cmd, iop_ram);
            }
            if !t.pcm.is_empty() {
                self.mix_half(cfg, &t.pcm, iop_ram);
            }
        }
        self.status.seq = self.seq;
        self.seq = self.seq.wrapping_add(1);
        let status = self.status.clone();
        // The half mixed last tick plays now.
        let half = std::mem::replace(&mut self.pending, Vec::new());
        let mut audio = Vec::with_capacity(TICK_FRAMES * 2);
        for chunk in half.chunks(BATCH) {
            self.spu.push_input(0, chunk);
            audio.extend_from_slice(&self.spu.render_next());
        }
        if self.pending.is_empty() {
            self.pending = vec![[0, 0]; TICK_FRAMES];
        }
        TickOutput { status, audio }
    }

    /// `SNDIOP_spusetvol`: move a voice volume towards the command's target,
    /// fixed when close, otherwise with a linear sweep.
    fn spu_set_vol(&mut self, v: usize, side: usize, target: i8) {
        let volx = self.get_param(v, VOLXL + side as u32 * 2);
        let cur = (volx >> 8) as u8 as i8;
        self.spu_cache[v].vol[side] = cur;
        let d = (target as i32 - cur as i32).abs();
        let reg = if side == 0 { VOLL } else { VOLR };
        if d >> 1 < 2 {
            self.set_param(v, reg, vfix(target));
            self.spu_cache[v].vol[side] = target;
            self.ramp[v][side] = false;
        } else {
            let c16 = volx as i16 as i32;
            let t16 = ((target as u8 as i32) << 24) >> 16;
            let rate = VOL_RAMP[(d >> 1) as usize] as u16;
            let value = if c16 < t16 {
                if c16 < 0 {
                    if t16 >= 0 {
                        0xB01B
                    } else {
                        0xB000 | rate
                    }
                } else {
                    0x8000 | rate
                }
            } else if c16 < 1 {
                0x9000 | rate
            } else if t16 < 1 {
                0xA01B
            } else {
                0xA000 | rate
            };
            self.set_param(v, reg, value);
            self.ramp[v][side] = true;
        }
    }

    /// `SNDIOP_updatevoices` (0x37d1c).
    fn update_voices(&mut self, cmd: &CommandBlock, iop_ram: &[u8]) {
        self.updates += 1;
        if self.updates == 3 {
            self.set_master(0, 0xC, [0x7FFF, 0x7FFF]);
        }
        // SNDIOP_checkspueffect: reverb stays off in SSX 3 (mode 0); the SPU2
        // model has no effect unit, so EVOL/ESA are not modelled here.
        // Wet/dry bus bits: cached only (the model mixes every voice dry).
        self.dry_bits = cmd.dry_bits;
        self.wet_bits = cmd.wet_bits;
        let (mut kon, mut koff) = (0u64, 0u64);
        for v in 0..SPU_VOICES {
            let c = cmd.spu[v];
            let mut keyed = false;
            let mut started = 0;
            if c.pitch != self.spu_cache[v].pitch {
                self.set_param(v, PITCH, c.pitch);
                self.spu_cache[v].pitch = c.pitch;
            }
            if c.vol_l != self.spu_cache[v].vol[0] || self.ramp[v][0] {
                self.spu_set_vol(v, 0, c.vol_l);
            }
            if c.vol_r != self.spu_cache[v].vol[1] || self.ramp[v][1] {
                self.spu_set_vol(v, 1, c.vol_r);
            }
            if c.word != self.spu_cache[v].word {
                let a = c.word & 0x7F_FFFF;
                started = a;
                if a == 0 {
                    koff |= 1 << v;
                    self.spu_cache[v].word = c.word;
                } else if self.get_param(v, ENVX) == 0 {
                    self.set_addr(v, SSA, a);
                    self.set_addr(v, LSAX, SAFE_LOOP_ADDR);
                    self.set_param(v, ADSR1, 0x000F);
                    self.set_param(v, ADSR2, 0x0005);
                    self.set_param(v, VOLL, vfix(c.vol_l));
                    self.spu_cache[v].vol[0] = c.vol_l;
                    self.ramp[v][0] = false;
                    keyed = true;
                    self.set_param(v, VOLR, vfix(c.vol_r));
                    kon |= 1 << v;
                    self.spu_cache[v].vol[1] = c.vol_r;
                    self.ramp[v][1] = false;
                    self.spu_cache[v].word = c.word;
                } else {
                    // Still sounding: key off now, retry next tick.
                    koff |= 1 << v;
                    keyed = true;
                }
            }
            let nax = self.get_addr(v, NAX);
            let pos = if keyed {
                started
            } else if nax < SAFE_LOOP_END {
                0
            } else {
                nax
            };
            self.status.spu[v] = (c.word, pos);
        }
        self.update_iop_voices(cmd, iop_ram);
        if kon != 0 {
            self.set_switch(KON, kon);
        }
        if koff != 0 {
            self.set_switch(KOFF, koff);
        }
    }

    /// `SNDIOP_setiopvoicevol`: dry gains of an active IOP voice.
    fn set_iop_voice_vol(&mut self, i: usize, channels: usize) {
        if !self.iop_active[i] {
            return;
        }
        for ch in 0..channels {
            let g = self.iop_cache[i].dry[ch] as i32;
            let g = if g < 0 { g << 8 } else { g * 0x102 };
            self.mix.set_dry(i, ch, g);
        }
    }

    /// `SNDIOP_updateiopvoices` (0x373b8).
    fn update_iop_voices(&mut self, cmd: &CommandBlock, iop_ram: &[u8]) {
        let Some(cfg) = self.cfg else { return };
        if cmd.resample_quality != self.quality {
            self.quality = cmd.resample_quality;
            self.mix.set_quality(self.quality);
        }
        let channels = (cfg.channels + cfg.aux) as usize;
        for i in 0..(cfg.iop_voices as usize).min(IOP_VOICES) {
            let c = cmd.iop[i];
            self.status.iop[i].0 = c.word;
            if c.pitch != self.iop_cache[i].pitch {
                self.iop_cache[i].pitch = c.pitch;
                if self.iop_active[i] {
                    self.mix.set_pitch(i, (c.pitch as u32) << 4);
                }
            }
            let mut gains = false;
            for ch in 0..channels {
                if c.dry[ch] != self.iop_cache[i].dry[ch] {
                    self.iop_cache[i].dry[ch] = c.dry[ch];
                    gains = true;
                }
            }
            if gains {
                self.set_iop_voice_vol(i, channels);
            }
            if c.wet != self.iop_cache[i].wet {
                self.iop_cache[i].wet = c.wet;
                if self.iop_active[i] {
                    self.mix.set_wet(i, (c.wet as i32) << 8);
                }
            }
            if c.word != self.iop_cache[i].word {
                self.iop_cache[i].word = c.word;
                let hdr = c.word & 0x7F_FFFF;
                if hdr != 0 {
                    if self.iop_active[i] {
                        self.mix.stop(i);
                        self.iop_active[i] = false;
                    }
                    self.mix.play(i, hdr, iop_ram);
                    self.iop_active[i] = true;
                    self.status.iop[i].1 = 0;
                    self.mix.set_pitch(i, (self.iop_cache[i].pitch as u32) << 4);
                    self.set_iop_voice_vol(i, channels);
                    self.mix.set_wet(i, (self.iop_cache[i].wet as i32) << 8);
                } else if self.iop_active[i] {
                    self.mix.stop(i);
                    self.iop_active[i] = false;
                    self.status.iop[i].1 = -1;
                }
            }
            if self.iop_active[i] {
                self.status.iop[i].1 = self.mix.frame(i).max(0);
            }
        }
    }

    /// `SNDIOP_mix`: IOP voices and EE PCM into the next AutoDMA half.
    fn mix_half(&mut self, cfg: InitParams, pcm: &[i16], iop_ram: &[u8]) {
        let frames = cfg.iop_frames();
        self.mix.slice(iop_ram, &mut self.buses, frames);
        // Voices that ended inside the slice (`SNDIOP_mixvoicefree`).
        for i in 0..(cfg.iop_voices as usize).min(IOP_VOICES) {
            if self.iop_active[i] && !self.mix.is_active(i) {
                self.iop_active[i] = false;
                self.status.iop[i].1 = -1;
            }
        }
        let mut half = [0i16; 1024];
        self.out.mix(cfg.ee_rate as u32, cfg.iop_rate as u32, pcm, &mut self.buses, &mut half);
        // Half layout: [L 256][R 256][L 256][R 256] (0x800 bytes, 512 frames).
        let mut frames_out = Vec::with_capacity(TICK_FRAMES);
        for blk in 0..2 {
            let l = &half[blk * 512..blk * 512 + 256];
            let r = &half[blk * 512 + 256..blk * 512 + 512];
            for n in 0..256 {
                frames_out.push([l[n], r[n]]);
            }
        }
        self.pending = frames_out;
    }
}
