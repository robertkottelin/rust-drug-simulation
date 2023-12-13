use bevy::prelude::*;
use nalgebra::Vector3;

#[derive(Component)]
struct Position(Vector3<f64>);

#[derive(Component)]
struct Velocity(Vector3<f64>);

#[derive(Component)]
struct Mass(f64);

// Derive Resource for SimulationParameters
#[derive(Resource)]
struct SimulationParameters {
    temperature: f64,
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(Position(Vector3::new(0.0, 0.0, 0.0)))
                    .insert(Velocity(Vector3::new(0.0, 0.0, 0.0)))
                    .insert(Mass(1.0));

    commands.spawn().insert(Position(Vector3::new(1.0, 0.0, 0.0)))
                    .insert(Velocity(Vector3::new(0.0, 0.0, 0.0)))
                    .insert(Mass(16.0));
}

fn physics_system(mut query: Query<(&mut Position, &mut Velocity, &Mass)>, sim_params: Res<SimulationParameters>) {
    for (mut position, mut velocity, mass) in query.iter_mut() {
        position.0 += velocity.0 * 0.1; // Dummy update for position
        velocity.0 += Vector3::new(sim_params.temperature * 0.01, 0.0, 0.0); // Dummy update for velocity
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SimulationParameters { temperature: 298.15 })
        .add_startup_system(setup)
        .add_system(physics_system)
        .run();
}
