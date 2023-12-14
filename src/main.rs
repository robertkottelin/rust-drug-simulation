use amethyst::{
    core::{timing::Time, transform::TransformBundle},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    window::WindowBundle,
};
use nphysics3d::{
    object::{Body, RigidBodyDesc},
    world::World,
};

struct Physics {
    world: World<f32>,
}

impl Default for Physics {
    fn default() -> Self {
        let mut world = World::new();
        world.set_gravity(nalgebra::Vector3::new(0.0, -9.81, 0.0));
        Physics { world }
    }
}

impl SimpleState for Physics {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        // Create a simple physical object
        let rigid_body = RigidBodyDesc::new()
            .gravity_enabled(true)
            .build();
        self.world.add_rigid_body(rigid_body);
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.world.step();
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            WindowBundle::from_config_path(display_config_path),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path).with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;
    let mut game = Application::new(app_root, Physics::default(), game_data)?;
    game.run();
    Ok(())
}
