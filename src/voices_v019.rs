/// v0.19 voices loader
/// Format: Raw f32 tensor with shape (11, 511, 256)
/// Total: 11 voices × 511 time_steps × 256 features = 1,438,976 floats

use std::path::Path;
use tokio::fs::read;
use crate::KokoroError;

const NUM_VOICES_V019: usize = 11;
const TIME_STEPS_V019: usize = 511;
const FEATURES_V019: usize = 256;
const FLOATS_PER_VOICE: usize = TIME_STEPS_V019 * FEATURES_V019; // 130,816

pub type VoicesV019 = Vec<Vec<Vec<f32>>>;  // Shape: (11, 511, 256)

/// Load v0.19 voices.bin as raw tensor
pub async fn load_voices_v019<P: AsRef<Path>>(voices_path: P) -> Result<VoicesV019, KokoroError> {
    let bytes = read(voices_path).await?;

    // Verify size
    let expected_size = NUM_VOICES_V019 * FLOATS_PER_VOICE * 4; // *4 for f32 bytes
    if bytes.len() != expected_size {
        return Err(KokoroError::VoiceNotFound(format!(
            "v0.19 voices.bin size mismatch: expected {} bytes, got {}",
            expected_size, bytes.len()
        )));
    }

    // Parse as raw f32 little-endian
    let mut all_voices = Vec::with_capacity(NUM_VOICES_V019);

    for voice_idx in 0..NUM_VOICES_V019 {
        let mut voice_data = Vec::with_capacity(TIME_STEPS_V019);

        for time_idx in 0..TIME_STEPS_V019 {
            let mut features = Vec::with_capacity(FEATURES_V019);

            for feat_idx in 0..FEATURES_V019 {
                // Calculate byte offset: voice * voice_size + time * features + feat
                let offset = (voice_idx * FLOATS_PER_VOICE + time_idx * FEATURES_V019 + feat_idx) * 4;

                // Read f32 little-endian
                let bytes_slice = &bytes[offset..offset+4];
                let value = f32::from_le_bytes([
                    bytes_slice[0],
                    bytes_slice[1],
                    bytes_slice[2],
                    bytes_slice[3],
                ]);

                features.push(value);
            }

            voice_data.push(features);
        }

        all_voices.push(voice_data);
    }

    Ok(all_voices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_v019_voices() {
        // This will only work if the file exists
        if let Ok(voices) = load_voices_v019("../kokoro-en-v0_19 2/voices.bin").await {
            assert_eq!(voices.len(), NUM_VOICES_V019);
            assert_eq!(voices[0].len(), TIME_STEPS_V019);
            assert_eq!(voices[0][0].len(), FEATURES_V019);

            // Check that values are in reasonable range
            let first_val = voices[0][0][0];
            assert!(first_val > -10.0 && first_val < 10.0);

            println!("✓ v0.19 voices loaded: {} voices", NUM_VOICES_V019);
        }
    }
}
