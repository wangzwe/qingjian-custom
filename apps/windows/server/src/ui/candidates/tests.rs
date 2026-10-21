//! 在真实 Windows GDI 位图中验证选中背景、候选及各类译文颜色。
use std::rc::Rc;

use qingjian_platform::{FontSize, LayoutMode};
use qingjian_render::{Row, ThemeLibrary, Tone};
use windows::Win32::Foundation::RECT;

use super::colors::{tone_color, word_color};
use super::theme::{Theme, rgb};
use super::{RenderData, view};
use crate::ui::layered::canvas::Canvas;

fn theme(selected: bool) -> Theme {
    let themes = ThemeLibrary::load(None);
    Theme::new(96, false)
        .with_candidate_box("#F2F2F2", (2, 1, 2), false, 4, 96)
        .with_palette(&themes.resolve("system-blue", false))
        .with_text_styles(
            96,
            &[
                (
                    "Microsoft YaHei".to_owned(),
                    true,
                    "#123456".to_owned(),
                    if selected { "#ff0000" } else { "" }.to_owned(),
                ),
                (
                    "Microsoft YaHei".to_owned(),
                    true,
                    "#654321".to_owned(),
                    if selected { "#ff00ff" } else { "" }.to_owned(),
                ),
            ],
            (FontSize(24.0), FontSize(24.0)),
        )
}

#[test]
fn gdi_overrides_apply_to_cloud_fresh_pos_and_selected_fallback() {
    for selected_override in [false, true] {
        let theme = theme(selected_override);
        for selected in [false, true] {
            let word = if selected && selected_override {
                rgb(255, 0, 0)
            } else {
                rgb(0x12, 0x34, 0x56)
            };
            let gloss = if selected && selected_override {
                rgb(255, 0, 255)
            } else {
                rgb(0x65, 0x43, 0x21)
            };
            for cloud in [false, true] {
                assert_eq!(word_color(&theme, cloud, selected), word);
            }
            for tone in [
                Tone::Gloss,
                Tone::Fresh,
                Tone::Pos,
                Tone::Faint,
                Tone::Separator,
                Tone::Code,
            ] {
                assert_eq!(tone_color(&theme, tone, selected), gloss);
            }
        }
    }
}

#[test]
fn gdi_theme_accent_and_alpha_are_preserved() {
    let themes = ThemeLibrary::load(None);
    let base = || Theme::new(96, false).with_candidate_box("#F2F2F2", (2, 1, 2), false, 4, 96);
    let green = base().with_palette(&themes.resolve("qingjian", false));
    assert_eq!(green.highlight, rgb(195, 216, 162));
    let blue = base().with_palette(&themes.resolve("system-blue", false));
    assert_eq!(blue.highlight, rgb(0, 122, 255));
    assert_eq!(blue.highlight_text_color, rgb(255, 255, 255));
    assert_eq!(word_color(&blue, true, true), rgb(255, 255, 255));
    assert_eq!(tone_color(&blue, Tone::Fresh, true), rgb(255, 224, 178));
    assert_ne!(blue.highlight_index_color, blue.index_color);
}

#[test]
fn gdi_bitmap_contains_selected_background_and_both_text_colors() {
    for (layout, columns, reversed) in [
        (LayoutMode::Vertical, 0, false),
        (LayoutMode::Vertical, 0, true),
        (LayoutMode::Horizontal, 0, false),
        (LayoutMode::Horizontal, 2, false),
    ] {
        let mut data = RenderData::empty(Rc::new(theme(true)));
        data.layout = layout;
        data.columns = columns;
        data.reversed = reversed;
        data.highlight = 0;
        data.rows = vec![
            Row {
                index: "1".to_owned(),
                text: "MMMM".to_owned(),
                code: None,
                cloud: true,
                annotation: vec![
                    ("MMMM".to_owned(), Tone::Fresh),
                    ("MMMM".to_owned(), Tone::Pos),
                ],
            },
            Row {
                index: "2".to_owned(),
                text: "MMMM".to_owned(),
                code: None,
                cloud: false,
                annotation: vec![("MMMM".to_owned(), Tone::Gloss)],
            },
        ];
        let mut canvas = Canvas::new(900, 300).unwrap();
        let rect = RECT {
            left: 0,
            top: 0,
            right: 900,
            bottom: 300,
        };
        view::fill_rect(canvas.dc(), rect, data.theme.background);
        view::paint(canvas.dc(), &data, rect);
        let pixels = canvas.pixels();
        for bgr in [[255, 122, 0], [0, 0, 255], [255, 0, 255]] {
            assert!(
                pixels.chunks_exact(4).any(|pixel| pixel[..3] == bgr),
                "missing color {bgr:?}, {layout:?}, columns={columns}, reversed={reversed}"
            );
        }
    }
}
