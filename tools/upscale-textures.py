#!/usr/bin/env python3
"""Upscale dumped textures (textures/dump, SSX3_TEX_DUMP) into a texture pack
(textures/replace) with Real-ESRGAN (realesrgan-ncnn-vulkan).

    tools/upscale-textures.py --setup                # download Real-ESRGAN and the models
    tools/upscale-textures.py NAME [NAME ...]        # names or files from textures/dump
    tools/upscale-textures.py --all                  # every dumped texture >= --min px
    options: --model ultrasharp-4x (default) | remacri-4x | realesrgan-x4plus | ...
             --scale 4  --min 32
             --esrgan PATH (default textures/.tools/realesrgan/realesrgan-ncnn-vulkan)

Models: UltraSharp keeps the textures' fine grain (snow) while sharpening;
realesrgan-x4plus denoises it away (flat, plastic snow). Remacri adds the most
detail to hard-edged art.

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
    w, h = im.size
    p = max(4, min(w, h) // 4)
    with tempfile.TemporaryDirectory() as tmp:
        rgb = esrgan(exe, model, scale, wrap_pad(im.convert("RGB"), p), tmp)
        a = im.getchannel("A")
        if a.getextrema() != (255, 255):
            alpha = esrgan(exe, model, scale, wrap_pad(a.convert("RGB"), p), tmp).convert("L")
        else:
            alpha = Image.new("L", rgb.size, 255)
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
    ap.add_argument("--model", default="ultrasharp-4x")
    ap.add_argument("--scale", type=int, default=4)
    ap.add_argument("--min", type=int, default=32)
    ap.add_argument("--esrgan", default=os.path.join(ROOT, "textures/.tools/realesrgan/realesrgan-ncnn-vulkan"))
    a = ap.parse_args()
    if a.setup:
        return setup(a.esrgan)
    names = sorted(os.listdir(a.dump)) if a.all else [n if n.endswith(".png") else n + ".png" for n in a.names]
    os.makedirs(a.out, exist_ok=True)
    for i, n in enumerate(names):
        src = os.path.join(a.dump, os.path.basename(n))
        im = Image.open(src)
        if min(im.size) < a.min:
            print(f"[{i + 1}/{len(names)}] {n}: {im.size[0]}x{im.size[1]}, skipped")
            continue
        upscale(src, os.path.join(a.out, os.path.basename(n)), a.esrgan, a.model, a.scale)
        print(f"[{i + 1}/{len(names)}] {n}: {im.size[0]}x{im.size[1]} -> x{a.scale}")


if __name__ == "__main__":
    sys.exit(main())
