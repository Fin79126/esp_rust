use common::bldc::BLDC;
use common::servo::Servo;
use common::types::LegsCommand;
use esp_idf_svc::hal::{gpio::Pins, ledc::*, units::FromValueType};

pub struct LegsUnit {
    servo1: Servo<'static>,
    servo2: Servo<'static>,
    servo3: Servo<'static>,
    servo4: Servo<'static>,
    servo5: Servo<'static>,
    servo6: Servo<'static>,
    servo7: Servo<'static>,
    servo8: Servo<'static>,
    servo9: Servo<'static>,
    servo10: Servo<'static>,
    servo11: Servo<'static>,
    servo12: Servo<'static>,
    bldc1: BLDC<'static>,
    bldc2: BLDC<'static>,
}

impl LegsUnit {
    pub fn new() -> Self {
        Self {
            servo1: Servo::new(),
            servo2: Servo::new(),
            servo3: Servo::new(),
            servo4: Servo::new(),
            servo5: Servo::new(),
            servo6: Servo::new(),
            servo7: Servo::new(),
            servo8: Servo::new(),
            servo9: Servo::new(),
            servo10: Servo::new(),
            servo11: Servo::new(),
            servo12: Servo::new(),
            bldc1: BLDC::new(),
            bldc2: BLDC::new(),
        }
    }

    pub fn setup_units(&mut self, ledc: LEDC, hledc: HLEDC, pins: Pins) -> anyhow::Result<()> {
        let timer_config = config::TimerConfig::default().frequency(50.Hz().into());
        let low_timer_driver = LedcTimerDriver::new(ledc.timer0, &timer_config)?;
        let high_timer_driver = LedcTimerDriver::new(hledc.timer0, &timer_config)?;
        self.servo1
            .setup(ledc.channel0, &low_timer_driver, pins.gpio23)?;
        self.servo2
            .setup(ledc.channel1, &low_timer_driver, pins.gpio22)?;
        self.servo3
            .setup(ledc.channel2, &low_timer_driver, pins.gpio21)?;
        self.servo4
            .setup(ledc.channel3, &low_timer_driver, pins.gpio19)?;
        self.servo5
            .setup(ledc.channel4, &low_timer_driver, pins.gpio18)?;
        self.servo6
            .setup(ledc.channel5, &low_timer_driver, pins.gpio32)?;
        self.servo7
            .setup(ledc.channel6, &low_timer_driver, pins.gpio25)?;
        self.servo8
            .setup(ledc.channel7, &low_timer_driver, pins.gpio5)?;
        self.servo9
            .setup(hledc.channel0, &high_timer_driver, pins.gpio26)?;
        self.servo10
            .setup(hledc.channel1, &high_timer_driver, pins.gpio27)?;
        self.servo11
            .setup(hledc.channel2, &high_timer_driver, pins.gpio14)?;
        self.servo12
            .setup(hledc.channel3, &high_timer_driver, pins.gpio13)?;
        self.bldc1
            .setup(hledc.channel4, &high_timer_driver, pins.gpio2)?;
        self.bldc2
            .setup(hledc.channel5, &high_timer_driver, pins.gpio4)?;
        Ok(())
    }

    pub fn set_units(&mut self, command: &LegsCommand) -> anyhow::Result<()> {
        self.servo1.set(command.servo[0])?;
        self.servo2.set(command.servo[1])?;
        self.servo3.set(command.servo[2])?;
        self.servo4.set(command.servo[3])?;
        self.servo5.set(command.servo[4])?;
        self.servo6.set(command.servo[5])?;
        self.servo7.set(command.servo[6])?;
        self.servo8.set(command.servo[7])?;
        self.servo9.set(command.servo[8])?;
        self.servo10.set(command.servo[9])?;
        self.servo11.set(command.servo[10])?;
        self.servo12.set(command.servo[11])?;
        self.bldc1.set(command.bldc[0])?;
        self.bldc2.set(command.bldc[1])?;
        Ok(())
    }

    pub fn attach_units(&mut self) {
        self.servo1.attach();
        self.servo2.attach();
        self.servo3.attach();
        self.servo4.attach();
        self.servo5.attach();
        self.servo6.attach();
        self.servo7.attach();
        self.servo8.attach();
        self.servo9.attach();
        self.servo10.attach();
        self.servo11.attach();
        self.servo12.attach();
        self.bldc1.attach();
        self.bldc2.attach();
    }

    pub fn detach_units(&mut self) {
        self.servo1.detach();
        self.servo2.detach();
        self.servo3.detach();
        self.servo4.detach();
        self.servo5.detach();
        self.servo6.detach();
        self.servo7.detach();
        self.servo8.detach();
        self.servo9.detach();
        self.servo10.detach();
        self.servo11.detach();
        self.servo12.detach();
        self.bldc1.detach();
        self.bldc2.detach();
    }
}
