use limitguard::protect::{should_trip, Reading, SENSOR_FAULT};

#[test]
fn req04_invalid_sentinel_must_not_trip() {
    let r = Reading { value_kpa: SENSOR_FAULT, valid: false, age_ms: 50 };
    assert!(!should_trip(&r, false));
}

#[test]
fn boundary_just_below_limit_does_not_trip() {
    let r = Reading { value_kpa: 799, valid: true, age_ms: 50 };
    assert!(!should_trip(&r, false));
}

#[test]
fn boundary_at_limit_trips() {
    let r = Reading { value_kpa: 800, valid: true, age_ms: 50 };
    assert!(should_trip(&r, false));
}
