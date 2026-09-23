use bevy::prelude::*;
use std::f32::consts::PI;

pub struct ProceduralSfx;

impl ProceduralSfx {
    /// Generates a valid in-memory 16-bit mono 44.1kHz WAV byte stream.
    pub fn create_wav(samples: &[i16], sample_rate: u32) -> Vec<u8> {
        let num_samples = samples.len() as u32;
        let data_size = num_samples * 2;
        let file_size = 36 + data_size;

        let mut wav = Vec::with_capacity(44 + data_size as usize);
        // RIFF header
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&file_size.to_le_bytes());
        wav.extend_from_slice(b"WAVE");

        // "fmt " subchunk
        wav.extend_from_slice(b"fmt ");
        wav.extend_from_slice(&16u32.to_le_bytes()); // Subchunk1Size (16 for PCM)
        wav.extend_from_slice(&1u16.to_le_bytes());  // AudioFormat (1 = PCM)
        wav.extend_from_slice(&1u16.to_le_bytes());  // NumChannels (1 = Mono)
        wav.extend_from_slice(&sample_rate.to_le_bytes()); // SampleRate
        wav.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // ByteRate (SampleRate * NumChannels * BitsPerSample/8)
        wav.extend_from_slice(&2u16.to_le_bytes());  // BlockAlign (NumChannels * BitsPerSample/8)
        wav.extend_from_slice(&16u16.to_le_bytes()); // BitsPerSample

        // "data" subchunk
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&data_size.to_le_bytes());

        // Samples
        for &sample in samples {
            wav.extend_from_slice(&sample.to_le_bytes());
        }

        wav
    }

    /// Synthesize cannon shot: quick downward frequency sweep with punchy attack
    pub fn shoot() -> Vec<u8> {
        let sample_rate = 44100;
        let duration = 0.08;
        let num_samples = (sample_rate as f32 * duration) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        let mut phase = 0.0;
        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let progress = t / duration;
            // Frequency drops from 650Hz to 120Hz
            let freq = 650.0 * (1.0 - progress) + 120.0;
            phase += 2.0 * PI * freq / sample_rate as f32;

            // Punchy exponential envelope
            let env = (1.0 - progress).powf(1.8);
            let s = phase.sin() * env;
            samples.push((s * 28000.0) as i16);
        }

        Self::create_wav(&samples, sample_rate)
    }

    /// Synthesize gate multiplication: bright harmonic crystal chime
    pub fn gate_chime() -> Vec<u8> {
        let sample_rate = 44100;
        let duration = 0.22;
        let num_samples = (sample_rate as f32 * duration) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let progress = t / duration;
            let env = (1.0 - progress).powf(2.0);

            // Shimmering chord: 880Hz (A5), 1320Hz (E6), 1760Hz (A6)
            let s = ((2.0 * PI * 880.0 * t).sin() * 0.5
                + (2.0 * PI * 1320.0 * t).sin() * 0.35
                + (2.0 * PI * 1760.0 * t).sin() * 0.2)
                * env;

            samples.push((s * 26000.0) as i16);
        }

        Self::create_wav(&samples, sample_rate)
    }

    /// Synthesize mob clash/pop
    pub fn pop() -> Vec<u8> {
        let sample_rate = 44100;
        let duration = 0.045;
        let num_samples = (sample_rate as f32 * duration) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        let mut phase = 0.0;
        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let progress = t / duration;
            let freq = 500.0 * (1.0 - progress) + 200.0;
            phase += 2.0 * PI * freq / sample_rate as f32;
            let env = (1.0 - progress).powf(2.5);
            let s = phase.sin() * env;
            samples.push((s * 22000.0) as i16);
        }

        Self::create_wav(&samples, sample_rate)
    }

    /// Synthesize heavy champion smash / roar
    pub fn champion_stomp() -> Vec<u8> {
        let sample_rate = 44100;
        let duration = 0.35;
        let num_samples = (sample_rate as f32 * duration) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        let mut phase = 0.0;
        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let progress = t / duration;
            let freq = 120.0 * (1.0 - progress * 0.6) + 40.0;
            phase += 2.0 * PI * freq / sample_rate as f32;
            let env = (1.0 - progress).powf(1.5);
            // Low rumble with sub harmonic
            let s = (phase.sin() * 0.7 + (phase * 0.5).sin() * 0.4) * env;
            samples.push((s * 30000.0) as i16);
        }

        Self::create_wav(&samples, sample_rate)
    }

    /// Synthesize Victory fanfare
    pub fn victory() -> Vec<u8> {
        let sample_rate = 44100;
        let duration = 0.7;
        let num_samples = (sample_rate as f32 * duration) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        // Arpeggio notes: C5 (523), E5 (659), G5 (784), C6 (1046)
        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let note_idx = (t / 0.15).floor() as usize;
            let freq = match note_idx {
                0 => 523.25,
                1 => 659.25,
                2 => 783.99,
                _ => 1046.50,
            };
            let note_t = t % 0.15;
            let env = (1.0 - (note_t / 0.15)).max(0.0).powf(1.2);
            let s = (2.0 * PI * freq * t).sin() * env;
            samples.push((s * 25000.0) as i16);
        }

        Self::create_wav(&samples, sample_rate)
    }

    /// Synthesize Defeat sound
    pub fn defeat() -> Vec<u8> {
        let sample_rate = 44100;
        let duration = 0.5;
        let num_samples = (sample_rate as f32 * duration) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let progress = t / duration;
            let freq = 280.0 * (1.0 - progress * 0.5) + 80.0;
            let env = (1.0 - progress).powf(1.3);
            let s = (2.0 * PI * freq * t).sin() * env;
            samples.push((s * 25000.0) as i16);
        }

        Self::create_wav(&samples, sample_rate)
    }
}

#[derive(Resource)]
pub struct SoundEffects {
    pub shoot: Handle<AudioSource>,
    pub gate_chime: Handle<AudioSource>,
    pub pop: Handle<AudioSource>,
    pub champion_stomp: Handle<AudioSource>,
    pub victory: Handle<AudioSource>,
    pub defeat: Handle<AudioSource>,
}

#[derive(Event)]
pub enum PlaySfx {
    Shoot,
    GateChime,
    Pop,
    ChampionStomp,
    Victory,
    Defeat,
}
