//! 候选与译文各自的字体、粗细和颜色设置。

use crate::message::{FieldId, Message, Popup};
use crate::view::Page;
use crate::view::parts::actions::with_notice;
use crate::view::parts::controls::{select, toggle_with};
use bamboo_slip::taffy::NodeId;
use bamboo_slip::{Button, RenderError, SettingRow, Stepper, TextField, Ui, group, section_title};

pub(super) fn controls(
    ui: &mut Ui<Message>,
    page: &Page,
    enabled: bool,
    style_enabled: bool,
    colors_enabled: bool,
) -> Result<Vec<NodeId>, RenderError> {
    let g = &page.settings.config.general;
    let mut out = Vec::new();
    out.extend(super::candidate_box::controls(ui, page, enabled)?);
    out.extend(super::annotation_layout::controls(ui, page, enabled)?);
    let gap = g.candidate_gap.clamp(-1, 40);
    let label = if gap < 0 {
        "自动".to_owned()
    } else {
        format!("{gap} px")
    };
    let input = Stepper::new(&label).limits(gap > -1, gap < 40).build(
        ui,
        &page.theme,
        "candidate-gap",
        Message::set("general", "candidate_gap", i64::from((gap - 1).max(-1))),
        Message::set("general", "candidate_gap", i64::from((gap + 1).min(40))),
    )?;
    let reset = Button::new("恢复自动间距").disabled(gap < 0).build(
        ui,
        &page.theme,
        "reset-gap",
        Message::set("general", "candidate_gap", -1_i64),
    )?;
    out.push(section_title(ui, &page.theme, "候选框位置")?);
    out.push(
        SettingRow::new("与光标间距")
            .info("自动避让阴影；0–40 px 按窗口内容边缘计算，0 最靠近输入行，随显示缩放调整")
            .control(input)
            .below(reset)
            .build(ui, &page.theme)?,
    );
    for (label, popup, family, bold_key, bold, normal, selected) in [
        (
            "候选文字",
            Popup::CandidateFont,
            &g.candidate_font,
            "candidate_bold",
            g.candidate_bold,
            FieldId::CandidateColor,
            FieldId::CandidateHighlightColor,
        ),
        (
            "译文文字",
            Popup::AnnotationFont,
            &g.annotation_font,
            "annotation_bold",
            g.annotation_bold,
            FieldId::AnnotationColor,
            FieldId::AnnotationHighlightColor,
        ),
    ] {
        let font = select(
            ui,
            page,
            popup,
            (
                if family.is_empty() {
                    "跟随主题"
                } else {
                    family
                },
                &[],
            ),
            180.0,
            !style_enabled,
        )?;
        let rows = vec![
            SettingRow::new("字体")
                .control(font)
                .build(ui, &page.theme)?,
            toggle_with(ui, page, "加粗", ("general", bold_key), bold, !style_enabled)?,
            color(ui, page, normal, "文字颜色", colors_enabled)?,
            color(ui, page, selected, "选中时颜色", colors_enabled)?,
        ];
        out.push(section_title(ui, &page.theme, label)?);
        out.push(group(ui, &page.theme, &rows)?);
    }
    let reset = Button::new("恢复主题文字样式")
        .disabled(!style_enabled && !colors_enabled)
        .build(
            ui,
            &page.theme,
            "reset-text-style",
            Message::ResetTextStyles,
        )?;
    out.push(with_notice(
        ui,
        page,
        SettingRow::new("文字样式").control(reset),
        "appearance",
    )?);
    Ok(out)
}

pub(super) fn color(
    ui: &mut Ui<Message>,
    page: &Page,
    id: FieldId,
    label: &str,
    enabled: bool,
) -> Result<NodeId, RenderError> {
    let edit = &page.settings.fields[&id];
    let input = TextField::new(edit)
        .placeholder(if id == FieldId::CandidateBackground {
            "浅灰 #F2F2F2，留空跟随主题"
        } else {
            "#RRGGBB，留空跟随主题"
        })
        .width(220.0)
        .disabled(!enabled)
        .build(
            ui,
            &page.theme,
            ("field", id),
            move |edit| Message::Edit(id, edit),
            Some(Message::Commit(id)),
            Some(Message::Cancel(id)),
        )?;
    SettingRow::new(label).control(input).build(ui, &page.theme)
}
