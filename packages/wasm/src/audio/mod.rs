use bevy::prelude::*;

use crate::aircraft::{Aircraft, AircraftType, SelectedAircraft};
use crate::state::GameState;

/// Marker for the engine sound entity.
#[derive(Component)]
pub struct EngineSound;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Flying), spawn_engine_sound)
            .add_systems(
                Update,
                update_engine_sound
                    .run_if(in_state(GameState::Flying)),
            );
    }
}

/// Generate a looping WAV buffer with a simple waveform.
/// Returns raw WAV bytes for a short loop at the given base frequency.
fn generate_engine_wav(freq: f32, harmonics: &[(f32, f32)], sample_rate: u32, duration_secs: f32) -> Vec<u8> {
    let num_samples = (sample_rate as f32 * duration_secs) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let mut val: f32 = 0.0;

        // Base frequency
        val += (2.0 * std::f32::consts::PI * freq * t).sin() * 0.3;

        // Harmonics for character
        for &(harmonic_mult, amplitude) in harmonics {
            val += (2.0 * std::f32::consts::PI * freq * harmonic_mult * t).sin() * amplitude;
        }

        // Slight noise for texture
        let noise = ((t * 7919.0).sin() * 43758.5453).fract() * 2.0 - 1.0;
        val += noise * 0.05;

        // Clamp
        val = val.clamp(-1.0, 1.0);
        let sample = (val * 16000.0) as i16;
        samples.push(sample);
    }

    // Build WAV file in memory
    let data_size = (num_samples * 2) as u32;
    let file_size = 36 + data_size;
    let mut wav = Vec::with_capacity(file_size as usize + 8);

    // RIFF header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&file_size.to_le_bytes());
    wav.extend_from_slice(b"WAVE");

    // fmt chunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // chunk size
    wav.extend_from_slice(&1u16.to_le_bytes());  // PCM
    wav.extend_from_slice(&1u16.to_le_bytes());  // mono
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // byte rate
    wav.extend_from_slice(&2u16.to_le_bytes());  // block align
    wav.extend_from_slice(&16u16.to_le_bytes()); // bits per sample

    // data chunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());
    for sample in &samples {
        wav.extend_from_slice(&sample.to_le_bytes());
    }

    wav
}

/// Spawn engine sound based on the selected aircraft type.
fn spawn_engine_sound(
    mut commands: Commands,
    selected: Res<SelectedAircraft>,
    mut audio_sources: ResMut<Assets<AudioSource>>,
) {
    let sample_rate = 22050u32;
    let duration = 0.5; // short loop

    // Each aircraft gets a distinct engine sound character
    let (freq, harmonics): (f32, Vec<(f32, f32)>) = match selected.0 {
        AircraftType::Prop => (
            // Prop: low buzzy drone, lots of harmonics
            80.0,
            vec![(2.0, 0.25), (3.0, 0.15), (4.0, 0.10), (5.0, 0.08), (6.0, 0.05)],
        ),
        AircraftType::Airliner => (
            // Jet airliner: mid-frequency whine/roar
            150.0,
            vec![(1.5, 0.20), (2.0, 0.15), (3.0, 0.08), (4.5, 0.05)],
        ),
        AircraftType::Fighter => (
            // Fighter: high-pitched afterburner scream
            200.0,
            vec![(1.5, 0.22), (2.0, 0.18), (2.5, 0.12), (3.0, 0.10), (5.0, 0.06)],
        ),
        AircraftType::Bomber => (
            // Stealth bomber: deep low rumble
            60.0,
            vec![(2.0, 0.20), (3.0, 0.12), (4.0, 0.08)],
        ),
    };

    let wav_data = generate_engine_wav(freq, &harmonics, sample_rate, duration);
    let audio_source = AudioSource {
        bytes: wav_data.into(),
    };
    let handle = audio_sources.add(audio_source);

    commands.spawn((
        EngineSound,
        AudioPlayer(handle),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: bevy::audio::Volume::new(0.3),
            speed: 1.0,
            ..default()
        },
    ));
}

/// Adjust engine sound pitch and volume based on throttle and speed.
fn update_engine_sound(
    aircraft_query: Query<&Aircraft>,
    mut sound_query: Query<&mut PlaybackSettings, With<EngineSound>>,
) {
    let Ok(aircraft) = aircraft_query.get_single() else {
        return;
    };

    let throttle = aircraft.throttle;
    let speed_factor = (aircraft.velocity.length() / 100.0).clamp(0.5, 2.0);

    for mut settings in sound_query.iter_mut() {
        // Pitch rises with throttle and speed
        settings.speed = 0.7 + throttle * 0.8 + (speed_factor - 0.5) * 0.3;
        // Volume increases with throttle
        settings.volume = bevy::audio::Volume::new(0.15 + throttle * 0.35);
    }
}
