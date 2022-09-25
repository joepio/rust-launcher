use rdev::{grab, Event, EventType, Key};
mod render;

fn main() {
    // This will block.
    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) -> Option<Event> {
    render::render();
    match event.event_type {
        EventType::KeyPress(Key::Tab) => {
            None
        }
        _ => Some(event),
    }
}
