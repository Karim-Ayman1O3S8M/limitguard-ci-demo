use limitguard::filter::{average, prefill, Filter, WINDOW};
use limitguard::protect::{classify, should_trip, Reading, Zone};
use limitguard::telemetry::format_telemetry;

fn reading(value_kpa: u16, age_ms: u32) -> Reading {
    Reading { value_kpa, valid: true, age_ms }
}

#[test]
fn fresh_over_limit_trips() {
    assert!(should_trip(&reading(850, 50), false));
}

#[test]
fn stale_over_limit_does_not_trip() {
    assert!(!should_trip(&reading(850, 500), false));
}

#[test]
fn fresh_below_limit_does_not_trip() {
    assert!(!should_trip(&reading(400, 50), false));
}

#[test]
fn manual_trip_always_trips() {
    assert!(should_trip(&reading(400, 50), true));
}

#[test]
fn normal_reading_is_classified_normal() {
    assert_eq!(classify(&reading(400, 50)), Zone::Normal);
}

#[test]
fn prefill_sets_first_slot() {
    let mut f = Filter { window: [0; WINDOW], count: 0 };
    prefill(&mut f, 100);
    assert_eq!(f.window[0], 100);
}

#[test]
fn average_of_filled_window() {
    let f = Filter { window: [100, 200, 300, 400, 100, 200, 300, 400], count: 8 };
    assert_eq!(average(&f), 250);
}

#[test]
fn telemetry_formats_valid_reading() {
    let line = format_telemetry(&reading(412, 50), false);
    assert_eq!(line, Some("{\"kpa\":412,\"tripped\":false}".to_string()));
}

// Stage 2: gap-closing tests (they pass while FC-2 is still present).
use limitguard::filter::average_indexed;
use limitguard::protect::snapshot_reading;

#[test]
fn invalid_reading_is_classified_invalid() {
    let r = Reading { value_kpa: 400, valid: false, age_ms: 50 };
    assert_eq!(classify(&r), Zone::Invalid);
}

#[test]
fn over_limit_reading_is_classified_over_limit() {
    assert_eq!(classify(&reading(850, 50)), Zone::OverLimit);
}

#[test]
fn average_of_empty_filter_is_zero() {
    let f = Filter { window: [0; WINDOW], count: 0 };
    assert_eq!(average(&f), 0);
}

#[test]
fn average_indexed_matches_average() {
    let f = Filter { window: [100, 200, 300, 400, 100, 200, 300, 400], count: 8 };
    assert_eq!(average_indexed(&f), 250);
}

#[test]
fn snapshot_copies_the_reading() {
    let r = reading(412, 50);
    assert_eq!(snapshot_reading(&r), r);
}

#[test]
fn telemetry_of_invalid_reading_is_none() {
    let r = Reading { value_kpa: 400, valid: false, age_ms: 50 };
    assert_eq!(format_telemetry(&r, false), None);
}
