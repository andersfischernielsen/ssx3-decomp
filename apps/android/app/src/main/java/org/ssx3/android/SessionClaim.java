package org.ssx3.android;

/** Never released: guest globals and winit are only reset by process death. */
final class SessionClaim {
    private boolean claimed;
    synchronized boolean claim() {
        if (claimed) return false;
        claimed = true;
        return true;
    }
}
