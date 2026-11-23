#!/usr/bin/env python3
"""Importer of the SSX Tricky plugin, run by `ssx3 plugins prepare tricky`.

Reads your SSX Tricky disc ($SSX3_PLUGIN_DISC) and your SSX 3 disc
($SSX3_BASE_DISC) and writes everything it derives to $SSX3_PLUGIN_OUT/overlay
(the plugin's cache folder, never into git), then creates $SSX3_PLUGIN_OUT/ready.

  SSX3_TRICKY_COURSES=garibaldi,alaska   only these courses (all by default)
  SSX3_TRICKY_FORCE=1                    rebuild what is already prepared
"""
import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "tools"))

import prepare_tricky_tracks  # noqa: E402


def main() -> None:
    out = Path(os.environ["SSX3_PLUGIN_OUT"])
    argv = [
        "prepare_tricky_tracks.py",
        "--ssx3-iso", os.environ["SSX3_BASE_DISC"],
        "--tricky-iso", os.environ["SSX3_PLUGIN_DISC"],
        "--overlay", str(out / "overlay"),
    ]
    for course in filter(None, os.environ.get("SSX3_TRICKY_COURSES", "").split(",")):
        argv += ["--course", course]
    if os.environ.get("SSX3_TRICKY_FORCE"):
        argv.append("--force")
    sys.argv = argv
    prepare_tricky_tracks.main()
    # The picture of the peak on the Select Peak screen (plugin.toml `picture`).
    from tricky_ui_assets import peak_picture
    peak_picture(Path(os.environ["SSX3_PLUGIN_DISC"]), out / "ui" / "tricky.png")
    (out / "ready").write_text("")


if __name__ == "__main__":
    main()
