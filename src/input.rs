use ch32_hal::gpio::Input;

pub enum Action {
    Pressed,
    Released,
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
        if self.pin.is_low() {
            if !self.on {
                self.on = true;
                return Some(Action::Pressed);
            }
        } else if self.on {
            self.on = false;
            return Some(Action::Released);
        }
        None
    }

    /*
    pub fn probe<F>(&mut self, f: F) -> Option<Action>
    where
        F: FnOnce() -> bool,
    {
        if f() {
            if !self.on {
                self.on = true;
                return Some(Action::Pressed);
            }
        } else if self.on {
            self.on = false;
            return Some(Action::Released);
        }
        None
    }
    */
}
