# Alternative Method to Test Piper TTS

Since `piper-onnx` has dependency issues on macOS, here's how to test Piper using the native binary.

## Issue with piper-onnx

The Python library `piper-onnx` has ONNX Runtime dependency conflicts on macOS:
```
ERROR: Cannot install piper-onnx because onnxruntime has no matching distributions for macOS
```

## Alternative: Use Native Piper Binary

### Option 1: Download Pre-built Binary

1. **Download Piper for macOS:**
   ```bash
   # Download the latest release
   curl -L "https://github.com/rhasspy/piper/releases/download/2023.11.14-2/piper_macos_x64.tar.gz" -o piper_macos.tar.gz

   # Extract
   tar -xzf piper_macos.tar.gz
   cd piper
   ```

2. **Download a voice model:**
   ```bash
   # Create voices directory
   mkdir -p voices

   # Download Northern English male voice (medium quality)
   curl -L "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_GB/northern_english_male/medium/en_GB-northern_english_male-medium.onnx" \
     -o voices/en_GB-northern_english_male-medium.onnx

   curl -L "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_GB/northern_english_male/medium/en_GB-northern_english_male-medium.onnx.json" \
     -o voices/en_GB-northern_english_male-medium.onnx.json
   ```

3. **Test it:**
   ```bash
   echo "Hello, world! This is a test of the Piper text to speech system." | \
     ./piper --model voices/en_GB-northern_english_male-medium.onnx \
     --output_file test_output.wav

   # Play the audio
   afplay test_output.wav
   ```

### Option 2: Use Homebrew (if available)

```bash
# Note: Piper may not be in Homebrew, but you can try:
brew install piper-tts
```

### Option 3: Manual Testing Script

Create a simple bash script to test Piper:

```bash
#!/bin/bash
# piper_test.sh

PIPER_BINARY="./piper"
MODEL_PATH="voices/en_GB-northern_english_male-medium.onnx"
OUTPUT_DIR="piper_test_output"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Test sentences (matching Kokoro test)
SENTENCES=(
    "Hello, world! This is a test of the Piper text to speech system."
    "The quick brown fox jumps over the lazy dog."
    "Artificial intelligence is transforming how we interact with technology."
    "Natural sounding speech synthesis requires careful attention to prosody and intonation."
)

echo "============================================================"
echo "Piper TTS Test"
echo "============================================================"
echo ""
echo "Model: Northern English Male (medium quality)"
echo ""

# Generate audio for each sentence
for i in "${!SENTENCES[@]}"; do
    SENTENCE="${SENTENCES[$i]}"
    OUTPUT_FILE="$OUTPUT_DIR/piper_northern_male_sample_$((i+1)).wav"

    echo "[$((i+1))/4] \"$SENTENCE\""

    # Time the generation
    START=$(python3 -c "import time; print(time.time())")

    echo "$SENTENCE" | "$PIPER_BINARY" --model "$MODEL_PATH" --output_file "$OUTPUT_FILE" 2>/dev/null

    END=$(python3 -c "import time; print(time.time())")
    DURATION=$(python3 -c "print($END - $START)")

    echo "  ✓ Generated in: ${DURATION}s"
    echo "  ✓ Saved to: $OUTPUT_FILE"
    echo ""
done

echo "============================================================"
echo "All samples saved to: $OUTPUT_DIR/"
echo "============================================================"
echo ""
echo "🔊 Listen to the files and compare with Kokoro samples"
```

Save as `piper_test.sh`, make executable, and run:
```bash
chmod +x piper_test.sh
./piper_test.sh
```

## Voice Options to Test

### US English Voices
- **lessac** (medium): 63.2 MB - Neutral US English
  ```bash
  curl -L "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/lessac/medium/en_US-lessac-medium.onnx" -o voices/en_US-lessac-medium.onnx
  curl -L "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/lessac/medium/en_US-lessac-medium.onnx.json" -o voices/en_US-lessac-medium.onnx.json
  ```

### GB English Voices
- **northern_english_male** (medium): 63.2 MB - Northern British accent
  ```bash
  curl -L "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_GB/northern_english_male/medium/en_GB-northern_english_male-medium.onnx" -o voices/en_GB-northern_english_male-medium.onnx
  curl -L "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_GB/northern_english_male/medium/en_GB-northern_english_male-medium.onnx.json" -o voices/en_GB-northern_english_male-medium.onnx.json
  ```

- **alba** (medium): British Female
  ```bash
  curl -L "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_GB/alba/medium/en_GB-alba-medium.onnx" -o voices/en_GB-alba-medium.onnx
  curl -L "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_GB/alba/medium/en_GB-alba-medium.onnx.json" -o voices/en_GB-alba-medium.onnx.json
  ```

## Quality Levels to Compare

| Quality | Size | Sample Rate | Best Use |
|---------|------|-------------|----------|
| x_low | ~10-20 MB | 16kHz | Fastest, embedded |
| low | ~40-50 MB | 16kHz | Fast, low resources |
| **medium** | ~63 MB | 22.05kHz | **Recommended balance** |
| high | ~80-120 MB | 22.05kHz | Best quality |

## Expected Performance

Based on research:
- **RTF**: ~0.14-0.5 (faster than Kokoro in some benchmarks)
- **Speed**: Very fast on CPU
- **Quality**: "Very slightly more robotic" than Kokoro
- **Pronunciation**: Slightly less accurate than Kokoro

## Comparison Checklist

Once you have both Kokoro and Piper audio samples:

### Performance Metrics
- [ ] Generation time per sentence
- [ ] Real-Time Factor (RTF)
- [ ] Model size (Kokoro: 88MB vs Piper: 63MB)
- [ ] Initialization time

### Audio Quality
- [ ] Naturalness (human-like quality)
- [ ] Pronunciation accuracy
- [ ] Prosody (intonation, rhythm)
- [ ] Clarity and crispness
- [ ] Voice characteristics

### Practical Considerations
- [ ] Ease of integration
- [ ] Resource requirements
- [ ] Platform support
- [ ] Active development status
- [ ] Number of available voices
- [ ] Language support

## Side-by-Side Listening Test

Create a comparison directory:
```bash
mkdir comparison_test

# Copy Kokoro samples
cp kokoro_test_output/kokoro_af_jessica_sample_1.wav comparison_test/1_kokoro_jessica.wav

# Copy Piper samples
cp piper_test_output/piper_northern_male_sample_1.wav comparison_test/1_piper_northern.wav

# Listen to both
afplay comparison_test/1_kokoro_jessica.wav
afplay comparison_test/1_piper_northern.wav
```

## Recommended Next Steps

1. **Download Piper binary** for macOS
2. **Download 2-3 voice models** (northern_english_male, lessac, alba)
3. **Generate test samples** using the same sentences
4. **Listen and compare** audio quality
5. **Measure performance** (timing, RTF)
6. **Document your findings**

## Additional Resources

- **Piper Releases**: https://github.com/rhasspy/piper/releases
- **Voice Samples**: https://rhasspy.github.io/piper-samples/
- **All Voices**: https://huggingface.co/rhasspy/piper-voices
- **Documentation**: https://github.com/rhasspy/piper

---

**Note**: The Piper project was archived in October 2025, so there won't be future updates. However, the existing releases are stable and functional.
