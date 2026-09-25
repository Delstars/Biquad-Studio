use crate::denormals::DenormalGuard;
use crate::error::AudioEngineError;
use crate::mmcss::MmcssGuard;
use crate::params::AtomicParams;
use crate::telemetry::MeterFrame;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct AudioEngine {
    params: Arc<AtomicParams>,
    audio_thread: Option<JoinHandle<()>>,
    telemetry_rx: Option<rtrb::Consumer<MeterFrame>>,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            params: Arc::new(AtomicParams::new()),
            audio_thread: None,
            telemetry_rx: None,
        }
    }

    pub fn params(&self) -> &Arc<AtomicParams> {
        &self.params
    }

    pub fn start(&mut self, _output_device_id: Option<&str>) -> Result<(), AudioEngineError> {
        if self.is_running() {
            return Err(AudioEngineError::AlreadyRunning);
        }

        self.params.running.store(true, Ordering::Relaxed);
        let params = Arc::clone(&self.params);

        let (producer, consumer) = rtrb::RingBuffer::new(32);
        self.telemetry_rx = Some(consumer);

        self.audio_thread = Some(thread::spawn(move || {
            let _denormals = DenormalGuard::enable();
            let _mmcss = MmcssGuard::register_pro_audio().ok();

            unsafe {
                let _ = windows::Win32::System::Com::CoInitializeEx(
                    None,
                    windows::Win32::System::Com::COINIT_MULTITHREADED,
                );
            }

            let mut producer = producer;
            let mut counter = 0;

            while params.running.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(33));

                if counter % 1 == 0 {
                    let mut frame = MeterFrame::silent();
                    frame.sample_rate = 48000;
                    frame.buffer_frames = 1024;
                    let _ = producer.push(frame);
                }
                counter += 1;
            }
        }));

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), AudioEngineError> {
        if !self.is_running() {
            return Err(AudioEngineError::NotRunning);
        }
        self.params.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.audio_thread.take() {
            let _ = handle.join();
        }
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.params.running.load(Ordering::Relaxed)
    }

    pub fn take_telemetry_rx(&mut self) -> Option<rtrb::Consumer<MeterFrame>> {
        self.telemetry_rx.take()
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
