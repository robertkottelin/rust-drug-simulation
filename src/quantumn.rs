use rand::Rng;
// use std::f32::consts::PI;

// A simplified quantum state with a 1D probability distribution
struct QuantumState {
    probability_distribution: Vec<f32>,
}

impl QuantumState {
    fn new(size: usize) -> Self {
        QuantumState {
            probability_distribution: vec![1.0 / size as f32; size],
        }
    }

    // Measure the quantum state, collapsing the wavefunction
    fn measure(&self) -> usize {
        let mut rng = rand::thread_rng();
        let random_value: f32 = rng.gen();
        let mut cumulative_probability = 0.0;
        for (index, &probability) in self.probability_distribution.iter().enumerate() {
            cumulative_probability += probability;
            if random_value <= cumulative_probability {
                return index;
            }
        }
        self.probability_distribution.len() - 1
    }

    // An abstract representation of quantum tunneling
    fn tunnel(&mut self) {
        let mut rng = rand::thread_rng();

        // Define tunneling probabilities for each position
        let tunneling_probabilities = self.probability_distribution.iter().map(|&prob| {
            // Probability of tunneling could be based on factors like barrier height and particle energy
            // For simplicity, we use a fixed small probability
            0.05 * prob
        }).collect::<Vec<f32>>();

        // Apply tunneling effect
        for (i, &tunnel_prob) in tunneling_probabilities.iter().enumerate() {
            if rng.gen::<f32>() < tunnel_prob {
                // For simplicity, let's say tunneling shifts the position by one unit
                // This is a basic representation and not how actual tunneling works
                let new_position = if i == 0 { self.probability_distribution.len() - 1 } else { i - 1 };
                self.probability_distribution[new_position] += tunnel_prob;
                self.probability_distribution[i] -= tunnel_prob;
            }
        }
    }

    // Update the quantum state (e.g., due to time evolution)
    fn update(&mut self) {
        // Placeholder for a more complex update mechanism
        self.probability_distribution.rotate_right(1);
    }
}

// Function to demonstrate entanglement (highly abstracted)
fn entangle_states(state1: &mut QuantumState, state2: &mut QuantumState) {
    // Create a joint probability distribution that represents the entangled state
    let mut joint_distribution = Vec::new();

    // Assuming the total system size is the product of individual state sizes
    for prob1 in &state1.probability_distribution {
        for prob2 in &state2.probability_distribution {
            // In a real quantum system, the joint distribution would be based on the specific entanglement
            // Here, we're just multiplying probabilities for simplicity
            joint_distribution.push(prob1 * prob2);
        }
    }
    // Update both states to this new joint distribution
    state1.probability_distribution = joint_distribution.clone();
    state2.probability_distribution = joint_distribution;
}

fn main() {
    let quantum_state_size = 100;
    let mut electron_state = QuantumState::new(quantum_state_size);
    let mut another_electron_state = QuantumState::new(quantum_state_size);

    // Entangle states (abstracted)
    entangle_states(&mut electron_state, &mut another_electron_state);

    // Main loop (simplified for demonstration)
    for _ in 0..10 {
        electron_state.tunnel(); // Quantum tunneling effect
        let measured_position = electron_state.measure();
        println!("Measured position of electron: {}", measured_position);
        
        // The entangled state should reflect the change
        another_electron_state.update();
        let entangled_position = another_electron_state.measure();
        println!("Measured position of entangled electron: {}", entangled_position);

        electron_state.update(); // Update the state for next iteration
    }
}
