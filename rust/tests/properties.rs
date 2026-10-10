use limitguard::protect::{should_trip, Reading};
use proptest::prelude::*;

proptest! {
    #[test]
    fn invalid_reading_never_trips_automatically(value in 0u16..=u16::MAX, age in 0u32..1000) {
        let r = Reading { value_kpa: value, valid: false, age_ms: age };
        prop_assert!(!should_trip(&r, false));
    }
}
