// The game runs here: FileReaderSync (synchronous disc reads) exists only
// in workers, and a long game frame never blocks the page.
import init, { start } from "./pkg/ssx3_web.js";

const FIELD_MS = 1000 / 59.94;
let game = null;
let buttons = 0;
let size = null;

onmessage = async (e) => {
  const m = e.data;
  if (m.type === "start") {
    try {
      await init();
      // VU helpers: the renderer's VU1 work in parallel (needs a shared
      // buffer, i.e. cross-origin isolation).
      // (`?helpers=N` on the page overrides the count.)
      const auto = self.crossOriginIsolated ? Math.max(1, Math.min(6, (navigator.hardwareConcurrency || 4) - 2)) : 0;
      const helpers = self.crossOriginIsolated && m.helpers != null ? m.helpers : auto;
      game = await start(m.canvas, m.iso, helpers);
      const sab = game.vu_buffer();
      for (let id = 0; sab && id < helpers; id++) {
        new Worker("./vuworker.js", { type: "module" }).postMessage({ sab, id });
      }
      postMessage({ type: "running" });
      loop();
    } catch (err) {
      postMessage({ type: "error", message: String(err && err.stack || err) });
    }
  } else if (m.type === "buttons") {
    buttons = m.buttons;
  } else if (m.type === "resize") {
    size = [m.width, m.height];
  }
};

function loop() {
  let next = performance.now();
  let frames = 0;
  let mark = next;
  const tick = (now) => {
    if (size) {
      game.resize(size[0], size[1]);
      size = null;
    }
    // Game frames due by now (at most 2 to catch up; further behind, the
    // game slows down instead of stuttering).
    let ran = 0;
    while (now >= next && ran < 2) {
      if (!game.frame(buttons)) {
        postMessage({ type: "error", message: "the game stopped" });
        return;
      }
      next += FIELD_MS;
      ran++;
      frames++;
    }
    if (now - next > 100) next = now;
    if (ran > 0) {
      game.present();
      const a = game.audio();
      if (a.length) postMessage({ type: "audio", samples: a }, [a.buffer]);
    }
    if (now - mark >= 1000) {
      postMessage({ type: "fps", fps: (frames * 1000) / (now - mark) });
      frames = 0;
      mark = now;
    }
    requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
}
