//! 用固定工作区验证上下定位、紧贴和自定义间距。

use super::{nearer_bottom, place_in_work_area};
use windows::Win32::Foundation::RECT;

#[test]
fn custom_gap_ignores_shadow_and_keeps_the_content_outside_the_input_line() {
    let work = RECT {
        left: -1000,
        top: 0,
        right: 1000,
        bottom: 1000,
    };
    for (anchor, above) in [
        (
            RECT {
                left: 100,
                right: 102,
                top: 100,
                bottom: 120,
            },
            false,
        ),
        (
            RECT {
                left: 100,
                right: 102,
                top: 950,
                bottom: 970,
            },
            true,
        ),
    ] {
        for gap in [0, 2, 20, 40] {
            let (_, y) = place_in_work_area(anchor, (200, 200), (25, 40), work, Some(gap));
            assert_eq!(
                if above {
                    anchor.top - (y + 200)
                } else {
                    y - anchor.bottom
                },
                gap
            );
        }
        let (_, y) = place_in_work_area(anchor, (200, 200), (25, 40), work, None);
        assert_eq!(
            if above {
                anchor.top - (y + 200)
            } else {
                y - anchor.bottom
            },
            if above { 42 } else { 27 }
        );
    }
}

#[test]
fn placement_clamps_to_negative_origin_monitor_work_area() {
    let work = RECT {
        left: -1000,
        top: -800,
        right: 0,
        bottom: 0,
    };
    let anchor = RECT {
        left: -20,
        right: -18,
        top: -30,
        bottom: -10,
    };
    assert_eq!(
        place_in_work_area(anchor, (200, 200), (0, 0), work, Some(0)),
        (-200, -230)
    );
    assert_eq!(
        place_in_work_area(anchor, (1200, 1200), (0, 0), work, Some(0)),
        (-1000, -800)
    );
}

#[test]
fn orientation_uses_final_position_even_when_clamped() {
    let anchor = RECT {
        left: 0,
        right: 2,
        top: 900,
        bottom: 920,
    };
    assert!(nearer_bottom(anchor, 650, 200));
    assert!(!nearer_bottom(anchor, 930, 200));
    assert!(nearer_bottom(anchor, 0, 1000));
    assert!(!nearer_bottom(anchor, 850, 1000));
}
