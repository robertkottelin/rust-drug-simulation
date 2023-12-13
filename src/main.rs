use nalgebra::Vector3;
use rand::Rng;
use std::fs::File;
use std::io::Write;

#[derive(Clone, Debug)]
struct Atom {
    id: usize,
    position: Vector3<f64>,
    velocity: Vector3<f64>,
    mass: f64,
}

#[derive(Clone, Debug)]
struct Molecule {
    atoms: Vec<Atom>,
}

fn calculate_force(atom1: &Atom, atom2: &Atom) -> Vector3<f64> {
    let sigma = 1.0;
    let epsilon = 1.0;
    let r_vec = atom1.position - atom2.position;
    let r = r_vec.norm();

    let force_magnitude = 24.0 * epsilon * ((2.0 * (sigma/r).powi(13)) - ((sigma/r).powi(7)));
    let force_direction = r_vec.normalize();
    force_direction * force_magnitude
}

fn apply_heisenberg_uncertainty(atom: &mut Atom) {
    let mut rng = rand::thread_rng();
    let uncertainty_factor = 0.001; // Adjust this factor based on desired level of uncertainty

    // Apply uncertainty to position
    for i in 0..3 {
        atom.position[i] += rng.gen_range(-uncertainty_factor..uncertainty_factor);
    }

    // Apply uncertainty to velocity
    for i in 0..3 {
        atom.velocity[i] += rng.gen_range(-uncertainty_factor..uncertainty_factor);
    }
}


fn adjust_velocity_for_temperature(atom: &mut Atom, temperature: f64) {
    atom.velocity += Vector3::new(temperature * 0.01, temperature * 0.01, temperature * 0.01);
}

fn main() {
    let mut water1 = Molecule {
        atoms: vec![
            Atom { id: 1, position: Vector3::new(0.0, 0.0, 0.0), velocity: Vector3::new(0.0, 0.0, 0.0), mass: 1.0 }, 
            Atom { id: 2, position: Vector3::new(1.0, 0.0, 0.0), velocity: Vector3::new(0.0, 0.0, 0.0), mass: 1.0 }, 
            Atom { id: 3, position: Vector3::new(0.5, 0.866, 0.0), velocity: Vector3::new(0.0, 0.0, 0.0), mass: 16.0 },
        ],
    };


    let mut water2 = Molecule {
        atoms: vec![
            Atom { id: 4, position: Vector3::new(2.0, 2.0, 2.0), velocity: Vector3::new(0.0, 0.0, 0.0), mass: 1.0 },
            Atom { id: 5, position: Vector3::new(3.0, 2.0, 2.0), velocity: Vector3::new(0.0, 0.0, 0.0), mass: 1.0 },
            Atom { id: 6, position: Vector3::new(2.5, 2.866, 2.0), velocity: Vector3::new(0.0, 0.0, 0.0), mass: 16.0 },
        ],
    };


    let temperature = 298.15;

    for _ in 0..1 {
        for atom1 in &mut water1.atoms {
            for atom2 in &mut water2.atoms {
                let force = calculate_force(atom1, atom2);
                let acceleration1 = force / atom1.mass;
                let acceleration2 = -force / atom2.mass;

                atom1.velocity += acceleration1;
                atom1.position += atom1.velocity;
                atom2.velocity += acceleration2;
                atom2.position += atom2.velocity;

                apply_heisenberg_uncertainty(atom1);
                apply_heisenberg_uncertainty(atom2);
            }
        }
    }

    // Print out the final states of the atoms
    println!("Final states are:");
    println!("Water 1: {:?}", water1);
    println!("Water 2: {:?}", water2);
    // Create a new file
    let mut file = File::create("final_states.txt").unwrap();

    // Write the final states to the file
    writeln!(file, "Final states:").unwrap();
    writeln!(file, "Water 1: {:?}", water1).unwrap();
    writeln!(file, "Water 2: {:?}", water2).unwrap();

    // Close the file
    file.flush().unwrap();
}
