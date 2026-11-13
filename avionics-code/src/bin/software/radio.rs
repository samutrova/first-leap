use crate::DELAY;
use crate::firmware::radio::Radio;
use embassy_time::Timer;
use log::error;
use log::warn;
use shared::radio::RadioInitArguments;

use crate::SHARED_CHANNEL;

#[embassy_executor::task]
pub async fn radio(mut arguments: RadioInitArguments<'static>) {
    let mut radio = Radio::init(arguments.reborrow()).await;

    let channel_receiver = SHARED_CHANNEL.receiver();

    loop {
        if let Some(ref mut radio) = radio {
            let packet = channel_receiver.receive().await;
            if !radio.transmit(&packet.to_string()).await {
                warn!("Radio -> Failed to transmit packet");
            }
        } else {
            error!("Radio -> Failed to initialize");

            drop(radio);
            radio = Radio::init(arguments.reborrow()).await;
        }

        Timer::after_millis(DELAY).await;
    }
}
