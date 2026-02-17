use crate::db::Entry;

const SECONDS_IN_HOUR: u64 = 3600;
const SECONDS_IN_DAY: u64 = 86400;
const SECONDS_IN_WEEK: u64 = 604800;

pub fn calculate_score(entry: &Entry, now: u64) -> f64 {
    let time_diff = now.saturating_sub(entry.last_accessed);

    let time_multiplier = if time_diff < SECONDS_IN_HOUR {
        4.0
    } else if time_diff < SECONDS_IN_DAY {
        2.0
    } else if time_diff < SECONDS_IN_WEEK {
        0.5
    } else {
        0.25
    };

    entry.rank as f64 * time_multiplier
}

pub fn update_entry(entry: &mut Entry, now: u64) {
    entry.rank += 1;
    entry.last_accessed = now;
}
