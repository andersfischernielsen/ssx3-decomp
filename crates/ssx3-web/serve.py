#!/usr/bin/env python3
"""Serve crates/ssx3-web/www on http://localhost:8000 (module workers need
http, not file://).

On the LAN: `serve.py --lan --cert cert.pem --key key.pem` serves HTTPS on
all interfaces. Browsers allow WebGPU and SharedArrayBuffer only on secure
pages, and http:// on a LAN address is not one. The disc image is never
served: each browser picks its own copy."""
import argparse, functools, http.server, os, ssl

class H(http.server.SimpleHTTPRequestHandler):
    extensions_map = {**http.server.SimpleHTTPRequestHandler.extensions_map, ".wasm": "application/wasm", ".js": "text/javascript"}

    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        # Cross-origin isolation: SharedArrayBuffer for the VU helpers.
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        super().end_headers()

ap = argparse.ArgumentParser()
ap.add_argument("--lan", action="store_true", help="listen on all interfaces")
ap.add_argument("--port", type=int, default=8000)
ap.add_argument("--cert", help="TLS certificate (PEM); serves HTTPS")
ap.add_argument("--key", help="TLS private key (PEM)")
a = ap.parse_args()

root = os.path.join(os.path.dirname(os.path.abspath(__file__)), "www")
srv = http.server.ThreadingHTTPServer(("0.0.0.0" if a.lan else "127.0.0.1", a.port), functools.partial(H, directory=root))
if a.cert:
    ctx = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
    ctx.load_cert_chain(a.cert, a.key)
    srv.socket = ctx.wrap_socket(srv.socket, server_side=True)
print(f"serving {root} on {'https' if a.cert else 'http'}://{srv.server_address[0]}:{a.port}")
srv.serve_forever()
