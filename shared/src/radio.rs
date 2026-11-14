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
    pub data: String<128>,
    pub rssi_dbm: i16,
    pub lqi: u8,
}

pub struct TelemetryData {
    pub time: u64,
    pub acceleration_x_ms2: f32,
    pub acceleration_y_ms2: f32,
    pub acceleration_z_ms2: f32,
    pub tilt_x_deg: f32,
    pub tilt_y_deg: f32,
    pub tilt_z_deg: f32,
}

impl TelemetryData {
    pub fn to_string(&self) -> String<128> {
        format!(128; "{};{};{};{};{};{};{}", self.time, self.acceleration_x_ms2, self.acceleration_y_ms2, self.acceleration_z_ms2, self.tilt_x_deg, self.tilt_y_deg, self.tilt_z_deg).unwrap()
    }

    pub fn into_units(&self) -> Self {
        TelemetryData {
            time: self.time,
            acceleration_x_ms2: Self::to_acceleration_ms2(self.acceleration_x_ms2),
            acceleration_y_ms2: Self::to_acceleration_ms2(self.acceleration_y_ms2),
            acceleration_z_ms2: Self::to_acceleration_ms2(self.acceleration_z_ms2),
            tilt_x_deg: Self::to_tilt_deg(self.tilt_x_deg),
            tilt_y_deg: Self::to_tilt_deg(self.tilt_y_deg),
            tilt_z_deg: Self::to_tilt_deg(self.tilt_z_deg),
        }
    }

    fn to_acceleration_ms2(accelerometer_value: f32) -> f32 {
        const ACCELEROMETER_SENSITIVITY: f32 = 4_096.0;
        const GRAVITY: f32 = 9.80665;

        accelerometer_value / ACCELEROMETER_SENSITIVITY * GRAVITY
    }

    fn to_tilt_deg(gyroscope_value: f32) -> f32 {
        const GYROSCOPE_SENSITIVITY: f32 = 262.4;

        gyroscope_value / GYROSCOPE_SENSITIVITY
    }
}
