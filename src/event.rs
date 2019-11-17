use std;
use glium;

pub struct AnimationLoop {
    last_update: std::time::Instant,
}

impl AnimationLoop {
    pub fn new() -> Self {
        AnimationLoop {
            last_update: std::time::Instant::now(),
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self, events_loop: &mut glium::glutin::EventsLoop, interval_ms: u64) -> Vec<glium::glutin::Event> {
        let last_update = self.last_update;
        let frame_interval_ms = std::time::Duration::from_millis(interval_ms);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < frame_interval_ms {
            std::thread::sleep(frame_interval_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        self.last_update = std::time::Instant::now();
        events
    }
}
