import json
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).parents[1] / 'python'))
from vivy_workbench import prepare


class PreparationTests(unittest.TestCase):
    def test_import_failure_repairs_instead_of_republishing_broken_environment(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            lock = root / 'empty.lock'
            lock.write_text('', encoding='utf-8')
            first = prepare.prepare(root, lock, offline=True)
            damaged = dict(first, imports=['vivy_missing_fixture_module'])
            (root/'current.json').write_text(json.dumps(damaged), encoding='utf-8')
            self.assertEqual(prepare.check(root)['outcome'], 'error')
            repaired = prepare.prepare(root, lock, offline=True)
            self.assertNotEqual(first['executable'], repaired['executable'])
            self.assertEqual(prepare.check(root)['outcome'], 'ready')

    def test_detection_is_read_only_and_scope_changes_require_reprepare(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            lock = root / 'empty.lock'
            lock.write_text('', encoding='utf-8')
            self.assertEqual(prepare.check(root, scope='approved')['outcome'], 'not_init')
            first = prepare.prepare(root, lock, offline=True, scope='approved')
            self.assertEqual(prepare.check(root, scope='approved')['outcome'], 'ready')
            self.assertEqual(prepare.check(root, scope='changed')['outcome'], 'not_init')
            self.assertEqual(json.loads((root / 'current.json').read_text()), first)
            (root / 'current.json').write_text('{bad', encoding='utf-8')
            self.assertEqual(prepare.check(root, scope='approved')['outcome'], 'error')

    def test_corrupt_pointer_can_be_repaired_explicitly(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            lock = root / 'empty.lock'
            lock.write_text('', encoding='utf-8')
            (root / 'current.json').write_text('{broken', encoding='utf-8')
            self.assertTrue(prepare.prepare(root, lock, offline=True)['executable'])

    def test_failed_install_preserves_the_published_environment(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            lock = root / 'empty.lock'
            lock.write_text('', encoding='utf-8')
            first = prepare.prepare(root, lock, python=sys.executable, offline=True)
            self.assertTrue(Path(first['executable']).is_file())
            broken = root / 'broken.lock'
            broken.write_text('vivy-nonexistent-fixture-7a192==0.0.1\n', encoding='utf-8')
            with self.assertRaises(RuntimeError):
                prepare.prepare(root, broken, python=sys.executable, offline=True)
            self.assertEqual(json.loads((root / 'current.json').read_text()), first)
            second = prepare.prepare(root, lock, python=sys.executable, offline=True)
            self.assertEqual(first, second)

    def test_competing_preparer_gets_busy_without_publishing(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            lock = root / 'empty.lock'
            lock.write_text('', encoding='utf-8')
            with prepare.environment_lock(root):
                with self.assertRaisesRegex(RuntimeError, 'busy'):
                    prepare.prepare(root, lock, python=sys.executable, offline=True)
            self.assertFalse((root / 'current.json').exists())
            self.assertTrue(prepare.prepare(root, lock, python=sys.executable, offline=True))


if __name__ == '__main__':
    unittest.main()
