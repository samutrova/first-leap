#![no_std]

use cc1101_embassy::{Modulation, TxPower};
use esp_hal::{dma::AnySpiDmaChannel, gpio::AnyPin, spi::master::AnySpi};
use heapless::String;

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
    pub data: String<32>,
    pub rssi: i16,
    pub lqi: u8,
}
