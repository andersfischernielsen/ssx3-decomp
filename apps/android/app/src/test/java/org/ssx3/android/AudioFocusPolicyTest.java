package org.ssx3.android;

import static org.junit.Assert.*;
import android.media.AudioManager;
import org.junit.Test;

public class AudioFocusPolicyTest {
    private AudioFocusPolicy playing() {
        AudioFocusPolicy policy = new AudioFocusPolicy();
        policy.requested(true);
        assertEquals(7, policy.bits(true, true, false));
        return policy;
    }
    @Test public void everyAudioFocusLossKeepsPlayBitsAndOnlyDropsAudio() {
        for (int loss : new int[] {AudioManager.AUDIOFOCUS_LOSS_TRANSIENT,
                AudioManager.AUDIOFOCUS_LOSS_TRANSIENT_CAN_DUCK, AudioManager.AUDIOFOCUS_LOSS}) {
            AudioFocusPolicy policy = playing();
            policy.changed(loss);
            assertEquals(3, policy.bits(true, true, false));
        }
    }
    @Test public void transientAndDuckLossReturnOnGain() {
        for (int loss : new int[] {AudioManager.AUDIOFOCUS_LOSS_TRANSIENT,
                AudioManager.AUDIOFOCUS_LOSS_TRANSIENT_CAN_DUCK}) {
            AudioFocusPolicy policy = playing();
            policy.changed(loss);
            policy.changed(AudioManager.AUDIOFOCUS_GAIN);
            assertEquals(7, policy.bits(true, true, false));
        }
    }
    @Test public void permanentLossReturnsOnTheNextGrantedRequestWithoutAKey() {
        AudioFocusPolicy policy = playing();
        policy.changed(AudioManager.AUDIOFOCUS_LOSS);
        policy.requested(false);
        assertEquals(3, policy.bits(true, true, false));
        policy.requested(true);
        assertEquals(7, policy.bits(true, true, false));
    }
    @Test public void gainWhileBackgroundedCannotEnablePlayback() {
        AudioFocusPolicy policy = playing();
        policy.changed(AudioManager.AUDIOFOCUS_LOSS_TRANSIENT);
        policy.abandon();
        policy.changed(AudioManager.AUDIOFOCUS_GAIN);
        assertNotEquals(7, policy.bits(false, true, false));
        assertNotEquals(7, policy.bits(true, false, false));
        assertNotEquals(7, policy.bits(true, true, true));
    }
}
