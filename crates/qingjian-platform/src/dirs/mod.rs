//! Windows 上用户目录的约定，Server / TSF DLL / 设置程序三处共用：
//!
//! - **数据** `%APPDATA%\Qingjian`：配置、密钥、学习数据、统计、输入日志（漫游目录，跟着账户走）。
//! - **日志** `%LOCALAPPDATA%\Qingjian\logs`：三个进程的运行日志都在这一个目录里，
//!   `server.<日期>.log` / `tsf.<日期>.log` / `settings.<日期>.log`，用户反馈问题时整个目录打包即可。
//!   放本机目录有两个原因：日志本来就是这台机器的东西，不该跟账户漫游；DLL 被加载进
//!   AppContainer 应用（任务栏搜索 / 设置）时写不了漫游目录，Server 启动时会给这个目录授权。
//!
//! Windows 便携包的主程序同目录有 `qingjian-portable.txt` 时，数据改放 `userdata`；
//! TSF DLL 的宿主不参与便携检测，运行日志仍保留在原来的本机日志目录。
//! macOS 有自己的 `paths.rs`，不走这里。

#[cfg(windows)]
mod portable;

use std::path::PathBuf;

/// 安装版为 `%APPDATA%\Qingjian`，Windows 便携主程序为同级 `userdata`。
pub fn user_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    if let Some(dir) = portable::user_dir() {
        return Some(dir);
    }
    std::env::var_os("APPDATA").map(|base| PathBuf::from(base).join("Qingjian"))
}

/// 用户数据目录中的 `config.toml`。
pub fn config_path() -> Option<PathBuf> {
    user_dir().map(|dir| dir.join("config.toml"))
}

/// 用户数据目录中的 `themes`，一个主题一个子目录（`<id>\theme.json`），不负责创建。
pub fn themes_dir() -> Option<PathBuf> {
    user_dir().map(|dir| dir.join("themes"))
}

/// 运行日志目录 `%LOCALAPPDATA%\Qingjian\logs`，不负责创建。
pub fn log_dir() -> Option<PathBuf> {
    std::env::var_os("LOCALAPPDATA").map(|base| PathBuf::from(base).join("Qingjian").join("logs"))
}
