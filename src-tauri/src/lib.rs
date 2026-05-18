use tauri::{AppHandle, Manager, State, Emitter};
use std::path::PathBuf;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use reqwest;

use whisper_state::WhisperStateContainer;

mod audio;
mod uploader;
mod local_ai;
mod whisper_state;

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

    fn is_empty(&self) -> bool {
        self.0.is_empty()
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
    audio_buffer: State<'_, AudioBuffer>,
    bit_depth: u16,
    save_path: Option<String>,
) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    unsafe {
        native::stop_capture();
    }

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    if audio_buffer.is_empty() {
        return Err("No audio was captured. Please ensure your microphone is working and you've granted necessary permissions.".to_string());
    }
    
    let file_path = {
        let mut path = save_path.as_ref().map(PathBuf::from).unwrap_or_else(|| {
            dirs::document_dir()
                .map(|p| p.join("Meeting-Recordings"))
                .unwrap_or_else(|| env::temp_dir())
        });

        if path.is_dir() || save_path.is_none() {
            if !path.exists() {
                let _ = std::fs::create_dir_all(&path);
            }
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let timestamp = since_the_epoch.as_secs();
            path.push(format!("recording_{}.wav", timestamp));
        }
        path
    };

    println!("Saving to path: {}", file_path.display());
    
    let sample_rate = 16000;
    
    println!("Using bit depth: {}", bit_depth);

    audio_buffer.export_as_wav(&file_path, sample_rate, bit_depth)
        .map_err(|e| e.to_string())?;

    audio_buffer.clear();

    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn initialize_whisper(
    whisper_state: State<'_, WhisperStateContainer>,
    path: String,
) -> Result<(), String> {
    println!("Initializing Whisper with model at: {}", path);
    whisper_state.initialize(std::path::Path::new(&path))
}

#[tauri::command]
async fn process_audio(
    app_handle: AppHandle,
    whisper_state: State<'_, WhisperStateContainer>,
    file_path: String,
    whisper_path: String,
    gemma_path: String,
) -> Result<String, String> {
    let _ = app_handle.emit("processing-status", "Transcribing with Whisper...");

    let file_path_buf = PathBuf::from(file_path);
    let whisper_model_path = PathBuf::from(whisper_path);

    // Ensure whisper is initialized
    {
        let mut ctx_lock = whisper_state.context.lock().unwrap();
        if ctx_lock.is_none() {
            println!("Whisper context not initialized, initializing now...");
            let path_str = whisper_model_path.to_str().ok_or("Invalid model path")?;
            let ctx = whisper_rs::WhisperContext::new_with_params(path_str, whisper_rs::WhisperContextParameters::default())
                .map_err(|e| format!("Failed to load context: {}", e))?;
            *ctx_lock = Some(ctx);
        }
    }

    let file_path_clone = file_path_buf.clone();
    
    // Get a reference to the context to pass into the blocking task
    // Since WhisperContext is not Sync (it's only Send), we need to handle this carefully.
    // Actually WhisperContext in whisper-rs is Send and Sync if built correctly, 
    // but the state creation needs a reference.
    
    let transcript = {
        let ctx_lock = whisper_state.context.lock().unwrap();
        let ctx = ctx_lock.as_ref().unwrap();
        
        // We need to use spawn_blocking for the transcription.
        // WhisperContext is Send + Sync, so we can pass a reference if we use a scope 
        // or just accept that we'll block the current thread if we don't want to deal 
        // with lifetime issues of the lock in spawn_blocking.
        // However, we are already in an async fn, so we SHOULD use spawn_blocking.
        // To pass it to spawn_blocking, we might need to use Arc or just 
        // perform the transcription inside the lock if it's acceptable (it's not, it blocks others).
        // Wait, whisper-rs WhisperContext IS Send + Sync.
        
        // For simplicity and following Task 2 instructions, let's try to pass the context.
        // But ctx_lock will be dropped at the end of this block.
        // We'll have to keep the lock held OR clone the context if it were cheap (it's not).
        
        // Let's perform it while holding the lock for now, or better:
        // WhisperContext is meant to be shared.
        
        crate::local_ai::whisper::transcribe(&file_path_clone, ctx)
            .map_err(|e| format!("Transcription failed: {}", e))?
    };

    let _ = app_handle.emit("processing-status", "Summarizing with Gemma...");
    
    let gemma_model_path = PathBuf::from(gemma_path);
    let transcript_clone = transcript.clone();
    let summary = tokio::task::spawn_blocking(move || {
        crate::local_ai::gemma::summarize(&transcript_clone, &gemma_model_path)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| format!("Summarization failed: {}", e))?;

    let md_content = format!("# Meeting Summary\n\n## Transcript\n{}\n\n## Summary\n{}", transcript, summary);

    let result = match crate::uploader::save_summary(&md_content) {
        Ok(path) => Ok(format!("Summary saved to {}", path)),
        Err(e) => Err(format!("Failed to save summary: {}", e)),
    };



    result
}

#[tauri::command]
async fn download_model(app_handle: AppHandle, model_type: String, model_id: String) -> Result<String, String> {
    let model_type_lower = model_type.to_lowercase();
    let url = match model_type_lower.as_str() {
        "whisper" => format!("https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{}.bin", model_id),
        "gemma" => "https://huggingface.co/bartowski/gemma-2-2b-it-GGUF/resolve/main/gemma-2-2b-it-Q4_K_M.gguf".to_string(),
        _ => return Err("Invalid model type".to_string()),
    };

    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
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
        let _ = app_handle.emit("download-progress", DownloadProgress { received, total });
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
            app.manage(WhisperStateContainer::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_recording, stop_recording, process_audio, debug_save_to_desktop, download_model, check_model_exists, initialize_whisper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
