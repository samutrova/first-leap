use cc1101_embassy::{Cc1101, Modulation, RadioConfig, TxPower};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    dma::{DmaRxBuf, DmaTxBuf},
    dma_buffers,
    gpio::{Input, InputConfig, Level, Output, OutputConfig},
    spi::master::{Config, Spi},
    time::Rate,
};
use heapless::{String, Vec};
use log::error;
use shared::{Packet, RadioInitArguments};

pub struct Radio<'a>(
    Cc1101<
        ExclusiveDevice<
            esp_hal::spi::master::SpiDmaBus<'a, esp_hal::Async>,
            Output<'a>,
            embedded_hal_bus::spi::NoDelay,
        >,
        Input<'a>,
        Input<'a>,
    >,
);

impl<'a> Radio<'a> {
    pub async fn init(arguments: RadioInitArguments<'a>) -> Option<Self> {
        let (rx_buffer, rx_descriptor, tx_buffer, tx_descriptor) = dma_buffers!(64);
        let dma_rx_buf = DmaRxBuf::new(rx_descriptor, rx_buffer).unwrap();
        let dma_tx_buf = DmaTxBuf::new(tx_descriptor, tx_buffer).unwrap();

        let spi_bus = Spi::new(
            arguments.spi,
            Config::default()
                .with_frequency(Rate::from_khz(100))
                .with_mode(esp_hal::spi::Mode::_0),
        )
        .unwrap()
        .with_sck(arguments.sclk)
        .with_mosi(arguments.mosi)
        .with_miso(arguments.miso)
        .with_dma(arguments.dma_channel)
        .with_buffers(dma_rx_buf, dma_tx_buf)
        .into_async();

        let cs = Output::new(arguments.cs, Level::High, OutputConfig::default());
        let spi_device = ExclusiveDevice::new_no_delay(spi_bus, cs).unwrap();

        let config = RadioConfig::new()
            .frequency_hz(433_920_000)
            .baud_rate(38_400)
            .modulation(Modulation::Gfsk)
            .tx_power(TxPower::Dbm10)
            .channel(0)
            .append_status(true);

        let gdo0 = Input::new(arguments.gdo0, InputConfig::default());
        let gdo2 = Input::new(arguments.gdo2, InputConfig::default());

        let radio = Cc1101::new(spi_device, gdo0, gdo2).await;

        match radio {
            Ok(mut radio) => match radio.configure(&config).await {
                Ok(_) => Some(Radio(radio)),
                Err(e) => {
                    error!("Radio -> {:#?}", e);
                    None
                }
            },
            Err(e) => {
                error!("Radio -> {:#?}", e);
                None
            }
        }
    }

    pub async fn receive(&mut self) -> Option<Packet> {
        let mut buf = [0u8; 32];
        if let Ok(packet) = self.0.receive(&mut buf).await {
            let vec: Vec<u8, 32> = Vec::from_slice(&buf[..packet.len]).unwrap();
            let data = String::from_utf8(vec).unwrap();
            Some(Packet {
                data,
                rssi: packet.rssi_dbm,
                lqi: packet.lqi,
            })
        } else {
            None
        }
    }
}
