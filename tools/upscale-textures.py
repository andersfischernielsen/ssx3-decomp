#!/usr/bin/env python3
"""Upscale dumped textures (textures/dump, SSX3_TEX_DUMP) into a texture pack
(textures/replace) with Real-ESRGAN (realesrgan-ncnn-vulkan).

    tools/upscale-textures.py --setup                # download Real-ESRGAN and the models
    tools/upscale-textures.py NAME [NAME ...]        # names or files from textures/dump
    tools/upscale-textures.py --all                  # every dumped texture >= --min px
    options: --model auto (default) | ultrasharp-4x | remacri-4x | realesrgan-x4plus | ...
             --scale 4  --min 32  --jobs 2 (images upscaled at once)
             --esrgan PATH (default textures/.tools/realesrgan/realesrgan-ncnn-vulkan)

Models: UltraSharp keeps the textures' fine grain (snow, wood) while
sharpening; realesrgan-x4plus denoises it away (flat, plastic snow) but draws
foliage cleanly where UltraSharp turns the PS2's palette dither into speckle.
`auto` takes x4plus for cut-out (alpha-tested) textures, UltraSharp for the
rest. Remacri adds the most detail to hard-edged art.

Per texture: the image is padded with its own wrapped edges (repeating
textures stay seamless), the colour is upscaled, the alpha (if not opaque)
is upscaled separately as a grey image, and the padding is cropped off.
Needs Pillow."""
import argparse, io, os, subprocess, sys, tempfile, urllib.request, zipfile
from PIL import Image

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def wrap_pad(im, p):
    w, h = im.size
    out = Image.new(im.mode, (w + 2 * p, h + 2 * p))
    for dy in (-1, 0, 1):
        for dx in (-1, 0, 1):
            out.paste(im, (p + dx * w, p + dy * h))
    return out


def bleed(im):
    """Colour for the transparent texels from their opaque neighbours
    (grown outwards one texel at a time, wrapping), so upscaling does not pull
    the colour hidden under them (often black) into the edges."""
    import numpy as np
    px = np.asarray(im.convert("RGBA")).astype(np.float64)
    rgb, known = px[..., :3].copy(), px[..., 3] > 8
    if known.all() or not known.any():
        return im.convert("RGB")
    while not known.all():
        acc, cnt = np.zeros_like(rgb), np.zeros(known.shape)
        for dy in (-1, 0, 1):
            for dx in (-1, 0, 1):
                k = np.roll(known, (dy, dx), (0, 1))
                acc += np.roll(rgb, (dy, dx), (0, 1)) * k[..., None]
                cnt += k
        grow = ~known & (cnt > 0)
        rgb[grow] = acc[grow] / cnt[grow][:, None]
        known = known | grow
    return Image.fromarray(rgb.round().astype(np.uint8), "RGB")


def esrgan(exe, model, scale, im, tmp):
    src, dst = os.path.join(tmp, "in.png"), os.path.join(tmp, "out.png")
    im.convert("RGB").save(src)
    models = os.path.join(os.path.dirname(exe), "models")
    r = subprocess.run([exe, "-i", src, "-o", dst, "-n", model, "-s", str(scale), "-m", models], capture_output=True, text=True)
    if r.returncode != 0 or not os.path.exists(dst):
        raise RuntimeError(r.stderr[-500:])
    return Image.open(dst).convert("RGB")


def upscale(path, out, exe, model, scale):
    im = Image.open(path).convert("RGBA")
    if model == "auto":
        levels = set(im.getchannel("A").getdata())
        model = "realesrgan-x4plus" if levels <= {0, 255} and levels != {255} else "ultrasharp-4x"
    w, h = im.size
    p = max(4, min(w, h) // 4)
    a = im.getchannel("A")
    with tempfile.TemporaryDirectory() as tmp:
        levels = set(a.getdata())
        rgb = esrgan(exe, model, scale, wrap_pad(bleed(im), p), tmp)
        # Some textures use alpha as a mask (environment map, lighting) and
        # are drawn opaque: their "transparent" texels are seen. There the
        # original colours are kept (upscaled as they are); the filled ones
        # only where the texture is opaque, so edges get no dark fringe.
        raw = esrgan(exe, model, scale, wrap_pad(im.convert("RGB"), p), tmp) if levels != {255} else None
        if levels == {255}:
            alpha = Image.new("L", rgb.size, 255)
        elif levels <= {0, 255}:
            # A cut-out (alpha test): a smooth outline, cut cleanly at half.
            soft = wrap_pad(a, p).resize(rgb.size, Image.BICUBIC)
            alpha = soft.point(lambda v: max(0, min(255, (v - 128) * 6 + 128)))
        else:
            alpha = esrgan(exe, model, scale, wrap_pad(a.convert("RGB"), p), tmp).convert("L")
    if raw is not None:
        rgb = Image.composite(rgb, raw, alpha.point(lambda v: 255 if v > 16 else 0))
    box = (p * scale, p * scale, (p + w) * scale, (p + h) * scale)
    res = rgb.crop(box)
    res.putalpha(alpha.crop(box))
    res.save(out)


ESRGAN_ZIP = "https://github.com/xinntao/Real-ESRGAN/releases/download/v0.2.5.0/realesrgan-ncnn-vulkan-20220424-{}.zip"
MODELS = "https://raw.githubusercontent.com/upscayl/upscayl/main/resources/models/{}.{}"


def setup(exe):
    d = os.path.dirname(exe)
    if not os.path.exists(exe):
        plat = {"darwin": "macos", "linux": "ubuntu", "win32": "windows"}[sys.platform]
        print("downloading Real-ESRGAN (ncnn-vulkan) ...")
        z = zipfile.ZipFile(io.BytesIO(urllib.request.urlopen(ESRGAN_ZIP.format(plat)).read()))
        z.extractall(d)
        os.chmod(exe, 0o755)
    for m in ("ultrasharp-4x", "remacri-4x"):
        for ext in ("param", "bin"):
            dst = os.path.join(d, "models", f"{m}.{ext}")
            if not os.path.exists(dst):
                print(f"downloading {m}.{ext} ...")
                urllib.request.urlretrieve(MODELS.format(m, ext), dst)
    print("ready:", exe)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("names", nargs="*")
    ap.add_argument("--all", action="store_true")
    ap.add_argument("--setup", action="store_true")
    ap.add_argument("--dump", default=os.path.join(ROOT, "textures/dump"))
    ap.add_argument("--out", default=os.path.join(ROOT, "textures/replace"))
    ap.add_argument("--model", default="auto")
    ap.add_argument("--scale", type=int, default=4)
    ap.add_argument("--min", type=int, default=32)
    ap.add_argument("--jobs", type=int, default=2)
    ap.add_argument("--esrgan", default=os.path.join(ROOT, "textures/.tools/realesrgan/realesrgan-ncnn-vulkan"))
    a = ap.parse_args()
    if a.setup:
        return setup(a.esrgan)
    names = sorted(os.listdir(a.dump)) if a.all else [n if n.endswith(".png") else n + ".png" for n in a.names]
    os.makedirs(a.out, exist_ok=True)
    from concurrent.futures import ThreadPoolExecutor, as_completed

    def one(n):
        src = os.path.join(a.dump, os.path.basename(n))
        im = Image.open(src)
        if min(im.size) < a.min:
            return f"{n}: {im.size[0]}x{im.size[1]}, skipped"
        upscale(src, os.path.join(a.out, os.path.basename(n)), a.esrgan, a.model, a.scale)
        return f"{n}: {im.size[0]}x{im.size[1]} -> x{a.scale}"

    # Each image runs its own Real-ESRGAN process.
    with ThreadPoolExecutor(max(1, a.jobs)) as pool:
        futs = [pool.submit(one, n) for n in names]
        for i, f in enumerate(as_completed(futs)):
            print(f"[{i + 1}/{len(names)}] {f.result()}", flush=True)


if __name__ == "__main__":
    sys.exit(main())
