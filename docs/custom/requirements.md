# 青简候选窗口需求与修改汇总

整理日期：2026-10-21。本文记录本次定制的需求、四版修改结果及后续查阅入口，以第四版的实际行为为准。

适用范围：Windows 本地修改版。候选字与译文的独立字体、加粗、字号及颜色支持青简渲染器和系统绘制；其他新增外观设置由青简渲染器提供。文中的“已实现”表示已完成本地源码修改、构建与检查，不代表官方已经合并。

源码仓库：[qingjian-team/qingjian](https://github.com/qingjian-team/qingjian)。基准提交：`c97ff2b61933d6fcd845ee4f4f470faf3ea83bf6`。

当前解压便携版为 **custom3-portable6**，包含 9 份外部词库、万象与数学词库、英语译文，以及系统绘制下完整候选框和译文外观设置，使用方式见第 9 节。

## 1. 需求与修改清单

| 编号 | 需求 | 当前实现 | 首次交付 |
|---|---|---|---|
| R01 | 候选中文与译文分别设置字体 | 两组独立字族设置；字号继续使用已有的独立控制 | 第一版 |
| R02 | 候选与译文分别设置加粗、颜色 | 各自设置加粗、普通文字颜色和选中颜色，支持恢复主题文字样式 | 第一版 |
| R03 | 竖排首个候选始终靠近输入光标 | 框在光标上方时自下而上显示，在下方时自上而下显示；拼音行与页码相应调整 | 第一版 |
| R04 | 候选框可以更靠近输入光标 | 增加与光标间距设置，支持自动或 0–40 px | 第一版 |
| R05 | 上方倒序时方向键与屏幕方向一致 | 按 ↑ 向上移动高亮，按 ↓ 向下移动高亮，随窗口实际方向切换 | 第二版 |
| R06 | 去掉候选框边缘阴影 | 阴影默认关闭，并保留开关 | 第二版 |
| R07 | 设置整个候选框的背景颜色 | 新增背景色设置；例如 `#F2F2F2` 为浅灰，留空跟随主题 | 第二版 |
| R08 | 调整整个候选框与每条词语方框的留白 | 分别设置候选框内边距、每条候选内边距，并增加候选行间距 | 第二版 |
| R09 | 词语在候选格内上下居中 | 去除原有单侧偏移，使用对称留白与垂直居中；行高按内容撑开 | 第二版 |
| R10 | 中英文之间保持固定距离，不被其他行的长中文拉开 | 竖排增加「表格对齐 / 固定间距」选择；固定模式逐行计算译文位置 | 第三版 |
| R11 | 可以调整中英文间距 | 固定间距模式下可设 0–64 px，默认 8 px | 第三版 |
| R12 | 英文词性显示在对应译词后面，并提供设置入口 | 新增「译文前 / 译文后」选择，默认后置；多个义项分别处理 | 第三版 |
| R13 | 提供免安装便携版本 | 解压目录运行，配置和学习数据放入 userdata；附启用、启动、设置、状态、停用脚本，首次启用仍需管理员注册输入法 | custom3-portable5 |

这些修改调整显示与交互，不改变候选词排序权重、数字编号或译词本身的内容。

## 2. 关键行为说明

### 2.1 上下排列与方向键

候选框在输入光标下方时：

```text
[输入光标]
1 开发  development
2 编程  programming
3 架构  architecture
```

候选框在输入光标上方时：

```text
3 架构  architecture
2 编程  programming
1 开发  development
[输入光标]
```

第一个候选始终位于靠近输入位置的一端。上方倒序时拼音行在底部、页码在顶部；文字本身保持正常阅读方向。

第一版仅修改视觉顺序，方向键仍使用原来的上一项／下一项语义。第二版已修正，因此第三版中 ↑ / ↓ 始终按屏幕方向移动高亮。数字选词、空格选词及点击位置保持对应。上方倒序且高亮在首项时，↑ 用于向上选词，不再触发原来的符号面板快捷操作。

### 2.2 表格对齐与固定间距

表格对齐会让译文按公共列排列，短中文与英文之间可能出现较大的空隙：

```text
1 你                you
2 一条比较长的中文  a longer phrase
3 你好              hello
```

固定间距按每行中文的实际宽度排布，英文不必整列对齐：

```text
1 你  you
2 一条比较长的中文  a longer phrase
3 你好  hello
```

第三版默认保留表格模式，需要在设置中手动选择「固定间距」。中英文间距仅在固定模式下生效；横排布局保持原样。

青简绿、系统蓝、微信绿、樱花的表格模式按列对齐；部分装饰主题在该模式下沿用自身的原始多行排布。

### 2.3 词性位置

```text
译文前：int. hello · int. hi
译文后：hello int. · hi int.
```

词性跟随各自的译词，不集中到整行末尾。位置设置不改变译词顺序或点击上屏的内容。需要原有词性显示已开启，且主题没有过滤词性；位置选项本身不会强制显示被隐藏的词性。

## 3. 设置入口与配置项

打开 **青简设置 → 外观**。Windows 的青简渲染器与系统绘制都支持候选字、译文字的独立字体、加粗、字号及颜色；窗口排布和留白项仍按各自控件的可用状态生效。以下 px 数值均按显示缩放换算，不等同于所有显示器上的物理像素数。

### 3.1 字体与文字颜色

| 设置入口 | 配置项，位于 `[general]` | 默认或说明 |
|---|---|---|
| 候选文字 → 字体 | `candidate_font` | 空值沿用主题／基础字体；未安装的字体会回退 |
| 译文文字 → 字体 | `annotation_font` | 同上，与候选字体独立 |
| 候选字号 / 译文字号 | `candidate_font_size` / `annotation_font_size` | 默认 16 px；候选文字和译文分别设置 |
| 候选文字 / 译文文字 → 加粗 | `candidate_bold` / `annotation_bold` | 默认 false；开启使用 700 字重，关闭沿用主题原字重 |
| 候选文字 → 文字颜色 / 选中时颜色 | `candidate_color` / `candidate_highlight_color` | `#RRGGBB`；选中颜色留空时沿用自定义文字颜色；两项都空则跟随主题 |
| 译文文字 → 文字颜色 / 选中时颜色 | `annotation_color` / `annotation_highlight_color` | `#RRGGBB`；选中颜色留空时沿用自定义文字颜色；两项都空则跟随主题 |

「恢复主题文字样式」清除两组字体、加粗、颜色覆盖及字号设置，不重置背景、边距等其他设置。

### 3.2 窗口位置、背景与留白

| 设置入口 | 配置项 | 范围与默认值 |
|---|---|---|
| 候选框位置 → 与光标间距 | `candidate_gap` | 默认 0，即贴近输入行；可设 0–40 |
| 候选框样式 → 候选框背景色 | `candidate_background` | 默认浅灰 `#F2F2F2`；可填写 `#RRGGBB` |
| 候选框样式 → 边缘阴影 | `candidate_shadow` | 默认 false，即关闭 |
| 候选框样式 → 候选框内边距 | `candidate_padding` | 默认 2 px；可设 0–32 |
| 候选框样式 → 每条候选内边距 | `candidate_row_padding` | 默认 1 px；可设 0–32 |
| 候选框样式 → 候选行间距 | `candidate_row_gap` | 默认 2 px，可设 0–24 |

几个距离分别控制不同位置：与光标间距是框外距离，候选框内边距是窗口边缘到内容的距离，每条候选内边距是词语到所在格边缘的距离，候选行间距是相邻格之间的距离。

自定义与光标间距从窗口内容边缘计算，不额外叠加透明阴影宽度；主题内部留白仍由内边距设置控制。

### 3.3 中英文排布与词性

| 设置入口 | 配置项 | 范围与默认值 |
|---|---|---|
| 译文排布 → 竖排译文布局 | `annotation_table` | 默认 false，即固定间距；true 为表格模式 |
| 译文排布 → 中英文间距 | `annotation_gap` | 默认 4，可设 0–64，仅固定间距模式生效 |
| 译文排布 → 词性位置 | `annotation_pos_after` | 默认 true，即译文后；false 为译文前 |
| 候选页 → 译词显示词性 | `translation_pos` | 原有开关，控制词性是否显示；与位置选项分别控制 |

以上均为便携版默认值。系统绘制（GDI）和青简渲染器均提供这些设置。若需直接编辑配置，可将下面字段合并到已有 `[general]` 中，不要重复创建同名分节：

```toml
[general]
candidate_background = "#F2F2F2"
candidate_shadow = false
candidate_gap = 0
candidate_padding = 2
candidate_row_padding = 1
candidate_row_gap = 2
annotation_table = false
annotation_gap = 4
annotation_pos_after = true
translation_pos = true
```

## 4. 版本与交付文件

| 版本 | 主要变化 | 安装说明 |
|---|---|---|
| 第一版 custom1 | 独立文字样式、竖排倒序、与光标间距 | [第一版安装说明](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable1) |
| 第二版 custom2 | 倒序方向键修正、背景、阴影、留白、上下居中 | [第二版安装说明](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable1) |
| 第三版 custom3 | 固定中英文间距、布局切换、词性位置 | [第三版安装说明](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable1) |
| 第四版 custom3-portable4 | 系统绘制完整设置、默认外观值、中文默认模式和 9 份便携词库；入口脚本编码有误，已由第五版替代 | [第四版 Release](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable4) |
| 第五版 custom3-portable5 | 修复 PowerShell 5.1 编码报错、恢复原始构建流程、增加发布前校验，保留第四版功能 | [第五版 Release](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable5) |

第五版包含此前默认外观配置、系统绘制下的完整设置支持及便携词库导入，并修复入口脚本编码；无需依次安装旧版：

- [第六版 Windows 便携包](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable6)。
- [第六版累计源码补丁](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable6)。
- [第六版 SHA-256 校验文件](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable6)。

安装器版本为 `0.2.2-dev-c97ff2b-custom3`。程序「关于」可能仍显示源码基础开发版本号，可通过新增的「译文排布」设置区辨认第三版。

安装包支持覆盖升级；完成后建议重启电脑或注销重新登录，使应用加载新版 DLL。个人设置与学习数据仍位于 `%APPDATA%\Qingjian`。这是个人修改版，未签名，Windows 可能显示未知发布者；uiAccess 关闭，部分系统界面可能遮挡候选窗口。

### 补丁应用方法

第六版补丁包含此前全部定制，应应用到上述基准提交的干净源码上，不要再叠加到已有定制的目录中。本次工作目录已经完成修改，不需要重复应用。

```powershell
git apply --check C:\path\to\qingjian-customization-portable6.patch
git apply C:\path\to\qingjian-customization-portable6.patch
```

## 5. GitHub Issue 跟踪

| Issue | 已提交内容 | 与本地修改的对应关系 |
|---|---|---|
| [#674：独立字体样式、竖排首选靠近光标及候选框间距](https://github.com/qingjian-team/qingjian/issues/674) | 候选／译文独立字体、加粗、颜色；上方倒序；窗口与光标距离 | 对应 R01–R04 |
| [#706：固定间距／表格对齐及词性位置](https://github.com/qingjian-team/qingjian/issues/706) | 固定中英文间距、可调数值、布局切换、词性前后位置，含文字示例 | 对应 R10–R12，并关联 #674、#430 |

第二版的方向键修正、背景、阴影、内边距与垂直居中（R05–R09）已在本地实现，但本次记录中没有为这一组单独创建新 Issue，不应把它们记为已经完整提交给官方。

[#430](https://github.com/qingjian-team/qingjian/issues/430) 是已有的词性显示开关讨论，作为相关背景引用，并非本次新建。Issue 的后续状态以 GitHub 页面为准；本次没有创建或提交 PR。

## 6. 后续修改时的源码入口

本地源码根目录：

```text
<源码目录>
```

以下路径均相对于该源码根目录：

| 修改方向 | 主要文件或目录 |
|---|---|
| 设置字段与默认值 | `crates/qingjian-platform/src/config/general.rs` |
| 外观页设置入口 | `apps/settings/src/view/pages/appearance/` 下的 `text_style.rs`、`candidate_box.rs`、`annotation_layout.rs` |
| 设置预览应用样式 | `apps/settings/src/settings/mod.rs`、`apps/settings/src/view/previews.rs` |
| Windows 配置传递 | `apps/windows/server/src/dispatch/config.rs`、`dispatch/candidates/sink.rs`、`ui/painter/mod.rs` |
| 字体、颜色、背景、留白与译文布局 | `crates/qingjian-render/src/theme/` 下的 `text_style.rs`、`candidate_box.rs`、`annotation_layout.rs` |
| 词性顺序与译词点击编号 | `crates/qingjian-render/src/renderer/build/annotation_order.rs`、`leaf.rs` |
| 表格、逐行布局与垂直居中 | `crates/qingjian-render/src/renderer/build/frame.rs`、`mod.rs` |
| 候选框定位与上方倒序 | `apps/windows/server/src/ui/candidates/`、`crates/qingjian-render/src/renderer/mod.rs` |
| 倒序时的方向键 | `apps/windows/server/src/dispatch/key/input.rs`、`orientation_tests.rs` |
| 回归测试 | `crates/qingjian-render/tests/annotation_layout.rs`、`candidate_box.rs`、`vertical_reversed.rs`；`apps/settings/src/offscreen/appearance.rs` |
| 用户与实现文档 | `docs/user/settings/preferences.md`、`docs/user/getting-started/keys.md`、`docs/notes/crate-notes.md` |
| Windows 安装包脚本 | `apps/windows/installer/qingjian.iss` |

后续跟进官方版本时，应重新比较这些改动与上游实现，避免直接将基于旧提交的累计补丁套到新版源码中。

## 7. 已完成验证与尚待实测

已完成的检查包括：

- 相关配置、渲染、设置程序和 Windows 服务端测试，以及格式与 Clippy 检查。
- 七个内置主题在 100% / 150% / 200% 缩放下的布局和点击映射检查。
- 固定间距不受其他行长中文影响，间距随缩放换算，横排布局保持一致。
- 多义项词性后置后，译词点击仍对应原义项。
- 上下倒序、方向键、不同字号下的垂直居中，以及设置保存和无效颜色校验。
- 优化构建、64 位与 32 位 TSF DLL 构建、安装包生成与复制后的哈希校验。
- 正式设置程序离屏启动成功，生成 31 张页面截图；候选和设置页截图已检查。

尚未完成本次安装包的覆盖安装及浏览器、编辑器等应用内端到端输入验证，也不能将离屏缩放测试视为真实多显示器验证。当前机器实际运行哪个版本，需要在安装后另行确认。

后续实测可依次检查：上／下方弹窗与 ↑ / ↓；长短中文混排与间距调整；点击各个英文译词；关闭再打开设置后配置是否保留；不同应用和显示器之间切换后的定位。

## 8. 效果图与查阅入口

以下图片由实际渲染器或设置程序离屏生成，用于查看实现效果：

- [表格对齐效果](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable1)。
- [固定间距与词性后置效果](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable1)。
- [第三版完整设置页](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable1)。
- [上方倒序候选效果](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable1)。
- [下方正常排列效果](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable1)。

公开交付文件见本仓库 Releases。本文中的历史包可能只在本地保留；公开发布以 Release 附件为准。

## 9. 解压便携包（2026-10-21）

- [便携 ZIP](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable5)。
- [便携版使用说明与迁移步骤](docs/custom/portable.md)。
- [万象词库、数学补充词条、9 份外部词库与英语译文说明](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable5)。
- [含便携支持的累计源码补丁](https://github.com/wangzwe/qingjian-custom/releases/tag/custom3-portable5)。

这份包不需要安装向导，程序不复制到 Program Files，不新增开机启动项。`qingjian-portable.txt` 使 Server 和设置程序把配置、学习记录与自定义主题放在程序同目录的 `userdata`；没有标记时仍使用安装版原来的数据目录。运行诊断日志仍留在 `%LOCALAPPDATA%\Qingjian\logs`。

Windows TSF 输入法仍需注册。首次运行 `Enable.cmd` 会请求管理员权限，随后以普通权限运行 `Start.cmd`。启用期间目录必须保持固定；移动或删除前运行 `Disable.cmd` 并注销或重启，在新路径重新启用。它不是免管理员、零注册或无痕软件。

脚本遇到其他目录的青简注册会拒绝覆盖。本次只制作和验证包，没有改变本机现有 custom3 安装版注册，也没有打包个人数据。迁移应先备份 `%APPDATA%\Qingjian`，卸载安装版后再启用便携包，具体见使用说明。

便携修改入口是 `crates/qingjian-platform/src/dirs/` 与 `apps/windows/portable/`。已验证路径识别、普通模式回退、Windows PowerShell 脚本语法和冲突保护、正式设置程序读取两种目录，以及 ZIP 完整性；尚未执行便携版注册和应用内输入实测。系统绘制文字样式已编译，但尚未在实际输入场景验证。

## portable5 构建审计补充

修复 portable4 手工打包遗漏 UTF-8 BOM 引起的 PowerShell 5.1 中文解码错误和连锁括号报错。恢复最初的 Cargo 与 `package.py --stage` 构建方式，所有程序和两种架构 TSF DLL 重新编译；打包增加实际 Windows PowerShell 5.1 校验及 DLL 名称、架构检查。此版保留此前外观、默认值、词库与译文修改。

## portable6 功能审计补充

修复系统绘制仍使用固定浅蓝选中背景的问题，改为读取所选主题的强调色及对应选中文字颜色；独立颜色设置覆盖所有候选和译文语义类型。统一新旧设置页与服务端的 Rime YAML 加载器，修复六份词库误报损坏，并逐份检查实际可用条数。候选和译文默认字体均为微软雅黑，恢复文字样式时还原为微软雅黑 16 号。交付标签为 `custom3-portable6`。
