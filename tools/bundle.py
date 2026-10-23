#!/usr/bin/env python3
"""Build a standalone SSX 3 for macOS, Linux or Windows: the ssx3 window
binary with the game data packed from your disc image.

    python3 tools/bundle.py TARGET... [--iso PATH] [--out DIR]
    python3 tools/bundle.py pack [--iso PATH] [OUT]      # only the data pack

TARGET: macos (this Mac's architecture), linux-x86_64, linux-aarch64,
windows-x86_64, or all. The Linux and Windows builds run in Docker
(tools/bundle/Dockerfile: a cross toolchain and MinGW); macOS builds natively
and needs a Mac.

The pack (`ssx3-core/src/pack.rs`) is the disc image without the padding
files (PAD0.000, PAD1.000) and all-zero sectors, about 2.4 GB. It is
appended to the Linux and Windows executables, which read it back from
themselves (a single file). macOS gets `SSX3.app` with the pack in
`Contents/Resources`: a signed binary cannot carry appended data, and Apple
Silicon runs only signed code (the bundle is signed ad hoc).

The result plays from anywhere: no ISO, no repository. Saves and settings go
to the user's data folder (see `game::init_standalone`).

The build needs the generated code (built from your game, not in git), so it
can only be made on a machine that has it. The output contains the game:
it is for your own use, do not distribute it.
"""
import argparse
import os
import platform
import shutil
import struct
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SECTOR = 2048
MAGIC, TRAILER = b"SSX3PAK1", b"SSX3PAKT"
# Files the game never reads (disc padding).
SKIP = {"/PAD0.000", "/PAD1.000"}
TARGETS = {
    "linux-x86_64": ("x86_64-unknown-linux-gnu", "ssx3", "/usr/lib/x86_64-linux-gnu/pkgconfig"),
    "linux-aarch64": ("aarch64-unknown-linux-gnu", "ssx3", "/usr/lib/aarch64-linux-gnu/pkgconfig"),
    "windows-x86_64": ("x86_64-pc-windows-gnu", "ssx3.exe", None),
}
IMAGE = "ssx3-bundle"


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


def append_pack(exe, pack, out):
    """`out` = `exe` + `pack` + trailer (the pack's file offset)."""
    shutil.copyfile(exe, out)
    with open(out, "ab") as o, open(pack, "rb") as p:
        start = o.tell()
        shutil.copyfileobj(p, o, 64 << 20)
        o.write(struct.pack("<Q", start) + TRAILER)
    os.chmod(out, 0o755)
    print(f"bundle: {out} ({os.path.getsize(out) / 1e9:.2f} GB)")


def build_macos(pack, out_dir):
    if platform.system() != "Darwin":
        sys.exit("macos: build it on a Mac")
    run(["cargo", "build", "--profile", "game", "-p", "ssx3", "--bin", "ssx3"], cwd=ROOT)
    app = os.path.join(out_dir, "SSX3.app")
    shutil.rmtree(app, ignore_errors=True)
    os.makedirs(os.path.join(app, "Contents/MacOS"))
    os.makedirs(os.path.join(app, "Contents/Resources"))
    shutil.copy2(os.path.join(ROOT, "target/game/ssx3"), os.path.join(app, "Contents/MacOS/ssx3"))
    # A hard link where possible (same volume), else a copy.
    res = os.path.join(app, "Contents/Resources/ssx3.pack")
    try:
        os.link(pack, res)
    except OSError:
        shutil.copyfile(pack, res)
    with open(os.path.join(app, "Contents/Info.plist"), "w") as f:
        f.write("""<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>CFBundleName</key><string>SSX 3</string>
  <key>CFBundleDisplayName</key><string>SSX 3</string>
  <key>CFBundleIdentifier</key><string>local.ssx3.native</string>
  <key>CFBundleExecutable</key><string>ssx3</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleVersion</key><string>1</string>
  <key>CFBundleShortVersionString</key><string>1.0</string>
  <key>LSMinimumSystemVersion</key><string>11.0</string>
  <key>NSHighResolutionCapable</key><true/>
</dict></plist>
""")
    run(["codesign", "--force", "--deep", "--sign", "-", app])
    print(f"bundle: {app}")


def build_docker(name, pack, out_dir):
    triple, exe, pkg = TARGETS[name]
    run(["docker", "build", "-t", IMAGE, os.path.join(ROOT, "tools/bundle")])
    # Per-target build directory inside the repo (kept between runs), and a
    # shared cargo registry cache.
    target_dir = f"target/bundle-{triple}"
    env = ["-e", f"CARGO_TARGET_DIR=/src/{target_dir}"]
    if pkg:
        env += ["-e", f"PKG_CONFIG_PATH={pkg}", "-e", "PKG_CONFIG_SYSROOT_DIR=/"]
    run(["docker", "run", "--rm", "-v", f"{ROOT}:/src", "-v", "ssx3-cargo-registry:/usr/local/cargo/registry",
         "-w", "/src", *env, IMAGE,
         "cargo", "build", "--profile", "game", "-p", "ssx3", "--bin", "ssx3", "--target", triple])
    built = os.path.join(ROOT, target_dir, triple, "game", exe)
    suffix = ".exe" if exe.endswith(".exe") else ""
    append_pack(built, pack, os.path.join(out_dir, f"ssx3-{name}{suffix}"))


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("targets", nargs="+", help="macos, linux-x86_64, linux-aarch64, windows-x86_64, all, or pack [OUT]")
    ap.add_argument("--iso", default=os.environ.get("SSX3_ISO", os.path.join(ROOT, "SSX 3 (USA).iso")))
    ap.add_argument("--out", default=os.path.join(ROOT, "dist"))
    a = ap.parse_args()
    pack = os.path.join(ROOT, "target/bundle/ssx3.pack")
    if a.targets[0] == "pack":
        make_pack(a.iso, a.targets[1] if len(a.targets) > 1 else pack)
        return
    targets = ["macos", *TARGETS] if a.targets == ["all"] else a.targets
    for t in targets:
        if t != "macos" and t not in TARGETS:
            sys.exit(f"unknown target {t}")
    make_pack(a.iso, pack)
    os.makedirs(a.out, exist_ok=True)
    # One build at a time (the machine's limit).
    for t in targets:
        if t == "macos":
            build_macos(pack, a.out)
        else:
            build_docker(t, pack, a.out)


if __name__ == "__main__":
    main()
