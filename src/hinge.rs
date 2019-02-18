use crate::util::Result;
use mcp23x17::Input;
use mcp23x17::IoValue;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum HingeState {
    Closed,
    Open,
}

pub struct Hinge {
    in_pin: Input,
}

impl Hinge {
    pub fn new(in_pin: Input) -> Self {
        Hinge { in_pin }
    }

    pub fn read_state(&mut self) -> Result<HingeState> {
        Ok(match self.in_pin.read_value()? {
            IoValue::Low => HingeState::Open,
            IoValue::High => HingeState::Closed,
        })
    }
}
