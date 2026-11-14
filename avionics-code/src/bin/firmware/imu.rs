use bmi160::{
    AccelerometerPowerMode, AccelerometerRange, Bmi160, GyroscopePowerMode, GyroscopeRange,
    SensorSelector, SlaveAddr,
};
use esp_hal::{
    gpio::AnyPin,
    i2c::master::{AnyI2c, Config, I2c},
    time::Rate,
};

pub struct Imu<'a>(Bmi160<bmi160::interface::I2cInterface<I2c<'a, esp_hal::Async>>>);

pub struct IMUData {
    pub accelerometer_x: f32,
    pub accelerometer_y: f32,
    pub accelerometer_z: f32,
    pub gyroscope_x: f32,
    pub gyroscope_y: f32,
    pub gyroscope_z: f32,
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

        let mut imu = Bmi160::new_with_i2c(i2c, SlaveAddr::default());

        if imu
            .set_accel_power_mode(AccelerometerPowerMode::Normal)
            .is_ok()
            && imu.set_accel_range(AccelerometerRange::G8).is_ok()
            && imu.set_gyro_power_mode(GyroscopePowerMode::Normal).is_ok()
            && imu.set_gyro_range(GyroscopeRange::Scale125).is_ok()
        {
            Some(Imu(imu))
        } else {
            None
        }
    }

    pub async fn get_data(&mut self) -> Option<IMUData> {
        if let Ok(data) = self.0.data_scaled(SensorSelector::new().accel().gyro()) {
            // We can safely call .unwrap() because we selected the sensors (theoretically)
            let accelerometer = data.accel.unwrap();
            let gyroscope = data.gyro.unwrap();
            Some(IMUData {
                accelerometer_x: accelerometer.x,
                accelerometer_y: accelerometer.y,
                accelerometer_z: accelerometer.z,
                gyroscope_x: gyroscope.x,
                gyroscope_y: gyroscope.y,
                gyroscope_z: gyroscope.z,
            })
        } else {
            None
        }
    }
}
