#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_io::Write;
use panic_probe as _;
use rp_pico::{
    self as bsp,
    hal::{
        fugit::RateExtU32,
        uart::{DataBits, StopBits, UartConfig, UartPeripheral},
    },
};

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let serial_pins = (pins.gpio0.into_function(), pins.gpio1.into_function());
    let uart = UartPeripheral::new(pac.UART0, serial_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(9600u32.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();
    let (_, mut tx) = uart.split();

    let mut count = 1;

    loop {
        writeln!(tx, "Hello World, {} times", count).unwrap();
        count += 1;
        delay.delay_ms(1000);
    }
}

// End of file
