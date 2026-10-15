#!/usr/bin/env bash
# Watch running OpenCode delegate workers with titles matching $1 (e.g. "w5-").
# Exits (so the caller is notified) when: all workers finished, a log shows a
# token/quota/rate-limit/model error, or a log stopped growing for $2 seconds
# (default 900). Prints the reason and the affected logs.
pat="${1:?worker title prefix}"; stall="${2:-900}"
cd "$(git rev-parse --show-toplevel)/.delegate/logs"
state=$(mktemp -d)   # per-log "size since" (bash 3.2: no associative arrays)
while true; do
  running=$(ps -Ao args | grep -o "delegate:${pat}[A-Za-z0-9_-]*" | sort -u)
  [[ -z "$running" ]] && { echo "ALL DONE"; exit 0; }
  now=$(date +%s)
  for t in $running; do
    log="${t#delegate:}.log"; [[ -f $log ]] || continue
    if tail -c 4000 "$log" | grep -qiE 'rate.?limit|quota exceeded|insufficient (credit|balance|quota)|out of (tokens|credits)|model unavailable|(http|status|error)[^0-9]{0,3}429|too many requests|usage limit|billing'; then
      echo "ERROR in $log:"; tail -c 600 "$log" | grep -iE 'rate.?limit|quota|insufficient|out of|unavailable|429|too many|usage limit|billing' | tail -3; exit 2
    fi
    s=$(stat -f %z "$log")
    if [[ -f "$state/$log" ]]; then read -r old t0 < "$state/$log"; else old=""; t0=$now; fi
    if [[ "$old" != "$s" ]]; then echo "$s $now" > "$state/$log"
    elif (( now - t0 > stall )); then echo "STALLED $log (no output for $((now - t0)) s)"; exit 3; fi
  done
  sleep 60
done
