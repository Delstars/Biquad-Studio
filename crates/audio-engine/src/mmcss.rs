//! MMCSS (Multimedia Class Scheduler Service) thread priority management.
//!
//! Registers the audio processing thread as "Pro Audio" with MMCSS to receive
//! elevated scheduling priority, preventing audio glitches from background tasks.

use crate::error::AudioEngineError;
use windows::core::w;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Threading::{
    AvRevertMmThreadCharacteristics, AvSetMmThreadCharacteristicsW,
};

/// RAII guard that registers the current thread with MMCSS "Pro Audio" priority
/// and reverts on drop.
pub struct MmcssGuard {
    handle: HANDLE,
}

impl MmcssGuard {
    /// Register the current thread with MMCSS as "Pro Audio" for real-time scheduling.
    pub fn register_pro_audio() -> Result<Self, AudioEngineError> {
        let mut task_index = 0u32;
        let handle = unsafe { AvSetMmThreadCharacteristicsW(w!("Pro Audio"), &mut task_index) }
            .map_err(|e| AudioEngineError::StreamInit(format!("MMCSS registration failed: {e}")))?;
        tracing::debug!("MMCSS Pro Audio registered (task_index={})", task_index);
        Ok(Self { handle })
    }
}

impl Drop for MmcssGuard {
    fn drop(&mut self) {
        unsafe {
            if let Err(e) = AvRevertMmThreadCharacteristics(self.handle) {
                tracing::warn!("Failed to revert MMCSS characteristics: {e}");
            }
        }
    }
}
