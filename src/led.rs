use crate::hinge::Hinge;
use crate::hinge::HingeState;
use crate::lock::Lock;
use crate::lock::LockState;
use crate::util::Result;
use mcp23x17::IoValue;
use mcp23x17::Output;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

pub struct LedController {
    green_leds: Vec<Output>,
    red_leds: Vec<Output>,
    lock: Arc<Mutex<Lock>>,
    hinge: Arc<Mutex<Hinge>>,
}

impl LedController {
    pub fn new(
        green_leds: Vec<Output>,
        red_leds: Vec<Output>,
        lock: Arc<Mutex<Lock>>,
        hinge: Arc<Mutex<Hinge>>,
    ) -> Self {
        Self {
            green_leds,
            red_leds,
            lock,
            hinge,
        }
    }

    pub fn run(&mut self) {
        loop {
            let _ = self.try_run().map_err(|err| {
                error!("LedController observed: {}", err);
            });
        }
    }

    pub fn try_run(&mut self) -> Result {
        let mut tick: u64 = 0;
        loop {
            match {
                let lock = &mut self.lock.lock().unwrap();
                let hinge = &mut self.hinge.lock().unwrap();
                (lock.read_state()?, hinge.read_state()?)
            } {
                (LockState::Locked, HingeState::Closed) => {
                    tick = 0;
                    self.red()?;
                }
                (LockState::Locked, HingeState::Open) => {
                    if tick < 5 {
                        self.red()?;
                    } else {
                        self.green()?;
                    }
                    tick += 1;
                    tick %= 10;
                }
                (LockState::Unlocked, _) => {
                    tick = 0;
                    self.green()?;
                }
            };
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn red(&mut self) -> Result {
        Self::set_values(&mut self.green_leds, IoValue::High)?;
        Self::set_values(&mut self.red_leds, IoValue::Low)?;
        Ok(())
    }

    fn green(&mut self) -> Result {
        Self::set_values(&mut self.green_leds, IoValue::Low)?;
        Self::set_values(&mut self.red_leds, IoValue::High)?;
        Ok(())
    }

    fn set_values(leds: &mut [Output], value: IoValue) -> Result {
        for led in leds {
            led.set_value(value)?;
        }
        Ok(())
    }
}
