use macroquad::prelude::*;
use ::rand::*;

struct Nucleus {
    position: Vec2,
    radius: f32,
    vibration_amplitude: f32,
}

impl Nucleus {
    fn new(x: f32, y: f32, radius: f32, vibration_amplitude: f32) -> Self {
        Nucleus {
            position: vec2(x, y),
            radius,
            vibration_amplitude,
        }
    }

    fn update(&mut self) {
        // Randomly vibrate the nucleus within a small range
        let mut rng = thread_rng();
        self.position.x += rng.gen_range(-self.vibration_amplitude..self.vibration_amplitude);
        self.position.y += rng.gen_range(-self.vibration_amplitude..self.vibration_amplitude);
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, YELLOW);
    }
}

struct Electron {
    orbit_center: Vec2,
    orbit_radius: f32,
}

impl Electron {
    fn new(orbit_center: Vec2, orbit_radius: f32) -> Self {
        Electron {
            orbit_center,
            orbit_radius,
        }
    }

    fn random_position(&self) -> Vec2 {
        let mut rng = thread_rng();
        let angle = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
        let radius_variation = rng.gen_range(-10.0..10.0); 
        let radius = self.orbit_radius + radius_variation;

        Vec2::new(
            self.orbit_center.x + radius * angle.cos(),
            self.orbit_center.y + radius * angle.sin(),
        )
    }

    fn draw(&self) {
        let electron_pos = self.random_position();
        draw_circle(electron_pos.x, electron_pos.y, 5.0, BLUE);
    }
}

#[macroquad::main("Hydrogen Atom Simulation")]
async fn main() {
    let mut nucleus = Nucleus::new(screen_width() / 2.0, screen_height() / 2.0, 20.0, 1.0);
    let electron = Electron::new(nucleus.position, 100.0);

    loop {
        clear_background(BLACK);

        nucleus.update();

        nucleus.draw();
        electron.draw();

        next_frame().await
    }
}