#!/bin/sh
# Build the reference VU interpreter harness (vu-ref) from a ps2-web-recomp
# checkout (MIT, https://github.com/snuri00/ps2-web-recomp).
#   tools/vu-diff/build.sh <ps2-web-recomp dir> <out dir>
# vu_micro.cpp is used unmodified except for one inserted line per pair loop:
# a call to the trace hook after `r.vi[0] = 0;`.
set -e
SRC=${1:?ps2-web-recomp dir}
OUT=${2:?out dir}
HERE=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$OUT"
sed 's/^\( *\)r\.vi\[0\] = 0;$/\1r.vi[0] = 0; vu_trace_hook(current, r);/' \
  "$SRC/runtime/src/vu_micro.cpp" > "$OUT/vu_micro_hooked.cpp"
test "$(grep -c vu_trace_hook "$OUT/vu_micro_hooked.cpp")" -ge 2
cat > "$OUT/hook.hpp" <<'EOF'
#pragma once
#include <cstdint>
namespace ps2 { struct Vu0Regs; }
void vu_trace_hook(std::uint32_t pc, const ps2::Vu0Regs &r);
EOF
clang++ -std=c++20 -O2 -w -include "$OUT/hook.hpp" \
  -I"$SRC/runtime/src" -I"$SRC/runtime/include" \
  "$OUT/vu_micro_hooked.cpp" "$HERE/harness.cpp" -o "$OUT/vu-ref"
echo "built $OUT/vu-ref"
