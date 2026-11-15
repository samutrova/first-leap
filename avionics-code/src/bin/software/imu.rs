use embassy_time::Timer;
use esp_hal::gpio::AnyPin;
use esp_hal::i2c::master::AnyI2c;
use esp_hal::time::Instant;
use log::{error, warn};
use shared::radio::TelemetryData;

use crate::DELAY;
use crate::firmware::imu::Imu;

use crate::TELEMETRY_CHANNEL;

#[embassy_executor::task]
pub async fn imu(mut sda: AnyPin<'static>, mut scl: AnyPin<'static>, mut i2c_bus: AnyI2c<'static>) {
    let mut imu = Imu::init(sda.reborrow(), scl.reborrow(), i2c_bus.reborrow()).await;

    let telemetry_sender = TELEMETRY_CHANNEL.sender();
    let start_time = Instant::now();

    loop {
        if let Some(ref mut imu) = imu {
            if let Some(data) = imu.get_data().await {
                let raw_data = TelemetryData {
                    time: start_time.elapsed().as_secs(),
                    acceleration_x_ms2: data.accelerometer_x,
                    acceleration_y_ms2: data.accelerometer_y,
                    acceleration_z_ms2: data.accelerometer_z,
                    tilt_x_deg: data.gyroscope_x,
                    tilt_y_deg: data.gyroscope_y,
                    tilt_z_deg: data.gyroscope_z,
                };
                telemetry_sender.send(raw_data.into_units()).await;
            } else {
                warn!("IMU -> Failed to read data");
            }
        } else {
            error!("IMU -> Failed to initialize");
            imu = Imu::init(sda.reborrow(), scl.reborrow(), i2c_bus.reborrow()).await;
        }

        Timer::after_millis(DELAY).await;
    }
}
