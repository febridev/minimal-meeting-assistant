use tauri::{AppHandle, Manager, State};
use std::path::PathBuf;
use std::env;

mod audio;
mod uploader;

#[cfg(target_os = "macos")]
mod native {
    use super::*;

    #[link(name = "meeting_assistant_swift", kind = "static")]
    extern "C" {
        pub fn start_capture(audio_buffer: *const audio::AudioBuffer);
        pub fn stop_capture();
    }
}

#[no_mangle]
pub extern "C" fn rust_add_data(buffer: *const audio::AudioBuffer, data: *const f32, len: usize) {
    let buffer = unsafe { &*buffer };
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    buffer.add_data(slice);
}

#[derive(Clone)]
pub struct AudioBuffer(std::sync::Arc<audio::AudioBuffer>);

impl AudioBuffer {
    fn new() -> Self {
        Self(std::sync::Arc::new(audio::AudioBuffer::new()))
    }

    fn add_data(&self, data: &[f32]) {
        self.0.add_data(data);
    }
    
    fn export_as_wav(&self, path: &PathBuf, sample_rate: u32, bits: u16) -> Result<(), hound::Error> {
        self.0.normalize();
        self.0.export_as_wav(path, sample_rate, bits)
    }

    fn clear(&self) {
        self.0.clear();
    }
}

#[tauri::command]
fn start_recording(audio_buffer: State<'_, AudioBuffer>) {
    #[cfg(target_os = "macos")]
    {
        unsafe {
            native::start_capture(std::sync::Arc::as_ptr(&audio_buffer.0));
        }
    }
}

#[tauri::command]
async fn debug_save_to_desktop(audio_buffer: State<'_, AudioBuffer>, bit_depth: u16) -> Result<String, String> {
    let desktop_dir = dirs::desktop_dir().ok_or("Could not find desktop directory")?;
    let file_path = desktop_dir.join("debug_audio.wav");
    
    let sample_rate = 48000;

    audio_buffer.export_as_wav(&file_path, sample_rate, bit_depth)
        .map_err(|e| e.to_string())?;

    audio_buffer.clear();

    Ok(format!("Saved audio to {}", file_path.display()))
}

#[tauri::command]
async fn stop_recording(app_handle: AppHandle, audio_buffer: State<'_, AudioBuffer>, bit_depth: u16) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    unsafe {
        native::stop_capture();
    }

    let temp_dir = env::temp_dir();
    let file_path = temp_dir.join("recorded_audio.wav");
    
    let sample_rate = 48000; 

    audio_buffer.export_as_wav(&file_path, sample_rate, bit_depth)
        .map_err(|e| e.to_string())?;

    // These should be securely stored or passed from the frontend
    let api_key = "your_api_key";
    let api_url = "your_api_url";

    match uploader::upload_to_9router(&file_path, api_key, api_url).await {
        Ok(summary) => {
            match uploader::save_summary(&summary) {
                Ok(path) => Ok(format!("Summary saved to {}", path)),
                Err(e) => Err(format!("Failed to save summary: {}", e)),
            }
        },
        Err(e) => Err(format!("Upload failed: {}", e)),
    }
}

fn add_data(buffer: &AudioBuffer, data: &[f32]) {
    buffer.add_data(data);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            let audio_buffer = AudioBuffer::new();
            app.manage(audio_buffer);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_recording, stop_recording, debug_save_to_desktop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
