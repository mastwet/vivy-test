"""Exercise the built launcher and species against a local scripted gateway.

Usage: python scripts/smoke.py <path-to-vivy.exe>
No credentials or network outside loopback are used.
"""
import json
import os
import pathlib
import subprocess
import sys
import tempfile
import threading
from http.server import ThreadingHTTPServer
from mock_gateway import Handler, text_of


class Gateway(Handler):
    def do_POST(self):
        body = json.loads(self.rfile.read(int(self.headers["content-length"])))
        self.server.records.append((self.path, body, self.headers.get("authorization")))
        messages = body["messages"]
        prompt = next(text_of(m) for m in reversed(messages) if m["role"] == "user")
        last = messages[-1]
        if "FAIL" in prompt:
            return self.reply_error(401, "scripted rejection")
        if "PYRESET" in prompt:
            if last["role"] == "tool":
                return self.reply(body, text="PYRESET " + text_of(last))
            return self.reply(body, tool="python_repl", call={"code": "'fresh' if 'value' not in globals() else 'stale'"})
        if "PYTHON" in prompt:
            if last["role"] == "tool":
                if "42" in text_of(last):
                    return self.reply(body, text="PYTHON " + text_of(last))
                return self.reply(body, tool="python_repl", call={"code": "value + 2"})
            return self.reply(body, tool="python_repl", call={"code": "value = 40\nimport os\nassert not any(k.startswith('RIG_SECRET_') for k in os.environ)\nprint('ready')"})
        if "CONTINUE" in prompt:
            assert sum(m["role"] == "user" for m in messages) >= 2
            return self.reply(body, text="continued")
        if "FILES" in prompt:
            if last["role"] == "user":
                return self.reply(body, tool="read", call={"path": "hello.txt"})
            last_tool = next((m["tool_calls"][-1]["function"]["name"] for m in reversed(messages) if m.get("tool_calls")), None)
            if last_tool == "read":
                return self.reply(body, tool="edit", call={"path": "hello.txt", "edits": [{"old_text": "hi", "new_text": "hello"}]})
            return self.reply(body, text="FILES edited")
        if last["role"] == "tool":
            return self.reply(body, text="DONE " + text_of(last))
        command = "printf hi > hello.txt; cat hello.txt; env | grep -c '^RIG_SECRET_' || true"
        return self.reply(body, call={"command": command})


def main():
    binary = pathlib.Path(sys.argv[1]).resolve()
    server = ThreadingHTTPServer(("127.0.0.1", 0), Gateway)
    server.records = []
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        with tempfile.TemporaryDirectory(prefix="vivy smoke ") as scratch:
            root = pathlib.Path(scratch)
            home = root / "home"
            home.mkdir()
            (home / "settings.toml").write_text(
                f'[gateway]\nbase_url = "http://127.0.0.1:{server.server_port}/v1"\nmodel = "org/model-1"\n', encoding="utf-8"
            )
            environment = dict(os.environ, RIG_HOME=str(home), RIG_SECRET_GATEWAY="test-key")

            def run(*args, expected=0):
                result = subprocess.run([str(binary), *args], cwd=root, env=environment,
                    input="", text=True, encoding="utf-8", capture_output=True, timeout=25)
                assert result.returncode == expected, (args, result.returncode, result.stdout, result.stderr)
                return result.stdout

            answer = run("-p", "CREATE")
            assert "DONE" in answer, answer
            assert (root / "hello.txt").read_text() == "hi"
            first = next((home / "sessions").iterdir()).name
            assert "continued" in run("--resume", first, "-p", "CONTINUE")
            assert "FILES edited" in run("--resume", first, "-p", "FILES")
            assert (root / "hello.txt").read_text() == "hello"
            before = {p.name for p in (home / "sessions").iterdir()}
            assert "42" in run("-p", "PYTHON")
            python_session = next(p.name for p in (home / "sessions").iterdir() if p.name not in before)
            assert "fresh" in run("--resume", python_session, "-p", "PYRESET")
            records = [json.loads(line) for line in run("--json", "-p", "CREATE").splitlines() if line.strip()]
            assert records, "no JSON events"
            run("-p", "FAIL", expected=1)
            run("--resume", first, "-p", "/new", expected=75)
            assert server.records
            for path, body, authorization in server.records:
                assert path == "/v1/chat/completions", path
                assert body["model"] == "org/model-1", body["model"]
                assert authorization == "Bearer test-key"
            offered = {t["function"]["name"] for t in server.records[0][1]["tools"]}
            assert {"read", "edit", "write", "shell", "python_repl", "search", "list"} <= offered, offered
            for path in home.rglob("*"):
                if path.is_file():
                    assert b"test-key" not in path.read_bytes(), path
            print("PASS: gateway/model, native shell, sessions, Python state/reset, JSON events, error exit, secret filtering")
    finally:
        server.shutdown()
        server.server_close()


if __name__ == "__main__":
    main()
