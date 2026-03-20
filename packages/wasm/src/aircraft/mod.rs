use bevy::prelude::*;

pub mod airliner;
pub mod bomber;
pub mod fighter;
pub mod prop;

use crate::state::GameState;

/// Marker component for the aircraft entity.
#[derive(Component)]
pub struct Aircraft {
    pub velocity: Vec3,
    pub throttle: f32,
    pub angular_velocity: Vec3,
    pub mass: f32,
    pub wing_area: f32,
    pub max_thrust: f32,
    pub cd0: f32,
    pub oswald_efficiency: f32,
    pub aspect_ratio: f32,
    pub pitch_rate: f32,
    pub roll_rate: f32,
    pub yaw_rate: f32,
    /// Lateral sideslip force coefficient — how strongly rudder/sideslip
    /// pushes the velocity. High for aircraft with large vertical tails,
    /// near zero for flying wings like the B-2.
    pub side_force_coeff: f32,
    /// Current angle of attack (radians), updated by flight model each frame.
    pub alpha: f32,
    /// Current G-load, updated by flight model each frame.
    pub g_load: f32,
}

/// Control input component attached to the aircraft.
#[derive(Component, Default)]
pub struct ControlInput {
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
    pub throttle_change: f32,
}

/// Marker for the propeller mesh (prop plane only).
#[derive(Component)]
pub struct Propeller;

/// Which type of aircraft is selected.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum AircraftType {
    #[default]
    Prop,
    Airliner,
    Fighter,
    Bomber,
}

impl AircraftType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Prop => "Cessna 172",
            Self::Airliner => "Boeing 737",
            Self::Fighter => "F-15 Eagle",
            Self::Bomber => "B-2 Spirit",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Prop => "Single-engine prop plane. Easy to fly.",
            Self::Airliner => "Twin-engine airliner. Stable but sluggish.",
            Self::Fighter => "Air superiority fighter. Fast and agile.",
            Self::Bomber => "Stealth flying wing. Heavy precision bomber.",
        }
    }

    pub fn has_weapons(&self) -> bool {
        matches!(self, Self::Fighter | Self::Bomber)
    }

    pub fn weapons_list(&self) -> &'static str {
        match self {
            Self::Fighter => "AIM-9  AGM-65",
            Self::Bomber => "GBU-31  JDAM",
            _ => "",
        }
    }
}

/// Resource indicating which aircraft the player chose.
#[derive(Resource, Default)]
pub struct SelectedAircraft(pub AircraftType);

pub struct AircraftPlugin;

impl Plugin for AircraftPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedAircraft>()
            .add_systems(OnEnter(GameState::Flying), spawn_selected_aircraft);
    }
}

/// Spawn the aircraft selected in the menu.
fn spawn_selected_aircraft(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    selected: Res<SelectedAircraft>,
) {
    match selected.0 {
        AircraftType::Prop => prop::spawn_aircraft(commands, meshes, materials),
        AircraftType::Airliner => airliner::spawn_aircraft(commands, meshes, materials),
        AircraftType::Fighter => fighter::spawn_aircraft(commands, meshes, materials),
        AircraftType::Bomber => bomber::spawn_aircraft(commands, meshes, materials),
    }
}
