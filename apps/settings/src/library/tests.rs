//! 设置页与输入法共用加载入口，Rime 头不能被当作 TSV 词条误报损坏。
use super::{Kind, entry};

#[test]
fn rime_dictionary_is_listed_as_usable() {
    let path = std::env::temp_dir().join(format!(
        "qingjian-settings-rime-{}.dict.yaml",
        std::process::id()
    ));
    std::fs::write(
        &path,
        "---\nname: 数学测试\n...\n哈达玛积\tha da ma ji\t50\n",
    )
    .unwrap();
    let info = entry(Kind::Dictionary, "sample".to_owned(), &path);
    assert!(!info.broken);
    assert_eq!(info.name, "数学测试");
    assert_eq!(info.entries, 1);
    std::fs::write(&path, "---\nname: broken\n...\n").unwrap();
    assert!(entry(Kind::Dictionary, "sample".to_owned(), &path).broken);
    std::fs::remove_file(path).unwrap();
}
