use crate::BAROMETER_CHANNEL;
use crate::firmware::radio::Radio;
use embassy_time::Timer;
use shared::DELAY;
use shared::radio::RadioInitArguments;

use crate::IMU_CHANNEL;

#[embassy_executor::task]
pub async fn radio(mut arguments: RadioInitArguments<'static>) {
    let mut radio = Radio::init(arguments.reborrow()).await;

    let imu_receiver = IMU_CHANNEL.receiver();
    let barometer_receiver = BAROMETER_CHANNEL.receiver();

    loop {
        if let Some(ref mut radio) = radio {
            let mut inertial_data = imu_receiver.receive().await;
            let mut barometric_data = barometer_receiver.receive().await;

            radio.transmit(inertial_data.as_string().as_bytes()).await;
            radio.transmit(barometric_data.as_string().as_bytes()).await;
        } else {
            drop(radio);
            radio = Radio::init(arguments.reborrow()).await;
        }

        Timer::after_millis(DELAY).await;
    }
}
