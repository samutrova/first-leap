pub struct MyRound;
impl MyRound {
    pub fn round(value: f32) -> f32 {
        const ROUND_POWER: f32 = 1_000.0; // 10^3 -> 3 decimal places
        libm::roundf(value * ROUND_POWER) / ROUND_POWER
    }
}
