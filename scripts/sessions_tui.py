"""Check resume, fork, rewind and the branch summary in vivy's terminal view.

Usage: python scripts/sessions_tui.py <path-to-vivy>
Unix only (pseudo-terminal). Loopback scripted gateway, no credentials.
Exits non-zero naming each check that failed.
"""
import json
import os
import pathlib
import subprocess
import sys
import tempfile
import threading
import time
from http.server import ThreadingHTTPServer

from mock_gateway import Handler, text_of
from tui import ALT_UP, ENTER, Terminal

SUMMARIZER = "You summarize a conversation"


class Gateway(Handler):
    def do_POST(self):
        body = json.loads(self.rfile.read(int(self.headers["content-length"])))
        self.server.records.append(body)
        messages = body["messages"]
        system = text_of(messages[0]) if messages and messages[0]["role"] == "system" else ""
        if SUMMARIZER in system:
            if self.server.fail_summary:
                return self.reply_error(401, "scripted rejection")
            return self.reply(body, text="## Goal\nBRANCH-SUMMARY-TEXT")
        last_user = [text_of(m) for m in messages if m["role"] == "user"][-1]
        return self.reply(body, text="answer to " + last_user.split()[-1])


def session(binary, server, fail_summary):
    """One terminal session through every check; returns failed check names."""
    server.records, server.fail_summary = [], fail_summary
    failed = []

    def check(name, ok):
        print(("PASS " if ok else "FAIL ") + name)
        if not ok:
            failed.append(name)

    with tempfile.TemporaryDirectory(prefix="vivy sessions ") as scratch:
        root = pathlib.Path(scratch)
        home, work = root / "home", root / "work"
        home.mkdir()
        work.mkdir()
        (home / "settings.toml").write_text(
            f'[gateway]\nbase_url = "http://127.0.0.1:{server.server_port}/v1"\n'
            'model = "org/model-1"\ncontext_window = 64000\n', encoding="utf-8")
        env = dict(os.environ, RIG_HOME=str(home), RIG_SECRET_GATEWAY="test-key")
        terminal = Terminal([str(binary)], work, env)
        try:
            terminal.read_for(4)
            for word in ("alpha", "beta"):
                mark = terminal.mark()
                terminal.type(f"first {word}")
                check(f"reply to {word}", terminal.read_for(20, until=f"answer to {word}", since=mark))

            # Rewind: select the second message, Enter forks just before it.
            mark = terminal.mark()
            terminal.type(ALT_UP, enter=False)
            time.sleep(0.5)
            terminal.type(ALT_UP, enter=False)
            time.sleep(0.5)
            terminal.type(ENTER, enter=False)
            check("rewind forks", terminal.read_for(15, until="Forked into a new agent", since=mark))
            asked = [b for b in server.records if SUMMARIZER in text_of(b["messages"][0])]
            check("summary request sent", len(asked) == 1)
            if asked:
                check("summary request reads the branch left behind",
                      "first beta" in json.dumps(asked[0]) and "answer to beta" in json.dumps(asked[0]))
            if fail_summary:
                check("failed summary is reported",
                      terminal.read_for(10, until="Branch summary failed", since=mark))
            else:
                check("summary shown in the fork", terminal.read_for(
                    10, until="summary of a branch that this conversation came back from", since=mark))
                check("notice counts the messages",
                      "leftbehind(2messages)" in "".join(terminal.since(mark).split()))

            # The recalled message goes on the fork, with the summary before it.
            before = len(server.records)
            mark = terminal.mark()
            terminal.type(ENTER, enter=False)
            terminal.read_for(15, until="answer to beta", since=mark)
            turn = server.records[before:] and server.records[-1]
            sent = json.dumps(turn) if turn else ""
            check("fork's next turn has the recalled message", "first beta" in sent)
            check("fork's next turn " + ("has no summary" if fail_summary else "carries the summary"),
                  ("BRANCH-SUMMARY-TEXT" in sent) != fail_summary)

            # Fork and the agent list.
            mark = terminal.mark()
            terminal.type("/fork")
            check("/fork forks", terminal.read_for(10, until="Forked into a new agent", since=mark))
            mark = terminal.mark()
            terminal.type("/agents", enter=False)
            time.sleep(0.3)
            terminal.type(ENTER, enter=False)
            terminal.read_for(4, since=mark)
            listed = "".join(terminal.since(mark).split()).count("·gateway/org/model-1·idle")
            check("/agents lists the three agents", listed >= 3)
            terminal.type(b"\x1b", enter=False)
            time.sleep(0.5)

            # A new session, then the picker back to this one.
            mark = terminal.mark()
            terminal.type("/new")
            terminal.read_for(8, since=mark)
            mark = terminal.mark()
            terminal.type("/resume", enter=False)
            time.sleep(0.3)
            terminal.type(ENTER, enter=False)
            check("/resume lists the earlier session", terminal.read_for(10, until="first alpha", since=mark))
            mark = terminal.mark()
            terminal.type(ENTER, enter=False)
            check("picking it resumes it", terminal.read_for(15, until="answer to alpha", since=mark))
            sessions = list((home / "sessions").iterdir())
            check("two sessions on disk", len(sessions) == 2)

            # /tree shows the branches (Rig's agents); /clone copies the session.
            mark = terminal.mark()
            terminal.type("/tree")
            check("/tree lists the branches", terminal.read_for(6, until="Show an agent", since=mark))
            terminal.type(b"\x1b", enter=False)
            time.sleep(0.5)
            mark = terminal.mark()
            terminal.type("/clone")
            check("/clone switches to a copy", terminal.read_for(15, until="answer to alpha", since=mark))
            check("three sessions after /clone", len(list((home / "sessions").iterdir())) == 3)

            # /export, then /new and /import of that file.
            mark = terminal.mark()
            terminal.type("/export saved.jsonl")
            check("/export writes JSONL", terminal.read_for(6, until="Exported the session", since=mark)
                  and (work / "saved.jsonl").is_file())
            mark = terminal.mark()
            terminal.type("/new")
            terminal.read_for(8, since=mark)
            mark = terminal.mark()
            terminal.type("/import saved.jsonl")
            check("/import loads it", terminal.read_for(8, until="Imported", since=mark))
            before = len(server.records)
            mark = terminal.mark()
            terminal.type("after import")
            terminal.read_for(15, until="answer to import", since=mark)
            sent = json.dumps(server.records[before:])
            check("imported history reaches the model", "first alpha" in sent and "after import" in sent)
        finally:
            terminal.close()
        # After the terminal closed: a running session is locked to its process.
        print_mode(binary, server, home, work, check)
    return failed


def print_mode(binary, server, home, work, check):
    """The launcher's Pi session options, in print mode."""
    env = dict(os.environ, RIG_HOME=str(home), RIG_SECRET_GATEWAY="test-key")

    def run(*args):
        return subprocess.run([str(binary), *args], cwd=work, env=env, input="", text=True,
                              capture_output=True, timeout=60)

    before = set((home / "sessions").iterdir())
    result = run("--no-session", "-p", "ephemeral one")
    check("--no-session answers", result.returncode == 0 and "answer to one" in result.stdout)
    check("--no-session keeps no session", set((home / "sessions").iterdir()) == before)
    # Ids start with their creation time: the first is the original session.
    source = sorted(before, key=lambda path: path.name)[0].name
    result = run("--fork", source, "-p", "forked two")
    sent = json.dumps(server.records[-1])
    check("--fork continues a copy", result.returncode == 0 and "first alpha" in sent)
    check("--fork leaves the original", len(set((home / "sessions").iterdir())) == len(before) + 1)
    result = run("--session", source, "-p", "again three")
    check("--session resumes", result.returncode == 0 and "first alpha" in json.dumps(server.records[-1]))


def main():
    binary = pathlib.Path(sys.argv[1]).resolve()
    server = ThreadingHTTPServer(("127.0.0.1", 0), Gateway)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    try:
        failed = []
        print("== with a summary")
        failed += session(binary, server, fail_summary=False)
        print("== with the summary request refused")
        failed += [f"refused: {name}" for name in session(binary, server, fail_summary=True)]
    finally:
        server.shutdown()
        server.server_close()
    if failed:
        print("FAILED: " + ", ".join(failed), file=sys.stderr)
        sys.exit(1)
    print("PASS: resume, fork, rewind and branch summary in the terminal view")


if __name__ == "__main__":
    main()
