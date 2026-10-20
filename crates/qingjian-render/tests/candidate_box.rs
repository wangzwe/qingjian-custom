//! 背景、无阴影、留白与倒序命中范围的组合回归。

mod scenes;
use qingjian_render::{FontLibrary, HitTarget, Layout, Renderer, Theme};

#[test]
fn candidate_ink_is_vertically_centered_with_independent_font_sizes() {
    let mut renderer = Renderer::new(FontLibrary::system("zh-CN").unwrap());
    let mut frame = scenes::nihao();
    frame.rows.truncate(1);
    for (candidate, annotation) in [(16.0, 12.0), (16.0, 26.0), (28.0, 12.0)] {
        let theme = Theme::light()
            .with_animations(false)
            .with_candidate_box("#F2F2F2", 8, 6, 0, false)
            .with_text_sizes(qingjian_render::TextSizes::new(candidate, annotation))
            .with_text_style("candidate", "", false, ("#0000FF", "#0000FF"));
        let rendered = renderer
            .render(&frame, Layout::Vertical, &theme, 2.0)
            .unwrap();
        let hit = rendered
            .hits
            .iter()
            .find(|h| h.target == HitTarget::Candidate(0))
            .unwrap();
        let mut ink = Vec::new();
        for y in hit.y as u32..(hit.y + hit.height) as u32 {
            for x in hit.x as u32..(hit.x + hit.width) as u32 {
                let index = ((y * rendered.pixmap.width() + x) * 4) as usize;
                let pixel = &rendered.pixmap.data()[index..index + 4];
                if pixel[2] > 150 && pixel[0] < 90 && pixel[1] < 90 {
                    ink.push(y);
                }
            }
        }
        let top = *ink.iter().min().expect("候选字形必须可见") as f32;
        let bottom = *ink.iter().max().unwrap() as f32 + 1.0;
        assert!(
            ((top + bottom) / 2.0 - (hit.y + hit.height / 2.0)).abs() <= 4.0,
            "candidate={candidate} annotation={annotation} ink={top}..{bottom} row={}..{}",
            hit.y,
            hit.y + hit.height
        );
    }
}

#[test]
fn candidate_box_keeps_hits_and_padding_across_themes_and_scales() {
    let mut renderer = Renderer::new(FontLibrary::system("zh-CN").unwrap());
    let mut frame = scenes::nihao();
    frame.rows.truncate(3);
    for original in Theme::builtins() {
        for scale in [1.0, 1.5, 2.0] {
            let theme = original
                .with_animations(false)
                .with_candidate_box("#F2F2F2", 8, 4, 3, false);
            renderer.load_theme_fonts(&theme, qingjian_render::system_fonts::family_files);
            for reversed in [false, true] {
                let result = if reversed {
                    renderer.render_vertical_reversed(&frame, &theme, scale)
                } else {
                    renderer.render(&frame, Layout::Vertical, &theme, scale)
                }
                .unwrap();
                for i in 0..3 {
                    let hit = result
                        .hits
                        .iter()
                        .find(|h| h.target == HitTarget::Candidate(i))
                        .unwrap();
                    assert_eq!(
                        result.hit_at((hit.x + 1.0) / scale, (hit.y + hit.height / 2.0) / scale),
                        Some(hit.target),
                        "{}",
                        theme.id()
                    );
                }
                if original.id() == "qingjian" && scale == 2.0 {
                    assert_eq!(result.content_x, 0);
                    assert_eq!(result.content_y, 0);
                    assert_eq!(result.pixmap.width(), result.content_width);
                    assert_eq!(result.pixmap.height(), result.content_height);
                    if let Some(dir) = std::env::var_os("QINGJIAN_REVIEW_DIR") {
                        std::fs::create_dir_all(&dir).unwrap();
                        result
                            .pixmap
                            .save_png(std::path::Path::new(&dir).join(if reversed {
                                "box-above-v2.png"
                            } else {
                                "box-below-v2.png"
                            }))
                            .unwrap();
                    }
                }
            }
            renderer
                .render(&frame, Layout::Horizontal, &theme, scale)
                .unwrap();
        }
    }
    let small = Theme::light()
        .with_candidate_box("#F2F2F2", 0, 0, 0, false)
        .with_animations(false);
    let large = Theme::light()
        .with_candidate_box("#F2F2F2", 12, 10, 6, false)
        .with_animations(false);
    let a = renderer
        .render(&frame, Layout::Vertical, &small, 1.0)
        .unwrap();
    let b = renderer
        .render(&frame, Layout::Vertical, &large, 1.0)
        .unwrap();
    assert!(b.content_height > a.content_height + 60);
    assert!(b.content_width > a.content_width);
}
