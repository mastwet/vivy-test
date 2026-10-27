"""Explicit uv preparation. Publish a pointer only after a fresh environment probes."""
from contextlib import contextmanager
import hashlib
import json
import os
import re
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import uuid


@contextmanager
def environment_lock(root):
    root = Path(root)
    root.mkdir(parents=True, exist_ok=True)
    with (root / 'prepare.lock').open('a+b') as stream:
        if stream.tell() == 0:
            stream.write(b'\0')
            stream.flush()
        stream.seek(0)
        try:
            if os.name == 'nt':
                import msvcrt
                msvcrt.locking(stream.fileno(), msvcrt.LK_NBLCK, 1)
            else:
                import fcntl
                fcntl.flock(stream.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
        except OSError as exc:
            raise RuntimeError('Python preparation is busy in another process') from exc
        try:
            yield
        finally:
            stream.seek(0)
            if os.name == 'nt':
                msvcrt.locking(stream.fileno(), msvcrt.LK_UNLCK, 1)
            else:
                fcntl.flock(stream.fileno(), fcntl.LOCK_UN)


def run(command, *, temporary=None):
    environment = dict(os.environ)
    for name in list(environment):
        if name.upper().startswith('RIG_SECRET_'):
            del environment[name]
    if temporary is not None:
        environment.update(TMP=str(temporary), TEMP=str(temporary), SQLITE_TMPDIR=str(temporary))
        environment['UV_CACHE_DIR'] = str(Path(temporary).parent / 'uv-cache')
    # A file prevents unbounded installer transcripts from filling host memory.
    with tempfile.TemporaryFile(dir=temporary) as output:
        result = subprocess.run(command, env=environment, stdout=output, stderr=subprocess.STDOUT)
        output.seek(0, 2)
        output.seek(max(0, output.tell() - 16384))
        tail = output.read().decode('utf-8', errors='replace')
    if result.returncode:
        tail = re.sub(r'https?://\S+', '[URL redacted]', tail)
        for key, value in environment.items():
            if len(value) >= 4 and any(word in key.upper() for word in ('KEY', 'TOKEN', 'SECRET', 'PASSWORD', 'CREDENTIAL')):
                tail = tail.replace(value, '[redacted]')
        raise RuntimeError(f'Python preparation failed at {command[1]} (exit {result.returncode}); existing environment preserved\n{tail}')
    return tail


def prepare(root, requirements, *, python=sys.executable, skills=(), offline=False, native_memory=False, memory_root=None):
    root, requirements = Path(root).resolve(), Path(requirements).resolve()
    memory_root = Path(memory_root).resolve() if memory_root else root / 'native-memory'
    temporary_directory = root / 'tmp'
    temporary_directory.mkdir(parents=True, exist_ok=True)
    invoke = lambda command: run(command, temporary=temporary_directory)
    skill_paths = [Path(row['file']).parent.resolve() for row in skills if (Path(row['file']).parent / 'pyproject.toml').is_file()]
    digest = hashlib.sha256(requirements.read_bytes() + str(python).encode() + str(native_memory).encode() + str(memory_root).encode())
    for path in skill_paths:
        digest.update(str(path).encode())
        digest.update((path / 'pyproject.toml').read_bytes())
    identity = digest.hexdigest()
    native_code = ('import sys;sys.path.insert(0,' + repr(str(Path(__file__).resolve().parents[1])) + ');'
                   'from vivy_workbench.memory import prepare_assets, probe;'
                   'prepare_assets(' + repr(str(memory_root)) + ', offline=' + repr(offline) + ');'
                   'probe(' + repr(str(memory_root)) + ')')
    with environment_lock(root):
        published = root / 'current.json'
        if published.is_file():
            try:
                current = json.loads(published.read_text(encoding='utf-8'))
                if current['identity'] == identity and Path(current['executable']).is_file():
                    invoke([current['executable'], '-I', '-c', 'import sys; assert sys.version_info >= (3,11)'])
                    invoke(['uv', 'pip', 'check', '--python', current['executable']])
                    if native_memory:
                        invoke([current['executable'], '-I', '-c', native_code])
                    return current
            except (ValueError, KeyError, RuntimeError):
                pass  # Explicit prepare can repair a damaged publication/environment.
        started = time.monotonic()
        environment = root / 'environments' / uuid.uuid4().hex
        invoke(['uv', 'venv', str(environment), '--python', str(python)] + (['--offline'] if offline else []))
        executable = environment / ('Scripts/python.exe' if os.name == 'nt' else 'bin/python')
        invoke(['uv', 'pip', 'sync', '--python', str(executable), '--require-hashes', str(requirements)] + (['--offline'] if offline else []))
        if skill_paths:
            command = ['uv', 'pip', 'install', '--python', str(executable), '--constraint', str(requirements)]
            for path in skill_paths:
                command.extend(['--editable', str(path)])
            invoke(command + (['--offline'] if offline else []))
        versions = invoke([str(executable), '-I', '-c', 'import importlib.metadata,json,sys; assert sys.version_info >= (3,11); print(json.dumps({d.metadata["Name"]:d.version for d in importlib.metadata.distributions()}))'])
        if native_memory:
            invoke([str(executable), '-I', '-c', native_code])
        current = {'identity': identity, 'executable': str(executable), 'packages': json.loads(versions),
                   'seconds': time.monotonic() - started}
        temporary = root / (uuid.uuid4().hex + '.json.tmp')
        with temporary.open('x', encoding='utf-8') as stream:
            json.dump(current, stream)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, published)
        return current


if __name__ == '__main__':
    print(json.dumps(prepare(sys.argv[1], sys.argv[2], python=sys.executable,
                             skills=json.loads(os.environ.get('VIVY_WORKBENCH_SKILLS', '[]')))))
