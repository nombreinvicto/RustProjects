// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate;
    if speed >= 1 && speed <= 4 {
        success_rate = 1.0;
    } else if speed > 4 && speed <= 8 {
        success_rate = 0.9
    } else {
        success_rate = 0.77;
    }

    // rust doesnt allow multiplication of u8
    // and float since you have to explicitly
    // declare whether you care about precision or not
    f64::from(speed) * 221.0 * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let wipm: u32 = (production_rate_per_hour(speed) as u32) / (60);
    return wipm;
}
