use embassy_time::Timer;
use esp_hal::gpio::AnyPin;
use esp_hal::i2c::master::AnyI2c;
use log::{error, warn};

use crate::DELAY;
use crate::firmware::barometer::Barometer;

#[embassy_executor::task]
pub async fn barometer(
    mut sda: AnyPin<'static>,
    mut scl: AnyPin<'static>,
    mut i2c_bus: AnyI2c<'static>,
) {
    let mut barometer = Barometer::init(sda.reborrow(), scl.reborrow(), i2c_bus.reborrow()).await;

    let mut done = false;
    let mut initial_pressure_hpa = 1013;

    loop {
        if let Some(ref mut barometer) = barometer {
            if let Some(data) = barometer.read().await {
                if !done {
                    initial_pressure_hpa = data.pressure_hpa;
                }
                done = true;

                let altitude_ft = calculate_altitude_ft(
                    initial_pressure_hpa,
                    data.pressure_hpa,
                    data.temperature_c,
                );
                if altitude_ft == 0 {
                    initial_pressure_hpa = data.pressure_hpa;
                }
            } else {
                warn!("Barometer -> Failed to read data");
            }
        } else {
            error!("Barometer -> Failed to initialize");
            barometer = Barometer::init(sda.reborrow(), scl.reborrow(), i2c_bus.reborrow()).await;
        }

        Timer::after_millis(DELAY).await;
    }
}

const FT_PER_HPA: i32 = 27;
const ISA_TEMPERATURE: i32 = 15;

fn calculate_altitude_ft(
    initial_pressure_hpa: i32,
    current_pressure_hpa: i32,
    temperature_c: i32,
) -> i32 {
    // Every 27ft the pressure rises about 1 hPa
    let calculated_altitude = (initial_pressure_hpa - current_pressure_hpa) * FT_PER_HPA;

    // Simple correction due to air temperature
    let true_altitude = calculated_altitude * (temperature_c - ISA_TEMPERATURE) / ISA_TEMPERATURE;

    if true_altitude > 0 {
        true_altitude
    } else {
        calculated_altitude
    }
}
