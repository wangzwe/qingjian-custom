#!/usr/bin/env python3
"""按 Inno 的随包文件清单生成解压版；不读取或打包当前用户数据。"""

import argparse
import glob
import hashlib
from pathlib import Path
import re
import shutil
import zipfile

REPO = Path(__file__).resolve().parents[3]
FOLDER = "Qingjian-custom3-portable4"
PORTABLE_DICTIONARIES = {
    "41448.dict.yaml",
    "8105.dict.yaml",
    "base.dict.yaml",
    "ext.dict.yaml",
    "others.dict.yaml",
    "tencent.dict.yaml",
    "qingjian-general-wanxiang.tsv",
    "qingjian-math-naer.tsv",
    "qingjian-math-open-supplemented.tsv",
}


def stage(destination: Path, guide: Path, dictionaries: Path | None = None) -> Path:
    root = destination.resolve() / FOLDER
    root.mkdir(parents=True, exist_ok=False)
    script = (REPO / "apps/windows/installer/qingjian.iss").read_text(encoding="utf-8-sig")
    files = script.split("[Files]", 1)[1].split("[Icons]", 1)[0]
    for line in files.splitlines():
        match = re.match(r'Source: "([^"]+)";\s*DestDir: "([^"]+)";(.*)', line)
        if not match:
            continue
        source, target, options = match.groups()
        source = source.replace("{#Repo}", str(REPO))
        target = target.replace("{app}", str(root))
        matches = [Path(path) for path in glob.glob(source)]
        if not matches:
            raise RuntimeError(f"打包资源缺失：{source}")
        rename = re.search(r'DestName: "([^"]+)"', options)
        for item in matches:
            name = rename.group(1) if rename else item.name
            name = name.replace("{#TsfDll}", "qingjian_tsf-custom3-portable1.dll")
            name = name.replace("{#TsfDll32}", "qingjian_tsf-custom3-portable1-x86.dll")
            dest = Path(target) / name
            if not dest.resolve().is_relative_to(root):
                raise RuntimeError(f"文件目标越界：{dest}")
            dest.parent.mkdir(parents=True, exist_ok=True)
            if item.is_dir():
                shutil.copytree(item, dest)
            else:
                shutil.copy2(item, dest)
    (root / "qingjian-portable.txt").write_text("Qingjian custom3 portable4\n", encoding="utf-8")
    (root / "userdata").mkdir()
    (root / "userdata/README.txt").write_text(
        "配置、学习数据和自定义主题保存在此目录。\n"
        "包中不附带个人数据；迁移前请阅读根目录的便携版使用说明。\n",
        encoding="utf-8-sig",
    )
    if dictionaries is not None:
        found = {path.name for path in dictionaries.iterdir() if path.is_file()}
        missing = PORTABLE_DICTIONARIES - found
        if missing:
            raise RuntimeError(f"缺少用户指定的便携词库文件：{', '.join(sorted(missing))}")
        target_dicts = root / "userdata/dicts"
        target_dicts.mkdir()
        for name in sorted(PORTABLE_DICTIONARIES):
            shutil.copy2(dictionaries / name, target_dicts / name)
    # Windows PowerShell 5.1 需要 BOM 才能可靠读取中文提示。
    (root / "portable.ps1").write_text(
        (Path(__file__).parent / "portable.ps1").read_text(encoding="utf-8-sig"), encoding="utf-8-sig"
    )
    for action in ["Enable", "Disable", "Start", "Settings", "Status"]:
        launcher = (
            '@echo off\r\nsetlocal\r\n'
            'set "QJ_PS=%SystemRoot%\\System32\\WindowsPowerShell\\v1.0\\powershell.exe"\r\n'
            'if exist "%SystemRoot%\\Sysnative\\WindowsPowerShell\\v1.0\\powershell.exe" '
            'set "QJ_PS=%SystemRoot%\\Sysnative\\WindowsPowerShell\\v1.0\\powershell.exe"\r\n'
            f'"%QJ_PS%" -NoProfile -ExecutionPolicy Bypass -File "%~dp0portable.ps1" -Action {action}\r\n'
            'set "QJ_RESULT=%ERRORLEVEL%"\r\n'
            + ('pause\r\n' if action == "Status" else 'if not "%QJ_RESULT%"=="0" pause\r\n')
            + 'exit /b %QJ_RESULT%\r\n'
        )
        (root / f"{action}.cmd").write_bytes(launcher.encode("ascii"))
    shutil.copy2(guide, root / "便携版使用说明.md")
    shutil.copy2(REPO / "LICENSE", root / "LICENSE")
    licenses = root / "licenses"
    licenses.mkdir()
    for name in ["qingjian-render", "bamboo-slip"]:
        shutil.copy2(REPO / f"crates/{name}/LICENSE.md", licenses / f"{name}-LICENSE.md")
    return root


def archive(root: Path, output: Path) -> None:
    if root.name != FOLDER or not (root / "qingjian-portable.txt").is_file():
        raise RuntimeError("不是本次便携包目录")
    # 不允许把试运行或个人迁移产生的数据发布出去。
    allowed_files = {"README.txt"}
    dicts = root / "userdata/dicts"
    unexpected_roots = {path.name for path in (root / "userdata").iterdir()} - {"README.txt", "dicts"}
    if unexpected_roots:
        raise RuntimeError("userdata 含有非词库个人数据，拒绝打包")
    if dicts.exists():
        allowed_dict_files = PORTABLE_DICTIONARIES
        unexpected = {path.name for path in dicts.iterdir() if path.is_file()} - allowed_dict_files
        if unexpected or any(path.is_dir() for path in dicts.iterdir()):
            raise RuntimeError("userdata/dicts 含有便携词库清单以外的内容，拒绝打包")
        if {path.name for path in dicts.iterdir()} != allowed_dict_files:
            raise RuntimeError("便携词库不完整，拒绝打包")
    for path in (root / "userdata").iterdir():
        if path.is_file() and path.name not in allowed_files:
            raise RuntimeError("userdata 已产生非词库文件，拒绝将个人数据放入交付包")
    files = sorted(path for path in root.rglob("*") if path.is_file() and path.name != "MANIFEST-SHA256.txt")
    lines = [f"{hashlib.file_digest(path.open('rb'), 'sha256').hexdigest()}  {path.relative_to(root).as_posix()}" for path in files]
    manifest = root / "MANIFEST-SHA256.txt"
    manifest.write_text("\n".join(lines) + "\n", encoding="utf-8")
    output.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(output, "x", zipfile.ZIP_DEFLATED, compresslevel=6) as bundle:
        for path in files + [manifest]:
            bundle.write(path, path.relative_to(root.parent))
    with zipfile.ZipFile(output) as bundle:
        bad = bundle.testzip()
        if bad:
            raise RuntimeError(f"压缩包 CRC 校验失败：{bad}")
        print(f"ZIP 验证通过：{len(bundle.namelist())} 个文件")
    print(f"文件：{output}\n大小：{output.stat().st_size}\nSHA256：{hashlib.file_digest(output.open('rb'), 'sha256').hexdigest()}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--stage", type=Path)
    parser.add_argument("--guide", type=Path)
    parser.add_argument("--dictionaries", type=Path)
    parser.add_argument("--archive", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    if args.stage and args.guide:
        print(stage(args.stage, args.guide, args.dictionaries))
    elif args.archive and args.output:
        archive(args.archive.resolve(), args.output.resolve())
    else:
        parser.error("使用 --stage DIR --guide FILE 或 --archive DIR --output ZIP")
