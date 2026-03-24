use esp_idf_svc::hal::{gpio::OutputPin, ledc::LedcChannel, ledc::*};

/// サーボ制御構造体
pub struct Servo<'d> {
    channel: Option<LedcDriver<'d>>,
    is_attached: bool,
}

impl<'d> Servo<'d> {
    /// new（未初期化状態）
    pub fn new() -> Self {
        Self {
            channel: None,
            is_attached: false,
        }
    }

    /// 初期化（setup）
    pub fn setup<C, P>(
        &mut self,
        channel: C,
        timer_driver: &LedcTimerDriver<'d, C::SpeedMode>,
        pin: P,
    ) -> anyhow::Result<()>
    where
        C: LedcChannel + 'd,
        P: OutputPin + 'd,
    {
        if let Some(mut ch) = self.channel.take() {
            let _ = ch.set_duty(0);
        }
        let channel_driver = LedcDriver::new(channel, timer_driver, pin)?;

        self.channel = Some(channel_driver);
        self.detach(); // 初期状態はdetach

        Ok(())
    }

    pub fn detach(&mut self) {
        match self.channel.as_mut() {
            Some(c) => {
                let _ = c.set_duty(0);
            }
            None => return,
        };
        self.is_attached = false;
    }

    pub fn attach(&mut self) {
        self.is_attached = true;
    }

    /// 角度設定 (0〜180)
    pub fn set(&mut self, angle: f32) -> anyhow::Result<()> {
        let ch = match self.channel.as_mut() {
            Some(c) => c,
            None => {
                // setupしてない → 無視
                return Ok(());
            }
        };
        if !self.is_attached {
            // detachされてる → 無視
            return Ok(());
        }

        // 角度制限
        let angle = angle.clamp(0.0, 180.0);

        // サーボPWM: 0.5ms〜2.5ms (周期20ms)
        let min_duty = 0.025; // 0.5ms / 20ms
        let max_duty = 0.125; // 2.5ms / 20ms

        let duty = min_duty + (angle / 180.0) * (max_duty - min_duty);

        let max_duty_val = ch.get_max_duty();
        let duty_val = (duty * max_duty_val as f32) as u32;

        ch.set_duty(duty_val)?;

        Ok(())
    }
}
