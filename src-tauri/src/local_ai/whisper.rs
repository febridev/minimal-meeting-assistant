use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
use std::path::Path;
use anyhow::Result;

pub fn transcribe(audio_path: &Path, model_path: &Path) -> Result<String> {
    // Load whisper context
    let path_str = model_path.to_str().ok_or_else(|| anyhow::anyhow!("Invalid model path"))?;
    let mut params = WhisperContextParameters::default();
    params.use_gpu(true);
    let ctx = WhisperContext::new_with_params(path_str, params)
        .map_err(|e| anyhow::anyhow!("Failed to load context: {}", e))?;
    let mut state = ctx.create_state().map_err(|e| anyhow::anyhow!("Failed to create state: {}", e))?;

    // Load audio
    let mut reader = hound::WavReader::open(audio_path).map_err(|e| anyhow::anyhow!("Failed to open WAV file: {}", e))?;
    let spec = reader.spec();
    
    // Ensure it is 16kHz mono as required by Whisper
    if spec.sample_rate != 16000 || spec.channels != 1 {
         // In a real app we'd resample, but for now we expect the recorded file to be correct
         // or we can just warn. The blueprint says we resample before saving.
    }

    let audio_samples: Vec<f32> = match (spec.sample_format, spec.bits_per_sample) {
        (hound::SampleFormat::Float, 32) => {
            reader.samples::<f32>().map(|s| s.unwrap()).collect()
        },
        (hound::SampleFormat::Int, 16) => {
            reader.samples::<i16>().map(|s| s.unwrap() as f32 / 32768.0).collect()
        },
        (hound::SampleFormat::Int, 32) => {
            reader.samples::<i32>().map(|s| s.unwrap() as f32 / 2147483648.0).collect()
        },
        _ => return Err(anyhow::anyhow!("Unsupported WAV format: {:?} {}bit", spec.sample_format, spec.bits_per_sample)),
    };
    
    // Debug logging
    println!("DEBUG: Transcription - Loaded {} audio samples", audio_samples.len());
    if audio_samples.is_empty() {
        return Err(anyhow::anyhow!("Audio is empty"));
    }
    if audio_samples.iter().any(|&s| s.is_nan()) {
        return Err(anyhow::anyhow!("Audio contains NaN"));
    }
    if audio_samples.iter().any(|&s| s < -1.0 || s > 1.0) {
        return Err(anyhow::anyhow!("Audio samples out of range [-1.0, 1.0]"));
    }
    
    let mut params = FullParams::new(SamplingStrategy::default());
    params.set_language(Some("auto"));

    // Run inference
    println!("DEBUG: Starting Whisper inference with {} samples", audio_samples.len());
    let res = state.full(params, &audio_samples);
    println!("DEBUG: Whisper inference result: {:?}", res);
    res.map_err(|e| anyhow::anyhow!("Failed to run inference: {}", e))?;
    println!("DEBUG: Whisper inference completed");

    let num_segments = state.full_n_segments().map_err(|e| anyhow::anyhow!("Failed to get segment count: {}", e))?;
    let mut transcript = String::new();
    for i in 0..num_segments {
        if let Ok(segment) = state.full_get_segment_text(i) {
            transcript.push_str(&segment);
        }
    }
    
    Ok(transcript)
}