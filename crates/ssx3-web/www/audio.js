// The game's sound (interleaved stereo, 48 kHz, i16), sent by the page as
// the worker produces it. The game is not in lock step with the sound card:
// at most ~200 ms is kept (older samples are dropped), and after running dry
// playback waits for ~50 ms before starting again.
const MAX = 48000 / 5;
const START = 48000 / 20;

class Sound extends AudioWorkletProcessor {
  constructor() {
    super();
    this.chunks = [];
    this.pos = 0; // in the first chunk, in samples
    this.frames = 0; // buffered
    this.waiting = true;
    this.port.onmessage = (e) => {
      this.chunks.push(e.data);
      this.frames += e.data.length / 2;
      while (this.frames > MAX && this.chunks.length > 1) {
        const c = this.chunks.shift();
        this.frames -= (c.length - this.pos) / 2;
        this.pos = 0;
      }
    };
  }

  process(_inputs, outputs) {
    const [l, r] = outputs[0];
    if (this.waiting && this.frames < START) return true;
    this.waiting = false;
    for (let i = 0; i < l.length; i++) {
      const c = this.chunks[0];
      if (!c) {
        this.waiting = true;
        l.fill(0, i);
        r.fill(0, i);
        break;
      }
      l[i] = c[this.pos] / 32768;
      r[i] = c[this.pos + 1] / 32768;
      this.pos += 2;
      this.frames--;
      if (this.pos >= c.length) {
        this.chunks.shift();
        this.pos = 0;
      }
    }
    return true;
  }
}

registerProcessor("ssx3-sound", Sound);
