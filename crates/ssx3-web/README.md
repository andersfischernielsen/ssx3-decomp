# ssx3-web: SSX 3 in the browser

The recompiled game as WebAssembly, drawn with WebGPU. Experimental.

## How it works

- **EE code.** It uses the unwind form of the generated code
  (`recomp-ee --unwind`). That form runs on one host thread: a thread switch
  unwinds to the dispatcher and does not block a host thread. The default
  host-thread form cannot run in a browser.
- **Worker.** The game runs in a Web Worker (`www/worker.js`), on a
  `requestAnimationFrame` loop paced to 59.94 game frames per second.
- **Disc image.** The page hands over the disc image as a `File`. The worker
  reads byte ranges from it synchronously with `FileReaderSync`, which only
  exists in workers, through the `ssx3_core::iso::set_opener` hook. The
  executable comes from the disc as well. Nothing is uploaded.
- **Rendering.** `ssx3-gfx` draws into an `OffscreenCanvas` with WebGPU, at
  the canvas's pixel size. With `cfg(target_arch = "wasm32")`, the renderer
  and machine do the following without threads or blocking:
  - the VU1 calls run inline;
  - the frame snapshot is a single copy;
  - the GS has no MTGS thread;
  - the light-occlusion depth readback is used only once it is mapped;
  - the device is created with `gpu::init_context().await`.
- **Input.** Keyboard only, with the native window's keys:
  - arrows: D-pad;
  - Enter: Start;
  - Right Shift: Select;
  - X: Cross, S: Circle, Z: Square, A: Triangle;
  - Q: L1, W: R1, 1: L2, 2: R2.

- **VU helpers.** The renderer's VU1 calls, the native VU programs of
  `ssx3-render`, run in chunks on helper workers (`www/vuworker.js`,
  `crates/ssx3-vuworker`, a 180 KB module), as they run on threads natively.
  - Jobs and results are bytes (`ssx3_render::vujob`), passed through a
    SharedArrayBuffer job queue.
  - The game worker takes jobs too, then waits with `Atomics.wait`.
  - This needs cross-origin isolation, which `serve.py` sends the headers
    for. Without it, everything runs in the game worker.
  - `SSX3_VU_OFFLOAD_SELFTEST=1` on the native `ssx3-headless` runs every
    chunk through the same encode/run/decode path; the screenshots are
    identical.

- **Sound.** The native mixer's 48 kHz stereo goes from the worker to the
  page each frame and plays through an AudioWorklet (`www/audio.js`), which
  buffers at most ~200 ms.
- **Image.** The game's 4:3 view boxed in the canvas, rendered at the box's
  pixel size (the render scale costs little: the time goes to the CPU
  side), no anisotropic filtering.

Not yet supported: the memory card, gamepads, the settings overlay,
interpolation and texture packs.

## Performance (headless Chrome, M-series Mac)

- **Menus and front end:** 60 fps.
- **Races:** about 55-60 fps once under way (17 ms per frame), about 33 fps
  in the heaviest stretch at the start. Per race frame, on the game worker:
  - about half is game code;
  - the renderer: building the typed frame, the GS draws from the VU1
    output and the WebGPU calls, a few ms each.
- **Frame staging.** A frame's VU1 chunks start on the helper workers when
  the game publishes it, and the frame is drawn when the next one is
  published (`Inner::publish_staged`). The game's next frame runs while the
  helpers work, which hides the largest chunk (11-17 ms; a run of draws of
  one program that pass state through VU memory cannot be split). One frame
  of latency, as with the native render thread.
- **Light queries.** The depth read back for the lens-flare queries is
  converted per pixel when a query reads it, not as a whole every frame.

Next steps, roughly by payoff:
- split that chunk. Measured, by running each call from the true memory and
  from the inputs-only image a split would start from:
  - the rider program's drawing calls all read the bone matrices its setup
    call computes;
  - the world-mesh program's second pass (0x22c8) reads the first pass's
    output, which is fine since both are in one draw;
  - but 1.6% of its first passes (0x2270) also read state an earlier call
    wrote (probably the fan tag toggle at qw33.w).

  So the world-mesh runs could be split if that state is identified and
  seeded at each split;
- run the game and the renderer in different workers;
- a wasm-SIMD version of the PS2 float helpers (the NEON path needs
  per-lane shifts and count-leading-zeros, which wasm SIMD lacks).

## Building

```sh
# once: the unwind form of the EE code, and wasm-bindgen matching Cargo.lock
./target/release/recomp-ee --unwind --out generated/ee-unwind --package-prefix ssx3-ee-unwind
cargo install wasm-bindgen-cli --version 0.2.129 --locked

crates/ssx3-web/build.sh      # → crates/ssx3-web/www/pkg
crates/ssx3-web/serve.py      # http://localhost:8000
```

Open the page in a browser with WebGPU, for example Chrome or Edge 113+, or
Safari 26+. Then choose the `SSX 3 (USA)` ISO.

To check the unwind form natively, without a browser:

```sh
CARGO_TARGET_DIR=target-unwind cargo build --profile game -p ssx3 --bin ssx3-headless --no-default-features --features ee-unwind
```
