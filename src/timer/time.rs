use std::time::Instant;

pub struct RsTime {
    last_frame: Instant
}

impl RsTime {
    pub fn new() -> Self {
        Self { 
            last_frame: Instant::now() 
        }
    }

    pub fn get_time(&mut self) -> f32 {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        dt
    }
 

}

