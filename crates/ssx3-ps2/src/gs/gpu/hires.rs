//! N× copies of the buffers native runs draw into (`SSX3_RENDER_SCALE=N`,
//! phase 3 rendering, `docs/investigation/phase3-render.md`).
//!
//! Every native run is drawn twice: into the 1× targets, which are written
//! back to VRAM exactly as before (everything that reads VRAM stays exact),
//! and into N× textures that only the presentation uses. The N× textures are
//! kept per buffer (frame buffers as colour, Z buffers as depth), all the
//! same size (N × the 1× target size) so any frame/Z pair can be bound
//! together.
//!
//! A buffer's N× copy is current while nothing else wrote its VRAM pages:
//! every VRAM write (uploads, compute raster batches, transfers, other
//! native runs) stamps its pages, and a copy older than its pages is
//! refreshed from VRAM (an upscale) before the next run that draws it.

use std::collections::HashMap;

use super::super::raster::Pages;

/// `SSX3_RENDER_SCALE` (1 = off; at most 8).
pub(crate) fn scale() -> u32 {
    static S: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *S.get_or_init(|| {
        std::env::var("SSX3_RENDER_SCALE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1)
            .clamp(1, 8)
    })
}

/// A buffer: base (64-word blocks), width (in 64 pixels), format, and
/// whether it is a Z buffer.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub(crate) struct Key {
    pub bp: u32,
    pub bw: u32,
    pub psm: u32,
    pub depth: bool,
}

pub(crate) struct Surf {
    pub view: wgpu::TextureView,
    /// The VRAM pages this copy stands for (the buffer's extent).
    pages: Pages,
    /// Write clock when this copy was last made current.
    clock: u64,
}

pub(crate) struct Hires {
    pub scale: u32,
    /// Texture size (N × the 1× target size).
    pub w: u32,
    pub h: u32,
    pub surfs: HashMap<Key, Surf>,
    /// Destination-read copy (`n3_prev`) at N×.
    pub prev: Option<wgpu::TextureView>,
    /// Bound when a run has no frame / no Z.
    pub dummy_color: Option<wgpu::TextureView>,
    pub dummy_depth: Option<wgpu::TextureView>,
    page_clock: Box<[u64; 512]>,
    clock: u64,
    pub refreshes: u64,
}

fn each_page(p: &Pages, mut f: impl FnMut(usize)) {
    for (i, w) in p.0.iter().enumerate() {
        let mut w = *w;
        while w != 0 {
            f(i * 64 + w.trailing_zeros() as usize);
            w &= w - 1;
        }
    }
}

fn texture(
    dev: &wgpu::Device,
    label: &str,
    w: u32,
    h: u32,
    format: wgpu::TextureFormat,
    extra: wgpu::TextureUsages,
) -> wgpu::TextureView {
    dev.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | extra,
        view_formats: &[],
    })
    .create_view(&Default::default())
}

impl Hires {
    pub fn new(scale: u32) -> Hires {
        Hires {
            scale,
            w: 0,
            h: 0,
            surfs: HashMap::new(),
            prev: None,
            dummy_color: None,
            dummy_depth: None,
            page_clock: Box::new([0; 512]),
            clock: 1,
            refreshes: 0,
        }
    }

    /// Match the 1× targets (`tw` x `th`); a new size drops every copy.
    pub fn ensure_size(&mut self, dev: &wgpu::Device, tw: u32, th: u32) {
        let (w, h) = (tw * self.scale, th * self.scale);
        if w == self.w && h == self.h {
            return;
        }
        self.w = w;
        self.h = h;
        self.surfs.clear();
        let c = wgpu::TextureUsages::COPY_SRC;
        self.prev = Some(texture(
            dev,
            "gs-hires-prev",
            w,
            h,
            super::native::COLOR_FORMAT,
            wgpu::TextureUsages::COPY_DST,
        ));
        self.dummy_color = Some(texture(
            dev,
            "gs-hires-dummy-c",
            w,
            h,
            super::native::COLOR_FORMAT,
            c,
        ));
        self.dummy_depth = Some(texture(
            dev,
            "gs-hires-dummy-z",
            w,
            h,
            super::native::DEPTH_FORMAT,
            c,
        ));
    }

    /// Any VRAM write to `pages` (native runs included).
    pub fn note_write(&mut self, pages: &Pages) {
        self.clock += 1;
        let c = self.clock;
        each_page(pages, |p| self.page_clock[p] = c);
    }

    /// The N× copy of `key` for a run over `pages`; true when it must first
    /// be refreshed from VRAM (new, or written since it was current).
    pub fn acquire(&mut self, dev: &wgpu::Device, key: Key, pages: &Pages) -> bool {
        let fresh = !self.surfs.contains_key(&key);
        if fresh {
            let format = if key.depth {
                super::native::DEPTH_FORMAT
            } else {
                super::native::COLOR_FORMAT
            };
            let view = texture(
                dev,
                "gs-hires-surface",
                self.w,
                self.h,
                format,
                wgpu::TextureUsages::COPY_SRC,
            );
            self.surfs.insert(
                key,
                Surf {
                    view,
                    pages: *pages,
                    clock: 0,
                },
            );
        }
        if let Some(s) = self.surfs.get_mut(&key) {
            s.pages.union(pages);
        }
        let s = &self.surfs[&key];
        let mut stale = fresh;
        each_page(pages, |p| stale |= self.page_clock[p] > s.clock);
        self.refreshes += stale as u64;
        stale
    }

    /// Whether the N× copy of `key` exists and nothing wrote `pages` since
    /// it was current (a texture can then be sampled from it).
    pub fn is_current(&self, key: &Key, pages: &Pages) -> bool {
        let Some(s) = self.surfs.get(key) else {
            return false;
        };
        // Only the texture pages inside the buffer: a texture's extent is a
        // power of two and can reach past the buffer into other buffers.
        let mut inside = *pages;
        for (w, o) in inside.0.iter_mut().zip(s.pages.0.iter()) {
            *w &= *o;
        }
        let mut ok = true;
        each_page(&inside, |p| ok &= self.page_clock[p] <= s.clock);
        ok
    }

    /// Whether the N× copy of `key` stands for all of `pages` and is
    /// current there (a displayed rectangle).
    pub fn covers(&self, key: &Key, pages: &Pages) -> bool {
        let Some(s) = self.surfs.get(key) else {
            return false;
        };
        let mut ok = true;
        each_page(pages, |p| {
            ok &= s.pages.0[p / 64] & 1 << (p % 64) != 0 && self.page_clock[p] <= s.clock
        });
        ok
    }

    /// `key` was just drawn by a run over `pages` (after `note_write`).
    pub fn mark_current(&mut self, key: Key) {
        let c = self.clock;
        if let Some(s) = self.surfs.get_mut(&key) {
            s.clock = c;
        }
    }
}
