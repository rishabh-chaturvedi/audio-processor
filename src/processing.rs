use std::time::Duration;
use crate::errors::AudioError;

/// Enum for available audio effects.
#[derive(Debug)]
pub enum AudioEffect {
    FadeIn(Duration),
    FadeOut(Duration),
    Echo { delay: Duration, decay: f32 },
    // Additional effects (e.g., reverb) can be added here.
}

/// Converts an AudioEffect into an FFmpeg filter string.
pub fn effect_to_filter(effect: &AudioEffect) -> String {
    match effect {
        AudioEffect::FadeIn(dur) => {
            // The afade filter: type=in, start_time=0, duration=dur
            format!("afade=t=in:st=0:d={}", dur.as_secs_f32())
        }
        AudioEffect::FadeOut(dur) => {
            // Assuming fade out at the end (this is a simplification)
            format!("afade=t=out:st=0:d={}", dur.as_secs_f32())
        }
        AudioEffect::Echo { delay, decay } => {
            // Using a simple aecho filter.
            // Format: aecho=in_gain:out_gain:delays:decays
            format!("aecho=0.8:0.9:{}:{}", delay.as_millis(), decay)
        }
    }
}

/// Reverses an audio file using FFmpeg’s areverse filter.
pub fn reverse_audio(input_path: &str, output_path: &str) -> Result<(), AudioError> {
    println!("Reversing audio: {} -> {}", input_path, output_path);
    let status = std::process::Command::new("ffmpeg")
        .args(&["-i", input_path, "-af", "areverse", output_path, "-y"])
        .status()
        .map_err(|e| AudioError::IoError(e))?;
    if status.success() {
        Ok(())
    } else {
        Err(AudioError::FfmpegError("ffmpeg reverse failed".to_string()))
    }
}

/// Normalizes the audio volume using FFmpeg’s loudnorm filter.
pub fn normalize_volume(input_path: &str, output_path: &str) -> Result<(), AudioError> {
    println!("Normalizing volume: {} -> {}", input_path, output_path);
    let status = std::process::Command::new("ffmpeg")
        .args(&["-i", input_path, "-af", "loudnorm", output_path, "-y"])
        .status()
        .map_err(|e| AudioError::IoError(e))?;
    if status.success() {
        Ok(())
    } else {
        Err(AudioError::FfmpegError("ffmpeg normalize failed".to_string()))
    }
}
