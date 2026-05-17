use tauri::{AppHandle, Manager, State, Emitter};
use std::path::PathBuf;
use std::env;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use reqwest;

mod audio;
mod uploader;
mod local_ai;

#[derive(Clone, serde::Serialize)]
struct DownloadProgress {
    received: u64,
    total: Option<u64>,
}

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

    let whisper_model_path = std::path::PathBuf::from("models/ggml-small.bin");
    let gemma_model_path = std::path::PathBuf::from("models/gemma-2b.safetensors");

    let transcript = crate::local_ai::whisper::transcribe(&file_path, &whisper_model_path)
        .map_err(|e| format!("Transcription failed: {}", e))?;
    
    let summary = crate::local_ai::gemma::summarize(&transcript, &gemma_model_path)
        .map_err(|e| format!("Summarization failed: {}", e))?;

    let md_content = format!("# Meeting Summary\n\n## Transcript\n{}\n\n## Summary\n{}", transcript, summary);

    match crate::uploader::save_summary(&md_content) {
        Ok(path) => Ok(format!("Summary saved to {}", path)),
        Err(e) => Err(format!("Failed to save summary: {}", e)),
    }
}

#[tauri::command]
async fn download_model(app_handle: AppHandle, model_type: String, model_id: String) -> Result<(), String> {
    let url = match model_type.as_str() {
        "Whisper" => format!("https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{}", model_id),
        "Gemma" => format!("https://huggingface.co/google/gemma-2b/resolve/main/{}", model_id),
        _ => return Err("Invalid model type".to_string()),
    };

    let app_data_dir = app_handle.path().app_data_dir().unwrap();
    let model_dir = app_data_dir.join("models");
    tokio::fs::create_dir_all(&model_dir).await.map_err(|e| e.to_string())?;
    let model_path = model_dir.join(&model_id);

    let mut response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let total = response.content_length();

    let mut file = File::create(&model_path).await.map_err(|e| e.to_string())?;
    let mut received = 0;

    while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
        received += chunk.len() as u64;
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;
        app_handle.emit("download-progress", DownloadProgress { received, total }).unwrap();
    }

    Ok(())
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
        .invoke_handler(tauri::generate_handler![start_recording, stop_recording, debug_save_to_desktop, download_model])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
