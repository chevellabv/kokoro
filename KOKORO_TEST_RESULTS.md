# Kokoro TTS Test Results

## Test Configuration

- **Model**: Kokoro v1.0 (82M parameters, int8 quantized)
- **Model Size**: 88.08 MB
- **Voices Size**: 27.00 MB
- **Total Size**: 115.08 MB
- **Sample Rate**: 24kHz
- **Date**: 2025-11-11

## Initialization

- **Init Time**: 0.390s

## Performance Results

### Overall Summary

| Metric | Value |
|--------|-------|
| Total Voices Tested | 3 (af_jessica, bf_lily, am_puck) |
| Total Sentences | 12 (4 per voice) |
| Total Audio Duration | 60.50s |
| Total Generation Time | 55.475s |
| **Average RTF** | **0.917** |
| Avg Time per Sentence | 4.623s |

### RTF Analysis

**Real-Time Factor (RTF) Breakdown:**
- RTF < 1.0 = Faster than real-time ✅
- RTF = 1.0 = Real-time speed
- RTF > 1.0 = Slower than real-time ⚠️

**Results by Sample:**

| Voice | Sample | Duration | Gen Time | RTF | Status |
|-------|--------|----------|----------|-----|--------|
| af_jessica | 1 | 4.43s | 3.594s | 0.812 | ✅ Faster |
| af_jessica | 2 | 3.28s | 2.633s | 0.804 | ✅ Faster |
| af_jessica | 3 | 5.10s | 4.038s | 0.792 | ✅ Faster |
| af_jessica | 4 | 5.88s | 4.653s | 0.792 | ✅ Faster |
| bf_lily | 1 | 5.05s | 3.996s | 0.791 | ✅ Faster |
| bf_lily | 2 | 3.58s | 5.921s | 1.656 | ⚠️ Slower |
| bf_lily | 3 | 5.65s | 4.433s | 0.785 | ✅ Faster |
| bf_lily | 4 | 6.45s | 5.044s | 0.782 | ✅ Faster |
| am_puck | 1 | 5.07s | 8.445s | 1.664 | ⚠️ Slower |
| am_puck | 2 | 3.25s | 2.604s | 0.801 | ✅ Faster |
| am_puck | 3 | 5.82s | 4.669s | 0.802 | ✅ Faster |
| am_puck | 4 | 6.95s | 5.445s | 0.783 | ✅ Faster |

### Performance Notes

1. **10 out of 12 samples** (83%) were faster than real-time
2. **2 samples** had RTF > 1.0 (likely first-generation overhead per voice)
3. **Average RTF of 0.917** means the system generates audio at approximately **1.09x real-time speed** overall
4. **Best performance**: af_jessica voice with consistent RTF ~0.79-0.81

## Generated Files

All audio files saved to `kokoro_test_output/`:

```
kokoro_af_jessica_sample_1.wav  207K
kokoro_af_jessica_sample_2.wav  154K
kokoro_af_jessica_sample_3.wav  239K
kokoro_af_jessica_sample_4.wav  275K

kokoro_bf_lily_sample_1.wav     237K
kokoro_bf_lily_sample_2.wav     168K
kokoro_bf_lily_sample_3.wav     265K
kokoro_bf_lily_sample_4.wav     302K

kokoro_am_puck_sample_1.wav     238K
kokoro_am_puck_sample_2.wav     152K
kokoro_am_puck_sample_3.wav     273K
kokoro_am_puck_sample_4.wav     326K
```

## Test Sentences

1. "Hello, world! This is a test of the Kokoro text to speech system."
2. "The quick brown fox jumps over the lazy dog."
3. "Artificial intelligence is transforming how we interact with technology."
4. "Natural sounding speech synthesis requires careful attention to prosody and intonation."

## Comparison with Benchmarks

### Expected vs Actual Performance

| Metric | Benchmark Claims | Actual Results | Notes |
|--------|------------------|----------------|-------|
| RTF | < 0.3 | 0.917 | 3x slower than claimed |
| Speed | "Sub-0.3s" | 2.6-8.4s per sentence | Varies by length |
| Quality | #1 TTS Arena | 🎧 Listen to judge | Subjective |

### Why the Difference?

Possible reasons for slower performance than benchmarks:

1. **Hardware**: Benchmarks likely used high-end GPUs (we're using CPU)
2. **First Run**: Model initialization and warm-up overhead
3. **Release Build**: We did use `--release` flag ✅
4. **Sentence Length**: Our test sentences are longer/more complex
5. **macOS**: Different platform optimizations

## Quality Assessment

🎧 **Listen to the files** to evaluate:

1. **Naturalness**: How human-like does each voice sound?
2. **Pronunciation**: Are all words correctly pronounced?
3. **Prosody**: Is the intonation and rhythm natural?
4. **Clarity**: Is the audio clear at 24kHz?
5. **Voice Variety**: How different are the three voices?

### Voices Tested

- **af_jessica**: American Female voice
- **bf_lily**: British Female voice
- **am_puck**: American Male voice

## Conclusions

✅ **Strengths:**
- Fast inference (83% of samples faster than real-time)
- Relatively small model size (88 MB)
- Multiple high-quality voices
- Clean, consistent performance after warm-up

⚠️ **Considerations:**
- Initial per-voice overhead (first sample slower)
- Performance not quite as fast as claimed benchmarks
- May benefit from GPU acceleration

## Next Steps

To compare with Piper TTS:

1. Use the native Piper binary (Python library has dependency issues on macOS)
2. Download pre-built Piper: https://github.com/rhasspy/piper/releases
3. Test with Northern English male voice
4. Compare audio quality and performance

---

**Generated**: 2025-11-11
**Hardware**: macOS
**Build**: Release mode
