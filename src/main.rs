#[derive(Clone, Debug)]
struct Atom {
    id: usize,
    position: [f64; 3], // x, y, z coordinates
    velocity: [f64; 3], // velocity in x, y, z directions
    mass: f64,
}
#[derive(Clone, Debug)]
struct Molecule {
    atoms: Vec<Atom>,
}

fn calculate_force(atom1: &Atom, atom2: &Atom) -> f64 {
    let sigma = 1.0; // these are constants for the Lennard-Jones potential
    let epsilon = 1.0;
    let r = (atom1.position - atom2.position).abs(); // distance between atoms
    let force = 24.0 * epsilon * ((2.0 * (sigma/r).powi(13)) - ((sigma/r).powi(7)));
    return force;
}

fn adjust_velocity_for_temperature(atom: &mut Atom, temperature: f64) {
    atom.velocity += temperature * 0.01; // This is an arbitrary adjustment. 
}

fn main() {
    let mut water1 = Molecule {
        atoms: vec![
            Atom { id: 1, position: [0.0, 0.0, 0.0], velocity: [0.0, 0.0, 0.0], mass: 1.0 }, // hydrogen
            Atom { id: 2, position: [1.0, 0.0, 0.0], velocity: [0.0, 0.0, 0.0], mass: 1.0 }, // hydrogen
            Atom { id: 3, position: [0.5, 0.866, 0.0], velocity: [0.0, 0.0, 0.0], mass: 16.0 }, // oxygen
        ],
    };


    let mut water1 = Molecule {
        atoms: vec![
            Atom { id: 1, position: [2.0, 2.0, 2.0], velocity: [0.0, 0.0, 0.0], mass: 1.0 }, // hydrogen
            Atom { id: 2, position: [2.0, 2.0, 2.0], velocity: [0.0, 0.0, 0.0], mass: 1.0 }, // hydrogen
            Atom { id: 3, position: [2.5, 2.866, 2.0], velocity: [0.0, 0.0, 0.0], mass: 16.0 }, // oxygen
        ],
    };

    let temperature = 298.15; // room temperature in Kelvin
    for _ in 0..1000 {
        for atom1 in &mut water1.atoms {
            for atom2 in &mut water2.atoms {
                let force = calculate_force(atom1, atom2);
                let acceleration1 = force / atom1.mass;
                let acceleration2 = force / atom2.mass;

                // Update velocities and positions based on forces
                atom1.velocity += acceleration1;
                atom1.position += atom1.velocity;
                atom2.velocity -= acceleration2; // force is opposite in direction for atom2
                atom2.position += atom2.velocity;
                // Adjust velocities based on temperature
                adjust_velocity_for_temperature(atom1, temperature);
                adjust_velocity_for_temperature(atom2, temperature);
            }
        }
    }

    // Print out the final states of the atoms
    println!("Final states:");
    println!("Water 1: {:?}", water1);
    println!("Water 2: {:?}", water2);
}
