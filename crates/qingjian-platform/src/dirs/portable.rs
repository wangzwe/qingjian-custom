//! 按主程序位置识别便携数据目录，不依赖启动器环境变量或当前工作目录。

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub(super) fn user_dir() -> Option<PathBuf> {
    static DIRECTORY: OnceLock<Option<PathBuf>> = OnceLock::new();
    DIRECTORY
        .get_or_init(|| resolve(&std::env::current_exe().ok()?))
        .clone()
}

fn resolve(executable: &Path) -> Option<PathBuf> {
    let name = executable.file_name()?.to_str()?.to_ascii_lowercase();
    // DLL 被加载进任意应用时，current_exe 是宿主；不能把宿主目录当成用户目录。
    if !matches!(
        name.as_str(),
        "qingjian-server.exe" | "qingjian-settings.exe" | "qingjian-settings-legacy.exe"
    ) {
        return None;
    }
    let root = executable.parent()?;
    root.join("qingjian-portable.txt")
        .is_file()
        .then(|| root.join("userdata"))
}

#[cfg(test)]
mod tests {
    use super::resolve;

    #[test]
    fn marker_only_redirects_qingjian_programs_and_moves_with_the_directory() {
        let root = std::env::temp_dir().join(format!("qingjian-portable-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        assert_eq!(resolve(&root.join("qingjian-server.exe")), None);
        std::fs::write(root.join("qingjian-portable.txt"), "portable-v1").unwrap();
        for name in [
            "qingjian-server.exe",
            "QINGJIAN-SETTINGS.EXE",
            "qingjian-settings-legacy.exe",
        ] {
            assert_eq!(resolve(&root.join(name)), Some(root.join("userdata")));
        }
        assert_eq!(resolve(&root.join("notepad.exe")), None);
        assert_eq!(resolve(&root.join("qingjian_tsf.dll")), None);
        let other = root.join("another location");
        std::fs::create_dir_all(&other).unwrap();
        assert_eq!(resolve(&other.join("qingjian-server.exe")), None);
        std::fs::write(other.join("qingjian-portable.txt"), "portable-v1").unwrap();
        assert_eq!(
            resolve(&other.join("qingjian-server.exe")),
            Some(other.join("userdata"))
        );
        // 只移除本测试创建的已知文件和空目录。
        std::fs::remove_file(other.join("qingjian-portable.txt")).unwrap();
        std::fs::remove_dir(other).unwrap();
        std::fs::remove_file(root.join("qingjian-portable.txt")).unwrap();
        std::fs::remove_dir(root).unwrap();
    }
}
