#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_time::{Duration, Ticker};
use esp_hal::{prelude::*, rng::Rng, time};
use esp_wifi::{
    esp_now::{EspNow, BROADCAST_ADDRESS},
    wifi::Protocol,
    EspWifiController,
};
use {defmt_rtt as _, esp_backtrace as _};

extern crate alloc;

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_alloc::heap_allocator!(72 * 1024);

    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    let esp_wifi_ctrl = &*mk_static!(
        EspWifiController<'static>,
        esp_wifi::init(
            timg0.timer0,
            Rng::new(peripherals.RNG),
            peripherals.RADIO_CLK,
        )
        .unwrap()
    );

    let sys_timer = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER)
        .split::<esp_hal::timer::systimer::Target>();
    esp_hal_embassy::init(sys_timer.alarm0);
    info!("Embassy initialized!");

    let _ = spawner;

    let wifi = peripherals.WIFI;
    let mut esp_now = esp_wifi::esp_now::EspNow::new(&esp_wifi_ctrl, wifi).unwrap();
    // esp_now.set_protocol(Protocol::P802D11LR.into()).unwrap();
    esp_now.set_protocol(Protocol::P802D11B.into()).unwrap();
    info!("esp-now version {}", esp_now.version().unwrap());

    let mut total = 0;
    let mut success = 0;

    let mut ticker = Ticker::every(Duration::from_millis(1000));
    loop {
        total += 1;
        if let Either::Second(delay_time) = ping(&mut esp_now).await {
            success += 1;
            let loss = total - success;
            info!(
                "Total: {}, Loss: {}%, Time: {}ms",
                total,
                (loss as f32 / total as f32) * 100.,
                delay_time
            );
        } else {
            info!("Lost");
        }
        ticker.next().await;
    }
}

async fn ping(esp_now: &mut EspNow<'_>) -> Either<(), u64> {
    let send_data: [u8; 250] = core::array::from_fn(|i| i as u8 + 1);

    esp_now
        .send_async(&BROADCAST_ADDRESS, &send_data)
        .await
        .unwrap();
    let send_time = time::now();
    let mut ticker = Ticker::every(Duration::from_millis(500));
    let receive = async {
        loop {
            let recv = esp_now.receive_async().await;
            if recv.data() == send_data {
                return time::now()
                    .checked_duration_since(send_time)
                    .unwrap()
                    .to_millis();
            }
        }
    };
    select(ticker.next(), receive).await
}
