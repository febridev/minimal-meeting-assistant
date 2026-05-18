use whisper_rs::{WhisperContext, WhisperContextParameters};
use std::sync::Mutex;
use std::path::Path;

pub struct WhisperStateContainer {
    pub context: Mutex<Option<WhisperContext>>,
}

impl WhisperStateContainer {
    pub fn new() -> Self {
        Self {
            context: Mutex::new(None),
        }
    }

    pub fn initialize(&self, model_path: &Path) -> Result<(), String> {
        let path_str = model_path.to_str().ok_or("Invalid model path")?;
        let ctx = WhisperContext::new_with_params(path_str, WhisperContextParameters::default())
            .map_err(|e| format!("Failed to load context: {}", e))?;
        let mut context = self.context.lock().unwrap();
        *context = Some(ctx);
        Ok(())
    }
}
