use ch32_hal::gpio::Input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Up,
    Down,
    A,
    B,
}

pub enum Action {
    Pressed,
}

pub struct Button<'d> {
    pin: Input<'d>,
    on: bool,
}

impl Button<'_> {
    pub const fn new(pin: Input) -> Button {
        Button { pin, on: false }
    }

    pub fn probe(&mut self) -> Option<Action> {
        if self.pin.is_low() && !self.on {
            self.on = true;
            Some(Action::Pressed)
        } else {
            self.on = false;
            None
        }
    }
}
