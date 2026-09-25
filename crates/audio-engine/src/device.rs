//! Audio device enumeration and management via WASAPI/MMDevice.

use crate::error::AudioEngineError;
use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::*;

/// Information about an available audio endpoint device.
#[derive(Debug, Clone, serde::Serialize)]
pub struct AudioDeviceInfo {
    pub id: String,
    pub name: String,
    pub is_default: bool,
    pub sample_rate: u32,
    pub channels: u16,
}

/// Enumerate all active render (playback) audio endpoints.
pub fn enumerate_render_devices() -> Result<Vec<AudioDeviceInfo>, AudioEngineError> {
    unsafe { enumerate_devices(eRender) }
}

/// Enumerate all active capture (recording) audio endpoints.
pub fn enumerate_capture_devices() -> Result<Vec<AudioDeviceInfo>, AudioEngineError> {
    unsafe { enumerate_devices(eCapture) }
}

/// Get the default render device.
pub fn get_default_render_device() -> Result<AudioDeviceInfo, AudioEngineError> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia)?;
        get_device_info(&device, true)
    }
}

unsafe fn enumerate_devices(
    data_flow: EDataFlow,
) -> Result<Vec<AudioDeviceInfo>, AudioEngineError> {
    let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
    let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
    let collection = enumerator.EnumAudioEndpoints(data_flow, DEVICE_STATE_ACTIVE)?;

    let count = collection.GetCount()?;
    let mut devices = Vec::with_capacity(count as usize);

    let default_id = enumerator
        .GetDefaultAudioEndpoint(data_flow, eMultimedia)
        .and_then(|dev| dev.GetId())
        .map(|pwstr| pwstr.to_string().unwrap_or_default())
        .unwrap_or_default();

    for i in 0..count {
        if let Ok(device) = collection.Item(i) {
            if let Ok(mut info) = get_device_info(&device, false) {
                info.is_default = info.id == default_id;
                devices.push(info);
            }
        }
    }

    Ok(devices)
}

unsafe fn get_device_info(
    device: &IMMDevice,
    is_default: bool,
) -> Result<AudioDeviceInfo, AudioEngineError> {
    // Get device ID
    let pwstr_id = device.GetId()?;
    let id = pwstr_id.to_string().unwrap_or_default();

    // Get friendly name from property store
    let store = device.OpenPropertyStore(STGM_READ)?;
    let prop = store.GetValue(&PKEY_Device_FriendlyName)?;

    // Extract the string from the PROPVARIANT union.
    // The pwszVal field is valid when the variant type is VT_LPWSTR.
    let name = {
        let pwstr = prop.Anonymous.Anonymous.Anonymous.pwszVal;
        if pwstr.is_null() {
            "Unknown Device".to_string()
        } else {
            pwstr
                .to_string()
                .unwrap_or_else(|_| "Unknown Device".to_string())
        }
    };

    // Get mix format for sample rate and channel count
    let client: IAudioClient = device.Activate(CLSCTX_ALL, None)?;
    let format_ptr = client.GetMixFormat()?;
    let format = &*format_ptr;

    let (sample_rate, channels) = (format.nSamplesPerSec, format.nChannels);
    CoTaskMemFree(Some(format_ptr as *const _ as *mut _));

    Ok(AudioDeviceInfo {
        id,
        name,
        is_default,
        sample_rate,
        channels,
    })
}
