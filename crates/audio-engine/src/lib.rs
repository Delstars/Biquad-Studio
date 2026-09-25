pub mod denormals;
pub mod device;
pub mod engine;
pub mod error;
pub mod mmcss;
pub mod noise_removal;
pub mod params;
pub mod spatial;
pub mod telemetry;
pub mod wasapi_capture;
pub mod wasapi_render;
pub mod stream_mixer;

pub use denormals::DenormalGuard;
pub use device::{
    enumerate_capture_devices, enumerate_render_devices, get_default_render_device, AudioDeviceInfo,
};
pub use engine::AudioEngine;
pub use error::AudioEngineError;
pub use mmcss::MmcssGuard;
pub use params::{
    AtomicParams, CHANNEL_CHAT, CHANNEL_GAME, CHANNEL_MEDIA, CHANNEL_MIC, MAX_EQ_BANDS,
    NUM_CHANNELS,
};
pub use spatial::SpatialAudioEngine;
pub use telemetry::{compute_meters, MeterFrame};
pub use stream_mixer::StreamMixer;
