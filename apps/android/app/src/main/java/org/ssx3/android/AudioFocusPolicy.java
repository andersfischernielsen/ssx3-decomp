package org.ssx3.android;

import android.media.AudioManager;

/**
 * UI-thread focus state. Losing audio focus only silences the game. A
 * transient loss ends with a gain; after a permanent loss Android sends no
 * gain, so the game asks again when its window next regains focus.
 */
final class AudioFocusPolicy {
    private boolean granted;
    void changed(int change) {
        if (change == AudioManager.AUDIOFOCUS_GAIN) {
            granted = true;
        } else if (change == AudioManager.AUDIOFOCUS_LOSS
                || change == AudioManager.AUDIOFOCUS_LOSS_TRANSIENT
                || change == AudioManager.AUDIOFOCUS_LOSS_TRANSIENT_CAN_DUCK) {
            granted = false; // Silence rather than duck.
        }
    }
    void abandon() {
        granted = false;
    }
    void requested(boolean value) {
        granted = value;
    }
    int bits(boolean foreground, boolean focused, boolean destroyed) {
        return (foreground ? 1 : 0) | (focused ? 2 : 0)
            | (granted ? 4 : 0) | (destroyed ? 8 : 0);
    }
}
