#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
// #![warn(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Timer;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;

esp_bootloader_esp_idf::esp_app_desc!();

mod firmware;
mod software;

use software::barometer;
use software::imu;
use software::radio;

use crate::firmware::radio::RadioInitArguments;

struct MainSharedChannelData {
    time: u64,
    accelerometer_y: i16,
}

impl MainSharedChannelData {
    fn new(time: u64, accelerometer_y: i16) -> Self {
        Self {
            time,
            accelerometer_y,
        }
    }
}

// TODO: not used for now
struct _SecondarySharedChannelData {
    accelerometer_x: i16,
    accelerometer_z: i16,
    gyroscope_x: i16,
    gyroscope_y: i16,
    gyroscope_z: i16,
}

static SHARED_CHANNEL: Channel<CriticalSectionRawMutex, MainSharedChannelData, 2> = Channel::new();

static DELAY: u64 = 50;

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    // ==== PROGRAM ====
    // -- LED --
    // let mut led = Output::new(peripherals.GPIO14, Level::Low, OutputConfig::default());

    // -- IMU --
    spawner.spawn(
        imu::imu(
            peripherals.GPIO26.into(),
            peripherals.GPIO27.into(),
            peripherals.I2C0.into(),
        )
        .unwrap(),
    );

    // -- Barometer --
    spawner.spawn(
        barometer::barometer(
            peripherals.GPIO33.into(),
            peripherals.GPIO32.into(),
            peripherals.I2C1.into(),
        )
        .unwrap(),
    );

    // -- Radio --
    spawner.spawn(
        radio::radio(RadioInitArguments {
            sclk: peripherals.GPIO0.into(),
            mosi: peripherals.GPIO16.into(),
            miso: peripherals.GPIO17.into(),
            cs: peripherals.GPIO21.into(),
            spi: peripherals.SPI2.into(),
            gdo0: peripherals.GPIO18.into(),
            gdo2: peripherals.GPIO19.into(),
            dma_channel: peripherals.DMA_SPI2.into(),
        })
        .unwrap(),
    );

    loop {
        // led.set_high();
        Timer::after_millis(DELAY).await;
    }
}
