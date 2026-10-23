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

/// The colour grade applied to the 3D image.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Grade {
    #[default]
    Original,
    /// More saturation and contrast.
    Vivid,
    /// A filmic tone curve (soft highlights, lifted shadows, slight warmth).
    Filmic,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Options {
    /// The rider shadow's render target: at least this many times its GS
    /// size (0: like every render target, the internal resolution's factor).
    pub shadow_scale: u32,
    /// Soft shadow edges: the shadow texture is sampled with a blur.
    pub soft_shadows: bool,
    /// Shadows from the sun on the terrain: every caster drawn into a
    /// shadow map from the light, the terrain darkened where it is occluded.
    pub sun_shadows: bool,
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
    pub bloom: Bloom,
    pub grade: Grade,
    /// Glints on bright snow.
    pub snow: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            shadow_scale: 0,
            soft_shadows: false,
            sun_shadows: false,
            aa: Aa::Off,
            mipmaps: false,
            lod_bias: 0.0,
            terrain: 1,
            ao: false,
            bloom: Bloom::Original,
            grade: Grade::Original,
            snow: false,
        }
    }
}

impl Options {
    /// Whether the renderer draws its own passes before the HUD.
    pub fn any_post(&self) -> bool {
        self.aa != Aa::Off || self.ao || self.grade != Grade::Original || self.bloom == Bloom::Modern || self.snow
    }

    /// Whether a batch of this post pass is drawn.
    pub fn keeps_post(&self, kind: Option<PostKind>) -> bool {
        !(kind == Some(PostKind::Bloom) && self.bloom != Bloom::Original)
    }
}
