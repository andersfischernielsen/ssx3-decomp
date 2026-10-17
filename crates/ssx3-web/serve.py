#!/usr/bin/env python3
"""Serve crates/ssx3-web/www on http://localhost:8000 (module workers need
http, not file://)."""
import functools, http.server, os

class H(http.server.SimpleHTTPRequestHandler):
    extensions_map = {**http.server.SimpleHTTPRequestHandler.extensions_map, ".wasm": "application/wasm", ".js": "text/javascript"}

    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        # Cross-origin isolation: SharedArrayBuffer for the VU helpers.
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        super().end_headers()

root = os.path.join(os.path.dirname(os.path.abspath(__file__)), "www")
http.server.ThreadingHTTPServer(("127.0.0.1", 8000), functools.partial(H, directory=root)).serve_forever()
