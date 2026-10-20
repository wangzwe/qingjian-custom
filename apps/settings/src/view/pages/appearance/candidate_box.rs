//! 候选框的背景、阴影和三组留白控制。

use crate::message::{FieldId, Message};
use crate::view::Page;
use crate::view::parts::controls::toggle_with;
use bamboo_slip::taffy::NodeId;
use bamboo_slip::{Button, RenderError, SettingRow, Stepper, Ui, group, section_title};

pub(super) fn controls(
    ui: &mut Ui<Message>,
    page: &Page,
    enabled: bool,
) -> Result<Vec<NodeId>, RenderError> {
    let g = &page.settings.config.general;
    let mut rows = vec![
        super::text_style::color(
            ui,
            page,
            FieldId::CandidateBackground,
            "候选框背景色",
            enabled,
        )?,
        toggle_with(
            ui,
            page,
            "边缘阴影",
            ("general", "candidate_shadow"),
            g.candidate_shadow,
            !enabled,
        )?,
    ];
    for (label, key, value, min, max, info) in [
        (
            "候选框内边距",
            "candidate_padding",
            g.candidate_padding,
            -1,
            32,
            "整个候选框边缘到内容的距离；候选框与光标的距离另行设置",
        ),
        (
            "每条候选内边距",
            "candidate_row_padding",
            g.candidate_row_padding,
            -1,
            32,
            "词语与所在候选格边缘的留白，文字在上下方向居中",
        ),
        (
            "候选行间距",
            "candidate_row_gap",
            g.candidate_row_gap,
            0,
            24,
            "相邻候选格之间的距离，随显示缩放调整",
        ),
    ] {
        let value = value.clamp(min, max);
        let display = if value < 0 {
            "默认".to_owned()
        } else {
            format!("{value} px")
        };
        let stepper = Stepper::new(&display)
            .limits(enabled && value > min, enabled && value < max)
            .build(
                ui,
                &page.theme,
                key,
                Message::set("general", key, i64::from((value - 1).max(min))),
                Message::set("general", key, i64::from((value + 1).min(max))),
            )?;
        let reset = Button::new("恢复默认")
            .disabled(!enabled || value == min)
            .build(
                ui,
                &page.theme,
                ("reset", key),
                Message::set("general", key, i64::from(min)),
            )?;
        rows.push(
            SettingRow::new(label)
                .info(info)
                .control(stepper)
                .below(reset)
                .build(ui, &page.theme)?,
        );
    }
    Ok(vec![
        section_title(ui, &page.theme, "候选框样式")?,
        group(ui, &page.theme, &rows)?,
    ])
}
