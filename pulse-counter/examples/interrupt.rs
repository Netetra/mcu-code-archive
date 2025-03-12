#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use stm32f3xx_hal::{
    interrupt,
    interrupts::InterruptNumber,
    pac::{self, NVIC, TIM2},
    prelude::*,
};

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // クロック供給有効化
    dp.RCC.apb1enr.write(|w| w.tim2en().enabled());

    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let _clocks = rcc
        .cfgr
        .sysclk(8.MHz())
        .pclk1(1.MHz())
        .freeze(&mut flash.acr);

    // TIM2設定
    // (1MHz(APB2) * 2) / 40000 = 50Hz
    dp.TIM2.psc.write(|w| w.psc().bits(40000 - 1));
    // 50Hz / 50 = 1Hzでイベント発生
    dp.TIM2.arr.write(|w| w.arr().bits(50 + 1));
    dp.TIM2.ccr1().write(|w| w.ccr().bits(50));
    // 割り込み有効化
    dp.TIM2.dier.write(|w| w.cc1ie().enabled());
    unsafe {
        NVIC::unmask(TIM2::INTERRUPT);
    }
    // 設定を反映
    dp.TIM2.egr.write(|w| w.ug().update());

    // TIM2有効化
    dp.TIM2.cr1.write(|w| w.cen().enabled());

    loop {}
}

#[interrupt]
fn TIM2() {
    static mut COUNT: u32 = 0;
    let dp = unsafe { pac::Peripherals::steal() };
    info!("Count: {}", COUNT);
    *COUNT += 1;
    // 割り込みフラグを折る
    dp.TIM2.sr.write(|w| w.cc1if().clear_bit());
}
