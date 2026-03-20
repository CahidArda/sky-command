use bevy::audio::Volume;
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
                update_engine_sound.run_if(in_state(GameState::Flying)),
            )
            .add_systems(OnExit(GameState::Flying), despawn_engine_sound);
    }
}

/// Spawn engine sound by loading the appropriate WAV asset.
fn spawn_engine_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selected: Res<SelectedAircraft>,
) {
    let path = match selected.0 {
        AircraftType::Prop => "audio/engine_prop.wav",
        AircraftType::Airliner => "audio/engine_jet.wav",
        AircraftType::Fighter => "audio/engine_jet.wav",
    };

    let handle: Handle<AudioSource> = asset_server.load(path);

    commands.spawn((
        EngineSound,
        AudioPlayer(handle),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: Volume::new(0.3),
            ..default()
        },
    ));
}

/// Adjust engine sound pitch and volume based on throttle and speed.
fn update_engine_sound(
    aircraft_query: Query<&Aircraft>,
    sound_query: Query<&AudioSink, With<EngineSound>>,
) {
    let Ok(aircraft) = aircraft_query.get_single() else {
        return;
    };
    let Ok(sink) = sound_query.get_single() else {
        return;
    };

    let throttle = aircraft.throttle;
    let speed_factor = (aircraft.velocity.length() / 100.0).clamp(0.5, 2.0);

    sink.set_speed(0.7 + throttle * 0.8 + (speed_factor - 0.5) * 0.3);
    sink.set_volume(0.15 + throttle * 0.35);
}

/// Despawn engine sound when leaving Flying state.
fn despawn_engine_sound(mut commands: Commands, query: Query<Entity, With<EngineSound>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
