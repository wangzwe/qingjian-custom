//! 上下键按实际候选位置导航，数字键与横排不受影响。

use super::tests::{compose, key, router};
use crate::dispatch::{CandidateSink, PanelFrame, RenderSettings};
use qingjian_platform::{
    LayoutMode,
    protocol::{Frame, KeyModifiers, ScreenRect},
};
use qingjian_render::Mode;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

struct PositionedSink(Arc<AtomicBool>);

impl CandidateSink for PositionedSink {
    fn vertical_reversed(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
    fn show(&self, _: Frame, _: Mode, _: ScreenRect) {}
    fn hide(&self) {}
    fn show_panel(&self, _: PanelFrame, _: Option<ScreenRect>) {}
    fn hide_panel(&self) {}
    fn configure(&self, _: RenderSettings) {}
}

#[test]
fn arrows_follow_visible_direction_and_switch_back() {
    let mut router = router(5);
    router.config.layout = LayoutMode::Vertical;
    let reversed = Arc::new(AtomicBool::new(true));
    router.set_candidate_sink(Box::new(PositionedSink(Arc::clone(&reversed))));
    let normal = KeyModifiers::default();
    compose(&mut router, "qq", normal);
    assert_eq!(key(&mut router, 0x26, None, normal).2.highlight, 1);
    assert!(router.panel.is_none(), "倒序时上键不能抢先打开符号面板");
    assert_eq!(key(&mut router, 0x26, None, normal).2.highlight, 2);
    assert_eq!(key(&mut router, 0x28, None, normal).2.highlight, 1);
    reversed.store(false, Ordering::Release);
    let frame = key(&mut router, 0x28, None, normal).2;
    assert_eq!(frame.highlight, 2);
    assert_eq!(key(&mut router, 0x26, None, normal).2.highlight, 1);
    let expected = frame.candidates.items[0].text.clone();
    assert_eq!(key(&mut router, 0x31, Some('1'), normal).1, Some(expected));
}

#[test]
fn horizontal_arrows_ignore_stale_vertical_direction() {
    let mut router = router(5);
    router.config.layout = LayoutMode::Horizontal;
    router.set_candidate_sink(Box::new(PositionedSink(Arc::new(AtomicBool::new(true)))));
    let normal = KeyModifiers::default();
    compose(&mut router, "qq", normal);
    assert_eq!(key(&mut router, 0x28, None, normal).2.highlight, 1);
    assert_eq!(key(&mut router, 0x26, None, normal).2.highlight, 0);
}
