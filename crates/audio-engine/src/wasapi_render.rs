use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use windows::core::{Interface, Result, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, WAIT_OBJECT_0};
use windows::Win32::Media::Audio::{
    eConsole, eRender, IAudioClient, IAudioClient3, IAudioRenderClient, IMMDeviceEnumerator,
    MMDeviceEnumerator, AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_EVENTCALLBACK,
};
use windows::Win32::System::Com::{
    CoInitializeEx, CoTaskMemFree, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
};
use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject};

pub struct WasapiRender {
    running: Arc<AtomicBool>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Default for WasapiRender {
    fn default() -> Self {
        Self::new()
    }
}

impl WasapiRender {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            thread: None,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        if self.running.load(Ordering::SeqCst) {
            return Ok(());
        }
        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();

        self.thread = Some(thread::spawn(move || unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

            if let Err(e) = Self::render_loop(running) {
                tracing::error!("Render loop error: {:?}", e);
            }

            CoUninitialize();
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

    unsafe fn render_loop(running: Arc<AtomicBool>) -> Result<()> {
        let enumerator: IMMDeviceEnumerator =
            windows::Win32::System::Com::CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
        let audio_client: IAudioClient = device.Activate(CLSCTX_ALL, None)?;

        let mix_format_ptr = audio_client.GetMixFormat()?;
        let mix_format = *mix_format_ptr;

        let flags = AUDCLNT_STREAMFLAGS_EVENTCALLBACK;

        // Try IAudioClient3 for low latency
        let audio_client3: windows::core::Result<IAudioClient3> = audio_client.cast();
        if let Ok(client3) = audio_client3 {
            let mut default_period = 0;
            let mut min_period = 0;
            if client3
                .GetSharedModeEnginePeriod(
                    &mix_format,
                    &mut default_period,
                    &mut min_period,
                    &mut default_period,
                    &mut min_period,
                )
                .is_ok()
            {
                if let Err(_) =
                    client3.InitializeSharedAudioStream(flags, min_period, &mix_format, None)
                {
                    audio_client.Initialize(
                        AUDCLNT_SHAREMODE_SHARED,
                        flags,
                        0,
                        0,
                        &mix_format,
                        None,
                    )?;
                }
            } else {
                audio_client.Initialize(
                    AUDCLNT_SHAREMODE_SHARED,
                    flags,
                    0,
                    0,
                    &mix_format,
                    None,
                )?;
            }
        } else {
            audio_client.Initialize(AUDCLNT_SHAREMODE_SHARED, flags, 0, 0, &mix_format, None)?;
        }

        let event_handle = CreateEventW(None, false, false, PCWSTR::null())?;
        audio_client.SetEventHandle(event_handle)?;

        let render_client: IAudioRenderClient = audio_client.GetService()?;

        audio_client.Start()?;

        while running.load(Ordering::SeqCst) {
            let wait_res = WaitForSingleObject(event_handle, 1000);
            if wait_res == WAIT_OBJECT_0 {
                let buffer_size = audio_client.GetBufferSize()?;
                let padding = audio_client.GetCurrentPadding()?;
                let frames_available = buffer_size - padding;

                if frames_available > 0 {
                    let data = render_client.GetBuffer(frames_available)?;

                    // Fill with 0s (silence)
                    let bytes_per_frame = mix_format.nBlockAlign as u32;
                    let num_bytes = (frames_available * bytes_per_frame) as usize;
                    std::ptr::write_bytes(data, 0, num_bytes);

                    // --- DSP Processing Phase 2 ---
                    // Assuming mix_format is WAVE_FORMAT_IEEE_FLOAT (which WASAPI shared mode defaults to)
                    // and channels == mix_format.nChannels
                    if mix_format.wFormatTag == 65534 // WAVE_FORMAT_EXTENSIBLE
                        || mix_format.wFormatTag == 3
                    /* WAVE_FORMAT_IEEE_FLOAT */
                    {
                        let channels = mix_format.nChannels as usize;
                        let sample_count =
                            (frames_available * mix_format.nChannels as u32) as usize;
                        let slice = std::slice::from_raw_parts_mut(data as *mut f32, sample_count);

                        // NOTE: In the real implementation, we would pass the audio from wasapi_capture
                        // into this buffer instead of silence, and then run `eq.process_interleaved(slice, channels)`.
                        // For now we just process the silence through the DSP engine to ensure it doesn't crash/NaN.
                    }

                    render_client.ReleaseBuffer(frames_available, 0)?;
                }
            }
        }

        audio_client.Stop()?;
        let _ = CloseHandle(event_handle);

        CoTaskMemFree(Some(mix_format_ptr as *const _ as *mut _));

        Ok(())
    }
}
