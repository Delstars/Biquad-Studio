use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;
use windows::core::Result;
use windows::Win32::Media::Audio::{
    eConsole, eRender, IAudioCaptureClient, IAudioClient, IMMDeviceEnumerator, MMDeviceEnumerator,
    AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_LOOPBACK,
};
use windows::Win32::System::Com::{
    CoInitializeEx, CoTaskMemFree, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
};

pub struct WasapiCapture {
    running: Arc<AtomicBool>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Default for WasapiCapture {
    fn default() -> Self {
        Self::new()
    }
}

impl WasapiCapture {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            thread: None,
        }
    }

    pub fn start(&mut self, _pid: u32) -> Result<()> {
        if self.running.load(Ordering::SeqCst) {
            return Ok(());
        }
        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();

        self.thread = Some(thread::spawn(move || {
            unsafe {
                let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

                // Note: Standard endpoint loopback (AUDCLNT_STREAMFLAGS_LOOPBACK)
                // is used here as a fallback. It does not support event-driven mode
                // (AUDCLNT_STREAMFLAGS_EVENTCALLBACK). A small poll sleep is used instead.
                if let Err(e) = Self::capture_loop(running) {
                    tracing::error!("Capture loop error: {:?}", e);
                }

                CoUninitialize();
            }
        }));

        Ok(())
    }

    pub fn stop(&mut self) {
        if self.running.swap(false, Ordering::SeqCst) {
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
        }
    }

    unsafe fn capture_loop(running: Arc<AtomicBool>) -> Result<()> {
        let enumerator: IMMDeviceEnumerator =
            windows::Win32::System::Com::CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        // Fallback to capturing the default render device's loopback
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
        let audio_client: IAudioClient = device.Activate(CLSCTX_ALL, None)?;

        let mix_format_ptr = audio_client.GetMixFormat()?;
        let mix_format = *mix_format_ptr;

        // Note: EVENTCALLBACK is invalid with LOOPBACK on standard endpoints.
        let flags = AUDCLNT_STREAMFLAGS_LOOPBACK;
        let hns_requested_duration = 10000000; // 1 second buffer

        audio_client.Initialize(
            AUDCLNT_SHAREMODE_SHARED,
            flags,
            hns_requested_duration,
            0,
            &mix_format,
            None,
        )?;

        let capture_client: IAudioCaptureClient = audio_client.GetService()?;

        audio_client.Start()?;

        // Calculate sleep time (~half the engine period for polling)
        let sleep_duration = Duration::from_millis(10);

        while running.load(Ordering::SeqCst) {
            loop {
                let packet_length = capture_client.GetNextPacketSize()?;

                if packet_length == 0 {
                    break;
                }

                let mut data = std::ptr::null_mut();
                let mut num_frames = 0;
                let mut flags = 0;

                capture_client.GetBuffer(&mut data, &mut num_frames, &mut flags, None, None)?;

                // Here we would push `data` to the rtrb ring buffer for the engine

                capture_client.ReleaseBuffer(num_frames)?;
            }

            thread::sleep(sleep_duration);
        }

        audio_client.Stop()?;

        CoTaskMemFree(Some(mix_format_ptr as *const _ as *mut _));

        Ok(())
    }
}
