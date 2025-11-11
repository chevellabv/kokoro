use kokoro_tts::{KokoroTts, Voice};
use std::fs::File;
use std::io::Write;
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("\n{}", "=".repeat(60));
    println!("Kokoro TTS Comparison Test");
    println!("{}\n", "=".repeat(60));

    // Test sentences (matching the Python script)
    let test_sentences = vec![
        "Hello, world! This is a test of the Kokoro text to speech system.",
        "The quick brown fox jumps over the lazy dog.",
        "Artificial intelligence is transforming how we interact with technology.",
        "Natural sounding speech synthesis requires careful attention to prosody and intonation.",
    ];

    // Initialize Kokoro TTS
    println!("Initializing Kokoro TTS...");
    let init_start = Instant::now();
    let tts = KokoroTts::new("kokoro-v1.0.int8.onnx", "voices.bin").await?;
    let init_time = init_start.elapsed();
    println!("✓ Initialization took: {:.3}s", init_time.as_secs_f32());

    // Model info
    let model_size = std::fs::metadata("kokoro-v1.0.int8.onnx")?.len() as f64 / (1024.0 * 1024.0);
    let voices_size = std::fs::metadata("voices.bin")?.len() as f64 / (1024.0 * 1024.0);
    println!("\nModel size: {:.2} MB", model_size);
    println!("Voices size: {:.2} MB", voices_size);
    println!("Total size: {:.2} MB", model_size + voices_size);

    // Create output directory
    std::fs::create_dir_all("kokoro_test_output")?;

    println!("\n{}", "=".repeat(60));
    println!("Generating audio samples...");
    println!("{}\n", "=".repeat(60));

    let mut total_audio_duration = 0.0f32;
    let mut total_generation_time = 0.0f32;

    // Test different voices
    let voices_to_test = vec![
        ("af_jessica", Voice::AfJessica(1.0)),
        ("bf_lily", Voice::BfLily(1.0)),
        ("am_puck", Voice::AmPuck(1.0)),
    ];

    for (voice_name, voice) in &voices_to_test {
        println!("\n--- Testing voice: {} ---\n", voice_name);

        for (i, text) in test_sentences.iter().enumerate() {
            println!("[{}/{}] \"{}\"", i + 1, test_sentences.len(), text);

            // Generate audio
            let start_time = Instant::now();
            let (audio, _duration) = tts.synth(text, voice.clone()).await?;
            let generation_time = start_time.elapsed().as_secs_f32();

            // Calculate audio duration
            // Kokoro outputs at 24000 Hz
            let sample_rate = 24000.0;
            let audio_duration = audio.len() as f32 / sample_rate;

            total_audio_duration += audio_duration;
            total_generation_time += generation_time;

            // Calculate Real-Time Factor (RTF)
            let rtf = if audio_duration > 0.0 {
                generation_time / audio_duration
            } else {
                0.0
            };

            println!("  ✓ Generated in: {:.3}s", generation_time);
            println!("  ✓ Audio duration: {:.2}s", audio_duration);
            println!(
                "  ✓ RTF: {:.3} ({})",
                rtf,
                if rtf < 1.0 { "faster" } else { "slower" }
            );

            // Save to WAV file
            let output_file = format!(
                "kokoro_test_output/kokoro_{}_sample_{}.wav",
                voice_name,
                i + 1
            );
            save_wav(&audio, 24000, &output_file)?;
            println!("  ✓ Saved to: {}", output_file);
        }
    }

    // Summary
    let num_voices = voices_to_test.len();
    let total_sentences = test_sentences.len() * num_voices;

    println!("\n{}", "=".repeat(60));
    println!("SUMMARY");
    println!("{}", "=".repeat(60));
    println!("Model: Kokoro v1.0 (82M parameters)");
    println!("Voices tested: {}", num_voices);
    println!("Total sentences: {}", total_sentences);
    println!("Total audio duration: {:.2}s", total_audio_duration);
    println!("Total generation time: {:.3}s", total_generation_time);
    println!(
        "Average RTF: {:.3}",
        total_generation_time / total_audio_duration
    );
    println!(
        "Average generation time per sentence: {:.3}s",
        total_generation_time / total_sentences as f32
    );
    println!("\nAll samples saved to: kokoro_test_output/");
    println!("{}\n", "=".repeat(60));

    println!("📝 Run the Python comparison script to test Piper:");
    println!("   python3 test_piper_comparison.py --compare-all");
    println!("\n🔊 Listen to the generated WAV files to compare quality");
    println!("{}\n", "=".repeat(60));

    Ok(())
}

fn save_wav(audio: &[f32], sample_rate: u32, output_path: &str) -> anyhow::Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(output_path, spec)?;

    for &sample in audio {
        let amplitude = (sample * i16::MAX as f32) as i16;
        writer.write_sample(amplitude)?;
    }

    writer.finalize()?;
    Ok(())
}
