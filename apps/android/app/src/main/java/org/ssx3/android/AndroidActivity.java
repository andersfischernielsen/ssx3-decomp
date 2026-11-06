package org.ssx3.android;

import android.app.NativeActivity;
import android.app.ActivityManager;
import android.os.Bundle;
import android.media.AudioManager;
import android.media.AudioAttributes;
import android.media.AudioFocusRequest;
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;

/** Java notifications never wait on Rust or the audio owner. */
public final class AndroidActivity extends NativeActivity {
    static {
        System.loadLibrary("ssx3_android");
    }
    private static final SessionClaim SESSION = new SessionClaim();
    private volatile boolean sessionOwner;
    // Queried by android_main before bootstrap or installing global callbacks.
    public boolean ownsSession() {
        return sessionOwner;
    }
    private boolean foreground, focused, destroyed, restarting, audioEnabled;
    private final AudioFocusPolicy audioFocus = new AudioFocusPolicy();
    private AudioManager audio;
    private AudioFocusRequest request;
    private native void nativeState(int bits);
    private void publish() {
        if (!sessionOwner) return;
        nativeState(audioFocus.bits(foreground, focused, destroyed));
    }
    private final AudioManager.OnAudioFocusChangeListener listener = change -> {
        if (destroyed) return;
        audioFocus.changed(change);
        publish();
    };
    private final BroadcastReceiver returnToGame = new BroadcastReceiver() {
        @Override public void onReceive(Context context, Intent intent) {
            if (!sessionOwner || destroyed || isFinishing()) return;
            // Only move this existing task. Never create a replacement Activity
            // in the process that owns the current guest and winit session.
            for (ActivityManager.AppTask task : getSystemService(ActivityManager.class).getAppTasks()) {
                if (task.getTaskInfo().taskId == getTaskId()) {
                    task.moveToFront();
                    setResultCode(RESULT_OK);
                    return;
                }
            }
        }
    };
    @Override public void onStart() {
        long started = android.os.SystemClock.elapsedRealtime();
        android.util.Log.i("SSX3", "onStart entering native lifecycle");
        super.onStart();
        android.util.Log.i("SSX3", "onStart acknowledged after "
            + (android.os.SystemClock.elapsedRealtime() - started) + " ms");
    }
    @Override public void onCreate(Bundle state) {
        sessionOwner = SESSION.claim();
        // NativeActivity must initialize its handle even for a rejected Activity.
        // android_main returns immediately for it; stock android-activity finishes it.
        super.onCreate(state);
        if (!sessionOwner) {
            android.util.Log.w("SSX3", "Rejected duplicate game Activity");
            finish();
            return;
        }
        registerReceiver(returnToGame,
            new IntentFilter(getPackageName() + LauncherActivity.RETURN_TO_GAME), Context.RECEIVER_NOT_EXPORTED);
        audio = getSystemService(AudioManager.class);
        request = new AudioFocusRequest.Builder(AudioManager.AUDIOFOCUS_GAIN)
            .setAudioAttributes(new AudioAttributes.Builder().setUsage(AudioAttributes.USAGE_GAME)
                .setContentType(AudioAttributes.CONTENT_TYPE_MUSIC).build())
            .setWillPauseWhenDucked(true).setOnAudioFocusChangeListener(listener).build();
    }
    private void abandon() {
        audioFocus.abandon();
        publish();
        if (audio != null && request != null) audio.abandonAudioFocusRequest(request);
    }
    private void requestAudio() {
        if (!audioEnabled || destroyed || !foreground || !focused || request == null) return;
        audioFocus.requested(audio.requestAudioFocus(request) == AudioManager.AUDIOFOCUS_REQUEST_GRANTED);
        publish();
    }
    public void enableAudio() {
        runOnUiThread(() -> {
            if (!destroyed) {
                audioEnabled = true;
                requestAudio();
            }
        });
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
        super.onResume();
        foreground = true;
        publish();
        requestAudio();
    }
    @Override public void onPause() {
        foreground = false;
        abandon();
        super.onPause();
    }
    @Override public void onWindowFocusChanged(boolean value) {
        super.onWindowFocusChanged(value);
        focused = value;
        if (value) {
            publish();
            requestAudio();
        } else {
            abandon();
        }
    }
    private void restart() {
        if (restarting) return;
        restarting = true;
        startActivity(new Intent(this, LauncherActivity.class)
            .putExtra("restart", true).addFlags(Intent.FLAG_ACTIVITY_CLEAR_TOP | Intent.FLAG_ACTIVITY_SINGLE_TOP));
    }
    @Override public void onDestroy() {
        if (!sessionOwner) {
            super.onDestroy();
            return;
        }
        if (isChangingConfigurations()) restart();
        destroyed = true;
        abandon();
        unregisterReceiver(returnToGame);
        // Publish BEFORE super: NativeActivity waits for android_main here.
        // Rust closes audio asynchronously, exits its event loop and ends
        // this dedicated process. The launcher cannot share this process.
        super.onDestroy();
    }
}
