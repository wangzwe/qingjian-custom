//! 候选窗口的背景、留白与阴影覆盖；主题原件和状态条保持独立。

use std::sync::Arc;

use super::Theme;
use super::file::color::parse_hex;
use super::file::node::{Edges, EffectSpec, FillSpec, NodeKind, NodeSpec};
use super::file::{ColorRef, ColorSpec};

impl Theme {
    /// -1 沿用主题留白；行内默认四像素对称留白，开启垂直居中。
    pub fn with_candidate_box(
        &self,
        background: &str,
        padding: i32,
        row_padding: i32,
        row_gap: i32,
        shadow: bool,
    ) -> Self {
        let mut theme = self.clone();
        let row_padding = row_padding.clamp(-1, 32);
        let row_gap = row_gap.clamp(0, 24);
        theme.candidate_layout = Some((row_padding, row_gap));
        let file = Arc::make_mut(&mut theme.file);
        let mut rows = Vec::new();
        for root in [&mut file.windows.vertical, &mut file.windows.horizontal] {
            if padding >= 0 {
                root.layout.padding = Some(Edges::all(padding.min(32) as f32));
            }
            if !shadow && let Some(effects) = root.layout.effects.as_mut() {
                effects.retain(|effect| !matches!(effect, EffectSpec::DropShadow(_)));
            }
            if let Some(color) = parse_hex(background)
                && let NodeKind::Frame { fill, .. } = &mut root.kind
            {
                *fill = Some(FillSpec::Color(ColorSpec::Fixed(ColorRef::Literal(color))));
            }
            configure_rows(root, row_padding, row_gap, &mut rows);
        }
        for name in rows {
            if let Some(row) = file.components.get_mut(&name) {
                row.layout.padding = Some(Edges::all(if row_padding < 0 {
                    4.0
                } else {
                    row_padding as f32
                }));
            }
        }
        theme
    }
}

fn configure_rows(node: &mut NodeSpec, padding: i32, gap: i32, rows: &mut Vec<String>) {
    if let NodeKind::Frame {
        table, children, ..
    } = &mut node.kind
    {
        let contains_candidates = children.iter().any(|child| {
            matches!(&child.kind,
            NodeKind::Repeat { bind, .. } if bind == "candidates")
        });
        if contains_candidates {
            node.layout.gap = Some(gap as f32);
            if let Some(table) = table {
                if padding >= 0 {
                    table.row_height = 0.0;
                }
            } else {
                for child in children.iter() {
                    if let NodeKind::Repeat { bind, component } = &child.kind
                        && bind == "candidates"
                    {
                        rows.push(component.clone());
                    }
                }
            }
        }
        for child in children {
            configure_rows(child, padding, gap, rows);
        }
    }
}
