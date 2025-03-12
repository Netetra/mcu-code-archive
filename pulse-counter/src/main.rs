#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use stm32f3xx_hal::{delay::Delay, pac, prelude::*};

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // クロック供給有効化
    dp.RCC.apb1enr.write(|w| w.tim2en().enabled());

    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc
        .cfgr
        .sysclk(8.MHz())
        .pclk1(1.MHz())
        .freeze(&mut flash.acr);

    // TIM2設定
    // エンコーダモードに設定
    dp.TIM2.smcr.write(|w| w.sms().encoder_mode_1());
    // 設定を反映
    dp.TIM2.egr.write(|w| w.ug().update());

    // TIM2有効化
    dp.TIM2.cr1.write(|w| w.cen().enabled());

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let _encoder_a =
        gpioa
            .pa0
            .into_af_push_pull::<1>(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let _encoder_b =
        gpioa
            .pa1
            .into_af_push_pull::<1>(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        let count = dp.TIM2.cnt.read().cnt().bits();
        dp.TIM2.cnt.reset();
        let rpm = count / 48 * 60;
        info!("RPM: {}", rpm);
        delay.delay_ms(1000u32);
    }
}
