"""A sequential, persistent CPython workbench; only stdlib is required."""
import ast
import asyncio
import codecs
import inspect
import io
import json
import os
import sys
import threading
import traceback
import uuid


def serve():
    protocol = io.open(os.dup(1), "w", encoding="utf-8", buffering=1)
    requests = io.open(os.dup(0), "r", encoding="utf-8")
    lock = threading.Lock()
    current = {"id": None, "bytes": 0}

    def send(event, **fields):
        with lock:
            protocol.write(json.dumps({"event": event, "id": current["id"], **fields}, ensure_ascii=False) + "\n")
            protocol.flush()

    def replace_fd(fd, source):
        os.dup2(source, fd)
        if os.name == "nt":
            import ctypes
            import msvcrt
            # CRT descriptors and Win32 std handles both need updating so a
            # subprocess inherits captured output, never the protocol channel.
            kernel = ctypes.WinDLL("kernel32", use_last_error=True)
            kernel.SetStdHandle.argtypes = [ctypes.c_uint32, ctypes.c_void_p]
            kernel.SetStdHandle.restype = ctypes.c_int
            if not kernel.SetStdHandle(((-10, -11, -12)[fd]) & 0xFFFFFFFF, msvcrt.get_osfhandle(fd)):
                raise ctypes.WinError(ctypes.get_last_error())

    null = os.open(os.devnull, os.O_RDONLY)
    replace_fd(0, null)
    os.close(null)
    captures = []
    for fd, kind in [(1, "stdout"), (2, "stderr")]:
        reader, writer = os.pipe()
        replace_fd(fd, writer)
        os.close(writer)
        barrier = ("\0" + uuid.uuid4().hex + "\0").encode()
        drained = threading.Event()

        def pump(reader=reader, kind=kind, barrier=barrier, drained=drained):
            decoder = codecs.getincrementaldecoder("utf-8")("replace")
            pending = b""

            def output(data):
                text = decoder.decode(data)
                if text and current["bytes"] < 10 * 1024 * 1024:
                    current["bytes"] += len(data)
                    send(kind, text=text)

            try:
                while True:
                    data = os.read(reader, 4096)
                    if not data:
                        output(pending)
                        return
                    pending += data
                    while barrier in pending:
                        before, pending = pending.split(barrier, 1)
                        output(before)
                        drained.set()
                    keep = next((size for size in range(min(len(pending), len(barrier) - 1), 0, -1)
                                 if pending.endswith(barrier[:size])), 0)
                    output(pending[:-keep] if keep else pending)
                    pending = pending[-keep:] if keep else b""
            finally:
                os.close(reader)

        threading.Thread(target=pump, daemon=True).start()
        captures.append((fd, barrier, drained))

    sys.stdout.reconfigure(encoding="utf-8", write_through=True)
    sys.stderr.reconfigure(encoding="utf-8", write_through=True)
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    namespace = {"__name__": "__main__"}
    send("ready", python=list(sys.version_info[:3]))
    if sys.version_info < (3, 11):
        send("error", text="Python 3.11 or newer is required")
        return

    async def execute(code):
        tree = ast.parse(code, filename="<python_repl>")
        expression = tree.body.pop() if tree.body and isinstance(tree.body[-1], ast.Expr) else None
        value = eval(compile(tree, "<python_repl>", "exec", ast.PyCF_ALLOW_TOP_LEVEL_AWAIT), namespace)
        if inspect.isawaitable(value):
            await value
        if expression is not None:
            value = eval(compile(ast.Expression(expression.value), "<python_repl>", "eval", ast.PyCF_ALLOW_TOP_LEVEL_AWAIT), namespace)
            if inspect.isawaitable(value):
                value = await value
            namespace["_"] = value
            if value is not None:
                rendered = repr(value).encode("utf-8", errors="backslashreplace")
                # Bound encoded bytes, including JSON's worst-case escaping,
                # below the host's two MiB protocol-frame limit.
                if len(rendered) > 128 * 1024:
                    return rendered[:128 * 1024].decode("utf-8", errors="ignore") + "\n[result truncated; inspect the value in smaller pieces]"
                return rendered.decode("utf-8")

    try:
        for line in requests:
            request = json.loads(line)
            current.update(id=request["id"], bytes=0)
            failure = None
            result = None
            try:
                result = loop.run_until_complete(execute(request["code"]))
            except BaseException:
                failure = traceback.format_exc()[-50 * 1024:]
            # Background task management is deliberately outside this worker's
            # contract. Do not leave detached asyncio tasks executing later.
            tasks = asyncio.all_tasks(loop)
            for task in tasks:
                task.cancel()
            if tasks:
                loop.run_until_complete(asyncio.gather(*tasks, return_exceptions=True))
            sys.stdout.flush()
            sys.stderr.flush()
            for fd, marker, drained in captures:
                drained.clear()
                os.write(fd, marker)
                if not drained.wait(5):
                    raise RuntimeError("output capture did not settle")
            if failure:
                send("error", text=failure)
            elif result is not None:
                send("result", text=result)
            send("done", ok=failure is None)
    finally:
        loop.close()


if __name__ == "__main__":
    serve()
