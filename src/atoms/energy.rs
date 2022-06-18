use crate::atoms::atoms::Atoms;
use crate::atoms::{constants, io};
use na::{MatrixSlice3x1, Vector3};

fn lennard_jones_potential(
    pos1: &MatrixSlice3x1<f64>,
    pos2: &MatrixSlice3x1<f64>,
    epsilon: f64,
    sigma: f64,
) -> f64 {
    let distance = pos1.metric_distance(&pos2);
    let pauli = (sigma / distance).powi(12);
    let london = (sigma / distance).powi(6);
    4.0 * epsilon * (pauli - london)
}

impl Atoms {
    pub fn potential_energy(&self, epsilon: f64, sigma: f64) -> f64 {
        let mut potential_energy = 0.0;
        for (i, v1) in self.positions.column_iter().enumerate() {
            for (j, v2) in self.positions.column_iter().enumerate() {
                if i != j {
                    let single_pot = lennard_jones_potential(&v1, &v2, epsilon, sigma);
                    potential_energy += single_pot
                }
            }
        }
        0.5 * potential_energy
    }

    pub fn kinetic_energy(&self) -> f64 {
        let mut kinetic_energy = 0.0;

        for (i, atom_vel) in self.velocities.column_iter().enumerate() {
            kinetic_energy += 0.5 * atom_vel.metric_distance(&Vector3::zeros()).powi(2) * self.mass;
        }
        kinetic_energy
    }

    pub fn total_energy(&self, epsilon: f64, sigma: f64) -> f64 {
        self.kinetic_energy() + self.potential_energy(epsilon, sigma)
    }

    pub fn temperature(&self) -> f64 {
        let kinetic = self.kinetic_energy();
        kinetic / ((3.0 / 2.0) * constants::BOLTZMANN * self.num_atoms() as f64)
    }

    pub fn berendsen(mut self, temperature: f64, timestep: f64, tau: f64) -> Atoms {
        let lambda = (1.0 + (temperature / self.temperature() - 1.0) * timestep / tau).sqrt();
        self.velocities = self.velocities * lambda;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_potential_energy() {
        let atom1 = Vector3::from_element(0.0);
        let atom2 = Vector3::from_element(1.0);
        io::p(lennard_jones_potential(
            &atom1.column(0),
            &atom2.column(0),
            1.0,
            1.0,
        ));
    }
}
