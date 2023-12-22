use macroquad::prelude::*;
use ::rand::*;
    
struct Proton {
    position: Vec2,
    radius: f32,
    vibration_amplitude: f32,
    charge: f32,
}

impl Proton {
    fn new(x: f32, y: f32, radius: f32, vibration_amplitude: f32) -> Self {
        Proton {
            position: vec2(x, y),
            radius,
            vibration_amplitude,
            charge: 1.0,
        }
    }

    fn update(&mut self) {
        // Randomly vibrate the proton within a small range
        let mut rng = thread_rng();
        self.position.x += rng.gen_range(-self.vibration_amplitude..self.vibration_amplitude);
        self.position.y += rng.gen_range(-self.vibration_amplitude..self.vibration_amplitude);
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, RED);
    }
}

struct Neutron {
    position: Vec2,
    radius: f32,
    vibration_amplitude: f32,
    charge: f32,
}

impl Neutron {
    fn new(x: f32, y: f32, radius: f32, vibration_amplitude: f32) -> Self {
        Neutron {
            position: vec2(x, y),
            radius,
            vibration_amplitude,
            charge: 0.0,
        }
    }

    fn update(&mut self) {
        // Randomly vibrate the neutron within a small range
        let mut rng = thread_rng();
        self.position.x += rng.gen_range(-self.vibration_amplitude..self.vibration_amplitude);
        self.position.y += rng.gen_range(-self.vibration_amplitude..self.vibration_amplitude);
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, GREEN);
    }
}

struct Electron {
    orbit_center: Vec2,
    orbit_radius: f32,
    charge: f32,
}

impl Electron {
    fn new(orbit_center: Vec2, orbit_radius: f32) -> Self {
        Electron {
            orbit_center,
            orbit_radius,
            charge: -1.0,
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

const STRONG_FORCE_DISTANCE: f32 = 25.0;

// Add a function to calculate the distance between two particles
fn distance(pos1: Vec2, pos2: Vec2) -> f32 {
    (pos1 - pos2).length()
}

#[macroquad::main("Particles")]
async fn main() {
    let mut protons = Vec::new();
    let mut neutrons = Vec::new();
    let mut electrons = Vec::new();

    for _ in 0..2 {
        protons.push(Proton::new(100.0, 100.0, 6.0, 10.0));
    }

    for _ in 0..2 {
        neutrons.push(Neutron::new(100.0, 100.0, 6.0, 10.0));
    }

    for _ in 0..1 {
        electrons.push(Electron::new(vec2(100.0, 100.0), 50.0));
    }

    loop {
        clear_background(BLACK);

        // Apply strong nuclear force
        for proton in &mut protons {
            for neutron in &mut neutrons {
                let dist = distance(proton.position, neutron.position);
                if dist < STRONG_FORCE_DISTANCE {
                    // Simulate strong force by reducing the vibration amplitude
                    proton.vibration_amplitude *= 0.9;
                    neutron.vibration_amplitude *= 0.9;
                }
            }
        }

        // Update and draw protons and neutrons
        for proton in &mut protons {
            proton.update();
            proton.draw();
        }

        for neutron in &mut neutrons {
            neutron.update();
            neutron.draw();
        }

        for electron in &electrons {
            electron.draw();
        }

        next_frame().await
    }
}