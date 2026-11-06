package org.ssx3.android;

import android.app.Activity;
import android.app.ActivityManager;
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.os.SystemClock;
import android.widget.TextView;

/** Starts a fresh session only after the old process exits. A live Activity
 * answers a probe by bringing its own task forward, without creating another. */
public final class LauncherActivity extends Activity {
    static final String RETURN_TO_GAME = ".RETURN_TO_GAME";
    private final Handler handler = new Handler(Looper.getMainLooper());
    private TextView message;
    private long deadline;
    private boolean waiting;
    private boolean restart;
    private boolean probing;
    private final Runnable waitForGame = new Runnable() {
        @Override public void run() {
            if (!waiting || isFinishing()) return;
            java.util.List<ActivityManager.RunningAppProcessInfo> processes =
                getSystemService(ActivityManager.class).getRunningAppProcesses();
            if (processes == null) {
                fail("Cannot confirm the previous session ended. Close and reopen the app.");
                return;
            }
            boolean alive = false;
            for (ActivityManager.RunningAppProcessInfo process : processes) {
                if (process.uid == android.os.Process.myUid()
                    && process.processName.equals(getPackageName() + ":game")) {
                    alive = true;
                    break;
                }
            }
            if (!alive) {
                launch();
                return;
            }
            if (SystemClock.uptimeMillis() >= deadline) {
                fail("Previous game session did not respond. Close and reopen the app.");
                return;
            }
            if (!restart && !probing) {
                probing = true;
                // Only our registered game receiver can answer. It refuses
                // while finishing; never start an Activity in a dying process.
                Intent probe = new Intent(getPackageName() + RETURN_TO_GAME)
                    .setPackage(getPackageName()).addFlags(Intent.FLAG_RECEIVER_FOREGROUND);
                sendOrderedBroadcast(probe, null, new BroadcastReceiver() {
                    @Override public void onReceive(Context context, Intent intent) {
                        probing = false;
                        if (waiting && !restart && getResultCode() == RESULT_OK) {
                            complete();
                        }
                    }
                }, handler, RESULT_CANCELED, null, null);
            }
            handler.postDelayed(this, 25);
        }
    };

    @Override public void onCreate(Bundle state) {
        super.onCreate(state);
        message = new TextView(this);
        message.setText("Opening game…");
        setContentView(message);
        accept(getIntent());
    }

    @Override protected void onNewIntent(Intent intent) {
        super.onNewIntent(intent);
        setIntent(intent);
        accept(intent);
    }

    private void accept(Intent intent) {
        restart |= intent.getBooleanExtra("restart", false);
        intent.removeExtra("restart");
        if (waiting || isFinishing()) return;
        waiting = true;
        deadline = SystemClock.uptimeMillis() + 2500;
        waitForGame.run();
    }

    private void launch() {
        startActivity(new Intent(this, AndroidActivity.class).addFlags(
            Intent.FLAG_ACTIVITY_NEW_TASK | Intent.FLAG_ACTIVITY_CLEAR_TOP | Intent.FLAG_ACTIVITY_SINGLE_TOP));
        complete();
    }

    private void complete() {
        waiting = false;
        handler.removeCallbacks(waitForGame);
        finish();
    }

    private void fail(String text) {
        waiting = false;
        handler.removeCallbacks(waitForGame);
        message.setText(text);
    }

    @Override protected void onDestroy() {
        waiting = false;
        handler.removeCallbacks(waitForGame);
        super.onDestroy();
    }
}
