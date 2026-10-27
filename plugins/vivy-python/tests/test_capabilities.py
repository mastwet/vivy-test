import importlib
import json
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).parents[1] / 'python'))


class CapabilityTests(unittest.TestCase):
    def test_complete_artifact_round_trip_and_no_silent_cap(self):
        from vivy_workbench import artifacts
        with tempfile.TemporaryDirectory() as directory, patch.dict(os.environ, {'VIVY_WORKBENCH_ARTIFACTS': directory}):
            value = {'rows': ['中文' * 200000], 'isError': False}
            record = artifacts.save(value, 'application/json')
            self.assertTrue(record['complete'])
            self.assertEqual(json.loads(Path(record['path']).read_text(encoding='utf-8')), value)
            self.assertEqual(record['bytes'], Path(record['path']).stat().st_size)
            with self.assertRaises(ValueError):
                artifacts.save(b'12345', 'application/octet-stream', max_bytes=4)

    def test_trusted_skill_binding_failure_isolation_and_lazy_markdown(self):
        from vivy_workbench import capabilities
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name, body in [('fixture_callable', 'def run(value): return value+2'), ('fixture_module', 'answer=42'), ('fixture_bad', 'raise ValueError("broken")')]:
                package = root / name
                package.mkdir()
                (package / '__init__.py').write_text(body, encoding='utf-8')
                (package / 'pyproject.toml').write_text(f'[project]\nname="{name}"\nversion="0.1.0"\n', encoding='utf-8')
            sys.path.insert(0, directory)
            self.addCleanup(sys.path.remove, directory)
            self.addCleanup(lambda: [sys.modules.pop(n, None) for n in ['fixture_callable', 'fixture_module', 'fixture_bad']])
            skills = [{'name': name, 'file': str(root / name / 'SKILL.md')} for name in ['fixture_callable', 'fixture_bad', 'fixture_module']]
            skills.append({'name': 'markdown', 'file': str(root / 'missing-until-read.md')})
            namespace = {}
            inventory = capabilities.load(skills, namespace)
            self.assertEqual(namespace['fixture_callable'](40), 42)
            self.assertEqual(namespace['fixture_module'].answer, 42)
            self.assertNotIn('fixture_bad', namespace)
            self.assertEqual({i['name']: i['state'] for i in inventory}, {'fixture_callable': 'ready', 'fixture_bad': 'unavailable', 'fixture_module': 'ready', 'markdown': 'instructions'})

    def test_reserved_bindings_and_absent_host_requirements_are_unavailable(self):
        from vivy_workbench import capabilities
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'pyproject.toml').write_text('[project]\nname="mcp"\nversion="0.1.0"\n', encoding='utf-8')
            namespace = {'mcp': 'existing'}
            records = capabilities.load([{'name': 'mcp', 'file': str(root / 'SKILL.md')}], namespace)
            self.assertEqual(namespace['mcp'], 'existing')
            self.assertEqual(records[0]['state'], 'unavailable')
            (root / 'pyproject.toml').write_text('[project]\nname="example"\nversion="0.1.0"\n[tool.vivy.skill]\nrequires_host=["factory"]\n', encoding='utf-8')
            self.assertEqual(capabilities.load([{'name': 'example', 'file': str(root / 'SKILL.md')}], {})[0]['state'], 'unavailable')


if __name__ == '__main__':
    unittest.main()
