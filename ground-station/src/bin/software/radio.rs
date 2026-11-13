use crate::DELAY;
use crate::firmware::radio::Radio;
use embassy_time::Timer;
use log::error;
use log::info;
use shared::radio::RadioInitArguments;

#[embassy_executor::task]
pub async fn radio(mut arguments: RadioInitArguments<'static>) {
    let mut radio = Radio::init(arguments.reborrow()).await;

    loop {
        if let Some(ref mut radio) = radio {
            if let Some(packet) = radio.receive().await {
                info!(
                    "Data: {} , rssi: {} , lqi: {}",
                    packet.data, packet.rssi_dbm, packet.lqi
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
