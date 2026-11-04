use crate::events::TapType;
use crate::touch::{
    TapClassifier, TouchDetector, CALIBRATION_SAMPLES, DOUBLE_TAP_MS, LONG_PRESS_MS, STUCK_PRESS_SAMPLES,
};

#[test]
fn test_touch_threshold_from_boot_calibration() {
    assert_eq!(TouchDetector::calibrate(&[]), None);
    assert_eq!(TouchDetector::calibrate(&[0; CALIBRATION_SAMPLES]), None);
    let mut pad = TouchDetector::calibrate(&[20_000; CALIBRATION_SAMPLES]).unwrap();
    assert_eq!(pad.baseline, 20_000);
    assert_eq!(pad.update(20_400), None);
    assert_eq!(pad.update(20_401), Some(true));
    assert_eq!(pad.update(20_300), None);
    assert_eq!(pad.baseline, 20_000);
    assert_eq!(pad.update(20_190), Some(false));
    assert_eq!(pad.baseline, 20_002);
    for _ in 0..2_000 {
        pad.update(20_150);
    }
    assert!(pad.baseline > 20_080 && !pad.pressed);

    assert_eq!(pad.update(21_000), Some(true));
    for _ in 0..STUCK_PRESS_SAMPLES - 1 {
        assert_eq!(pad.update(21_000), None);
    }
    assert_eq!(pad.update(21_000), Some(false));
    assert_eq!(pad.baseline, 21_000);
    assert_eq!(pad.update(21_500), Some(true));
}

#[test]
fn test_tap_classification() {
    let mut taps = TapClassifier::default();
    assert_eq!(taps.edge(false, 0), None);
    assert_eq!(taps.edge(true, 0), None);
    assert_eq!(taps.edge(false, 80), None);
    assert_eq!(taps.poll(80 + DOUBLE_TAP_MS), None);
    assert_eq!(taps.poll(81 + DOUBLE_TAP_MS), Some(TapType::SingleTap));
    assert_eq!(taps.poll(2_000), None);

    taps.edge(true, 1_000);
    taps.edge(false, 1_090);
    taps.edge(true, 1_200);
    assert_eq!(taps.edge(false, 1_280), Some(TapType::DoubleTap));
    assert_eq!(taps.poll(5_000), None);

    taps.edge(true, 6_000);
    assert_eq!(taps.edge(false, 6_000 + LONG_PRESS_MS), Some(TapType::LongPress));
    assert_eq!(taps.poll(9_000), None);
}
