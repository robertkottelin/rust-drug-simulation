use rand::Rng;
use std::f32::consts::PI;

struct Vec2 {
    x: f32,
    y: f32,
}

struct Proton {
    position: Vec2,
    radius: f32,
    vibration_amplitude: f32,
    charge: f32,
}

impl Proton {
    fn new(x: f32, y: f32, radius: f32, vibration_amplitude: f32) -> Self {
        Proton {
            position: Vec2 { x, y },
            radius,
            vibration_amplitude,
            charge: 1.0,
        }
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();
        self.position.x += rng.gen_range(-self.vibration_amplitude..self.vibration_amplitude);
        self.position.y += rng.gen_range(-self.vibration_amplitude..self.vibration_amplitude);
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
            position: Vec2 { x, y },
            radius,
            vibration_amplitude,
            charge: 0.0,
        }
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();
        self.position.x += rng.gen_range(-self.vibration_amplitude..self.vibration_amplitude);
        self.position.y += rng.gen_range(-self.vibration_amplitude..self.vibration_amplitude);
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
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..2.0 * PI);
        let radius_variation = rng.gen_range(-10.0..10.0); 
        let radius = self.orbit_radius + radius_variation;

        Vec2 {
            x: self.orbit_center.x + radius * angle.cos(),
            y: self.orbit_center.y + radius * angle.sin(),
        }
    }
}

const STRONG_FORCE_DISTANCE: f32 = 25.0;

fn distance(pos1: Vec2, pos2: Vec2) -> f32 {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    let mut protons = vec![Proton::new(100.0, 100.0, 10.0, 10.0)];
    let mut neutrons = vec![Neutron::new(100.0, 100.0, 10.0, 10.0)];
    let electrons = vec![Electron::new(Vec2 { x: 100.0, y: 100.0 }, 50.0)];

    // Main loop logic (example of one iteration)
    // In a real application, this would be in a loop
    for proton in &mut protons {
        for neutron in &mut neutrons {
            let dist = distance(proton.position, neutron.position);
            if dist < STRONG_FORCE_DISTANCE {
                proton.vibration_amplitude *= 0.9;
                neutron.vibration_amplitude *= 0.9;
            }
        }
    }

    for proton in &mut protons {
        proton.update();
    }

    for neutron in &mut neutrons {
        neutron.update();
    }

    // Electron positions can be calculated, but not drawn
    for electron in &electrons {
        let _electron_pos = electron.random_position();
        // Normally, you would draw here, but we're skipping that
    }

}