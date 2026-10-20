//! 固定间距不受其他候选宽度影响，倒序、缩放和译词点击保持一致。

mod scenes;
use qingjian_render::{FontLibrary, HitTarget, Layout, Rendered, Renderer, Theme, Tone};

fn translation_x(rendered: &Rendered, candidate: usize) -> f32 {
    rendered
        .hits
        .iter()
        .find(|hit| {
            hit.target
                == HitTarget::Translation {
                    candidate,
                    sense: 0,
                }
        })
        .unwrap()
        .x
}

#[test]
fn fixed_gap_is_local_to_each_row_and_scales_without_affecting_horizontal() {
    let mut renderer = Renderer::new(FontLibrary::system("zh-CN").unwrap());
    let mut short = scenes::nihao();
    short.rows.truncate(2);
    short.rows[0].text = "你".into();
    short.rows[1].text = "你好".into();
    let mut long = short.clone();
    long.rows[1].text = "这是一条很长的中文候选词".into();
    for original in Theme::builtins() {
        let base = original
            .with_candidate_box("#F2F2F2", 8, 6, 3, false)
            .with_animations(false);
        renderer.load_theme_fonts(&base, qingjian_render::system_fonts::family_files);
        for scale in [1.0, 1.5, 2.0] {
            let fixed = base.with_annotation_layout(false, 8, true);
            let wider = base.with_annotation_layout(false, 24, true);
            let a = renderer
                .render(&short, Layout::Vertical, &fixed, scale)
                .unwrap();
            let b = renderer
                .render(&long, Layout::Vertical, &fixed, scale)
                .unwrap();
            let c = renderer
                .render(&long, Layout::Vertical, &wider, scale)
                .unwrap();
            assert!(
                (translation_x(&a, 0) - translation_x(&b, 0)).abs() <= 1.0,
                "{}: other row moved translation",
                base.id()
            );
            assert!(
                (translation_x(&c, 0) - translation_x(&b, 0) - 16.0 * scale).abs() <= 1.0,
                "{}: gap did not scale",
                base.id()
            );
            let reversed = renderer
                .render_vertical_reversed(&long, &fixed, scale)
                .unwrap();
            assert!((translation_x(&reversed, 0) - translation_x(&b, 0)).abs() <= 1.0);
            for rendered in [&b, &reversed] {
                for candidate in 0..2 {
                    let target = HitTarget::Translation {
                        candidate,
                        sense: 0,
                    };
                    let hit = rendered
                        .hits
                        .iter()
                        .find(|hit| hit.target == target)
                        .unwrap();
                    assert_eq!(
                        rendered.hit_at(
                            (hit.x + hit.width / 2.0) / scale,
                            (hit.y + hit.height / 2.0) / scale
                        ),
                        Some(target)
                    );
                }
            }
            let table = base.with_annotation_layout(true, 8, true);
            let h1 = renderer
                .render(&long, Layout::Horizontal, &table, scale)
                .unwrap();
            let h2 = renderer
                .render(&long, Layout::Horizontal, &fixed, scale)
                .unwrap();
            assert_eq!(h1.pixmap, h2.pixmap, "{} horizontal changed", base.id());
            if ["qingjian", "wechat", "system-blue", "sakura"].contains(&base.id()) {
                let t1 = renderer
                    .render(&short, Layout::Vertical, &table, scale)
                    .unwrap();
                let t2 = renderer
                    .render(&long, Layout::Vertical, &table, scale)
                    .unwrap();
                assert!(translation_x(&t2, 0) > translation_x(&t1, 0) + 80.0 * scale);
            }
        }
    }
}

#[test]
fn pos_after_preserves_multiple_sense_hits_and_renders_examples() {
    let mut renderer = Renderer::new(FontLibrary::system("zh-CN").unwrap());
    let mut frame = scenes::nihao();
    frame.rows.truncate(3);
    frame.rows[0] = scenes::annotated(
        0,
        "你",
        &[("pron. ", Tone::Pos), ("you", Tone::Gloss)],
        false,
    );
    frame.rows[1] = scenes::annotated(
        1,
        "一条比较长的中文",
        &[("n. ", Tone::Pos), ("a longer phrase", Tone::Gloss)],
        false,
    );
    frame.rows[2] = scenes::annotated(
        2,
        "你好",
        &[
            ("int. ", Tone::Pos),
            ("hello", Tone::Gloss),
            (" · ", Tone::Separator),
            ("int. ", Tone::Pos),
            ("hi", Tone::Fresh),
        ],
        false,
    );
    for table in [true, false] {
        let theme = Theme::light()
            .with_candidate_box("#F2F2F2", 8, 6, 3, false)
            .with_annotation_layout(table, 8, true)
            .with_animations(false);
        let result = renderer
            .render(&frame, Layout::Vertical, &theme, 2.0)
            .unwrap();
        for sense in 0..2 {
            let target = HitTarget::Translation {
                candidate: 2,
                sense,
            };
            let hit = result.hits.iter().find(|hit| hit.target == target).unwrap();
            assert_eq!(
                result.hit_at(
                    (hit.x + hit.width / 2.0) / 2.0,
                    (hit.y + hit.height / 2.0) / 2.0
                ),
                Some(target)
            );
        }
        if let Some(dir) = std::env::var_os("QINGJIAN_REVIEW_DIR") {
            std::fs::create_dir_all(&dir).unwrap();
            result
                .pixmap
                .save_png(std::path::Path::new(&dir).join(if table {
                    "translation-table-v3.png"
                } else {
                    "translation-fixed-v3.png"
                }))
                .unwrap();
        }
    }
}
