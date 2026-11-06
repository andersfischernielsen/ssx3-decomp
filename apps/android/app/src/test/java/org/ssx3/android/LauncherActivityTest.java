package org.ssx3.android;

import static org.junit.Assert.*;
import static org.robolectric.Shadows.shadowOf;

import android.app.Activity;
import android.app.ActivityManager;
import android.app.Application;
import android.content.BroadcastReceiver;
import android.content.ComponentName;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.content.pm.ActivityInfo;
import android.content.pm.PackageManager;
import android.os.Bundle;
import android.os.Looper;
import java.time.Duration;
import java.util.Arrays;
import java.util.Collections;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.robolectric.Robolectric;
import org.robolectric.RobolectricTestRunner;
import org.robolectric.RuntimeEnvironment;
import org.robolectric.annotation.Config;
import org.robolectric.annotation.LooperMode;
import org.robolectric.android.controller.ActivityController;

@RunWith(RobolectricTestRunner.class)
@Config(sdk = 33)
@LooperMode(LooperMode.Mode.PAUSED)
public class LauncherActivityTest {
    private Application app;

    @Before public void noGame() {
        app = RuntimeEnvironment.getApplication();
        processes(false);
    }

    private void processes(boolean alive) {
        // The launcher itself is alive even when the separate game is not.
        // Robolectric maps an empty list to null (enumeration unavailable).
        ActivityManager.RunningAppProcessInfo launcher = new ActivityManager.RunningAppProcessInfo();
        launcher.processName = app.getPackageName();
        launcher.uid = android.os.Process.myUid();
        ActivityManager.RunningAppProcessInfo game = new ActivityManager.RunningAppProcessInfo();
        game.processName = app.getPackageName() + ":game";
        game.uid = android.os.Process.myUid();
        shadowOf(app.getSystemService(ActivityManager.class)).setProcesses(
            alive ? Arrays.asList(launcher, game) : Collections.singletonList(launcher));
        assertNotNull(app.getSystemService(ActivityManager.class).getRunningAppProcesses());
    }

    private Intent incoming(boolean launcher) {
        Intent intent = new Intent(app, LauncherActivity.class);
        if (launcher) {
            intent.setAction(Intent.ACTION_MAIN).addCategory(Intent.CATEGORY_LAUNCHER)
                .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK | Intent.FLAG_ACTIVITY_RESET_TASK_IF_NEEDED);
        }
        return intent;
    }

    private void assertForwarded(LauncherActivity activity) {
        Intent forwarded = shadowOf(activity).getNextStartedActivity();
        assertNotNull(forwarded);
        assertEquals(new ComponentName(activity, AndroidActivity.class), forwarded.getComponent());
        assertEquals(Intent.FLAG_ACTIVITY_NEW_TASK | Intent.FLAG_ACTIVITY_CLEAR_TOP
            | Intent.FLAG_ACTIVITY_SINGLE_TOP, forwarded.getFlags());
        assertNull(forwarded.getAction());
        assertNull(forwarded.getCategories());
        assertNull(shadowOf(activity).getNextStartedActivity());
        assertTrue(activity.isFinishing());
    }

    private void cold(boolean launcher) {
        try (ActivityController<LauncherActivity> controller =
                Robolectric.buildActivity(LauncherActivity.class, incoming(launcher)).create()) {
            assertForwarded(controller.get());
        }
    }

    @Test public void unavailableProcessListDoesNotStartAnotherGame() {
        shadowOf(app.getSystemService(ActivityManager.class)).setProcesses(Collections.emptyList());
        assertNull(app.getSystemService(ActivityManager.class).getRunningAppProcesses());
        try (ActivityController<LauncherActivity> controller =
                Robolectric.buildActivity(LauncherActivity.class, incoming(true)).create()) {
            LauncherActivity activity = controller.get();
            shadowOf(Looper.getMainLooper()).idleFor(Duration.ofSeconds(3));
            assertNull(shadowOf(activity).getNextStartedActivity());
            assertFalse(activity.isFinishing());
            android.widget.TextView message = (android.widget.TextView)
                ((android.view.ViewGroup) activity.findViewById(android.R.id.content)).getChildAt(0);
            assertTrue(message.getText().toString().startsWith("Cannot confirm"));
        }
    }

    @Test public void coldComponent() { cold(false); }
    @Test public void coldLauncher() { cold(true); }

    @Test public void restoredLauncherStillForwards() {
        try (ActivityController<LauncherActivity> controller =
                Robolectric.buildActivity(LauncherActivity.class, incoming(true)).create(new Bundle())) {
            assertForwarded(controller.get());
        }
    }

    @Test public void dyingGameWaitsForExitThenStartsOnce() {
        processes(true);
        try (ActivityController<LauncherActivity> controller =
                Robolectric.buildActivity(LauncherActivity.class, incoming(true)).create()) {
            LauncherActivity activity = controller.get();
            shadowOf(Looper.getMainLooper()).idleFor(Duration.ofMillis(900));
            assertNull(shadowOf(activity).getNextStartedActivity());
            assertFalse(activity.isFinishing());
            processes(false);
            shadowOf(Looper.getMainLooper()).idleFor(Duration.ofMillis(25));
            assertForwarded(activity);
            shadowOf(Looper.getMainLooper()).idleFor(Duration.ofSeconds(3));
            assertNull(shadowOf(activity).getNextStartedActivity());
        }
    }

    private void live(boolean cold, boolean reentry) {
        processes(true);
        BroadcastReceiver game = new BroadcastReceiver() {
            @Override public void onReceive(Context context, Intent intent) {
                // Models the live Activity moving its own task forward.
                setResultCode(Activity.RESULT_OK);
            }
        };
        app.registerReceiver(game, new IntentFilter(app.getPackageName() + LauncherActivity.RETURN_TO_GAME),
            Context.RECEIVER_NOT_EXPORTED);
        try (ActivityController<LauncherActivity> controller =
                Robolectric.buildActivity(LauncherActivity.class, incoming(cold)).create()) {
            LauncherActivity activity = controller.get();
            activity.onNewIntent(incoming(reentry));
            shadowOf(Looper.getMainLooper()).idle();
            assertNull(shadowOf(activity).getNextStartedActivity());
            assertTrue(activity.isFinishing());
        } finally {
            app.unregisterReceiver(game);
        }
    }

    @Test public void componentThenComponent() { live(false, false); }
    @Test public void componentThenLauncher() { live(false, true); }
    @Test public void launcherThenComponent() { live(true, false); }
    @Test public void launcherThenLauncher() { live(true, true); }

    @Test public void reentryCannotBypassPendingRestart() {
        processes(true);
        try (ActivityController<LauncherActivity> controller = Robolectric.buildActivity(
                LauncherActivity.class, incoming(true).putExtra("restart", true)).create()) {
            LauncherActivity activity = controller.get();
            activity.onNewIntent(incoming(false));
            activity.onNewIntent(incoming(true));
            shadowOf(Looper.getMainLooper()).idleFor(Duration.ofMillis(100));
            assertNull(shadowOf(activity).getNextStartedActivity());
            processes(false);
            shadowOf(Looper.getMainLooper()).idleFor(Duration.ofMillis(25));
            assertForwarded(activity);
        }
    }

    @Test public void unresponsiveGameNeverGetsASecondActivity() {
        processes(true);
        try (ActivityController<LauncherActivity> controller =
                Robolectric.buildActivity(LauncherActivity.class, incoming(true)).create()) {
            shadowOf(Looper.getMainLooper()).idleFor(Duration.ofSeconds(3));
            assertNull(shadowOf(controller.get()).getNextStartedActivity());
            assertFalse(controller.get().isFinishing());
        }
    }

    @Test public void gameTaskCannotBeClearedByLauncherReentry() throws Exception {
        PackageManager pm = app.getPackageManager();
        ActivityInfo game = pm.getActivityInfo(new ComponentName(app, AndroidActivity.class), 0);
        ActivityInfo launcher = pm.getActivityInfo(new ComponentName(app, LauncherActivity.class), 0);
        assertEquals(ActivityInfo.LAUNCH_SINGLE_TASK, game.launchMode);
        assertEquals(ActivityInfo.LAUNCH_SINGLE_TASK, launcher.launchMode);
        assertEquals(app.getPackageName() + ".game", game.taskAffinity);
        assertNotEquals(launcher.taskAffinity, game.taskAffinity);
        assertEquals(app.getPackageName() + ":game", game.processName);
        assertFalse(game.exported);
        int handled = ActivityInfo.CONFIG_SMALLEST_SCREEN_SIZE | ActivityInfo.CONFIG_DENSITY
            | ActivityInfo.CONFIG_FONT_SCALE | ActivityInfo.CONFIG_LOCALE | ActivityInfo.CONFIG_LAYOUT_DIRECTION;
        assertEquals(handled, game.configChanges & handled);
    }

    @Test public void processClaimNeverReopens() {
        SessionClaim process = new SessionClaim();
        assertTrue(process.claim());
        for (int i = 0; i < 10; i++) assertFalse(process.claim());
        assertTrue(new SessionClaim().claim());
    }
}
