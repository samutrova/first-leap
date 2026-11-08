use crate::DELAY;
use crate::firmware::radio::Radio;
use embassy_time::Timer;
use log::error;
use log::info;
use shared::RadioInitArguments;

#[embassy_executor::task]
pub async fn radio(mut arguments: RadioInitArguments<'static>) {
    let mut radio = Radio::init(arguments.reborrow()).await;

    loop {
        if let Some(ref mut radio) = radio {
            if let Some(data) = radio.receive().await {
                info!(
                    "Data: {} , rssi: {} , lqi: {}",
                    data.data, data.rssi, data.lqi
                );
            }
        } else {
            error!("Radio -> Failed to initialize");

            drop(radio);
            radio = Radio::init(arguments.reborrow()).await;
        }

        Timer::after_millis(DELAY).await;
    }
}
