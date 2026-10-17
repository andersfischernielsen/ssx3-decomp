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

Not yet supported: sound, the memory card, gamepads, the settings overlay,
interpolation and texture packs.

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
