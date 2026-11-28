#!/usr/bin/env python3
"""List the uses of generated bodies (`__generated`, `__entries`, `generated_body`)
in production code: everything outside `#[cfg(test)]` items.

    python3 tools/prod_generated.py [--count | --check]

`--check` exits 1 when production code (other than the reference tools listed
in ALLOWED) uses a generated body: CI runs it.
"""
import glob, os, re, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# Reference tools that compare a port with the generated body on purpose.
ALLOWED = {"crates/ssx3-game/src/shadow.rs"}


def strip_tests(text):
    """Blank out `#[cfg(test)]` items (mod or fn), keeping line numbers."""
    out = list(text)
    for m in re.finditer(r"#\[cfg\(test\)\]", text):
        i = m.end()
        j = text.find("{", i)
        k = text.find(";", i)
        if j < 0 or (0 <= k < j):
            end = k + 1 if k >= 0 else i
        else:
            d, p = 1, j + 1
            while d and p < len(text):
                c = text[p]
                if c == "{":
                    d += 1
                elif c == "}":
                    d -= 1
                p += 1
            end = p
        for x in range(m.start(), end):
            if out[x] != "\n":
                out[x] = " "
    return "".join(out)


def main():
    hits = []
    for pat in ("crates/ssx3-game/src/**/*.rs", "crates/ssx3-engine/src/**/*.rs", "crates/ssx3/src/**/*.rs"):
        for f in glob.glob(os.path.join(ROOT, pat), recursive=True):
            s = strip_tests(open(f).read())
            for n, line in enumerate(s.split("\n"), 1):
                if re.search(r"__generated|__entries|generated_body", line) and not line.lstrip().startswith("//"):
                    hits.append((f[len(ROOT) + 1:], n, line.strip()))
    if "--check" in sys.argv:
        bad = [h for h in hits if h[0] not in ALLOWED]
        for f, n, l in bad:
            print(f"{f}:{n}: {l[:140]}")
        sys.exit(1 if bad else 0)
    if "--count" in sys.argv:
        print(len(hits))
    else:
        for f, n, l in hits:
            print(f"{f}:{n}: {l[:140]}")


main()
