// A VU helper: runs chunks of the renderer's VU1 calls (ssx3-vuworker) from
// the shared job queue the game worker fills each frame (the layout is
// described in crates/ssx3-web/src/lib.rs, `vu_pool`).
import init, { run_job, store_reset, store_append } from "./pkg-vu/ssx3_vuworker.js";

onmessage = async ({ data: { sab, id } }) => {
  await init();
  const i32 = new Int32Array(sab);
  const u8 = new Uint8Array(sab);
  const outBase = i32[4], arena = i32[5];
  let seen = 0;
  // The store mirror: its generation and the bytes copied so far.
  let storeGen = 0, storeEnd = 0;
  for (;;) {
    Atomics.wait(i32, 0, seen);
    const gen = Atomics.load(i32, 0);
    seen = gen;
    const sg = Atomics.load(i32, 6), se = Atomics.load(i32, 7), sb = i32[8];
    if (sg !== storeGen) {
      store_reset();
      storeGen = sg;
      storeEnd = 0;
    }
    if (se > storeEnd) {
      store_append(u8.subarray(sb + storeEnd, sb + se));
      storeEnd = se;
    }
    const tag = (gen & 0x7fff) << 16;
    const n = Atomics.load(i32, 1);
    let out = outBase + id * arena;
    const end = out + arena;
    for (;;) {
      const v = Atomics.load(i32, 2);
      if ((v & ~0xffff) !== tag || (v & 0xffff) >= n) break;
      if (Atomics.compareExchange(i32, 2, v, v + 1) !== v) continue;
      const t = 16 + 4 * (v & 0xffff);
      const r = run_job(u8.subarray(i32[t], i32[t] + i32[t + 1]));
      if (out + r.length <= end) {
        u8.set(r, out);
        i32[t + 2] = out;
        i32[t + 3] = r.length;
        out += r.length;
      } else {
        i32[t + 2] = -1;
      }
      Atomics.add(i32, 3, 1);
      Atomics.notify(i32, 3);
    }
  }
};
