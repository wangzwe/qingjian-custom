# 青简 Windows 非官方定制版

修改及分发者：GitHub 用户 wangzwe。修改日期：2026-10-16 至 2026-10-20。
上游项目：https://github.com/qingjian-team/qingjian
基准提交：`c97ff2b61933d6fcd845ee4f4f470faf3ea83bf6`，保留原作者署名、版权声明及完整提交历史。

本版本在上游基础上修改了候选/译文字体与颜色、候选窗口靠近光标的排列及方向键行为、
窗口背景/阴影/边距、译文固定间距与词性位置，并增加文件夹内保存用户数据的便携模式。
本修改版整体按 **GPL-3.0-or-later** 发布，完整许可见根目录 LICENSE。
上游双许可组件在此按 GPL 选项分发，不宣称取得商业许可。
软件按原样提供，不提供担保；本仓库及发布包不是青简官方发行版。

## 源码与下载

- 修改源码：https://github.com/wangzwe/qingjian-custom/tree/custom-windows
- 本次发布及对应源码：https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable1
- Release 同时提供完整源代码 ZIP、便携 ZIP、SHA256SUMS.txt。
- 构建说明：docs/custom/build.md；修改清单：docs/custom/requirements.md。
- 产品数据采用上游 tools/release/data.lock 锁定的 data-v7，构建脚本和数据来源随源码提供。
- 便携版首次启用仍需要管理员注册 Windows TSF；仅 Windows x64（含 x86 TSF DLL）。
- 已完成编译、自动化测试、离屏布局检查与便携路径验证；尚未完成便携版注册后的真实输入测试。
- 二进制未签名，uiAccess 关闭。应用内更新入口仍指向上游，官方更新可能覆盖定制功能。

## 第三方组件与素材

程序代码的 GPL 许可不替代词库、字体、运行库、主题素材各自的许可。
保留 assets 下各来源说明以及 crates/qingjian-render/LICENSE.md、
crates/bamboo-slip/LICENSE.md。主题中的第三方角色形象权利仍归原权利人。
便携包 licenses/ 附有数据来源署名、Cargo 依赖许可副本、
Microsoft Windows App SDK Runtime 2.4.0 的原始许可与 NOTICE。
依赖许可汇集包含构建/跨平台依赖，不表示每项均被链接进 Windows 可执行文件。
