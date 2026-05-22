pub const WIDTH: f64 = 300.0;
pub const HEIGHT: f64 = 150.0;
pub const SHIFT: f64 = 15.0;
pub const EDGE: f64 = SHIFT / 2.0;
pub const FULL: f64 = WIDTH - SHIFT * 2.0;
pub fn length(multiplier: f64) -> f64 { HEIGHT - SHIFT * multiplier }
