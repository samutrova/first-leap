use embassy_time::Timer;
use esp_hal::gpio::AnyPin;
use esp_hal::i2c::master::AnyI2c;
use esp_hal::time::Instant;
use log::{error, warn};

use crate::firmware::imu::Imu;
use crate::{DELAY, MainSharedChannelData};

use crate::SHARED_CHANNEL;

#[embassy_executor::task]
pub async fn imu(mut sda: AnyPin<'static>, mut scl: AnyPin<'static>, mut i2c_bus: AnyI2c<'static>) {
    let mut imu = Imu::init(sda.reborrow(), scl.reborrow(), i2c_bus.reborrow()).await;

    let sender = SHARED_CHANNEL.sender();
    let start_time = Instant::now();

    loop {
        if let Some(ref mut imu) = imu {
            if let Some(data) = imu.get_accelerometer_data().await {
                let time = start_time.elapsed().as_secs();
                let data = MainSharedChannelData::new(time, data.y);
                sender.send(data).await;
            } else {
                warn!("IMU -> Failed to read accelerometer data");
            }
        } else {
            error!("IMU -> Failed to initialize");
            imu = Imu::init(sda.reborrow(), scl.reborrow(), i2c_bus.reborrow()).await;
        }

        Timer::after_millis(DELAY).await;
    }
}
