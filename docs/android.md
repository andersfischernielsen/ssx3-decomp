# Android developer build

Android 13 or newer, arm64-v8a. This is a NativeActivity frontend using the
upstream game and graphics defaults. Physical controllers are not supported
by this change; use a keyboard with the desktop key bindings.

Install JDK 17, Android SDK 34/build tools 34.0.0 and NDK 28.2.13676358,
Rust with the aarch64-linux-android target, cargo-ndk 4.1.2 and Gradle 8.9.
Set ANDROID_HOME and ANDROID_NDK_HOME to those SDK/NDK installations.
Generate the game code using the repository's normal setup with your own
SSX 3 USA disc. From apps/android, run `gradle assembleDebug`.
The Gradle build invokes cargo-ndk, and packages code only. Use
`-PapplicationId=your.application.id` for a separate developer install.
Keep the same debug signing key when upgrading; install with `adb install -r`.

The pack is separate. Produce it with `python3 tools/bundle.py --iso /path/to/your.iso pack /path/to/ssx3.pack`. Launch once to create app storage. Copy to the app-specific external
files directory as ssx3.pack.partial, compare its SHA-256 (`sha256sum ssx3.pack` on the host and
`adb shell sha256sum <external-files>/ssx3.pack.partial` on the device), then rename to
ssx3.pack. Never create the external app directory with adb shell; the app
creates it. The NeedsData screen reports invalid or missing data and starts
no game or audio work. Restart after provisioning.

Saves and settings live in internal app files. There is no storage permission
and no automatic fallback or save import. Uninstalling deletes app saves.
Do not replace a card when provisioning game data.

Backgrounding and surface replacement retain the current game. Launcher and
component-only re-entry ask a live game Activity to bring its separate task
forward. If the game is finishing, the launcher waits for its process to exit
before starting a new session. Successful forwarding closes the launcher. An
unexpected duplicate is finished before guest initialization or winit startup;
its Java notifications cannot modify the live session. Destroying
or explicitly recreating the Activity ends the dedicated game process;
the launcher waits for that process to exit before starting a cold session
from saved data. Unsaved progress does not survive that boundary. This
avoids reusing process-global guest state or recreating a winit event loop.

Host lifecycle tests run through Cargo: `tools/android/test-lifecycle.sh`.
Launcher intent-permutation and session-claim tests run on the JVM with
`gradle -p apps/android :app:testDebugUnitTest -x :app:buildRust` (JDK/SDK as
above; no generated guest code needed). These check forwarding and the manifest;
real task reuse, native duplicate rejection and lifecycle timing still need a device.
