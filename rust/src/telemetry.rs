use crate::protect::Reading;

/// Returns None for an invalid reading, so nothing is allocated and nothing can leak.
pub fn format_telemetry(r: &Reading, tripped: bool) -> Option<String> {
    if !r.valid {
        return None;
    }
    Some(format!("{{\"kpa\":{},\"tripped\":{}}}", r.value_kpa, tripped))
}
