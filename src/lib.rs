mod error;
mod g2p;
mod stream;
mod synthesizer;
mod tokenizer;
mod tokenizer_v019;
mod transcription;
mod voice;
mod voice_v019;
mod voices_v019;

use {
    bincode::{config::standard, decode_from_slice},
    ort::{execution_providers::CUDAExecutionProvider, session::Session},
    std::{collections::HashMap, path::Path, sync::Arc, time::Duration},
    tokio::{fs::read, sync::Mutex},
};
pub use {error::*, g2p::*, stream::*, tokenizer::*, tokenizer_v019::*, transcription::*, voice::*, voice_v019::*};

pub struct KokoroTts {
    model: Arc<Mutex<Session>>,
    voices: Arc<HashMap<String, Vec<Vec<Vec<f32>>>>>,
}

impl KokoroTts {
    pub async fn new<P: AsRef<Path>>(model_path: P, voices_path: P) -> Result<Self, KokoroError> {
        let voices = read(voices_path).await?;
        let (voices, _) = decode_from_slice(&voices, standard())?;

        let model = Session::builder()?
            .with_execution_providers([CUDAExecutionProvider::default().build()])?
            .commit_from_file(model_path)?;
        Ok(Self {
            model: Arc::new(model.into()),
            voices,
        })
    }

    pub async fn new_from_bytes<B>(model: B, voices: B) -> Result<Self, KokoroError>
    where
        B: AsRef<[u8]>,
    {
        let (voices, _) = decode_from_slice(voices.as_ref(), standard())?;

        let model = Session::builder()?
            .with_execution_providers([CUDAExecutionProvider::default().build()])?
            .commit_from_memory(model.as_ref())?;
        Ok(Self {
            model: Arc::new(model.into()),
            voices,
        })
    }

    pub async fn synth<S>(&self, text: S, voice: Voice) -> Result<(Vec<f32>, Duration), KokoroError>
    where
        S: AsRef<str>,
    {
        let name = voice.get_name();
        let pack = self
            .voices
            .get(name)
            .ok_or(KokoroError::VoiceNotFound(name.to_owned()))?;
        synthesizer::synth(Arc::downgrade(&self.model), text, pack, voice).await
    }

    pub fn stream<S>(&self, voice: Voice) -> (SynthSink<S>, SynthStream)
    where
        S: AsRef<str> + Send + 'static,
    {
        let voices = Arc::downgrade(&self.voices);
        let model = Arc::downgrade(&self.model);

        start_synth_session(voice, move |text, voice| {
            let voices = voices.clone();
            let model = model.clone();
            async move {
                let name = voice.get_name();
                let voices = voices.upgrade().ok_or(KokoroError::ModelReleased)?;
                let pack = voices
                    .get(name)
                    .ok_or(KokoroError::VoiceNotFound(name.to_owned()))?;
                synthesizer::synth(model, text, pack, voice).await
            }
        })
    }
}

/// Kokoro TTS for v0.19 models
/// v0.19 uses a different voice format: raw tensor (11, 511, 256)
pub struct KokoroTtsV019 {
    model: Arc<Mutex<Session>>,
    voices: Arc<voices_v019::VoicesV019>,
}

impl KokoroTtsV019 {
    pub async fn new<P: AsRef<Path>>(model_path: P, voices_path: P) -> Result<Self, KokoroError> {
        let voices = voices_v019::load_voices_v019(voices_path).await?;

        let model = Session::builder()?
            .with_execution_providers([CUDAExecutionProvider::default().build()])?
            .commit_from_file(model_path)?;
        Ok(Self {
            model: Arc::new(model.into()),
            voices: Arc::new(voices),
        })
    }

    pub async fn synth<S>(&self, text: S, voice: VoiceV019) -> Result<(Vec<f32>, Duration), KokoroError>
    where
        S: AsRef<str>,
    {
        let voice_idx = voice.get_index();
        if voice_idx >= self.voices.len() {
            return Err(KokoroError::VoiceNotFound(format!("Voice index {} out of range", voice_idx)));
        }

        let pack = &self.voices[voice_idx];
        synthesizer::synth_v019(Arc::downgrade(&self.model), text, pack, voice).await
    }
}
