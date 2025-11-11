#!/usr/bin/env python3
"""
Comparison test between Piper TTS and Kokoro TTS
This script generates audio samples using Piper TTS for comparison with Kokoro
"""

import time
import wave
import os
from pathlib import Path

try:
    from piper_onnx import PiperOnnx
except ImportError:
    print("piper-onnx not installed. Installing...")
    import subprocess
    subprocess.check_call(["pip", "install", "piper-onnx"])
    from piper_onnx import PiperOnnx


# Test sentences
TEST_SENTENCES = [
    "Hello, world! This is a test of the Piper text to speech system.",
    "The quick brown fox jumps over the lazy dog.",
    "Artificial intelligence is transforming how we interact with technology.",
    "Natural sounding speech synthesis requires careful attention to prosody and intonation.",
]


def download_voice_model(quality="medium", voice_type="lessac"):
    """
    Download a voice model from Hugging Face
    Available qualities: x_low, low, medium, high
    Available voices: lessac (US), northern_english_male (GB)
    """
    print(f"Downloading Piper voice model (voice: {voice_type}, quality: {quality})...")

    # Voice configurations
    voice_configs = {
        "lessac": {
            "locale": "en_US",
            "path": "en/en_US/lessac",
            "name": "en_US-lessac",
        },
        "northern_english_male": {
            "locale": "en_GB",
            "path": "en/en_GB/northern_english_male",
            "name": "en_GB-northern_english_male",
        }
    }

    config = voice_configs.get(voice_type, voice_configs["lessac"])
    voice_name = f"{config['name']}-{quality}"

    # Download using huggingface_hub
    try:
        from huggingface_hub import hf_hub_download
    except ImportError:
        print("Installing huggingface_hub...")
        import subprocess
        subprocess.check_call(["pip", "install", "huggingface-hub"])
        from huggingface_hub import hf_hub_download

    # Download model and config
    model_path = hf_hub_download(
        repo_id="rhasspy/piper-voices",
        filename=f"{config['path']}/{quality}/{voice_name}.onnx"
    )
    config_path = hf_hub_download(
        repo_id="rhasspy/piper-voices",
        filename=f"{config['path']}/{quality}/{voice_name}.onnx.json"
    )

    print(f"✓ Downloaded model: {model_path}")
    print(f"✓ Downloaded config: {config_path}")

    return model_path, config_path


def save_wav(audio_data, sample_rate, output_path):
    """Save audio data to WAV file"""
    with wave.open(str(output_path), 'wb') as wav_file:
        wav_file.setnchannels(1)  # Mono
        wav_file.setsampwidth(2)  # 16-bit
        wav_file.setframerate(sample_rate)
        wav_file.writeframes(audio_data)


def test_piper(quality="medium", voice_type="lessac", output_dir="piper_test_output"):
    """Test Piper TTS with different quality levels and voices"""

    # Create output directory
    output_path = Path(output_dir)
    output_path.mkdir(exist_ok=True)

    print(f"\n{'='*60}")
    print(f"Testing Piper TTS (Voice: {voice_type}, Quality: {quality})")
    print(f"{'='*60}\n")

    # Download and load model
    model_path, config_path = download_voice_model(quality, voice_type)

    print("\nInitializing Piper TTS...")
    start_init = time.time()
    piper = PiperOnnx(model_path=model_path)
    init_time = time.time() - start_init
    print(f"✓ Initialization took: {init_time:.3f}s")

    # Get model info
    model_size = os.path.getsize(model_path) / (1024 * 1024)  # MB
    config_size = os.path.getsize(config_path) / (1024 * 1024)  # MB
    print(f"\nModel size: {model_size:.2f} MB")
    print(f"Config size: {config_size:.2f} MB")
    print(f"Total size: {model_size + config_size:.2f} MB")

    # Test each sentence
    print(f"\n{'='*60}")
    print("Generating audio samples...")
    print(f"{'='*60}\n")

    total_audio_duration = 0
    total_generation_time = 0

    for i, text in enumerate(TEST_SENTENCES, 1):
        print(f"\n[{i}/{len(TEST_SENTENCES)}] \"{text}\"")

        # Generate audio
        start_time = time.time()
        audio_data = piper.synth_to_wav(text)
        generation_time = time.time() - start_time

        # Save to file
        output_file = output_path / f"piper_{voice_type}_{quality}_sample_{i}.wav"
        with open(output_file, 'wb') as f:
            f.write(audio_data)

        # Calculate audio duration (approximate)
        # WAV header is 44 bytes, then raw PCM data
        audio_bytes = len(audio_data) - 44
        # Piper typically outputs 22050 Hz, 16-bit mono
        sample_rate = 22050
        bytes_per_sample = 2
        audio_duration = audio_bytes / (sample_rate * bytes_per_sample)

        total_audio_duration += audio_duration
        total_generation_time += generation_time

        # Calculate Real-Time Factor (RTF)
        rtf = generation_time / audio_duration if audio_duration > 0 else 0

        print(f"  ✓ Generated in: {generation_time:.3f}s")
        print(f"  ✓ Audio duration: {audio_duration:.2f}s")
        print(f"  ✓ RTF: {rtf:.3f} ({'faster' if rtf < 1 else 'slower'} than real-time)")
        print(f"  ✓ Saved to: {output_file}")

    # Summary
    print(f"\n{'='*60}")
    print("SUMMARY")
    print(f"{'='*60}")
    print(f"Quality level: {quality}")
    print(f"Total sentences: {len(TEST_SENTENCES)}")
    print(f"Total audio duration: {total_audio_duration:.2f}s")
    print(f"Total generation time: {total_generation_time:.3f}s")
    print(f"Average RTF: {total_generation_time/total_audio_duration:.3f}")
    print(f"Average generation time per sentence: {total_generation_time/len(TEST_SENTENCES):.3f}s")
    print(f"\nAll samples saved to: {output_path.absolute()}")
    print(f"{'='*60}\n")

    return {
        "voice_type": voice_type,
        "quality": quality,
        "model_size_mb": model_size,
        "init_time": init_time,
        "total_audio_duration": total_audio_duration,
        "total_generation_time": total_generation_time,
        "avg_rtf": total_generation_time / total_audio_duration,
        "avg_time_per_sentence": total_generation_time / len(TEST_SENTENCES),
    }


def compare_quality_levels():
    """Compare different Piper quality levels and voices"""
    print("\n" + "="*60)
    print("PIPER TTS QUALITY & VOICE COMPARISON")
    print("="*60 + "\n")

    # Test combinations: [(voice_type, quality)]
    test_configs = [
        ("lessac", "medium"),               # US English, medium
        ("northern_english_male", "medium"), # GB English, medium (the one you're interested in!)
        ("lessac", "high"),                 # US English, high quality
    ]

    results = []

    for voice_type, quality in test_configs:
        result = test_piper(quality, voice_type, f"piper_test_output/{voice_type}_{quality}")
        results.append(result)
        print("\n" + "="*60 + "\n")

    # Comparison table
    print("\n" + "="*80)
    print("COMPARISON TABLE")
    print("="*80)
    print(f"{'Voice':<25} {'Quality':<10} {'Size (MB)':<12} {'Avg RTF':<10} {'Avg Time/Sent':<15}")
    print("-" * 80)

    for r in results:
        print(f"{r['voice_type']:<25} {r['quality']:<10} {r['model_size_mb']:<12.2f} {r['avg_rtf']:<10.3f} {r['avg_time_per_sentence']:<15.3f}")

    print("="*80)
    print(f"\n📝 Compare these with Kokoro by running the Rust examples:")
    print(f"   cargo run --example synth_directly_v10 --release")
    print("\n🔊 Listen to the generated WAV files in piper_test_output/ to compare quality")
    print("="*60 + "\n")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="Test Piper TTS and compare with Kokoro")
    parser.add_argument(
        "--quality",
        choices=["low", "medium", "high"],
        default="medium",
        help="Voice quality level (default: medium)"
    )
    parser.add_argument(
        "--voice",
        choices=["lessac", "northern_english_male"],
        default="lessac",
        help="Voice type (default: lessac)"
    )
    parser.add_argument(
        "--compare-all",
        action="store_true",
        help="Compare multiple quality levels and voices"
    )

    args = parser.parse_args()

    if args.compare_all:
        compare_quality_levels()
    else:
        test_piper(args.quality, args.voice)
