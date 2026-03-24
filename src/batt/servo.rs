use common::servo::Servo;
use common::types::BattCommand;
use esp_idf_svc::hal::{gpio::Pins, ledc::*, units::FromValueType};

pub struct BattServo {
    servo1: Servo<'static>,
    servo2: Servo<'static>,
    servo3: Servo<'static>,
    servo4: Servo<'static>,
}

impl BattServo {
    pub fn new() -> Self {
        Self {
            servo1: Servo::new(),
            servo2: Servo::new(),
            servo3: Servo::new(),
            servo4: Servo::new(),
        }
    }

    pub fn setup_servos(&mut self, ledc: LEDC, pins: Pins) -> anyhow::Result<()> {
        let timer_config = config::TimerConfig::default().frequency(50.Hz().into());
        let timer_driver = LedcTimerDriver::new(ledc.timer0, &timer_config)?;
        self.servo1
            .setup(ledc.channel0, &timer_driver, pins.gpio18)?;
        self.servo2
            .setup(ledc.channel1, &timer_driver, pins.gpio19)?;
        self.servo3
            .setup(ledc.channel2, &timer_driver, pins.gpio20)?;
        self.servo4
            .setup(ledc.channel3, &timer_driver, pins.gpio21)?;
        Ok(())
    }

    pub fn set_servos_angle(&mut self, command: &BattCommand) -> anyhow::Result<()> {
        self.servo1.set(command.servo[0] as f32)?;
        self.servo2.set(command.servo[1] as f32)?;
        self.servo3.set(command.servo[2] as f32)?;
        self.servo4.set(command.servo[3] as f32)?;
        Ok(())
    }

    pub fn attach_servos(&mut self) {
        self.servo1.attach();
        self.servo2.attach();
        self.servo3.attach();
        self.servo4.attach();
    }

    pub fn detach_servos(&mut self) {
        self.servo1.detach();
        self.servo2.detach();
        self.servo3.detach();
        self.servo4.detach();
    }
}
