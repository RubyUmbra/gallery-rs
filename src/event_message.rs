use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub(crate) enum EventMessage {
    Next,
    Prev,
    Move,
    Quit,
}

impl TryFrom<Event> for EventMessage {
    type Error = ();

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        match value {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => Ok(EventMessage::Quit),
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => Ok(EventMessage::Next),
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => Ok(EventMessage::Prev),
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => Ok(EventMessage::Move),
            _ => Err(()),
        }
    }
}
