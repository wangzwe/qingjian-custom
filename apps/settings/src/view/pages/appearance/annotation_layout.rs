//! 译文列对齐、逐行固定间距与词性位置。

use crate::message::Message;
use crate::view::Page;
use bamboo_slip::taffy::NodeId;
use bamboo_slip::{RenderError, Segmented, SettingRow, Stepper, Ui, group, section_title};

pub(super) fn controls(
    ui: &mut Ui<Message>,
    page: &Page,
    enabled: bool,
) -> Result<Vec<NodeId>, RenderError> {
    let g = &page.settings.config.general;
    let layout = Segmented::new(&["表格对齐", "固定间距"], usize::from(!g.annotation_table))
        .disabled(!enabled)
        .build(ui, &page.theme, "annotation-table", |index| {
            Message::set("general", "annotation_table", index == 0)
        })?;
    let gap = g.annotation_gap.clamp(0, 64);
    let spacing = Stepper::new(&format!("{gap} px"))
        .limits(
            enabled && !g.annotation_table && gap > 0,
            enabled && !g.annotation_table && gap < 64,
        )
        .build(
            ui,
            &page.theme,
            "annotation-gap",
            Message::set("general", "annotation_gap", i64::from((gap - 1).max(0))),
            Message::set("general", "annotation_gap", i64::from((gap + 1).min(64))),
        )?;
    let position = Segmented::new(&["译文前", "译文后"], usize::from(g.annotation_pos_after))
        .disabled(!enabled)
        .build(ui, &page.theme, "annotation-pos-after", |index| {
            Message::set("general", "annotation_pos_after", index == 1)
        })?;
    let rows = [
        SettingRow::new("竖排译文布局")
            .info("表格对齐沿用主题列布局；固定间距让译文紧跟每行中文，不受其他行长词影响")
            .control(layout)
            .build(ui, &page.theme)?,
        SettingRow::new("中英文间距")
            .info("固定间距模式下生效，默认 8 px，随显示缩放调整")
            .control(spacing)
            .build(ui, &page.theme)?,
        SettingRow::new("词性位置")
            .info("控制已显示词性的顺序，例如 int. hello 或 hello int.，多个译词分别处理")
            .control(position)
            .build(ui, &page.theme)?,
    ];
    Ok(vec![
        section_title(ui, &page.theme, "译文排布")?,
        group(ui, &page.theme, &rows)?,
    ])
}
