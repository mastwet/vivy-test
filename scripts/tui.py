"""A small pseudo-terminal driver for checking terminal views (Unix only).

Terminal views redraw with cursor moves, so `seen` matches text with all
whitespace removed, in what arrived since a mark.
"""
import fcntl
import os
import pty
import re
import select
import signal
import struct
import termios
import time

ALT_UP = b"\x1b[1;3A"
ENTER = b"\r"
CTRL_C = b"\x03"
CTRL_D = b"\x04"


def plain(raw):
    """`raw` terminal output as text, escape sequences removed."""
    text = raw.decode("utf-8", "replace")
    return re.sub(r"\x1b\[[0-9;?<>=]*[a-zA-Z~]|\x1b\][^\x07\x1b]*(\x07|\x1b\\)|\x1b[()=>][A-Z0-9]?", "", text)


def squeeze(text):
    return re.sub(r"\s+", "", text)


class Terminal:
    def __init__(self, argv, cwd, env, rows=40, cols=120):
        self.pid, self.fd = pty.fork()
        if self.pid == 0:
            os.chdir(cwd)
            os.execvpe(argv[0], argv, env)
        fcntl.ioctl(self.fd, termios.TIOCSWINSZ, struct.pack("HHHH", rows, cols, 0, 0))
        self.output = b""

    def mark(self):
        return len(self.output)

    def since(self, mark):
        return plain(self.output[mark:])

    def read_for(self, seconds, until=None, since=0):
        """Reads for `seconds`, or until `until` shows after `since`."""
        end = time.time() + seconds
        while time.time() < end:
            ready, _, _ = select.select([self.fd], [], [], 0.2)
            if ready:
                try:
                    self.output += os.read(self.fd, 65536)
                except OSError:
                    return False
            if until is not None and squeeze(until) in squeeze(self.since(since)):
                return True
        return until is None

    def type(self, text, enter=True):
        os.write(self.fd, text.encode() if isinstance(text, str) else text)
        if enter:
            time.sleep(0.3)
            os.write(self.fd, ENTER)

    def close(self):
        for key in (CTRL_C, CTRL_C, CTRL_D, CTRL_D):
            try:
                os.write(self.fd, key)
            except OSError:
                break
            time.sleep(0.3)
        try:
            os.kill(self.pid, signal.SIGTERM)
        except ProcessLookupError:
            pass
        try:
            os.waitpid(self.pid, 0)
        except ChildProcessError:
            pass
        os.close(self.fd)
