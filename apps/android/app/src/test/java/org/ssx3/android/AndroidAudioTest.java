package org.ssx3.android;

import static org.junit.Assert.*;
import static org.robolectric.Shadows.shadowOf;
import static org.robolectric.util.reflector.Reflector.reflector;

import android.app.Activity;
import android.app.NativeActivity;
import android.content.Intent;
import android.media.AudioManager;
import android.os.Bundle;
import android.os.Looper;
import java.util.ArrayList;
import java.util.List;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.robolectric.Robolectric;
import org.robolectric.RobolectricTestRunner;
import org.robolectric.annotation.Config;
import org.robolectric.annotation.Implementation;
import org.robolectric.annotation.Implements;
import org.robolectric.annotation.LooperMode;
import org.robolectric.annotation.RealObject;
import org.robolectric.shadow.api.Shadow;
import org.robolectric.shadows.ShadowActivity;
import org.robolectric.util.ReflectionHelpers;
import org.robolectric.util.reflector.Direct;
import org.robolectric.util.reflector.ForType;

/** Audio-only events reach the real game Activity; it must keep its play bits (1|2). */
@RunWith(RobolectricTestRunner.class)
@Config(sdk = 33, shadows = {AndroidAudioTest.NativeShadow.class, AndroidAudioTest.GameShadow.class})
@LooperMode(LooperMode.Mode.PAUSED)
public class AndroidAudioTest {
    @ForType(Activity.class)
    interface ActivityDirect {
        @Direct void onCreate(Bundle state);
    }
    // NativeActivity.onCreate loads the native library; keep only Activity's part.
    @Implements(NativeActivity.class)
    public static class NativeShadow extends ShadowActivity {
        @RealObject NativeActivity real;
        @Implementation protected void onCreate(Bundle state) {
            reflector(ActivityDirect.class, real).onCreate(state);
        }
    }
    @Implements(AndroidActivity.class)
    public static class GameShadow extends NativeShadow {
        final List<Integer> states = new ArrayList<>();
        @Implementation protected static void __staticInitializer__() {}
        @Implementation protected void nativeState(int bits) {
            states.add(bits);
        }
    }

    private AndroidActivity activity;
    private GameShadow game;
    private AudioManager.OnAudioFocusChangeListener listener;

    private int last() {
        return game.states.get(game.states.size() - 1);
    }
    @Before public void playingWithSound() {
        // The stubbed static initializer skips SESSION; each test is a fresh game process.
        ReflectionHelpers.setStaticField(AndroidActivity.class, "SESSION", new SessionClaim());
        activity = Robolectric.buildActivity(AndroidActivity.class).setup().get();
        game = Shadow.extract(activity);
        activity.onWindowFocusChanged(true);
        activity.enableAudio();
        shadowOf(Looper.getMainLooper()).idle();
        assertEquals("foreground, focused and audio focus granted", 7, last());
        listener = ReflectionHelpers.getField(activity, "listener");
    }
    @Test public void everyAudioFocusLossKeepsPlayingAndGainNeedsNoKey() {
        for (int loss : new int[] {AudioManager.AUDIOFOCUS_LOSS_TRANSIENT,
                AudioManager.AUDIOFOCUS_LOSS_TRANSIENT_CAN_DUCK}) {
            listener.onAudioFocusChange(loss);
            assertEquals(3, last());
            listener.onAudioFocusChange(AudioManager.AUDIOFOCUS_GAIN);
            assertEquals(7, last());
        }
        listener.onAudioFocusChange(AudioManager.AUDIOFOCUS_LOSS);
        assertEquals(3, last());
        assertFalse(activity.isFinishing());
    }
    @Test public void permanentLossAsksAgainWhenTheWindowRegainsFocus() {
        listener.onAudioFocusChange(AudioManager.AUDIOFOCUS_LOSS);
        assertEquals(3, last());
        activity.onWindowFocusChanged(false); // for example the notification shade
        assertEquals(1, last());
        activity.onWindowFocusChanged(true);
        assertEquals(7, last());
    }
    @Test public void headphoneUnplugNeitherPausesNorSilences() {
        int published = game.states.size();
        activity.sendBroadcast(new Intent(AudioManager.ACTION_AUDIO_BECOMING_NOISY));
        shadowOf(Looper.getMainLooper()).idle();
        assertEquals(published, game.states.size());
        assertEquals(7, last());
        assertNull(shadowOf(activity.getSystemService(AudioManager.class)).getLastAbandonedAudioFocusRequest());
    }
}
