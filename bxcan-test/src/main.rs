#![no_main]
#![no_std]

use core::{cell::RefCell, ops::DerefMut};

use bxcan::{Frame, StandardId};
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use stm32f4xx_hal::{gpio::GpioExt, hal::delay::DelayNs, interrupt, pac, prelude::*, rcc::RccExt};

static CAN: Mutex<RefCell<Option<bxcan::Can<stm32f4xx_hal::can::Can<pac::CAN1>>>>> =
    Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(16.MHz()).sysclk(180.MHz()).freeze();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let mut led_tx = gpioa.pa0.into_push_pull_output();
    let mut can = bxcan::Can::builder(dp.CAN1.can((gpiob.pb9, gpiob.pb8)))
        .set_bit_timing(0x001c_0003)
        .set_loopback(false)
        .set_silent(false)
        .leave_disabled();
    let mut filters = can.modify_filters();
    filters.enable_bank(0, bxcan::Fifo::Fifo0, bxcan::filter::Mask32::accept_all());
    filters.enable_bank(0, bxcan::Fifo::Fifo1, bxcan::filter::Mask32::accept_all());
    drop(filters);
    can.enable_interrupt(bxcan::Interrupt::Fifo0MessagePending);
    can.enable_interrupt(bxcan::Interrupt::Fifo1MessagePending);
    unsafe {
        stm32f4xx_hal::pac::NVIC::unmask(stm32f4xx_hal::interrupt::CAN1_RX0);
        stm32f4xx_hal::pac::NVIC::unmask(stm32f4xx_hal::interrupt::CAN1_RX1);
    }
    nb::block!(can.enable_non_blocking()).ok();
    info!("bxCAN initialized.");
    let mut delay = cp.SYST.delay(&clocks);
    free(|cs| {
        CAN.borrow(cs).replace(Some(can));
    });
    info!("Setup Complated.");

    let frame = Frame::new_data(StandardId::new(0x001).unwrap(), [3, 2, 1]);
    // let frame = Frame::new_data(StandardId::new(0x002).unwrap(), [1, 2, 3]);

    loop {
        led_tx.set_high();
        free(|cs| {
            if let Some(can) = CAN.borrow(cs).borrow_mut().deref_mut() {
                nb::block!(can.transmit(&frame)).unwrap();
            }
        });
        led_tx.set_low();
        delay.delay_ms(500);
    }
}

#[interrupt]
fn CAN1_RX0() {
    free(|cs| {
        if let Some(can) = CAN.borrow(cs).borrow_mut().deref_mut() {
            let data = nb::block!(can.receive()).unwrap();
            info!("FIFO0: {}", data);
        }
    });
}

#[interrupt]
fn CAN1_RX1() {
    free(|cs| {
        if let Some(can) = CAN.borrow(cs).borrow_mut().deref_mut() {
            let data = nb::block!(can.receive()).unwrap();
            info!("FIFO1: {}", data);
        }
    });
}
