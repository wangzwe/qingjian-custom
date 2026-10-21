# 定制版构建与打包

对应分支 `custom-windows`，本次发布标签 `custom3-portable3`。以该标签的源码为准。
构建环境：Windows 11 x64、Rust 1.96.0（MSVC）、Visual Studio 2022 C++ 工具、
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
python apps/windows/portable/package.py --stage dist --guide docs/custom/portable.md
python apps/windows/portable/package.py --archive dist/Qingjian-custom3-portable3 --output qingjian-custom3-portable3-windows-x86_64.zip
```

`custom3-portable3` 延续 portable2 附加的万象通用词库、数学补充词条和英语译文。以 portable2 ZIP 为数据基底，解压后将目录改名为 `Qingjian-custom3-portable3`，用本次 x64 / x86 构建覆盖其中的服务程序、设置程序和两份 TSF DLL；更新 `qingjian-portable.txt` 与本说明，重新生成包内 `MANIFEST-SHA256.txt`，再压成 ZIP 并计算外层 SHA256。不要把本机的 `userdata` 配置或学习记录带进发行包。

向他人发布前，将 CUSTOM-NOTICE.md、GPL LICENSE 和第三方组件的许可/署名文件
放进便携包，并在同一个 Release 提供本标签的完整源码及 SHA256 校验值。
公开发布包比初版便携 ZIP 多了许可和来源说明，程序二进制不变。
包内 userdata 必须只有说明文件；打包脚本会拒绝包含实际用户文件的目录。

安装器构建参考 `apps/windows/installer/build.ps1` 和 `qingjian.iss`；
本次 Release 只提供便携二进制，避免把旧的安装包误认为便携版。
源码是完整项目，保留上游其他平台代码，但本定制发行只验证 Windows。
