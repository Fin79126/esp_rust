use anyhow::Result;
use common::servo::Servo;
use esp_idf_svc::hal::{
    ledc::config, ledc::LedcTimerDriver, peripherals::Peripherals, units::FromValueType,
};
use esp_idf_svc::{log::EspLogger, sys::link_patches};
use serde::{Deserialize, Serialize};
use std::{thread::sleep, time::Duration};

#[derive(Serialize, Deserialize, Debug)]
pub struct MotorCommand {
    pub motor_left: f32,
    pub motor_right: f32,
    pub servo: f32,
}
fn main() -> Result<()> {
    // ESP-IDF 初期化
    link_patches();

    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    // =========================
    // PWM（サーボ用）
    // =========================
    let ledc = peripherals.ledc;
    let timer = LedcTimerDriver::new(
        ledc.timer0,
        &config::TimerConfig::default().frequency(50.Hz().into()),
    )?;

    let mut servo = Servo::new();
    servo.setup(ledc.channel0, &timer, peripherals.pins.gpio18)?;

    loop {
        servo.set(90.0)?;
        log::info!("Set servo to 90 degrees");
        sleep(Duration::from_secs(2));
        servo.set(0.0)?;
        log::info!("Set servo to 0 degrees");
        sleep(Duration::from_secs(2));
    }
}
