"""回归验证 portable4 的编码故障与实际 5.1 入口；不注册输入法。"""
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

import package


@unittest.skipUnless(os.name == "nt", "Windows PowerShell 5.1 required")
class PortableScriptTests(unittest.TestCase):
    def setUp(self):
        (package.REPO / "target").mkdir(exist_ok=True)
        self.temp = tempfile.TemporaryDirectory(prefix="青简 便携验证 ", dir=package.REPO / "target")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.source = Path(__file__).with_name("portable.ps1").read_text(encoding="utf-8-sig")
        (self.root / "portable.ps1").write_text(self.source, encoding="utf-8-sig")
        for name in ["qingjian-portable.txt", "qingjian-server.exe", "qingjian-settings.exe",
                     "qingjian_tsf-custom3-portable1.dll", "qingjian_tsf-custom3-portable1-x86.dll",
                     "data/generated/dict.qj"]:
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.touch()

    def run_validator(self):
        powershell = Path(os.environ["SystemRoot"]) / "System32/WindowsPowerShell/v1.0/powershell.exe"
        return subprocess.run([str(powershell), "-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass",
                               "-File", str(Path(__file__).with_name("validate.ps1")),
                               "-BundleRoot", str(self.root)], capture_output=True)

    def test_archive_rejects_missing_bom_before_creating_zip(self):
        (self.root / "portable.ps1").write_text(self.source, encoding="utf-8")
        with self.assertRaisesRegex(RuntimeError, "BOM"):
            package.validate(self.root)

    def test_bom_script_loads_in_windows_powershell_51(self):
        result = self.run_validator()
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_wrong_dll_filename_is_rejected(self):
        dll = self.root / "qingjian_tsf-custom3-portable1-x86.dll"
        dll.rename(self.root / "qingjian-tsf-custom3-portable3-x86.dll")
        self.assertNotEqual(self.run_validator().returncode, 0)


if __name__ == "__main__":
    unittest.main()
