use embassy_time::Timer;
use esp_hal::gpio::AnyPin;
use esp_hal::i2c::master::AnyI2c;
use heapless::{String, format};
use log::error;
use shared::round::MyRound;

use crate::DELAY;
use crate::firmware::imu::{IMUData, Imu};

use crate::IMU_CHANNEL;

#[embassy_executor::task]
pub async fn imu(mut sda: AnyPin<'static>, mut scl: AnyPin<'static>, mut i2c_bus: AnyI2c<'static>) {
    let mut imu = Imu::init(sda.reborrow(), scl.reborrow(), i2c_bus.reborrow()).await;

    let telemetry_sender = IMU_CHANNEL.sender();

    loop {
        if let Some(ref mut imu) = imu {
            if let Some(data) = imu.get_data().await {
                let raw_data = IMUData {
                    accelerometer_x: data.accelerometer_x,
                    accelerometer_y: data.accelerometer_y,
                    accelerometer_z: data.accelerometer_z,

                    gyroscope_x: data.gyroscope_x,
                    gyroscope_y: data.gyroscope_y,
                    gyroscope_z: data.gyroscope_z,
                };
                telemetry_sender.send(raw_data.into_units()).await;
            }
        } else {
            imu = Imu::init(sda.reborrow(), scl.reborrow(), i2c_bus.reborrow()).await;
        }

        Timer::after_millis(DELAY).await;
    }
}

pub struct InertialData {
    pub acceleration_x_ms2: f32,
    pub acceleration_y_ms2: f32,
    pub acceleration_z_ms2: f32,

    pub tilt_x_deg: f32,
    pub tilt_y_deg: f32,
    pub tilt_z_deg: f32,

    pub g_force_x: f32,
    pub g_force_y: f32,
    pub g_force_z: f32,
}

impl InertialData {
    pub fn round(&mut self) {
        self.acceleration_x_ms2 = MyRound::round(self.acceleration_x_ms2);
        self.acceleration_y_ms2 = MyRound::round(self.acceleration_y_ms2);
        self.acceleration_z_ms2 = MyRound::round(self.acceleration_z_ms2);

        self.tilt_x_deg = MyRound::round(self.tilt_x_deg);
        self.tilt_y_deg = MyRound::round(self.tilt_y_deg);
        self.tilt_z_deg = MyRound::round(self.tilt_z_deg);

        self.g_force_x = MyRound::round(self.g_force_x);
        self.g_force_y = MyRound::round(self.g_force_y);
        self.g_force_z = MyRound::round(self.g_force_z);
    }

    pub fn as_string(&mut self) -> String<128> {
        self.round();
        match format!(
            128;
            "{};{};{};{};{};{};{};{};{}",

            self.acceleration_x_ms2,
            self.acceleration_y_ms2,
            self.acceleration_z_ms2,

            self.tilt_x_deg,
            self.tilt_y_deg,
            self.tilt_z_deg,

            self.g_force_x,
            self.g_force_y,
            self.g_force_z,
        ) {
            Ok(string) => string,
            Err(e) => {
                error!("IMU -> Failed to format string: {}", e);
                String::new()
            }
        }
    }
}

impl IMUData {
    pub fn into_units(self) -> InertialData {
        InertialData {
            acceleration_x_ms2: Self::to_acceleration_ms2(self.accelerometer_x),
            acceleration_y_ms2: Self::to_acceleration_ms2(self.accelerometer_y),
            acceleration_z_ms2: Self::to_acceleration_ms2(self.accelerometer_z),

            tilt_x_deg: Self::to_tilt_deg(self.gyroscope_x),
            tilt_y_deg: Self::to_tilt_deg(self.gyroscope_y),
            tilt_z_deg: Self::to_tilt_deg(self.gyroscope_z),

            g_force_x: Self::to_g_force(self.accelerometer_x),
            g_force_y: Self::to_g_force(self.accelerometer_y),
            g_force_z: Self::to_g_force(self.accelerometer_z),
        }
    }

    const ACCELEROMETER_SENSITIVITY: f32 = 4_096.0;

    fn to_acceleration_ms2(value: f32) -> f32 {
        const GRAVITY: f32 = 9.80665;

        value / Self::ACCELEROMETER_SENSITIVITY * GRAVITY
    }

    fn to_g_force(value: f32) -> f32 {
        value / Self::ACCELEROMETER_SENSITIVITY
    }

    fn to_tilt_deg(value: f32) -> f32 {
        const GYROSCOPE_SENSITIVITY: f32 = 262.4;

        value / GYROSCOPE_SENSITIVITY
    }
}
