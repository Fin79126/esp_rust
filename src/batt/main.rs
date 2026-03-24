// mod monitor;
// mod old;
mod servo;
mod tcp;
mod udp;

use core::sync::atomic::AtomicU32;
use esp_idf_svc::hal::delay::FreeRtos;
use std::thread;

use anyhow::Result;
// use monitor::monitor_task;
use common::secret::{wifi_password, wifi_ssid};
use core::sync::atomic::AtomicBool;
use dotenv::dotenv;
use servo::BattServo;
use tcp::tcp_task;
use udp::udp_task;

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, log::EspLogger, sys::link_patches, wifi::*};
use heapless::String;
use std::{thread::sleep, time::Duration};

pub static ATTACH_SERVO: AtomicBool = AtomicBool::new(false);
pub static DETACH_SERVO: AtomicBool = AtomicBool::new(false);
pub static HEARTBEAT: AtomicU32 = AtomicU32::new(0);

fn main() -> Result<()> {
    // ESP-IDF 初期化
    link_patches();
    EspLogger::initialize_default();

    dotenv().ok();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    let mut batt_servo = BattServo::new();
    batt_servo.setup_servos(peripherals.ledc, peripherals.pins)?;

    // =========================
    // WiFi設定
    // =========================
    let mut wifi = EspWifi::new(peripherals.modem, sysloop.clone(), None)?;

    let ssid: String<32> = wifi_ssid();
    let password: String<64> = wifi_password();
    let wifi_config = Configuration::Client(ClientConfiguration {
        ssid: ssid,
        password: password,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_config)?;
    wifi.start()?;
    wifi.connect()?;

    log::info!("Connecting to WiFi...");
    while !wifi.is_connected()? {
        sleep(Duration::from_millis(2000));
        log::info!("Waiting for WiFi connection...");
    }
    log::info!("WiFi connected!");

    thread::spawn(move || tcp_task());

    while let Err(e) = udp_task(&mut batt_servo) {
        log::error!("UDP task error: {:?}", e);
        // エラーが発生したら少し待ってから再起動
        FreeRtos::delay_ms(1000);
    }

    // monitor_task(heartbeat);

    Ok(())
}
