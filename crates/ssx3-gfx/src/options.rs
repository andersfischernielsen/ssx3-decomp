//! Modern graphics options. The default is the game as it was: every option
//! off or at the original's setting, and the image identical to before.

use ssx3_render::direct::PostKind;

/// Anti-aliasing of the 3D image (the HUD is drawn after it, untouched).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Aa {
    #[default]
    Off,
    /// Fast approximate anti-aliasing (a post pass on the luminance edges).
    Fxaa,
}

/// The bloom pass.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Bloom {
    /// The game's own (`RenderBloom`).
    #[default]
    Original,
    /// The game's bloom replaced by a wide, soft one (bright-pass,
    /// progressive downsample and upsample blur).
    Modern,
    Off,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Options {
    pub aa: Aa,
    /// Mipmaps for the game's textures (the original has none: distant
    /// textures shimmer, anisotropic filtering has nothing to work with).
    pub mipmaps: bool,
    /// Texture LOD bias (negative: sharper).
    pub lod_bias: f32,
    /// Bezier terrain tessellation factor (1: the game's own grids).
    pub terrain: u32,
    /// Screen-space ambient occlusion on the 3D image.
    pub ao: bool,
    /// Sun shadows (shadow maps, shadow.rs).
    pub shadows: bool,
    pub bloom: Bloom,
    /// Glints on bright snow.
    pub snow: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            aa: Aa::Off,
            mipmaps: false,
            lod_bias: 0.0,
            terrain: 1,
            ao: false,
            shadows: false,
            bloom: Bloom::Original,
            snow: false,
        }
    }
}

impl Options {
    /// Whether the renderer draws its own passes before the HUD.
    pub fn any_post(&self) -> bool {
        self.aa != Aa::Off || self.ao || self.shadows || self.bloom == Bloom::Modern || self.snow
    }

    /// Whether a batch of this post pass is drawn.
    pub fn keeps_post(&self, kind: Option<PostKind>) -> bool {
        !(kind == Some(PostKind::Bloom) && self.bloom != Bloom::Original)
    }
}
