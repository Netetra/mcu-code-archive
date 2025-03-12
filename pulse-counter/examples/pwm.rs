#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;
use stm32f3xx_hal::{delay::Delay, pac, prelude::*};

// NOTE: PB3 AF1 = TIM2_CH2

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
    // (1MHz(APB2) * 2) / 40 = 50kHz
    dp.TIM2.psc.write(|w| w.psc().bits(40 - 1));
    // 1000段階 (50Hz)
    dp.TIM2.arr.write(|w| w.arr().bits(1000 + 1));
    // PWM1モードに設定
    dp.TIM2
        .ccmr1_output()
        .write(|w| w.oc2m().pwm_mode1().oc2pe().enabled());
    dp.TIM2.cr1.write(|w| w.arpe().enabled());
    // 設定を反映
    dp.TIM2.egr.write(|w| w.ug().update());
    // TIM2_CH2の出力有効化
    dp.TIM2.ccer.write(|w| w.cc2e().set_bit());
    // TIM2有効化
    dp.TIM2.cr1.write(|w| w.cen().enabled());

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let _led =
        gpiob
            .pb3
            .into_af_push_pull::<1>(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        for i in 0..1000 {
            dp.TIM2.ccr2().write(|w| w.ccr().bits(i));
            delay.delay_ms(1u32);
        }
        for i in 0..1000 {
            dp.TIM2.ccr2().write(|w| w.ccr().bits(1000 - i));
            delay.delay_ms(1u32);
        }
    }
}
