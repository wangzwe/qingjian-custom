//! 上方竖排只改变视觉顺序；编号、高亮与译文点击仍指向同一个候选。

mod scenes;

use qingjian_render::{FontLibrary, HitTarget, Layout, Renderer, Theme};

#[test]
fn reversed_rows_preserve_candidate_and_translation_targets() {
    let mut renderer = Renderer::new(FontLibrary::system("zh-CN").expect("需要系统字体"));
    for theme in Theme::builtins() {
        renderer.load_theme_fonts(theme, qingjian_render::system_fonts::family_files);
        for scale in [1.0, 1.5, 2.0] {
            for count in [1, 3] {
                let mut frame = scenes::nihao();
                frame.rows.truncate(count);
                frame.highlighted = Some(0);
                let theme = theme.with_animations(false);
                let normal = renderer
                    .render(&frame, Layout::Vertical, &theme, scale)
                    .unwrap();
                let reversed = renderer
                    .render_vertical_reversed(&frame, &theme, scale)
                    .unwrap();
                if theme.id() == "qingjian"
                    && scale == 1.5
                    && count == 3
                    && let Some(output) = std::env::var_os("QINGJIAN_REVIEW_DIR")
                {
                    std::fs::create_dir_all(&output).unwrap();
                    let dir = std::path::Path::new(&output);
                    normal
                        .pixmap
                        .save_png(dir.join("candidates-below.png"))
                        .unwrap();
                    reversed
                        .pixmap
                        .save_png(dir.join("candidates-above.png"))
                        .unwrap();
                }
                let candidate_y = |hits: &[qingjian_render::HitRegion], i| {
                    hits.iter()
                        .find(|h| h.target == HitTarget::Candidate(i))
                        .unwrap()
                        .y
                };
                for i in 1..frame.rows.len() {
                    assert!(
                        candidate_y(&normal.hits, i - 1) < candidate_y(&normal.hits, i),
                        "{}",
                        theme.id()
                    );
                    assert!(
                        candidate_y(&reversed.hits, i - 1) > candidate_y(&reversed.hits, i),
                        "{}",
                        theme.id()
                    );
                }
                for hit in &reversed.hits {
                    let x = if matches!(hit.target, HitTarget::Candidate(_)) {
                        hit.x + 1.0
                    } else {
                        hit.x + hit.width / 2.0
                    };
                    let y = hit.y + hit.height / 2.0;
                    assert_eq!(
                        reversed.hit_at(x / scale, y / scale),
                        Some(hit.target),
                        "{} {:?}",
                        theme.id(),
                        hit.target
                    );
                }
                assert_eq!(
                    normal.content_height,
                    reversed.content_height,
                    "{}",
                    theme.id()
                );
                let restored = renderer
                    .render(&frame, Layout::Vertical, &theme, scale)
                    .unwrap();
                assert_eq!(normal.pixmap.data(), restored.pixmap.data());
            }
        }
    }
}

#[test]
fn reversed_empty_frame_is_valid() {
    let mut renderer = Renderer::new(FontLibrary::system("zh-CN").unwrap());
    let mut frame = scenes::nihao();
    frame.rows.clear();
    frame.highlighted = None;
    let result = renderer
        .render_vertical_reversed(&frame, &Theme::light(), 1.0)
        .unwrap();
    assert!(result.hits.is_empty());
}
