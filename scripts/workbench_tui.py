"""Windows terminal Python status, state, cancellation and reset smoke."""
import json
import os
from pathlib import Path
import sys
import tempfile
import threading
from http.server import ThreadingHTTPServer
from mock_gateway import Handler, text_of
from plan_tui import View, command
from tui import Terminal


class Gateway(Handler):
    def do_POST(self):
        body = json.loads(self.rfile.read(int(self.headers['content-length'])))
        last = body['messages'][-1]
        if last['role']=='tool':
            return self.reply(body, text='PYTHON_TURN_DONE ' + text_of(last)[-200:])
        prompt = next(text_of(m) for m in reversed(body['messages']) if m['role']=='user')
        if 'WAIT' in prompt:
            code="import asyncio\nprint('PYTHON_WAITING')\nawait asyncio.sleep(60)"
        elif 'FRESH' in prompt:
            code="assert 'value' not in globals()\nprint('PYTHON_FRESH_CONFIRMED')"
        else:
            code="value=42\nprint('PYTHON_STATE_READY')"
        return self.reply(body, tool='python_repl', call={'code':code})


def main():
    server = ThreadingHTTPServer(('127.0.0.1',0),Gateway)
    threading.Thread(target=server.serve_forever,daemon=True).start()
    terminal=None
    try:
        with tempfile.TemporaryDirectory(prefix='vivy-workbench-tui-') as scratch:
            root=Path(scratch)
            home=root/'home'
            home.mkdir()
            (home/'settings.toml').write_text(f'[trust]\ndefault="never"\n[gateway]\nbase_url="http://127.0.0.1:{server.server_port}/v1"\nmodel="fixture"\n',encoding='utf-8')
            env=dict(os.environ,RIG_HOME=str(home),RIG_SECRET_GATEWAY='test-key')
            for key in list(env):
                if key.upper().endswith('_PROXY'): del env[key]
            terminal=Terminal([str(Path(sys.argv[1]).resolve())],root,env)
            view=View(terminal)
            view.wait('terminal ready',lambda t:'idle' in t)
            command(terminal,view,'/python status')
            view.wait('Python status visible',lambda t:'prepared=false' in t)
            terminal.type('STATE')
            view.wait('Python output visible',lambda t:'PYTHON_STATE_READY' in t and 'idle' in t.splitlines()[-1])
            terminal.type('WAIT')
            view.wait('Python awaiting cancellation',lambda t:'PYTHON_WAITING' in t and 'Esc stop' in t)
            terminal.type(b'\x1b',enter=False)
            view.wait('Python cancellation returns idle',lambda t:'idle' in t.splitlines()[-1] and 'Esc stop' not in t)
            terminal.type('FRESH')
            view.wait('cancelled Python starts fresh',lambda t:'PYTHON_FRESH_CONFIRMED' in t and 'idle' in t.splitlines()[-1])
            command(terminal,view,'/python reset')
            view.wait('reset visible',lambda t:'Python closed' in t)
            command(terminal,view,'/quit')
            terminal.read_for(2)
            assert not terminal.process.isalive()
            print('PASS: Windows Python terminal status, output, cancellation, fresh state, reset and exit')
    finally:
        if terminal and terminal.process.isalive(): terminal.process.terminate(force=True)
        server.shutdown()


if __name__=='__main__':main()
