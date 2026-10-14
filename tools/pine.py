#!/usr/bin/env python3
"""Minimal PCSX2 PINE client (EE memory read/write, status, savestates).

PCSX2 must have PINE enabled (Settings -> Advanced) and a game booted; on
macOS/Linux the socket is $TMPDIR/pcsx2.sock (slot != 28011: pcsx2.sock.<slot>).

  tools/pine.py info
  tools/pine.py read32 0x4a19cc [count]
  tools/pine.py dump 0x100000 0x1000 out.bin
  tools/pine.py save|load <slot>
"""
import os, socket, struct, sys, tempfile

OP_READ = {1: 0, 2: 1, 4: 2, 8: 3}
OP_WRITE = {1: 4, 2: 5, 4: 6, 8: 7}
OP_VERSION, OP_SAVE, OP_LOAD, OP_TITLE, OP_ID, OP_UUID, OP_GAMEVER, OP_STATUS = 8, 9, 10, 11, 12, 13, 14, 15


class Pine:
    def __init__(self, slot=28011):
        base = os.environ.get("XDG_RUNTIME_DIR") or os.environ.get("TMPDIR") or tempfile.gettempdir()
        path = os.path.join(base, "pcsx2.sock" + ("" if slot == 28011 else f".{slot}"))
        self.s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.s.connect(path)

    def _recv(self, n):
        b = b""
        while len(b) < n:
            c = self.s.recv(n - len(b))
            if not c:
                raise IOError("PINE socket closed")
            b += c
        return b

    def batch(self, cmds):
        """cmds: list of raw command bytes. Returns the reply payload (after the status byte)."""
        body = b"".join(cmds)
        self.s.sendall(struct.pack("<I", len(body) + 4) + body)
        size = struct.unpack("<I", self._recv(4))[0]
        rest = self._recv(size - 4)
        if rest[0] != 0:
            raise IOError("PINE command failed")
        return rest[1:]

    def read(self, addr, width=4):
        r = self.batch([struct.pack("<BI", OP_READ[width], addr)])
        return int.from_bytes(r[:width], "little")

    def write(self, addr, value, width=4):
        self.batch([struct.pack("<BI", OP_WRITE[width], addr) + value.to_bytes(width, "little")])

    def read_block(self, addr, size, chunk=4096):
        """Read `size` bytes (multiple of 8) with batched 64-bit reads."""
        out = bytearray()
        for base in range(addr, addr + size, chunk):
            n = min(chunk, addr + size - base) // 8
            r = self.batch([struct.pack("<BI", 3, base + 8 * i) for i in range(n)])
            out += r[: 8 * n]
        return bytes(out)

    def _string(self, op):
        r = self.batch([bytes([op])])
        n = struct.unpack("<I", r[:4])[0]
        return r[4 : 4 + n].rstrip(b"\0").decode(errors="replace")

    def version(self): return self._string(OP_VERSION)
    def title(self): return self._string(OP_TITLE)
    def game_id(self): return self._string(OP_ID)
    def game_version(self): return self._string(OP_GAMEVER)
    def status(self):
        return {0: "running", 1: "paused", 2: "shutdown"}.get(struct.unpack("<I", self.batch([bytes([OP_STATUS])])[:4])[0], "?")
    def save_state(self, slot): self.batch([bytes([OP_SAVE, slot])])
    def load_state(self, slot): self.batch([bytes([OP_LOAD, slot])])


def main(a):
    p = Pine()
    cmd = a[0] if a else "info"
    if cmd == "info":
        print(p.version(), "|", p.title(), "|", p.game_id(), p.game_version(), "|", p.status())
    elif cmd.startswith("read"):
        w = int(cmd[4:]) // 8
        addr = int(a[1], 0)
        for i in range(int(a[2], 0) if len(a) > 2 else 1):
            print(f"{addr + i * w:08x}: {p.read(addr + i * w, w):0{2 * w}x}")
    elif cmd.startswith("write"):
        p.write(int(a[1], 0), int(a[2], 0), int(cmd[5:]) // 8)
    elif cmd == "dump":
        open(a[3], "wb").write(p.read_block(int(a[1], 0), int(a[2], 0)))
    elif cmd == "save":
        p.save_state(int(a[1]))
    elif cmd == "load":
        p.load_state(int(a[1]))
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main(sys.argv[1:])
