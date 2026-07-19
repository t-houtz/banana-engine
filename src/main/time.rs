use std::time::Instant;

#[derive(Debug, Clone)]

pub struct Time {
    start: Instant,
    last_frame: Instant,

    frame_count: u64,

    raw_delta_seconds: f32,
    raw_elapsed_seconds: f32,

    delta_seconds: f32,
    elapsed_seconds: f32,

    time_scale: f32
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();

        Self {
            start: now,
            last_frame: now,

            frame_count: 0,

            raw_delta_seconds: 0.0,
            raw_elapsed_seconds: 0.0,

            delta_seconds: 0.0,
            elapsed_seconds: 0.0,

            time_scale: 1.0
        }
    }

    pub fn tick(&mut self) {
        self.frame_count += 1;
        let now = Instant::now();

        let mut raw_delta: f32 = (now - self.last_frame).as_secs_f32();
        if raw_delta > 0.1 {
            raw_delta = 0.1;
        }
        self.raw_delta_seconds = raw_delta;
        self.raw_elapsed_seconds = (now-self.start).as_secs_f32();

        self.delta_seconds = self.raw_delta_seconds * self.time_scale;
        self.elapsed_seconds += self.delta_seconds;

        self.last_frame = now;
    }

    pub fn frame_count(&self) -> f32 {
        self.frame_count
    }

    pub fn raw_delta_seconds(&self) -> f32 {
        self.raw_delta_seconds
    }
    pub fn raw_elapsed_seconds(&self) -> f32 {
        self.raw_elapsed_seconds
    }
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }
    pub fn elapsed_seconds(&self) -> f32 {
        self.elapsed_seconds
    }

    pub fn fps(&self) -> f32 {
        if self.raw_delta_seconds == 0.0 {
            0.0
        } 
        else {
            1.0 / self.raw_delta_seconds
        }
    }

    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale;
    }
    pub fn time_scale(&self) {
        self.time_scale
    }

    pub fn restart(&mut self) {
        let now = Instant::now();

        self.start = now;
        self.last_frame = now;
        self.frame_count = 0;
        self.raw_delta_seconds= 0.0;
        self.raw_elapsed_seconds= 0.0;
        self.delta_seconds = 0.0;
        self.elapsed_seconds = 0.0;
        self.time_scale = 1.0;
    }
}