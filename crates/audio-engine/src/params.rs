use atomic_float::AtomicF32;
use std::sync::atomic::AtomicBool;

pub const MAX_EQ_BANDS: usize = 10;
pub const NUM_CHANNELS: usize = 4; // Game, Chat, Media, Mic

pub const CHANNEL_GAME: usize = 0;
pub const CHANNEL_CHAT: usize = 1;
pub const CHANNEL_MEDIA: usize = 2;
pub const CHANNEL_MIC: usize = 3;

pub struct AtomicParams {
    pub master_volume: AtomicF32,
    pub channel_volumes: [AtomicF32; NUM_CHANNELS],
    pub crossfade_position: AtomicF32,
    pub master_mute: AtomicBool,
    pub channel_mutes: [AtomicBool; NUM_CHANNELS],
    pub eq_bypass: AtomicBool,
    pub running: AtomicBool,
}

impl AtomicParams {
    pub fn new() -> Self {
        Self {
            master_volume: AtomicF32::new(1.0),
            channel_volumes: [
                AtomicF32::new(1.0),
                AtomicF32::new(1.0),
                AtomicF32::new(1.0),
                AtomicF32::new(1.0),
            ],
            crossfade_position: AtomicF32::new(0.5),
            master_mute: AtomicBool::new(false),
            channel_mutes: [
                AtomicBool::new(false),
                AtomicBool::new(false),
                AtomicBool::new(false),
                AtomicBool::new(false),
            ],
            eq_bypass: AtomicBool::new(false),
            running: AtomicBool::new(false),
        }
    }
}

impl Default for AtomicParams {
    fn default() -> Self {
        Self::new()
    }
}
