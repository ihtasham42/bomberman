use crate::constants::FIXED_UPDATE_FREQUENCY;

pub fn seconds_to_freq(seconds: i32) -> i32 {
    seconds * FIXED_UPDATE_FREQUENCY as i32
}
