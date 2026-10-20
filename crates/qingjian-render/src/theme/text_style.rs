//! 设置里的独立文字样式；只覆盖候选或译文，不改变拼音、序号和状态条。

use super::file::color::parse_hex;
use super::{FontFamilies, Theme};
use bamboo_slip::{Color, FamilyList, FontWeight};
use std::sync::Arc;

impl Theme {
    /// 空字族、false 和空颜色沿用主题；无效颜色忽略。
    pub fn with_text_style(
        &self,
        role: &str,
        family: &str,
        bold: bool,
        colors: (&str, &str),
    ) -> Self {
        let mut theme = self.clone();
        let file = Arc::make_mut(&mut theme.file);
        if let Some(style) = file.text.styles.get_mut(role) {
            if !family.trim().is_empty() {
                style.family = Some(FamilyList(vec![
                    family.trim().to_owned(),
                    "system".to_owned(),
                ]));
            }
            if bold {
                style.weight = FontWeight(700);
            }
        }
        theme.families = Arc::new(FontFamilies::new(&file.text));
        theme
            .text_colors
            .insert(role.to_owned(), (parse_hex(colors.0), parse_hex(colors.1)));
        theme
    }

    pub(crate) fn text_color(&self, role: &str, selected: bool, fallback: Color) -> Color {
        self.text_colors
            .get(role)
            .and_then(|colors| if selected { colors.1 } else { colors.0 })
            .unwrap_or(fallback)
    }
}
