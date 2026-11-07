use embassy_bmp280::{Bmp280, Bmp280Address, Bmp280Config};
use embassy_time::Timer;
use esp_hal::{
    gpio::AnyPin,
    i2c::master::{AnyI2c, Config, I2c},
    time::Rate,
};
use log::error;

pub struct Barometer<'a>(Bmp280<I2c<'a, esp_hal::Async>>);

pub struct BarometerData {
    pub pressure_hpa: i32,
    pub temperature_c: i32,
}

impl<'a> Barometer<'a> {
    pub async fn init(sda: AnyPin<'a>, scl: AnyPin<'a>, i2c_bus: AnyI2c<'a>) -> Option<Self> {
        let i2c = I2c::new(
            i2c_bus,
            Config::default().with_frequency(Rate::from_khz(100)),
        )
        .unwrap()
        .with_sda(sda)
        .with_scl(scl)
        .into_async();

        let barometer = Bmp280::new(i2c, Bmp280Address::Secondary, Bmp280Config::default()).await;

        Timer::after_millis(1000).await;

        match barometer {
            Ok(barometer) => Some(Barometer(barometer)),
            Err(e) => {
                error!("Barometer -> {:#?}", e);
                None
            }
        }
    }

    pub async fn read(&mut self) -> Option<BarometerData> {
        if let Ok(data) = self.0.read().await {
            let pressure_hpa = (data.pressure_pa256 / (256 * 100)) as i32;
            let temperature_c = data.temperature_cdeg / 100;
            Some(BarometerData {
                pressure_hpa,
                temperature_c,
            })
        } else {
            None
        }
    }
}
