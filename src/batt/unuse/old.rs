use anyhow::Result;
use esp_idf_svc::hal::{ledc::*, peripherals::Peripherals, units::FromValueType};
use esp_idf_svc::netif::NetifStack;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop, log::EspLogger, netif::EspNetif, sys::link_patches, wifi::*,
};
use heapless::String;
use postcard::from_bytes;
use serde::{Deserialize, Serialize};
use std::net::UdpSocket;
use std::{thread::sleep, time::Duration};

#[derive(Serialize, Deserialize, Debug)]
pub struct MotorCommand {
    pub motor_left: f32,
    pub motor_right: f32,
    pub servo: f32,
}
fn main_old() -> Result<()> {
    // ESP-IDF 初期化
    link_patches();

    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    // =========================
    // WiFi設定
    // =========================
    let mut wifi = EspWifi::new(peripherals.modem, sysloop.clone(), None)?;

    let ssid: String<32> = String::try_from("Buffalo-G-AB50_EXT").unwrap();
    let password: String<64> = String::try_from("5mmbput4a4bck").unwrap();
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

    if let Ok(netif) = EspNetif::new(NetifStack::Sta) {
        let ip_info = netif.get_ip_info().unwrap();
        log::info!("IP: {}", ip_info.ip);
    }
    // =========================
    // PWM（サーボ用）
    // =========================
    let ledc = peripherals.ledc;
    let timer = LedcTimerDriver::new(
        ledc.timer0,
        &config::TimerConfig::default().frequency(50.Hz().into()),
    )?;

    let mut channel = LedcDriver::new(ledc.channel0, &timer, peripherals.pins.gpio18)?;

    // =========================
    // UDP受信
    // =========================
    let socket = UdpSocket::bind("0.0.0.0:5001")?;
    log::info!("UDP listening on 5001");

    let mut buf = [0u8; 128];

    loop {
        let (size, src) = socket.recv_from(&mut buf)?;

        let data = &buf[..size];

        if let Ok(cmd) = from_bytes::<MotorCommand>(data) {
            log::info!("Received from {}: {:?}", src, cmd);

            set_servo_angle(&mut channel, cmd.servo)?;
        } else {
            log::warn!("Failed to decode packet from {}", src);
        }
    }
}

// =========================
// サーボ制御
// =========================
fn set_servo_angle(channel: &mut LedcDriver, angle: f32) -> Result<()> {
    // 角度制限
    let angle = angle.clamp(0.0, 180.0);

    // サーボパルス（0.5ms〜2.5ms）
    let min_duty = 0.05; // 0.5ms / 20ms
    let max_duty = 0.125; // 2.5ms / 20ms

    let duty = min_duty + (angle / 180.0) * (max_duty - min_duty);

    let max = channel.get_max_duty();
    let duty_val = (duty * max as f32) as u32;

    channel.set_duty(duty_val)?;

    log::info!("Set angle: {}", angle);

    Ok(())
}

// let ssid: String<32> = String::try_from("ESP32_AP").unwrap();
// let password: String<64> = String::try_from("12345678").unwrap();

// let wifi_config = Configuration::AccessPoint(AccessPointConfiguration {
//     ssid,
//     password,
//     channel: 6,
//     max_connections: 4,
//     auth_method: AuthMethod::WPA2Personal,
//     ..Default::default()
// });

// wifi.set_configuration(&wifi_config)?;
// wifi.start()?;

// // APでは「接続待ち」は不要
// log::info!("WiFi Access Point started!");
// log::info!("SSID: ESP32_AP");
// log::info!("IP: 192.168.4.1 (default)");

// // =========================
// // PWM（サーボ用）
// // =========================
// let ledc = peripherals.ledc;
// let timer = LedcTimerDriver::new(
//     ledc.timer0,
//     &config::TimerConfig::default().frequency(50.Hz().into()),
// )?;

// let mut channel = LedcDriver::new(ledc.channel0, &timer, peripherals.pins.gpio18)?;

// // =========================
// // UDP受信（AP側）
// // =========================
// let socket = UdpSocket::bind("0.0.0.0:8888")?;
// log::info!("UDP listening on 8888");

// let mut buf = [0u8; 128];

// loop {
//     let (size, src) = socket.recv_from(&mut buf)?;
//     let msg = core::str::from_utf8(&buf[..size])?.trim();

//     log::info!("Received from {}: {}", src, msg);

//     if let Ok(angle) = msg.parse::<f32>() {
//         set_servo_angle(&mut channel, angle)?;
//     }
// }
