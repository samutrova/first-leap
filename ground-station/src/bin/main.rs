#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
// #![warn(clippy::large_stack_frames)]

use crate::software::radio;
use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use shared::radio::RadioInitArguments;
extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

mod firmware;
mod software;

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
    // -- Radio --
    spawner.spawn(
        radio::radio(RadioInitArguments {
            sclk: peripherals.GPIO32.into(),
            mosi: peripherals.GPIO25.into(),
            miso: peripherals.GPIO27.into(),
            cs: peripherals.GPIO14.into(),
            spi: peripherals.SPI2.into(),
            gdo0: peripherals.GPIO35.into(),
            gdo2: peripherals.GPIO33.into(),
            dma_channel: peripherals.DMA_SPI2.into(),
        })
        .unwrap(),
    );

    loop {
        Timer::after_millis(50).await;
    }
}
