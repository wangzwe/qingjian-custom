# 定制版构建与打包

对应分支 `custom-windows`，本次发布标签 `custom3-portable5`。以该标签的源码为准。
构建环境：Windows 11 x64、Rust 1.96.0（MSVC）、Visual Studio C++ Build Tools（本次为 18 / MSVC 14.50）、
Windows SDK、Python 3。不要求使用开发者电脑上的绝对路径。

```powershell
rustup target add i686-pc-windows-msvc
$env:QINGJIAN_UIACCESS='0'
cargo build --release --locked -p qingjian-windows-server -p qingjian-windows-tsf -p qingjian-settings -p qingjian-windows-settings
cargo build --release --locked -p qingjian-windows-tsf --target i686-pc-windows-msvc
```

产品词库和模型不在 Git 中。使用 Git Bash 运行 `bash tools/release/data-fetch.sh`，
它依据 `tools/release/data.lock` 从上游下载 data-v7 并校验 SHA256。
数据来源与再生成工具见 `docs/notes/release.md` 的“产品数据从哪来”以及 assets 各目录说明。

初版便携包沿用 Inno 的文件清单，也包含旧版设置程序的 Windows App SDK 运行库。
先按 `apps/windows/installer/build.ps1` 的运行库准备逻辑，将
Microsoft.WindowsAppSDK.Runtime 2.4.0 的 x64 framework 内容准备到
`target/installer/settings-runtime`（参考该脚本，勿执行安装或注册步骤）。
官方 NuGet 地址：
https://api.nuget.org/v3-flatcontainer/microsoft.windowsappsdk.runtime/2.4.0/microsoft.windowsappsdk.runtime.2.4.0.nupkg
保留该包内 license.txt 与 NOTICE.txt。

```powershell
python apps/windows/portable/test_package.py
python apps/windows/portable/package.py --stage target/portable5-stage --guide docs/custom/portable.md --dictionaries "F:\1a实用软件\1-青简输入法自改\0-词库汇总大全"
python apps/windows/portable/package.py --archive target/portable5-stage/Qingjian-custom3-portable5 --output target/qingjian-custom3-portable5-windows-x86_64.zip
```

便携包的两份注册 DLL 固定使用 `qingjian_tsf-custom3-portable1.dll` 与 `qingjian_tsf-custom3-portable1-x86.dll`，并由启停脚本检查相同名称。包内 `userdata/dicts` 默认含指定目录中的 9 份词库；允许保留此前的万象数学补充词库及词库说明。`userdata/user-glossary-en.tsv` 是用户的英语译文词库，也会随既有便携包保留。禁止加入个人配置、学习数据或其他文件。重新生成 `MANIFEST-SHA256.txt` 后，再压成 ZIP 并计算外层 SHA256。

向他人发布前，将 CUSTOM-NOTICE.md、GPL LICENSE 和第三方组件的许可/署名文件
放进便携包，并在同一个 Release 提供本标签的完整源码及 SHA256 校验值。
发布包包含 GPL 许可、第三方许可、词库来源说明、用户英语译文词库和所选词库；打包脚本拒绝未列入白名单的个人文件。

安装器构建参考 `apps/windows/installer/build.ps1` 和 `qingjian.iss`；
本次 Release 只提供便携二进制，避免把旧的安装包误认为便携版。
源码是完整项目，保留上游其他平台代码，但本定制发行只验证 Windows。

## Windows PowerShell 5.1 编码回归

portable4 手工覆盖脚本时丢失了 UTF-8 BOM，Windows PowerShell 5.1 按系统 ANSI 编码读取中文，继而报告字符串和括号不完整。portable5 恢复原 `stage()` 流程，禁止直接用源码脚本覆盖生成文件。`archive()` 现在强制检查 BOM、启动入口、x64/x86 PE 架构，并调用系统 Windows PowerShell 5.1 解析脚本、仅载入函数及验证所需文件。PowerShell 7 的验证不能替代此检查。发布前还需解压最终 ZIP，运行 `Status.cmd` 并核对清单哈希。

本次按上述原始 Cargo 命令重新生成全部程序与 x64/x86 TSF DLL，不复用此前发行包的程序文件。只为 TSF 构建 x86 目标，无需构建 x86 Server 或设置程序。
