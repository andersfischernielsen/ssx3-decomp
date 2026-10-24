#!/usr/bin/env python3
"""Build a standalone SSX 3 for this machine's OS: the ssx3 window binary
with the game data packed from your disc image.

    python3 tools/bundle.py [--iso PATH] [--out DIR]
    python3 tools/bundle.py pack [--iso PATH] [OUT]      # only the data pack

It builds for the OS it runs on, no cross-building: run it on a Mac for
macOS, on Linux for Linux, on Windows for Windows. A Linux build can be made
on any machine with Docker (tools/bundle/Dockerfile is a Linux build
environment; see the README).

- macOS: `dist/SSX3.app`, the pack in `Contents/Resources` (a signed binary
  cannot carry appended data, and Apple Silicon runs only signed code; the
  bundle is signed ad hoc).
- Linux: `dist/ssx3-linux-<arch>`, one file: the pack appended to the
  executable, which reads it back from itself.
- Windows: `dist\ssx3-windows-<arch>.exe`, one file, the same way.

The icon is the game's own SSX 3 logo (tools/bundle_icon.py): the app's
icon on macOS, compiled into the executable on Windows; the window sets it
at run time on every OS (Linux executables carry none).

The pack (`ssx3-core/src/pack.rs`) is the disc image without the padding
files (PAD0.000, PAD1.000) and all-zero sectors, about 2.4 GB. The result
plays from anywhere: no ISO, no repository. Saves and settings go to the
user's data folder (see `game::init_standalone`).

The build needs the generated code (built from your game, not in git), so
it can only be made on a machine that has it. The output contains the game:
it is for your own use, do not distribute it.
"""
import argparse
import os
import platform
import shutil
import struct
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bundle_icon  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SECTOR = 2048
MAGIC, TRAILER = b"SSX3PAK1", b"SSX3PAKT"
# Files the game never reads (disc padding).
SKIP = {"/PAD0.000", "/PAD1.000"}
# Cargo's output directory (CARGO_TARGET_DIR, as cargo itself reads it).
TARGET_DIR = os.environ.get("CARGO_TARGET_DIR", os.path.join(ROOT, "target"))


def run(cmd, **kw):
    print("+", " ".join(cmd), flush=True)
    subprocess.run(cmd, check=True, **kw)


def iso_files(f):
    """(path, first sector, size) of every file on the ISO9660 image."""
    def sec(n, c=1):
        f.seek(n * SECTOR)
        return f.read(SECTOR * c)

    out = []

    def walk(rec, path):
        lba, size = struct.unpack_from("<I", rec, 2)[0], struct.unpack_from("<I", rec, 10)[0]
        data, i = sec(lba, (size + SECTOR - 1) // SECTOR), 0
        while i < min(len(data), size):
            n = data[i]
            if n == 0:
                i = (i // SECTOR + 1) * SECTOR
                continue
            r = data[i:i + n]
            name = r[33:33 + r[32]]
            if name not in (b"\x00", b"\x01"):
                name = name.decode("latin-1").split(";")[0]
                if r[25] & 2:
                    walk(r, path + name + "/")
                else:
                    out.append((path + name, struct.unpack_from("<I", r, 2)[0], struct.unpack_from("<I", r, 10)[0]))
            i += n

    walk(sec(16)[156:190], "/")
    return out


def make_pack(iso, out):
    """Write the pack for `iso` to `out` (reused while the image is unchanged)."""
    stamp = f"{os.path.getsize(iso)} {int(os.path.getmtime(iso))}\n"
    if os.path.exists(out) and os.path.exists(out + ".stamp") and open(out + ".stamp").read() == stamp:
        print(f"pack: {out} is up to date")
        return
    with open(iso, "rb") as f:
        files = iso_files(f)
        names = {p for p, _, _ in files}
        if "/SLUS_207.72" not in names or "/SYSTEM.CNF" not in names:
            sys.exit(f"{iso}: not the SSX 3 (USA) disc (no SLUS_207.72)")
        total = os.path.getsize(iso) // SECTOR
        skip = bytearray(total)
        for p, lba, size in files:
            if p in SKIP:
                for s in range(lba, min(total, lba + (size + SECTOR - 1) // SECTOR)):
                    skip[s] = 1
        # Runs of kept, non-zero sectors.
        extents, cur = [], None
        zero = bytes(SECTOR)
        f.seek(0)
        for s in range(total):
            d = f.read(SECTOR)
            keep = not skip[s] and d != zero
            if keep and cur is not None and cur[0] + cur[1] == s:
                cur[1] += 1
            elif keep:
                cur = [s, 1]
                extents.append(cur)
            if s % 65536 == 0:
                print(f"\rpack: scanning {100 * s // total}%", end="", flush=True)
        print("\rpack: scanning 100%")
        head = 16 + 16 * len(extents)
        tmp = out + ".tmp"
        os.makedirs(os.path.dirname(os.path.abspath(out)), exist_ok=True)
        with open(tmp, "wb") as o:
            o.write(MAGIC + struct.pack("<II", total, len(extents)))
            off = head
            for first, n in extents:
                o.write(struct.pack("<IIQ", first, n, off))
                off += n * SECTOR
            for i, (first, n) in enumerate(extents):
                f.seek(first * SECTOR)
                left = n * SECTOR
                while left:
                    chunk = f.read(min(left, 64 << 20))
                    o.write(chunk)
                    left -= len(chunk)
                print(f"\rpack: writing {100 * (i + 1) // len(extents)}%", end="", flush=True)
        print()
        os.replace(tmp, out)
        open(out + ".stamp", "w").write(stamp)
    kept = sum(n for _, n in extents) * SECTOR
    print(f"pack: {out}: {kept / 1e9:.2f} GB of {total * SECTOR / 1e9:.2f} GB, {len(extents)} extents")


def make_icons(iso, out_dir):
    """The app icon from the game's logo (tools/bundle_icon.py): `ssx3.ico`
    (Windows) and `ssx3.iconset` (macOS) in `out_dir`."""
    stamp = f"{os.path.getsize(iso)} {int(os.path.getmtime(iso))} {bundle_icon.LOGO_WIDTH}\n"
    stamp_file = os.path.join(out_dir, "stamp")
    if os.path.exists(stamp_file) and open(stamp_file).read() == stamp:
        return
    os.makedirs(out_dir, exist_ok=True)
    with open(iso, "rb") as f:
        files = {p: (lba, size) for p, lba, size in iso_files(f)}

        def read(path):
            lba, size = files[path]
            f.seek(lba * SECTOR)
            return f.read(size)

        print("icon: from the logo in DATA/UI/FE_1.SSH")
        icon = bundle_icon.compose(bundle_icon.logo_from_disc(read), 1024)
    pngs = bundle_icon.sizes(icon, [16, 32, 48, 64, 128, 256, 512, 1024])
    bundle_icon.write_ico({k: v for k, v in pngs.items() if k <= 256}, os.path.join(out_dir, "ssx3.ico"))
    bundle_icon.write_iconset(pngs, os.path.join(out_dir, "ssx3.iconset"))
    open(stamp_file, "w").write(stamp)


def append_pack(exe, pack, out):
    """`out` = `exe` + `pack` + trailer (the pack's file offset)."""
    shutil.copyfile(exe, out)
    with open(out, "ab") as o, open(pack, "rb") as p:
        start = o.tell()
        shutil.copyfileobj(p, o, 64 << 20)
        o.write(struct.pack("<Q", start) + TRAILER)
    os.chmod(out, 0o755)
    print(f"bundle: {out} ({os.path.getsize(out) / 1e9:.2f} GB)")


def build_binary(ico=None):
    """The ssx3 window binary for this machine (cargo's `game` profile); on
    Windows with `ico` compiled in as its icon (crates/ssx3/build.rs)."""
    env = dict(os.environ, SSX3_ICON=os.path.abspath(ico)) if ico else None
    run(["cargo", "build", "--profile", "game", "-p", "ssx3", "--bin", "ssx3"], cwd=ROOT, env=env)
    exe = "ssx3.exe" if os.name == "nt" else "ssx3"
    return os.path.join(TARGET_DIR if os.path.isabs(TARGET_DIR) else os.path.join(ROOT, TARGET_DIR), "game", exe)


def bundle_macos(exe, pack, icons, out_dir):
    app = os.path.join(out_dir, "SSX3.app")
    shutil.rmtree(app, ignore_errors=True)
    os.makedirs(os.path.join(app, "Contents/MacOS"))
    os.makedirs(os.path.join(app, "Contents/Resources"))
    shutil.copy2(exe, os.path.join(app, "Contents/MacOS/ssx3"))
    # A hard link where possible (same volume), else a copy.
    res = os.path.join(app, "Contents/Resources/ssx3.pack")
    try:
        os.link(pack, res)
    except OSError:
        shutil.copyfile(pack, res)
    run(["iconutil", "-c", "icns", os.path.join(icons, "ssx3.iconset"),
         "-o", os.path.join(app, "Contents/Resources/ssx3.icns")])
    with open(os.path.join(app, "Contents/Info.plist"), "w") as f:
        f.write("""<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>CFBundleName</key><string>SSX 3</string>
  <key>CFBundleDisplayName</key><string>SSX 3</string>
  <key>CFBundleIdentifier</key><string>local.ssx3.native</string>
  <key>CFBundleExecutable</key><string>ssx3</string>
  <key>CFBundleIconFile</key><string>ssx3</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleVersion</key><string>1</string>
  <key>CFBundleShortVersionString</key><string>1.0</string>
  <key>LSMinimumSystemVersion</key><string>11.0</string>
  <key>NSHighResolutionCapable</key><true/>
</dict></plist>
""")
    run(["codesign", "--force", "--deep", "--sign", "-", app])
    print(f"bundle: {app}")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("cmd", nargs="*", help="nothing (build the standalone game), or: pack [OUT]")
    ap.add_argument("--iso", default=os.environ.get("SSX3_ISO", os.path.join(ROOT, "SSX 3 (USA).iso")))
    ap.add_argument("--out", default=os.path.join(ROOT, "dist"))
    a = ap.parse_args()
    pack = os.path.join(ROOT, "target", "bundle", "ssx3.pack")
    if a.cmd[:1] == ["pack"]:
        make_pack(a.iso, a.cmd[1] if len(a.cmd) > 1 else pack)
        return
    if a.cmd:
        sys.exit(f"unknown command {' '.join(a.cmd)} (this tool builds for the OS it runs on)")
    system = platform.system()
    if system not in ("Darwin", "Linux", "Windows"):
        sys.exit(f"unsupported OS {system}")
    make_pack(a.iso, pack)
    icons = os.path.join(ROOT, "target", "bundle", "icon")
    make_icons(a.iso, icons)
    os.makedirs(a.out, exist_ok=True)
    exe = build_binary(os.path.join(icons, "ssx3.ico") if system == "Windows" else None)
    arch = {"amd64": "x86_64", "arm64": "aarch64"}.get(platform.machine().lower(), platform.machine().lower())
    if system == "Darwin":
        bundle_macos(exe, pack, icons, a.out)
    elif system == "Linux":
        append_pack(exe, pack, os.path.join(a.out, f"ssx3-linux-{arch}"))
    else:
        append_pack(exe, pack, os.path.join(a.out, f"ssx3-windows-{arch}.exe"))


if __name__ == "__main__":
    main()
