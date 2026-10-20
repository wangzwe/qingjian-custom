//! 独立样式不污染主题原件、拼音或另一组文字。

use super::Theme;
use bamboo_slip::Color;

#[test]
fn independent_text_style_preserves_other_roles_and_theme() {
    let base = Theme::light();
    let custom = base
        .with_text_style("candidate", "Microsoft YaHei", true, ("#123456", "#abcdef"))
        .with_text_style("annotation", "Arial", false, ("#654321", ""));
    assert_eq!(custom.font("candidate").weight.0, 700);
    assert_ne!(
        custom.font("candidate").family,
        custom.font("annotation").family
    );
    assert_eq!(custom.font("preedit").size, base.font("preedit").size);
    assert_eq!(custom.font("preedit").weight, base.font("preedit").weight);
    assert_eq!(
        custom.font("annotation").weight,
        base.font("annotation").weight
    );
    let fallback = Color::rgb(0, 0, 0);
    assert_eq!(
        custom.text_color("candidate", false, fallback),
        Color::rgb(0x12, 0x34, 0x56)
    );
    assert_eq!(
        custom.text_color("candidate", true, fallback),
        Color::rgb(0xab, 0xcd, 0xef)
    );
    assert_eq!(custom.text_color("annotation", true, fallback), fallback);
    assert_eq!(base.text_color("candidate", false, fallback), fallback);
    assert_eq!(
        custom
            .with_dark(true)
            .text_color("candidate", false, fallback),
        Color::rgb(0x12, 0x34, 0x56)
    );
}

#[test]
fn empty_or_invalid_style_values_keep_theme_defaults() {
    let base = Theme::light();
    let custom = base.with_text_style("candidate", "", false, ("invalid", "#xyzxyz"));
    assert_eq!(custom.font("candidate"), base.font("candidate"));
    assert_eq!(
        custom.text_color("candidate", false, Color::rgb(1, 2, 3)),
        Color::rgb(1, 2, 3)
    );
}
