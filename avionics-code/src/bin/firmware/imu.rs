use embassy_gy_bmi160::Bmi160;
use esp_hal::{
    gpio::AnyPin,
    i2c::master::{AnyI2c, Config, I2c},
    time::Rate,
};
use log::error;

pub struct Imu<'a>(Bmi160<'a, I2c<'a, esp_hal::Async>>);

pub struct AccelerometerData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl<'a> Imu<'a> {
    pub async fn init(sda: AnyPin<'a>, scl: AnyPin<'a>, i2c_bus: AnyI2c<'a>) -> Option<Self> {
        let i2c = I2c::new(
            i2c_bus,
            Config::default().with_frequency(Rate::from_khz(100)),
        )
        .unwrap()
        .with_sda(sda)
        .with_scl(scl)
        .into_async();

        let mut imu = Bmi160::new(i2c, 0x68);

        match imu.init().await {
            Ok(_) => Some(Imu(imu)),
            Err(e) => {
                error!("IMU -> {}", e);
                None
            }
        }
    }

    pub async fn get_accelerometer_data(&mut self) -> Option<AccelerometerData> {
        if let Ok(data) = self.0.read_accel().await {
            Some(AccelerometerData {
                x: data.x,
                y: data.y,
                z: data.z,
            })
        } else {
            None
        }
    }
}
