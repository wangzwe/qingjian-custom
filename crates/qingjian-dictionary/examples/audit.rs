//! 逐份实际加载指定目录的词库，输出词条数；任一词库无法加载时返回失败。
use qingjian_dictionary::Dictionary;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let directory = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .ok_or("missing dictionary directory")?,
    );
    let mut paths: Vec<_> = std::fs::read_dir(directory)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| {
                    ["qj", "tsv", "yaml", "yml"]
                        .iter()
                        .any(|allowed| ext.eq_ignore_ascii_case(allowed))
                })
        })
        .collect();
    paths.sort();
    if paths.is_empty() {
        return Err("no dictionary files found".into());
    }
    for path in paths {
        let dictionary = Dictionary::from_path(&path)?;
        if dictionary.is_empty() {
            return Err(format!("empty dictionary: {}", path.display()).into());
        }
        println!(
            "{}\t{}",
            path.file_name().unwrap().to_string_lossy(),
            dictionary.len()
        );
    }
    Ok(())
}
