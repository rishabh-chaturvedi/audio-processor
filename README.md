# audio_processor

A modular audio processing crate for Rust that leverages FFmpeg to perform a wide range of audio operations. The crate is designed for ease of integration and reuse in various projects. It provides a set of functions for audio editing—such as trimming, seeking, transcoding, applying effects, merging, reversing, normalizing, and overlaying audio.

## Features

- **File I/O & Metadata**
  - Load audio files.
  - Save processed audio files.

- **Basic Editing Operations**
  - **Seek:** Jump to a specified time position.
  - **Trim:** Extract a segment from an audio file.
  - **Merge:** Concatenate multiple audio files using FFmpeg's concat demuxer.

- **Transcoding**
  - Convert audio files between formats (e.g., WAV, MP3, FLAC, OGG).

- **Audio Effects & Processing**
  - **Volume Adjustment:** Scale the audio volume.
  - **Speed Change:** Modify playback speed.
  - **Effects:** Apply fade-in, fade-out, echo, and more.
  - **Reverse:** Reverse the audio stream.
  - **Normalize:** Adjust audio volume to a standard level.
  - **Overlay:** Mix one audio file onto another starting at a specified time.

- **Integration Tests**
  - Comprehensive tests to ensure each feature works as expected (requires FFmpeg to be installed).

## Prerequisites

- **Rust:** Ensure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).
- **FFmpeg:** This crate requires the FFmpeg executable. Install FFmpeg and ensure it is available in your system's PATH.
- **Tempfile Crate:** Used for managing temporary files (configured via Cargo.toml).

## Installation

Add the following dependency in your `Cargo.toml` file:

```toml
[dependencies]
audio_processor = "0.1.0"
```

If you are developing or testing locally, clone the repository and build using:

```bash
cargo build
```

## Usage

Here’s an example demonstrating how to use the crate:

```rust
use std::time::Duration;
use audio_processor::{
    AudioProcessor,
    transcoding::AudioFormat,
    processing::{AudioEffect},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the processor with an audio file.
    let audio = AudioProcessor::new("path/to/audio.wav")?;
    
    // Seek to 30 seconds into the audio.
    let seeked_audio = audio.seek(Duration::from_secs(30))?;
    
    // Trim the audio from 10 to 20 seconds.
    let trimmed_audio = seeked_audio.trim(Duration::from_secs(10), Duration::from_secs(20))?;
    
    // Transcode the trimmed audio to MP3.
    trimmed_audio.transcode(AudioFormat::Mp3, "output.mp3")?;
    
    // Adjust the volume by 1.5 times.
    let louder_audio = trimmed_audio.adjust_volume(1.5)?;
    
    // Change the playback speed by a factor of 1.25.
    let speed_changed_audio = louder_audio.change_speed(1.25)?;
    
    // Apply a fade-in effect of 2 seconds.
    let effected_audio = speed_changed_audio.apply_effect(AudioEffect::FadeIn(Duration::from_secs(2)))?;
    
    // Save the final output.
    effected_audio.save("final_output.wav")?;
    
    // Merge two audio files.
    let merged_audio = AudioProcessor::merge_audios(&[audio.clone(), seeked_audio.clone()], "merged_output.wav")?;
    
    // Reverse the audio.
    let reversed_audio = audio.reverse()?;
    
    // Normalize the audio volume.
    let normalized_audio = audio.normalize()?;
    
    // Overlay one audio onto another starting at 5 seconds.
    let overlayed_audio = audio.overlay(&seeked_audio, Duration::from_secs(5))?;
    
    Ok(())
}
```

## Running Integration Tests

The repository includes integration tests to validate each feature. The tests automatically generate a 5‑second silent audio file (using FFmpeg) if one is not present. To run the tests, execute:

```bash
cargo test -- --nocapture
```

Make sure that FFmpeg is installed and accessible in your PATH.

## Project Structure

```
audio_processor/
├── Cargo.toml          # Package metadata and dependencies.
├── src
│   ├── lib.rs          # Core library exposing the public API.
│   ├── io.rs           # Audio file input/output functions.
│   ├── processing.rs   # Audio processing functions and effects.
│   ├── transcoding.rs  # Audio format definitions and transcoding functions.
│   └── errors.rs       # Custom error definitions.
└── tests
    └── integration_tests.rs  # Integration tests covering all features.
```

## Contributing

Contributions are welcome! If you have ideas for new features, bug fixes, or improvements, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgements

- [FFmpeg](https://ffmpeg.org/) for the robust command-line tool used for audio processing.
- The Rust community for providing great tooling and libraries to build upon.
