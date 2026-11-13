use embassy_gy_bmi160::Bmi160;
use esp_hal::{
    gpio::AnyPin,
    i2c::master::{AnyI2c, Config, I2c},
    time::Rate,
};
use log::error;

pub struct Imu<'a>(Bmi160<'a, I2c<'a, esp_hal::Async>>);

pub struct IMUData {
    pub accelerometer_x: i16,
    pub accelerometer_y: i16,
    pub accelerometer_z: i16,
    pub gyroscope_x: i16,
    pub gyroscope_y: i16,
    pub gyroscope_z: i16,
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

    pub async fn get_data(&mut self) -> Option<IMUData> {
        match self.0.read_accel().await {
            Ok(accelerometer) => match self.0.read_gyro().await {
                Ok(gyroscope) => Some(IMUData {
                    accelerometer_x: accelerometer.x,
                    accelerometer_y: accelerometer.y,
                    accelerometer_z: accelerometer.z,
                    gyroscope_x: gyroscope.x,
                    gyroscope_y: gyroscope.y,
                    gyroscope_z: gyroscope.z,
                }),
                Err(_) => Some(IMUData {
                    accelerometer_x: accelerometer.x,
                    accelerometer_y: accelerometer.y,
                    accelerometer_z: accelerometer.z,
                    gyroscope_x: 0,
                    gyroscope_y: 0,
                    gyroscope_z: 0,
                }),
            },
            Err(_) => match self.0.read_gyro().await {
                Ok(gyroscope) => Some(IMUData {
                    accelerometer_x: 0,
                    accelerometer_y: 0,
                    accelerometer_z: 0,
                    gyroscope_x: gyroscope.x,
                    gyroscope_y: gyroscope.y,
                    gyroscope_z: gyroscope.z,
                }),
                Err(_) => None,
            },
        }
    }
}
