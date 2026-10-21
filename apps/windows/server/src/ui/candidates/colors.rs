//! 系统绘制的候选与译文颜色：用户覆盖优先于云候选、生词和词性语义色。

use super::theme::Theme;
use qingjian_render::Tone;
use windows::Win32::Foundation::COLORREF;

pub(super) fn word_color(theme: &Theme, cloud: bool, selected: bool) -> COLORREF {
    theme.text_overrides[usize::from(selected)].unwrap_or(match (cloud, selected) {
        (true, true) => theme.highlight_cloud_color,
        (true, false) => theme.cloud_color,
        (false, true) => theme.highlight_text_color,
        (false, false) => theme.text_color,
    })
}

pub(super) fn tone_color(theme: &Theme, tone: Tone, selected: bool) -> COLORREF {
    if let Some(color) = theme.annotation_overrides[usize::from(selected)] {
        return color;
    }
    match tone {
        Tone::Gloss if selected => theme.highlight_gloss_color,
        Tone::Gloss => theme.gloss_color,
        Tone::Fresh if selected => theme.highlight_fresh_color,
        Tone::Fresh => theme.fresh_color,
        Tone::Faint | Tone::Pos | Tone::Separator if selected => theme.highlight_pos_color,
        Tone::Faint | Tone::Pos | Tone::Separator => theme.pos_color,
        Tone::Code if selected => theme.highlight_gloss_color,
        Tone::Code => theme.gloss_color,
    }
}
