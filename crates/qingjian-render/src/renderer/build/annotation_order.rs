//! 只改变显示顺序；每个片段仍携带原义项编号，点击上屏不变。

use crate::frame::Tone;

pub(super) fn arrange(
    segments: Vec<(&(String, Tone), usize)>,
    after: bool,
) -> Vec<((String, Tone), usize)> {
    let mut result = Vec::with_capacity(segments.len());
    for group in segments.chunk_by(|a, b| a.1 == b.1) {
        let move_pos = after
            && group
                .iter()
                .any(|((_, tone), _)| matches!(tone, Tone::Gloss | Tone::Fresh | Tone::Faint));
        for (segment, sense) in group {
            if !move_pos || segment.1 != Tone::Pos {
                result.push(((*segment).clone(), *sense));
            }
        }
        if move_pos {
            for ((text, tone), sense) in group {
                if *tone == Tone::Pos && !text.trim().is_empty() {
                    result.push(((format!(" {}", text.trim()), *tone), *sense));
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_changes_per_sense_without_changing_click_identity() {
        let source = [
            (("int. ".into(), Tone::Pos), 0),
            (("hello".into(), Tone::Gloss), 0),
            ((" · ".into(), Tone::Separator), 1),
            (("int. ".into(), Tone::Pos), 1),
            (("hi".into(), Tone::Fresh), 1),
        ];
        let picked = || {
            source
                .iter()
                .map(|(segment, sense)| (segment, *sense))
                .collect()
        };
        assert_eq!(arrange(picked(), false), source);
        let moved = arrange(picked(), true);
        assert_eq!(
            moved
                .iter()
                .map(|((text, _), _)| text.as_str())
                .collect::<String>(),
            "hello int. · hi int."
        );
        assert_eq!(
            moved.iter().map(|(_, sense)| *sense).collect::<Vec<_>>(),
            [0, 0, 1, 1, 1]
        );
        assert_eq!(
            arrange(vec![(&source[0].0, 0)], true),
            vec![source[0].clone()]
        );
        assert!(arrange(Vec::new(), true).is_empty());
    }
}
