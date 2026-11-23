"""Prepare every Tricky course from the two local PS2 discs.

Derived game resources stay in the ignored overlay; no disc data enters Git.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path

from build_tricky_world import build
from ssx3_world_archive import pack_bigf
from ssx3_world_probe import bigf_members
from tricky_courses import COURSES


def world_name(course, mode: str) -> str:
    """The location name of a course/mode: `data/worlds/<name>.big` (the engine
    allows at most 15 letters, digits and '_')."""
    return f"TR_{course.code}_{mode[0].upper()}"


def rename_world(output: Path, world: str) -> Path:
    """Rewrite the archive so that its world members are `<world>.*` instead of
    `bam.*`, and move it to `DATA/WORLDS/<world>.BIG`, where the engine looks
    for the location it is told to play."""
    members = bigf_members(output.read_bytes())
    renamed = [(name.replace("data/worlds/bam.", f"data/worlds/{world.lower()}."), data)
               for name, data in members.items()]
    target = output.parent.parent / f"{world}.BIG"
    target.write_bytes(pack_bigf(renamed))
    output.unlink()
    return target


def main() -> None:
    root = Path(__file__).resolve().parent.parent
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ssx3-iso", type=Path,
                        default=Path(os.environ.get("SSX3_ISO", root / "SSX 3 (USA).iso")))
    parser.add_argument("--tricky-iso", type=Path,
                        default=Path(os.environ.get("SSX_TRICKY_ISO", root / "SSX Tricky (USA).iso")))
    parser.add_argument("--overlay", type=Path, default=root / ".cache/overlay")
    parser.add_argument("--course", action="append", help="Course ID or disc code; all by default")
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--worlds-only", action="store_true")
    parser.add_argument("--ui-only", action="store_true")
    args = parser.parse_args()
    if not args.ui_only:
        from tricky_audio import prepare as prepare_audio
        reports = prepare_audio(args.ssx3_iso, args.tricky_iso, args.overlay)
        print(f"Prepared {len(reports)} native object-sound banks", flush=True)
    sources = sorted(Path(__file__).parent.glob("ssx3_*.py")) + sorted(Path(__file__).parent.glob("tricky_*.py"))
    sources = [p for p in sources if p.name not in ("tricky_ui_assets.py",)]
    sources += [Path(__file__).with_name("build_tricky_world.py")]
    fingerprint = hashlib.sha256(b"".join(p.name.encode() + p.read_bytes() for p in sources)).hexdigest()
    results = []
    for selected in COURSES:
        if args.course and not any(value.lower() in (selected.id, selected.code.lower()) for value in args.course):
            continue
        for mode in selected.modes:
            output = args.overlay / f"DATA/WORLDS/TRICKY/{selected.code}-{mode}.BIG"
            manifest = output.with_suffix(".json")
            final = args.overlay / f"DATA/WORLDS/{world_name(selected, mode)}.BIG"
            complete = final.exists() and output.with_suffix(".FX").exists()
            previous = json.loads(manifest.read_text()) if manifest.exists() else {}
            movie_frames = previous.get("world", {}).get("native_movie_frames", 0)
            if complete and movie_frames:
                movie_file = output.with_name(f"{selected.code}.TVM")
                complete = movie_file.is_file() and movie_file.stat().st_size == 32 + movie_frames * 16384
            if args.ui_only and not (complete and previous):
                raise ValueError(f"course archive is not prepared: {final}")
            if args.ui_only or (not args.force and complete and previous.get("importer") == fingerprint):
                report = previous["world"]
                print(f"Retained {selected.name} / {mode}", flush=True)
            else:
                print(f"Importing {selected.name} / {mode}", flush=True)
                report = build(args.ssx3_iso, args.tricky_iso, output, selected=selected, mode=mode)
                rename_world(output, world_name(selected, mode))
                manifest.write_text(json.dumps({"importer": fingerprint, "world": report}, indent=2), encoding="utf-8")
                print(f"  {report['archive_bytes']} bytes; {report['world_texture_slots']} texture slots; original Tricky sky", flush=True)
            results.append({"course": selected.id, "mode": mode, "archive": str(final),
                            "texture_slots": report["world_texture_slots"]})
    if not args.worlds_only:
        from tricky_ui_assets import prepare
        prepare(args.tricky_iso, args.overlay, args.ssx3_iso)
        (args.overlay / "custom-tracks.ready").write_text(
            json.dumps({"version": 2, "tracks": results}, indent=2), encoding="utf-8")
    print(f"Prepared {len(results)} course/mode archives", flush=True)


if __name__ == "__main__":
    main()
