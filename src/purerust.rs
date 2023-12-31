use rand::Rng;
use std::f32::consts::PI;

struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

struct Proton {
    position: Vec3,
    radius: f32,
    vibration_amplitude: f32,
    charge: f32,
}

impl Proton {
    fn new(x: f32, y: f32, z: f32, radius: f32, vibration_amplitude: f32) -> Self {
        Proton {
            position: Vec3 { x, y, z},
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
    position: Vec3,
    radius: f32,
    vibration_amplitude: f32,
    charge: f32,
}

impl Neutron {
    fn new(x: f32, y: f32, z: f32, radius: f32, vibration_amplitude: f32) -> Self {
        Neutron {
            position: Vec3 { x, y, z },
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
    orbit_center: Vec3,
    orbit_radius: f32,
    charge: f32,
}

impl Electron {
    fn new(orbit_center: Vec3, orbit_radius: f32) -> Self {
        Electron {
            orbit_center,
            orbit_radius,
            charge: -1.0,
        }
    }

    fn random_position(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..2.0 * PI);
        let radius_variation = rng.gen_range(-10.0..10.0); 
        let radius = self.orbit_radius + radius_variation;
        let z_variation = rng.gen_range(-10.0..10.0);

        Vec3 {
            x: self.orbit_center.x + radius * angle.cos(),
            y: self.orbit_center.y + radius * angle.sin(),
            z: self.orbit_center.z + z_variation,
        }
    }
}

const STRONG_FORCE_DISTANCE: f32 = 25.0;

fn distance(pos1: &Vec3, pos2: &Vec3) -> f32 {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    let dz = pos1.z - pos2.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

struct Atom {
    protons: Vec<Proton>,
    neutrons: Vec<Neutron>,
    electrons: Vec<Electron>,
    position: Vec3, // Center position of the atom
}

impl Atom {
    fn new(protons: Vec<Proton>, neutrons: Vec<Neutron>, electrons: Vec<Electron>, position: Vec3) -> Self {
        Atom {
            protons,
            neutrons,
            electrons,
            position,
        }
    }
}

struct Bond {
    atom1_index: usize,
    atom2_index: usize,
    bond_type: String, // For simplicity, just a string like "single", "double", etc.
}

struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}

const HYDROGEN_OXYGEN_DISTANCE: f32 = 1.0; // Approximate distance in some units
const HOH_ANGLE: f32 = 104.5; // Angle in degrees between hydrogen atoms in water

impl Molecule {
    fn new(atoms: Vec<Atom>, bonds: Vec<Bond>) -> Self {
        Molecule { atoms, bonds }
    }

    fn arrange_atoms(&mut self) {
        // Find the index of the oxygen atom (assuming it's the one with the most protons)
        let oxygen_index = self.atoms.iter().enumerate().max_by_key(|&(_, atom)| atom.protons.len()).map(|(index, _)| index).unwrap();

        // Assuming the first two other atoms are hydrogens
        let hydrogen_indices: Vec<usize> = self.atoms.iter().enumerate().filter_map(|(index, _)| {
            if index != oxygen_index { Some(index) } else { None }
        }).collect();

        // Position the oxygen atom at the origin for simplicity
        self.atoms[oxygen_index].position = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

        // Calculate positions for hydrogen atoms
        for (i, &hydrogen_index) in hydrogen_indices.iter().enumerate() {
            let angle_rad = (HOH_ANGLE / 2.0 + 180.0 * (i as f32)) * PI / 180.0;
            self.atoms[hydrogen_index].position = Vec3 {
                x: HYDROGEN_OXYGEN_DISTANCE * angle_rad.cos(),
                y: HYDROGEN_OXYGEN_DISTANCE * angle_rad.sin(),
                z: 0.0, // Assume they are in the same plane for simplicity
            };
        }
    }
}

fn main() {
    // Create hydrogen and oxygen atoms
    // For simplicity, we're not populating protons, neutrons, and electrons
    let hydrogen_atom1 = Atom::new(vec![], vec![], vec![], Vec3 { x: 0.0, y: 0.0, z: 0.0 });
    let hydrogen_atom2 = Atom::new(vec![], vec![], vec![], Vec3 { x: 0.0, y: 0.0, z: 0.0 });
    let oxygen_atom = Atom::new(vec![], vec![], vec![], Vec3 { x: 0.0, y: 0.0, z: 0.0 });

    // Create a water molecule
    let mut water_molecule = Molecule::new(
        vec![hydrogen_atom1, hydrogen_atom2, oxygen_atom],
        vec![
            Bond { atom1_index: 0, atom2_index: 2, bond_type: "single".to_string() },
            Bond { atom1_index: 1, atom2_index: 2, bond_type: "single".to_string() },
        ],
    );

    // Arrange the atoms in the molecule
    water_molecule.arrange_atoms();
}