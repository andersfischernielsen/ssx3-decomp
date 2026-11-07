package org.ssx3.android;

import static org.junit.Assert.*;
import static org.robolectric.Shadows.shadowOf;

import org.junit.Test;
import org.junit.runner.RunWith;
import org.robolectric.Robolectric;
import org.robolectric.RobolectricTestRunner;
import org.robolectric.annotation.Config;
import org.robolectric.annotation.Implementation;
import org.robolectric.annotation.Implements;
import org.robolectric.shadow.api.Shadow;
import org.robolectric.shadows.ShadowActivity;

@RunWith(RobolectricTestRunner.class)
@Config(sdk = 33, shadows = AndroidBackTest.GameActivityShadow.class)
public class AndroidBackTest {
    @Implements(AndroidActivity.class)
    public static class GameActivityShadow extends ShadowActivity {
        int moves;
        boolean nonRoot;

        // This test calls the real Back override without loading the native game.
        @Implementation protected static void __staticInitializer__() {}

        @Implementation protected boolean moveTaskToBack(boolean nonRoot) {
            this.nonRoot = nonRoot;
            moves++;
            return true;
        }
    }

    @SuppressWarnings("deprecation")
    @Test public void backBackgroundsInsteadOfFinishingOrStartingAnotherActivity() {
        AndroidActivity activity = Robolectric.buildActivity(AndroidActivity.class).get();
        GameActivityShadow game = Shadow.extract(activity);
        activity.onBackPressed();
        activity.onBackPressed();
        assertEquals(2, game.moves);
        assertTrue(game.nonRoot);
        assertFalse(activity.isFinishing());
        assertNull(shadowOf(activity).getNextStartedActivity());
    }
    @Test public void nativeBackPostsBackgroundingToTheUiThread() throws Exception {
        AndroidActivity activity = Robolectric.buildActivity(AndroidActivity.class).get();
        GameActivityShadow game = Shadow.extract(activity);
        Thread nativeInput = new Thread(activity::background);
        nativeInput.start();
        nativeInput.join();
        assertEquals(0, game.moves);
        shadowOf(android.os.Looper.getMainLooper()).idle();
        assertEquals(1, game.moves);
        assertTrue(game.nonRoot);
        assertFalse(activity.isFinishing());
        assertNull(shadowOf(activity).getNextStartedActivity());
    }

}
