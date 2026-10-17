#!/usr/bin/env python3
"""Serve crates/ssx3-web/www on http://localhost:8000 (module workers need
http, not file://).

On the LAN: `serve.py --lan --cert cert.pem --key key.pem` serves HTTPS on
all interfaces. Browsers allow WebGPU and SharedArrayBuffer only on secure
pages, and http:// on a LAN address is not one. The disc image is never
served: each browser picks its own copy."""
import argparse, functools, http.server, os, socket, ssl, threading

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

class V6(http.server.ThreadingHTTPServer):
    address_family = socket.AF_INET6

    def server_bind(self):
        # --lan: one dual-stack socket for IPv4 and IPv6.
        self.socket.setsockopt(socket.IPPROTO_IPV6, socket.IPV6_V6ONLY, 0 if a.lan else 1)
        super().server_bind()

# Browsers may resolve localhost to ::1 or to 127.0.0.1: listen on both.
servers = [V6(("::" if a.lan else "::1", a.port), functools.partial(H, directory=root))]
if not a.lan:
    servers.append(http.server.ThreadingHTTPServer(("127.0.0.1", a.port), functools.partial(H, directory=root)))
if a.cert:
    ctx = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
    ctx.load_cert_chain(a.cert, a.key)
    for srv in servers:
        srv.socket = ctx.wrap_socket(srv.socket, server_side=True)
print(f"serving {root} on {'https' if a.cert else 'http'}://{'0.0.0.0' if a.lan else 'localhost'}:{a.port}")
for srv in servers[1:]:
    threading.Thread(target=srv.serve_forever, daemon=True).start()
servers[0].serve_forever()
