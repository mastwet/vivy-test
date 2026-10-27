"""Bind only host-discovered trusted Skills. Instructions remain lazy."""
import importlib
import keyword
import os
import importlib.util
from pathlib import Path
import tomllib

RESERVED = {'workbench', 'mcp', 'memory', 'asyncio', '__builtins__', '_'}
HOST_HANDLERS = {'mcp.config', 'mcp.list_connections'}


class CallableSkill:
    def __init__(self, module):
        self.module = module
        self.__doc__ = module.run.__doc__

    def __call__(self, *args, **kwargs):
        return self.module.run(*args, **kwargs)

    def __getattr__(self, name):
        return getattr(self.module, name)


def load(skills, namespace):
    inventory = []
    for skill in skills:
        record = dict(skill, state='instructions')
        inventory.append(record)
        metadata = Path(skill['file']).parent / 'pyproject.toml'
        if not metadata.is_file():
            continue
        try:
            with metadata.open('rb') as stream:
                project = tomllib.load(stream)
            options = project.get('tool', {}).get('vivy', {}).get('skill', {})
            name = options.get('module', project['project']['name'].replace('-', '_'))
            binding = options.get('binding', name)
            if not binding.isidentifier() or keyword.iskeyword(binding) or binding in RESERVED or binding in namespace:
                raise ValueError(f'Unavailable or reserved binding: {binding}')
            if set(options.get('requires_host', [])) - HOST_HANDLERS:
                raise ValueError('Skill requires an unavailable host handler')
            module = importlib.import_module(name)
            namespace[binding] = CallableSkill(module) if callable(getattr(module, 'run', None)) else module
            record.update(state='ready', binding=binding, module=name)
        except Exception as exc:
            # Broken trusted code must not hide the remaining usable capabilities.
            record.update(state='unavailable', error=f'{type(exc).__name__}: {str(exc)[:512]}')
    return inventory


class Workbench:
    def __init__(self, inventory):
        self._inventory = inventory
        self.readiness = {}

    def list_skills(self):
        return [dict(row) for row in self._inventory]

    def instructions(self, name):
        match = next(row for row in self._inventory if row['name'] == name)
        return Path(match['file']).read_text(encoding='utf-8')

    @staticmethod
    def artifact(value, media_type='text/plain'):
        from .artifacts import save
        return save(value, media_type)


def memory_status(namespace):
    """Observe the current binding only; never query the database for the footer."""
    if not os.environ.get('VIVY_WORKBENCH_MEMORY_ROOT'):
        return 'off'
    try:
        from .memory import Memory
        memory = namespace.get('memory')
        # Users may rebind names. Observation must not invoke their descriptors
        # or change the success/failure of the code cell being reported.
        if type(memory) is not Memory:
            return 'error'
        closed = object.__getattribute__(memory, '__dict__').get('closed')
        return 'closed' if closed is True else 'ready' if closed is False else 'error'
    except Exception:
        return 'error'


def bootstrap(namespace, skills):
    from . import mcp
    import asyncio
    namespace.update(mcp=mcp, asyncio=asyncio)
    namespace['workbench'] = Workbench([])
    namespace['workbench'].readiness['mcp'] = 'ready' if importlib.util.find_spec('mcp') else 'run /python prepare'
    root = os.environ.get('VIVY_WORKBENCH_MEMORY_ROOT')
    if root:
        try:
            from .memory import Memory
            namespace['memory'] = Memory(root)
            namespace['workbench'].readiness['memory'] = 'ready'
        except Exception as exc:
            namespace['workbench'].readiness['memory'] = f'{type(exc).__name__}: {str(exc)[:512]}'
    inventory = load(skills, namespace)
    namespace['workbench']._inventory = inventory
    return inventory
