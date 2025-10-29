#![deny(unsafe_code)]
#![no_main]
#![no_std]

use defmt_rtt as _;
use embassy_time::Timer;
use panic_probe as _;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    self,
    gpio::OutputType,
    time::khz,
    timer::simple_pwm::{PwmPin, SimplePwm},
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let device_config = embassy_stm32::Config::default();
    let dp = embassy_stm32::init(device_config);
    info!("Embassy initialized.");

    let led_pin = PwmPin::new_ch1(dp.PA0, OutputType::PushPull);
    let pwm = SimplePwm::new(
        dp.TIM2,
        Some(led_pin),
        None,
        None,
        None,
        khz(10),
        Default::default(),
    );
    let mut led = pwm.split().ch1;
    led.enable();

    // let mut can = bxcan::Can::builder(dp.CAN1.can((gpiob.pb9, gpiob.pb8)))
    //     .set_bit_timing(0x001c_0003)
    //     .set_loopback(false)
    //     .set_silent(false)
    //     .leave_disabled();
    // let mut filters = can.modify_filters();
    // filters.enable_bank(0, bxcan::Fifo::Fifo0, bxcan::filter::Mask32::accept_all());
    // filters.enable_bank(0, bxcan::Fifo::Fifo1, bxcan::filter::Mask32::accept_all());
    // drop(filters);

    loop {
        for i in 0..100 {
            led.set_duty_cycle_percent(i);
            Timer::after_millis(10).await;
        }
        for i in 0..100 {
            led.set_duty_cycle_percent(100 - i);
            Timer::after_millis(10).await;
        }
    }
}
