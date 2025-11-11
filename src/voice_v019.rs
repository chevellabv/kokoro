//noinspection SpellCheckingInspection
#[derive(Copy, Clone, Debug)]
pub enum VoiceV019 {
    // v0.19 has 11 voices in the bin file
    // We'll map them by index for now until we identify them
    Voice0(f32),  // Likely af_heart or similar
    Voice1(f32),
    Voice2(f32),
    Voice3(f32),
    Voice4(f32),
    Voice5(f32),
    Voice6(f32),
    Voice7(f32),
    Voice8(f32),
    Voice9(f32),
    Voice10(f32),
}

impl VoiceV019 {
    pub(super) fn get_index(&self) -> usize {
        match self {
            Self::Voice0(_) => 0,
            Self::Voice1(_) => 1,
            Self::Voice2(_) => 2,
            Self::Voice3(_) => 3,
            Self::Voice4(_) => 4,
            Self::Voice5(_) => 5,
            Self::Voice6(_) => 6,
            Self::Voice7(_) => 7,
            Self::Voice8(_) => 8,
            Self::Voice9(_) => 9,
            Self::Voice10(_) => 10,
        }
    }

    pub(super) fn get_speed(&self) -> f32 {
        match self {
            Self::Voice0(v) | Self::Voice1(v) | Self::Voice2(v) | Self::Voice3(v) |
            Self::Voice4(v) | Self::Voice5(v) | Self::Voice6(v) | Self::Voice7(v) |
            Self::Voice8(v) | Self::Voice9(v) | Self::Voice10(v) => *v
        }
    }
}
