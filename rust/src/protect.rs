pub const TRIP_LIMIT_KPA: u16 = 800;
pub const MAX_VALID_KPA: u16 = 1000;
pub const SENSOR_FAULT: u16 = 0xFFFF;
pub const MAX_AGE_MS: u32 = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reading {
    pub value_kpa: u16,
    pub valid: bool,
    pub age_ms: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Normal,
    OverLimit,
    Invalid,
}

pub fn classify(r: &Reading) -> Zone {
    if !r.valid || r.value_kpa > MAX_VALID_KPA {
        return Zone::Invalid;
    }
    if r.value_kpa >= TRIP_LIMIT_KPA {
        Zone::OverLimit
    } else {
        Zone::Normal
    }
}

pub fn should_trip(r: &Reading, manual_trip: bool) -> bool {
    let fresh = r.age_ms <= MAX_AGE_MS;
    let over = r.value_kpa >= TRIP_LIMIT_KPA;
    // FC-2: r.valid is never consulted, so REQ-04 is not implemented.
    (fresh && over) || manual_trip
}

/// Copies a reading for the event log.
pub fn snapshot_reading(r: &Reading) -> Reading {
    r.clone()
}
