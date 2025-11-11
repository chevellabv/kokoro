# Kokoro vs Piper TTS Comparison Guide

This guide will help you compare the quality and performance between Kokoro TTS and Piper TTS.

## Overview

**Kokoro TTS (Current)**
- 82M parameters
- Model size: ~88 MB (int8 quantized)
- Sample rate: 24kHz
- Based on StyleTTS2 architecture
- #1 in HuggingFace TTS Arena

**Piper TTS**
- 5-32M parameters (depending on quality level)
- Model size: 10-120 MB per voice
- Sample rate: 16-22.05kHz
- Based on VITS architecture
- Optimized for CPU/embedded devices

## Quality Levels (Piper)

| Level | Sample Rate | Parameters | Size | Speed |
|-------|-------------|------------|------|-------|
| x_low | 16kHz | 5-7M | ~10-20 MB | Fastest |
| low | 16kHz | 15-20M | ~40-50 MB | Fast |
| medium | 22.05kHz | 15-20M | ~63 MB | Balanced |
| high | 22.05kHz | 28-32M | ~80-120 MB | Best quality |

## Available Test Voices

### Piper Voices
1. **lessac** (en_US) - US English, neutral voice
2. **northern_english_male** (en_GB) - British English, Northern accent (63.2 MB)

### Kokoro Voices (examples)
1. **af_jessica** - American Female
2. **bf_lily** - British Female
3. **am_puck** - American Male

## Running the Comparison

### Step 1: Test Piper TTS (Python)

Install dependencies:
```bash
pip install piper-onnx huggingface-hub
```

Run a single voice test:
```bash
# Test US English voice (lessac)
python3 test_piper_comparison.py --voice lessac --quality medium

# Test Northern English male voice (GB)
python3 test_piper_comparison.py --voice northern_english_male --quality medium
```

Run comprehensive comparison:
```bash
python3 test_piper_comparison.py --compare-all
```

This will test:
- lessac (US) - medium quality
- northern_english_male (GB) - medium quality (the one you're interested in!)
- lessac (US) - high quality

### Step 2: Test Kokoro TTS (Rust)

Make sure you have the required model files:
- `kokoro-v1.0.int8.onnx`
- `voices.bin`

Run the comparison example:
```bash
cargo run --example compare_with_piper --release
```

This will generate audio samples for multiple Kokoro voices using the same test sentences.

### Step 3: Compare the Results

Both scripts will generate WAV files in their respective output directories:

**Piper output:**
```
piper_test_output/
├── lessac_medium/
│   ├── piper_lessac_medium_sample_1.wav
│   ├── piper_lessac_medium_sample_2.wav
│   └── ...
├── northern_english_male_medium/
│   ├── piper_northern_english_male_medium_sample_1.wav
│   └── ...
└── lessac_high/
    └── ...
```

**Kokoro output:**
```
kokoro_test_output/
├── kokoro_af_jessica_sample_1.wav
├── kokoro_bf_lily_sample_1.wav
├── kokoro_am_puck_sample_1.wav
└── ...
```

## What to Compare

### 1. Audio Quality
Listen to the generated WAV files and evaluate:
- **Naturalness**: How human-like does it sound?
- **Pronunciation**: Are words pronounced correctly?
- **Prosody**: Does the intonation and rhythm sound natural?
- **Clarity**: Is the audio clear and crisp?

### 2. Performance Metrics

The scripts will output:

**Speed Metrics:**
- **RTF (Real-Time Factor)**: < 1.0 = faster than real-time
  - Kokoro typically: < 0.3 RTF
  - Piper typically: < 1.0 RTF
- **Average generation time per sentence**
- **Total generation time**

**Model Metrics:**
- Model size (MB)
- Initialization time
- Total audio duration generated

### 3. Trade-offs

**Choose Kokoro if:**
- ✅ You want the fastest possible inference (< 0.3s)
- ✅ You prefer slightly better naturalness
- ✅ You want active development and updates
- ✅ You need 24kHz audio output
- ✅ 53 voices across 8 languages is sufficient

**Choose Piper if:**
- ✅ You need smaller model sizes
- ✅ You want 100+ voice options across 35 languages
- ✅ You're targeting embedded devices (Raspberry Pi)
- ✅ You need the absolute lowest resource usage
- ✅ You prefer the specific voice characteristics (e.g., Northern English)
- ⚠️ But note: Project archived in Oct 2025 (no future updates)

## Example Output

You should see something like:

**Piper:**
```
============================================================
Testing Piper TTS (Voice: northern_english_male, Quality: medium)
============================================================

Model size: 63.20 MB
✓ Initialization took: 0.234s

[1/4] "Hello, world! This is a test..."
  ✓ Generated in: 0.456s
  ✓ Audio duration: 3.21s
  ✓ RTF: 0.142 (faster than real-time)
```

**Kokoro:**
```
============================================================
Kokoro TTS Comparison Test
============================================================

Model size: 88.00 MB
✓ Initialization took: 0.152s

[1/4] "Hello, world! This is a test..."
  ✓ Generated in: 0.089s
  ✓ Audio duration: 3.18s
  ✓ RTF: 0.028 (faster than real-time)
```

## Test Sentences

Both scripts use identical test sentences:
1. "Hello, world! This is a test of the text to speech system."
2. "The quick brown fox jumps over the lazy dog."
3. "Artificial intelligence is transforming how we interact with technology."
4. "Natural sounding speech synthesis requires careful attention to prosody and intonation."

## Notes

- **File sizes**: Each voice in Piper is a separate ~63MB download (medium quality)
- **Quality differences**: Piper "medium" ≈ similar to Kokoro, but "very slightly more robotic"
- **Speed**: Kokoro is generally 3-5x faster than Piper
- **Naturalness**: Kokoro has a slight edge, but the difference is described as "very slight"

## Recommendation

Based on research and user feedback:

1. **If speed is your top priority**: Stick with **Kokoro** (< 0.3s vs Piper's ~0.5-1s)
2. **If you want better naturalness**: Try **Piper high quality** vs Kokoro
3. **If you want a specific accent**: **Piper northern_english_male** is a good unique option
4. **For production**: **Kokoro** (actively maintained) vs Piper (archived)

Try both and decide based on your ears! 🎧
