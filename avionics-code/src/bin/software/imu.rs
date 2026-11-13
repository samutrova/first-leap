use embassy_time::Timer;
use esp_hal::gpio::AnyPin;
use esp_hal::i2c::master::AnyI2c;
use esp_hal::time::Instant;
use log::{error, warn};
use shared::radio::TelemetryData;

use crate::DELAY;
use crate::firmware::imu::Imu;

use crate::SHARED_CHANNEL;

#[embassy_executor::task]
pub async fn imu(mut sda: AnyPin<'static>, mut scl: AnyPin<'static>, mut i2c_bus: AnyI2c<'static>) {
    let mut imu = Imu::init(sda.reborrow(), scl.reborrow(), i2c_bus.reborrow()).await;

    let sender = SHARED_CHANNEL.sender();
    let start_time = Instant::now();

    loop {
        if let Some(ref mut imu) = imu {
            if let Some(data) = imu.get_data().await {
                let data = TelemetryData {
                    time: start_time.elapsed().as_secs(),
                    accelerometer_x: data.accelerometer_x,
                    accelerometer_y: data.accelerometer_y,
                    accelerometer_z: data.accelerometer_z,
                    gyroscope_x: data.gyroscope_x,
                    gyroscope_y: data.gyroscope_y,
                    gyroscope_z: data.gyroscope_z,
                }; // 16 530
                /*println!(
                    "{} , {} , {} , {} , {} , {} , {}",
                    data.time,
                    data.accelerometer_x,
                    data.accelerometer_y,
                    data.accelerometer_z,
                    data.gyroscope_x,
                    data.gyroscope_y,
                    data.gyroscope_z
                );*/
                sender.send(data).await;
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
