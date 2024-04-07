#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed, Input, Pull};
use embassy_time::Ticker;
use embassy_time::Duration;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn led_blink() {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PA5, Level::High, Speed::Low);
    let mut ticker = Ticker::every(Duration::from_millis(100));

    loop {
        led.toggle();
        ticker.next().await;
    }
}
#[embassy_executor::task]
async fn button(){
    let p = embassy_stm32::init(Default::default());

    let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Up);

    info!("Press the USER button...");

    loop {
        button.wait_for_falling_edge().await;
        info!("Pressed!");
        button.wait_for_rising_edge().await;
        info!("Released!");
    }
}


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    spawner.spawn(led_blink()).unwrap();
    spawner.spawn(button()).unwrap();
}
