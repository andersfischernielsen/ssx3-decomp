package org.ssx3.android;

import android.app.Activity;
import android.app.ActivityManager;
import android.content.Intent;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.os.SystemClock;
import android.widget.Button;

/** Owns no native state. A replacement session starts only after the old
 * game process disappears; a timeout stays visible instead of reusing it. */
public final class LauncherActivity extends Activity {
    private final Handler handler = new Handler(Looper.getMainLooper());
    private Button button;
    private long deadline;
    private boolean waiting;
    private final Runnable waitForExit = new Runnable() {
        @Override public void run() {
            if (!waiting || isFinishing()) return;
            boolean alive = false;
            java.util.List<ActivityManager.RunningAppProcessInfo> processes =
                getSystemService(ActivityManager.class).getRunningAppProcesses();
            if (processes == null) {
                fail("Cannot confirm the previous session ended. Close and reopen the app.");
                return;
            }
            for (ActivityManager.RunningAppProcessInfo process : processes) {
                if (process.uid == android.os.Process.myUid()
                    && process.processName.equals(getPackageName() + ":game")) alive = true;
            }
            if (!alive) { waiting = false; launch(); }
            else if (SystemClock.uptimeMillis() >= deadline) {
                fail("Previous game session did not stop. Close and reopen the app.");
            } else handler.postDelayed(this, 25);
        }
    };
    @Override public void onCreate(Bundle state) {
        super.onCreate(state);
        button = new Button(this);
        button.setText("Start SSX 3");
        button.setOnClickListener(view -> launch());
        setContentView(button);
        accept(getIntent(), state == null);
    }
    @Override protected void onNewIntent(Intent intent) {
        super.onNewIntent(intent); setIntent(intent); accept(intent, true);
    }
    private void accept(Intent intent, boolean initial) {
        if (intent.getBooleanExtra("restart", false)) {
            intent.removeExtra("restart");
            waiting = true;
            deadline = SystemClock.uptimeMillis() + 2500;
            button.setEnabled(false); button.setText("Restarting game from saved data…");
            handler.post(waitForExit);
        } else if (initial) launch();
    }
    private void launch() {
        button.setEnabled(true); button.setText("Start SSX 3");
        startActivity(new Intent(this, AndroidActivity.class));
    }
    private void fail(String message) {
        waiting = false; button.setText(message); button.setEnabled(false);
    }
    @Override protected void onDestroy() {
        waiting = false; handler.removeCallbacks(waitForExit); super.onDestroy();
    }
}
