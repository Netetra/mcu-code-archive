#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use stm32f3xx_hal::{delay::Delay, pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // クロック供給有効化
    dp.RCC.apb1enr.write(|w| w.tim2en().enabled());

    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(8.MHz()).freeze(&mut flash.acr);

    // TIM2設定
    // 8MHz(SYST) / 40000 = 200Hz
    dp.TIM2.psc.write(|w| w.psc().bits(40000 - 1));
    // 設定を反映
    dp.TIM2.egr.write(|w| w.ug().update());

    // TIM2有効化
    dp.TIM2.cr1.write(|w| w.cen().enabled());

    let mut delay = Delay::new(cp.SYST, clocks);
    loop {
        let count = dp.TIM2.cnt.read().cnt().bits();
        info!("Count: {}", count / 200);
        delay.delay_ms(100u32);
    }
}
