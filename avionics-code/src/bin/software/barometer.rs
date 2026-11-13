use embassy_time::Timer;
use esp_hal::gpio::AnyPin;
use esp_hal::i2c::master::AnyI2c;
use esp_println::println;
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

    let mut qfe_hpa = -1.0;

    loop {
        if let Some(ref mut barometer) = barometer {
            if let Some(data) = barometer.read().await {
                if qfe_hpa == -1.0 {
                    qfe_hpa = data.pressure_hpa;
                }

                let altitude_m =
                    calculate_barometric_altitude_m(qfe_hpa, data.pressure_hpa, data.temperature_c);
                if altitude_m == 0.0 {
                    qfe_hpa = data.pressure_hpa;
                }

                println!("Altitude    [m]   : {}", altitude_m);
                println!("QFE         [hPa] : {}", qfe_hpa);
                println!("Pressure    [hPa] : {}", data.pressure_hpa);
                println!("Temperature [°C]  : {}", data.temperature_c)
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

fn calculate_barometric_altitude_m(qfe_hpa: f32, pressure_hpa: f32, temperature_c: f32) -> f32 {
    const FT_PER_HPA: f32 = 27.66; // Every 27.66 ft pressure rises about 1 hPa
    const ISA_SEA_LEVEL_TEMPERATURE_K: f32 = 288.15; // 15 + 273.15

    let raw_altitude_ft = (qfe_hpa - pressure_hpa) * FT_PER_HPA;
    let temperature_k = temperature_c + 273.15;

    // Temperature correction
    let corrected_altitude_ft = raw_altitude_ft * (temperature_k / ISA_SEA_LEVEL_TEMPERATURE_K);
    //          ft -> cm              m
    corrected_altitude_ft * 30.48 / 100.0
}
