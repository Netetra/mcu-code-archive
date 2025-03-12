#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::Write;

use defmt::info;
use embedded_graphics::{
    image::{self, Image, ImageDrawable, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::Point,
    Drawable,
};
// Print panic message to probe console
use panic_probe as _;

use cortex_m_rt::entry;
use defmt_rtt as _;
use ssd1306::{
    mode::DisplayConfig, prelude::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface,
    Ssd1306,
};
use stm32f3xx_hal::{delay::Delay, i2c::I2c, pac, prelude::*};

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(8u32.MHz()).freeze(&mut flash.acr);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let scl =
        gpiob
            .pb6
            .into_af_open_drain::<4>(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let sda =
        gpiob
            .pb7
            .into_af_open_drain::<4>(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        100.kHz().try_into().unwrap(),
        clocks,
        &mut rcc.apb1,
    );

    let mut delay = Delay::new(cp.SYST, clocks);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();
    display.init().unwrap();
    display.clear().unwrap();

    let mut count = 0;

    loop {
        display.clear().unwrap();
        write!(display, "Hello World!\nCount: {}\n", count).unwrap();
        count += 1;
        delay.delay_ms(1000u32);
    }
}
