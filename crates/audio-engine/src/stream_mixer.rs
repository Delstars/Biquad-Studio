//! Multi-Track Streaming Mixer (Premium)
//!
//! Mixes game and media buffers to a separate output matrix for streaming software like OBS.

/// A mixer that combines different audio sources into a dedicated stream output.
pub struct StreamMixer {
    game_volume: f32,
    media_volume: f32,
}

impl StreamMixer {
    /// Creates a new `StreamMixer` with default volumes.
    pub fn new() -> Self {
        Self {
            game_volume: 1.0,
            media_volume: 1.0,
        }
    }

    /// Sets the volume of the game track.
    pub fn set_game_volume(&mut self, volume: f32) {
        self.game_volume = volume;
    }

    /// Sets the volume of the media track.
    pub fn set_media_volume(&mut self, volume: f32) {
        self.media_volume = volume;
    }

    /// Mixes the game and media buffers into the output buffer for streaming.
    ///
    /// # Panics
    ///
    /// Panics if the lengths of `game`, `media`, and `out` are not equal.
    #[inline(always)]
    pub fn mix_for_stream(&self, game: &[f32], media: &[f32], out: &mut [f32]) {
        assert_eq!(game.len(), media.len(), "Game and media buffer lengths must match");
        assert_eq!(game.len(), out.len(), "Input and output buffer lengths must match");

        let g_vol = self.game_volume;
        let m_vol = self.media_volume;

        for ((o, &g), &m) in out.iter_mut().zip(game.iter()).zip(media.iter()) {
            *o = (g * g_vol) + (m * m_vol);
        }
    }
}

impl Default for StreamMixer {
    fn default() -> Self {
        Self::new()
    }
}
