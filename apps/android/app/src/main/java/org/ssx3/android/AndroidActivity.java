package org.ssx3.android;

import android.app.NativeActivity;
import android.os.Bundle;
import android.media.AudioManager;
import android.media.AudioAttributes;
import android.media.AudioFocusRequest;
import android.content.Intent;

/** Java notifications never wait on Rust or the audio owner. */
public final class AndroidActivity extends NativeActivity {
    static { System.loadLibrary("ssx3_android"); }
    private boolean foreground, focused, audioFocus, destroyed, restarting, audioEnabled;
    private AudioManager audio;
    private AudioFocusRequest request;
    private native void nativeState(int bits);
    private void publish() {
        nativeState((foreground ? 1 : 0) | (focused ? 2 : 0)
            | (audioFocus ? 4 : 0) | (destroyed ? 8 : 0));
    }
    private final AudioManager.OnAudioFocusChangeListener listener = change -> {
        if (destroyed) return;
        audioFocus = change == AudioManager.AUDIOFOCUS_GAIN;
        publish();
    };
    @Override public void onCreate(Bundle state) {
        super.onCreate(state);
        audio = getSystemService(AudioManager.class);
        request = new AudioFocusRequest.Builder(AudioManager.AUDIOFOCUS_GAIN)
            .setAudioAttributes(new AudioAttributes.Builder().setUsage(AudioAttributes.USAGE_GAME)
                .setContentType(AudioAttributes.CONTENT_TYPE_MUSIC).build())
            .setWillPauseWhenDucked(true).setOnAudioFocusChangeListener(listener).build();
    }
    private void abandon() {
        audioFocus = false;
        publish();
        if (audio != null && request != null) audio.abandonAudioFocusRequest(request);
    }
    private void requestAudio() {
        if (!audioEnabled || destroyed || !foreground || !focused || request == null) return;
        audioFocus = audio.requestAudioFocus(request) == AudioManager.AUDIOFOCUS_REQUEST_GRANTED;
        publish();
    }
    public void enableAudio() {
        runOnUiThread(() -> { if (!destroyed) { audioEnabled = true; requestAudio(); } });
    }
    public void playing() {
        runOnUiThread(() -> {
            if (!destroyed) {
                getWindow().addFlags(android.view.WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
                android.util.Log.i("SSX3", "Native playback confirmed");
            }
        });
    }
    public void paused() {
        runOnUiThread(() -> {
            getWindow().clearFlags(android.view.WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
            android.util.Log.i("SSX3", "Native pause confirmed");
        });
    }
    // Keep the guest alive; normal focus/surface callbacks pause playback.
    // The manifest routes button and gesture Back through this API on Android 13+.
    @SuppressWarnings("deprecation")
    @Override public void onBackPressed() {
        moveTaskToBack(true);
    }
    @Override public void onResume() {
        super.onResume(); foreground = true; publish(); requestAudio();
    }
    @Override public void onPause() {
        foreground = false; abandon(); super.onPause();
    }
    @Override public void onWindowFocusChanged(boolean value) {
        super.onWindowFocusChanged(value);
        focused = value;
        if (value) { publish(); requestAudio(); } else abandon();
    }
    @Override public void recreate() {
        // Neither winit nor all guest globals support a second session in
        // the same process. Restart from saved data in a fresh process.
        restart();
        finish();
    }
    private void restart() {
        if (restarting) return;
        restarting = true;
        startActivity(new Intent(this, LauncherActivity.class)
            .putExtra("restart", true).addFlags(Intent.FLAG_ACTIVITY_CLEAR_TOP | Intent.FLAG_ACTIVITY_SINGLE_TOP));
    }
    @Override public void onDestroy() {
        if (isChangingConfigurations()) restart();
        destroyed = true;
        abandon();
        // Publish BEFORE super: NativeActivity waits for android_main here.
        // Rust closes audio asynchronously, exits its event loop and ends
        // this dedicated process. The launcher cannot share this process.
        super.onDestroy();
    }
}
