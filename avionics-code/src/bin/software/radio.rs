use crate::BAROMETRIC_CHANNEL;
use crate::DELAY;
use crate::firmware::radio::Radio;
use embassy_time::Timer;
use esp_println::println;
use log::{error, warn};
use shared::radio::RadioInitArguments;

use crate::TELEMETRY_CHANNEL;

#[embassy_executor::task]
pub async fn radio(mut arguments: RadioInitArguments<'static>) {
    let mut radio = Radio::init(arguments.reborrow()).await;

    let telemetry_receiver = TELEMETRY_CHANNEL.receiver();
    let barometric_receiver = BAROMETRIC_CHANNEL.receiver();

    loop {
        if let Some(ref mut radio) = radio {
            let telemetry_data = telemetry_receiver.receive().await;
            let barometric_data = barometric_receiver.receive().await;

            if let Some(data) = telemetry_data.as_str() {
                println!("Telemetry: {}", data);
                if !radio.transmit(data.as_bytes()).await {
                    warn!("Radio -> Failed to transmit packet");
                }
            } else {
                warn!("Radio -> Telemetry data not found");
            }

            if let Some(data) = barometric_data.as_str() {
                println!("Barometric Data: {}", data);
                if !radio.transmit(data.as_bytes()).await {
                    warn!("Radio -> Failed to transmit packet");
                }
            } else {
                warn!("Radio -> Barometric data not found");
            }
        } else {
            error!("Radio -> Failed to initialize");

            drop(radio);
            radio = Radio::init(arguments.reborrow()).await;
        }

        Timer::after_millis(DELAY).await;
    }
}
