//! Denormal float protection for the real-time audio processing thread.
//!
//! On x86/x64 CPUs, denormalized (subnormal) floating-point numbers trigger
//! hardware microcode exceptions that are orders of magnitude slower than
//! normal float operations. This causes catastrophic CPU spikes in audio
//! processing loops when signals fade to silence.
//!
//! This module provides an RAII guard that enables Flush-To-Zero (FTZ) and
//! Denormals-Are-Zero (DAZ) modes in the SSE control register (MXCSR).

/// Bit flag for Flush-To-Zero mode in the MXCSR register.
const FTZ_BIT: u32 = 1 << 15; // 0x8000
/// Bit flag for Denormals-Are-Zero mode in the MXCSR register.
const DAZ_BIT: u32 = 1 << 6; // 0x0040

/// RAII guard that enables FTZ+DAZ on construction and restores the previous
/// MXCSR state on drop. Must be created on the audio processing thread.
pub struct DenormalGuard {
    #[cfg(target_arch = "x86_64")]
    previous_mxcsr: u32,
}

impl DenormalGuard {
    /// Enable Flush-To-Zero and Denormals-Are-Zero modes on the current thread.
    ///
    /// # Safety
    /// This modifies the thread-local SSE control register. Only call on threads
    /// where denormal protection is needed (i.e., the audio processing thread).
    #[inline(always)]
    pub fn enable() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            let previous_mxcsr: u32;
            // Read current MXCSR via STMXCSR, set FTZ+DAZ, write back via LDMXCSR
            unsafe {
                let mut mxcsr: u32 = 0;
                std::arch::asm!(
                    "stmxcsr [{}]",
                    in(reg) &mut mxcsr,
                    options(nostack)
                );
                previous_mxcsr = mxcsr;
                mxcsr |= FTZ_BIT | DAZ_BIT;
                std::arch::asm!(
                    "ldmxcsr [{}]",
                    in(reg) &mxcsr,
                    options(nostack, readonly)
                );
            }
            Self { previous_mxcsr }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            Self {}
        }
    }
}

impl Drop for DenormalGuard {
    #[inline(always)]
    fn drop(&mut self) {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            std::arch::asm!(
                "ldmxcsr [{}]",
                in(reg) &self.previous_mxcsr,
                options(nostack, readonly)
            );
        }
    }
}
