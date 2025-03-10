pub mod io;
pub mod processing;
pub mod transcoding;
pub mod errors;

use std::time::Duration;
use crate::errors::AudioError;
use crate::transcoding::AudioFormat;
use crate::processing::{AudioEffect, effect_to_filter};

/// Main struct for processing an audio file.
#[derive(Debug, Clone)]
pub struct AudioProcessor {
    pub file_path: String,
}

impl AudioProcessor {
    /// Creates a new audio processor instance from a file path.
    pub fn new(file_path: &str) -> Result<Self, AudioError> {
        // Check if file exists; real FFmpeg initialization could be done here.
        std::fs::metadata(file_path).map_err(|e| AudioError::IoError(e))?;
        println!("Initializing audio processor for file: {}", file_path);
        io::load_audio(file_path)?;
        Ok(AudioProcessor {
            file_path: file_path.to_string(),
        })
    }

    /// Seeks to a given time position and outputs a new file.
    pub fn seek(&self, position: Duration) -> Result<Self, AudioError> {
        let output_file = format!("seeked_{}", self.file_path);
        let pos_str = format!("{}", position.as_secs());
        // Using "-ss" before input to perform a fast seek (copying streams)
        let status = std::process::Command::new("ffmpeg")
            .args(&["-ss", &pos_str, "-i", &self.file_path, "-c", "copy", &output_file, "-y"])
            .status()
            .map_err(|e| AudioError::IoError(e))?;
        if status.success() {
            println!("Seeked {} seconds into {} -> {}", pos_str, self.file_path, output_file);
            Ok(AudioProcessor { file_path: output_file })
        } else {
            Err(AudioError::FfmpegError("ffmpeg seek failed".to_string()))
        }
    }

    /// Trims the audio between start and end durations.
    /// Returns a new AudioProcessor instance with the trimmed segment.
    pub fn trim(&self, start: Duration, end: Duration) -> Result<Self, AudioError> {
        let output_file = format!("trimmed_{}", self.file_path);
        let start_str = format!("{}", start.as_secs());
        let end_str = format!("{}", end.as_secs());
        // "-ss" before input and "-to" after input for trimming without re-encoding.
        let status = std::process::Command::new("ffmpeg")
            .args(&["-ss", &start_str, "-to", &end_str, "-i", &self.file_path, "-c", "copy", &output_file, "-y"])
            .status()
            .map_err(|e| AudioError::IoError(e))?;
        if status.success() {
            println!("Trimmed {} from {} to {} seconds -> {}", self.file_path, start_str, end_str, output_file);
            Ok(AudioProcessor { file_path: output_file })
        } else {
            Err(AudioError::FfmpegError("ffmpeg trim failed".to_string()))
        }
    }

    /// Transcodes the current audio to a different format.
    pub fn transcode(&self, output_format: AudioFormat, output_path: &str) -> Result<(), AudioError> {
        // Let FFmpeg decide the codec based on output extension.
        let status = std::process::Command::new("ffmpeg")
            .args(&["-i", &self.file_path, output_path, "-y"])
            .status()
            .map_err(|e| AudioError::IoError(e))?;
        if status.success() {
            println!("Transcoded {} to format {:?} -> {}", self.file_path, output_format, output_path);
            Ok(())
        } else {
            Err(AudioError::FfmpegError("ffmpeg transcode failed".to_string()))
        }
    }

    /// Adjusts the audio volume by a scaling factor.
    pub fn adjust_volume(&self, factor: f32) -> Result<Self, AudioError> {
        let output_file = format!("volume_adjusted_{}", self.file_path);
        let filter = format!("volume={}", factor);
        let status = std::process::Command::new("ffmpeg")
            .args(&["-i", &self.file_path, "-af", &filter, &output_file, "-y"])
            .status()
            .map_err(|e| AudioError::IoError(e))?;
        if status.success() {
            println!("Adjusted volume of {} by factor {} -> {}", self.file_path, factor, output_file);
            Ok(AudioProcessor { file_path: output_file })
        } else {
            Err(AudioError::FfmpegError("ffmpeg adjust volume failed".to_string()))
        }
    }

    /// Changes the playback speed (and optionally pitch) by a factor.
    pub fn change_speed(&self, factor: f32) -> Result<Self, AudioError> {
        let output_file = format!("speed_changed_{}", self.file_path);
        // atempo filter supports 0.5 to 2.0; for other values, chain multiple filters.
        let filter = format!("atempo={}", factor);
        let status = std::process::Command::new("ffmpeg")
            .args(&["-i", &self.file_path, "-filter:a", &filter, &output_file, "-y"])
            .status()
            .map_err(|e| AudioError::IoError(e))?;
        if status.success() {
            println!("Changed speed of {} by factor {} -> {}", self.file_path, factor, output_file);
            Ok(AudioProcessor { file_path: output_file })
        } else {
            Err(AudioError::FfmpegError("ffmpeg change speed failed".to_string()))
        }
    }

    /// Applies an audio effect using FFmpeg filters.
    pub fn apply_effect(&self, effect: AudioEffect) -> Result<Self, AudioError> {
        let output_file = format!("effected_{}", self.file_path);
        // Convert our enum into an FFmpeg filter string.
        let filter = effect_to_filter(&effect);
        let status = std::process::Command::new("ffmpeg")
            .args(&["-i", &self.file_path, "-af", &filter, &output_file, "-y"])
            .status()
            .map_err(|e| AudioError::IoError(e))?;
        if status.success() {
            println!("Applied effect {:?} on {} -> {}", effect, self.file_path, output_file);
            Ok(AudioProcessor { file_path: output_file })
        } else {
            Err(AudioError::FfmpegError("ffmpeg apply effect failed".to_string()))
        }
    }

    /// Merges multiple audio files sequentially (concatenation).
    /// Uses FFmpeg’s concat demuxer.
    pub fn merge_audios(audios: &[AudioProcessor], output_path: &str) -> Result<Self, AudioError> {
        use std::io::Write;
        use tempfile::NamedTempFile;

        // Create a temporary file listing all input files.
        let mut list_file = NamedTempFile::new().map_err(|e| AudioError::IoError(e))?;
        for audio in audios {
            // The concat demuxer expects lines like: file 'path/to/file'
            writeln!(list_file, "file '{}'", audio.file_path).map_err(|e| AudioError::IoError(e))?;
        }
        list_file.flush().map_err(|e| AudioError::IoError(e))?;

        let status = std::process::Command::new("ffmpeg")
            .args(&["-f", "concat", "-safe", "0", "-i", list_file.path().to_str().unwrap(), "-c", "copy", output_path, "-y"])
            .status()
            .map_err(|e| AudioError::IoError(e))?;

        if status.success() {
            println!("Merged {} audio files -> {}", audios.len(), output_path);
            Ok(AudioProcessor { file_path: output_path.to_string() })
        } else {
            Err(AudioError::FfmpegError("ffmpeg merge failed".to_string()))
        }
    }

    /// Reverses the audio.
    pub fn reverse(&self) -> Result<Self, AudioError> {
        let output_file = format!("reversed_{}", self.file_path);
        let status = std::process::Command::new("ffmpeg")
            .args(&["-i", &self.file_path, "-af", "areverse", &output_file, "-y"])
            .status()
            .map_err(|e| AudioError::IoError(e))?;
        if status.success() {
            println!("Reversed audio {} -> {}", self.file_path, output_file);
            Ok(AudioProcessor { file_path: output_file })
        } else {
            Err(AudioError::FfmpegError("ffmpeg reverse failed".to_string()))
        }
    }

    /// Normalizes the audio volume.
    pub fn normalize(&self) -> Result<Self, AudioError> {
        let output_file = format!("normalized_{}", self.file_path);
        // Using loudnorm filter for normalization.
        let status = std::process::Command::new("ffmpeg")
            .args(&["-i", &self.file_path, "-af", "loudnorm", &output_file, "-y"])
            .status()
            .map_err(|e| AudioError::IoError(e))?;
        if status.success() {
            println!("Normalized audio {} -> {}", self.file_path, output_file);
            Ok(AudioProcessor { file_path: output_file })
        } else {
            Err(AudioError::FfmpegError("ffmpeg normalize failed".to_string()))
        }
    }

    /// Overlays another audio onto this one at a given start time.
    pub fn overlay(&self, overlay_audio: &AudioProcessor, start_time: Duration) -> Result<Self, AudioError> {
        let output_file = format!("overlayed_{}", self.file_path);
        // Using amix to mix two audio streams.
        // First, apply a delay to the overlay using the "adelay" filter.
        let delay_ms = start_time.as_millis();
        let filter = format!("[1]adelay={}|{delay}|{delay}[d]; [0][d]amix=inputs=2:duration=first", delay=delay_ms);
        let status = std::process::Command::new("ffmpeg")
            .args(&["-i", &self.file_path, "-i", &overlay_audio.file_path, "-filter_complex", &filter, &output_file, "-y"])
            .status()
            .map_err(|e| AudioError::IoError(e))?;
        if status.success() {
            println!("Overlayed {} onto {} at {} seconds -> {}", overlay_audio.file_path, self.file_path, start_time.as_secs(), output_file);
            Ok(AudioProcessor { file_path: output_file })
        } else {
            Err(AudioError::FfmpegError("ffmpeg overlay failed".to_string()))
        }
    }
}
