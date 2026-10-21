//! GDI 文字测量和基础绘制，候选窗与状态条共用。

use super::theme::Theme;
use windows::Win32::Foundation::{COLORREF, RECT, SIZE};
use windows::Win32::Graphics::Gdi::{
    CreateRoundRectRgn, CreateSolidBrush, DeleteObject, FillRect, FillRgn, GetTextExtentPoint32W,
    HDC, HFONT, SelectObject, SetTextColor, TextOutW,
};

/// 云端候选词前的小云朵（macOS 用 SF Symbol `cloud`）。
pub(super) const CLOUD_GLYPH: &str = "☁";

/// 矩阵里一格里候选词最多多宽（按候选字宽的倍数；帧没给列宽时用），与 Core `Grid::column_ems` 的封顶一致。
pub(super) const MAX_CELL_EMS: f32 = 4.0;

/// 超宽候选截尾补的省略号。
pub(super) const ELLIPSIS: &str = "…";

/// 画一段文字，`(x, y)` 是左上角，返回宽度。调用方先 `SetBkMode(TRANSPARENT)`。
pub(crate) fn draw_text(hdc: HDC, font: HFONT, color: COLORREF, x: i32, y: i32, text: &str) -> i32 {
    let utf16: Vec<u16> = text.encode_utf16().collect();
    let mut size = SIZE::default();
    unsafe {
        SelectObject(hdc, font.into());
        SetTextColor(hdc, color);
        let _ = TextOutW(hdc, x, y, &utf16);
        let _ = GetTextExtentPoint32W(hdc, &utf16, &mut size);
    }
    size.cx
}

pub(crate) fn measure(hdc: HDC, font: HFONT, text: &str) -> SIZE {
    let utf16: Vec<u16> = text.encode_utf16().collect();
    let mut size = SIZE::default();
    unsafe {
        SelectObject(hdc, font.into());
        let _ = GetTextExtentPoint32W(hdc, &utf16, &mut size);
    }
    size
}

pub(super) fn line_height(hdc: HDC, font: HFONT) -> i32 {
    measure(hdc, font, "x").cy
}

/// 细线 / 光标的粗细，随 DPI 放大。
pub(super) fn scale_line(theme: &Theme) -> i32 {
    (theme.padding / 8).max(1)
}

pub(crate) fn fill_rect(hdc: HDC, rect: RECT, color: COLORREF) {
    unsafe {
        let brush = CreateSolidBrush(color);
        FillRect(hdc, &rect, brush);
        let _ = DeleteObject(brush.into());
    }
}

pub(super) fn fill_round_rect(hdc: HDC, rect: RECT, color: COLORREF, radius: i32) {
    let diameter = (radius * 2).max(1);
    unsafe {
        let region = CreateRoundRectRgn(
            rect.left,
            rect.top,
            rect.right,
            rect.bottom,
            diameter,
            diameter,
        );
        let brush = CreateSolidBrush(color);
        let _ = FillRgn(hdc, region, brush);
        let _ = DeleteObject(brush.into());
        let _ = DeleteObject(region.into());
    }
}
