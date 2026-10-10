pub const WINDOW: usize = 8;

pub struct Filter {
    pub window: [u16; WINDOW],
    pub count: u16,
}

pub fn prefill(f: &mut Filter, first_sample: u16) {
    f.count = WINDOW as u16;
    for slot in f.window.iter_mut() {
        *slot = first_sample;
    }
}

pub fn average(f: &Filter) -> u16 {
    if f.count == 0 {
        return 0;
    }
    let sum: u32 = f.window.iter().map(|&s| u32::from(s)).sum();
    (sum / u32::from(f.count)) as u16
}

/// Index-based variant kept for the clippy walkthrough.
pub fn average_indexed(f: &Filter) -> u16 {
    let mut sum: u32 = 0;
    for i in 0..f.window.len() {
        sum += u32::from(f.window[i]);
    }
    (sum / WINDOW as u32) as u16
}
