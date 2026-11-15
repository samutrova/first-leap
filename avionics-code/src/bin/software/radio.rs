use crate::BAROMETER_CHANNEL;
use crate::DELAY;
use crate::firmware::radio::Radio;
use embassy_time::Timer;
use log::{error, warn};
use shared::radio::RadioInitArguments;

use crate::IMU_CHANNEL;

#[embassy_executor::task]
pub async fn radio(mut arguments: RadioInitArguments<'static>) {
    let mut radio = Radio::init(arguments.reborrow()).await;

    let imu_receiver = IMU_CHANNEL.receiver();
    let barometer_receiver = BAROMETER_CHANNEL.receiver();

    loop {
        if let Some(ref mut radio) = radio {
            let inertial_data = imu_receiver.receive().await;
            let barometric_data = barometer_receiver.receive().await;

            if !radio.transmit(inertial_data.to_string().as_bytes()).await {
                warn!("Radio -> Failed to transmit inertial packet");
            }

            if !radio.transmit(barometric_data.to_string().as_bytes()).await {
                warn!("Radio -> Failed to transmit barometric packet");
            }
        } else {
            error!("Radio -> Failed to initialize");

            drop(radio);
            radio = Radio::init(arguments.reborrow()).await;
        }

        Timer::after_millis(DELAY).await;
    }
}
