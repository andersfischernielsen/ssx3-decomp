#!/usr/bin/env python3
"""One-time setup of a fresh checkout, on macOS, Linux or Windows: extract the
game's executable from your disc image and translate the game into Rust
(`generated/`, not in git: it is made from your own copy of the game).

    python3 tools/setup.py [--iso PATH] [recomp-ee args...]
    python3 tools/setup.py --stubs-only     # no disc: only the stub crates (CI)

What `tools/extract-from-iso.sh` and `tools/bootstrap.sh` do, in Python (no
POSIX shell or bsdtar needed):

1. `orig/SLUS_207.72` and `orig/SYSTEM.CNF` from the image (its ISO9660 file
   system is read directly), with the SHA-256 check of the executable.
2. Empty stub crates for the generated code, so cargo can load the workspace.
3. The VU microcode (`vu_recomp`) and the EE game code (`recomp-ee`), which
   also writes the IOP modules.

Run it again after pulling changes to the recompiler (`crates/ssx3-recomp`).
"""
import argparse
import hashlib
import os
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from bundle import SECTOR, iso_files  # noqa: E402

# SLUS_207.72 of SSX 3 (USA), the only version this project supports.
EXPECTED_SHA256 = "1b49d05ca2793922180851b9e1ce9ae2291d61a7863565ac4e71f12e967af7bc"

STUBS = {
    "generated/ee/game": ("ssx3-ee", "ssx3-ee-rt = { path = \"../../../crates/ssx3-ee-rt\" }",
                          "pub static FUNCTIONS: [ssx3_ee_rt::funcs::FnEntry; 0] = [];\n"
                          "pub const HOST_THREADS: bool = false;\n"),
    # The browser build's copy (recomp-ee --unwind); only its manifest is needed here.
    "generated/ee-unwind/game": ("ssx3-ee-unwind", "ssx3-ee-rt = { path = \"../../../crates/ssx3-ee-rt\" }",
                                 "pub static FUNCTIONS: [ssx3_ee_rt::funcs::FnEntry; 0] = [];\n"
                                 "pub const HOST_THREADS: bool = false;\n"),
    "generated/iop/modules": ("ssx3-iop-gen", "ssx3-iop = { path = \"../../../crates/ssx3-iop\" }",
                              "pub static GENERATED: ssx3_iop::Generated = "
                              "ssx3_iop::Generated { functions: &[], stubs: &[], modules: &[] };\n"),
}
VU_PLACEHOLDER = ("//! Placeholder; tools/setup.py replaces it.\nuse crate::vu::ProgramEntry;\n\n"
                  "pub static PROGRAMS: &[ProgramEntry] = &[];\n")


def run(cmd):
    print("+", " ".join(cmd), flush=True)
    subprocess.run(cmd, check=True, cwd=ROOT)


def extract(iso):
    """orig/SLUS_207.72 and orig/SYSTEM.CNF from the image, checked."""
    if not os.path.isfile(iso):
        sys.exit(f"error: disc image not found: {iso}\n"
                 "Put your image at the repository root as 'SSX 3 (USA).iso' or pass --iso.")
    out = os.path.join(ROOT, "orig")
    os.makedirs(out, exist_ok=True)
    print(f"Extracting from {iso} ...")
    with open(iso, "rb") as f:
        files = {p: (lba, size) for p, lba, size in iso_files(f)}
        for name in ("SLUS_207.72", "SYSTEM.CNF"):
            if "/" + name not in files:
                sys.exit(f"error: {name} is not on the image: not the SSX 3 (USA) disc")
            lba, size = files["/" + name]
            f.seek(lba * SECTOR)
            data = f.read(size)
            path = os.path.join(out, name)
            if os.path.exists(path):
                os.chmod(path, 0o644)
            with open(path, "wb") as o:
                o.write(data)
    sha = hashlib.sha256(open(os.path.join(out, "SLUS_207.72"), "rb").read()).hexdigest()
    if sha != EXPECTED_SHA256:
        sys.exit(f"error: orig/SLUS_207.72 has SHA-256 {sha}\n"
                 f"       expected {EXPECTED_SHA256} (SSX 3, USA, SLUS-20772).\n"
                 "       Other regions or revisions are not supported.")
    print("orig/SLUS_207.72 OK (SHA-256 matches SSX 3 USA)")


def stubs():
    for d, (name, dep, lib) in STUBS.items():
        if os.path.isfile(os.path.join(ROOT, d, "Cargo.toml")):
            continue
        os.makedirs(os.path.join(ROOT, d, "src"), exist_ok=True)
        with open(os.path.join(ROOT, d, "Cargo.toml"), "w") as f:
            f.write(f'[package]\nname = "{name}"\nversion = "0.1.0"\nedition = "2021"\npublish = false\n\n'
                    f"[dependencies]\n{dep}\n")
        with open(os.path.join(ROOT, d, "src", "lib.rs"), "w") as f:
            f.write(lib)


def vu_placeholder():
    os.makedirs(os.path.join(ROOT, "generated", "vu"), exist_ok=True)
    with open(os.path.join(ROOT, "generated", "vu", "mod.rs"), "w") as f:
        f.write(VU_PLACEHOLDER)


def needs_vu():
    p = os.path.join(ROOT, "generated", "vu", "mod.rs")
    return not os.path.isfile(p) or open(p).read().startswith("//! Placeholder")


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--iso", default=os.environ.get("SSX3_ISO", os.path.join(ROOT, "SSX 3 (USA).iso")))
    ap.add_argument("--stubs-only", action="store_true",
                    help="only write the stub crates (no disc needed): enough for the crates "
                         "that do not use the generated game (CI)")
    a, recomp_args = ap.parse_known_args(argv)
    if a.stubs_only:
        stubs()
        if needs_vu():
            vu_placeholder()
        return
    extract(a.iso)
    stubs()
    if needs_vu():
        # Empty registry so ssx3-vu builds; vu_recomp then translates the microcode.
        vu_placeholder()
        run(["cargo", "run", "--release", "-p", "ssx3-ps2", "--bin", "vu_recomp"])
    run(["cargo", "run", "--release", "-p", "ssx3-recomp", "--bin", "recomp-ee", "--", *recomp_args])


def is_set_up():
    """Whether the generated game exists (a real one, not the stubs)."""
    lib = os.path.join(ROOT, "generated", "ee", "game", "src", "lib.rs")
    return os.path.isfile(lib) and "FnEntry; 0]" not in open(lib).read() and not needs_vu()


if __name__ == "__main__":
    main()
