#![allow(dead_code)]

#[derive(Debug,Clone,Copy,Eq,PartialEq,Hash)]
pub struct Timebase {
    pub fps: u8,
    pub drop_frame: bool,
}

impl Timebase {
    pub fn new(fps: u8) -> Self {
        Timebase { fps, drop_frame: false }
    }

    pub fn from_real_fps(fps: f64) -> Self {
        let base_fps = (fps * 100.0) as i32;
        let rounded = (fps.round() as i32) * 100;
        let drop_frame = base_fps != rounded;
        Self {
            fps: (rounded / 100) as u8,
            drop_frame,
        }
    }

    pub fn milliseconds_per_frame(&self) -> u64 {
        1000 / self.fps as u64
    }
}

impl Default for Timebase {
    fn default() -> Self {
        Self{
            fps: 24,
            drop_frame: false,
        }
    }
}