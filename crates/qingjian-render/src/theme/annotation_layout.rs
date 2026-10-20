//! 竖排译文的逐行固定间距；只复制竖排组件，横排和原始主题不受影响。

use std::collections::HashMap;
use std::sync::Arc;

use super::Theme;
use super::file::node::{
    Align, BoxSpec, Direction, Edges, Length, NodeKind, NodeSpec, Position, Span,
};

impl Theme {
    /// 表格模式保留主题的列布局，固定模式按各行候选的实际宽度排译文。
    pub fn with_annotation_layout(&self, table: bool, gap: i32, pos_after: bool) -> Self {
        let mut theme = self.clone();
        theme.annotation_pos_after = pos_after;
        if table {
            return theme;
        }
        let (padding, row_gap) = theme.candidate_layout.unwrap_or((-1, 0));
        let file = Arc::make_mut(&mut theme.file);
        let source = file.components.clone();
        fixed_rows(
            &mut file.windows.vertical,
            &source,
            &mut file.components,
            gap.clamp(0, 64) as f32,
            if padding < 0 { 4.0 } else { padding as f32 },
            row_gap as f32,
        );
        theme
    }
}

fn fixed_rows(
    node: &mut NodeSpec,
    source: &HashMap<String, NodeSpec>,
    components: &mut HashMap<String, NodeSpec>,
    gap: f32,
    padding: f32,
    row_gap: f32,
) {
    if let NodeKind::Frame {
        direction,
        table,
        children,
        ..
    } = &mut node.kind
    {
        if children.iter().any(
            |child| matches!(&child.kind, NodeKind::Repeat { bind, .. } if bind == "candidates"),
        ) {
            *table = None;
            *direction = Direction::Column;
            node.layout.gap = Some(row_gap);
            for child in children.iter_mut() {
                if let NodeKind::Repeat { bind, component } = &mut child.kind
                    && bind == "candidates"
                    && let Some(row) = source.get(component)
                {
                    let mut row = expanded(row, source, 0);
                    inline(&mut row, gap);
                    row.layout.padding = Some(Edges::all(padding));
                    // Repeat 的覆盖放在居中处理之后，保证每行背景仍铺满窗口。
                    child.layout.align_self = Some(Align::Stretch);
                    let name = format!("__fixed_gap_{component}");
                    components.insert(name.clone(), row);
                    *component = name;
                }
            }
        }
        for child in children {
            fixed_rows(child, source, components, gap, padding, row_gap);
        }
    }
}

fn expanded(node: &NodeSpec, components: &HashMap<String, NodeSpec>, depth: usize) -> NodeSpec {
    let mut node = node.clone();
    if depth >= 16 {
        return node;
    }
    if let NodeKind::Use { component } = &node.kind
        && let Some(component) = components.get(component)
    {
        let mut inner = expanded(component, components, depth + 1);
        inner.layout = inner.layout.overridden_by(&node.layout);
        if node.when.is_some() && inner.when.is_some() {
            // 两层条件分别保留，不把“或”表达式误拼成“与”。
            node.kind = NodeKind::Frame {
                direction: Direction::Row,
                fill: None,
                border: None,
                radius: 0.0,
                table: None,
                children: vec![inner],
            };
            node.layout = BoxSpec::default();
        } else {
            inner.when = node.when.or(inner.when);
            node = inner;
        }
    } else if let NodeKind::Frame { children, .. } = &mut node.kind {
        *children = children
            .iter()
            .map(|child| expanded(child, components, depth + 1))
            .collect();
    }
    node
}

fn contains(node: &NodeSpec, annotation: bool) -> bool {
    match &node.kind {
        NodeKind::Annotation { bind, .. } => annotation && bind == "annotation",
        NodeKind::Text { bind, .. } => !annotation && bind.as_deref() == Some("text"),
        NodeKind::Frame { children, .. } => {
            children.iter().any(|child| contains(child, annotation))
        }
        _ => false,
    }
}

fn inline(node: &mut NodeSpec, gap: f32) {
    if node.layout.span == Some(Span::Row) {
        node.layout.span = None;
        node.layout.position = Some(Position::Absolute);
        node.layout.inset = Some(Edges::all(0.0));
        node.layout.margin = Some(Edges::all(0.0));
    }
    if let NodeKind::Frame {
        direction,
        children,
        ..
    } = &mut node.kind
    {
        let split = children
            .iter()
            .any(|child| contains(child, false) && !contains(child, true))
            && children
                .iter()
                .any(|child| contains(child, true) && !contains(child, false));
        if split {
            *direction = Direction::Row;
            node.layout.gap = Some(0.0);
            for child in children.iter_mut() {
                if contains(child, true) && !contains(child, false) {
                    let mut margin = child.layout.margin.unwrap_or(Edges::all(0.0));
                    margin.left = Length::Points(gap);
                    child.layout.margin = Some(margin);
                }
                if contains(child, false)
                    && !contains(child, true)
                    && let Some(margin) = child.layout.margin.as_mut()
                {
                    margin.right = Length::Points(0.0);
                }
            }
        }
        for child in children {
            inline(child, gap);
        }
    }
}
