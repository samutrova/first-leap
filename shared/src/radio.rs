use cc1101_embassy::{Modulation, TxPower};
use esp_hal::{dma::AnySpiDmaChannel, gpio::AnyPin, spi::master::AnySpi};
use heapless::{String, format};

pub const FREQUENCY_HZ: u32 = 433_920_000;
pub const BAUD_RATE: u32 = 38_400;
pub const MODULATION: Modulation = Modulation::Gfsk;
pub const TX_POWER: TxPower = TxPower::Dbm10;
pub const CHANNEL: u8 = 0;
pub const APPEND_STATUS: bool = true;

pub struct RadioInitArguments<'a> {
    pub sclk: AnyPin<'a>,
    pub mosi: AnyPin<'a>,
    pub miso: AnyPin<'a>,
    pub cs: AnyPin<'a>,
    pub spi: AnySpi<'a>,
    pub gdo0: AnyPin<'a>,
    pub gdo2: AnyPin<'a>,
    pub dma_channel: AnySpiDmaChannel<'a>,
}

impl<'a> RadioInitArguments<'a> {
    pub fn reborrow(&mut self) -> RadioInitArguments<'_> {
        RadioInitArguments {
            sclk: self.sclk.reborrow(),
            mosi: self.mosi.reborrow(),
            miso: self.miso.reborrow(),
            cs: self.cs.reborrow(),
            spi: self.spi.reborrow(),
            gdo0: self.gdo0.reborrow(),
            gdo2: self.gdo2.reborrow(),
            dma_channel: self.dma_channel.reborrow(),
        }
    }
}

pub struct Packet {
    pub data: String<64>,
    pub rssi_dbm: i16,
    pub lqi: u8,
}

pub struct TelemetryData {
    pub time: u64,
    pub accelerometer_x: i16,
    pub accelerometer_y: i16,
    pub accelerometer_z: i16,
    pub gyroscope_x: i16,
    pub gyroscope_y: i16,
    pub gyroscope_z: i16,
}

impl TelemetryData {
    pub fn to_string(&self) -> heapless::String<64> {
        format!(64; "{};{};{};{};{};{};{}", self.time, self.accelerometer_x, self.accelerometer_y, self.accelerometer_z, self.gyroscope_x, self.gyroscope_y, self.gyroscope_y).unwrap()
    }
}
