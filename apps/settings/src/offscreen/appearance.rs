//! 独立颜色的保存、校验、恢复默认与离屏界面检查。

use crate::message::{FieldId, Message};
use crate::page_id::PageId;
use crate::settings::Settings;
use bamboo_slip::{App, FontLibrary, Runner, TextEdit};
use qingjian_platform::Config;

#[test]
fn appearance_controls_save_validate_and_reset_without_losing_other_settings() {
    let dir = std::env::temp_dir().join(format!("qingjian-appearance-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("config.toml");
    std::fs::write(&path, "[general]\nlearning_language = 'ja'\n").unwrap();
    let mut settings = Settings::load(path.clone()).unwrap();
    settings.update(Message::set("general", "candidate_font", "Microsoft YaHei"));
    settings.update(Message::set("general", "annotation_font", "Arial"));
    settings.update(Message::set("general", "candidate_bold", true));
    settings.update(Message::set("general", "candidate_gap", 0_i64));
    settings.update(Message::set("general", "candidate_padding", 10_i64));
    settings.update(Message::set("general", "candidate_row_padding", 6_i64));
    settings.update(Message::set("general", "candidate_row_gap", 3_i64));
    settings.update(Message::set("general", "annotation_table", false));
    settings.update(Message::set("general", "annotation_gap", 12_i64));
    settings.update(Message::set("general", "annotation_pos_after", false));
    assert!(!Config::load(&path).unwrap().general.annotation_pos_after);
    settings.update(Message::set("general", "annotation_pos_after", true));
    settings
        .fields
        .insert(FieldId::CandidateBackground, TextEdit::new("#F2F2F2"));
    settings.update(Message::Commit(FieldId::CandidateBackground));
    settings
        .fields
        .insert(FieldId::CandidateColor, TextEdit::new("#123456"));
    settings.update(Message::Commit(FieldId::CandidateColor));
    settings
        .fields
        .insert(FieldId::AnnotationColor, TextEdit::new("#654321"));
    settings.update(Message::Commit(FieldId::AnnotationColor));
    let saved = Config::load(&path).unwrap();
    assert_eq!(saved.general.candidate_color, "#123456");
    assert_eq!(saved.general.annotation_color, "#654321");
    assert_eq!(saved.general.candidate_gap, 0);
    assert_eq!(saved.general.candidate_background, "#F2F2F2");
    assert_eq!(saved.general.candidate_padding, 10);
    assert_eq!(saved.general.candidate_row_padding, 6);
    assert_eq!(saved.general.candidate_row_gap, 3);
    assert!(!saved.general.candidate_shadow);
    assert!(!saved.general.annotation_table);
    assert_eq!(saved.general.annotation_gap, 12);
    assert!(saved.general.annotation_pos_after);
    settings
        .fields
        .insert(FieldId::CandidateColor, TextEdit::new("#中文123"));
    settings.update(Message::Commit(FieldId::CandidateColor));
    assert_eq!(
        Config::load(&path).unwrap().general.candidate_color,
        "#123456"
    );
    assert!(settings.notice.is_some());
    settings
        .fields
        .insert(FieldId::CandidateColor, TextEdit::new(""));
    settings.update(Message::Commit(FieldId::CandidateColor));
    assert!(
        Config::load(&path)
            .unwrap()
            .general
            .candidate_color
            .is_empty()
    );
    settings.update(Message::ResetTextStyles);
    let saved = Config::load(&path).unwrap();
    assert_eq!(saved.general.candidate_font, "Microsoft YaHei");
    assert_eq!(saved.general.annotation_font, "Microsoft YaHei");
    assert_eq!(saved.general.candidate_font_size.get(), Some(16.0));
    assert_eq!(saved.general.annotation_font_size.get(), Some(16.0));
    assert!(!saved.general.candidate_bold);
    assert!(saved.general.annotation_color.is_empty());
    assert_eq!(saved.general.learning_language, "ja");
    assert_eq!(saved.general.candidate_gap, 0);
    assert_eq!(saved.general.candidate_background, "#F2F2F2");
    assert_eq!(saved.general.candidate_row_padding, 6);
    settings.update(Message::Page(PageId::Appearance));
    let mut runner = Runner::new(settings, FontLibrary::system("zh-CN").unwrap());
    let painted = runner
        .rebuild(1.0, (1100, 3500), std::time::Instant::now())
        .unwrap();
    if let Some(output) = std::env::var_os("QINGJIAN_REVIEW_DIR") {
        std::fs::create_dir_all(&output).unwrap();
        painted
            .pixmap
            .save_png(std::path::Path::new(&output).join("settings-appearance.png"))
            .unwrap();
    }
    std::fs::remove_file(&path).unwrap();
    let _ = std::fs::remove_dir(&dir);
}
