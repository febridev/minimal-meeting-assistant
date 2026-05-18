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
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    audio_buffer.clear();

    Ok(format!("Saved audio to {}", file_path.display()))
}

#[tauri::command]
async fn stop_recording(
    _app_handle: AppHandle, 
    audio_buffer: State<'_, AudioBuffer>, 
    bit_depth: u16,
    whisper_path: String,
    gemma_path: String
) -> Result<String, String> {
    println!("DEBUG: [BREADCRUMB 1] stop_recording entry");
    #[cfg(target_os = "macos")]
    unsafe {
        println!("DEBUG: [BREADCRUMB 2] calling native::stop_capture");
        native::stop_capture();
        println!("DEBUG: [BREADCRUMB 3] native::stop_capture returned");
    }

    println!("DEBUG: [BREADCRUMB 4] waiting for capture to flush");
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let temp_dir = env::temp_dir();
    let file_path = temp_dir.join("recorded_audio.wav");
    
    let sample_rate = 16000; 

    println!("DEBUG: [BREADCRUMB 5] exporting to WAV");
    audio_buffer.export_as_wav(&file_path, sample_rate, bit_depth)
        .map_err(|e| {
            println!("ERROR: WAV export failed: {}", e);
            e.to_string()
        })?;

    let whisper_model_path = std::path::PathBuf::from(whisper_path);
    let gemma_model_path = std::path::PathBuf::from(gemma_path);

    println!("DEBUG: [BREADCRUMB 6] starting transcription (blocking)");
    let file_path_clone = file_path.clone();
    let transcript = tokio::task::spawn_blocking(move || {
        println!("DEBUG: [BREADCRUMB 7] inside whisper thread");
        crate::local_ai::whisper::transcribe(&file_path_clone, &whisper_model_path)
    }).await.map_err(|e| {
        println!("ERROR: Whisper task panicked: {}", e);
        e.to_string()
    })?
    .map_err(|e| {
        println!("ERROR: Whisper error: {}", e);
        format!("Transcription failed: {}", e)
    })?;
    
    println!("DEBUG: [BREADCRUMB 8] transcription complete");
    
    println!("DEBUG: [BREADCRUMB 9] starting summarization (blocking)");
    let transcript_for_summary = transcript.clone();
    let summary = tokio::task::spawn_blocking(move || {
        println!("DEBUG: [BREADCRUMB 10] inside gemma thread");
        crate::local_ai::gemma::summarize(&transcript_for_summary, &gemma_model_path)
    }).await.map_err(|e| {
        println!("ERROR: Gemma task panicked: {}", e);
        e.to_string()
    })?
    .map_err(|e| {
        println!("ERROR: Gemma error: {}", e);
        format!("Summarization failed: {}", e)
    })?;

    println!("[BREADCRUMB 11] summarization complete");

    let md_content = format!("# Meeting Summary\n\n## Transcript\n{}\n\n## Summary\n{}", transcript, summary);

    audio_buffer.clear();

    println!("[BREADCRUMB 12] saving summary");
    match crate::uploader::save_summary(&md_content) {
        Ok(path) => {
            println!("DEBUG: Saved to {}", path);
            Ok(format!("Summary saved to {}", path))
        },
        Err(e) => {
            println!("ERROR: Save failed: {}", e);
            Err(format!("Failed to save summary: {}", e))
        }
    }
}

#[tauri::command]
async fn download_model(app_handle: AppHandle, model_type: String, model_id: String) -> Result<String, String> {
    let model_type_lower = model_type.to_lowercase();
    let url = match model_type_lower.as_str() {
        "whisper" => format!("https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{}.bin", model_id),
        "gemma" => "https://huggingface.co/bartowski/gemma-2-2b-it-GGUF/resolve/main/gemma-2-2b-it-Q4_K_M.gguf".to_string(),
        _ => return Err("Invalid model type".to_string()),
    };

    let app_data_dir = app_handle.path().app_data_dir().unwrap();
    let model_dir = app_data_dir.join("models");
    tokio::fs::create_dir_all(&model_dir).await.map_err(|e| e.to_string())?;
    
    let filename = if model_type_lower == "whisper" { 
        format!("ggml-{}.bin", model_id) 
    } else { 
        "gemma-2-2b-it-Q4_K_M.gguf".to_string() 
    };
    let model_path = model_dir.join(filename);

    if model_path.exists() {
        return Ok(model_path.to_string_lossy().to_string());
    }

    let mut response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("Download failed with status: {}", response.status()));
    }

    let total = response.content_length();
    let mut file = File::create(&model_path).await.map_err(|e| e.to_string())?;
    let mut received = 0;

    while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
        received += chunk.len() as u64;
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;
        app_handle.emit("download-progress", DownloadProgress { received, total }).unwrap();
    }

    Ok(model_path.to_string_lossy().to_string())
}

#[tauri::command]
fn check_model_exists(path: String) -> bool {
    std::path::Path::new(&path).exists()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .targets([
                        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir { file_name: Some("app".to_string()) }),
                        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                    ])
                    .level(log::LevelFilter::Info)
                    .build(),
            )?;
            let audio_buffer = AudioBuffer::new();
            app.manage(audio_buffer);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_recording, stop_recording, debug_save_to_desktop, download_model, check_model_exists])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
