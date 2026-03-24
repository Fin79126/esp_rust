use esp_idf_svc::hal::{gpio::OutputPin, ledc::LedcChannel, ledc::*};

/// サーボ制御構造体
pub struct BLDC<'d> {
    channel: Option<LedcDriver<'d>>,
    is_attached: bool,
}

impl<'d> BLDC<'d> {
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

    pub fn set(&mut self, power: i16) -> anyhow::Result<()> {
        if !self.is_attached {
            return Ok(()); // detach状態なら何もしない
        }
        if let Some(c) = self.channel.as_mut() {
            // power: -100〜100 → duty: 0.025〜0.125
            let power = power.clamp(-100, 100);
            let duty = 0.075 + (power as f32 / 200.0) * 0.1; // -100で0.025、0で0.075、100で0.125
            let max_duty_val = c.get_max_duty();
            let duty_val = (duty * max_duty_val as f32) as u32;
            c.set_duty(duty_val)?;
        }
        Ok(())
    }
}
