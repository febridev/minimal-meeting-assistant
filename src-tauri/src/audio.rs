use std::sync::Mutex;
use hound;
use std::path::Path;

pub struct AudioBuffer {
    pub data: Mutex<Vec<f32>>,
}

impl AudioBuffer {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(Vec::new()),
        }
    }

    pub fn clear(&self) {
        let mut data = self.data.lock().unwrap();
        data.clear();
    }

    pub fn add_data(&self, new_data: &[f32]) {
        let mut data = self.data.lock().unwrap();
        data.extend_from_slice(new_data);
    }


    pub fn normalize(&self) {
        let mut data = self.data.lock().unwrap();
        if data.is_empty() { return; }

        let peak = data.iter()
            .map(|&s| s.abs())
            .fold(0.0, f32::max);

        if peak > 0.0 {
            let scale = 0.9 / peak;
            for sample in data.iter_mut() {
                *sample *= scale;
            }
        }
    }

    pub fn export_as_wav(&self, path: &Path, sample_rate: u32, bits: u16) -> Result<(), hound::Error> {
        let spec = if bits == 32 {
            hound::WavSpec {
                channels: 1,
                sample_rate,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            }
        } else {
            hound::WavSpec {
                channels: 1,
                sample_rate,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            }
        };

        let mut writer = hound::WavWriter::create(path, spec)?;
        let data = self.data.lock().unwrap();

        if bits == 32 {
            for &sample in data.iter() {
                writer.write_sample(sample)?;
            }
        } else {
            for &sample in data.iter() {
                let amplitude = (sample.clamp(-1.0, 1.0) * std::i16::MAX as f32) as i16;
                writer.write_sample(amplitude)?;
            }
        }
        writer.finalize()?;
        Ok(())
    }
}
