use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};
use std::path::Path;
use anyhow::Result;

pub fn transcribe(audio_path: &Path, model_path: &Path) -> Result<String> {
    // Load whisper context
    let path_str = model_path.to_str().ok_or_else(|| anyhow::anyhow!("Invalid model path"))?;
    let ctx = WhisperContext::new(path_str).map_err(|e| anyhow::anyhow!("Failed to load context: {}", e))?;
    let mut state = ctx.create_state().map_err(|e| anyhow::anyhow!("Failed to create state: {}", e))?;

    // Load audio
    let mut reader = hound::WavReader::open(audio_path).map_err(|e| anyhow::anyhow!("Failed to open WAV file: {}", e))?;
    let samples: Vec<i16> = reader.samples().collect::<std::result::Result<_, _>>().map_err(|e| anyhow::anyhow!("Failed to read WAV samples: {}", e))?;
    let mut audio_samples: Vec<f32> = Vec::with_capacity(samples.len());
    for sample in samples {
        audio_samples.push(sample as f32 / 32768.0);
    }
    
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some("auto"));

    // Run inference
    state.full(params, &audio_samples).map_err(|e| anyhow::anyhow!("Failed to run inference: {}", e))?;

    let num_segments = state.full_n_segments().map_err(|e| anyhow::anyhow!("Failed to get segment count: {}", e))?;
    let mut transcript = String::new();
    for i in 0..num_segments {
        if let Ok(segment) = state.full_get_segment_text(i) {
            transcript.push_str(&segment);
        }
    }
    
    Ok(transcript)
}