//! 候选窗口主题：字体、颜色、间距。视觉层级对齐 macOS 端；GDI 没有随外观切换的语义色，浅 / 深各写一套（[`Palette`]）。

mod palette;

use qingjian_platform::FontSize;
use windows::Win32::Foundation::COLORREF;
use windows::Win32::Graphics::Gdi::{
    CLEARTYPE_QUALITY, CLIP_DEFAULT_PRECIS, CreateFontW, DEFAULT_CHARSET, DeleteObject,
    FF_DONTCARE, HFONT, OUT_TT_PRECIS, VARIABLE_PITCH,
};
use windows::core::{PCWSTR, w};

use self::palette::Palette;

/// 常规字重；windows crate 未导出。
const FW_NORMAL: i32 = 400;
const FW_BOLD: i32 = 700;

/// COLORREF 低位到高位是 R、G、B。
pub(super) const fn rgb(r: u8, g: u8, b: u8) -> COLORREF {
    COLORREF((r as u32) | ((g as u32) << 8) | ((b as u32) << 16))
}

/// 一套配色 + 按 DPI 造好的字体。字体是 GDI 资源，`Drop` 里删。
pub(crate) struct Theme {
    pub text_font: HFONT,

    pub annotation_font: HFONT,

    pub index_font: HFONT,

    /// 符号字体（状态条的齿轮 ⚙）：雅黑没有这些字形。
    pub symbol_font: HFONT,

    pub text_color: COLORREF,

    pub highlight_text_color: COLORREF,

    pub gloss_color: COLORREF,

    pub highlight_gloss_color: COLORREF,

    pub text_overrides: [Option<COLORREF>; 2],

    pub annotation_overrides: [Option<COLORREF>; 2],

    pub highlight_pos_color: COLORREF,

    pub highlight_fresh_color: COLORREF,

    pub highlight_cloud_color: COLORREF,

    pub highlight_index_color: COLORREF,

    pub pos_color: COLORREF,

    /// 生词译文，比普通译文醒目。
    pub fresh_color: COLORREF,

    pub index_color: COLORREF,

    /// 云联想的云朵与文字。
    pub cloud_color: COLORREF,

    pub background: COLORREF,

    /// 当前候选的高亮底色（mac 的半透明蓝预混成不透明值，GDI 无 alpha）。
    pub highlight: COLORREF,

    /// 窗口内边距（已按 DPI 缩放）。
    pub padding: i32,

    /// 行内上下留白。
    pub row_padding: i32,

    /// 列间距。
    pub column_gap: i32,

    /// 候选格之间的间距，以及竖排译文对齐方式。
    pub row_gap: i32,

    pub annotation_table: bool,

    pub annotation_gap: i32,

    /// 窗口圆角半径。
    pub corner_radius: i32,
}

impl Theme {
    /// `dpi` 96 为 100%。
    pub(crate) fn new(dpi: u32, dark: bool) -> Self {
        let scale = |px: i32| (px * dpi as i32) / 96;
        // 负高度 = 字符高度（不含内部行距）。
        let font = |px: i32| create_font(-scale(px), w!("Microsoft YaHei UI"), false);
        let palette = if dark {
            Palette::dark()
        } else {
            Palette::light()
        };
        Self {
            text_font: font(16),
            annotation_font: font(12),
            index_font: font(11),
            symbol_font: create_font(-scale(15), w!("Segoe UI Symbol"), false),
            text_color: palette.text_color,
            highlight_text_color: palette.text_color,
            gloss_color: palette.gloss_color,
            highlight_gloss_color: palette.gloss_color,
            text_overrides: [None; 2],
            annotation_overrides: [None; 2],
            highlight_pos_color: palette.pos_color,
            highlight_fresh_color: palette.fresh_color,
            highlight_cloud_color: palette.cloud_color,
            highlight_index_color: palette.index_color,
            pos_color: palette.pos_color,
            fresh_color: palette.fresh_color,
            index_color: palette.index_color,
            cloud_color: palette.cloud_color,
            background: palette.background,
            highlight: palette.highlight,
            padding: scale(8),
            row_padding: scale(4),
            column_gap: scale(8),
            row_gap: 0,
            annotation_table: true,
            annotation_gap: scale(8),
            corner_radius: scale(8),
        }
    }
    /// GDI 也应用独立的候选 / 译文字体、字重、字号与颜色。
    pub(crate) fn with_text_styles(
        mut self,
        dpi: u32,
        styles: &[(String, bool, String, String); 2],
        sizes: (FontSize, FontSize),
    ) -> Self {
        let theme = &mut self;
        unsafe {
            let _ = DeleteObject(theme.text_font.into());
            let _ = DeleteObject(theme.annotation_font.into());
        }
        let candidate_size = font_height(dpi, sizes.0, 16);
        let annotation_size = font_height(dpi, sizes.1, 12);
        theme.text_font = create_font_family(-candidate_size, &styles[0].0, styles[0].1);
        theme.annotation_font = create_font_family(-annotation_size, &styles[1].0, styles[1].1);
        let candidate = parse_color(&styles[0].2);
        let annotation = parse_color(&styles[1].2);
        theme.text_overrides = [candidate, parse_color(&styles[0].3).or(candidate)];
        theme.annotation_overrides = [annotation, parse_color(&styles[1].3).or(annotation)];
        theme.text_color = candidate.unwrap_or(theme.text_color);
        theme.highlight_text_color = parse_color(&styles[0].3)
            .or(candidate)
            .unwrap_or(theme.highlight_text_color);
        theme.gloss_color = annotation.unwrap_or(theme.gloss_color);
        theme.highlight_gloss_color = parse_color(&styles[1].3)
            .or(annotation)
            .unwrap_or(theme.highlight_gloss_color);
        self
    }

    /// GDI 不支持 alpha，语义色分别预混到普通背景和选中背景。
    pub(crate) fn with_palette(mut self, source: &qingjian_render::Theme) -> Self {
        let color = |name, background, fallback| {
            source
                .palette_color(name)
                .map_or(fallback, |color| blend(color, background))
        };
        self.highlight = color("accent", self.background, self.highlight);
        self.text_color = color("text", self.background, self.text_color);
        self.gloss_color = color("gloss", self.background, self.gloss_color);
        self.pos_color = color("faint", self.background, self.pos_color);
        self.index_color = self.pos_color;
        self.fresh_color = color("fresh", self.background, self.fresh_color);
        self.cloud_color = color("cloud", self.background, self.cloud_color);
        self.highlight_text_color = color("hl_text", self.highlight, self.text_color);
        self.highlight_gloss_color = color("hl_gloss", self.highlight, self.gloss_color);
        self.highlight_pos_color = color("hl_faint", self.highlight, self.pos_color);
        self.highlight_index_color = self.highlight_pos_color;
        self.highlight_fresh_color = color("hl_fresh", self.highlight, self.fresh_color);
        self.highlight_cloud_color = color("hl_cloud", self.highlight, self.cloud_color);
        self
    }

    pub(crate) fn with_candidate_box(
        mut self,
        background: &str,
        (padding, row_padding, row_gap): (i32, i32, i32),
        annotation_table: bool,
        annotation_gap: i32,
        dpi: u32,
    ) -> Self {
        let scale = |px: i32| (px * dpi as i32) / 96;
        if let Some(color) = parse_color(background) {
            self.background = color;
        }
        if padding >= 0 {
            self.padding = scale(padding.clamp(0, 32));
        }
        if row_padding >= 0 {
            self.row_padding = scale(row_padding.clamp(0, 32));
        }
        self.row_gap = scale(row_gap.clamp(0, 24));
        self.annotation_table = annotation_table;
        self.annotation_gap = scale(annotation_gap.clamp(0, 64));
        self
    }
}

fn blend(color: qingjian_render::Color, background: COLORREF) -> COLORREF {
    let alpha = u32::from(color.a);
    let channel = |value: u8, shift: u32| {
        ((u32::from(value) * alpha + ((background.0 >> shift) & 255) * (255 - alpha) + 127) / 255)
            as u8
    };
    rgb(
        channel(color.r, 0),
        channel(color.g, 8),
        channel(color.b, 16),
    )
}

fn font_height(dpi: u32, size: FontSize, default_px: i32) -> i32 {
    size.get().map_or_else(
        || (default_px * dpi as i32) / 96,
        |points| (points.clamp(6.0, 72.0) * dpi as f32 / 72.0).round() as i32,
    )
}

fn parse_color(value: &str) -> Option<COLORREF> {
    let hex = value.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let value = u32::from_str_radix(hex, 16).ok()?;
    Some(rgb((value >> 16) as u8, (value >> 8) as u8, value as u8))
}

impl Drop for Theme {
    fn drop(&mut self) {
        for font in [
            self.text_font,
            self.annotation_font,
            self.index_font,
            self.symbol_font,
        ] {
            if !font.is_invalid() {
                let _ = unsafe { DeleteObject(font.into()) };
            }
        }
    }
}

/// 缺字由 GDI 字体链回落。`height` 为负的字符高度。
fn create_font(height: i32, face: PCWSTR, bold: bool) -> HFONT {
    create_font_with_weight(height, face, bold)
}

fn create_font_family(height: i32, family: &str, bold: bool) -> HFONT {
    let family = if family.trim().is_empty() {
        "Microsoft YaHei UI"
    } else {
        family.trim()
    };
    let mut face: Vec<u16> = family.encode_utf16().collect();
    face.push(0);
    create_font_with_weight(height, PCWSTR(face.as_ptr()), bold)
}

fn create_font_with_weight(height: i32, face: PCWSTR, bold: bool) -> HFONT {
    unsafe {
        CreateFontW(
            height,
            0,
            0,
            0,
            if bold { FW_BOLD } else { FW_NORMAL },
            0,
            0,
            0,
            DEFAULT_CHARSET,
            OUT_TT_PRECIS,
            CLIP_DEFAULT_PRECIS,
            CLEARTYPE_QUALITY,
            (VARIABLE_PITCH.0 | FF_DONTCARE.0) as u32,
            face,
        )
    }
}
