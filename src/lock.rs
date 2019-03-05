use crate::util::Result;
use mcp23x17::IoValue;
use mcp23x17::Output;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::time::Duration;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum LockState {
    Locked,
    Unlocked,
}

#[derive(Debug)]
pub struct LockBusy {}

impl Display for LockBusy {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Lock is busy")
    }
}

impl Error for LockBusy {}

pub struct Lock {
    out_pin: Output,
    last_change: SystemTime,
}

impl Lock {
    pub fn new(out_pin: Output) -> Result<Self> {
        let mut lock = Lock {
            out_pin,
            last_change: std::time::UNIX_EPOCH,
        };
        lock.set_state(LockState::Locked)?;
        Ok(lock)
    }

    pub fn read_state(&self) -> Result<LockState> {
        Ok(match self.out_pin.read_value()? {
            IoValue::Low => LockState::Locked,
            IoValue::High => LockState::Unlocked,
        })
    }

    pub fn set_state(&mut self, state: LockState) -> Result {
        let now = SystemTime::now();
        if now < self.last_change + Duration::from_millis(250) {
            Err(LockBusy {})?;
        }
        self.last_change = now;
        self.out_pin.set_value(match state {
            LockState::Locked => IoValue::Low,
            LockState::Unlocked => IoValue::High,
        })?;
        Ok(())
    }

    pub fn last_change(&self) -> SystemTime {
        self.last_change
    }
}
