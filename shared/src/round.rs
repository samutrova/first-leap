pub struct MyRound;
impl MyRound {
    pub fn roundf(value: f32, decimals: u8) -> f32 {
        // decimals = 3 -> 10^3 round_power
        let round_power = libm::powf(10.0, decimals as f32);
        libm::roundf(value * round_power) / round_power
    }
}
