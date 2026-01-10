use bitflags::bitflags;
use ch32_hal::gpio::Input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Up,
    Down,
    A,
    B,
}

impl Event {
    const fn flag(self) -> Pressed {
        match self {
            Event::Up => Pressed::UP,
            Event::Down => Pressed::DOWN,
            Event::A => Pressed::A,
            Event::B => Pressed::B,
        }
    }
}

pub enum Action {
    Pressed,
}

bitflags! {
    #[derive(Clone, Copy)]
    struct Pressed: u8 {
        const UP = 1 << 0;
        const DOWN = 1 << 1;
        const A = 1 << 2;
        const B = 1 << 3;
    }
}

pub struct Buttons<'d> {
    pub up: Input<'d>,
    pub down: Input<'d>,
    pub a: Input<'d>,
    pub b: Input<'d>,
    pressed: Pressed,
}

impl<'d> Buttons<'d> {
    pub const fn new(up: Input<'d>, down: Input<'d>, a: Input<'d>, b: Input<'d>) -> Self {
        Self {
            up,
            down,
            a,
            b,
            pressed: Pressed::empty(),
        }
    }

    pub fn scan(&mut self) -> Option<Event> {
        for (pin, event) in [
            (&self.up, Event::Up),
            (&self.down, Event::Down),
            (&self.a, Event::A),
            (&self.b, Event::B),
        ] {
            if let Some(Action::Pressed) = Self::probe(&pin, &mut self.pressed, event.flag()) {
                return Some(event);
            }
        }
        None
    }

    fn probe(input: &Input<'d>, pressed: &mut Pressed, flag: Pressed) -> Option<Action> {
        if input.is_low() && !pressed.contains(flag) {
            pressed.insert(flag);
            Some(Action::Pressed)
        } else {
            pressed.remove(flag);
            None
        }
    }
}
