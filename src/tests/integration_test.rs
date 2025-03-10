use std::fs;
use std::path::Path;
use std::time::Duration;

use audio_processor::{
    AudioProcessor,
    transcoding::AudioFormat,
    processing::AudioEffect,
};

/// Helper function to ensure that a test audio file exists.
/// This function uses FFmpeg to generate a 5-second silent audio file if needed.
fn setup_test_file() -> String {
    let test_dir = "tests/test_data";
    let file_path = format!("{}/silence.wav", test_dir);
    if !Path::new(&file_path).exists() {
        // Create the test directory if it doesn't exist.
        fs::create_dir_all(test_dir).expect("Failed to create test_data directory");
        // Generate a 5-second silent audio file.
        let status = std::process::Command::new("ffmpeg")
            .args(&[
                "-f", "lavfi",
                "-i", "anullsrc=r=44100:cl=stereo",
                "-t", "5",
                &file_path,
                "-y"
            ])
            .status()
            .expect("Failed to generate test silence file using ffmpeg");
        assert!(status.success(), "ffmpeg failed to create test file");
    }
    file_path
}

#[test]
fn test_seek() {
    let file = setup_test_file();
    let processor = AudioProcessor::new(&file).expect("Failed to create processor");
    let seeked_processor = processor.seek(Duration::from_secs(2)).expect("Seek failed");
    assert!(Path::new(&seeked_processor.file_path).exists());
    let _ = fs::remove_file(&seeked_processor.file_path);
}

#[test]
fn test_trim() {
    let file = setup_test_file();
    let processor = AudioProcessor::new(&file).expect("Failed to create processor");
    let trimmed_processor = processor.trim(Duration::from_secs(1), Duration::from_secs(4))
        .expect("Trim failed");
    assert!(Path::new(&trimmed_processor.file_path).exists());
    let _ = fs::remove_file(&trimmed_processor.file_path);
}

#[test]
fn test_transcode() {
    let file = setup_test_file();
    let processor = AudioProcessor::new(&file).expect("Failed to create processor");
    let output_path = "tests/test_data/transcoded.mp3";
    processor.transcode(AudioFormat::Mp3, output_path).expect("Transcode failed");
    assert!(Path::new(output_path).exists());
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_adjust_volume() {
    let file = setup_test_file();
    let processor = AudioProcessor::new(&file).expect("Failed to create processor");
    let vol_processor = processor.adjust_volume(1.5).expect("Adjust volume failed");
    assert!(Path::new(&vol_processor.file_path).exists());
    let _ = fs::remove_file(&vol_processor.file_path);
}

#[test]
fn test_change_speed() {
    let file = setup_test_file();
    let processor = AudioProcessor::new(&file).expect("Failed to create processor");
    let speed_processor = processor.change_speed(1.25).expect("Change speed failed");
    assert!(Path::new(&speed_processor.file_path).exists());
    let _ = fs::remove_file(&speed_processor.file_path);
}

#[test]
fn test_apply_effect() {
    let file = setup_test_file();
    let processor = AudioProcessor::new(&file).expect("Failed to create processor");
    let effect_processor = processor.apply_effect(AudioEffect::FadeIn(Duration::from_secs(2)))
        .expect("Apply effect failed");
    assert!(Path::new(&effect_processor.file_path).exists());
    let _ = fs::remove_file(&effect_processor.file_path);
}

#[test]
fn test_merge_audios() {
    let file = setup_test_file();
    let processor1 = AudioProcessor::new(&file).expect("Failed to create processor 1");
    let processor2 = AudioProcessor::new(&file).expect("Failed to create processor 2");
    let merged_output = "tests/test_data/merged.wav";
    let merged_processor = AudioProcessor::merge_audios(&[processor1, processor2], merged_output)
        .expect("Merge audios failed");
    assert!(Path::new(&merged_processor.file_path).exists());
    let _ = fs::remove_file(&merged_processor.file_path);
}

#[test]
fn test_reverse() {
    let file = setup_test_file();
    let processor = AudioProcessor::new(&file).expect("Failed to create processor");
    let reversed_processor = processor.reverse().expect("Reverse failed");
    assert!(Path::new(&reversed_processor.file_path).exists());
    let _ = fs::remove_file(&reversed_processor.file_path);
}

#[test]
fn test_normalize() {
    let file = setup_test_file();
    let processor = AudioProcessor::new(&file).expect("Failed to create processor");
    let normalized_processor = processor.normalize().expect("Normalize failed");
    assert!(Path::new(&normalized_processor.file_path).exists());
    let _ = fs::remove_file(&normalized_processor.file_path);
}

#[test]
fn test_overlay() {
    let file = setup_test_file();
    let base_processor = AudioProcessor::new(&file).expect("Failed to create base processor");
    let overlay_processor = AudioProcessor::new(&file).expect("Failed to create overlay processor");
    let overlayed_processor = base_processor.overlay(&overlay_processor, Duration::from_secs(1))
        .expect("Overlay failed");
    assert!(Path::new(&overlayed_processor.file_path).exists());
    let _ = fs::remove_file(&overlayed_processor.file_path);
}
