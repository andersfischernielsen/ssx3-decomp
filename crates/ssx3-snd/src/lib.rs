//! # ssx3-snd: native sound driver (phase 3)
//!
//! EA's SND library splits its work between the EE and an IOP driver
//! (SNDDRV.IRX on Sony's LIBSD.IRX). Until now the IOP side ran as statically
//! recompiled IOP code on an emulated IOP kernel and SIF (`SSX3_IOP=1`,
//! `docs/investigation/iop-audio.md`). This crate is that driver in plain
//! Rust: it consumes exactly what the EE sends (tag buffers, sample uploads,
//! the init RPC) and returns exactly what the EE reads (the status block and
//! the per-tick command). Specification: `docs/investigation/phase3-sound.md`.
//!
//! - [`proto`]: the wire formats.
//! - [`output`]: SNDDRV's resampling of the EE and IOP mixes to 48 kHz.
//! - [`iopmix`]: EA's IOP software voices (MIX/SFILTER), used for banks in
//!   IOP RAM (`zboard.bnk`).
//! - [`driver`]: the per-tick logic (`SNDIOP_updatevoices` and friends) on the
//!   SPU2 model (`ssx3_iop::spu2`).

pub mod driver;
pub mod iopmix;
pub mod output;
pub mod proto;
